import { describe, expect, it } from 'vitest';
import { RELOAD_KEY, ensureIsolation, type IsolationEnvironment } from './isolation';

function environment(patch: Partial<IsolationEnvironment> = {}) {
  const items = new Map<string, string>();
  const calls = { register: 0, reload: 0 };
  const env: IsolationEnvironment = {
    isolated: false,
    supported: true,
    register: async () => {
      calls.register += 1;
      return true;
    },
    reload: () => {
      calls.reload += 1;
    },
    storage: {
      getItem: (key) => items.get(key) ?? null,
      setItem: (key, value) => void items.set(key, value),
      removeItem: (key) => void items.delete(key),
    },
    ...patch,
  };
  return { env, items, calls };
}

/** Whether a promise is still waiting after the microtasks ran. */
async function pending(promise: Promise<unknown>): Promise<boolean> {
  const marker = Symbol('pending');
  return (await Promise.race([promise, Promise.resolve(marker)])) === marker;
}

describe('cross-origin isolation', () => {
  it('does nothing on a page that is isolated already', async () => {
    const { env, calls } = environment({ isolated: true });
    expect(await ensureIsolation(env)).toBe(true);
    expect(calls).toEqual({ register: 0, reload: 0 });
  });

  it('registers the worker and reloads once on the first visit, and then never returns', async () => {
    const { env, items, calls } = environment();
    const outcome = ensureIsolation(env);
    expect(await pending(outcome)).toBe(true);
    expect(calls).toEqual({ register: 1, reload: 1 });
    expect(items.get(RELOAD_KEY)).toBe('1');
  });

  it('does not reload a second time when the page is still not isolated', async () => {
    const { env, calls } = environment();
    void ensureIsolation(env);
    await Promise.resolve();
    // The reloaded page: same tab, so the guard is still there.
    const again = environment({ storage: env.storage });
    expect(await ensureIsolation(again.env)).toBe(false);
    expect(again.calls).toEqual({ register: 0, reload: 0 });
    expect(calls.reload).toBe(1);
  });

  it('forgets the guard once isolated, so that a later visit may try again', async () => {
    const { env, items } = environment();
    items.set(RELOAD_KEY, '1');
    env.isolated = true;
    expect(await ensureIsolation(env)).toBe(true);
    expect(items.has(RELOAD_KEY)).toBe(false);
  });

  it('gives up without reloading when the worker cannot be registered', async () => {
    const { env, calls } = environment({ register: async () => false });
    expect(await ensureIsolation(env)).toBe(false);
    expect(calls.reload).toBe(0);
  });

  it('gives up where there is no service worker, or no storage to guard the reload with', async () => {
    const none = environment({ supported: false });
    expect(await ensureIsolation(none.env)).toBe(false);
    expect(none.calls).toEqual({ register: 0, reload: 0 });
    const noStorage = environment({ storage: null });
    expect(await ensureIsolation(noStorage.env)).toBe(false);
    expect(noStorage.calls).toEqual({ register: 0, reload: 0 });
  });

  it('does not reload when the storage throws, since a loop could not be prevented', async () => {
    const broken = environment({
      storage: {
        getItem: () => {
          throw new Error('blocked');
        },
        setItem: () => {
          throw new Error('blocked');
        },
        removeItem: () => {},
      },
    });
    expect(await ensureIsolation(broken.env)).toBe(false);
    expect(broken.calls).toEqual({ register: 0, reload: 0 });
  });
});
