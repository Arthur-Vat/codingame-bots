/* Runs an analysis program in a module Worker (analysisWorker.ts): the factory the page gives the
   browser engine. Unit tests give `Analyses` their own factory, so that no Worker is needed. */

import type { AnalysisFactory } from './analyses';
import type { AnalysisMessage, AnalysisStart } from './analysisProtocol';

export const workerAnalysisFactory: AnalysisFactory = (module, request, onLine, onEnd) => {
  const worker = new Worker(new URL('./analysisWorker.ts', import.meta.url), { type: 'module' });
  let ended = false;
  worker.onmessage = ({ data }: MessageEvent<AnalysisMessage>) => {
    if ('line' in data) {
      onLine(data.line);
    } else if (!ended) {
      ended = true;
      worker.terminate();
      onEnd(data);
    }
  };
  worker.onerror = (event) => {
    if (ended) return;
    ended = true;
    worker.terminate();
    onEnd({ error: event.message === '' ? 'the worker failed' : event.message });
  };
  const start: AnalysisStart = { module, request };
  worker.postMessage(start);
  return {
    terminate: () => {
      ended = true;
      worker.terminate();
    },
  };
};
