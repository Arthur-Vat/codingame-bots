import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  MEASURE_ITERS,
  Speed,
  iterations,
  localStorageSpeedCache,
  measure,
  median,
  memorySpeedCache,
  rateFromMedian,
  speedKey,
} from './speed';
import { firstLegalMove, loadedRules, scriptedFactory, scriptedBot } from './testSupport';
import { sha256Hex } from './wasm';

afterEach(() => vi.unstubAllGlobals());

describe('median', () => {
  it('is the middle of an odd count, the mean of the middle two of an even one, 0 for none', () => {
    expect(median([5, 1, 3])).toBe(3);
    expect(median([4, 1, 3, 2])).toBe(2.5);
    expect(median([7])).toBe(7);
    expect(median([])).toBe(0);
  });

  it('does not reorder its input', () => {
    const samples = [3, 1, 2];
    median(samples);
    expect(samples).toEqual([3, 1, 2]);
  });
});

describe('iterations', () => {
  it('is the think time times the rate, rounded, and at least 1', () => {
    expect(iterations(100, 2.5)).toBe(250);
    expect(iterations(10, 0.123)).toBe(1);
    expect(iterations(10, 0.0001)).toBe(1);
    expect(iterations(1000, 3.3333)).toBe(3333);
  });
});

describe('rateFromMedian', () => {
  it('is the measuring iterations per millisecond, with a floor on the time', () => {
    expect(rateFromMedian(30)).toBe(MEASURE_ITERS / 30);
    expect(rateFromMedian(0)).toBe(MEASURE_ITERS / 0.001);
  });
});

describe('the measuring game', () => {
  it('plays the release against itself with 3000 iterations and its seeds', async () => {
    const rules = await loadedRules();
    const progress: number[] = [];
    const { factory, envs } = scriptedFactory([
      async (input) => [firstLegalMove(input)],
      async (input) => [firstLegalMove(input)],
    ]);
    const rate = await measure({
      rules,
      makeBot: factory,
      module: new WebAssembly.Module(new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0])),
      game: 'uttt',
      release: 'uttt-v001',
      progress: (done) => progress.push(done),
    });
    expect(rate).toBeGreaterThan(0);
    expect(Number.isFinite(rate)).toBe(true);
    expect(envs).toHaveLength(2);
    for (const env of envs) {
      expect(env.CG_FIXED_ITERS).toBe('3000');
      expect(env.CG_SEED).toMatch(/^\d{17,}$/);
    }
    expect(envs[0]?.CG_SEED).not.toBe(envs[1]?.CG_SEED);
    expect(progress.length).toBeGreaterThan(10);
    expect(progress).toEqual(progress.map((_, index) => index + 1));
  });

  it('fails when a bot answers against the rules, and stops the bots', async () => {
    const rules = await loadedRules();
    let stopped = 0;
    const factory = async () => {
      const bot = scriptedBot(async () => ['9 9']);
      const terminate = bot.terminate;
      bot.terminate = () => {
        stopped += 1;
        terminate();
      };
      return bot;
    };
    await expect(
      measure({
        rules,
        makeBot: factory,
        module: new WebAssembly.Module(new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0])),
        game: 'uttt',
        release: 'uttt-v001',
        progress: () => {},
      }),
    ).rejects.toThrow('answered against the rules');
    expect(stopped).toBe(2);
  });
});

