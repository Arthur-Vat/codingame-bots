/* The side line of the analysis board, as pure functions: the one variation that can be explored at
   a time, branching from a frame of the game. The page keeps it in its state and asks the rules
   (`POST /api/view`, in both engines) for the frames; nothing here knows a game's rules. */

import type { AnalysisModel } from './analysis';
import { analysisKey } from './analysis';
import { lineTokens, MAX_REQUEST_MOVES, requestMoves } from './analysisView';
import type { Answer, GameRecord, RecordView } from './api';

/** The side line on the board. */
export interface Variation {
  /** The frame of the game it branches from. */
  from: number;
  /** Its moves, as the analysis program writes them (`4 4`). */
  moves: string[];
  /** `frames[i]` is the frame after `moves[0..=i]`. */
  frames: unknown[];
  /** 0 is the branch position (frame `from`); `c` is the position after `moves[c - 1]`. */
  cursor: number;
}

/** A side line whose frames are still to be asked for. */
export interface Branch {
  from: number;
  moves: string[];
  cursor: number;
}

/** Where the board is: a frame of the game, or inside a side line (`from` is its `from`). */
export interface Place {
  frame: number;
  variation: Variation | null;
}

/** What playing a move, or a line, on the board comes to. */
export type Plan =
  /** The game's own next move: step to this frame of the game. */
  | { kind: 'frame'; frame: number }
  /** A move the side line has already: step to this cursor. */
  | { kind: 'cursor'; cursor: number }
  /** A new side line, or a new end for it: the rules have to replay it. */
  | { kind: 'branch'; branch: Branch }
  /** Not allowed; the message says why. */
  | { kind: 'refused'; message: string }
  /** Nothing to do. */
  | { kind: 'ignored' };

type Played = Pick<AnalysisModel, 'turns' | 'openingTurns'>;

function sameMove(a: string, b: string): boolean {
  return a.trim() === b.trim();
}

/** The answer text of the game's turn after frame `frame`, or null when the game has no such turn. */
export function mainNext(model: Played, frame: number): string | null {
  const turn = model.turns[model.openingTurns + frame];
  const answer = turn?.length === 1 ? turn[0] : undefined;
  return answer === undefined ? null : answer.lines.join('\n');
}

/**
 * What playing `path` (one move from the board, or a whole engine line) on `place` does. When the
 * first move is the next move of the line the board is on (the game's, or the side line's) the board
 * steps one move along it. Otherwise the moves after the board's position are replaced by `path`
 * (a new side line on the game), and the cursor goes just after the first move. A path of more
 * moves than the engine takes is refused.
 */
export function planPath(model: Played, place: Place, path: readonly string[]): Plan {
  const first = path[0];
  if (first === undefined || path.some((move) => typeof move !== 'string' || move.trim() === '')) {
    return { kind: 'ignored' };
  }
  const { variation } = place;
  if (variation === null) {
    const next = mainNext(model, place.frame);
    if (next !== null && sameMove(next, first)) {
      return { kind: 'frame', frame: place.frame + 1 };
    }
    return branch(model, place.frame, [...path], 1);
  }
  const at = variation.cursor;
  const next = variation.moves[at];
  if (next !== undefined && sameMove(next, first)) {
    return { kind: 'cursor', cursor: at + 1 };
  }
  return branch(model, variation.from, [...variation.moves.slice(0, at), ...path], at + 1);
}

function branch(model: Played, from: number, moves: string[], cursor: number): Plan {
  if (model.openingTurns + from + moves.length > MAX_REQUEST_MOVES) {
    return {
      kind: 'refused',
      message: `A side line stops at ${MAX_REQUEST_MOVES} moves in all.`,
    };
  }
  return { kind: 'branch', branch: { from, moves, cursor } };
}

/** The seat that plays the first move of a side line on frame `from`: the other one than the last turn's, seat 0 without one. */
function firstSeat(turns: readonly Answer[][]): number {
  const last = turns[turns.length - 1]?.[0];
  return last === undefined ? 0 : 1 - (last.seat % 2);
}

