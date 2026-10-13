/** A typed client of the studio API, see studio/README.md: the local server (`studio/server`) or
    the browser engine (`src/engine/`), which answer the same routes. */

import { engineKind } from './engine/detect';

export interface GameSummary {
  id: string;
  name: string;
  /** Release names, newest first. */
  releases: string[];
  /** Whether the game has an analysis program (ADR 0032). */
  analysis: boolean;
}

export type BotMode = 'fixed' | 'realtime';

export type SeatRequest =
  | { kind: 'human'; name: string }
  | { kind: 'bot'; release: string; think_ms: number; mode: BotMode };

export interface SessionRequest {
  game: string;
  seed: number;
  opening_plies: number;
  seats: [SeatRequest, SeatRequest];
}

export type SeatInfo =
  | { kind: 'human'; name: string }
  | {
      kind: 'bot';
      name: string;
      release: string;
      think_ms: number;
      mode: BotMode;
      fixed_iters: number | null;
    };

export type SessionStatus =
  'compiling' | 'measuring' | 'rewinding' | 'bot_thinking' | 'waiting_human' | 'over' | 'failed';

export interface Answer {
  seat: number;
  lines: string[];
  ms: number;
}

/** The server's `EndReason`, tagged by `kind`. */
export type EndReason =
  | { kind: 'finished' }
  | { kind: 'timeout'; seat: number; limit_ms: number }
  | { kind: 'crash'; seat: number; detail: string }
  | { kind: 'invalid'; seat: number; reason: string }
  | { kind: 'resigned'; seat: number }
  | { kind: 'aborted'; reason: string };

export interface SessionResult {
  /** The winning seat, null for a draw. */
  winner: number | null;
  end: EndReason;
}

export interface Progress {
  done: number;
  /** Null while the total is not known (measuring a release). */
  total: number | null;
}

export interface Session {
  id: string;
  game: string;
  seed: number;
  opening_plies: number;
  seats: [SeatInfo, SeatInfo];
  status: SessionStatus;
  error: string | null;
  to_act: number[];
  opening_turns: number;
  /** One entry per turn, each holding the answers given in it. */
  turns: Answer[][];
  /** The game's board frames: the board renderer reads them, this client does not. */
  frames: unknown[];
  human_moves: unknown[];
  result: SessionResult | null;
  progress: Progress | null;
}

/** A failed call. `unreachable` is true when there was no usable answer from the server. */
export class ApiError extends Error {
  unreachable: boolean;

  constructor(message: string, unreachable: boolean) {
    super(message);
    this.name = 'ApiError';
    this.unreachable = unreachable;
  }
}

/** What a route answered: the status and the body as text. */
interface Reply {
  status: number;
  text: string;
}

/**
 * Sends a request to the engine in use: the local server's API over `fetch`, or the browser engine
 * (`src/engine/`), which answers the same routes with the same JSON.
 */
async function transport(
  method: string,
  path: string,
  bodyText: string | undefined,
): Promise<Reply> {
  if (engineKind() === 'browser') {
    const { browserEngine } = await import('./engine/browser');
    const reply = await browserEngine.handle(method, path, bodyText);
    return { status: reply.status, text: reply.bodyText };
  }
  let response: Response;
  try {
    response = await fetch(path, {
      method,
      headers: bodyText === undefined ? undefined : { 'Content-Type': 'application/json' },
      body: bodyText,
    });
  } catch {
    throw new ApiError('The server cannot be reached.', true);
  }
  if (response.status === 204) {
    return { status: 204, text: '' };
  }
  try {
    return { status: response.status, text: await response.text() };
  } catch {
    throw new ApiError('The server cannot be reached.', true);
  }
}

async function call<T>(method: string, path: string, body?: unknown): Promise<T> {
  return callText(method, path, body === undefined ? undefined : JSON.stringify(body));
}

