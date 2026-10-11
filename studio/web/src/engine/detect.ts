/* Which engine the front end uses (ADR 0030, decision 2), decided once before the app renders:
   the local server's API when there is one, the engine in the browser otherwise. */

export type EngineKind = 'server' | 'browser';

let chosen: EngineKind = 'server';

/** The engine in use. Until `chooseEngine` has run it is the server's, the front end's default. */
export function engineKind(): EngineKind {
  return chosen;
}

/** For tests. */
export function setEngineKind(kind: EngineKind): void {
  chosen = kind;
}

/** The key of localStorage that forces an engine (`server` or `browser`), to diagnose or to test. */
export const FORCE_KEY = 'studio.engine';

function forced(): EngineKind | null {
  try {
    const value = localStorage.getItem(FORCE_KEY);
    return value === 'server' || value === 'browser' ? value : null;
  } catch {
    return null;
  }
}

/**
 * Whether `GET /api/games` is the studio server's: a 200 whose body is a JSON array. A 404, a page
 * of HTML (a static host's fallback), an error or no answer at all is not.
 */
export async function serverIsThere(fetcher: typeof fetch = fetch): Promise<boolean> {
  try {
    const response = await fetcher('/api/games');
    if (response.status !== 200) return false;
    return Array.isArray(JSON.parse(await response.text()));
  } catch {
    return false;
  }
}

/** Decides the engine and remembers it. */
export async function chooseEngine(fetcher: typeof fetch = fetch): Promise<EngineKind> {
  chosen = forced() ?? ((await serverIsThere(fetcher)) ? 'server' : 'browser');
  return chosen;
}
