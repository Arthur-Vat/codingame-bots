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
