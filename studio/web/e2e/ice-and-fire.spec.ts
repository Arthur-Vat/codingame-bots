import { readFile } from 'node:fs/promises';
import { expect, test, type Page } from '@playwright/test';

/** The board shown: the hidden Play tab draws an empty board of its own. */
const view = (page: Page) => page.locator('.iaf').filter({ visible: true });

// A game of A Code of Ice and Fire on the analysis board, by the browser engine as GitHub Pages
// serves it (nothing mocked; it runs in the project "browser-engine"): the rules program knows the
// game, reads the record and gives the frames the board draws. The record is a real game of the
// arena, bronze (red) against random (blue), ended by the capture of blue's HQ after 21 answers:
// ice-and-fire-game.json, made by `ice-and-fire-arena match --pairs 1 --seed 2 --records DIR`.

async function isolated(page: Page): Promise<boolean> {
  try {
    return await page.evaluate<boolean>('crossOriginIsolated');
  } catch {
    // The page is reloading.
    return false;
  }
}

/** Opens `hash` and waits until the page is cross-origin isolated (after its one reload). */
async function open(page: Page, hash: string): Promise<void> {
  await page.goto(hash);
  await expect.poll(() => isolated(page), { timeout: 30_000 }).toBe(true);
}

/** Loads the record into the history and opens it on the analysis board. */
async function openRecord(page: Page): Promise<void> {
  const buffer = await readFile(new URL('./ice-and-fire-game.json', import.meta.url));
  await open(page, '/#/game/ice-and-fire/history');
  await page
    .getByLabel('Load game files')
    .setInputFiles({ name: 'ice-and-fire-game.json', mimeType: 'application/json', buffer });
  await expect(
    page.getByRole('status').filter({ hasText: 'Imported 1, 0 already there, 0 refused' }),
  ).toBeVisible();
  const rows = page.getByTestId('history-row');
  await expect(rows).toHaveCount(1);
  await expect(rows.first()).toContainText('bronze');
  await expect(rows.first()).toContainText('random');
  await rows.first().getByRole('link', { name: 'Analysis board' }).click();
  await expect(page).toHaveURL(
    /#\/game\/ice-and-fire\/analysis\/saved\/ice-and-fire-\d+-[0-9a-f]{12}$/,
  );
}

test('a record is loaded into the history and stepped through on the analysis board', async ({
  page,
}) => {
  await openRecord(page);
  const board = page.getByRole('group', { name: /^A Code of Ice and Fire board/ });
  const units = view(page).locator('.unit');
  const next = page.getByRole('button', { name: 'Next move' });

  // The start: red to move, nothing on the map but the two HQs, no last answer.
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(board).toHaveAttribute('aria-label', /turn 1 \/ 100, Red to move/);
  await expect(view(page).locator('.building')).toHaveCount(2);
  await expect(units).toHaveCount(0);
  await expect(view(page).locator('.iaf-answer')).toHaveText('');

  // Red trains two units: they appear with a ring, and the answer is shown whole.
  await next.click();
  await expect(units).toHaveCount(2);
  await expect(view(page).locator('.unit.r')).toHaveCount(2);
  await expect(view(page).locator('.trained')).toHaveCount(2);
  await expect(view(page).locator('.iaf-answer')).toHaveText('Red: TRAIN 1 1 0;TRAIN 1 2 0');
  await expect(board).toHaveAttribute('aria-label', /Blue to move/);

  // Blue trains one.
  await next.click();
  await expect(view(page).locator('.unit.b')).toHaveCount(1);
  await expect(view(page).locator('.iaf-answer')).toHaveText('Blue: TRAIN 2 11 10');

  // Red moves both: an arrow for each move, and the rings are gone.
  await next.click();
  await expect(view(page).locator('.move')).toHaveCount(2);
  await expect(view(page).locator('.trained')).toHaveCount(0);
  await expect(view(page).locator('.iaf-answer')).toHaveText('Red: MOVE 2 2 1;MOVE 1 1 1');

  // The end: the capture of blue's HQ, in the result and on the board.
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(page.getByTestId('result').filter({ visible: true })).toContainText(
    "Red captured Blue's HQ.",
  );
  await expect(view(page).locator('.iaf-turn .to-move')).toHaveText('Game over');
  await expect(page.getByTestId('result').filter({ visible: true })).toContainText(
    '1–0 · Red wins',
  );

  // The seats are Red and Blue, not X and O.
  await expect(page.getByTestId('player-0').filter({ visible: true })).toContainText('Red');
  await expect(page.getByTestId('player-1').filter({ visible: true })).toContainText('Blue');
});

