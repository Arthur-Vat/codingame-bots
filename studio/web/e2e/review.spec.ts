import { expect, test, type Locator, type Page } from '@playwright/test';
import {
  answer,
  frame,
  gameFrames,
  gameTurns,
  mockGames,
  pageHeights,
  record,
  recordView,
} from './fixtures';

// The server is not run by this job: the API is mocked with page.route.

const SAVED = 'uttt-1760000000-abcdef123456';

async function mockSaved(page: Page, view: unknown = recordView()) {
  await mockGames(page);
  await page.route(`**/api/history/${SAVED}/view`, (route) => route.fulfill({ json: view }));
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
}

/** The part of a locator that is on screen: the Play tab stays in the page, hidden, behind the others. */
const shown = (locator: Locator) => locator.filter({ visible: true });

async function slotBox(page: Page) {
  const box = await shown(page.getByTestId('board-slot')).boundingBox();
  expect(box).not.toBeNull();
  return box;
}

const cell = (page: Page, row: number, col: number, mark: 'X' | 'O') =>
  page.getByRole('img', { name: `row ${row} column ${col} ${mark}` });

function sessionOver(patch: Record<string, unknown> = {}) {
  return {
    id: 'ab12',
    game: 'uttt',
    seed: 5,
    opening_plies: 0,
    seats: [
      {
        kind: 'bot',
        name: 'uttt-v010',
        release: 'uttt-v010',
        think_ms: 100,
        mode: 'realtime',
        fixed_iters: null,
      },
      {
        kind: 'bot',
        name: 'uttt-v009',
        release: 'uttt-v009',
        think_ms: 100,
        mode: 'realtime',
        fixed_iters: null,
      },
    ],
    status: 'over',
    error: null,
    to_act: [],
    opening_turns: 0,
    turns: gameTurns(),
    frames: gameFrames(),
    human_moves: [],
    result: { winner: 1, end: { kind: 'finished' } },
    progress: null,
    ...patch,
  };
}

test('a saved game starts at the first frame and steps with the buttons and the keys', async ({
  page,
}) => {
  await mockSaved(page);
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);

  // Info card and players.
  await expect(page.getByText('Saved (arena match)')).toBeVisible();
  await expect(page.getByText('uttt-v009, time × 2').first()).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('result')).toContainText('Small boards won: 1–2');

  // The first frame: an empty board, nothing in the list is current.
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Previous move' })).toBeDisabled();
  const start = await slotBox(page);

  await page.getByRole('button', { name: 'Next move' }).click();
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  await expect(page.getByRole('button', { name: '4 4' })).toHaveAttribute('aria-current', 'true');

  await page.keyboard.press('ArrowRight');
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: '4 5' })).toHaveAttribute('aria-current', 'true');
  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);

  await page.keyboard.press('End');
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();
  await page.keyboard.press('Home');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);

  await page.getByRole('button', { name: '5 3' }).click();
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await page.getByRole('button', { name: 'First move' }).click();
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);

  expect(await slotBox(page)).toEqual(start);
  await expect(page.getByTestId('analysis-slot')).toBeAttached();
});

