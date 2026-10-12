import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { HistoryEntry } from './api';
import {
  arenaBase,
  arenaRecordText,
  forgetArenaIndex,
  listArena,
  parseIndex,
  PUBLISHED_ARENA_URL,
  validArenaId,
  validRow,
} from './arena';
import { setEngineKind } from './engine/detect';

const row = (patch: Record<string, unknown> = {}): Record<string, unknown> => ({
  id: 'uttt-1760000000-aaaaaaaaaaaa',
  game: 'uttt',
  unix_time: 1_760_000_000,
  players: ['uttt-v010', 'uttt-v009'],
  winner: 0,
  end: 'finished',
  turns: 41,
  source: 'arena match',
  ...patch,
});

const BASE = PUBLISHED_ARENA_URL;

/** A `fetch` that answers by URL; an unknown URL is a 404. Records what it was asked. */
function fetcher(
  answers: Record<string, { status?: number; body?: string; type?: string } | Error>,
): { fetch: typeof fetch; urls: string[]; inits: (RequestInit | undefined)[] } {
  const urls: string[] = [];
  const inits: (RequestInit | undefined)[] = [];
  const fake = (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
    const url = String(input);
    urls.push(url);
    inits.push(init);
    const answer = answers[url] ?? { status: 404, body: 'not found' };
    if (answer instanceof Error) return Promise.reject(answer);
    return Promise.resolve(
      new Response(answer.body ?? '', {
        status: answer.status ?? 200,
        headers: { 'Content-Type': answer.type ?? 'application/json' },
      }),
    );
  };
  return { fetch: fake as typeof fetch, urls, inits };
}

/** A filter that keeps the rows as they are, and remembers the query. */
function recordingFilter() {
  const calls: { query: string; rows: readonly HistoryEntry[] }[] = [];
  const filter = (query: string, rows: readonly HistoryEntry[]): Promise<HistoryEntry[]> => {
    calls.push({ query, rows });
    return Promise.resolve([...rows]);
  };
  return { filter, calls };
}

beforeEach(() => {
  forgetArenaIndex();
  setEngineKind('server');
});

afterEach(() => {
  setEngineKind('server');
  vi.unstubAllGlobals();
});

describe('where the arena games are', () => {
  it('is the published site for the local server, and the page-relative folder in the browser', () => {
    expect(PUBLISHED_ARENA_URL).toBe('https://arthur-vat.github.io/codingame-bots/arena-games/');
    setEngineKind('server');
    expect(arenaBase()).toBe(PUBLISHED_ARENA_URL);
    vi.stubGlobal('document', { baseURI: 'https://example.org/sub/page/' });
    setEngineKind('browser');
    expect(arenaBase()).toBe('https://example.org/sub/page/arena-games/');
  });

  it('accepts only ids of a-z, 0-9 and dashes', () => {
    expect(validArenaId('uttt-1760000000-0123456789ab')).toBe(true);
    for (const id of ['', 'A', 'a/b', '../x', 'a b', 'a.json', 'a?b', 'a#b', 'é']) {
      expect(validArenaId(id)).toBe(false);
    }
  });
});

