/* The live analysis of the position on the analysis board, as a state machine with no React in it:
   it starts a request, polls it, stops it after 2 seconds unless asked to keep thinking, and
   ignores every answer of a request that was stopped (a generation counter). */

import {
  getAnalysis,
  startAnalysis,
  stopAnalysis,
  type AnalysisReport,
  type AnalysisRequest,
  type AnalysisState,
} from './api';
import { errorMessage } from './browser';
import { startPoller } from './poller';

/** The three routes of the analysis, as the page calls them (a fake one stands in for tests). */
export interface EngineApi {
  start: (body: AnalysisRequest & { game: string }) => Promise<string>;
  get: (id: string) => Promise<AnalysisState>;
  stop: (id: string) => Promise<void>;
}

export const engineApi: EngineApi = { start: startAnalysis, get: getAnalysis, stop: stopAnalysis };

/** How long to wait between two reads of a running analysis. */
export const POLL_MS = 150;
/** How long the engine thinks about a position without Keep thinking. */
export const THINK_MS = 2000;

/** What the live request asks: as long as the program allows, a report every 20,000 iterations. */
export function liveRequest(game: string, moves: string[]): AnalysisRequest & { game: string } {
  return { game, moves, iterations: 100_000_000, report_every: 20_000, depth: 10, seed: '1' };
}

export type LiveStatus =
  /** Nothing is asked: the engine is off, or the position cannot be analysed. */
  | 'idle'
  /** Thinking, or about to: the request is on its way. */
  | 'running'
  /** Stopped by the time limit or by the user; the report stays. */
  | 'stopped'
  /** The program stopped itself: its iterations, its node limit, the position proven or over. */
  | 'done'
  | 'error';

export interface LiveState {
  status: LiveStatus;
  /** What position this is the state of (the caller's key); the state of another is not to be shown. */
  position: string | null;
  report: AnalysisReport | null;
  error: string | null;
  /** The time thinking on this position, across stops and Keep thinking. */
  elapsedMs: number;
  /** Keep thinking is on: no time limit. */
  keep: boolean;
}

export const IDLE: LiveState = {
  status: 'idle',
  position: null,
  report: null,
  error: null,
  elapsedMs: 0,
  keep: false,
};

const ignore = (): void => undefined;

export class LiveAnalysis {
  private generation = 0;
  private id: string | null = null;
  private stopPolling: (() => void) | null = null;
  private limit: ReturnType<typeof setTimeout> | undefined;
  private request: (AnalysisRequest & { game: string }) | null = null;
  private state: LiveState = IDLE;
  private startedAt = 0;
  /** Time spent on this position by runs that were stopped, and what they had found. */
  private carriedMs = 0;
  private carried: AnalysisReport | null = null;
  /** A request is on its way, and the newest one that waits for it. */
  private sending = false;
  private wanted: { generation: number; request: AnalysisRequest & { game: string } } | null = null;

  private readonly api: EngineApi;
  private readonly onChange: (state: LiveState) => void;
  private readonly now: () => number;

  constructor(
    api: EngineApi,
    onChange: (state: LiveState) => void,
    now: () => number = () => Date.now(),
  ) {
    this.api = api;
    this.onChange = onChange;
    this.now = now;
  }

  get current(): LiveState {
    return this.state;
  }

  /** Starts on a position (`position` names it): whatever ran before is stopped. */
  begin(position: string, request: AnalysisRequest & { game: string }): void {
    this.halt();
    this.request = request;
    this.carriedMs = 0;
    this.carried = null;
    this.run(position, false);
  }

  /** Stops and forgets, with no state to show: the engine is off, the position changed to one that cannot be asked, or the page left. */
  reset(notify = true): void {
    this.halt();
    this.request = null;
    this.carriedMs = 0;
    this.carried = null;
    this.state = IDLE;
    if (notify) this.onChange(this.state);
  }

  /**
   * Keep thinking (true) or Stop (false). Keeping a run that is going removes its time limit; a run
   * that stopped by the time limit, or by Stop, starts again and goes on from the time and the
   * report it had. Stop ends the run, and the report stays.
   */
  setKeep(keep: boolean): void {
    const { status, position } = this.state;
    if (keep) {
      if (status === 'running') {
        clearTimeout(this.limit);
        this.limit = undefined;
        this.emit({ keep: true });
      } else if (status === 'stopped' && this.request !== null && position !== null) {
        this.carriedMs = this.state.elapsedMs;
        this.carried = this.state.report;
        this.run(position, true);
      }
    } else if (status === 'running') {
      this.stopNow();
    } else if (this.state.keep) {
      this.emit({ keep: false });
    }
  }