test("the arrows step from anywhere, and Space is the browser's on a button, a switch or a link", async ({
  page,
}) => {
  await mockSaved(page);
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await page.getByRole('button', { name: 'Next move' }).click();
  await expect(cell(page, 4, 4, 'X')).toBeVisible();

  // The switch is not text entry; the arrows still step, and Space toggles the switch only.
  const coordinates = shown(page.getByLabel('Show coordinates'));
  await coordinates.focus();
  await page.keyboard.press('ArrowRight');
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
  await page.keyboard.press('Space');
  await expect(coordinates).toBeChecked();
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();

  // Space on a focused button presses that button: here it steps once, and does not play.
  await page.getByRole('button', { name: 'Next move' }).focus();
  await page.keyboard.press('Space');
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();

  // Space on a link plays nothing either.
  await page.getByRole('link', { name: 'History' }).focus();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();

  // With the focus on the page, Space plays and pauses.
  await page.mouse.click(30, 700);
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();

  // The Play button itself, focused after a click, is pressed once by Space.
  await page.getByRole('button', { name: 'Play' }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();
});

test('jumping to the last frame while playing stops the playback', async ({ page }) => {
  await page.clock.install();
  await mockSaved(page);
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await page.getByRole('button', { name: 'Play' }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();
});

test('playback at 4× reaches the end and stops; a speed change keeps playing', async ({ page }) => {
  await page.clock.install();
  await mockSaved(page);
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  const start = await slotBox(page);

  // 5 frames: at 1× a step takes 120 s / 4 = 30 s, at 4× 7.5 s.
  await expect(page.getByRole('button', { name: '1×' })).toHaveAttribute('aria-pressed', 'true');
  await page.getByRole('button', { name: '4×' }).click();
  await expect(page.getByRole('button', { name: '4×' })).toHaveAttribute('aria-pressed', 'true');
  await page.getByRole('button', { name: 'Play' }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

  await page.clock.runFor(7500);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);

  // A new speed while playing: still playing, with the new interval (30 s at 1×).
  await page.getByRole('button', { name: '1×' }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.clock.runFor(29_000);
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);
  await page.clock.runFor(1_500);
  await expect(cell(page, 4, 5, 'O')).toBeVisible();

  await page.getByRole('button', { name: '4×' }).click();
  await page.clock.runFor(7500 * 2);
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  // At the end it stops by itself.
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();
  await page.clock.runFor(60_000);
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();
  expect(await slotBox(page)).toEqual(start);

  // Play at the end starts again from the first frame.
  await page.getByRole('button', { name: 'Play' }).click();
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
});

test('a game whose last answer was invalid shows the turns that replay', async ({ page }) => {
  await mockSaved(
    page,
    recordView(
      {
        turns: [...gameTurns(3), answer(1, '0 0')],
        end: { kind: 'invalid', seat: 1, reason: 'cell taken' },
        winner: 0,
      },
      3,
    ),
  );
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  const moves = shown(page.getByLabel('Moves', { exact: true }));
  await expect(moves).toContainText('5 3');
  await expect(moves).not.toContainText('0 0');
  await expect(page.getByTestId('result')).toContainText('1–0 · X wins');
  await expect(page.getByTestId('result')).toContainText('Invalid answer, not shown');
  await page.keyboard.press('End');
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
});

test('a finished session is its own review, and is saved and exported there', async ({ page }) => {
  await mockGames(page);
  const posts: string[] = [];

  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  await page.route('**/api/sessions/ab12/save', (route) => {
    posts.push(route.request().method());
    return route.fulfill({ json: { id: SAVED, duplicate: false } });
  });
  await page.route('**/api/sessions/ab12/record', (route) => route.fulfill({ json: record() }));
  await page.goto('/#/game/uttt/ab12');

  await expect(page.getByText('Played here')).toBeVisible();
  await expect(page.getByText('uttt-v010, ≈ 100 ms').first()).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('playback')).toBeVisible();
  const start = await slotBox(page);

  await page.getByRole('button', { name: 'Save' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  expect(posts).toEqual(['POST']);
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();
  expect(await slotBox(page)).toEqual(start);

  const exported = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export file' }).click();
  expect((await exported).suggestedFilename()).toMatch(/^uttt-\d{4}-\d{2}-\d{2}-5\.json$/);
});

test('saving a game that is in the history already says so', async ({ page }) => {
  await mockGames(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  await page.route('**/api/sessions/ab12/save', (route) =>
    route.fulfill({ json: { id: SAVED, duplicate: true } }),
  );
  await page.goto('/#/game/uttt/ab12');
  await page.getByRole('button', { name: 'Save' }).click();
  await expect(
    page.getByRole('status').filter({ hasText: 'Already in your history' }),
  ).toBeVisible();
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();
});

test('a saved game is already in the history and exports its record', async ({ page }) => {
  await mockSaved(page);
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();
  const exported = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export file' }).click();
  expect((await exported).suggestedFilename()).toBe(`${SAVED}.json`);
});

test('a game that cannot be shown says why', async ({ page }) => {
  await mockGames(page);
  await page.route('**/api/history/nope/view', (route) =>
    route.fulfill({ status: 404, json: { error: 'no saved game "nope"' } }),
  );
  await page.goto('/#/game/uttt/review/saved/nope');
  await expect(page.getByRole('alert')).toHaveText('no saved game "nope"');

  await page.goto('/#/game/uttt/review/file/f99');
  await expect(page.getByRole('alert')).toContainText('no longer loaded');
});

const HUMANS = [
  { kind: 'human', name: 'Player 1' },
  { kind: 'human', name: 'Player 2' },
];

test('two bots that finish while watched stay on the page and become its review', async ({
  page,
}) => {
  await mockGames(page);
  let current: Record<string, unknown> = sessionOver({
    status: 'bot_thinking',
    to_act: [1],
    result: null,
    turns: gameTurns(1),
    frames: gameFrames(1),
  });
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: current }));
  await page.goto('/#/game/uttt/ab12');
  // The move number is in the info card, not over the board.
  await expect(page.locator('.info dt:text-is("Move") + dd')).toHaveText('2');
  await expect(page.getByTestId('board-overlay')).toHaveCount(0);
  await expect(page.getByTestId('playback')).toHaveCount(0);

  current = sessionOver();
  // The address stays; the playback bar appears, on the first frame (a game between two bots is
  // not shown while it is played).
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Review', exact: true })).toHaveCount(0);
  // Nothing moves the address later on either.
  await page.keyboard.press('Home');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
});

test('a finished game is its review in place: no Review button, the keys and the bar work', async ({
  page,
}) => {
  await mockGames(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('end-block')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Rematch' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'New game' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Review', exact: true })).toHaveCount(0);
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page.getByTestId('playback-slot')).toHaveCount(0);
  await expect(page.getByTestId('analysis-slot')).toBeAttached();
  await expect(page.getByText('Keys: ← → step, Home/End jump, Space plays.')).toBeVisible();
  // It opens on the first frame, paused (a game between two bots is not shown while it is played).
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Pause' })).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();

  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(page.getByRole('button', { name: '5 3' })).toHaveAttribute('aria-current', 'true');
  await page.getByRole('button', { name: '4 5' }).click();
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
  await expect(cell(page, 5, 3, 'X')).toHaveCount(0);

  // Space plays from here, and at the end plays again from the first frame.
  await page.mouse.click(30, 700);
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Play', exact: true })).toBeVisible();
  await page.keyboard.press('End');
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
});

