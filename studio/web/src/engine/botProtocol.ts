/* What the page and a bot's worker say to each other (bot.ts, botWorker.ts). */

/** Messages from the page to the worker. */
export interface BotStart {
  module: WebAssembly.Module;
  sab: SharedArrayBuffer;
  env: Record<string, string>;
}

/** Messages from the worker to the page. */
export type BotMessage = { ready: true } | { line: string } | { exit: number } | { error: string };

/** Bytes before the data in the shared buffer: the control Int32Array [unused, length]. */
export const HEADER_BYTES = 8;
/** The size of the data part, in bytes: longer inputs are written in several pieces. */
export const DATA_BYTES = 65536;
