/* The worker of one analysis (ADR 0032): it runs the game's analysis program, a WASI command, with
   the request as its whole standard input (no shared memory: this works on a page that is not
   cross-origin isolated), and sends each line of its standard output to the page as it is
   written. The page ends the worker with `terminate()` to stop the analysis. Standard error is
   dropped. */

import type { AnalysisMessage, AnalysisStart } from './analysisProtocol';
import { makeWasi } from './wasi';

// The worker's global scope, typed by hand: the DOM library of this project is the page's.
const scope = self as unknown as {
  onmessage: ((event: MessageEvent<AnalysisStart>) => void) | null;
  postMessage: (message: AnalysisMessage) => void;
};

scope.onmessage = ({ data: { module, request } }) => {
  scope.onmessage = null;
  let input: Uint8Array = new TextEncoder().encode(request);
  let text = '';
  const decoder = new TextDecoder();
  try {
    const wasi = makeWasi(module, {
      readStdin(max) {
        const chunk = input.subarray(0, max);
        input = input.subarray(chunk.length);
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
    scope.postMessage({ exit: wasi.start(instance) });
  } catch (error) {
    scope.postMessage({ error: error instanceof Error ? error.message : String(error) });
  }
};