/** `call` with a body that is already text, sent exactly as it is. */
async function callText<T>(method: string, path: string, bodyText: string | undefined): Promise<T> {
  const answer = await transport(method, path, bodyText);
  if (answer.status === 204) {
    return undefined as T;
  }
  let json: unknown;
  try {
    json = JSON.parse(answer.text);
  } catch {
    const who = engineKind() === 'browser' ? 'The browser engine' : 'The server';
    throw new ApiError(`${who} sent something that is not JSON (HTTP ${answer.status}).`, true);
  }
  if (answer.status < 200 || answer.status >= 300) {
    throw new ApiError(errorText(json) ?? `HTTP ${answer.status}`, false);
  }
  return json as T;
}

/** The text of a `{"error": ...}` body, if it is one. */
export function errorText(json: unknown): string | null {
  if (typeof json === 'object' && json !== null && 'error' in json) {
    const { error } = json as { error: unknown };
    return typeof error === 'string' ? error : null;
  }
  return null;
}

export function listGames(): Promise<GameSummary[]> {
  return call('GET', '/api/games');
}

/** Starts a game and returns the new session's id. */
export async function createSession(request: SessionRequest): Promise<string> {
  const { id } = await call<{ id: string }>('POST', '/api/sessions', request);
  return id;
}

export function getSession(id: string): Promise<Session> {
  return call('GET', `/api/sessions/${encodeURIComponent(id)}`);
}

export async function deleteSession(id: string): Promise<void> {
  await call('DELETE', `/api/sessions/${encodeURIComponent(id)}`);
}

/** Plays `human_moves[index]` for the human in `seat`. Answers with the session. */
export function postMove(id: string, seat: number, index: number): Promise<Session> {
  return call('POST', `/api/sessions/${encodeURIComponent(id)}/move`, { seat, index });
}

/** Keeps the first `turns` turns. Answers with the session. */
export function postTakeback(id: string, turns: number): Promise<Session> {
  return call('POST', `/api/sessions/${encodeURIComponent(id)}/takeback`, { turns });
}

export type EndKind = 'resign' | 'timeout';

/** Ends the game with `seat` resigning or out of time; the other seat wins. */
export function postEnd(id: string, seat: number, reason: EndKind): Promise<Session> {
  return call('POST', `/api/sessions/${encodeURIComponent(id)}/end`, { seat, reason });
}

/** Saves the session's record in the history. `duplicate` is true when it was there already. */
export function saveSession(id: string): Promise<{ id: string; duplicate: boolean }> {
  return call('POST', `/api/sessions/${encodeURIComponent(id)}/save`, {});
}

/** The session's game record (format 1), as it is exported to a file. */
export function getRecord(id: string): Promise<unknown> {
  return call('GET', `/api/sessions/${encodeURIComponent(id)}/record`);
}

/** A player of a game record. */
export interface RecordPlayer {
  name: string;
  kind: 'human' | 'bot';
  bot_seed: number | null;
  command: string | null;
  time_scale: number;
  fixed_iters: number | null;
}

/** A game record, format 1 (ADR 0027). */
export interface GameRecord {
  format: number;
  game: string;
  seed: number;
  opening_plies: number;
  /** Seconds since the Unix epoch. */
  unix_time: number;
  source: string;
  players: [RecordPlayer, RecordPlayer];
  turns: Answer[][];
  end: EndReason;
  winner: number | null;
}

/** A record with the game's frames: `GET /api/history/{id}/view` and `POST /api/view`. */
export interface RecordView {
  record: GameRecord;
  frames: unknown[];
  opening_turns: number;
  /** The turns that replay: all of them, or all but a final invalid one. */
  shown_turns: number;
}

export function getHistoryView(id: string): Promise<RecordView> {
  return call('GET', `/api/history/${encodeURIComponent(id)}/view`);
}

/** The view of a record that is not saved. */
export function postView(record: unknown): Promise<RecordView> {
  return call('POST', '/api/view', record);
}

/**
 * The view of a record given as text, sent as it is: numbers beyond 2^53 (a bot's seed) would
 * change if the text were parsed in JavaScript and written again.
 */
export function postViewText(recordText: string): Promise<RecordView> {
  return callText('POST', '/api/view', recordText);
}

/** The saved record, as it is exported to a file. */
export function getHistoryRecord(id: string): Promise<unknown> {
  return call('GET', `/api/history/${encodeURIComponent(id)}`);
}

