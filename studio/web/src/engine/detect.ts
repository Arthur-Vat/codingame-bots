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

/** How the engine was chosen: forced through localStorage, or by the probe (and what it found). */
export interface EngineChoice {
  forced: boolean;
  serverFound: boolean;
  /** Whether the page tells the developer that no server answered (`devNoServerNotice`). */
  notice: boolean;
}

let choice: EngineChoice = { forced: false, serverFound: true, notice: false };

/** How the last `chooseEngine` decided. */
export function engineChoice(): EngineChoice {
  return choice;
}

/**
 * Whether to tell the developer that no studio server answered: only in development (`dev`),
 * when the probe chose (`forced` is false) and found no server.
 */
export function devNoServerNotice(input: {
  dev: boolean;
  forced: boolean;
  serverFound: boolean;
}): boolean {
  return input.dev && !input.forced && !input.serverFound;
}

/** The notice about a missing server in development, as plain text (the console's). */
export const NO_SERVER_TEXT =
  "No studio server answered on port 8411, so this page uses the browser engine. Start the server with cargo run --release -p studio and reload, or set localStorage['studio.engine'] to 'browser' to use the browser engine on purpose.";

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

/**
 * Decides the engine and remembers it, and whether it was forced or probed. `dev` is whether this
 * is a development build, which the notice about a missing server depends on.
 */
export async function chooseEngine(
  fetcher: typeof fetch = fetch,
  dev: boolean = import.meta.env.DEV,
): Promise<EngineKind> {
  const force = forced();
  const serverFound = force === null ? await serverIsThere(fetcher) : force === 'server';
  chosen = force ?? (serverFound ? 'server' : 'browser');
  const isForced = force !== null;
  choice = {
    forced: isForced,
    serverFound,
    notice: devNoServerNotice({ dev, forced: isForced, serverFound }),
  };
  return chosen;
}
