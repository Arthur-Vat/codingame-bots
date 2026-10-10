import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { startPoller } from './poller';
import { POLL_DONE_MS, POLL_LIVE_MS, pollDelay } from './useSession';

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('pollDelay', () => {
  it('polls fast while the game goes on and slowly after it ended', () => {
    expect(POLL_LIVE_MS).toBe(200);
    expect(POLL_DONE_MS).toBe(2000);
    expect(pollDelay('bot_thinking')).toBe(200);
    expect(pollDelay('waiting_human')).toBe(200);
    expect(pollDelay('measuring')).toBe(200);
    expect(pollDelay(undefined)).toBe(200);
    expect(pollDelay('over')).toBe(2000);
    expect(pollDelay('failed')).toBe(2000);
  });
});

describe('startPoller', () => {
  it('asks again after the delay the value chooses', async () => {
    const values = ['live', 'live', 'over', 'over'];
    const fetch = vi.fn(async () => values.shift() ?? 'over');
    const stop = startPoller({
      fetch,
      delayAfter: (value) => (value === 'over' ? 2000 : 200),
      delayAfterError: () => 2000,
      onValue: () => {},
      onError: () => {},
    });
    await vi.advanceTimersByTimeAsync(0);
    expect(fetch).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(199);
    expect(fetch).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(1);
    expect(fetch).toHaveBeenCalledTimes(2);
    await vi.advanceTimersByTimeAsync(200); // third: "over"
    expect(fetch).toHaveBeenCalledTimes(3);
    await vi.advanceTimersByTimeAsync(1999);
    expect(fetch).toHaveBeenCalledTimes(3);
    await vi.advanceTimersByTimeAsync(1);
    expect(fetch).toHaveBeenCalledTimes(4);
    stop();
  });

  it('does not start a request while the previous one is still running', async () => {
    let release: (value: string) => void = () => {};
    const fetch = vi.fn(() => new Promise<string>((resolve) => (release = resolve)));
    const stop = startPoller({
      fetch,
      delayAfter: () => 200,
      delayAfterError: () => 200,
      onValue: () => {},
      onError: () => {},
    });
    await vi.advanceTimersByTimeAsync(5000);
    expect(fetch).toHaveBeenCalledTimes(1);
    release('a');
    await vi.advanceTimersByTimeAsync(200);
    expect(fetch).toHaveBeenCalledTimes(2);
    stop();
  });

  it('drops the response that arrives after stop, and asks no more', async () => {
    let release: (value: string) => void = () => {};
    const fetch = vi.fn(() => new Promise<string>((resolve) => (release = resolve)));
    const onValue = vi.fn();
    const stop = startPoller({
      fetch,
      delayAfter: () => 200,
      delayAfterError: () => 200,
      onValue,
      onError: () => {},
    });
    stop();
    release('stale');
    await vi.advanceTimersByTimeAsync(10_000);
    expect(onValue).not.toHaveBeenCalled();
    expect(fetch).toHaveBeenCalledTimes(1);
  });

  it('drops the failure that arrives after stop', async () => {
    let fail: (error: Error) => void = () => {};
    const fetch = vi.fn(() => new Promise<string>((_, reject) => (fail = reject)));
    const onError = vi.fn();
    const stop = startPoller({
      fetch,
      delayAfter: () => 200,
      delayAfterError: () => 200,
      onValue: () => {},
      onError,
    });
    stop();
    fail(new Error('late'));
    await vi.advanceTimersByTimeAsync(10_000);
    expect(onError).not.toHaveBeenCalled();
  });

  it('stops a pending wait', async () => {
    const fetch = vi.fn(async () => 'a');
    const stop = startPoller({
      fetch,
      delayAfter: () => 200,
      delayAfterError: () => 200,
      onValue: () => {},
      onError: () => {},
    });
    await vi.advanceTimersByTimeAsync(0);
    stop();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(fetch).toHaveBeenCalledTimes(1);
  });

  it('retries after a failure with the error delay, and reports it', async () => {
    const outcomes = [new Error('down'), 'up'];
    const fetch = vi.fn(async () => {
      const outcome = outcomes.shift();
      if (outcome instanceof Error) throw outcome;
      return outcome ?? 'up';
    });
    const onError = vi.fn();
    const onValue = vi.fn();
    const stop = startPoller({
      fetch,
      delayAfter: () => null,
      delayAfterError: () => 3000,
      onValue,
      onError,
    });
    await vi.advanceTimersByTimeAsync(0);
    expect(onError).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(2999);
    expect(fetch).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(1);
    expect(onValue).toHaveBeenCalledWith('up');
    // null from delayAfter ends the polling.
    await vi.advanceTimersByTimeAsync(60_000);
    expect(fetch).toHaveBeenCalledTimes(2);
    stop();
  });
});