/** One saved game in the history list. */
export interface HistoryEntry {
  id: string;
  game: string;
  unix_time: number;
  /** The names of the players of seat 0 and seat 1. */
  players: [string, string];
  winner: number | null;
  /** `finished`, `timeout`, `crash`, `invalid`, `aborted` or `resigned`. */
  end: string;
  turns: number;
  source: string;
}

/** The saved games, newest first. `query` is `''` or a query string starting with `?`. */
export function listHistory(query: string): Promise<HistoryEntry[]> {
  return call('GET', `/api/history${query}`);
}

/**
 * The rows that pass the filter `query` (`''` or a query string starting with `?`, as for
 * `listHistory`), newest first. The rows are the caller's, not the saved games: the arena games.
 */
export function filterRows(query: string, rows: readonly HistoryEntry[]): Promise<HistoryEntry[]> {
  return call('POST', `/api/filter${query}`, rows);
}

/** Deletes a saved game. */
export async function deleteHistory(id: string): Promise<void> {
  await call('DELETE', `/api/history/${encodeURIComponent(id)}`);
}

/** Saves a record in the history. `duplicate` is true when it was there already. */
export function postHistory(record: unknown): Promise<{ id: string; duplicate: boolean }> {
  return call('POST', '/api/history', record);
}

/** Saves a record given as text, sent as it is (see `postViewText`). */
export function postHistoryText(recordText: string): Promise<{ id: string; duplicate: boolean }> {
  return callText('POST', '/api/history', recordText);
}

/**
 * A request of the analysis program (studio/README.md, "The analysis program"): the moves played
 * from the start, each the answer the player printed in that turn (lines joined by "\n"), and the
 * program's limits. A field left out has the program's default.
 */
export interface AnalysisRequest {
  moves: string[];
  /** From 1 to 100000000; the search stops after this many. */
  iterations?: number;
  /** From 1000 to 10000000: a report after every this many iterations. */
  report_every?: number;
  /** From 1 to 40: the longest line reported. */
  depth?: number;
  /** A decimal string of an unsigned 64-bit number. */
  seed?: string;
}

/** `[seat 0 wins, draw, seat 1 wins]`, from 0 to 1 and adding up to 1 in ten-thousandths. */
export type Chances = [number, number, number];

/** A move the search tried. */
export interface AnalysisMove {
  /** The answer text, for example "4 4". */
  move: string;
  visits: number;
  /** Null for a move with no visit. */
  wdl: Chances | null;
  /** The seat proven to win after this move, 0 or 1, else null. */
  proven: 0 | 1 | null;
  /** The move, then the most visited answer at each ply. */
  line: string[];
}

/** One report of the program: the value under `report` in a line of its answer. */
export interface AnalysisReport {
  iterations: number;
  nodes: number;
  /** The seat to move; null when the game is over. */
  to_move: 0 | 1 | null;
  proven: 0 | 1 | null;
  wdl: Chances;
  /** Every legal move, the one the search would play first, then by visits. */
  moves: AnalysisMove[];
  done: boolean;
  /** On the last report, why it stopped. */
  stop: 'iterations' | 'proven' | 'nodes' | 'over' | null;
}

/** The body of `GET /api/analysis/{id}`. */
export interface AnalysisState {
  status: 'running' | 'done' | 'error';
  /** The latest report; null before the first. */
  report: AnalysisReport | null;
  /** The program's error message (or that it crashed); null unless the status is `error`. */
  error: string | null;
}

/** Starts an analysis of `body.game` and returns its id. */
export async function startAnalysis(body: AnalysisRequest & { game: string }): Promise<string> {
  const { id } = await call<{ id: string }>('POST', '/api/analysis', body);
  return id;
}

/** The latest state of an analysis. Reading it keeps the analysis alive (10 s without a read). */
export function getAnalysis(id: string): Promise<AnalysisState> {
  return call('GET', `/api/analysis/${encodeURIComponent(id)}`);
}

/** Stops an analysis and forgets it. */
export async function stopAnalysis(id: string): Promise<void> {
  await call('DELETE', `/api/analysis/${encodeURIComponent(id)}`);
}