/**
 * The record that replays the game up to frame `from` and then `moves`, for `POST /api/view`. It
 * has no opening of its own (`opening_plies: 0`): the opening turns are in `model.turns` as
 * ordinary answers, and replaying them as moves gives the same position whatever the seed.
 */
export function variationRecord(
  model: Pick<AnalysisModel, 'game' | 'turns' | 'openingTurns'>,
  from: number,
  moves: readonly string[],
): GameRecord {
  const before = model.turns.slice(0, model.openingTurns + from);
  const seat = firstSeat(before);
  const human = (name: string) => ({
    name,
    kind: 'human' as const,
    bot_seed: null,
    command: null,
    time_scale: 1,
    fixed_iters: null,
  });
  return {
    format: 1,
    game: model.game,
    seed: 0,
    opening_plies: 0,
    unix_time: 0,
    source: 'studio',
    players: [human('X'), human('O')],
    turns: [
      ...before,
      ...moves.map((move, index) => [{ seat: (seat + index) % 2, lines: [move], ms: 0 }]),
    ],
    end: { kind: 'aborted', reason: 'variation' },
    winner: null,
  };
}

/** The frames of a side line, from the view of its record (the last `count` ones). */
export function variationFrames(view: RecordView, count: number, before: number): unknown[] {
  // The record has no opening: one frame for the start and one for each turn.
  const frames = Array.isArray(view.frames) ? view.frames : [];
  if (view.shown_turns !== before + count || frames.length !== 1 + before + count) {
    throw new Error('The side line could not be shown.');
  }
  return frames.slice(frames.length - count);
}

/**
 * Asks the rules for the frames of a side line: `post` is `postView` of the API client, which both
 * engines answer. A refused move rejects with the engine's message.
 */
export async function fetchVariation(
  model: Pick<AnalysisModel, 'game' | 'turns' | 'openingTurns'>,
  branch: Branch,
  post: (record: unknown) => Promise<RecordView>,
): Promise<Variation> {
  const record = variationRecord(model, branch.from, branch.moves);
  const view = await post(record);
  const before = record.turns.length - branch.moves.length;
  return {
    from: branch.from,
    moves: branch.moves,
    frames: variationFrames(view, branch.moves.length, before),
    cursor: branch.cursor,
  };
}

/** The frame the board shows: the game's frame `from` at cursor 0, else the frame after the move at the cursor. */
export function shownFrame(model: Pick<AnalysisModel, 'frames'>, variation: Variation): unknown {
  return variation.cursor <= 0
    ? model.frames[variation.from]
    : variation.frames[variation.cursor - 1];
}

/** The moves of the shown position, for the engine; null when the position cannot be asked. */
export function shownMoves(model: Played, variation: Variation): string[] | null {
  const before = requestMoves(model, variation.from);
  return before === null ? null : [...before, ...variation.moves.slice(0, variation.cursor)];
}

/** The side line with its cursor moved to `cursor`, kept within its moves. */
export function withCursor(variation: Variation, cursor: number): Variation {
  const clamped = Math.max(0, Math.min(cursor, variation.moves.length));
  return clamped === variation.cursor ? variation : { ...variation, cursor: clamped };
}

/** What a key does in a side line: a cursor to go to, `'toggle'` for Space (which does nothing here), or undefined. */
export function variationKey(key: string, variation: Variation): number | 'toggle' | undefined {
  return analysisKey(key, variation.cursor, variation.moves.length);
}

/** A piece of the strip: a move number, or a move with its index in the side line. */
export type StripToken =
  { kind: 'number'; text: string } | { kind: 'move'; text: string; index: number };

/** The side line written like an engine line, numbered from the start of the game. */
export function stripTokens(
  model: Played,
  variation: Pick<Variation, 'from' | 'moves'>,
): StripToken[] {
  let index = 0;
  return lineTokens(variation.moves, model.openingTurns + variation.from).map((token) =>
    token.kind === 'number' ? token : { kind: 'move', text: token.text, index: index++ },
  );
}
