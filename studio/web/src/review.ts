/* What the review screen needs, as pure functions: the one model a session, a saved game and a
   file are turned into, and the timing and the stepping of the playback. */

import type { Answer, GameRecord, RecordPlayer, RecordView, Session, SessionResult } from './api';
import { exportFileName } from './play';
import type { ReviewKind } from './route';
import { thinkLabel } from './setup';

/** One player of a reviewed game. */
export interface ReviewPlayer {
  name: string;
  kind: 'human' | 'bot';
  /** How a bot played, when it is known (think time, iterations, time factor). */
  settings: string | null;
}

/** A reviewed game, whatever it came from. */
export interface ReviewModel {
  kind: ReviewKind;
  /** The session id, the saved game's id, or the id of the loaded file. */
  id: string;
  game: string;
  players: [ReviewPlayer, ReviewPlayer];
  /** The turns the board can show, openings included. */
  turns: Answer[][];
  /** The turns of the record that do not replay (a last invalid answer) and are not listed. */
  hiddenTurns: number;
  /** The board's frames: frame 0 is the position after the opening. */
  frames: unknown[];
  openingTurns: number;
  /** Null while a session is still being played. */
  result: SessionResult | null;
  seed: number;
  /** Where the game comes from, as the info card says it. */
  source: string;
  /** Seconds since the Unix epoch, when known. */
  unixTime: number | null;
  /** Whether the game is in the history already. */
  saved: boolean;
  /** The failure of a session that failed, if any. */
  error: string | null;
}

function sessionPlayer(seat: Session['seats'][number]): ReviewPlayer {
  if (seat.kind === 'human') return { name: seat.name, kind: 'human', settings: null };
  const mode = seat.mode === 'realtime' ? ', real time' : '';
  return { name: seat.name, kind: 'bot', settings: `${thinkLabel(seat.think_ms)}${mode}` };
}

/** The review of a session of the server, which may still be going on. */
export function reviewOfSession(session: Session): ReviewModel {
  return {
    kind: 'session',
    id: session.id,
    game: session.game,
    players: [sessionPlayer(session.seats[0]), sessionPlayer(session.seats[1])],
    turns: session.turns,
    hiddenTurns: 0,
    frames: session.frames,
    openingTurns: session.opening_turns,
    result: session.result,
    seed: session.seed,
    source: 'Played here',
    unixTime: null,
    saved: false,
    error: session.status === 'failed' ? (session.error ?? 'The game failed.') : null,
  };
}

function recordPlayer(player: RecordPlayer): ReviewPlayer {
  let settings: string | null = null;
  if (player.kind === 'bot') {
    if (player.fixed_iters !== null) {
      settings = `${player.fixed_iters} iterations`;
    } else if (player.time_scale !== 1) {
      settings = `time × ${player.time_scale}`;
    }
  }
  return { name: player.name, kind: player.kind, settings };
}

/** Where a record came from. */
export type RecordOrigin =
  { kind: 'saved'; id: string } | { kind: 'file'; id: string; name: string };

/** The review of a record with its frames: a saved game, or a file shown before it is saved. */
export function reviewOfRecord(view: RecordView, origin: RecordOrigin): ReviewModel {
  const record: GameRecord = view.record;
  const shown = Math.min(view.shown_turns, record.turns.length);
  return {
    kind: origin.kind,
    id: origin.id,
    game: record.game,
    players: [recordPlayer(record.players[0]), recordPlayer(record.players[1])],
    turns: record.turns.slice(0, shown),
    hiddenTurns: record.turns.length - shown,
    frames: view.frames,
    openingTurns: view.opening_turns,
    result: { winner: record.winner, end: record.end },
    seed: record.seed,
    source: origin.kind === 'file' ? origin.name : `Saved (${record.source})`,
    unixTime: record.unix_time,
    saved: origin.kind === 'saved',
    error: null,
  };
}

/** The last frame index of a model, or -1 without frames. */
export function lastFrame(model: ReviewModel): number {
  return model.frames.length - 1;
}

/** How long the whole game lasts at 1×, in milliseconds. */
export const GAME_PLAYBACK_MS = 120_000;