describe('the index', () => {
  it('keeps the rows of a good index, rebuilt from the fields they must have', () => {
    const parsed = parseIndex({
      format: 1,
      rows: [row(), row({ id: 'uttt-2-bbbbbbbbbbbb', winner: null, extra: '<script>' })],
    });
    expect(parsed.dropped).toBe(0);
    expect(parsed.rows).toHaveLength(2);
    expect(parsed.rows[1]).toEqual({
      id: 'uttt-2-bbbbbbbbbbbb',
      game: 'uttt',
      unix_time: 1_760_000_000,
      players: ['uttt-v010', 'uttt-v009'],
      winner: null,
      end: 'finished',
      turns: 41,
      source: 'arena match',
    });
  });

  it('accepts an empty index', () => {
    expect(parseIndex({ format: 1, rows: [] })).toEqual({ rows: [], dropped: 0 });
  });

  it('refuses another format, a missing list and what is not an object', () => {
    expect(() => parseIndex({ format: 2, rows: [] })).toThrow('format is not 1');
    expect(() => parseIndex({ rows: [] })).toThrow('format is not 1');
    expect(() => parseIndex({ format: 1 })).toThrow('no list of rows');
    expect(() => parseIndex({ format: 1, rows: {} })).toThrow('no list of rows');
    expect(() => parseIndex([])).toThrow();
    expect(() => parseIndex(null)).toThrow('not an object');
    expect(() => parseIndex('x')).toThrow('not an object');
  });

  it('drops, and counts, the rows with an id that cannot be used or a field missing', () => {
    const parsed = parseIndex({
      format: 1,
      rows: [
        row({ id: '../etc/passwd' }),
        row({ id: 'UPPER' }),
        row({ id: 'a/b' }),
        row({ id: 7 }),
        row({ id: 'uttt-3-cccccccccccc', game: undefined }),
        row({ id: 'uttt-4-dddddddddddd', players: ['only one'] }),
        row({ id: 'uttt-5-eeeeeeeeeeee', unix_time: -1 }),
        row({ id: 'uttt-6-ffffffffffff', turns: 1.5 }),
        row({ id: 'uttt-7-000000000000', winner: 'x' }),
        'text',
        null,
        row({ id: 'uttt-8-111111111111' }),
        // The same id twice is one game.
        row({ id: 'uttt-8-111111111111' }),
      ],
    });
    expect(parsed.rows.map((kept) => kept.id)).toEqual(['uttt-8-111111111111']);
    expect(parsed.dropped).toBe(12);
  });

  it('checks a row on its own', () => {
    expect(validRow(row())).not.toBeNull();
    expect(validRow(row({ end: 3 }))).toBeNull();
    expect(validRow(row({ source: null }))).toBeNull();
    expect(validRow(row({ players: ['a', 2] }))).toBeNull();
    expect(validRow([])).toBeNull();
  });
});

