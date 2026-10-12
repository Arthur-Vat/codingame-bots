import { useEffect, useEffectEvent, useRef, useState } from 'react';
import type { Session } from './api';
import { nextPacedCursor, paceDelay, type Pace } from './pace';
import { sessionMode } from './play';

interface Held {
  id: string;
  cursor: number;
}

/**
 * The frame that is the latest of the game shown for `session`: a game between two bots is shown
 * one frame at a time, `pace` ms apart (Max: as they come). A frame is shown `pace` ms after the
 * previous one, or when it arrives if that is later. A new pace takes effect at once, timed from
 * the last step. The session `launchedId` was started from this page, so it is shown from its
 * first frame; any other is shown from the latest frame of the first poll that has frames (a game
 * already over is its review at once). Other games, and a failed one, are shown as they are. The
 * pacing goes on while the page is hidden. Its state is kept for one session id.
 */
export function usePacedCursor(
  session: Session | null,
  pace: Pace,
  launchedId: string | null,
): number {
  const [held, setHeld] = useState<Held | null>(null);
  // When the shown frame was shown: the next step is timed from it.
  const shownAt = useRef(0);

  const id = session?.id ?? null;
  const latest = session === null ? -1 : session.frames.length - 1;

  // A state adjusted while rendering: it only changes when the session does, or at its first frames.
  let current = held !== null && held.id === id ? held : null;
  if (session !== null && current === null) {
    const start = launchedId === session.id ? 0 : latest >= 0 ? latest : null;
    if (start !== null) {
      current = { id: session.id, cursor: start };
      setHeld(current);
    }
  }
  const cursor = current === null ? 0 : current.cursor;

  // Only a game between two bots is paced; a failed one is shown as it is.
  const paced = session !== null && sessionMode(session) === 'bots' && session.status !== 'failed';
  const effective: Pace = paced ? pace : null;
  const shown = effective === null ? Math.max(cursor, latest, 0) : Math.min(cursor, latest);

  useEffect(() => {
    shownAt.current = Date.now();
  }, [id, cursor]);

  const step = useEffectEvent(() => {
    setHeld((previous) =>
      previous !== null && previous.id === id
        ? { id: previous.id, cursor: nextPacedCursor(previous.cursor, latest, effective) }
        : previous,
    );
  });
  const behind = current !== null && cursor < latest;
  useEffect(() => {
    if (!behind) return;
    const timer = setTimeout(step, paceDelay(effective, shownAt.current, Date.now()));
    return () => clearTimeout(timer);
  }, [id, effective, cursor, behind]);

  return shown;
}