test('the board fits a phone: no sideways scrolling at 360 px wide', async ({ page }) => {
  await page.setViewportSize({ width: 360, height: 740 });
  await openRecord(page);
  await page.getByRole('button', { name: 'Next move' }).click();
  await expect(view(page).locator('.unit')).toHaveCount(2);
  const box = await view(page).boundingBox();
  expect(box).not.toBeNull();
  expect(box!.x).toBeGreaterThanOrEqual(0);
  expect(box!.x + box!.width).toBeLessThanOrEqual(360);
  const wider = await page.evaluate<boolean>(
    'document.documentElement.scrollWidth > document.documentElement.clientWidth',
  );
  expect(wider).toBe(false);
});

test('the board is drawn with the default themes, and the choice is made in Board settings and remembered', async ({
  page,
}) => {
  await openRecord(page);
  const board = view(page);
  await expect(board).toHaveAttribute('data-pieces', 'lava');
  await expect(board).toHaveAttribute('data-board', 'scandinavian');
  // Every cell has a ground image, a file of the build.
  const tiles = board.locator('image.tile');
  await expect(tiles).toHaveCount(144);
  expect(
    await tiles.evaluateAll((images) =>
      images.every((image) => /\.svg$/.test(image.getAttribute('href') ?? '')),
    ),
  ).toBe(true);

  const settings = page.getByRole('region', { name: 'Board settings' }).filter({ visible: true });
  const pieces = settings.getByLabel('Pieces');
  const ground = settings.getByLabel('Board', { exact: true });
  await expect(pieces).toHaveValue('lava');
  await expect(ground).toHaveValue('scandinavian');
  await expect(pieces.locator('option')).toHaveText([
    'Lava and glacier',
    'Storybook',
    'Ink and watercolour',
    'Japanese print',
    'Bauhaus',
    'Scandinavian minimal',
    'Art deco',
  ]);

  await pieces.selectOption('storybook');
  await ground.selectOption('japanese');
  await expect(board).toHaveAttribute('data-pieces', 'storybook');
  await expect(board).toHaveAttribute('data-board', 'japanese');

  await page.reload();
  await expect.poll(() => isolated(page), { timeout: 30_000 }).toBe(true);
  await expect(view(page)).toHaveAttribute('data-pieces', 'storybook');
  await expect(view(page)).toHaveAttribute('data-board', 'japanese');
  await expect(
    page
      .getByRole('region', { name: 'Board settings' })
      .filter({ visible: true })
      .getByLabel('Pieces'),
  ).toHaveValue('storybook');
});

test('the board still fits 360 px wide with other themes chosen', async ({ page }) => {
  await page.setViewportSize({ width: 360, height: 740 });
  await openRecord(page);
  await page.evaluate(() => {
    localStorage.setItem('studio.theme.ice-and-fire.pieces', 'bauhaus');
    localStorage.setItem('studio.theme.ice-and-fire.board', 'deco');
  });
  await page.reload();
  await expect.poll(() => isolated(page), { timeout: 30_000 }).toBe(true);
  await expect(view(page)).toHaveAttribute('data-board', 'deco');
  const box = await view(page).boundingBox();
  expect(box).not.toBeNull();
  expect(box!.x).toBeGreaterThanOrEqual(0);
  expect(box!.x + box!.width).toBeLessThanOrEqual(360);
  const wider = await page.evaluate<boolean>(
    'document.documentElement.scrollWidth > document.documentElement.clientWidth',
  );
  expect(wider).toBe(false);
});

test('the setup menu of a game without human play offers only two bots', async ({ page }) => {
  await open(page, '/#/game/ice-and-fire');
  const dialog = page.getByRole('dialog', { name: 'Game setup' });
  await expect(dialog).toBeVisible({ timeout: 30_000 });
  const modes = dialog.getByRole('group', { name: 'Mode' }).getByRole('button');
  await expect(modes).toHaveCount(1);
  await expect(modes.first()).toHaveText('Bot vs bot');
  await expect(modes.first()).toHaveAttribute('aria-pressed', 'true');
});

