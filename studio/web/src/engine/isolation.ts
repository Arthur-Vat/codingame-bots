/* Cross-origin isolation for the browser engine. A bot's blocking read waits on a
   SharedArrayBuffer, which browsers allow only on an isolated page (COOP and COEP headers). A
   static host such as GitHub Pages sets no headers, so a service worker of ours (public/coi-sw.js)
   adds them: the first visit registers it and reloads the page once, and later ones are isolated
   from the start. Where that fails the page still works for games between humans, the history and
   reviews; only what needs a bot says so. */

export interface IsolationEnvironment {
  isolated: boolean;
  /** Whether a service worker can be registered here. */
  supported: boolean;
  /** Registers the service worker and waits until it is active, or gives up. */
  register: () => Promise<boolean>;
  reload: () => void;
  storage: Pick<Storage, 'getItem' | 'setItem' | 'removeItem'> | null;
}

/** The key of sessionStorage that remembers a reload was done, so that none is done twice. */
export const RELOAD_KEY = 'studio.coi-reload';

/** How long to wait for the service worker to become active. */
export const REGISTER_TIMEOUT_MS = 10_000;

/**
 * Makes the page isolated if it can: true when it is isolated, false when it is not and will not
 * be. When it has registered the service worker it reloads the page and never returns.
 */
export async function ensureIsolation(env: IsolationEnvironment): Promise<boolean> {
  if (env.isolated) {
    // A later visit that needs the worker again may reload again.
    forget(env);
    return true;
  }
  if (!env.supported || env.storage === null) return false;
  if (remembered(env)) return false;
  remember(env);
  if (!(await env.register())) return false;
  env.reload();
  return new Promise<boolean>(() => {});
}

function remembered(env: IsolationEnvironment): boolean {
  try {
    return env.storage?.getItem(RELOAD_KEY) !== null;
  } catch {
    // Without storage the guard cannot work: do not reload.
    return true;
  }
}

function remember(env: IsolationEnvironment): void {
  try {
    env.storage?.setItem(RELOAD_KEY, '1');
  } catch {
    // Checked by `remembered`.
  }
}

function forget(env: IsolationEnvironment): void {
  try {
    env.storage?.removeItem(RELOAD_KEY);
  } catch {
    // Nothing to forget.
  }
}

/** The environment of the page. */
export function pageEnvironment(): IsolationEnvironment {
  const storage = ((): Storage | null => {
    try {
      return sessionStorage;
    } catch {
      return null;
    }
  })();
  return {
    isolated: globalThis.crossOriginIsolated === true,
    supported: 'serviceWorker' in navigator && globalThis.isSecureContext,
    register: async () => {
      const active = (async () => {
        // Relative to the page, so that it works under a subpath.
        await navigator.serviceWorker.register('./coi-sw.js');
        await navigator.serviceWorker.ready;
        return true;
      })();
      const timeout = new Promise<boolean>((resolve) =>
        setTimeout(() => resolve(false), REGISTER_TIMEOUT_MS),
      );
      try {
        return await Promise.race([active, timeout]);
      } catch {
        return false;
      }
    },
    reload: () => location.reload(),
    storage,
  };
}
