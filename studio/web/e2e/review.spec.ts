import { expect, test, type Page } from '@playwright/test';
import { answer, frame, gameFrames, gameTurns, mockGames, record, recordView } from './fixtures';

// The server is not run by this job: the API is mocked with page.route.

const SAVED = 'uttt-1760000000-abcdef123456';

async function mockSaved(page: Page, view: unknown = recordView()) {
  await mockGames(page);
  await page.route(`**/api/history/${SAVED}/view`, (route) => route.fulfill({ json: view }));
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
}

async function slotBox(page: Page) {
  const box = await page.getByTestId('board-slot').boundingBox();
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
  await page.goto(`/#/review/saved/${SAVED}`);

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
  await page.goto(`/#/review/saved/${SAVED}`);
  await page.getByRole('button', { name: 'Next move' }).click();
  await expect(cell(page, 4, 4, 'X')).toBeVisible();

  // The switch is not text entry; the arrows still step, and Space toggles the switch only.
  const coordinates = page.getByLabel('Show coordinates');
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
  await page.goto(`/#/review/saved/${SAVED}`);
  await page.getByRole('button', { name: 'Play' }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Play' })).toBeVisible();
});

test('playback at 4× reaches the end and stops; a speed change keeps playing', async ({ page }) => {
  await page.clock.install();
  await mockSaved(page);
  await page.goto(`/#/review/saved/${SAVED}`);
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
  await page.goto(`/#/review/saved/${SAVED}`);
  const moves = page.getByLabel('Moves', { exact: true });
  await expect(moves).toContainText('5 3');
  await expect(moves).not.toContainText('0 0');
  await expect(page.getByTestId('result')).toContainText('1–0 · X wins');
  await expect(page.getByTestId('result')).toContainText('Invalid answer, not shown');
  await page.keyboard.press('End');
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
});

test('a session is reviewed, saved, and exported', async ({ page }) => {
  await mockGames(page);
  const posts: string[] = [];

  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  await page.route('**/api/sessions/ab12/save', (route) => {
    posts.push(route.request().method());
    return route.fulfill({ json: { id: SAVED, duplicate: false } });
  });
  await page.route('**/api/sessions/ab12/record', (route) => route.fulfill({ json: record() }));
  await page.goto('/#/review/session/ab12');

  await expect(page.getByText('Played here')).toBeVisible();
  await expect(page.getByText('≈ 100 ms, real time').first()).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  const start = await slotBox(page);

  await page.getByRole('button', { name: 'Save to history' }).click();
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
  await page.goto('/#/review/session/ab12');
  await page.getByRole('button', { name: 'Save to history' }).click();
  await expect(
    page.getByRole('status').filter({ hasText: 'Already in your history' }),
  ).toBeVisible();
});

test('a saved game is already in the history and exports its record', async ({ page }) => {
  await mockSaved(page);
  await page.goto(`/#/review/saved/${SAVED}`);
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
  await page.goto('/#/review/saved/nope');
  await expect(page.getByRole('alert')).toHaveText('no saved game "nope"');

  await page.goto('/#/review/file/f99');
  await expect(page.getByRole('alert')).toContainText('no longer loaded');
});

test('the end block of a finished game has a Review button, and two bots open it by themselves', async ({
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
  await expect(page.getByTestId('board-overlay')).toContainText('move 2');

  current = sessionOver();
  await expect(page).toHaveURL(/#\/review\/session\/ab12$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);

  // Back on the game page, a finished game stays there, with its Review button.
  await page.goBack();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('end-block')).toBeVisible();
  await page.waitForTimeout(500);
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await page.getByRole('button', { name: 'Review' }).click();
  await expect(page).toHaveURL(/#\/review\/session\/ab12$/);
});

test('a game against a person ends with a Review button and no automatic jump', async ({
  page,
}) => {
  await mockGames(page);
  const f = frame([[4, 4, 0]], 0, [5, 3]);
  await page.route('**/api/sessions/ab12', (route) =>
    route.fulfill({
      json: sessionOver({
        seats: [
          { kind: 'human', name: 'Player 1' },
          { kind: 'human', name: 'Player 2' },
        ],
        turns: [answer(0, '4 4')],
        frames: [frame(), f],
        result: { winner: 0, end: { kind: 'finished' } },
      }),
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('end-block')).toBeVisible();
  await page.getByRole('button', { name: 'Review' }).click();
  await expect(page).toHaveURL(/#\/review\/session\/ab12$/);
  await expect(page.getByTestId('result')).toContainText('1–0 · X wins');
});

test('the review keeps its board where the game page has it', async ({ page }) => {
  await mockSaved(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: sessionOver() }));
  await page.goto('/#/game/uttt/ab12');
  const onGame = await slotBox(page);
  await page.goto('/#/review/session/ab12');
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  expect(await slotBox(page)).toEqual(onGame);
});
