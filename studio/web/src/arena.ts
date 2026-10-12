/* The arena games (ADR 0031): the games played by the releases on GitHub, published read-only
   with the site as `arena-games/index.json` and `arena-games/records/<id>.json`. This module is
   the only place that knows where they are and how to read them; they come over the network, so
   the index is validated here and a record is checked by the rules when it is shown or saved. */

import { filterRows, type HistoryEntry } from './api';
import { engineKind } from './engine/detect';

/** Where the site published on GitHub Pages keeps the arena games. */
export const PUBLISHED_ARENA_URL = 'https://arthur-vat.github.io/codingame-bots/arena-games/';

/**
 * The folder of the published files. The browser engine is served by that site, so the folder is
 * relative to the page (the site may live under a subpath); the local server has none, so its
 * front end reads the published site, with CORS and no credentials.
 */
export function arenaBase(): string {
  return engineKind() === 'browser'
    ? new URL('arena-games/', document.baseURI).href
    : PUBLISHED_ARENA_URL;
}

/** Whether `id` can be an arena game's id: `a-z`, `0-9` and `-`, as for a saved game. */
export function validArenaId(id: string): boolean {
  return /^[a-z0-9-]+$/.test(id);
}

function isCount(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
}

/** The row, rebuilt from the fields it must have, or null: the filter would refuse a bad one. */
export function validRow(value: unknown): HistoryEntry | null {
  if (typeof value !== 'object' || value === null) return null;
  const row = value as Record<string, unknown>;
  const { id, game, unix_time: time, players, winner, end, turns, source } = row;
  if (typeof id !== 'string' || !validArenaId(id)) return null;
  if (typeof game !== 'string' || !isCount(time) || !isCount(turns)) return null;
  if (!Array.isArray(players) || players.length !== 2) return null;
  const [first, second] = players as unknown[];
  if (typeof first !== 'string' || typeof second !== 'string') return null;
  if (winner !== null && !isCount(winner)) return null;
  if (typeof end !== 'string' || typeof source !== 'string') return null;
  return { id, game, unix_time: time, players: [first, second], winner, end, turns, source };
}

/** The parsed index: its valid rows, and how many it listed that were not. */
export interface ArenaIndex {
  rows: HistoryEntry[];
  dropped: number;
}

/** Reads the index's JSON: format 1 and an array of rows. A row that is not valid is dropped. */
export function parseIndex(value: unknown): ArenaIndex {
  if (typeof value !== 'object' || value === null) {
    throw new Error('it is not an object');
  }
  const { format, rows } = value as Record<string, unknown>;
  if (format !== 1) throw new Error(`its format is not 1 (${JSON.stringify(format) ?? 'missing'})`);
  if (!Array.isArray(rows)) throw new Error('it has no list of rows');
  const valid: HistoryEntry[] = [];
  const seen = new Set<string>();
  let dropped = 0;
  for (const candidate of rows as unknown[]) {
    const row = validRow(candidate);
    // A repeated id would be two rows for one record.
    if (row === null || seen.has(row.id)) {
      dropped += 1;
    } else {
      seen.add(row.id);
      valid.push(row);
    }
  }
  return { rows: valid, dropped };
}

/** What the arena games are right now: the index, or that nothing is published. */
type Published = { published: true; index: ArenaIndex } | { published: false };

/** The index of each folder, kept for the life of the page (a build of the site replaces it). */
const loaded = new Map<string, Promise<Published>>();

/** For tests: forgets the indexes read so far. */
export function forgetArenaIndex(): void {
  loaded.clear();
}

function describe(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

async function readIndex(url: string, fetcher: typeof fetch): Promise<Published> {
  let response: Response;
  try {
    response = await fetcher(url, { cache: 'no-cache' });
  } catch (error) {
    throw new Error(`Cannot load the arena games from ${url}: ${describe(error)}`, {
      cause: error,
    });
  }
  if (response.status === 404) return { published: false };
  if (!response.ok) {
    throw new Error(`Cannot load the arena games from ${url}: HTTP ${response.status}`);
  }
  // A static host without the file may answer with its fallback page instead of a 404.
  if ((response.headers.get('content-type') ?? '').toLowerCase().includes('text/html')) {
    return { published: false };
  }
  let json: unknown;
  try {
    json = JSON.parse(await response.text());
  } catch (error) {
    throw new Error(`The arena games at ${url} are not valid JSON: ${describe(error)}`, {
      cause: error,
    });
  }
  try {
    return { published: true, index: parseIndex(json) };
  } catch (error) {
    throw new Error(`The arena games at ${url} cannot be used: ${describe(error)}`, {
      cause: error,
    });
  }
}

/** The index, read once per folder; a failure is not kept, so that the next call tries again. */
function published(fetcher: typeof fetch): Promise<Published> {
  const url = `${arenaBase()}index.json`;
  let found = loaded.get(url);
  if (found === undefined) {
    found = readIndex(url, fetcher);
    loaded.set(url, found);
    found.catch(() => {
      if (loaded.get(url) === found) loaded.delete(url);
    });
  }
  return found;
}

/** The arena games that pass a filter, newest first. */
export interface ArenaListing {
  entries: HistoryEntry[];
  /** False when nothing is published at all (the index is missing). */
  published: boolean;
  /** Rows of the index that were not valid and are not listed. */
  dropped: number;
}

/**
 * The arena games for the history query (`''` or a string starting with `?`). The index is
 * fetched once; the filter is the one of the saved games (`POST /api/filter`), so a list of 500
 * rows costs one request and no record is fetched. Rejects with a message naming the URL when the
 * index cannot be read.
 */
export async function listArena(
  query: string,
  fetcher: typeof fetch = fetch,
  filter: typeof filterRows = filterRows,
): Promise<ArenaListing> {
  const found = await published(fetcher);
  if (!found.published) return { entries: [], published: false, dropped: 0 };
  const { rows, dropped } = found.index;
  const entries = rows.length === 0 ? [] : await filter(query, rows);
  return { entries, published: true, dropped };
}

/** The text of an arena game's record, as published. Rejects for an id that cannot be one. */
export async function arenaRecordText(id: string, fetcher: typeof fetch = fetch): Promise<string> {
  if (!validArenaId(id)) throw new Error(`"${id}" is not the id of an arena game.`);
  const url = `${arenaBase()}records/${id}.json`;
  let response: Response;
  try {
    response = await fetcher(url, { cache: 'no-cache' });
  } catch (error) {
    throw new Error(`Cannot load the arena game from ${url}: ${describe(error)}`, { cause: error });
  }
  if (response.status === 404) {
    throw new Error(`The arena game ${id} is not published (${url}).`);
  }
  if (!response.ok) {
    throw new Error(`Cannot load the arena game from ${url}: HTTP ${response.status}`);
  }
  try {
    return await response.text();
  } catch (error) {
    throw new Error(`Cannot load the arena game from ${url}: ${describe(error)}`, { cause: error });
  }
}
