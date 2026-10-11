/* The clocks of the human seats, as pure transitions. The server has no clock for humans
   (studio/README.md, `/end`): the front end runs them and ends the game on a timeout. */

import type { ClockSettings } from './setup';

/** Below this the running clock is drawn in the "low time" colour. */
export const LOW_MS = 20_000;
/** Below this the clock shows tenths of a second. */
export const TENTHS_MS = 10_000;

export interface ClockState {
  /** Milliseconds left per seat; null for a seat without a clock (a bot). */
  remaining: (number | null)[];
  /** The seat of each turn counted so far, to give increments and to undo them on a takeback. */
  turns: (number | null)[];
  /** The seat that ran out of time. Set once, by `tick`. */
  expired: number | null;
  /** The increment after each move, in milliseconds. */
  incrementMs: number;
}

/**
 * Clocks for a new session. `timed[seat]` says whether the seat has a clock; `turns` are the
 * turns that exist already (they cost nothing and earn no increment).
 */
export function initClocks(
  settings: ClockSettings,
  timed: readonly boolean[],
  turns: readonly (number | null)[],
): ClockState {
  return {
    remaining: timed.map((on) => (on ? settings.minutes * 60_000 : null)),
    turns: [...turns],
    expired: null,
    incrementMs: settings.incrementSeconds * 1000,
  };
}

/**
 * Lets `dtMs` pass on the clock of `running`, the seat that is to move and may think (null when
 * nobody's clock runs: a bot's turn, a request in flight, a rewind). A clock that reaches zero
 * stops there and `timeout` names its seat, once: later ticks return the same state.
 */
export function tick(
  state: ClockState,
  dtMs: number,
  running: number | null,
): { state: ClockState; timeout: number | null } {
  if (running === null || state.expired !== null || dtMs <= 0) {
    return { state, timeout: null };
  }
  const left = state.remaining[running];
  if (left === null || left === undefined) {
    return { state, timeout: null };
  }
  const next = Math.max(0, left - dtMs);
  const remaining = state.remaining.slice();
  remaining[running] = next;
  if (next > 0) {
    return { state: { ...state, remaining }, timeout: null };
  }
  return { state: { ...state, remaining, expired: running }, timeout: running };
}

/**
 * Brings the clocks in line with the seats of the session's turns. A turn that is new and
 * belongs to a clocked seat adds the increment to it. When turns were taken back, the
 * increment of each is taken off again (but a clock is left at 1 s at least, or what it was if
 * that is less), so a takeback is not a way to earn time.
 */
export function reconcile(state: ClockState, turns: readonly (number | null)[]): ClockState {
  let common = 0;
  while (
    common < turns.length &&
    common < state.turns.length &&
    turns[common] === state.turns[common]
  ) {
    common += 1;
  }
  if (common === state.turns.length && common === turns.length) {
    return state;
  }
  const remaining = state.remaining.slice();
  const change = (seat: number | null, delta: number, floor: boolean) => {
    if (seat === null) return;
    const left = remaining[seat];
    if (left === null || left === undefined) return;
    remaining[seat] = floor ? Math.max(left + delta, Math.min(left, 1000)) : left + delta;
  };
  for (const seat of state.turns.slice(common)) {
    change(seat, -state.incrementMs, true);
  }
  for (const seat of turns.slice(common)) {
    change(seat, state.incrementMs, false);
  }
  // An expiry stands (a move made after the clock hit zero does not undo it), unless turns were
  // taken back.
  const removed = common < state.turns.length;
  return { ...state, remaining, turns: [...turns], expired: removed ? null : state.expired };
}

/** `m:ss`, and `0:ss.t` under ten seconds, as in the mockup. */
export function formatClock(ms: number): string {
  const clamped = Math.max(0, ms);
  const seconds = Math.floor(clamped / 1000);
  if (clamped < TENTHS_MS) {
    return `0:0${seconds}.${Math.floor((clamped % 1000) / 100)}`;
  }
  const minutes = Math.floor(seconds / 60);
  return `${minutes}:${String(seconds % 60).padStart(2, '0')}`;
}

/** The time a bot has used, in seconds with one decimal. */
export function formatSpent(ms: number): string {
  return `${(ms / 1000).toFixed(1)} s`;
}

/** The class of a clock: `run` for the one that runs, `low` when it runs under 20 s. */
export function clockClass(remaining: number | null, running: boolean): string {
  if (!running) return '';
  return remaining !== null && remaining < LOW_MS ? 'low' : 'run';
}
