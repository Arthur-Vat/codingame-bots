import { useEffect, useState } from 'react';
import type { Session } from './api';
import { initClocks, reconcile, tick, type ClockState } from './clock';
import { runningSeat, turnSeats } from './play';
import type { ClockSettings } from './setup';

/** How often a running clock is redrawn: often enough for tenths of a second. */
export const TICK_MS = 100;

interface Held {
  id: string;
  state: ClockState;
}

/**
 * The clocks of the human seats of a session, run by the front end. They need the clock chosen
 * in the setup dialog (`settings`; without it there are none, after a reload for example). A
 * clock runs while the server waits for that human, also while their move is on its way.
 */
export function useClocks(
  session: Session | null,
  settings: ClockSettings | null,
): ClockState | null {
  const [held, setHeld] = useState<Held | null>(null);
  const id = session?.id ?? null;
  const running = settings === null ? null : runningSeat(session);

  // Follow the turns: increments for new moves, and the undoing of them after a takeback. This
  // is a state adjusted while rendering (it only changes when the turns changed).
  if (session !== null && settings !== null) {
    const turns = turnSeats(session.turns);
    const next: Held =
      held === null || held.id !== session.id
        ? {
            id: session.id,
            state: initClocks(
              settings,
              session.seats.map((seat) => seat.kind === 'human'),
              turns,
            ),
          }
        : { id: session.id, state: reconcile(held.state, turns) };
    if (held === null || next.id !== held.id || next.state !== held.state) {
      setHeld(next);
    }
  }

  // Run the clock of the seat to move. The time since the last tick is counted when the clock
  // stops or changes hands, so no time is lost between a move and the next tick.
  useEffect(() => {
    if (id === null || running === null) return;
    let last = performance.now();
    const count = () => {
      const now = performance.now();
      const dt = now - last;
      last = now;
      setHeld((previous) =>
        previous === null || previous.id !== id
          ? previous
          : { id, state: tick(previous.state, dt, running).state },
      );
    };
    const timer = setInterval(count, TICK_MS);
    return () => {
      clearInterval(timer);
      count();
    };
  }, [id, running]);

  return held !== null && held.id === id && settings !== null ? held.state : null;
}