test('the setup menu offers the releases, then the baseline bots, v005 against v004 first', async ({
  page,
}) => {
  await open(page, '/#/game/ice-and-fire');
  const dialog = page.getByRole('dialog', { name: 'Game setup' });
  await expect(dialog).toBeVisible({ timeout: 30_000 });
  await expect(dialog.getByText('This game has no bot yet')).toHaveCount(0);
  const red = dialog.getByLabel(/^Red \(moves first\)/);
  const blue = dialog.getByLabel('Blue', { exact: true });
  await expect(red).toHaveValue('ice-and-fire-v005');
  await expect(blue).toHaveValue('ice-and-fire-v004');
  // The releases first, newest first, then the baseline bots in a group, by their folder.
  await expect(red.locator(':scope > option')).toHaveCount(5);
  await expect(red.locator('optgroup')).toHaveAttribute('label', 'Baseline bots');
  await expect(red.locator('optgroup option')).toHaveText([
    'cutter',
    'guard',
    'chain',
    'bronze',
    'wood1',
    'wood2',
    'wood3',
    'random',
  ]);
  await expect(dialog.getByRole('button', { name: 'Start' })).toBeEnabled();
});

test('the release plays random to the end in the browser and wins', async ({ page }) => {
  test.setTimeout(240_000);
  await open(page, '/#/game/ice-and-fire');
  const dialog = page.getByRole('dialog', { name: 'Game setup' });
  await expect(dialog).toBeVisible({ timeout: 30_000 });
  await dialog.getByLabel('Blue', { exact: true }).selectOption('ice-and-fire-random');
  await dialog.getByLabel(/^Think time/).fill('0');
  await dialog.getByRole('button', { name: 'Start' }).click();

  await expect(page.getByTestId('playback')).toBeVisible({ timeout: 180_000 });
  await expect(page.getByTestId('result').filter({ visible: true })).toContainText('Red');
  await expect(page.getByTestId('player-0').filter({ visible: true })).toContainText(
    'ice-and-fire-v005',
  );
});

test('bronze plays random to the end in the browser, and the game is kept under their names', async ({
  page,
}) => {
  test.setTimeout(240_000);
  await open(page, '/#/game/ice-and-fire');
  const dialog = page.getByRole('dialog', { name: 'Game setup' });
  await expect(dialog).toBeVisible({ timeout: 30_000 });
  await dialog.getByLabel(/^Red \(moves first\)/).selectOption('ice-and-fire-bronze');
  await dialog.getByLabel('Blue', { exact: true }).selectOption('ice-and-fire-random');
  await dialog.getByLabel(/^Think time/).fill('0');
  await dialog.getByRole('button', { name: 'Start' }).click();

  // A game between two bots stays in the Play tab with its playback when it is over.
  await expect(page.getByTestId('playback')).toBeVisible({ timeout: 180_000 });
  await expect(page.getByTestId('result').filter({ visible: true })).toContainText('Red');
  await expect(page.getByTestId('player-0').filter({ visible: true })).toContainText(
    'ice-and-fire-bronze',
  );
  await expect(page.getByTestId('player-1').filter({ visible: true })).toContainText(
    'ice-and-fire-random',
  );
  // The playback opens on the first move; the last one shows the end.
  await expect(view(page).locator('.building')).toHaveCount(2);
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(view(page).locator('.iaf-turn .to-move')).toHaveText('Game over');

  // The record names the bots where a release would be, and the history lists it.
  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  await page.getByRole('link', { name: 'History', exact: true }).click();
  const rows = page.getByTestId('history-row');
  await expect(rows).toHaveCount(1);
  await expect(rows.first()).toContainText('ice-and-fire-bronze');
  await expect(rows.first()).toContainText('ice-and-fire-random');
  const filter = page.getByLabel('Release', { exact: true });
  await filter.selectOption('ice-and-fire-bronze');
  await expect(rows).toHaveCount(1);
  await filter.selectOption('ice-and-fire-wood1');
  await expect(page.getByTestId('history-empty')).toHaveText(
    'No saved game matches these filters.',
  );
});
