import { readFile } from 'node:fs/promises';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { arenaRow, serveArena } from './arena';
import { chooseMode, setupMenu, type SetupMode } from './fixtures';

// The browser engine as GitHub Pages serves it: static files from `npm run preview`, which has
// no /api and sets no headers. Nothing is mocked: the rules, the bots (the real releases, built
// to WebAssembly by `npm run wasm`) and the saved games (IndexedDB) are the real ones. The one
// exception is the block "the arena games (ADR 0031)": it serves a fixture archive of
// `arena-games/` by intercepting those requests (e2e/arena.ts), since the preview site has none.
// Playwright cannot intercept a request that a service worker handles, so that block runs with
// the service worker blocked (the page is then not isolated, which the arena games do not need);
// the rules, the saved games and the filters in it are still the real ones.

async function isolated(page: Page): Promise<boolean> {
  try {
    return await page.evaluate<boolean>('crossOriginIsolated');
  } catch {
    // The page is reloading.
    return false;
  }
}

/** Opens `hash` and waits until the page is cross-origin isolated (after its one reload). */
async function open(page: Page, hash = '/'): Promise<void> {
  await page.goto(hash);
  await expect.poll(() => isolated(page), { timeout: 30_000 }).toBe(true);
}

const moves = (page: Page): Locator => page.getByLabel('Moves', { exact: true }).locator('button');
const cells = (page: Page): Locator => page.getByRole('button', { name: /^play row/ });

/** Resigning asks twice: the button, then its confirmation. */
async function resign(page: Page) {
  await page.getByRole('button', { name: '⚑ Resign' }).click();
  await page.getByRole('button', { name: 'Confirm resign' }).click();
}

/** Starts a game from the setup menu, which the Play tab opens by itself when it shows no game. */
async function startGame(page: Page, mode: SetupMode, adjust?: (dialog: Locator) => Promise<void>) {
  await expect(setupMenu(page)).toBeVisible();
  const dialog = await chooseMode(setupMenu(page), mode);
  await adjust?.(dialog);
  await dialog.getByRole('button', { name: 'Start' }).click();
}

test('the page becomes cross-origin isolated after one reload, and says which engine it uses', async ({
  page,
}) => {
  const loads: string[] = [];
  page.on('load', () => loads.push(page.url()));
  await open(page);
  await expect(page.getByRole('heading', { name: 'Choose a game' })).toBeVisible();
  await expect(page.locator('header.top')).toHaveAttribute('title', 'Running in your browser');
  await expect(page.getByText('in your browser', { exact: true })).toBeVisible();

  expect(await page.evaluate(() => typeof SharedArrayBuffer)).toBe('function');
  expect(await page.evaluate<boolean>('navigator.serviceWorker.controller !== null')).toBe(true);
  // One reload, not a loop: the guard is forgotten once the page is isolated.
  expect(loads).toHaveLength(2);
  expect(await page.evaluate(() => sessionStorage.getItem('studio.coi-reload'))).toBeNull();

  // The games and their releases come from the WebAssembly files, newest first.
  const card = page.getByRole('link', { name: /Ultimate Tic-Tac-Toe/ });
  await expect(card).toBeVisible();
  await expect(card).toContainText('10 releases');
});

test('a friend game is played in the browser, by the Rust rules', async ({ page }) => {
  await open(page, '/#/game/uttt');
  await startGame(page, 'Human vs human');
  await expect(page.getByRole('button', { name: 'play row 4 column 4' })).toBeVisible();
  await page.getByRole('button', { name: 'play row 4 column 4' }).click();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  // The rules send the second player to the small board in the middle.
  await expect(cells(page)).toHaveCount(8);
  await page.getByRole('button', { name: 'play row 4 column 5' }).click();
  await expect(page.getByRole('img', { name: 'row 4 column 5 O' })).toBeVisible();
  await expect(moves(page)).toHaveCount(2);

  await page.getByRole('button', { name: '↶ Takeback' }).click();
  await expect(moves(page)).toHaveCount(1);
  await expect(cells(page)).toHaveCount(8);

  await resign(page);
  await expect(page.getByTestId('result')).toBeVisible();
  await expect(cells(page)).toHaveCount(0);
});

