import { afterEach, describe, expect, it, vi } from 'vitest';
import type { AnalysisMessage, AnalysisStart } from './analysisProtocol';
import { EMPTY_MODULE, analysisModule } from './testSupport';

/* The worker's script and the factory that starts it, run in this process with a stand-in for the
   worker's global scope and for `Worker`: what is real is the code of both files and the program. */

afterEach(() => {
  vi.unstubAllGlobals();
  vi.resetModules();
});

interface FakeScope {
  onmessage: ((event: { data: AnalysisStart }) => void) | null;
  postMessage: (message: AnalysisMessage) => void;
}

async function loadedWorker(): Promise<{ scope: FakeScope; posted: AnalysisMessage[] }> {
  const posted: AnalysisMessage[] = [];
  const scope: FakeScope = { onmessage: null, postMessage: (message) => posted.push(message) };
  vi.stubGlobal('self', scope);
  await import('./analysisWorker');
  return { scope, posted };
}

describe('the analysis worker', () => {
  it('runs the program on the request and posts each line, then its exit', async () => {
    const { scope, posted } = await loadedWorker();
    const request = JSON.stringify({ moves: ['4 4'], iterations: 2000, report_every: 1000 });
    scope.onmessage?.({ data: { module: await analysisModule('uttt'), request } });
    expect(posted).toHaveLength(3);
    expect(posted.slice(0, 2).every((message) => 'line' in message)).toBe(true);
    expect(posted[2]).toEqual({ exit: 0 });
    const last = posted[1];
    expect(last !== undefined && 'line' in last ? JSON.parse(last.line) : null).toMatchObject({
      report: { iterations: 2000, done: true },
    });
    // The worker answers one start only.
    expect(scope.onmessage).toBeNull();
  });

  it('posts an error when the program cannot run', async () => {
    const { scope, posted } = await loadedWorker();
    // A module with no _start function.
    const empty = new WebAssembly.Module(EMPTY_MODULE);
    scope.onmessage?.({ data: { module: empty, request: '{}' } });
    expect(posted).toHaveLength(1);
    expect(posted[0]).toHaveProperty('error');
  });
});

describe('the worker factory', () => {
  class FakeWorker {
    static all: FakeWorker[] = [];
    onmessage: ((event: { data: AnalysisMessage }) => void) | null = null;
    onerror: ((event: { message: string }) => void) | null = null;
    terminated = 0;
    sent: unknown[] = [];

    constructor() {
      FakeWorker.all.push(this);
    }

    postMessage(message: unknown) {
      this.sent.push(message);
    }

    terminate() {
      this.terminated += 1;
    }
  }

  async function started() {
    FakeWorker.all = [];
    vi.stubGlobal('Worker', FakeWorker);
    const { workerAnalysisFactory } = await import('./analysisFactory');
    const lines: string[] = [];
    const ends: unknown[] = [];
    const module = new WebAssembly.Module(EMPTY_MODULE);
    const run = workerAnalysisFactory(
      module,
      '{"moves":[]}',
      (line) => lines.push(line),
      (end) => ends.push(end),
    );
    const worker = FakeWorker.all[0];
    if (worker === undefined) throw new Error('no worker');
    return { run, worker, lines, ends, module };
  }

  it('sends the program and the request, and forwards lines and the end', async () => {
    const { worker, lines, ends, module } = await started();
    expect(worker.sent).toEqual([{ module, request: '{"moves":[]}' }]);
    worker.onmessage?.({ data: { line: 'a' } });
    worker.onmessage?.({ data: { line: 'b' } });
    worker.onmessage?.({ data: { exit: 0 } });
    expect(lines).toEqual(['a', 'b']);
    expect(ends).toEqual([{ exit: 0 }]);
    expect(worker.terminated).toBe(1);
  });

  it('terminates the worker on request and reports no end after it', async () => {
    const { run, worker, ends } = await started();
    run.terminate();
    expect(worker.terminated).toBe(1);
    worker.onmessage?.({ data: { exit: 0 } });
    worker.onerror?.({ message: 'late' });
    expect(ends).toEqual([]);
  });

  it('reports a worker that fails', async () => {
    const { worker, ends } = await started();
    worker.onerror?.({ message: '' });
    expect(ends).toEqual([{ error: 'the worker failed' }]);
    expect(worker.terminated).toBe(1);
  });
});
