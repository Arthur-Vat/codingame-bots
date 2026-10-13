import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { FakeEngine } from './analysisFake';
import {
  cachedChart,
  CHART_ITERATIONS,
  chartRequest,
  ChartRunner,
  clearChartCache,
  type ChartState,
} from './chartAnalysis';

beforeEach(() => {
  vi.useFakeTimers();
  clearChartCache();
});

afterEach(() => {
  vi.useRealTimers();
});

/** Frame `i` of a game of `n` frames has `i` moves. */
const requests = (n: number) => Array.from({ length: n }, (_, i) => Array<string>(i).fill('4 4'));

/** An engine that answers each request done at its first read, with X's chance the number of moves over 10. */
function answering() {
  const engine = new FakeEngine();
  let open = 0;
  let most = 0;
  engine.onStart = (id, body) => {
    open++;
    most = Math.max(most, open);
    engine.finish(id, { wdl: [body.moves.length / 10, 0, 1 - body.moves.length / 10] });
  };
  const stop = engine.stop;
  engine.stop = async (id) => {
    open--;
    await stop(id);
  };
  return { engine, most: () => most };
}

function setup(engine: FakeEngine) {
  const seen: ChartState[] = [];
  const runner = new ChartRunner(engine, (state) => seen.push(state));
  return { runner, seen, last: () => seen[seen.length - 1]! };
}

