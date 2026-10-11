import { describe, expect, it } from 'vitest';
import { clockClass, formatClock, formatSpent, initClocks, reconcile, tick } from './clock';

const settings = { minutes: 1, incrementSeconds: 2 };

describe('initClocks', () => {
  it('gives a clock to the timed seats only', () => {
    const state = initClocks(settings, [true, false], []);
    expect(state.remaining).toEqual([60_000, null]);
    expect(state.expired).toBeNull();
    expect(state.incrementMs).toBe(2000);
  });

  it('counts half a minute', () => {
    expect(initClocks({ minutes: 0.5, incrementSeconds: 0 }, [true, true], []).remaining).toEqual([
      30_000, 30_000,
    ]);
  });
});

describe('tick', () => {
  it('runs the clock of the seat to move only', () => {
    const state = initClocks(settings, [true, true], []);
    const { state: next, timeout } = tick(state, 1500, 1);
    expect(next.remaining).toEqual([60_000, 58_500]);
    expect(timeout).toBeNull();
  });

  it('does not run when nobody is to move or the seat has no clock', () => {
    const state = initClocks(settings, [true, false], []);
    expect(tick(state, 1000, null).state).toBe(state);
    expect(tick(state, 1000, 1).state).toBe(state);
  });

  it('fires the timeout once, at zero', () => {
    const state = initClocks({ minutes: 0.5, incrementSeconds: 0 }, [true, true], []);
    const first = tick(state, 29_000, 0);
    expect(first.timeout).toBeNull();
    const second = tick(first.state, 5000, 0);
    expect(second.timeout).toBe(0);
    expect(second.state.remaining[0]).toBe(0);
    expect(second.state.expired).toBe(0);
    const third = tick(second.state, 5000, 0);
    expect(third.timeout).toBeNull();
    expect(third.state).toBe(second.state);
    expect(tick(second.state, 5000, 1).state).toBe(second.state);
  });

  it('is paused when the caller says nobody runs (a request in flight, a rewind)', () => {
    const state = initClocks(settings, [true, true], []);
    expect(tick(state, 10_000, null).state.remaining).toEqual([60_000, 60_000]);
  });
});

describe('reconcile', () => {
  it('changes nothing when the turns are the same', () => {
    const state = initClocks(settings, [true, true], [0, 1]);
    expect(reconcile(state, [0, 1])).toBe(state);
  });

  it('adds the increment after a new move of a clocked seat', () => {
    let state = initClocks(settings, [true, true], []);
    state = tick(state, 5000, 0).state;
    state = reconcile(state, [0]);
    expect(state.remaining).toEqual([57_000, 60_000]);
    state = reconcile(state, [0, 1]);
    expect(state.remaining).toEqual([57_000, 62_000]);
  });

  it('adds nothing for a bot move, and nothing for turns present at the start', () => {
    const state = initClocks(settings, [true, false], [1, 0]);
    expect(state.remaining).toEqual([60_000, null]);
    expect(reconcile(state, [1, 0, 1]).remaining).toEqual([60_000, null]);
  });

  it('takes the increments of taken back moves off again', () => {
    let state = initClocks(settings, [true, false], []);
    state = reconcile(state, [0, 1, 0, 1]);
    expect(state.remaining[0]).toBe(64_000);
    state = reconcile(state, [0, 1]);
    expect(state.remaining[0]).toBe(62_000);
    expect(state.turns).toEqual([0, 1]);
  });

  it('keeps a clock at one second at least when undoing increments', () => {
    let state = initClocks({ minutes: 1, incrementSeconds: 30 }, [true, true], []);
    state = reconcile(state, [0]);
    state = tick(state, 85_000, 1).state;
    state = tick(state, 0, 1).state;
    const low = { ...state, remaining: [5000, 60_000] };
    expect(reconcile(low, []).remaining[0]).toBe(1000);
  });

  it('keeps an expiry through a later move, and drops it when turns are taken back', () => {
    let state = initClocks({ minutes: 0.5, incrementSeconds: 0 }, [true, true], []);
    state = tick(state, 31_000, 0).state;
    expect(state.expired).toBe(0);
    state = reconcile(state, [0]);
    expect(state.expired).toBe(0);
    state = reconcile(state, []);
    expect(state.expired).toBeNull();
  });

  it('follows a takeback followed by another move in one step', () => {
    let state = initClocks(settings, [true, true], []);
    state = reconcile(state, [0, 1, 0]);
    state = reconcile(state, [0, 1, 1]);
    expect(state.turns).toEqual([0, 1, 1]);
    expect(state.remaining).toEqual([62_000, 64_000]);
  });
});

describe('formatClock', () => {
  it('writes minutes and seconds', () => {
    expect(formatClock(600_000)).toBe('10:00');
    expect(formatClock(61_000)).toBe('1:01');
    expect(formatClock(10_000)).toBe('0:10');
  });

  it('shows tenths under ten seconds', () => {
    expect(formatClock(9_950)).toBe('0:09.9');
    expect(formatClock(450)).toBe('0:00.4');
    expect(formatClock(-3)).toBe('0:00.0');
  });
});

describe('formatSpent', () => {
  it('shows seconds with a decimal', () => {
    expect(formatSpent(1234)).toBe('1.2 s');
  });
});

describe('clockClass', () => {
  it('highlights the running clock and warns under 20 seconds', () => {
    expect(clockClass(60_000, false)).toBe('');
    expect(clockClass(60_000, true)).toBe('run');
    expect(clockClass(19_999, true)).toBe('low');
    expect(clockClass(19_999, false)).toBe('');
    expect(clockClass(null, true)).toBe('run');
  });
});
