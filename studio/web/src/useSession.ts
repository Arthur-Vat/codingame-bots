import { useCallback, useEffect, useRef, useState } from 'react';
import { ApiError, getSession, type Session } from './api';
import { startPoller } from './poller';

/** How often a session is polled while the game goes on, and after it ended. */
export const POLL_LIVE_MS = 200;
export const POLL_DONE_MS = 2000;

/** The polling delay for a status. */
export function pollDelay(status: Session['status'] | undefined): number {
  return status === 'over' || status === 'failed' ? POLL_DONE_MS : POLL_LIVE_MS;
}

export interface SessionState {
  session: Session | null;
  /** The last polling error; cleared by the next success. */
  error: ApiError | null;
  /** Shows a session an action answered with, and drops any poll that started before it. */
  apply: (session: Session) => void;
}

interface Held {
  id: string;
  session: Session | null;
  error: ApiError | null;
}

/** Polls a session. Stops when the component unmounts or `id` changes. */
export function useSession(id: string | null): SessionState {
  const [held, setHeld] = useState<Held | null>(null);
  // Counts the sessions applied by hand: a poll that started before one is older than it.
  const applied = useRef(0);

  const apply = useCallback(
    (session: Session) => {
      if (id === null) return;
      applied.current += 1;
      setHeld({ id, session, error: null });
    },
    [id],
  );

  useEffect(() => {
    if (id === null) {
      return;
    }
    return startPoller<{ session: Session; stale: boolean }>({
      fetch: async () => {
        const before = applied.current;
        const session = await getSession(id);
        return { session, stale: applied.current !== before };
      },
      delayAfter: ({ session }) => pollDelay(session.status),
      delayAfterError: () => POLL_DONE_MS,
      onValue: ({ session, stale }) => {
        if (!stale) setHeld({ id, session, error: null });
      },
      onError: (error) =>
        setHeld((previous) => ({
          id,
          session: previous?.id === id ? previous.session : null,
          error: error instanceof ApiError ? error : new ApiError(String(error), true),
        })),
    });
  }, [id]);

  // What is held for another id is not this session's.
  if (id === null || held === null || held.id !== id) {
    return { session: null, error: null, apply };
  }
  return { session: held.session, error: held.error, apply };
}
