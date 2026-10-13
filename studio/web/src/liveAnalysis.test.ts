import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fakeReport, FakeEngine } from './analysisFake';
import { IDLE, liveRequest, LiveAnalysis, POLL_MS, THINK_MS, type LiveState } from './liveAnalysis';

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

function setup() {
  const engine = new FakeEngine();
  const seen: LiveState[] = [];
  const live = new LiveAnalysis(engine, (state) => seen.push(state));
  return { engine, live, seen };
}

const request = (...moves: string[]) => liveRequest('uttt', moves);
const advance = (ms: number) => vi.advanceTimersByTimeAsync(ms);

describe('the live request', () => {
  it('asks for as long as the program allows, a report every 20,000 iterations, seed 1', () => {
    expect(liveRequest('uttt', ['4 4'])).toEqual({
      game: 'uttt',
      moves: ['4 4'],
      iterations: 100000000,
      report_every: 20000,
      depth: 10,
      seed: '1',
    });
  });
});

describe('a position', () => {
  it('is started, read about every 150 ms, and shows the latest report', async () => {
    const { engine, live } = setup();
    live.begin('p1', request('4 4'));
    expect(live.current.status).toBe('running');
    expect(live.current.position).toBe('p1');
    await advance(0);
    expect(engine.started).toHaveLength(1);
    expect(engine.started[0]?.body.moves).toEqual(['4 4']);
    expect(engine.reads).toHaveLength(1);

    engine.report('a1', { iterations: 20000 });
    await advance(POLL_MS);
    expect(live.current.report?.iterations).toBe(20000);
    expect(engine.reads).toHaveLength(2);
    engine.report('a1', { iterations: 40000 });
    await advance(POLL_MS);
    expect(live.current.report?.iterations).toBe(40000);
    expect(live.current.elapsedMs).toBeGreaterThanOrEqual(POLL_MS);
    expect(live.current.status).toBe('running');
  });

  it('never reads twice at once, however slow the answer', async () => {
    const { engine, live } = setup();
    let open = 0;
    let most = 0;
    const get = engine.get;
    engine.get = async (id) => {
      open++;
      most = Math.max(most, open);
      await new Promise((resolve) => setTimeout(resolve, 500));
      open--;
      return get(id);
    };
    live.begin('p', request());
    await advance(3000);
    expect(most).toBe(1);
    live.reset();
  });

  it('stops after 2.0 s, keeps its report and reads no more', async () => {
    const { engine, live } = setup();
    live.begin('p', request('4 4'));
    await advance(0);
    engine.report('a1', { iterations: 812000 });
    await advance(THINK_MS - 1);
    expect(live.current.status).toBe('running');
    expect(engine.stopped).toEqual([]);

    await advance(1);
    expect(live.current.status).toBe('stopped');
    expect(live.current.elapsedMs).toBe(THINK_MS);
    expect(live.current.report?.iterations).toBe(812000);
    expect(engine.stopped).toEqual(['a1']);

    const reads = engine.reads.length;
    await advance(5000);
    expect(engine.reads).toHaveLength(reads);
    expect(engine.started).toHaveLength(1);
  });

  it('thinks for its 2.0 s from when its request was accepted, not from when it was asked', async () => {
    const { engine, live } = setup();
    engine.hold = true;
    live.begin('p', request('4 4'));
    await advance(1500);
    expect(engine.started).toHaveLength(0);
    engine.release();
    await advance(THINK_MS - 100);
    expect(live.current.status).toBe('running');
    await advance(100);
    expect(live.current.status).toBe('stopped');
    expect(live.current.elapsedMs).toBe(THINK_MS);
    expect(engine.stopped).toEqual(['a1']);
  });

  it('gives a position that waited for the one before its whole 2.0 s', async () => {
    const { engine, live } = setup();
    engine.hold = true;
    live.begin('p1', request('1'));
    await advance(0);
    live.begin('p2', request('2'));
    await advance(1000);
    engine.hold = false;
    engine.release();
    await advance(THINK_MS - 1);
    expect(live.current.position).toBe('p2');
    expect(live.current.status).toBe('running');
    await advance(1);
    expect(live.current.status).toBe('stopped');
  });

  it('is replaced by the next position: the old one stopped, its late answers ignored', async () => {
    const { engine, live } = setup();
    live.begin('p1', request('4 4'));
    await advance(0);
    // The read of the first position is still on its way when the position changes.
    let answer: (value: unknown) => void = () => undefined;
    engine.get = async (id) => {
      engine.reads.push(id);
      await new Promise((resolve) => {
        answer = resolve;
      });
      return { status: 'running', report: fakeReport({ iterations: 999 }), error: null };
    };
    await advance(POLL_MS);
    live.begin('p2', request('4 4', '4 5'));
    expect(live.current.position).toBe('p2');
    expect(live.current.report).toBeNull();
    expect(engine.stopped).toContain('a1');
    answer(null);
    await advance(0);
    expect(live.current.position).toBe('p2');
    expect(live.current.report).toBeNull();
  });

  it('gives the new position its own 2.0 s', async () => {
    const { engine, live } = setup();
    live.begin('p1', request('4 4'));
    await advance(1500);
    live.begin('p2', request('4 4', '4 5'));
    await advance(1500);
    expect(live.current.status).toBe('running');
    await advance(500);
    expect(live.current.status).toBe('stopped');
    expect(engine.stopped).toEqual(['a1', 'a2']);
  });

  it('is stopped, and a request still on its way too, when it is reset', async () => {
    const { engine, live } = setup();
    engine.hold = true;
    live.begin('p1', request('4 4'));
    await advance(0);
    live.reset();
    expect(live.current).toEqual(IDLE);
    engine.release();
    await advance(0);
    // It started after all, found it was no longer wanted, and was stopped without being read.
    expect(engine.started).toHaveLength(1);
    expect(engine.stopped).toEqual(['a1']);
    expect(engine.reads).toHaveLength(0);
    await advance(THINK_MS * 2);
    expect(live.current).toEqual(IDLE);
  });

  it('sends one request at a time: stepping fast leaves only the last one to start', async () => {
    const { engine, live } = setup();
    engine.hold = true;
    live.begin('p1', request('1'));
    await advance(0);
    live.begin('p2', request('2'));
    live.begin('p3', request('3'));
    live.begin('p4', request('4'));
    engine.hold = false;
    engine.release();
    await advance(0);
    // The first was stopped on arrival; of the others only the newest was sent.
    expect(engine.started.map((s) => s.body.moves)).toEqual([['1'], ['4']]);
    expect(engine.stopped).toEqual(['a1']);
    expect(engine.live).toEqual(['a2']);
    expect(live.current.position).toBe('p4');
    live.reset();
  });
});