test('a finished game opens on the frame that was pinned when it ended', async ({ page }) => {
  await mockGames(page);
  let current: Record<string, unknown> = sessionOver({
    seats: HUMANS,
    status: 'waiting_human',
    to_act: [0],
    result: null,
    turns: gameTurns(3),
    frames: gameFrames(3),
  });
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: current }));
  await page.goto('/#/game/uttt/ab12');
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 5, 3, 'X')).toHaveCount(0);

  current = sessionOver({ seats: HUMANS });
  await expect(page.getByTestId('playback')).toBeVisible();
  // Still on the second frame, not carried to the last.
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Next move' })).toBeEnabled();
});

test('a game against a person ends with the end block and no Review button', async ({ page }) => {
  await mockGames(page);
  const f = frame([[4, 4, 0]], 0, [5, 3]);
  await page.route('**/api/sessions/ab12', (route) =>
    route.fulfill({
      json: sessionOver({
        seats: HUMANS,
        turns: [answer(0, '4 4')],
        frames: [frame(), f],
        result: { winner: 0, end: { kind: 'finished' } },
      }),
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('end-block')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Review', exact: true })).toHaveCount(0);
  await expect(page.getByTestId('result')).toContainText('1–0 · X wins');
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
});

test('the board keeps its place: running game, finished game, saved game in the Review tab', async ({
  page,
}) => {
  await mockSaved(page);
  let current: Record<string, unknown> = sessionOver({
    seats: HUMANS,
    status: 'waiting_human',
    to_act: [0],
    result: null,
    turns: gameTurns(2),
    frames: gameFrames(2),
  });
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: current }));
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playback-slot')).toBeAttached();
  const running = await slotBox(page);

  current = sessionOver({ seats: HUMANS });
  await expect(page.getByTestId('playback')).toBeVisible();
  expect(await slotBox(page)).toEqual(running);

  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  expect(await slotBox(page)).toEqual(running);
});

