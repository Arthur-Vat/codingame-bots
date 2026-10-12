import { readFile } from 'node:fs/promises';
import { expect, test, type Page } from '@playwright/test';
import { arenaRow, serveArena, type ArenaRow } from './arena';
import { mockGames, record, recordView } from './fixtures';

// The arena games (ADR 0031) with the server's engine: the API is mocked with page.route, and the
// archive is served from the published site (github.io) by intercepting the requests. The same
// feature on the browser engine, with the real rules, is tested in browser-engine.spec.ts.

const ROWS: ArenaRow[] = [
  arenaRow({ id: 'uttt-1760000000-aaaaaaaaaaaa' }),
  arenaRow({
    id: 'uttt-1759990000-bbbbbbbbbbbb',
    unix_time: 1_759_990_000,
    players: ['uttt-v009', 'uttt-v001'],
    winner: 1,
    turns: 30,
    source: 'arena sprt',
  }),
  arenaRow({
    id: 'uttt-1759980000-cccccccccccc',
    unix_time: 1_759_980_000,
    players: ['uttt-v010', 'uttt-v001'],
    winner: null,
    turns: 81,
  }),
];

/** The record files of the fixture, by id. One has a seed beyond what JavaScript keeps exactly. */
function recordTexts(): Record<string, string> {
  const texts: Record<string, string> = {};
  ROWS.forEach((row, n) => {
    texts[row.id] = JSON.stringify(record({ seed: 100 + n, unix_time: row.unix_time }), null, 2);
  });
  const first = texts[ROWS[0]!.id]!;
  texts[ROWS[0]!.id] = first.replace('"bot_seed": 1,', '"bot_seed": 18446744073709551615,');
  return texts;
}

interface Api {
  /** The saved games ("my games"). */
  mine: ArenaRow[];
  /** The bodies POSTed to /api/view and /api/history, as text. */
  viewed: string[];
  saved: string[];
  /** The queries and bodies POSTed to /api/filter. */
  filtered: { query: string; rows: ArenaRow[] }[];
  /** When set, the next filter answer waits for it. */
  gate: Promise<void> | null;
}

/** The server's routes the pages of the history and of a review call. */
async function mockApi(page: Page): Promise<Api> {
  const api: Api = { mine: [], viewed: [], saved: [], filtered: [], gate: null };
  await mockGames(page);
  await page.route('**/api/filter**', async (route) => {
    const request = route.request();
    const query = new URL(request.url()).search;
    const rows = request.postDataJSON() as ArenaRow[];
    api.filtered.push({ query, rows });
    const gate = api.gate;
    api.gate = null;
    await gate;
    const params = new URLSearchParams(query);
    const release = params.get('release');
    const result = params.get('result');
    const fault = (row: ArenaRow) => ['timeout', 'crash', 'invalid'].includes(row.end);
    const kept = rows
      .filter((row) => release === null || row.players.includes(release))
      .filter((row) => result !== 'fault' || fault(row))
      .filter((row) => result !== 'x' || row.winner === 0)
      .sort((a, b) => b.unix_time - a.unix_time);
    await route.fulfill({ json: kept });
  });
  await page.route('**/api/view', async (route) => {
    api.viewed.push(route.request().postData() ?? '');
    await route.fulfill({ json: recordView() });
  });
  await page.route(
    (url) => url.pathname === '/api/history',
    async (route) => {
      const request = route.request();
      if (request.method() === 'POST') {
        const text = request.postData() ?? '';
        api.saved.push(text);
        const parsed = JSON.parse(text) as { seed: number; unix_time: number };
        const id = `uttt-${parsed.unix_time}-saved${parsed.seed}`;
        const duplicate = api.mine.some((row) => row.id === id);
        if (!duplicate)
          api.mine.push(arenaRow({ id, unix_time: parsed.unix_time, source: 'arena' }));
        await route.fulfill({ json: { id, duplicate } });
      } else {
        await route.fulfill({ json: api.mine });
      }
    },
  );
  return api;
}

const rows = (page: Page) => page.getByTestId('history-row');

/** Lets what the page does after an answer happen: a few frames. */
async function settled(page: Page): Promise<void> {
  await page.evaluate(
    'new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 50))))',
  );
}

