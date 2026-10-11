/* How fast a release searches, for "think time" (the server's speed.rs, in the browser). A
   computer seat in "fixed" mode plays a number of iterations, so that a takeback can replay it
   exactly; the number is the think time times the release's speed in iterations per
   millisecond, measured here, in this browser, by letting the release play itself. The measure
   is kept in localStorage under the release's name and the SHA-256 of its file. */

import type { BotFactory } from './bot';
import { botSeeds, type Rules, type RulesState, type Settled } from './rules';

/** The iterations per answer in the measuring game. */
export const MEASURE_ITERS = 3000;

/** No answer of the measuring game may take longer than this. */
export const MEASURE_LIMIT_MS = 120_000;

/** The iterations for `thinkMs` at `rate` iterations per millisecond. */
export function iterations(thinkMs: number, rate: number): number {
  return Math.max(1, Math.round(thinkMs * rate));
}

/** The median of `samples`; 0 when there are none. */
export function median(samples: readonly number[]): number {
  if (samples.length === 0) return 0;
  const sorted = [...samples].sort((a, b) => a - b);
  const middle = Math.floor(sorted.length / 2);
  const high = sorted[middle] ?? 0;
  return sorted.length % 2 === 1 ? high : ((sorted[middle - 1] ?? 0) + high) / 2;
}

/** The rate from the median answer time, not counting each seat's first answer. */
export function rateFromMedian(medianMs: number): number {
  return MEASURE_ITERS / Math.max(medianMs, 0.001);
}

/** Where measured rates are kept. */
export interface SpeedCache {
  get: (key: string) => number | null;
  set: (key: string, rate: number) => void;
}

const STORAGE_KEY = 'studio.speed.v1';

/** The cache key of a release, or null when its file has no hash (nothing is kept then). */
export function speedKey(release: string, hash: string | null): string | null {
  return hash === null ? null : `${release}:${hash}`;
}

/** A cache in localStorage. Every access may fail (blocked storage): it then forgets. */
export function localStorageSpeedCache(): SpeedCache {
  const read = (): Record<string, unknown> => {
    try {
      const parsed: unknown = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '{}');
      return typeof parsed === 'object' && parsed !== null
        ? (parsed as Record<string, unknown>)
        : {};
    } catch {
      return {};
    }
  };
  return {
    get(key) {
      const rate = read()[key];
      return typeof rate === 'number' && Number.isFinite(rate) && rate > 0 ? rate : null;
    },
    set(key, rate) {
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify({ ...read(), [key]: rate }));
      } catch {
        // Only costs a new measurement later.
      }
    },
  };
}

/** A cache that lives in memory, for tests. */
export function memorySpeedCache(): SpeedCache {
  const rates = new Map<string, number>();
  return {
    get: (key) => rates.get(key) ?? null,
    set: (key, rate) => void rates.set(key, rate),
  };
}

export interface MeasureOptions {
  rules: Rules;
  makeBot: BotFactory;
  module: WebAssembly.Module;
  game: string;
  release: string;
  /** Called with the number of answers of the measuring game so far. */
  progress: (answered: number) => void;
}

/**
 * Plays one game of the release against itself with `MEASURE_ITERS` iterations per answer and
 * seed 1, and returns the iterations per millisecond from the median answer time, not counting
 * each seat's first answer (it includes the start-up).
 */
export async function measure(options: MeasureOptions): Promise<number> {
  const { rules, makeBot, module, game, release } = options;
  const seat = { kind: 'bot', release, think_ms: 10, mode: 'fixed' };
  const state = rules.call<RulesState>({
    op: 'new',
    request: { game, seed: 1, opening_plies: 0, seats: [seat, seat] },
    releases: [release],
  });
  const seeds = botSeeds(rules, state);
  const bots = [];
  try {
    for (const seed of seeds) {
      bots.push(
        await makeBot(module, {
          CG_SEED: seed ?? '0',
          CG_FIXED_ITERS: String(MEASURE_ITERS),
        }),
      );
    }
    const samples: number[] = [];
    const answered = [0, 0];
    for (;;) {
      const settled = rules.call<Settled>({ op: 'settle', state });
      state.result = settled.result;
      const next = settled.next;
      if (next.kind === 'over') break;
      if (next.kind !== 'bot') throw new Error('the measuring game cannot be played');
      const bot = bots[next.seat];
      if (bot === undefined) throw new Error('the measuring game has no such seat');
      const { lines, ms } = await bot.ask(next.input, next.answer_lines, MEASURE_LIMIT_MS);
      if (answered[next.seat] !== 0) samples.push(ms);
      answered[next.seat] = (answered[next.seat] ?? 0) + 1;
      const played = rules.call<Pick<RulesState, 'turns' | 'result'>>({
        op: 'play',
        state,
        answer: { seat: next.seat, lines, ms },
      });
      state.turns = played.turns;
      state.result = played.result;
      if (played.result !== null) {
        const { end } = played.result;
        const why = end.kind === 'invalid' ? `: ${end.reason}` : '';
        throw new Error(`seat ${next.seat} answered against the rules${why}`);
      }
      options.progress(state.turns.length);
    }
    if (samples.length === 0) throw new Error('the game was too short to measure');
    return rateFromMedian(median(samples));
  } finally {
    for (const bot of bots) bot.terminate();
  }
}

/** Measures releases one at a time (two at once would slow each other) and remembers the result. */
export class Speed {
  private chain: Promise<unknown> = Promise.resolve();
  private readonly cache: SpeedCache;
  private readonly run: (options: MeasureOptions) => Promise<number>;

  constructor(cache: SpeedCache, run: (options: MeasureOptions) => Promise<number> = measure) {
    this.cache = cache;
    this.run = run;
  }

  /** The kept rate of a release, or null. */
  cached(release: string, hash: string | null): number | null {
    const key = speedKey(release, hash);
    return key === null ? null : this.cache.get(key);
  }

  /** The release's rate, measured if it is not kept. */
  async rate(release: string, hash: string | null, options: MeasureOptions): Promise<number> {
    const kept = this.cached(release, hash);
    if (kept !== null) return kept;
    const turn = this.chain.then(async () => {
      const again = this.cached(release, hash);
      if (again !== null) return again;
      let rate: number;
      try {
        rate = await this.run(options);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        throw new Error(`cannot measure the speed of ${release}: ${message}`, { cause: error });
      }
      const key = speedKey(release, hash);
      if (key !== null) this.cache.set(key, rate);
      return rate;
    });
    this.chain = turn.catch(() => {});
    return turn;
  }
}