test('against uttt-v001 in fixed mode: two moves, a takeback that replays the bot, and play goes on', async ({
  page,
}) => {
  test.setTimeout(180_000);
  await open(page, '/#/game/uttt');
  await startGame(page, 'Human vs bot', async (dialog) => {
    await dialog.getByLabel('Release', { exact: true }).selectOption('uttt-v001');
    // The smallest think time of the slider.
    await dialog.getByLabel(/^Think time/).fill('0');
  });
  // The first game against a release measures its speed in this browser.
  await expect(page.getByRole('button', { name: 'play row 4 column 4' })).toBeVisible({
    timeout: 120_000,
  });
  await page.getByRole('button', { name: 'play row 4 column 4' }).click();
  await expect(moves(page)).toHaveCount(2, { timeout: 60_000 });
  await expect(cells(page).first()).toBeVisible({ timeout: 60_000 });
  await cells(page).first().click();
  await expect(moves(page)).toHaveCount(4, { timeout: 60_000 });
  await expect(cells(page).first()).toBeVisible({ timeout: 60_000 });
  const bots = await moves(page).allTextContents();

  // The takeback restarts the bot and replays its answers: the same ones, or the game fails.
  await page.getByRole('button', { name: '↶ Takeback' }).click();
  await expect(moves(page)).toHaveCount(2, { timeout: 60_000 });
  await expect(cells(page).first()).toBeVisible({ timeout: 60_000 });
  expect(await moves(page).allTextContents()).toEqual(bots.slice(0, 2));
  await expect(page.getByRole('alert')).toHaveCount(0);

  await cells(page).first().click();
  await expect(moves(page)).toHaveCount(4, { timeout: 60_000 });
  // The speed is kept in this browser, under the release and the hash of its file.
  const speeds = await page.evaluate(() => localStorage.getItem('studio.speed.v1'));
  expect(Object.keys(JSON.parse(speeds ?? '{}') as object)).toEqual([
    expect.stringMatching(/^uttt-v001:[0-9a-f]{64}$/),
  ]);
});

