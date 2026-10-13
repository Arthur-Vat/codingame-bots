/* The analyses of the browser engine (ADR 0032): a game's analysis program, WebAssembly in a Web
   Worker, one worker for each analysis, and the state the routes give (studio/README.md, "The
   analysis program" and "API"). The local server keeps the same rules natively
   (studio/server/src/analysis.rs): the same limits, the same states, the same stop.

   - At most MAX_ANALYSES exist at once; starting one more stops and removes the oldest.
   - One that no request has read for ANALYSIS_IDLE_MS is stopped and removed. Every analysis
     request sweeps the idle ones, and each line of a running analysis checks its own reads, so
     that nothing needs a timer.
   - Stopping ends the worker (`terminate`) at once: lines that arrive after it are ignored. */

import { HttpError } from './http';

export const MAX_ANALYSES = 4;
export const ANALYSIS_IDLE_MS = 10_000;

/** How a running analysis ended, as the page learns it. */
export type AnalysisEnd = { exit: number } | { error: string };

/** A running analysis program. */
export interface AnalysisRun {
  /** Ends the program at once; safe to call twice. */
  terminate: () => void;
}

/**
 * Starts the program `module` with `request` as its whole standard input. `onLine` gets each line
 * of its standard output, without the newline; `onEnd` how it ended (once, and not after
 * `terminate`).
 */
export type AnalysisFactory = (
  module: WebAssembly.Module,
  request: string,
  onLine: (line: string) => void,
  onEnd: (end: AnalysisEnd) => void,
) => AnalysisRun;

/** The body of `GET /api/analysis/{id}`. */
export interface AnalysisView {
  status: 'running' | 'done' | 'error';
  report: unknown;
  error: string | null;
}

interface Entry {
  id: string;
  view: AnalysisView;
  lastRead: number;
  stopped: boolean;
  run: AnalysisRun | null;
}

export interface AnalysesDeps {
  factory: AnalysisFactory;
  /** Milliseconds since the Unix epoch. */
  now: () => number;
  /** A fresh id; asked again when it is in use. */
  newId: () => string;
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function unknown(): HttpError {
  return new HttpError(404, 'unknown analysis');
}

export class Analyses {
  /** In order of creation, the oldest first. */
  private readonly entries: Entry[] = [];
  private readonly deps: AnalysesDeps;

  constructor(deps: AnalysesDeps) {
    this.deps = deps;
  }

  /** The ids of the analyses that exist, the oldest first. */
  ids(): string[] {
    return this.entries.map((entry) => entry.id);
  }

  /** Starts an analysis of `request` (the program's JSON, as text) and returns its id. */
  start(module: WebAssembly.Module, request: string): string {
    this.sweep();
    for (let oldest = this.entries[0]; oldest !== undefined; oldest = this.entries[0]) {
      if (this.entries.length < MAX_ANALYSES) break;
      this.stop(oldest);
    }
    let id = this.deps.newId();
    while (this.entries.some((entry) => entry.id === id)) id = this.deps.newId();
    const entry: Entry = {
      id,
      view: { status: 'running', report: null, error: null },
      lastRead: this.deps.now(),
      stopped: false,
      run: null,
    };
    this.entries.push(entry);
    try {
      entry.run = this.deps.factory(
        module,
        request,
        (line) => this.onLine(entry, line),
        (end) => this.onEnd(entry, end),
      );
    } catch (error) {
      this.fail(entry, `cannot start the analysis: ${(error as Error).message}`);
    }
    return id;
  }

  /** The state for `GET`, which counts as a read. */
  read(id: string): AnalysisView {
    this.sweep();
    const entry = this.entries.find((candidate) => candidate.id === id);
    if (entry === undefined) throw unknown();
    entry.lastRead = this.deps.now();
    return { ...entry.view };
  }

  /** Stops and removes the analysis `id`. */
  remove(id: string): void {
    this.sweep();
    const entry = this.entries.find((candidate) => candidate.id === id);
    if (entry === undefined) throw unknown();
    this.stop(entry);
  }

  /** Stops and removes the analyses nobody has read for too long. */
  sweep(): void {
    for (const entry of [...this.entries]) {
      if (this.idle(entry)) this.stop(entry);
    }
  }

  private idle(entry: Entry): boolean {
    return this.deps.now() - entry.lastRead > ANALYSIS_IDLE_MS;
  }

  private stop(entry: Entry): void {
    entry.stopped = true;
    const at = this.entries.indexOf(entry);
    if (at >= 0) this.entries.splice(at, 1);
    entry.run?.terminate();
  }

  private fail(entry: Entry, message: string): void {
    entry.view = { status: 'error', report: entry.view.report, error: message };
  }

  /** What the program wrote: the latest report, or the end of the analysis. */
  private onLine(entry: Entry, line: string): void {
    if (entry.stopped || entry.view.status !== 'running') return;
    let parsed: unknown;
    try {
      parsed = JSON.parse(line);
    } catch {
      parsed = null;
    }
    if (isObject(parsed) && 'error' in parsed) {
      const { error } = parsed;
      this.fail(entry, typeof error === 'string' ? error : JSON.stringify(error));
    } else if (isObject(parsed) && isObject(parsed.report)) {
      entry.view = {
        status: parsed.report.done === true ? 'done' : 'running',
        report: parsed.report,
        error: null,
      };
    } else {
      this.fail(entry, 'the analysis program wrote an unexpected line');
    }
    if (entry.view.status !== 'running') {
      // Nothing more is wanted: end the worker, and keep the state for the reads.
      entry.run?.terminate();
    } else if (this.idle(entry)) {
      this.stop(entry);
    }
  }

  private onEnd(entry: Entry, end: AnalysisEnd): void {
    if (entry.stopped || entry.view.status !== 'running') return;
    this.fail(
      entry,
      'error' in end || end.exit !== 0
        ? 'the analysis program crashed'
        : 'the analysis program ended without a final report',
    );
  }
}
