/* The evaluation chart's background analysis: X's win chance after each move of the game, from one
   analysis per position, one at a time, in order (ADR 0032, decision 5: 50,000 iterations, seed
   "1"). What is computed is kept in memory for the game, so that returning to it computes nothing. */

import type { AnalysisRequest } from './api';
import { errorMessage } from './browser';
import { validWdl } from './analysisView';
import type { EngineApi } from './liveAnalysis';

/** Iterations for each position of the chart. */
export const CHART_ITERATIONS = 50_000;
/** How long to wait between two reads of the analysis of one position. */
export const CHART_POLL_MS = 80;

/** X's win chance after each frame, from the first; null for a frame not computed yet. */
export type ChartValues = (number | null)[];

export interface ChartState {
  values: ChartValues;
  /** Why the chart stopped before the end. */
  error: string | null;
}

/** The request for one position of the chart. */
export function chartRequest(game: string, moves: string[]): AnalysisRequest & { game: string } {
  return {
    game,
    moves,
    iterations: CHART_ITERATIONS,
    report_every: CHART_ITERATIONS,
    seed: '1',
  };
}

const CACHE_LIMIT = 40;
const cache = new Map<string, ChartValues>();

/** Forgets every computed chart (for tests). */
export function clearChartCache(): void {
  cache.clear();
}

/** What is known of a game's chart: `count` values, those computed first. */
export function cachedChart(key: string, count: number): ChartValues {
  const known = cache.get(key);
  if (known === undefined || known.length !== count) return Array<number | null>(count).fill(null);
  return known.slice();
}

function remember(key: string, values: ChartValues): void {
  cache.delete(key);
  cache.set(key, values.slice());
  while (cache.size > CACHE_LIMIT) {
    const oldest = cache.keys().next();
    if (oldest.done) break;
    cache.delete(oldest.value);
  }
}

const ignore = (): void => undefined;
const sleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));

/** Computes a game's chart in the background, one analysis at a time. */
export class ChartRunner {
  private generation = 0;
  private id: string | null = null;

  private readonly api: EngineApi;
  private readonly onChange: (state: ChartState) => void;

  constructor(api: EngineApi, onChange: (state: ChartState) => void) {
    this.api = api;
    this.onChange = onChange;
  }

  /**
   * Starts (or goes on with) the chart of the game `key`: `requests[i]` are the moves of frame `i`.
   * What the cache holds is shown at once and not computed again.
   */
  start(key: string, game: string, requests: string[][]): void {
    this.stop();
    const generation = this.generation;
    const values = cachedChart(key, requests.length);
    this.onChange({ values: values.slice(), error: null });
    void this.fill(generation, key, game, requests, values);
  }

  /** Stops the analysis that runs; the values stay in the cache. */
  stop(): void {
    this.generation += 1;
    const id = this.id;
    this.id = null;
    if (id !== null) this.api.stop(id).catch(ignore);
  }

  private async fill(
    generation: number,
    key: string,
    game: string,
    requests: string[][],
    values: ChartValues,
  ): Promise<void> {
    try {
      for (let frame = 0; frame < requests.length; frame++) {
        if (values[frame] !== null) continue;
        const moves = requests[frame];
        if (moves === undefined) return;
        const chance = await this.analyseWithRetry(generation, chartRequest(game, moves));
        if (chance === null || generation !== this.generation) return;
        values[frame] = chance;
        remember(key, values);
        this.onChange({ values: values.slice(), error: null });
      }
    } catch (error) {
      if (generation === this.generation) {
        this.onChange({ values: values.slice(), error: errorMessage(error) });
      }
    }
  }

  /**
   * `analyse`, and once more when it fails (an analysis the server dropped, a network blip): a
   * second failure for the same position is the chart's end.
   */
  private async analyseWithRetry(
    generation: number,
    request: AnalysisRequest & { game: string },
  ): Promise<number | null> {
    try {
      return await this.analyse(generation, request);
    } catch {
      if (generation !== this.generation) return null;
      const failed = this.id;
      this.id = null;
      if (failed !== null) this.api.stop(failed).catch(ignore);
      return await this.analyse(generation, request);
    }
  }

  /** X's win chance for one position, or null when this run was stopped meanwhile. */
  private async analyse(
    generation: number,
    request: AnalysisRequest & { game: string },
  ): Promise<number | null> {
    const id = await this.api.start(request);
    if (generation !== this.generation) {
      this.api.stop(id).catch(ignore);
      return null;
    }
    this.id = id;
    for (;;) {
      const state = await this.api.get(id);
      if (generation !== this.generation) return null;
      if (state.status === 'error') {
        // Its slot is freed before the error is dealt with.
        this.id = null;
        this.api.stop(id).catch(ignore);
        throw new Error(state.error ?? 'The analysis failed.');
      }
      if (state.status === 'done' || state.report?.done === true) {
        this.id = null;
        this.api.stop(id).catch(ignore);
        const wdl = validWdl(state.report?.wdl);
        if (wdl === null) throw new Error('The analysis gave no result.');
        return wdl[0];
      }
      await sleep(CHART_POLL_MS);
      if (generation !== this.generation) return null;
    }
  }
}