describe('listing the arena games', () => {
  it('fetches the index once, whatever the number of queries, and filters its rows', async () => {
    const rows = Array.from({ length: 600 }, (_, n) => row({ id: `uttt-${n}-aaaaaaaaaaaa` }));
    const served = fetcher({
      [`${BASE}index.json`]: { body: JSON.stringify({ format: 1, rows }) },
    });
    const { filter, calls } = recordingFilter();

    const all = await listArena('', served.fetch, filter);
    expect(all).toMatchObject({ published: true, dropped: 0 });
    expect(all.entries).toHaveLength(600);
    await listArena('?release=uttt-v010', served.fetch, filter);

    expect(served.urls).toEqual([`${BASE}index.json`]);
    expect(served.inits[0]).toEqual({ cache: 'no-cache' });
    expect(calls.map((call) => call.query)).toEqual(['', '?release=uttt-v010']);
    expect(calls[0]?.rows).toHaveLength(600);
  });

  it('says nothing is published for a 404, and for a fallback page', async () => {
    const { filter, calls } = recordingFilter();
    const missing = await listArena('', fetcher({}).fetch, filter);
    expect(missing).toEqual({ entries: [], published: false, dropped: 0 });

    forgetArenaIndex();
    const page = fetcher({
      [`${BASE}index.json`]: { body: '<!doctype html>', type: 'text/html; charset=utf-8' },
    });
    expect((await listArena('', page.fetch, filter)).published).toBe(false);
    expect(calls).toEqual([]);
  });

  it('lists nothing for an empty index, without asking the filter', async () => {
    const { filter, calls } = recordingFilter();
    const served = fetcher({
      [`${BASE}index.json`]: { body: JSON.stringify({ format: 1, rows: [] }) },
    });
    expect(await listArena('', served.fetch, filter)).toEqual({
      entries: [],
      published: true,
      dropped: 0,
    });
    expect(calls).toEqual([]);
  });

  it('reports the rows it dropped', async () => {
    const served = fetcher({
      [`${BASE}index.json`]: {
        body: JSON.stringify({ format: 1, rows: [row(), row({ id: '../x' })] }),
      },
    });
    const listing = await listArena('', served.fetch, recordingFilter().filter);
    expect(listing.entries).toHaveLength(1);
    expect(listing.dropped).toBe(1);
  });

  it('names the URL when the network fails, the JSON is bad or the format is wrong', async () => {
    const { filter } = recordingFilter();
    const down = fetcher({ [`${BASE}index.json`]: new TypeError('Failed to fetch') });
    await expect(listArena('', down.fetch, filter)).rejects.toThrow(
      `Cannot load the arena games from ${BASE}index.json: Failed to fetch`,
    );
    const bad = fetcher({ [`${BASE}index.json`]: { body: '{nope' } });
    await expect(listArena('', bad.fetch, filter)).rejects.toThrow(
      `The arena games at ${BASE}index.json are not valid JSON`,
    );
    const wrong = fetcher({ [`${BASE}index.json`]: { body: '{"format":2,"rows":[]}' } });
    await expect(listArena('', wrong.fetch, filter)).rejects.toThrow(
      `The arena games at ${BASE}index.json cannot be used: its format is not 1 (2)`,
    );
    const broken = fetcher({ [`${BASE}index.json`]: { status: 503, body: 'down' } });
    await expect(listArena('', broken.fetch, filter)).rejects.toThrow(
      `Cannot load the arena games from ${BASE}index.json: HTTP 503`,
    );
  });

  it('tries again after a failure, and keeps a good answer', async () => {
    const { filter } = recordingFilter();
    const flaky = fetcher({ [`${BASE}index.json`]: new TypeError('offline') });
    await expect(listArena('', flaky.fetch, filter)).rejects.toThrow('offline');
    const good = fetcher({
      [`${BASE}index.json`]: { body: JSON.stringify({ format: 1, rows: [row()] }) },
    });
    expect((await listArena('', good.fetch, filter)).entries).toHaveLength(1);
    expect((await listArena('', flaky.fetch, filter)).entries).toHaveLength(1);
    expect(flaky.urls).toHaveLength(1);
  });

  it('passes the failure of the filter on', async () => {
    const served = fetcher({
      [`${BASE}index.json`]: { body: JSON.stringify({ format: 1, rows: [row()] }) },
    });
    const refuse = () => Promise.reject(new Error('unknown result'));
    await expect(listArena('?result=bogus', served.fetch, refuse)).rejects.toThrow(
      'unknown result',
    );
  });
});

describe('a record', () => {
  it('is fetched as text from the records folder', async () => {
    const text = '{"format":1,\n "seed": 12345678901234567890}';
    const served = fetcher({ [`${BASE}records/uttt-1-abc.json`]: { body: text } });
    expect(await arenaRecordText('uttt-1-abc', served.fetch)).toBe(text);
    expect(served.urls).toEqual([`${BASE}records/uttt-1-abc.json`]);
  });

  it('is not fetched for an id that could leave the folder', async () => {
    const served = fetcher({});
    for (const id of ['', '../index', 'a/b', 'A', 'a.json', 'a?x=1']) {
      await expect(arenaRecordText(id, served.fetch)).rejects.toThrow('is not the id');
    }
    expect(served.urls).toEqual([]);
  });

  it('says which game and URL it could not get', async () => {
    const url = `${BASE}records/uttt-1-abc.json`;
    await expect(arenaRecordText('uttt-1-abc', fetcher({}).fetch)).rejects.toThrow(
      `The arena game uttt-1-abc is not published (${url})`,
    );
    await expect(
      arenaRecordText('uttt-1-abc', fetcher({ [url]: new TypeError('offline') }).fetch),
    ).rejects.toThrow(`Cannot load the arena game from ${url}: offline`);
    await expect(
      arenaRecordText('uttt-1-abc', fetcher({ [url]: { status: 500 } }).fetch),
    ).rejects.toThrow(`Cannot load the arena game from ${url}: HTTP 500`);
  });
});