test('the arena games are listed read-only, beside my games', async ({ page }) => {
  await mockApi(page);
  const arena = await serveArena(page.context(), { index: { format: 1, rows: ROWS } });
  await page.goto('/#/history/arena');

  await expect(rows(page)).toHaveCount(3);
  expect(arena.requests).toEqual([
    'https://arthur-vat.github.io/codingame-bots/arena-games/index.json',
  ]);
  // Newest first, with the result and the number of moves.
  await expect(rows(page).nth(0)).toContainText('1–0');
  await expect(rows(page).nth(0)).toContainText('41');
  await expect(rows(page).nth(1)).toContainText('0–1');
  await expect(rows(page).nth(2)).toContainText('½–½');

  // The switch: two links, the arena one is current; the top bar's History is too.
  const switcher = page.getByRole('navigation', { name: 'Games shown' });
  await expect(switcher.getByRole('link', { name: 'Arena games' })).toHaveAttribute(
    'aria-current',
    'page',
  );
  await expect(switcher.getByRole('link', { name: 'My games' })).not.toHaveAttribute(
    'aria-current',
    'page',
  );
  await expect(
    page.getByText(
      'Games played by the releases on GitHub (SPRT and league samples). Read-only: save one to add it to your games.',
    ),
  ).toBeVisible();

  // Read-only: no import, no delete, no source filter; the other filters stay.
  await expect(page.getByLabel('Load game files')).toHaveCount(0);
  await expect(page.getByLabel('Review a file')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Delete', exact: true })).toHaveCount(0);
  await expect(page.getByLabel('Source')).toHaveCount(0);
  await expect(page.getByLabel('Release')).toBeVisible();
  await expect(page.getByLabel('Result')).toBeVisible();
  await expect(rows(page).first().getByRole('link', { name: 'Review' })).toHaveAttribute(
    'href',
    '#/review/arena/uttt-1760000000-aaaaaaaaaaaa',
  );
  await expect(rows(page).first().getByRole('button', { name: 'Export' })).toBeVisible();
});

test('the switch goes between my games and the arena games', async ({ page }) => {
  const api = await mockApi(page);
  api.mine = [arenaRow({ id: 'uttt-1-mine', source: 'studio' })];
  const arena = await serveArena(page.context(), { index: { format: 1, rows: ROWS } });
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(1);
  await expect(page.getByLabel('Source')).toBeVisible();
  await expect(page.getByLabel('Load game files')).toBeVisible();
  expect(arena.requests).toEqual([]);

  await page.getByRole('link', { name: 'Arena games' }).click();
  await expect(page).toHaveURL(/#\/history\/arena$/);
  await expect(rows(page)).toHaveCount(3);
  await expect(page.getByLabel('Source')).toHaveCount(0);

  await page.getByRole('link', { name: 'My games' }).click();
  await expect(page).toHaveURL(/#\/history$/);
  await expect(rows(page)).toHaveCount(1);
  await expect(rows(page).first()).toContainText('studio');
  await expect(page.getByRole('link', { name: 'My games' })).toHaveAttribute(
    'aria-current',
    'page',
  );

  // The index is read once for the life of the page.
  await page.getByRole('link', { name: 'Arena games' }).click();
  await expect(rows(page)).toHaveCount(3);
  expect(arena.requests).toHaveLength(1);
});

test('a filter narrows the arena games with the history filter, and no record is fetched', async ({
  page,
}) => {
  const api = await mockApi(page);
  const arena = await serveArena(page.context(), {
    index: { format: 1, rows: ROWS },
    records: recordTexts(),
  });
  await page.goto('/#/history/arena');
  await expect(rows(page)).toHaveCount(3);
  expect(api.filtered.map((call) => call.query)).toEqual(['']);
  // The rows go to the filter as the index has them.
  expect(api.filtered[0]?.rows).toEqual(ROWS);

  await page.getByLabel('Release').selectOption('uttt-v009');
  await expect(rows(page)).toHaveCount(2);
  await expect.poll(() => api.filtered.at(-1)?.query).toBe('?release=uttt-v009');
  await expect(rows(page).nth(1)).toContainText('uttt-v001');

  await page.getByLabel('Release').selectOption('');
  await expect(rows(page)).toHaveCount(3);
  // The list only ever asked for the index.
  expect(new Set(arena.requests)).toEqual(
    new Set(['https://arthur-vat.github.io/codingame-bots/arena-games/index.json']),
  );
});

test('an answer to a filter that has been changed since is ignored', async ({ page }) => {
  const api = await mockApi(page);
  await serveArena(page.context(), { index: { format: 1, rows: ROWS } });
  await page.goto('/#/history/arena');
  await expect(rows(page)).toHaveCount(3);

  // The answer to the first query is held back until the second has been answered.
  let release: () => void = () => {};
  api.gate = new Promise<void>((resolve) => (release = resolve));
  await page.getByLabel('Release').selectOption('uttt-v009');
  await expect.poll(() => api.filtered.length).toBe(2);
  await page.getByLabel('Release').selectOption('uttt-v001');
  await expect(rows(page)).toHaveCount(2);
  const late = page.waitForResponse((response) => response.url().includes('release=uttt-v009'));
  release();
  await late;
  // The late answer (two other rows, for uttt-v009) does not replace the current one.
  await settled(page);
  await expect(rows(page)).toHaveCount(2);
  await expect(rows(page).nth(0)).toContainText('uttt-v001');
  await expect(rows(page).nth(1)).toContainText('uttt-v001');
});

test('a late list of one set does not show in the other', async ({ page }) => {
  const api = await mockApi(page);
  api.mine = [arenaRow({ id: 'uttt-1-mine', source: 'studio' })];
  const arena = await serveArena(page.context(), { index: { format: 1, rows: ROWS } });
  let release: () => void = () => {};
  arena.gate = new Promise<void>((resolve) => (release = resolve));
  await page.goto('/#/history/arena');
  await expect.poll(() => arena.requests.length).toBe(1);
  await page.getByRole('link', { name: 'My games' }).click();
  await expect(rows(page)).toHaveCount(1);
  release();
  await settled(page);
  await expect(rows(page)).toHaveCount(1);
  await expect(rows(page).first()).toContainText('studio');
});

test('Review opens the arena game, checked by the rules from the published text', async ({
  page,
}) => {
  const api = await mockApi(page);
  const texts = recordTexts();
  const arena = await serveArena(page.context(), {
    index: { format: 1, rows: ROWS },
    records: texts,
  });
  await page.goto('/#/history/arena');
  await rows(page).nth(0).getByRole('link', { name: 'Review' }).click();

  await expect(page).toHaveURL(/#\/review\/arena\/uttt-1760000000-aaaaaaaaaaaa$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByTestId('result')).toBeVisible();
  await expect(page.getByText('Arena game (arena match)')).toBeVisible();
  expect(arena.requests).toContain(
    'https://arthur-vat.github.io/codingame-bots/arena-games/records/uttt-1760000000-aaaaaaaaaaaa.json',
  );
  // The rules get the text as it was published, a 64-bit number included.
  expect(api.viewed).toEqual([texts['uttt-1760000000-aaaaaaaaaaaa']]);
  expect(api.viewed[0]).toContain('18446744073709551615');
  // History in the top bar leads back to the arena games.
  await page.getByRole('link', { name: 'History', exact: true }).click();
  await expect(page).toHaveURL(/#\/history\/arena$/);
  await expect(rows(page)).toHaveCount(3);
});

test('Save adds an arena game to my games, and says so when it is there already', async ({
  page,
}) => {
  const api = await mockApi(page);
  const texts = recordTexts();
  await serveArena(page.context(), { index: { format: 1, rows: ROWS }, records: texts });
  await page.goto('/#/review/arena/uttt-1760000000-aaaaaaaaaaaa');
  await expect(page.getByRole('button', { name: 'Save to history' })).toBeEnabled();

  await page.getByRole('button', { name: 'Save to history' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();
  // The record went as the text published.
  expect(api.saved).toEqual([texts['uttt-1760000000-aaaaaaaaaaaa']]);

  await page.getByRole('link', { name: 'History', exact: true }).click();
  await page.getByRole('link', { name: 'My games' }).click();
  await expect(rows(page)).toHaveCount(1);
  await expect(rows(page).first()).toContainText('arena');
  // Opened again, the same game is a duplicate.
  await page.goto('/#/review/arena/uttt-1760000000-aaaaaaaaaaaa');
  await page.getByRole('button', { name: 'Save to history' }).click();
  await expect(
    page.getByRole('status').filter({ hasText: 'Already in your history' }),
  ).toBeVisible();
  expect(api.mine).toHaveLength(1);
});

test('Export downloads the published record as <id>.json', async ({ page }) => {
  await mockApi(page);
  const texts = recordTexts();
  await serveArena(page.context(), { index: { format: 1, rows: ROWS }, records: texts });

  // From the list ...
  await page.goto('/#/history/arena');
  await expect(rows(page)).toHaveCount(3);
  const fromList = page.waitForEvent('download');
  await rows(page).nth(1).getByRole('button', { name: 'Export' }).click();
  const listed = await fromList;
  expect(listed.suggestedFilename()).toBe('uttt-1759990000-bbbbbbbbbbbb.json');
  expect(await readFile(await listed.path(), 'utf8')).toBe(texts['uttt-1759990000-bbbbbbbbbbbb']);

  // ... and from the review, byte for byte.
  await page.goto('/#/review/arena/uttt-1760000000-aaaaaaaaaaaa');
  await expect(page.getByRole('button', { name: 'Export file' })).toBeEnabled();
  const fromReview = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export file' }).click();
  const reviewed = await fromReview;
  expect(reviewed.suggestedFilename()).toBe('uttt-1760000000-aaaaaaaaaaaa.json');
  const file = await readFile(await reviewed.path(), 'utf8');
  expect(file).toBe(texts['uttt-1760000000-aaaaaaaaaaaa']);
  expect(file).toContain('18446744073709551615');
});

test('nothing published yet says so, for a missing index and for an empty one', async ({
  page,
}) => {
  await mockApi(page);
  const arena = await serveArena(page.context(), { index: null });
  await page.goto('/#/history/arena');
  await expect(page.getByTestId('history-empty')).toHaveText('No arena games published yet.');
  await expect(page.getByRole('alert')).toHaveCount(0);

  // An index with no row says the same.
  arena.index = { format: 1, rows: [] };
  await page.reload();
  await expect.poll(() => arena.requests.length).toBe(2);
  await expect(page.getByTestId('history-empty')).toHaveText('No arena games published yet.');
});

test('a filter that matches nothing in the arena games says so', async ({ page }) => {
  await mockApi(page);
  await serveArena(page.context(), { index: { format: 1, rows: ROWS } });
  await page.goto('/#/history/arena');
  await expect(rows(page)).toHaveCount(3);
  await page.getByLabel('Release').selectOption('uttt-v010');
  await page.getByLabel('Result').selectOption({ label: 'Faults' });
  await expect(page.getByTestId('history-empty')).toHaveText(
    'No saved game matches these filters.',
  );
});

test('an index that cannot be read says which URL, and a bad row is left out and counted', async ({
  page,
}) => {
  await mockApi(page);
  const arena = await serveArena(page.context(), { index: '{not json' });
  await page.goto('/#/history/arena');
  await expect(page.getByRole('alert')).toContainText(
    'The arena games at https://arthur-vat.github.io/codingame-bots/arena-games/index.json are not valid JSON',
  );

  arena.index = { format: 2, rows: [] };
  await page.reload();
  await expect(page.getByRole('alert')).toContainText('its format is not 1');

  arena.index = { format: 1, rows: [...ROWS, arenaRow({ id: '../../x' }), { id: 'uttt-1-x' }] };
  await page.reload();
  await expect(rows(page)).toHaveCount(3);
  await expect(page.getByText('2 entries of the published index are not valid')).toBeVisible();
});

test('the site being unreachable is an error naming the URL, not an empty list', async ({
  page,
}) => {
  await mockApi(page);
  await page.context().route('https://arthur-vat.github.io/**', (route) => route.abort());
  await page.goto('/#/history/arena');
  await expect(page.getByRole('alert')).toContainText(
    'Cannot load the arena games from https://arthur-vat.github.io/codingame-bots/arena-games/index.json',
  );
});

test('an arena game that is not published, or has a bad id, says why on the review page', async ({
  page,
}) => {
  await mockApi(page);
  await serveArena(page.context(), { index: { format: 1, rows: ROWS }, records: {} });
  await page.goto('/#/review/arena/uttt-1-missing');
  await expect(page.getByRole('alert')).toContainText(
    'The arena game uttt-1-missing is not published',
  );
  await page.goto('/#/review/arena/Not%20An%20Id');
  await expect(page.getByRole('alert')).toContainText('is not the id of an arena game');
});