  private run(position: string, keep: boolean): void {
    const request = this.request;
    if (request === null) return;
    this.halt();
    const generation = ++this.generation;
    this.startedAt = this.now();
    this.state = {
      status: 'running',
      position,
      report: this.carried,
      error: null,
      elapsedMs: this.carriedMs,
      keep,
    };
    this.onChange(this.state);
    if (!keep) {
      this.limit = setTimeout(() => this.timeUp(generation), THINK_MS);
    }
    this.wanted = { generation, request };
    this.send();
  }

  /**
   * Sends the newest request, one at a time: while a request is on its way the next ones wait, and
   * only the last of them is sent, so that stepping quickly through a game never leaves many
   * analyses to stop (the program keeps at most 4).
   */
  private send(): void {
    const wanted = this.wanted;
    if (this.sending || wanted === null) return;
    this.wanted = null;
    const { generation, request } = wanted;
    if (generation !== this.generation) return;
    this.sending = true;
    this.api.start(request).then(
      (id) => {
        this.sending = false;
        this.started(generation, id);
        this.send();
      },
      (error: unknown) => {
        this.sending = false;
        this.fail(generation, errorMessage(error));
        this.send();
      },
    );
  }

  private started(generation: number, id: string): void {
    if (generation !== this.generation) {
      // Stopped while the request was on its way: it is no longer wanted.
      this.api.stop(id).catch(ignore);
      return;
    }
    this.id = id;
    this.stopPolling = startPoller<AnalysisState>({
      fetch: () => this.api.get(id),
      delayAfter: (value) => (value.status === 'running' ? POLL_MS : null),
      delayAfterError: () => null,
      onValue: (value) => this.onValue(generation, value),
      onError: (error) => this.fail(generation, errorMessage(error)),
    });
  }

  private timeUp(generation: number): void {
    if (generation !== this.generation || this.state.keep) return;
    this.stopNow();
  }

  private onValue(generation: number, value: AnalysisState): void {
    if (generation !== this.generation) return;
    if (value.status === 'error') {
      this.fail(generation, value.error ?? 'The analysis failed.');
      return;
    }
    const report = this.better(value.report);
    const done = value.status === 'done' || value.report?.done === true;
    if (done) {
      // The program is finished; free its slot, the report stays.
      this.halt();
      this.state = {
        ...this.state,
        status: 'done',
        report,
        elapsedMs: this.elapsed(),
        keep: false,
      };
      this.onChange(this.state);
      return;
    }
    this.emit({ report, elapsedMs: this.elapsed() });
  }

  /** A restarted run begins again from the first iteration: what it reports is shown once it has gone past what was found. */
  private better(report: AnalysisReport | null): AnalysisReport | null {
    const before = this.carried;
    if (report === null) return this.state.report;
    if (before === null || report.done || report.iterations >= before.iterations) return report;
    return this.state.report;
  }

  private fail(generation: number, message: string): void {
    if (generation !== this.generation) return;
    this.halt();
    this.state = {
      ...this.state,
      status: 'error',
      error: message,
      elapsedMs: this.elapsed(),
      keep: false,
    };
    this.onChange(this.state);
  }

  /** Ends the run that goes on, keeping what it found. */
  private stopNow(): void {
    const elapsed = this.elapsed();
    this.halt();
    this.state = { ...this.state, status: 'stopped', elapsedMs: elapsed, keep: false };
    this.onChange(this.state);
  }

  private elapsed(): number {
    return this.carriedMs + Math.max(0, this.now() - this.startedAt);
  }

  private emit(patch: Partial<LiveState>): void {
    this.state = { ...this.state, ...patch };
    this.onChange(this.state);
  }

  /** Stops what runs: the answers still to come are ignored, and the program is told to stop. */
  private halt(): void {
    this.generation += 1;
    this.stopPolling?.();
    this.stopPolling = null;
    clearTimeout(this.limit);
    this.limit = undefined;
    const id = this.id;
    this.id = null;
    if (id !== null) this.api.stop(id).catch(ignore);
  }
}
