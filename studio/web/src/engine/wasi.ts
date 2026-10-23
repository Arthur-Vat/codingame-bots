/* A small WASI layer (wasi_snapshot_preview1) of our own, for the WebAssembly programs the browser
   engine runs: the rules and the releases. It offers the calls they make: environment, clock,
   random bytes, read, write and exit. Any other function a module imports from that namespace is a stub that
   fails with ENOSYS, so a future release that needs more fails cleanly instead of not loading. */

const ERRNO_SUCCESS = 0;
const ERRNO_BADF = 8;
const ERRNO_NOSYS = 52;

/** Thrown by `proc_exit` to unwind out of the program. */
export class WasiExit extends Error {
  readonly code: number;

  constructor(code: number) {
    super(`exit ${code}`);
    this.name = 'WasiExit';
    this.code = code;
  }
}

export interface WasiOptions {
  /** The environment variables the program sees. */
  env?: Record<string, string>;
  /**
   * Gives up to `max` bytes of the standard input, blocking until some are there. An empty
   * result is the end of the input, except when `mayBlock` is false: this read already has some
   * bytes, so a blocking input returns what it holds now, possibly nothing, rather than wait.
   */
  readStdin: (max: number, mayBlock: boolean) => Uint8Array;
  onStdout: (bytes: Uint8Array) => void;
  onStderr: (bytes: Uint8Array) => void;
}

export interface Wasi {
  /** The import object for the module, to instantiate it with. */
  imports: WebAssembly.Imports;
  /** Runs the instance's `_start`: its exit code (0 when it simply returns). */
  start: (instance: WebAssembly.Instance) => number;
}

const NAMESPACE = 'wasi_snapshot_preview1';

/** Builds the WASI functions for `module`: those implemented here, a stub for any other import. */
export function makeWasi(module: WebAssembly.Module, options: WasiOptions): Wasi {
  let memory: WebAssembly.Memory | null = null;
  const mem = (): WebAssembly.Memory => {
    if (memory === null) throw new Error('the program used its memory before it started');
    return memory;
  };
  const encoder = new TextEncoder();
  const environment = Object.entries(options.env ?? {}).map(([name, value]) =>
    encoder.encode(`${name}=${value}\0`),
  );

  const functions: Record<string, WebAssembly.ImportValue> = {
    environ_sizes_get(countPtr: number, sizePtr: number): number {
      const view = new DataView(mem().buffer);
      view.setUint32(countPtr, environment.length, true);
      view.setUint32(
        sizePtr,
        environment.reduce((total, bytes) => total + bytes.length, 0),
        true,
      );
      return ERRNO_SUCCESS;
    },
    environ_get(environPtr: number, bufferPtr: number): number {
      const view = new DataView(mem().buffer);
      const bytes = new Uint8Array(mem().buffer);
      for (const entry of environment) {
        view.setUint32(environPtr, bufferPtr, true);
        environPtr += 4;
        bytes.set(entry, bufferPtr);
        bufferPtr += entry.length;
      }
      return ERRNO_SUCCESS;
    },
    clock_time_get(id: number, _precision: bigint, outPtr: number): number {
      // Id 0 is the wall clock; the others (monotonic, CPU time) count from the page's start.
      const ms = id === 0 ? performance.timeOrigin + performance.now() : performance.now();
      new DataView(mem().buffer).setBigUint64(outPtr, BigInt(Math.round(ms * 1e6)), true);
      return ERRNO_SUCCESS;
    },
    fd_read(fd: number, iovs: number, iovsLen: number, readPtr: number): number {
      if (fd !== 0) return ERRNO_BADF;
      let total = 0;
      for (let i = 0; i < iovsLen; i++) {
        const view = new DataView(mem().buffer);
        const ptr = view.getUint32(iovs + 8 * i, true);
        const len = view.getUint32(iovs + 8 * i + 4, true);
        if (len === 0) continue;
        const chunk = options.readStdin(len, total === 0);
        if (chunk.length === 0) break;
        new Uint8Array(mem().buffer, ptr, chunk.length).set(chunk);
        total += chunk.length;
        if (chunk.length < len) break;
      }
      new DataView(mem().buffer).setUint32(readPtr, total, true);
      return ERRNO_SUCCESS;
    },
    fd_write(fd: number, iovs: number, iovsLen: number, writtenPtr: number): number {
      if (fd !== 1 && fd !== 2) return ERRNO_BADF;
      const view = new DataView(mem().buffer);
      let total = 0;
      for (let i = 0; i < iovsLen; i++) {
        const ptr = view.getUint32(iovs + 8 * i, true);
        const len = view.getUint32(iovs + 8 * i + 4, true);
        const bytes = new Uint8Array(mem().buffer, ptr, len).slice();
        if (fd === 1) options.onStdout(bytes);
        else options.onStderr(bytes);
        total += len;
      }
      view.setUint32(writtenPtr, total, true);
      return ERRNO_SUCCESS;
    },
    random_get(bufferPtr: number, length: number): number {
      // Always zeros: Rust's hash maps take their keys from here, and the same bytes on every run
      // keep a program's answers the same, which a replay needs.
      new Uint8Array(mem().buffer, bufferPtr, length).fill(0);
      return ERRNO_SUCCESS;
    },
    proc_exit(code: number): number {
      throw new WasiExit(code);
    },
  };

  const imports: Record<string, WebAssembly.ImportValue> = { ...functions };
  for (const entry of WebAssembly.Module.imports(module)) {
    if (entry.module === NAMESPACE && entry.kind === 'function' && !(entry.name in imports)) {
      imports[entry.name] = () => ERRNO_NOSYS;
    }
  }

  return {
    imports: { [NAMESPACE]: imports },
    start(instance) {
      const exported = instance.exports.memory;
      if (!(exported instanceof WebAssembly.Memory)) {
        throw new Error('the program does not export its memory');
      }
      memory = exported;
      const entry = instance.exports._start;
      if (typeof entry !== 'function') {
        throw new Error('the program has no _start function');
      }
      try {
        entry();
        return 0;
      } catch (error) {
        if (error instanceof WasiExit) return error.code;
        throw error;
      }
    },
  };
}

/** The result of running a program to its end. */
export interface RunResult {
  code: number;
  stdout: string;
}

/** Runs a command to its end with `input` as its whole standard input. */
export function runOnce(module: WebAssembly.Module, input: string): RunResult {
  let data: Uint8Array = new TextEncoder().encode(input);
  const out: Uint8Array[] = [];
  const wasi = makeWasi(module, {
    readStdin(max) {
      const chunk = data.subarray(0, max);
      data = data.subarray(chunk.length);
      return chunk;
    },
    onStdout: (bytes) => out.push(bytes),
    onStderr: () => {},
  });
  const instance = new WebAssembly.Instance(module, wasi.imports);
  const code = wasi.start(instance);
  const all = new Uint8Array(out.reduce((total, bytes) => total + bytes.length, 0));
  let offset = 0;
  for (const bytes of out) {
    all.set(bytes, offset);
    offset += bytes.length;
  }
  return { code, stdout: new TextDecoder().decode(all) };
}