test('the review fits the window, with the playback bar and a table as tall as the board', async ({
  page,
}) => {
  await mockSaved(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  const pages = [
    { name: 'review tab', hash: `/#/game/uttt/review/saved/${SAVED}` },
    { name: 'finished game', hash: '/#/game/uttt/ab12' },
  ];
  for (const { name: where, hash } of pages) {
    for (const viewport of [
      { width: 1536, height: 730 },
      { width: 1366, height: 650 },
      { width: 1920, height: 970 },
    ]) {
      await page.setViewportSize(viewport);
      await page.goto(hash);
      await expect(page.getByRole('button', { name: 'Next move' })).toBeVisible();
      const name = `${where} ${viewport.width}x${viewport.height}`;
      const fit = await pageHeights(page);
      expect(fit.scroll, name).toBeLessThanOrEqual(fit.inner);
      const bar = await shown(page.getByTestId('playback')).boundingBox();
      expect(bar!.y + bar!.height, name).toBeLessThanOrEqual(viewport.height);
      const squares = await shown(page.locator('.uttt-board')).boundingBox();
      const table = await shown(page.locator('.panel.table')).boundingBox();
      expect(Math.abs(table!.y - squares!.y), name).toBeLessThanOrEqual(1);
      expect(
        Math.abs(table!.y + table!.height - (squares!.y + squares!.height)),
        name,
      ).toBeLessThanOrEqual(1);
    }
  }
});

test('in a narrower window the board and playback bar still fit the height', async ({ page }) => {
  await mockSaved(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  for (const hash of [`/#/game/uttt/review/saved/${SAVED}`, '/#/game/uttt/ab12']) {
    for (const viewport of [
      { width: 1100, height: 700 },
      { width: 1024, height: 768 },
    ]) {
      await page.setViewportSize(viewport);
      await page.goto(hash);
      await expect(page.getByRole('button', { name: 'Next move' })).toBeVisible();
      const name = `${hash} ${viewport.width}x${viewport.height}`;
      const squares = await shown(page.locator('.uttt-board')).boundingBox();
      const bar = await shown(page.getByTestId('playback')).boundingBox();
      expect(squares!.y + squares!.height, name).toBeLessThanOrEqual(viewport.height);
      expect(bar!.y + bar!.height, name).toBeLessThanOrEqual(viewport.height);
    }
  }
});

test('the final frame of a game won by three small boards has the dashed line, the first has none', async ({
  page,
}) => {
  const small: (0 | 1 | null)[][] = [
    [1, 1, 1],
    [null, null, null],
    [null, null, null],
  ];
  const frames = gameFrames();
  const last = { ...frames[frames.length - 1]!, small };
  await mockSaved(page, { ...recordView(), frames: [...frames.slice(0, -1), last] });
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await expect(page.getByRole('button', { name: 'Next move' })).toBeVisible();
  await expect(page.getByTestId('win-lines')).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(page.getByTestId('win-lines')).toBeVisible();
  await page.keyboard.press('Home');
  await expect(page.getByTestId('win-lines')).toHaveCount(0);
});
