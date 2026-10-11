import { describe, expect, it } from 'vitest';
import {
  buildSessionRequest,
  clockLabel,
  clockOf,
  clockSeed,
  defaultRelease,
  defaultSecondRelease,
  orderReleases,
  releaseLabel,
  resolveSeed,
  type SetupForm,
} from './setup';

const computer: SetupForm = {
  mode: 'computer',
  release: 'uttt-v010',
  thinkMs: 100,
  side: 'x',
  minutes: 10,
  incrementSeconds: 0,
};

describe('buildSessionRequest', () => {
  it('makes two human seats for a friend game', () => {
    const form: SetupForm = { mode: 'friend', minutes: 5, incrementSeconds: 2 };
    expect(buildSessionRequest('uttt', form, 42, () => 0.9)).toEqual({
      game: 'uttt',
      seed: 42,
      opening_plies: 0,
      seats: [
        { kind: 'human', name: 'Player 1' },
        { kind: 'human', name: 'Player 2' },
      ],
    });
  });

  it('seats the human as X against a fixed-mode bot', () => {
    expect(buildSessionRequest('uttt', computer, 7, () => 0.9)).toEqual({
      game: 'uttt',
      seed: 7,
      opening_plies: 0,
      seats: [
        { kind: 'human', name: 'You' },
        { kind: 'bot', release: 'uttt-v010', think_ms: 100, mode: 'fixed' },
      ],
    });
  });

  it('seats the human as O after the bot', () => {
    const request = buildSessionRequest(
      'uttt',
      { ...computer, side: 'o', thinkMs: 500 },
      7,
      () => 0,
    );
    expect(request.seats).toEqual([
      { kind: 'bot', release: 'uttt-v010', think_ms: 500, mode: 'fixed' },
      { kind: 'human', name: 'You' },
    ]);
  });

  it('resolves a random side to seat 0 or 1', () => {
    const random = { ...computer, side: 'random' } as const;
    expect(buildSessionRequest('uttt', random, 1, () => 0.1).seats[0].kind).toBe('human');
    expect(buildSessionRequest('uttt', random, 1, () => 0.49).seats[0].kind).toBe('human');
    expect(buildSessionRequest('uttt', random, 1, () => 0.5).seats[1].kind).toBe('human');
    expect(buildSessionRequest('uttt', random, 1, () => 0.99).seats[1].kind).toBe('human');
  });

  it('makes two real-time bots with their own releases and think times', () => {
    const form: SetupForm = {
      mode: 'bots',
      xRelease: 'uttt-v010',
      oRelease: 'uttt-v009',
      xThinkMs: 25,
      oThinkMs: 2000,
    };
    expect(buildSessionRequest('uttt', form, 99, () => 0)).toEqual({
      game: 'uttt',
      seed: 99,
      opening_plies: 0,
      seats: [
        { kind: 'bot', release: 'uttt-v010', think_ms: 25, mode: 'realtime' },
        { kind: 'bot', release: 'uttt-v009', think_ms: 2000, mode: 'realtime' },
      ],
    });
  });

  it('passes the seed through untouched', () => {
    expect(buildSessionRequest('uttt', computer, 9_999_999_999, () => 0).seed).toBe(9_999_999_999);
  });
});

describe('clockOf', () => {
  it('keeps the chosen clock, and has none between bots', () => {
    expect(clockOf(computer)).toEqual({ minutes: 10, incrementSeconds: 0 });
    expect(
      clockOf({ mode: 'bots', xRelease: 'a', oRelease: 'b', xThinkMs: 25, oThinkMs: 25 }),
    ).toBeNull();
    expect(clockLabel({ minutes: 0.5, incrementSeconds: 3 })).toBe('½ + 3');
  });
});

describe('resolveSeed', () => {
  it('takes the typed seed only in advanced mode', () => {
    expect(resolveSeed(true, ' 1234 ', 5)).toBe(1234);
    expect(resolveSeed(false, '1234', 5)).toBe(5);
  });

  it('falls back to the clock on text that is not a safe whole number', () => {
    expect(resolveSeed(true, '', 12)).toBe(12);
    expect(resolveSeed(true, '-3', 12)).toBe(12);
    expect(resolveSeed(true, '1.5', 12)).toBe(12);
    expect(resolveSeed(true, '99999999999999999999', 12)).toBe(12);
  });

  it('keeps a clock seed under ten digits', () => {
    expect(clockSeed(1_790_000_123_456)).toBe(123_456);
    expect(clockSeed(1_799_999_999_999)).toBeLessThan(1e10);
  });
});

describe('releases', () => {
  const unordered = ['uttt-v002', 'uttt-v010', 'uttt-v009', 'uttt-v001'];

  it('orders newest first by version number, not by text', () => {
    expect(orderReleases(['uttt-v9', 'uttt-v10', 'uttt-v2'])).toEqual([
      'uttt-v10',
      'uttt-v9',
      'uttt-v2',
    ]);
    expect(orderReleases(unordered)).toEqual(['uttt-v010', 'uttt-v009', 'uttt-v002', 'uttt-v001']);
  });

  it('defaults to the latest, and the second bot to the second latest', () => {
    expect(defaultRelease(unordered)).toBe('uttt-v010');
    expect(defaultSecondRelease(unordered)).toBe('uttt-v009');
  });

  it('copes with one release and with none', () => {
    expect(defaultSecondRelease(['uttt-v001'])).toBe('uttt-v001');
    expect(defaultRelease([])).toBe('');
    expect(defaultSecondRelease([])).toBe('');
  });

  it('marks the latest in its label', () => {
    expect(releaseLabel('uttt-v010', 'uttt-v010')).toBe('uttt-v010 (latest)');
    expect(releaseLabel('uttt-v009', 'uttt-v010')).toBe('uttt-v009');
  });
});