describe('Keep thinking', () => {
  it('removes the time limit, and Stop ends it', async () => {
    const { engine, live } = setup();
    live.begin('p', request('4 4'));
    await advance(0);
    engine.report('a1');
    await advance(500);
    live.setKeep(true);
    expect(live.current.keep).toBe(true);
    await advance(THINK_MS * 3);
    expect(live.current.status).toBe('running');
    expect(engine.stopped).toEqual([]);

    live.setKeep(false);
    expect(live.current.status).toBe('stopped');
    expect(live.current.keep).toBe(false);
    expect(engine.stopped).toEqual(['a1']);
    expect(live.current.report).not.toBeNull();
  });

  it('restarts after the time limit, going on from the time and the report it had', async () => {
    const { engine, live } = setup();
    live.begin('p', request('4 4'));
    await advance(0);
    engine.report('a1', { iterations: 800000 });
    await advance(THINK_MS);
    expect(live.current.status).toBe('stopped');

    live.setKeep(true);
    expect(live.current.status).toBe('running');
    expect(live.current.keep).toBe(true);
    expect(live.current.report?.iterations).toBe(800000);
    expect(live.current.elapsedMs).toBe(THINK_MS);
    await advance(0);
    expect(engine.started).toHaveLength(2);
    expect(engine.started[1]?.body).toEqual(engine.started[0]?.body);

    // The new run starts over; its early reports do not replace what was found.
    engine.report('a2', { iterations: 20000 });
    await advance(POLL_MS);
    expect(live.current.report?.iterations).toBe(800000);
    expect(live.current.elapsedMs).toBeGreaterThan(THINK_MS);
    engine.report('a2', { iterations: 900000 });
    await advance(POLL_MS);
    expect(live.current.report?.iterations).toBe(900000);

    // No limit this time.
    await advance(THINK_MS * 3);
    expect(live.current.status).toBe('running');
  });

  it('restarts after Stop too', async () => {
    const { engine, live } = setup();
    live.begin('p', request());
    await advance(0);
    live.setKeep(true);
    live.setKeep(false);
    expect(live.current.status).toBe('stopped');
    live.setKeep(true);
    await advance(0);
    expect(live.current.status).toBe('running');
    expect(engine.started).toHaveLength(2);
    live.reset();
  });

  it('does nothing for an analysis that is done or failed', async () => {
    const { engine, live } = setup();
    live.begin('p', request());
    await advance(0);
    engine.finish('a1', { stop: 'proven', proven: 0 });
    await advance(POLL_MS);
    expect(live.current.status).toBe('done');
    live.setKeep(true);
    expect(live.current.status).toBe('done');
    expect(live.current.keep).toBe(false);
    expect(engine.started).toHaveLength(1);
  });

  it('is off again for the next position', async () => {
    const { live } = setup();
    live.begin('p1', request());
    live.setKeep(true);
    live.begin('p2', request('1'));
    expect(live.current.keep).toBe(false);
    live.reset();
  });
});

