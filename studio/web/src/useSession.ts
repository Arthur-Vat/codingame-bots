import { useEffect, useState } from 'react';
import { ApiError, getSession, type Session } from './api';

/** How often a session is polled while the game goes on, and after it ended. */
export const POLL_LIVE_MS = 200;
export const POLL_DONE_MS = 2000;

/** The polling delay for a status. */
export function pollDelay(status: Session['status'] | undefined): number {
  return status === 'over' || status === 'failed' ? POLL_DONE_MS : POLL_LIVE_MS;
}

export interface SessionState {
  session: Session | null;
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
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    const poll = async (): Promise<void> => {
      let delay: number;
      try {
        const session = await getSession(id);
        if (stopped) return;
        setHeld({ id, session, error: null });
        delay = pollDelay(session.status);
      } catch (error) {
        if (stopped) return;
        const apiError = error instanceof ApiError ? error : new ApiError(String(error), true);
        setHeld((previous) => ({
          id,
          session: previous?.id === id ? previous.session : null,
          error: apiError,
        }));
        delay = POLL_DONE_MS;
      }
      timer = setTimeout(() => void poll(), delay);
    };

    void poll();
    return () => {
      stopped = true;
      clearTimeout(timer);
    };
  }, [id]);

  // What is held for another id is not this session's.
  if (id === null || held === null || held.id !== id) {
    return { session: null, error: null };
  }
  return { session: held.session, error: held.error };
}
