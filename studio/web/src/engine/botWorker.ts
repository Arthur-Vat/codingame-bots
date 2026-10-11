/* The worker of one bot: it instantiates the release, tells the page it is ready, then runs it.
   The release's standard input blocks on the shared buffer the page fills (see bot.ts); each line
   of its standard output goes to the page; its standard error is dropped. */

import { HEADER_BYTES, type BotMessage, type BotStart } from './botProtocol';
import { makeWasi } from './wasi';

// The worker's global scope, typed by hand: the DOM library of this project is the page's.
const scope = self as unknown as {
  onmessage: ((event: MessageEvent<BotStart>) => void) | null;
  postMessage: (message: BotMessage) => void;
};

scope.onmessage = ({ data: { module, sab, env } }) => {
  scope.onmessage = null;
  const control = new Int32Array(sab, 0, 2);
  const data = new Uint8Array(sab, HEADER_BYTES);
  let pending: Uint8Array = new Uint8Array(0);
  let text = '';
  const decoder = new TextDecoder();
  try {
    const wasi = makeWasi(module, {
      env,
      readStdin(max, mayBlock) {
        if (pending.length === 0) {
          if (!mayBlock) return new Uint8Array(0);
          while (Atomics.load(control, 1) === 0) Atomics.wait(control, 1, 0);
          const size = Atomics.load(control, 1);
          pending = data.slice(0, size);
          Atomics.store(control, 1, 0);
          Atomics.notify(control, 1);
        }
        const chunk = pending.subarray(0, max);
        pending = pending.subarray(chunk.length);
        return chunk;
      },
      onStdout(bytes) {
        text += decoder.decode(bytes, { stream: true });
        for (let end = text.indexOf('\n'); end >= 0; end = text.indexOf('\n')) {
          const line = text.slice(0, end);
          text = text.slice(end + 1);
          scope.postMessage({ line: line.endsWith('\r') ? line.slice(0, -1) : line });
        }
      },
      onStderr() {},
    });
    const instance = new WebAssembly.Instance(module, wasi.imports);
    scope.postMessage({ ready: true });
    scope.postMessage({ exit: wasi.start(instance) });
  } catch (error) {
    scope.postMessage({ error: error instanceof Error ? error.message : String(error) });
  }
};