describe('what the program does itself', () => {
  it('ends the analysis when it is done: its slot is freed, its report stays', async () => {
    const { engine, live } = setup();
    live.begin('p', request('4 4'));
    await advance(0);
    engine.finish('a1', { stop: 'nodes', iterations: 3_000_000 });
    await advance(POLL_MS);
    expect(live.current.status).toBe('done');
    expect(live.current.report?.stop).toBe('nodes');
    expect(engine.stopped).toEqual(['a1']);
    const reads = engine.reads.length;
    await advance(THINK_MS * 2);
    expect(engine.reads).toHaveLength(reads);
    expect(live.current.status).toBe('done');
  });

  it('shows the result of a finished game the same way', async () => {
    const { engine, live } = setup();
    live.begin('p', request('4 4'));
    await advance(0);
    engine.finish('a1', { stop: 'over', to_move: null, wdl: [1, 0, 0], moves: [], iterations: 0 });
    await advance(POLL_MS);
    expect(live.current.status).toBe('done');
    expect(live.current.report?.wdl).toEqual([1, 0, 0]);
  });
});

describe('errors', () => {
  it('shows the program’s message', async () => {
    const { engine, live } = setup();
    live.begin('p', request('9 9'));
    await advance(0);
    engine.set('a1', { status: 'error', report: null, error: 'move 1 ("9 9") is not legal' });
    await advance(POLL_MS);
    expect(live.current.status).toBe('error');
    expect(live.current.error).toBe('move 1 ("9 9") is not legal');
    expect(engine.stopped).toEqual(['a1']);
    await advance(THINK_MS * 2);
    expect(live.current.status).toBe('error');
  });

  it('shows a refused request', async () => {
    const { engine, live } = setup();
    engine.startError = new Error('too many analyses');
    live.begin('p', request());
    await advance(0);
    expect(live.current.status).toBe('error');
    expect(live.current.error).toBe('too many analyses');
  });

  it('shows a failed read, and stops reading', async () => {
    const { engine, live } = setup();
    live.begin('p', request());
    await advance(0);
    engine.getError = new Error('unknown analysis');
    await advance(POLL_MS);
    expect(live.current.status).toBe('error');
    expect(live.current.error).toBe('unknown analysis');
    const reads = engine.reads.length;
    await advance(1000);
    expect(engine.reads).toHaveLength(reads);
  });

  it('ignores the error of a request that was replaced', async () => {
    const { engine, live } = setup();
    engine.hold = true;
    live.begin('p1', request());
    await advance(0);
    engine.startError = new Error('late');
    engine.hold = false;
    live.begin('p2', request('1'));
    engine.startError = null;
    engine.release();
    await advance(0);
    expect(live.current.status).toBe('running');
    expect(live.current.position).toBe('p2');
    live.reset();
  });
});
