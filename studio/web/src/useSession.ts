import { useEffect, useState } from 'react';
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
}

interface Held extends SessionState {
  id: string;
}

/** Polls a session. Stops when the component unmounts or `id` changes. */
export function useSession(id: string | null): SessionState {
  const [held, setHeld] = useState<Held | null>(null);

  useEffect(() => {
    if (id === null) {
      return;
    }
    return startPoller<Session>({
      fetch: () => getSession(id),
      delayAfter: (session) => pollDelay(session.status),
      delayAfterError: () => POLL_DONE_MS,
      onValue: (session) => setHeld({ id, session, error: null }),
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
    return { session: null, error: null };
  }
  return { session: held.session, error: held.error };
}
