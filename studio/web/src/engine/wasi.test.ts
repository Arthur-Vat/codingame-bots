import { describe, expect, it } from 'vitest';
import { rulesModule } from './testSupport';
import { makeWasi, runOnce, type WasiOptions } from './wasi';

type Call = (...args: unknown[]) => number;

const EMPTY = new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]);

/** A WASI layer over an empty module, with a memory of its own that the calls write to. */
function harness(options: Partial<WasiOptions> = {}) {
  const wasi = makeWasi(new WebAssembly.Module(EMPTY), {
    readStdin: () => new Uint8Array(0),
    onStdout: () => {},
    onStderr: () => {},
    ...options,
  });
  const memory = new WebAssembly.Memory({ initial: 1 });
  const instance = { exports: { memory, _start: () => {} } } as unknown as WebAssembly.Instance;
  wasi.start(instance);
  const imports = wasi.imports.wasi_snapshot_preview1 as Record<string, Call>;
  const call = (name: string, ...args: unknown[]): number => {
    const fn = imports[name];
    if (fn === undefined) throw new Error(`no ${name}`);
    return fn(...args);
  };
  return {
    call,
    memory,
    view: () => new DataView(memory.buffer),
    bytes: () => new Uint8Array(memory.buffer),
  };
}

describe('the environment', () => {
  it('lists the variables as NAME=value, NUL-ended', () => {
    const { call, view, bytes } = harness({ env: { CG_SEED: '12', CG_FIXED_ITERS: '3000' } });
    expect(call('environ_sizes_get', 0, 4)).toBe(0);
    expect(view().getUint32(0, true)).toBe(2);
    const size = view().getUint32(4, true);
    expect(size).toBe('CG_SEED=12\0CG_FIXED_ITERS=3000\0'.length);
    expect(call('environ_get', 16, 64)).toBe(0);
    const first = view().getUint32(16, true);
    const second = view().getUint32(20, true);
    expect(first).toBe(64);
    expect(second).toBe(64 + 'CG_SEED=12\0'.length);
    expect(new TextDecoder().decode(bytes().subarray(64, 64 + size))).toBe(
      'CG_SEED=12\0CG_FIXED_ITERS=3000\0',
    );
  });

  it('is empty without variables', () => {
    const { call, view } = harness();
    call('environ_sizes_get', 0, 4);
    expect(view().getUint32(0, true)).toBe(0);
    expect(view().getUint32(4, true)).toBe(0);
  });
});

describe('the clock', () => {
  it('gives the wall time for id 0, in nanoseconds as a 64-bit number', () => {
    const { call, view } = harness();
    const before = Date.now();
    expect(call('clock_time_get', 0, 1n, 8)).toBe(0);
    const ms = Number(view().getBigUint64(8, true)) / 1e6;
    expect(ms).toBeGreaterThanOrEqual(before - 5);
    expect(ms).toBeLessThanOrEqual(Date.now() + 5);
  });

  it('gives a time that does not go back for the other ids', () => {
    const { call, view } = harness();
    call('clock_time_get', 1, 1n, 8);
    const first = view().getBigUint64(8, true);
    call('clock_time_get', 1, 1n, 8);
    expect(view().getBigUint64(8, true)).toBeGreaterThanOrEqual(first);
  });
});

describe('random bytes', () => {
  it('fills the buffer with the same bytes on every call: zeros', () => {
    const { call, bytes } = harness();
    bytes().fill(7, 0, 32);
    expect(call('random_get', 8, 16)).toBe(0);
    expect(Array.from(bytes().subarray(8, 24))).toEqual(new Array(16).fill(0));
    expect(Array.from(bytes().subarray(0, 8))).toEqual(new Array(8).fill(7));
    expect(Array.from(bytes().subarray(24, 32))).toEqual(new Array(8).fill(7));
  });
});

describe('the input and the output', () => {
  it('reads standard input into the buffers, and tells how much', () => {
    const data = new TextEncoder().encode('hello');
    const { call, view, bytes } = harness({ readStdin: (max) => data.subarray(0, max) });
    // Two buffers: 3 bytes at 100, 10 bytes at 200.
    view().setUint32(0, 100, true);
    view().setUint32(4, 3, true);
    view().setUint32(8, 200, true);
    view().setUint32(12, 10, true);
    expect(call('fd_read', 0, 0, 2, 32)).toBe(0);
    expect(view().getUint32(32, true)).toBe(3 + 5);
    expect(new TextDecoder().decode(bytes().subarray(100, 103))).toBe('hel');
    expect(new TextDecoder().decode(bytes().subarray(200, 205))).toBe('hello');
  });

  it('only lets the first buffer of a read wait for input', () => {
    const asked: boolean[] = [];
    let left = new TextEncoder().encode('abc');
    const { call, view } = harness({
      readStdin: (max, mayBlock) => {
        asked.push(mayBlock);
        const chunk = left.subarray(0, max);
        left = left.subarray(chunk.length);
        return chunk;
      },
    });
    // Two buffers; the first one takes the whole input, so the second must not wait for more.
    view().setUint32(0, 100, true);
    view().setUint32(4, 3, true);
    view().setUint32(8, 200, true);
    view().setUint32(12, 10, true);
    expect(call('fd_read', 0, 0, 2, 32)).toBe(0);
    expect(view().getUint32(32, true)).toBe(3);
    expect(asked).toEqual([true, false]);
  });

  it('reads nothing at the end of the input', () => {
    const { call, view } = harness();
    view().setUint32(0, 100, true);
    view().setUint32(4, 10, true);
    expect(call('fd_read', 0, 0, 1, 32)).toBe(0);
    expect(view().getUint32(32, true)).toBe(0);
  });

  it('refuses to read any other descriptor (EBADF)', () => {
    const { call } = harness();
    expect(call('fd_read', 3, 0, 0, 32)).toBe(8);
  });

  it('writes descriptors 1 and 2 to their sinks and refuses the others', () => {
    const out: string[] = [];
    const err: string[] = [];
    const decode = (bytes: Uint8Array) => new TextDecoder().decode(bytes);
    const { call, view, bytes } = harness({
      onStdout: (b) => out.push(decode(b)),
      onStderr: (b) => err.push(decode(b)),
    });
    bytes().set(new TextEncoder().encode('ab'), 100);
    bytes().set(new TextEncoder().encode('cde'), 200);
    view().setUint32(0, 100, true);
    view().setUint32(4, 2, true);
    view().setUint32(8, 200, true);
    view().setUint32(12, 3, true);
    expect(call('fd_write', 1, 0, 2, 32)).toBe(0);
    expect(view().getUint32(32, true)).toBe(5);
    expect(call('fd_write', 2, 0, 1, 32)).toBe(0);
    expect(out).toEqual(['ab', 'cde']);
    expect(err).toEqual(['ab']);
    expect(call('fd_write', 3, 0, 1, 32)).toBe(8);
  });
});

