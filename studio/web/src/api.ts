/** A typed client of the studio server (`studio/server`), see studio/README.md. */

export interface GameSummary {
  id: string;
  name: string;
  /** Release names, newest first. */
  releases: string[];
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

async function call<T>(method: string, path: string, body?: unknown): Promise<T> {
  let response: Response;
  try {
    response = await fetch(path, {
      method,
      headers: body === undefined ? undefined : { 'Content-Type': 'application/json' },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
  } catch {
    throw new ApiError('The server cannot be reached.', true);
  }
  if (response.status === 204) {
    return undefined as T;
  }
  let json: unknown;
  try {
    json = await response.json();
  } catch {
    throw new ApiError(
      `The server sent something that is not JSON (HTTP ${response.status}).`,
      true,
    );
  }
  if (!response.ok) {
    throw new ApiError(errorText(json) ?? `HTTP ${response.status}`, false);
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

/** The saved record, as it is exported to a file. */
export function getHistoryRecord(id: string): Promise<unknown> {
  return call('GET', `/api/history/${encodeURIComponent(id)}`);
}

/** Saves a record in the history. `duplicate` is true when it was there already. */
export function postHistory(record: unknown): Promise<{ id: string; duplicate: boolean }> {
  return call('POST', '/api/history', record);
}
