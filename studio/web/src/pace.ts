/* The pace at which a game between two bots is shown, as pure functions. The server plays the
   bots at their own speed; the page shows the frames one by one, `pace` milliseconds apart, or as
   they come ("Max"). The shown game is the real session as it looked when a given frame was its
   latest one. */

import type { Session } from './api';
import { sessionMode } from './play';

/** Milliseconds per move; null is Max, as fast as the bots play. */
export type Pace = number | null;

/** The paces, as the pace bar offers them. */
export const PACES = [250, 500, 1000, 2000, null] as const;
export const DEFAULT_PACE: Pace = 500;

/** `0.25 s`, `1 s`, `Max`. */
export function paceLabel(pace: Pace): string {
  return pace === null ? 'Max' : `${pace / 1000} s`;
}

/** The statuses in which the game is not played move by move: its state is shown as it is. */
const UNPACED: readonly Session['status'][] = ['failed', 'compiling', 'measuring', 'rewinding'];

/**
 * The session as it looked when frame `cursor` was its latest frame: the frames and the turns up
 * to it, the bot to move still thinking, no result. A session that is not a game between two
 * bots, one that is not being played, and one already shown up to its latest frame are given back
 * as they are (the same object).
 */
export function pacedSession(session: Session, cursor: number): Session {
  if (
    sessionMode(session) !== 'bots' ||
    UNPACED.includes(session.status) ||
    cursor >= session.frames.length - 1
  ) {
    return session;
  }
  const at = Math.max(0, cursor);
  // Frame `i` is the position after turn `opening_turns + i - 1`: `at` turns follow the opening.
  const turnCount = session.opening_turns + at;
  const seat = session.turns[turnCount]?.[0]?.seat;
  return {
    ...session,
    frames: session.frames.slice(0, at + 1),
    turns: session.turns.slice(0, turnCount),
    status: 'bot_thinking',
    result: null,
    human_moves: [],
    error: null,
    to_act: seat === undefined ? session.to_act : [seat],
  };
}

/**
 * The frame that is the shown game's latest after a step: the next one, never past `latest`; with
 * Max (null), the latest one.
 */
export function nextPacedCursor(cursor: number, latest: number, pace: Pace): number {
  const end = Math.max(0, latest);
  return pace === null ? end : Math.max(0, Math.min(cursor + 1, end));
}

/**
 * The milliseconds to wait before the next frame is shown: `pace` after the previous one was shown
 * (`shownAt`), so a slow bot adds nothing on top; zero if that moment is past, and for Max.
 */
export function paceDelay(pace: Pace, shownAt: number, now: number): number {
  return pace === null ? 0 : Math.max(0, shownAt + pace - now);
}