/** The playback speeds, as the speed buttons offer them. */
export const SPEEDS = [0.25, 0.5, 1, 2, 4] as const;
export const DEFAULT_SPEED = 1;

/** `0.25×`, `1×`. */
export function speedLabel(speed: number): string {
  return `${speed}×`;
}

/**
 * The time between two frames, in milliseconds: the whole game lasts 120 s at 1×, so the
 * interval is that over the number of steps, and shorter at a higher speed. Null for a game with
 * no step to play.
 */
export function playbackInterval(frameCount: number, speed: number): number | null {
  if (frameCount < 2 || !(speed > 0)) return null;
  return GAME_PLAYBACK_MS / (frameCount - 1) / speed;
}

/** How much the wanted delay may differ from the one in use before the timer is restarted. */
const RETIME_RATIO = 0.25;

/**
 * The delay to keep between two frames. The one in use stays unless the wanted one differs from it
 * by more than a quarter: a game still being played adds a frame every few moments, and each one
 * must not restart the timer.
 */
export function retime(current: number | null, wanted: number | null): number | null {
  if (current === null || wanted === null) return wanted;
  return Math.abs(wanted - current) / current > RETIME_RATIO ? wanted : current;
}

export interface PlaybackState {
  /** The frame on the board. */
  cursor: number;
  playing: boolean;
  speed: number;
}

export type PlaybackAction =
  | { type: 'goto'; frame: number; last: number }
  | { type: 'tick'; last: number }
  | { type: 'toggle'; last: number }
  | { type: 'speed'; speed: number };

export const PLAYBACK_START: PlaybackState = { cursor: 0, playing: false, speed: DEFAULT_SPEED };

function clamp(frame: number, last: number): number {
  return Math.max(0, Math.min(frame, Math.max(last, 0)));
}

/**
 * The playback's state machine. A step moves the cursor and keeps playing; reaching the last frame
 * stops. A change of speed while playing keeps playing. Play at the end starts again from the
 * first frame. A jump by the user keeps playing, unless it lands on the last frame.
 */
export function playbackReducer(state: PlaybackState, action: PlaybackAction): PlaybackState {
  switch (action.type) {
    case 'goto': {
      const cursor = clamp(action.frame, action.last);
      return { ...state, cursor, playing: state.playing && cursor < action.last };
    }
    case 'tick': {
      if (!state.playing) return state;
      const cursor = clamp(state.cursor + 1, action.last);
      return { ...state, cursor, playing: cursor < action.last };
    }
    case 'toggle': {
      if (state.playing) return { ...state, playing: false };
      if (action.last < 1) return state;
      const cursor = state.cursor >= action.last ? 0 : state.cursor;
      return { ...state, cursor, playing: true };
    }
    case 'speed':
      return { ...state, speed: action.speed };
  }
}

/** What a key does in the review: a frame to go to, `'toggle'` for Space, or undefined. */
export function reviewKey(
  key: string,
  cursor: number,
  last: number,
): number | 'toggle' | undefined {
  if (key === ' ') return 'toggle';
  switch (key) {
    case 'ArrowLeft':
      return clamp(cursor - 1, last);
    case 'ArrowRight':
      return clamp(cursor + 1, last);
    case 'Home':
      return 0;
    case 'End':
      return clamp(last, last);
    default:
      return undefined;
  }
}

function two(n: number): string {
  return String(n).padStart(2, '0');
}

/** `2026-10-10 14:05`, in the computer's local time. */
export function formatDate(unixSeconds: number): string {
  const date = new Date(unixSeconds * 1000);
  return `${date.getFullYear()}-${two(date.getMonth() + 1)}-${two(date.getDate())} ${two(date.getHours())}:${two(date.getMinutes())}`;
}

/** The name a record is exported under: `uttt-1-ab…` for a saved game, else the game and seed. */
export function exportName(model: ReviewModel, now: Date): string {
  if (model.kind === 'saved') return `${model.id}.json`;
  if (model.kind === 'file') {
    return model.source.toLowerCase().endsWith('.json') ? model.source : `${model.source}.json`;
  }
  return exportFileName(model.game, model.seed, now);
}