describe('the speed cache', () => {
  it('keys a release by its name and the hash of its file, and keeps nothing without a hash', () => {
    expect(speedKey('uttt-v001', 'abc')).toBe('uttt-v001:abc');
    expect(speedKey('uttt-v001', null)).toBeNull();
    const speed = new Speed(memorySpeedCache());
    expect(speed.cached('uttt-v001', null)).toBeNull();
  });

  it('measures once for a release and keeps the result', async () => {
    let runs = 0;
    const speed = new Speed(memorySpeedCache(), async () => {
      runs += 1;
      return 4;
    });
    const options = {} as Parameters<Speed['rate']>[2];
    expect(await speed.rate('r', 'h', options)).toBe(4);
    expect(await speed.rate('r', 'h', options)).toBe(4);
    expect(speed.cached('r', 'h')).toBe(4);
    expect(runs).toBe(1);
    // Another file of the same name is another release.
    expect(speed.cached('r', 'other')).toBeNull();
  });

  it('measures one release at a time, and a second request for the same one waits for it', async () => {
    let running = 0;
    let most = 0;
    let runs = 0;
    const speed = new Speed(memorySpeedCache(), async () => {
      runs += 1;
      running += 1;
      most = Math.max(most, running);
      await new Promise((resolve) => setTimeout(resolve, 5));
      running -= 1;
      return 2;
    });
    const options = {} as Parameters<Speed['rate']>[2];
    await Promise.all([
      speed.rate('a', 'h', options),
      speed.rate('a', 'h', options),
      speed.rate('b', 'h', options),
    ]);
    expect(most).toBe(1);
    expect(runs).toBe(2);
  });

  it('says which release failed, and measures again next time', async () => {
    let attempts = 0;
    const speed = new Speed(memorySpeedCache(), async () => {
      attempts += 1;
      if (attempts === 1) throw new Error('boom');
      return 3;
    });
    const options = {} as Parameters<Speed['rate']>[2];
    await expect(speed.rate('uttt-v001', 'h', options)).rejects.toThrow(
      'cannot measure the speed of uttt-v001: boom',
    );
    expect(await speed.rate('uttt-v001', 'h', options)).toBe(3);
  });
});

describe('localStorage', () => {
  function storage(initial: Record<string, string> = {}) {
    const items = new Map(Object.entries(initial));
    return {
      items,
      getItem: (key: string) => items.get(key) ?? null,
      setItem: (key: string, value: string) => void items.set(key, value),
    };
  }

  it('keeps the rates between pages', () => {
    const fake = storage();
    vi.stubGlobal('localStorage', fake);
    localStorageSpeedCache().set('r:h', 2.5);
    expect(localStorageSpeedCache().get('r:h')).toBe(2.5);
    expect(localStorageSpeedCache().get('other:h')).toBeNull();
    localStorageSpeedCache().set('s:h', 1);
    expect(localStorageSpeedCache().get('r:h')).toBe(2.5);
  });

  it('ignores what is not a rate', () => {
    vi.stubGlobal(
      'localStorage',
      storage({ 'studio.speed.v1': JSON.stringify({ a: 'x', b: -1, c: null, d: 2 }) }),
    );
    const cache = localStorageSpeedCache();
    expect(cache.get('a')).toBeNull();
    expect(cache.get('b')).toBeNull();
    expect(cache.get('c')).toBeNull();
    expect(cache.get('d')).toBe(2);
    vi.stubGlobal('localStorage', storage({ 'studio.speed.v1': '{not json' }));
    expect(localStorageSpeedCache().get('d')).toBeNull();
  });

  it('forgets when the storage is blocked, and the release is measured again', async () => {
    const blocked = () => {
      throw new Error('blocked');
    };
    vi.stubGlobal('localStorage', { getItem: blocked, setItem: blocked });
    const cache = localStorageSpeedCache();
    expect(() => cache.set('r:h', 2)).not.toThrow();
    expect(cache.get('r:h')).toBeNull();
    let runs = 0;
    const speed = new Speed(cache, async () => {
      runs += 1;
      return 2;
    });
    const options = {} as Parameters<Speed['rate']>[2];
    await speed.rate('r', 'h', options);
    await speed.rate('r', 'h', options);
    expect(runs).toBe(2);
  });

  it('is absent in a page without localStorage', () => {
    vi.stubGlobal('localStorage', undefined);
    const cache = localStorageSpeedCache();
    expect(cache.get('r:h')).toBeNull();
    expect(() => cache.set('r:h', 2)).not.toThrow();
  });
});

describe('sha256Hex', () => {
  it('hashes bytes', async () => {
    const bytes = new TextEncoder().encode('abc');
    expect(await sha256Hex(bytes.buffer as ArrayBuffer)).toBe(
      'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad',
    );
  });
});
