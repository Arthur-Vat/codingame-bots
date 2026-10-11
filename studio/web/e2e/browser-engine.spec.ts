import { readFile } from 'node:fs/promises';
import { expect, test, type Locator, type Page } from '@playwright/test';

// The browser engine as GitHub Pages serves it: static files from `npm run preview`, which has
// no /api and sets no headers. Nothing is mocked: the rules, the bots (the real releases, built
// to WebAssembly by `npm run wasm`) and the saved games (IndexedDB) are the real ones.

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

async function startGame(page: Page, mode: string, adjust?: (dialog: Locator) => Promise<void>) {
  await page.getByRole('button', { name: new RegExp(mode) }).click();
  const dialog = page.getByRole('dialog', { name: mode });
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
  await startGame(page, 'Play with a friend');
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

  await page.getByRole('button', { name: '⚑ Resign' }).click();
  await expect(page.getByTestId('result')).toBeVisible();
  await expect(cells(page)).toHaveCount(0);
});

test('against uttt-v001 in fixed mode: two moves, a takeback that replays the bot, and play goes on', async ({
  page,
}) => {
  test.setTimeout(180_000);
  await open(page, '/#/game/uttt');
  await startGame(page, 'Play the computer', async (dialog) => {
    await dialog.getByLabel('Release').selectOption('uttt-v001');
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

test('bot against bot in real time to the end, its review, and the history kept in the browser', async ({
  page,
}) => {
  test.setTimeout(240_000);
  await open(page, '/#/game/uttt');
  await startGame(page, 'Bot against bot', async (dialog) => {
    await dialog.getByLabel(/^X \(moves first\)/).selectOption('uttt-v001');
    await dialog.getByLabel('O', { exact: true }).selectOption('uttt-v002');
    await dialog.getByLabel(/^X think/).fill('0');
    await dialog.getByLabel(/^O think/).fill('0');
  });
  // A game between two bots opens its review by itself when it is over.
  await expect(page).toHaveURL(/#\/review\/session\//, { timeout: 180_000 });
  await expect(page.getByTestId('result')).toBeVisible();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByTestId('player-0')).toContainText('uttt-v001');
  await expect(page.getByTestId('player-1')).toContainText('uttt-v002');

  await page.getByRole('button', { name: 'Save to history' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();

  // The saved game is listed, and is still there after the page is loaded again.
  await page.getByRole('link', { name: 'History' }).click();
  const rows = page.getByTestId('history-row');
  await expect(rows).toHaveCount(1);
  await page.reload();
  await expect.poll(() => isolated(page)).toBe(true);
  await expect(rows).toHaveCount(1);
  await expect(rows.first()).toContainText('uttt-v001');
  await expect(rows.first()).toContainText('uttt-v002');
  await expect(rows.first()).toContainText('studio');

  // The filters are the rules' own.
  await page.getByLabel('Release').selectOption('uttt-v002');
  await expect(rows).toHaveCount(1);
  await page.getByLabel('Release').selectOption('uttt-v003');
  await expect(page.getByTestId('history-empty')).toHaveText(
    'No saved game matches these filters.',
  );
  await page.getByLabel('Release').selectOption('');
  await expect(rows).toHaveCount(1);

  // Review opens it from the browser's storage.
  await rows.first().getByRole('link', { name: 'Review' }).click();
  await expect(page).toHaveURL(/#\/review\/saved\/uttt-\d+-[0-9a-f]{12}$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await page.getByRole('link', { name: 'History' }).click();

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

  // ... and a file can be reviewed without being saved.
  await page.getByLabel('Review a file').setInputFiles(loadable);
  await expect(page).toHaveURL(/#\/review\/file\/f[0-9a-z]+$/);
  await expect(page.getByText('game.json')).toBeVisible();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
});

test.describe('where the page cannot be isolated', () => {
  // No service worker, so no isolation: games between people, the history and reviews work, and
  // what needs a bot says why it cannot.
  test.use({ serviceWorkers: 'block' });

  test('a friend game and the history work, and a bot is refused with a clear message', async ({
    page,
  }) => {
    await page.goto('/#/game/uttt');
    // The page waits a few seconds for a service worker that never comes.
    await expect(page.getByRole('button', { name: /Play with a friend/ })).toBeVisible({
      timeout: 30_000,
    });
    expect(await isolated(page)).toBe(false);

    await startGame(page, 'Play with a friend');
    await page.getByRole('button', { name: 'play row 4 column 4' }).click();
    await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
    await page.getByRole('button', { name: '⚑ Resign' }).click();
    await expect(page.getByTestId('result')).toBeVisible();
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    await expect(
      page.getByRole('status').filter({ hasText: 'Saved to your history' }),
    ).toBeVisible();
    await page.getByRole('link', { name: 'History' }).click();
    await expect(page.getByTestId('history-row')).toHaveCount(1);

    await page.goto('/#/game/uttt');
    await startGame(page, 'Play the computer');
    await expect(page.getByRole('alert')).toContainText(
      'This browser does not let bots run on this page (cross-origin isolation is unavailable)',
    );
  });
});
