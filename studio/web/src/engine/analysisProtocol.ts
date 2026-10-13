/* What the page and an analysis worker say to each other (analyses.ts, analysisWorker.ts). */

/** The message from the page to the worker: the compiled program and its whole standard input. */
export interface AnalysisStart {
  module: WebAssembly.Module;
  request: string;
}

/** Messages from the worker to the page: a line of standard output, then how the program ended. */
export type AnalysisMessage = { line: string } | { exit: number } | { error: string };
