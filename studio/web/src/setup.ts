import type { SessionRequest } from './api';

/** The think times a computer seat offers, in milliseconds. */
export const THINK_MS = [25, 50, 100, 200, 500, 1000, 2000] as const;
export const DEFAULT_THINK_INDEX = 2;

/** The minutes per side a clock offers. */
export const MINUTES = [0.5, 1, 2, 3, 5, 10, 15, 20, 30, 45, 60] as const;
export const DEFAULT_MINUTES_INDEX = 5;

/** The increments per move a clock offers, in seconds. */
export const INCREMENTS = [0, 1, 2, 3, 5, 10, 15, 20, 30] as const;
export const DEFAULT_INCREMENT_INDEX = 0;

export type Mode = 'friend' | 'computer' | 'bots';
export type Side = 'x' | 'random' | 'o';

export const MODE_TITLES: Record<Mode, string> = {
  friend: 'Play with a friend',
  computer: 'Play the computer',
  bots: 'Bot against bot',
};

/** What the owner chose in the setup dialog, one shape per mode. */
export type SetupForm =
  | { mode: 'friend'; minutes: number; incrementSeconds: number }
  | {
      mode: 'computer';
      release: string;
      thinkMs: number;
      side: Side;
      minutes: number;
      incrementSeconds: number;
    }
  | { mode: 'bots'; xRelease: string; oRelease: string; xThinkMs: number; oThinkMs: number };

/** The clock of a human seat. A later change runs it. */
export interface ClockSettings {
  minutes: number;
  incrementSeconds: number;
}

/** The clock a form asks for, or null when no human plays. */
export function clockOf(form: SetupForm): ClockSettings | null {
  return form.mode === 'bots'
    ? null
    : { minutes: form.minutes, incrementSeconds: form.incrementSeconds };
}

/**
 * Builds the body of `POST /api/sessions`. `random` is `Math.random` (a number in [0, 1)),
 * used only for the random side: X is seat 0, which moves first.
 */
export function buildSessionRequest(
  game: string,
  form: SetupForm,
  seed: number,
  random: () => number,
): SessionRequest {
  const base = { game, seed, opening_plies: 0 };
  switch (form.mode) {
    case 'friend':
      return {
        ...base,
        seats: [
          { kind: 'human', name: 'Player 1' },
          { kind: 'human', name: 'Player 2' },
        ],
      };
    case 'computer': {
      const humanSeat =
        form.side === 'random' ? (random() < 0.5 ? 0 : 1) : form.side === 'x' ? 0 : 1;
      const human = { kind: 'human', name: 'You' } as const;
      const bot = {
        kind: 'bot',
        release: form.release,
        think_ms: form.thinkMs,
        mode: 'fixed',
      } as const;
      return { ...base, seats: humanSeat === 0 ? [human, bot] : [bot, human] };
    }
    case 'bots':
      return {
        ...base,
        seats: [
          { kind: 'bot', release: form.xRelease, think_ms: form.xThinkMs, mode: 'realtime' },
          { kind: 'bot', release: form.oRelease, think_ms: form.oThinkMs, mode: 'realtime' },
        ],
      };
  }
}

/** The seed taken from the clock: the same one for every game started in the same millisecond. */
export function clockSeed(now: number): number {
  return now % 1e10;
}

/**
 * The seed to send: the typed one when advanced mode is on and the text is a whole number the
 * server can hold exactly, else one from the clock.
 */
export function resolveSeed(advanced: boolean, text: string, now: number): number {
  const trimmed = text.trim();
  if (advanced && /^\d+$/.test(trimmed)) {
    const typed = Number(trimmed);
    if (Number.isSafeInteger(typed)) {
      return typed;
    }
  }
  return clockSeed(now);
}

/** The number at the end of a release name (`uttt-v010` is 10), or -1. */
function releaseNumber(name: string): number {
  const match = /-v(\d+)$/.exec(name);
  return match?.[1] ? Number(match[1]) : -1;
}

/** Release names, newest first. The server already sends them so; this makes sure. */
export function orderReleases(names: readonly string[]): string[] {
  return [...names].sort((a, b) => releaseNumber(b) - releaseNumber(a) || (a < b ? 1 : -1));
}

/** The release a computer seat starts with: the latest. */
export function defaultRelease(names: readonly string[]): string {
  return orderReleases(names)[0] ?? '';
}

/** The release the second bot starts with: the second latest, or the only one. */
export function defaultSecondRelease(names: readonly string[]): string {
  const ordered = orderReleases(names);
  return ordered[1] ?? ordered[0] ?? '';
}

/** The label of a release in a list: the latest says so. */
export function releaseLabel(name: string, latest: string): string {
  return name === latest ? `${name} (latest)` : name;
}

/** How a think time is shown. */
export function thinkLabel(ms: number): string {
  return `≈ ${ms} ms`;
}

/** How minutes per side are shown: half a minute is written ½. */
export function minutesLabel(minutes: number): string {
  return minutes === 0.5 ? '½' : String(minutes);
}

/** A clock as "minutes + increment", for the game's info card. */
export function clockLabel(clock: ClockSettings): string {
  return `${minutesLabel(clock.minutes)} + ${clock.incrementSeconds}`;
}