describe('exit', () => {
  it('ends the program with the code given to proc_exit', () => {
    const wasi = makeWasi(new WebAssembly.Module(EMPTY), {
      readStdin: () => new Uint8Array(0),
      onStdout: () => {},
      onStderr: () => {},
    });
    const imports = wasi.imports.wasi_snapshot_preview1 as Record<string, Call>;
    const instance = {
      exports: {
        memory: new WebAssembly.Memory({ initial: 1 }),
        _start: () => imports.proc_exit?.(7),
      },
    } as unknown as WebAssembly.Instance;
    expect(wasi.start(instance)).toBe(7);
  });

  it('returns 0 when the program just returns', () => {
    const wasi = makeWasi(new WebAssembly.Module(EMPTY), {
      readStdin: () => new Uint8Array(0),
      onStdout: () => {},
      onStderr: () => {},
    });
    const instance = {
      exports: { memory: new WebAssembly.Memory({ initial: 1 }), _start: () => {} },
    } as unknown as WebAssembly.Instance;
    expect(wasi.start(instance)).toBe(0);
  });
});

describe('an import this layer does not offer', () => {
  /** A module importing wasi_snapshot_preview1.sock_accept () -> i32, exported as `probe`. */
  function moduleImportingUnknown(): WebAssembly.Module {
    const text = new TextEncoder();
    const namespace = text.encode('wasi_snapshot_preview1');
    const name = text.encode('sock_accept');
    const probe = text.encode('probe');
    const importSize = 1 + 1 + namespace.length + 1 + name.length + 1 + 1;
    return new WebAssembly.Module(
      new Uint8Array([
        ...[0, 0x61, 0x73, 0x6d, 1, 0, 0, 0],
        ...[1, 5, 1, 0x60, 0, 1, 0x7f], // type 0: () -> i32
        ...[2, importSize, 1, namespace.length, ...namespace, name.length, ...name, 0, 0],
        ...[3, 2, 1, 0], // one function of type 0
        ...[7, 1 + 1 + probe.length + 2, 1, probe.length, ...probe, 0, 1], // export probe = func 1
        ...[10, 6, 1, 4, 0, 0x10, 0, 0x0b], // body: call 0
      ]),
    );
  }

  it('is a stub that fails with ENOSYS, so that the program still loads', () => {
    const module = moduleImportingUnknown();
    const wasi = makeWasi(module, {
      readStdin: () => new Uint8Array(0),
      onStdout: () => {},
      onStderr: () => {},
    });
    const instance = new WebAssembly.Instance(module, wasi.imports);
    const probe = instance.exports.probe as () => number;
    expect(probe()).toBe(52);
  });
});

describe('a real program', () => {
  it('runs the rules program on a request and prints its answer', async () => {
    const module = await rulesModule();
    const { code, stdout } = runOnce(module, '{"op":"games"}');
    expect(code).toBe(0);
    expect(JSON.parse(stdout)).toEqual({
      ok: [
        {
          id: 'uttt',
          name: 'Ultimate Tic-Tac-Toe',
          human_play: true,
          baselines: ['uttt-greedy', 'uttt-random'],
        },
        {
          id: 'ice-and-fire',
          name: 'A Code of Ice and Fire',
          human_play: false,
          baselines: [
            'ice-and-fire-cutter',
            'ice-and-fire-guard',
            'ice-and-fire-chain',
            'ice-and-fire-bronze',
            'ice-and-fire-wood1',
            'ice-and-fire-wood2',
            'ice-and-fire-wood3',
            'ice-and-fire-random',
          ],
        },
      ],
    });
  });

  it('can be run again from the same module, with nothing kept between runs', async () => {
    const module = await rulesModule();
    const first = runOnce(module, '{"op":"games"}').stdout;
    expect(runOnce(module, '{"op":"games"}').stdout).toBe(first);
  });
});