test('bot against bot in real time to the end, its playback, and the history kept in the browser', async ({
  page,
}) => {
  test.setTimeout(240_000);
  await open(page, '/#/game/uttt');
  await startGame(page, 'Bot vs bot', async (dialog) => {
    await dialog.getByLabel(/^X \(moves first\)/).selectOption('uttt-v001');
    await dialog.getByLabel('O', { exact: true }).selectOption('uttt-v002');
    await dialog.getByLabel(/^Think time/).fill('0');
  });
  // A game between two bots stays in the Play tab with its playback when it is over.
  await expect(page.getByTestId('playback')).toBeVisible({ timeout: 180_000 });
  await expect(page).toHaveURL(/#\/game\/uttt\/[0-9a-f]+$/);
  await expect(page.getByTestId('result')).toBeVisible();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByTestId('player-0')).toContainText('uttt-v001');
  await expect(page.getByTestId('player-1')).toContainText('uttt-v002');

  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();

  // The saved game is listed, and is still there after the page is loaded again.
  await page.getByRole('link', { name: 'History', exact: true }).click();
  const rows = page.getByTestId('history-row');
  await expect(rows).toHaveCount(1);
  await page.reload();
  await expect.poll(() => isolated(page)).toBe(true);
  await expect(rows).toHaveCount(1);
  await expect(rows.first()).toContainText('uttt-v001');
  await expect(rows.first()).toContainText('uttt-v002');
  await expect(rows.first()).toContainText('studio');

  // The filters are the rules' own.
  await page.getByLabel('Release', { exact: true }).selectOption('uttt-v002');
  await expect(rows).toHaveCount(1);
  await page.getByLabel('Release', { exact: true }).selectOption('uttt-v003');
  await expect(page.getByTestId('history-empty')).toHaveText(
    'No saved game matches these filters.',
  );
  await page.getByLabel('Release', { exact: true }).selectOption('');
  await expect(rows).toHaveCount(1);

  // Analysis board opens it from the browser's storage.
  await rows.first().getByRole('link', { name: 'Analysis board' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/analysis\/saved\/uttt-\d+-[0-9a-f]{12}$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await page.getByRole('link', { name: 'History', exact: true }).click();

  // Export gives the record file.
  const exported = page.waitForEvent('download');
  await rows.first().getByRole('button', { name: 'Export' }).click();
  const download = await exported;
  expect(download.suggestedFilename()).toMatch(/^uttt-\d+-[0-9a-f]{12}\.json$/);
  const path = await download.path();
  const file = await readFile(path, 'utf8');
  const record = JSON.parse(file) as {
    format: number;
    game: string;
    source: string;
    players: { name: string }[];
  };
  expect(record).toMatchObject({ format: 1, game: 'uttt', source: 'studio' });
  expect(record.players.map((player) => player.name)).toEqual(['uttt-v001', 'uttt-v002']);

  // Delete asks twice; the game is gone, also after a reload.
  await rows.first().getByRole('button', { name: 'Delete', exact: true }).click();
  await rows.first().getByRole('button', { name: 'Delete for good' }).click();
  await expect(page.getByTestId('history-empty')).toContainText('No saved games yet');
  await page.reload();
  await expect.poll(() => isolated(page)).toBe(true);
  await expect(page.getByTestId('history-empty')).toContainText('No saved games yet');

  // A record file is loaded into the history (and read twice, not kept twice) ...
  const loadable = { name: 'game.json', mimeType: 'application/json', buffer: Buffer.from(file) };
  await page.getByLabel('Load game files').setInputFiles(loadable);
  await expect(
    page.getByRole('status').filter({ hasText: 'Imported 1, 0 already there, 0 refused' }),
  ).toBeVisible();
  await expect(rows).toHaveCount(1);
  await page.getByLabel('Load game files').setInputFiles(loadable);
  await expect(
    page.getByRole('status').filter({ hasText: 'Imported 0, 1 already there, 0 refused' }),
  ).toBeVisible();
  await expect(rows).toHaveCount(1);

  // ... one that is not a record is refused with the rules' reason ...
  await page.getByLabel('Load game files').setInputFiles({
    name: 'nonsense.json',
    mimeType: 'application/json',
    buffer: Buffer.from('{"format": 1}'),
  });
  await expect(page.getByRole('alert', { name: 'Refused files' })).toContainText('nonsense.json');
  await expect(rows).toHaveCount(1);

  // ... and a file can be opened on the analysis board without being saved.
  await page.getByLabel('Open a file').setInputFiles(loadable);
  await expect(page).toHaveURL(/#\/game\/uttt\/analysis\/file\/f[0-9a-z]+$/);
  await expect(page.getByText('game.json')).toBeVisible();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
});

test.describe('where the page cannot be isolated', () => {
  // No service worker, so no isolation: games between people, the history and the analysis board work, and
  // what needs a bot says why it cannot.
  test.use({ serviceWorkers: 'block' });

  test('a friend game and the history work, and a bot is refused with a clear message', async ({
    page,
  }) => {
    await page.goto('/#/game/uttt');
    // The page waits a few seconds for a service worker that never comes.
    await expect(setupMenu(page)).toBeVisible({
      timeout: 30_000,
    });
    expect(await isolated(page)).toBe(false);

    await startGame(page, 'Human vs human');
    await page.getByRole('button', { name: 'play row 4 column 4' }).click();
    await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
    await resign(page);
    await expect(page.getByTestId('result')).toBeVisible();
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    await expect(
      page.getByRole('status').filter({ hasText: 'Saved to your history' }),
    ).toBeVisible();
    await page.getByRole('link', { name: 'History', exact: true }).click();
    await expect(page.getByTestId('history-row')).toHaveCount(1);

    await page.goto('/#/game/uttt');
    await startGame(page, 'Human vs bot');
    await expect(page.getByRole('alert')).toContainText(
      'This browser does not let bots run on this page (cross-origin isolation is unavailable)',
    );
  });
});

test.describe('the arena games (ADR 0031)', () => {
  // Playwright's interception does not see the requests that a service worker handles, so these
  // tests run without ours: on a full load the page waits ten seconds for it, and the history and
  // the analysis board works all the same.
  test.use({ serviceWorkers: 'block' });

  test('the arena games are read from the site itself, filtered by the rules, analysed, saved and exported', async ({
    page,
  }) => {
    test.setTimeout(120_000);
    // A real record to publish: a game between friends, played by the Rust rules, saved and exported
    // from the history, then deleted from it.
    await page.goto('/#/game/uttt');
    await expect(setupMenu(page)).toBeVisible({
      timeout: 30_000,
    });
    await startGame(page, 'Human vs human');
    await page.getByRole('button', { name: 'play row 4 column 4' }).click();
    await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
    await resign(page);
    await expect(page.getByTestId('result')).toBeVisible();
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    await expect(
      page.getByRole('status').filter({ hasText: 'Saved to your history' }),
    ).toBeVisible();
    await page.getByRole('link', { name: 'History', exact: true }).click();
    const mine = page.getByTestId('history-row');
    await expect(mine).toHaveCount(1);
    const exported = page.waitForEvent('download');
    await mine.first().getByRole('button', { name: 'Export' }).click();
    const download = await exported;
    const id = download.suggestedFilename().replace(/\.json$/, '');
    expect(id).toMatch(/^uttt-\d+-[0-9a-f]{12}$/);
    const text = await readFile(await download.path(), 'utf8');
    const parsed = JSON.parse(text) as {
      unix_time: number;
      source: string;
      winner: number | null;
      end: { kind: string };
      turns: unknown[];
      players: { name: string }[];
    };
    await mine.first().getByRole('button', { name: 'Delete', exact: true }).click();
    await mine.first().getByRole('button', { name: 'Delete for good' }).click();
    await expect(page.getByTestId('history-empty')).toContainText('No saved games yet');

    // The archive: that game, and an older one with the other winner (no record is needed to list).
    const real = arenaRow({
      id,
      unix_time: parsed.unix_time,
      players: [parsed.players[0]?.name ?? '', parsed.players[1]?.name ?? ''],
      winner: parsed.winner,
      end: parsed.end.kind,
      turns: parsed.turns.length,
      source: parsed.source,
    });
    const decoy = arenaRow({
      id: 'uttt-1-decoy',
      unix_time: 1,
      winner: parsed.winner === 0 ? 1 : 0,
      end: 'finished',
    });
    const arena = await serveArena(page.context(), {
      index: { format: 1, rows: [real, decoy] },
      records: { [id]: text },
    });
    await page.getByRole('link', { name: 'Arena games' }).click();
    await expect(page).toHaveURL(/#\/game\/uttt\/history\/arena$/);
    const listed = page.getByTestId('history-row');
    await expect(listed).toHaveCount(2);
    // Relative to the page: the site's own folder, nothing from github.io.
    expect(arena.requests).toEqual(['http://127.0.0.1:4173/arena-games/index.json']);
    await expect(page.getByLabel('Source')).toHaveCount(0);
    await expect(page.getByLabel('Load game files')).toHaveCount(0);

    // The filters are the rules' own: the winner narrows the list to one game, or to none.
    const winnerFilter = parsed.winner === 0 ? 'X wins' : 'O wins';
    const otherFilter = parsed.winner === 0 ? 'O wins' : 'X wins';
    await page.getByLabel('Result').selectOption({ label: winnerFilter });
    await expect(listed).toHaveCount(1);
    await expect(listed.first().getByRole('link', { name: 'Analysis board' })).toHaveAttribute(
      'href',
      `#/game/uttt/analysis/arena/${id}`,
    );
    await page.getByLabel('Result').selectOption({ label: 'Draw' });
    await expect(page.getByTestId('history-empty')).toHaveText(
      'No saved game matches these filters.',
    );
    await page.getByLabel('Result').selectOption({ label: otherFilter });
    await expect(listed).toHaveCount(1);
    await expect(listed.first().getByRole('link', { name: 'Analysis board' })).toHaveAttribute(
      'href',
      '#/game/uttt/analysis/arena/uttt-1-decoy',
    );
    await page.getByLabel('Result').selectOption('');
    await expect(listed).toHaveCount(2);

    // Export gives the published file, byte for byte.
    const fromList = page.waitForEvent('download');
    await listed.first().getByRole('button', { name: 'Export' }).click();
    const file = await fromList;
    expect(file.suggestedFilename()).toBe(`${id}.json`);
    expect(await readFile(await file.path(), 'utf8')).toBe(text);

    // Analysis board replays the record with the rules, from the published text.
    await listed.first().getByRole('link', { name: 'Analysis board' }).click();
    await expect(page).toHaveURL(new RegExp(`#/game/uttt/analysis/arena/${id}$`));
    await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
    await expect(page.getByTestId('result').filter({ visible: true })).toBeVisible();
    expect(arena.requests.at(-1)).toBe(`http://127.0.0.1:4173/arena-games/records/${id}.json`);

    // Save copies it to my games, as a game of the studio like any other.
    await page.getByRole('button', { name: 'Save to history' }).click();
    await expect(
      page.getByRole('status').filter({ hasText: 'Saved to your history' }),
    ).toBeVisible();
    await page.getByRole('link', { name: 'History', exact: true }).click();
    await page.getByRole('link', { name: 'My games' }).click();
    await expect(mine).toHaveCount(1);
    await expect(mine.first()).toContainText('studio');
    await page.getByLabel('Source').selectOption({ label: 'Played in the studio' });
    await expect(mine).toHaveCount(1);
    // Saved again, it is the same game.
    await page.goto(`/#/game/uttt/analysis/arena/${id}`);
    await page.getByRole('button', { name: 'Save to history' }).click();
    await expect(
      page.getByRole('status').filter({ hasText: 'Already in your history' }),
    ).toBeVisible();

    // A site without arena games (a 404, as GitHub Pages answers) says so.
    arena.index = null;
    await page.goto('/#/game/uttt/history/arena');
    await page.reload();
    await expect(page.getByTestId('history-empty')).toHaveText('No arena games published yet.', {
      timeout: 30_000,
    });
    await expect(page.getByRole('alert')).toHaveCount(0);
  });
});

test("without arena-games files, the static host's fallback page counts as nothing published", async ({
  page,
}) => {
  // `npm run preview` answers an unknown path with index.html and a 200, as a single-page app
  // host may; the front end must not call that an unreadable index. Here the service worker
  // handles the request, as on the site.
  await open(page, '/#/game/uttt/history/arena');
  await expect(page.getByTestId('history-empty')).toHaveText('No arena games published yet.');
  await expect(page.getByRole('alert')).toHaveCount(0);
});
