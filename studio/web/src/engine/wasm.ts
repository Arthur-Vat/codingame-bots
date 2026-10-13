/* Where the WebAssembly files come from: `wasm/rules.wasm`, `wasm/manifest.json` and
   `wasm/<game>/<release>.wasm`, built by scripts/build-studio-wasm.sh. They are fetched relative
   to the page (the site may live under a subpath), compiled once, and kept for the page's life. */

export interface Manifest {
  /** `analysis`: whether `wasm/<game>/analysis.wasm`, the game's analysis program, is there. */
  games: { id: string; releases: string[]; analysis: boolean }[];
}

/** A compiled release and the SHA-256 of its file (null where the browser cannot compute it). */
export interface LoadedRelease {
  module: WebAssembly.Module;
  hash: string | null;
}

export interface WasmSource {
  manifest: () => Promise<Manifest>;
  rules: () => Promise<WebAssembly.Module>;
  release: (game: string, release: string) => Promise<LoadedRelease>;
  /** The game's analysis program (`wasm/<game>/analysis.wasm`), compiled once and reused. */
  analysis: (game: string) => Promise<WebAssembly.Module>;
}

/** The hex SHA-256 of `bytes`, or null when `crypto.subtle` is not there (an insecure page). */
export async function sha256Hex(bytes: ArrayBuffer): Promise<string | null> {
  const subtle = globalThis.crypto?.subtle;
  if (subtle === undefined) return null;
  const digest = new Uint8Array(await subtle.digest('SHA-256', bytes));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

export const MISSING_FILES = 'run scripts/build-studio-wasm.sh to build them';

/** Reads `manifest.json` and checks its shape. */
export function parseManifest(text: string): Manifest {
  let json: unknown;
  try {
    json = JSON.parse(text);
  } catch {
    throw new Error(`wasm/manifest.json is not JSON (${MISSING_FILES})`);
  }
  const games = (json as { games?: unknown } | null)?.games;
  if (!Array.isArray(games)) throw new Error('wasm/manifest.json has no list of games');
  return {
    games: games.map((entry: unknown) => {
      const { id, releases, analysis } = (entry ?? {}) as {
        id?: unknown;
        releases?: unknown;
        analysis?: unknown;
      };
      if (typeof id !== 'string' || !Array.isArray(releases)) {
        throw new Error('wasm/manifest.json lists a game wrongly');
      }
      return {
        id,
        releases: releases.filter((name): name is string => typeof name === 'string'),
        analysis: analysis === true,
      };
    }),
  };
}

/** Fetches the files from `base` (a URL ending with `/`, the folder `wasm/`). */
export function fetchedWasm(base: string, fetcher: typeof fetch = fetch): WasmSource {
  const bytesOf = async (path: string): Promise<ArrayBuffer> => {
    const response = await fetcher(new URL(path, base));
    if (!response.ok) throw new Error(`cannot load wasm/${path} (HTTP ${response.status})`);
    return response.arrayBuffer();
  };
  // A failed load is forgotten, so that the next request tries again.
  const once = <T>(cache: Map<string, Promise<T>>, key: string, load: () => Promise<T>) => {
    let promise = cache.get(key);
    if (promise === undefined) {
      promise = load().catch((error: unknown) => {
        cache.delete(key);
        throw error;
      });
      cache.set(key, promise);
    }
    return promise;
  };
  const manifests = new Map<string, Promise<Manifest>>();
  const rules = new Map<string, Promise<WebAssembly.Module>>();
  const releases = new Map<string, Promise<LoadedRelease>>();
  const analyses = new Map<string, Promise<WebAssembly.Module>>();
  return {
    manifest: () =>
      once(manifests, 'manifest', async () => {
        const response = await fetcher(new URL('manifest.json', base));
        if (!response.ok) {
          throw new Error(`cannot load wasm/manifest.json (HTTP ${response.status})`);
        }
        return parseManifest(await response.text());
      }),
    rules: () => once(rules, 'rules', async () => WebAssembly.compile(await bytesOf('rules.wasm'))),
    release: (game, release) =>
      once(releases, `${game}/${release}`, async () => {
        const bytes = await bytesOf(`${game}/${release}.wasm`);
        return { module: await WebAssembly.compile(bytes), hash: await sha256Hex(bytes) };
      }),
    analysis: (game) =>
      once(analyses, game, async () => WebAssembly.compile(await bytesOf(`${game}/analysis.wasm`))),
  };
}
