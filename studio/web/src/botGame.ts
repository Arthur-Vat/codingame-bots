/* How a game between two bots is shown: nothing of it while the bots play, then the game. */

import type { Session } from './api';
import { sessionMode } from './play';

/** Whether two bots are playing the game now: it has frames, and is neither over nor failed. */
export function botsPlaying(session: Session): boolean {
  return (
    sessionMode(session) === 'bots' &&
    session.status === 'bot_thinking' &&
    session.frames.length > 0
  );
}

/**
 * The session as the page shows it. While two bots play, nothing of the game is shown: only its
 * first frame and the turns of its opening, no result. Every other session (another mode, a game
 * that is over or failed, one still compiling, measuring or rewinding) is given back as it is,
 * the same object.
 */
export function unplayedView(session: Session): Session {
  if (!botsPlaying(session)) return session;
  return {
    ...session,
    frames: session.frames.slice(0, 1),
    turns: session.turns.slice(0, session.opening_turns),
    result: null,
    human_moves: [],
  };
}
