import { useEffect, useMemo, useState } from 'react';
import { chartRequests, requestMoves } from './analysisView';
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

export interface Engine {
  /** The live analysis of the frame on the board: idle when the engine is off or has not started on this frame yet. */
  live: LiveState;
  /** X's win chance by frame, as far as computed; null when there is no chart to draw. */
  chart: ChartState | null;
  /** Keep thinking (true) or Stop (false). */
  keep: (keep: boolean) => void;
}

/**
 * The engine of the analysis board for the frame `cursor`: the live analysis of that position,
 * stopped when the position changes, after 2 seconds unless asked to keep thinking, when the
 * engine is switched off and when the page is left; and the evaluation chart, filled in the
 * background, one position at a time, while the engine is on. `available` is false for a game
 * without an analysis program.
 */
export function useEngine(
  model: AnalysisModel,
  cursor: number,
  available: boolean,
  on: boolean,
  api: EngineApi = engineApi,
): Engine {
  const active = available && on;
  const moves = useMemo(() => requestMoves(model, cursor), [model, cursor]);
  const position = moves === null ? null : moves.join('|');

  const [live, setLive] = useState<LiveState>(IDLE);
  const [analysis] = useState(() => new LiveAnalysis(api, setLive));
  useEffect(() => {
    if (!active || moves === null || position === null) {
      analysis.reset();
      return;
    }
    analysis.begin(position, liveRequest(model.game, moves));
    return () => analysis.reset(false);
  }, [analysis, active, moves, position, model.game]);

  const requests = useMemo(() => chartRequests(model, lastFrame(model)), [model]);
  const chartKey = `${model.kind}/${model.id}`;
  const [chart, setChart] = useState<ChartState>(() => ({
    values: cachedChart(chartKey, model.frames.length),
    error: null,
  }));
  const [runner] = useState(() => new ChartRunner(api, setChart));
  useEffect(() => {
    if (!active || requests === null || requests.length < 2) return;
    runner.start(chartKey, model.game, requests);
    return () => runner.stop();
  }, [runner, active, requests, chartKey, model.game]);

  return {
    live: active && live.position === position ? live : IDLE,
    chart: active && requests !== null && requests.length >= 2 ? chart : null,
    keep: (keep) => analysis.setKeep(keep),
  };
}
