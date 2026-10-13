/* A stand-in for the analysis routes, for the tests of the live analysis and of the chart: it keeps
   the requests it got and the analyses it was told to stop, and answers with what a test sets. */

import type { AnalysisReport, AnalysisRequest, AnalysisState } from './api';
import type { EngineApi } from './liveAnalysis';

type Body = AnalysisRequest & { game: string };

export function fakeReport(patch: Partial<AnalysisReport> = {}): AnalysisReport {
  return {
    iterations: 20000,
    nodes: 1000,
    to_move: 0,
    proven: null,
    wdl: [0.5, 0.1, 0.4],
    moves: [{ move: '4 4', visits: 100, wdl: [0.5, 0.1, 0.4], proven: null, line: ['4 4', '4 3'] }],
    done: false,
    stop: null,
    ...patch,
  };
}

export class FakeEngine implements EngineApi {
  /** Every request, with the id it got. */
  readonly started: { id: string; body: Body }[] = [];
  readonly stopped: string[] = [];
  /** The state each running analysis answers with. */
  readonly states = new Map<string, AnalysisState>();
  readonly reads: string[] = [];
  /** Called when a request arrives, to give its analysis a first state. */
  onStart: (id: string, body: Body) => void = () => undefined;
  /** When set, `start` waits for `release()`. */
  hold = false;
  private held: (() => void)[] = [];
  /** An error for the next `start` that gets going, if any. */
  startError: Error | null = null;
  getError: Error | null = null;
  private next = 1;

  /** The ids that are neither stopped nor finished with. */
  get live(): string[] {
    return this.started.map((s) => s.id).filter((id) => !this.stopped.includes(id));
  }

  start = async (body: Body): Promise<string> => {
    if (this.hold) await new Promise<void>((resolve) => this.held.push(resolve));
    if (this.startError !== null) {
      const error = this.startError;
      this.startError = null;
      throw error;
    }
    const id = `a${this.next++}`;
    this.started.push({ id, body });
    this.states.set(id, { status: 'running', report: null, error: null });
    this.onStart(id, body);
    return id;
  };

  get = async (id: string): Promise<AnalysisState> => {
    this.reads.push(id);
    if (this.getError !== null) throw this.getError;
    const state = this.states.get(id);
    if (state === undefined || this.stopped.includes(id)) throw new Error('unknown analysis');
    return state;
  };

  stop = async (id: string): Promise<void> => {
    this.stopped.push(id);
  };

  /** Lets the requests that wait go on. */
  release(): void {
    const waiting = this.held;
    this.held = [];
    for (const resume of waiting) resume();
  }

  set(id: string, state: AnalysisState): void {
    this.states.set(id, state);
  }

  report(id: string, patch: Partial<AnalysisReport> = {}): void {
    this.states.set(id, { status: 'running', report: fakeReport(patch), error: null });
  }

  finish(id: string, patch: Partial<AnalysisReport> = {}): void {
    this.states.set(id, {
      status: 'done',
      report: fakeReport({ done: true, stop: 'iterations', ...patch }),
      error: null,
    });
  }
}