describe('the chart', () => {
  it('asks 50,000 iterations with seed 1 for each position', () => {
    expect(chartRequest('uttt', ['4 4'])).toEqual({
      game: 'uttt',
      moves: ['4 4'],
      iterations: 50000,
      report_every: 50000,
      seed: '1',
    });
    expect(CHART_ITERATIONS).toBe(50000);
  });

  it('computes the frames in order, one analysis at a time, and fills the values', async () => {
    const { engine, most } = answering();
    const { runner, last } = setup(engine);
    runner.start('saved/g1', 'uttt', requests(4));
    expect(last().values).toEqual([null, null, null, null]);
    await vi.advanceTimersByTimeAsync(2000);
    expect(last()).toEqual({ values: [0, 0.1, 0.2, 0.3], error: null });
    expect(engine.started.map((s) => s.body.moves.length)).toEqual([0, 1, 2, 3]);
    expect(engine.started.every((s) => s.body.iterations === 50000 && s.body.seed === '1')).toBe(
      true,
    );
    expect(most()).toBe(1);
    // Each analysis is freed once read.
    expect(engine.live).toEqual([]);
  });

  it('shows the values as they come', async () => {
    const { engine } = answering();
    const { runner, seen } = setup(engine);
    runner.start('k', 'uttt', requests(3));
    await vi.advanceTimersByTimeAsync(2000);
    expect(seen.map((s) => s.values.filter((v) => v !== null).length)).toEqual([0, 1, 2, 3]);
  });

  it('keeps what it computed: the same game again computes nothing', async () => {
    const { engine } = answering();
    const first = setup(engine);
    first.runner.start('saved/g1', 'uttt', requests(3));
    await vi.advanceTimersByTimeAsync(2000);
    expect(engine.started).toHaveLength(3);

    const again = setup(engine);
    again.runner.start('saved/g1', 'uttt', requests(3));
    expect(again.last().values).toEqual([0, 0.1, 0.2]);
    await vi.advanceTimersByTimeAsync(2000);
    expect(engine.started).toHaveLength(3);
    // Another game does not share it.
    const other = setup(engine);
    other.runner.start('saved/g2', 'uttt', requests(3));
    expect(other.last().values).toEqual([null, null, null]);
    other.runner.stop();
  });

  it('stops where it is, and goes on from there when started again', async () => {
    const engine = new FakeEngine();
    // Each analysis takes a while to finish.
    engine.onStart = (id, body) => {
      setTimeout(() => engine.finish(id, { wdl: [body.moves.length / 10, 0, 0] }), 300);
    };
    const { runner, last } = setup(engine);
    runner.start('k', 'uttt', requests(5));
    await vi.advanceTimersByTimeAsync(800);
    const done = last().values.filter((v) => v !== null).length;
    expect(done).toBeGreaterThan(0);
    expect(done).toBeLessThan(5);

    runner.stop();
    // What was running is stopped, and nothing more is asked.
    expect(engine.live).toEqual([]);
    const asked = engine.started.length;
    await vi.advanceTimersByTimeAsync(3000);
    expect(engine.started).toHaveLength(asked);
    expect(last().values.filter((v) => v !== null)).toHaveLength(done);
    expect(cachedChart('k', 5).filter((v) => v !== null)).toHaveLength(done);

    runner.start('k', 'uttt', requests(5));
    await vi.advanceTimersByTimeAsync(3000);
    expect(last().values.every((v) => v !== null)).toBe(true);
    // The first frames were not asked again.
    expect(engine.started.map((s) => s.body.moves.length).slice(asked)).toEqual(
      [...Array(5).keys()].slice(done),
    );
  });

  it('ignores a late answer of a run that was stopped', async () => {
    const engine = new FakeEngine();
    const { runner, last } = setup(engine);
    runner.start('k', 'uttt', requests(3));
    await vi.advanceTimersByTimeAsync(0);
    const id = engine.started[0]!.id;
    runner.stop();
    engine.finish(id, { wdl: [0.9, 0, 0.1] });
    await vi.advanceTimersByTimeAsync(1000);
    expect(last().values).toEqual([null, null, null]);
    expect(engine.stopped).toContain(id);
    expect(engine.started).toHaveLength(1);
  });

  it('stops a request that was still on its way when the run stopped', async () => {
    const engine = new FakeEngine();
    engine.hold = true;
    const { runner } = setup(engine);
    runner.start('k', 'uttt', requests(3));
    await vi.advanceTimersByTimeAsync(0);
    runner.stop();
    engine.hold = false;
    engine.release();
    await vi.advanceTimersByTimeAsync(1000);
    expect(engine.started).toHaveLength(1);
    expect(engine.live).toEqual([]);
    expect(engine.reads).toHaveLength(0);
  });

  it('stops with the program’s message, keeping the values it has', async () => {
    const engine = new FakeEngine();
    engine.onStart = (id, body) => {
      if (body.moves.length === 2) {
        engine.set(id, { status: 'error', report: null, error: 'move 2 is not legal' });
      } else {
        engine.finish(id, { wdl: [0.5, 0, 0.5] });
      }
    };
    const { runner, last } = setup(engine);
    runner.start('k', 'uttt', requests(5));
    await vi.advanceTimersByTimeAsync(3000);
    expect(last()).toEqual({ values: [0.5, 0.5, null, null, null], error: 'move 2 is not legal' });
    expect(engine.started).toHaveLength(3);
  });

  it('stops with the message of a refused request', async () => {
    const engine = new FakeEngine();
    engine.startError = new Error('too many analyses');
    const { runner, last } = setup(engine);
    runner.start('k', 'uttt', requests(3));
    await vi.advanceTimersByTimeAsync(100);
    expect(last().error).toBe('too many analyses');
  });

  it('waits for a running analysis, reading it until it is done', async () => {
    const engine = new FakeEngine();
    engine.onStart = (id) => engine.report(id, { iterations: 100 });
    const { runner, last } = setup(engine);
    runner.start('k', 'uttt', requests(2));
    await vi.advanceTimersByTimeAsync(500);
    expect(last().values).toEqual([null, null]);
    expect(engine.reads.length).toBeGreaterThan(2);
    engine.finish('a1', { wdl: [0.7, 0, 0.3] });
    engine.onStart = (id) => engine.finish(id, { wdl: [0.2, 0, 0.8] });
    await vi.advanceTimersByTimeAsync(500);
    expect(last().values).toEqual([0.7, 0.2]);
  });
});
