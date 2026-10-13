import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fakeReport, FakeEngine } from './analysisFake';
import { chartRequests, requestMoves } from './analysisView';
import { ChartRunner, clearChartCache, type ChartState } from './chartAnalysis';
import { LiveAnalysis, type LiveState } from './liveAnalysis';
import { driveChart, driveLive } from './useEngine';

// The hook's effects, driven as React drives them. There is no page here (the project has no DOM
// test library): `driveLive` and `driveChart` are what the hook's effects call, so the sequences
// of strict mode (start, clean up, start again), of a new position and of leaving the page are
// run by hand.

beforeEach(() => {
  vi.useFakeTimers();
  clearChartCache();
});

afterEach(() => {
  vi.useRealTimers();
});

const turns = ['4 4', '4 5', '5 3', '3 7'].map((line, index) => [
  { seat: index % 2, lines: [line], ms: 1 },
]);
const model = { turns, openingTurns: 0 };

function setup() {
  const engine = new FakeEngine();
  // Every analysis of the chart finishes at once; the live ones run on.
  engine.onStart = (id, body) => {
    if (body.iterations === 50_000) engine.finish(id, { wdl: [0.5, 0, 0.5] });
    else engine.report(id);
  };
  const states: LiveState[] = [];
  const charts: ChartState[] = [];
  const analysis = new LiveAnalysis(engine, (state) => states.push(state));
  const runner = new ChartRunner(engine, (state) => charts.push(state));
  return { engine, analysis, runner, states, charts };
}

const advance = (ms: number) => vi.advanceTimersByTimeAsync(ms);

describe('the live effect', () => {
  it('survives strict mode: started, cleaned up, started again, with one analysis left', async () => {
    const { engine, analysis } = setup();
    const moves = requestMoves(model, 2);
    const first = driveLive(analysis, true, 'uttt', moves);
    first?.();
    const second = driveLive(analysis, true, 'uttt', moves);
    await advance(0);
    // The request that was dropped never started, or was stopped at once.
    expect(engine.live).toHaveLength(1);
    expect(analysis.current.status).toBe('running');
    second?.();
    await advance(0);
    expect(engine.live).toEqual([]);
    expect(engine.reads.length).toBeGreaterThan(0);
  });

  it('stops every analysis it started when the page is left, whatever it was doing', async () => {
    const { engine, analysis } = setup();
    engine.hold = true;
    const first = driveLive(analysis, true, 'uttt', requestMoves(model, 1));
    // The position changes while the first request is still on its way, then the page is left.
    first?.();
    const second = driveLive(analysis, true, 'uttt', requestMoves(model, 2));
    engine.hold = false;
    engine.release();
    await advance(0);
    second?.();
    await advance(0);
    expect(engine.started.length).toBeGreaterThan(0);
    expect(engine.live).toEqual([]);
    const reads = engine.reads.length;
    await advance(5000);
    expect(engine.reads).toHaveLength(reads);
  });

  it('stops on a new position and starts on the next', async () => {
    const { engine, analysis } = setup();
    const first = driveLive(analysis, true, 'uttt', requestMoves(model, 1));
    await advance(0);
    first?.();
    const second = driveLive(analysis, true, 'uttt', requestMoves(model, 2));
    await advance(0);
    expect(engine.stopped).toContain('a1');
    expect(engine.live).toEqual(['a2']);
    expect(engine.started[1]?.body.moves).toEqual(['4 4', '4 5']);
    second?.();
  });

  it('asks nothing while the engine is off, and stops what ran when it is switched off', async () => {
    const { engine, analysis, states } = setup();
    const clean = driveLive(analysis, true, 'uttt', requestMoves(model, 1));
    await advance(0);
    expect(engine.live).toHaveLength(1);
    clean?.();
    expect(driveLive(analysis, false, 'uttt', requestMoves(model, 1))).toBeUndefined();
    await advance(0);
    expect(engine.live).toEqual([]);
    expect(states[states.length - 1]?.status).toBe('idle');
    // A position that cannot be asked is not asked either.
    expect(driveLive(analysis, true, 'uttt', null)).toBeUndefined();
    await advance(10);
    expect(engine.started).toHaveLength(1);
  });

  it('shows the report of the position it started on', async () => {
    const { analysis } = setup();
    driveLive(analysis, true, 'uttt', requestMoves(model, 1))?.();
    const clean = driveLive(analysis, true, 'uttt', requestMoves(model, 3));
    await advance(200);
    expect(analysis.current.position).toBe('4 4|4 5|5 3');
    expect(analysis.current.report).toEqual(fakeReport());
    clean?.();
  });
});

describe('the chart effect', () => {
  const requests = chartRequests(model, 4);

  it('survives strict mode: one chart runs, and no analysis is left once it is cleaned up', async () => {
    const { engine, runner, charts } = setup();
    driveChart(runner, true, 'saved/g', 'uttt', requests)?.();
    const clean = driveChart(runner, true, 'saved/g', 'uttt', requests);
    await advance(3000);
    expect(charts[charts.length - 1]?.values).toEqual([0.5, 0.5, 0.5, 0.5, 0.5]);
    // The first position was asked twice, by the run that was cleaned up and by the next, and the
    // first of the two stopped; then each position once, in order.
    expect(engine.started.map((s) => s.body.moves.length)).toEqual([0, 0, 1, 2, 3, 4]);
    expect(engine.stopped).toContain('a1');
    clean?.();
    expect(engine.live).toEqual([]);
  });

  it('stops the analysis it was waiting for when it is cleaned up in the middle', async () => {
    const { engine, runner } = setup();
    engine.onStart = () => undefined;
    const clean = driveChart(runner, true, 'saved/g', 'uttt', requests);
    await advance(200);
    expect(engine.live).toHaveLength(1);
    clean?.();
    expect(engine.live).toEqual([]);
    await advance(2000);
    expect(engine.started).toHaveLength(1);
  });

  it('does nothing with the engine off, or for a game with one frame', async () => {
    const { engine, runner } = setup();
    expect(driveChart(runner, false, 'k', 'uttt', requests)).toBeUndefined();
    expect(driveChart(runner, true, 'k', 'uttt', null)).toBeUndefined();
    expect(driveChart(runner, true, 'k', 'uttt', chartRequests(model, 0))).toBeUndefined();
    await advance(1000);
    expect(engine.started).toEqual([]);
  });
});
