import { useEffect, useMemo, useState } from 'react';
import { chartRequests } from './analysisView';
import { lastFrame, type AnalysisModel } from './analysis';
import { cachedChart, ChartRunner, type ChartState } from './chartAnalysis';
import {
  engineApi,
  IDLE,
  liveRequest,
  LiveAnalysis,
  type EngineApi,
  type LiveState,
} from './liveAnalysis';

/**
 * What the live analysis's effect does: starts on the position (the engine on, the position
 * askable) or stops everything, and gives back what the effect cleans up with. Written as a
 * function of its own so that the sequence React runs in strict mode (start, clean up, start) and
 * when the page is left can be tested with no page.
 */
export function driveLive(
  analysis: LiveAnalysis,
  active: boolean,
  game: string,
  moves: string[] | null,
): (() => void) | undefined {
  if (!active || moves === null) {
    analysis.reset();
    return undefined;
  }
  analysis.begin(moves.join('|'), liveRequest(game, moves));
  return () => analysis.reset(false);
}

/** The same for the evaluation chart: it computes the game's frames while the engine is on. */
export function driveChart(
  runner: ChartRunner,
  active: boolean,
  key: string,
  game: string,
  requests: string[][] | null,
): (() => void) | undefined {
  if (!active || requests === null || requests.length < 2) return undefined;
  runner.start(key, game, requests);
  return () => runner.stop();
}

export interface Engine {
  /** The live analysis of the position on the board: idle when the engine is off or has not started on this frame yet. */
  live: LiveState;
  /** X's win chance by frame, as far as computed; null when there is no chart to draw. */
  chart: ChartState | null;
  /** Keep thinking (true) or Stop (false). */
  keep: (keep: boolean) => void;
}

/**
 * The engine of the analysis board for the position on the board, given as its `moves` (the
 * request's, from the game's start; null when it cannot be asked; the same array while the position
 * is the same, as the effect restarts on a new one): the live analysis of that position, stopped
 * when the position changes, after 2 seconds unless asked to keep thinking, when the engine is
 * switched off and when the page is left; and the evaluation chart of the game (never of a side
 * line), filled in the background, one position at a time, while the engine is on. `available` is
 * false for a game without an analysis program.
 */
export function useEngine(
  model: AnalysisModel,
  moves: string[] | null,
  available: boolean,
  on: boolean,
  api: EngineApi = engineApi,
): Engine {
  const active = available && on;
  const position = moves === null ? null : moves.join('|');

  const [live, setLive] = useState<LiveState>(IDLE);
  const [analysis] = useState(() => new LiveAnalysis(api, setLive));
  useEffect(
    () => driveLive(analysis, active, model.game, moves),
    [analysis, active, moves, model.game],
  );

  const requests = useMemo(() => chartRequests(model, lastFrame(model)), [model]);
  const chartKey = `${model.kind}/${model.id}`;
  const [chart, setChart] = useState<ChartState>(() => ({
    values: cachedChart(chartKey, model.frames.length),
    error: null,
  }));
  const [runner] = useState(() => new ChartRunner(api, setChart));
  useEffect(
    () => driveChart(runner, active, chartKey, model.game, requests),
    [runner, active, requests, chartKey, model.game],
  );

  return {
    live: active && live.position === position ? live : IDLE,
    chart: active && requests !== null && requests.length >= 2 ? chart : null,
    keep: (keep) => analysis.setKeep(keep),
  };
}
