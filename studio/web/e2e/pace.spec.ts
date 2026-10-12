import { expect, test, type Locator, type Page } from '@playwright/test';
import { gameFrames, gameTurns, mockGames } from './fixtures';

// A game between two bots is shown move by move at a pace the viewer picks. The server is not
// run: the API is mocked with page.route, and the mock changes the session by hand where the
// server would. The pacing only lags what the page shows behind the mocked session, so the tests
// count the moves and the frames shown, never exact times.

const bot = (name: string) => ({
  kind: 'bot',
  name,
  release: name,
  think_ms: 100,
  mode: 'realtime',
  fixed_iters: null,
});

/** A game between two bots: by default the whole short game, over (4 moves, 5 frames). */
function botGame(patch: Record<string, unknown> = {}) {
  return {
    id: 'ab12',
    game: 'uttt',
    seed: 5,
    opening_plies: 0,
    seats: [bot('uttt-v010'), bot('uttt-v009')],
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

/** The game after `count` moves, still going on. */
function botGoing(count: number, patch: Record<string, unknown> = {}) {
  return botGame({
    status: 'bot_thinking',
    to_act: [count % 2],
    result: null,
    turns: gameTurns(count),
    frames: gameFrames(count).slice(0, count + 1),
    ...patch,
  });
}

interface Mock {
  current: Record<string, unknown>;
  saves: number;
  /** The games started so far get these ids in turn. */
  created: number;
}

const IDS = ['ab12', 'cd34', 'ef56'];

async function mockApi(page: Page, initial: Record<string, unknown>): Promise<Mock> {
  const mock: Mock = { current: initial, saves: 0, created: 0 };
  await mockGames(page);
  await page.route('**/api/sessions', (route) =>
    route.fulfill({ json: { id: IDS[mock.created++] } }),
  );
  await page.route(/\/api\/sessions\/[0-9a-f]+(\/.*)?$/, async (route) => {
    const request = route.request();
    const [, id = '', rest = ''] = /^\/api\/sessions\/([0-9a-f]+)(\/.*)?$/.exec(
      new URL(request.url()).pathname,
    ) ?? ['', '', ''];
    const path = rest === '' ? '/' : rest;
    if (request.method() === 'GET' && path === '/') {
      await route.fulfill({ json: { ...mock.current, id } });
    } else if (request.method() === 'POST' && path === '/save') {
      mock.saves += 1;
      await route.fulfill({ json: { id: 'uttt-1-abc', duplicate: false } });
    } else {
      await route.fulfill({ status: 204, body: '' });
    }
  });
  return mock;
}

/** The part of a locator on screen: the Play tab stays in the page, hidden, behind the others. */
const shown = (locator: Locator) => locator.filter({ visible: true });

const moves = (page: Page) => page.getByLabel('Moves', { exact: true }).locator('button');
const cell = (page: Page, row: number, col: number, mark: 'X' | 'O') =>
  page.getByRole('img', { name: `row ${row} column ${col} ${mark}` });
/** The info card's status line. */
const status = (page: Page) => page.locator('.info dd').first();
const pace = (page: Page) => page.getByTestId('pace');
const setPace = (page: Page, label: string) =>
  pace(page).getByRole('button', { name: label }).click();

/** Starts a game between two bots from the page, as the viewer does: it is shown from its first move. */
async function startBots(page: Page) {
  await page.goto('/#/game/uttt');
  await page.getByRole('button', { name: /Bot against bot/ }).click();
  await page
    .getByRole('dialog', { name: 'Bot against bot' })
    .getByRole('button', { name: 'Start' })
    .click();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(pace(page)).toBeVisible();
}

test('the lobby says what the bot game is', async ({ page }) => {
  await mockApi(page, botGoing(1));
  await page.goto('/#/game/uttt');
  await expect(page.getByRole('button', { name: /Bot against bot/ })).toContainText(
    'Watch two releases play, move by move',
  );
});

test('a game between two bots shows the pace bar, 0.5 s by default, in the playback bar’s place', async ({
  page,
}) => {
  const mock = await mockApi(page, botGoing(2));
  await page.goto('/#/game/uttt/ab12');
  const bar = pace(page);
  await expect(bar).toBeVisible();
  await expect(bar).toHaveAttribute('role', 'group');
  await expect(page.getByRole('group', { name: 'Pace' })).toBeVisible();
  await expect(bar.getByRole('button')).toHaveText(['0.25 s', '0.5 s', '1 s', '2 s', 'Max']);
  await expect(bar.getByRole('button', { name: '0.5 s' })).toHaveAttribute('aria-pressed', 'true');
  await expect(bar.getByRole('button', { name: '1 s' })).toHaveAttribute('aria-pressed', 'false');
  await expect(page.getByTestId('playback')).toHaveCount(0);
  const box = await bar.boundingBox();
  const board = await page.getByTestId('board-slot').boundingBox();

  await setPace(page, '1 s');
  await expect(bar.getByRole('button', { name: '1 s' })).toHaveAttribute('aria-pressed', 'true');
  await expect(bar.getByRole('button', { name: '0.5 s' })).toHaveAttribute('aria-pressed', 'false');

  // The game ends: the playback bar takes the place of the pace bar, and the board stays.
  mock.current = botGame();
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(bar).toHaveCount(0);
  expect(await page.getByTestId('playback').boundingBox()).toEqual(box);
  expect(await page.getByTestId('board-slot').boundingBox()).toEqual(board);
});

test('the pace bar is not there for a friend game, nor while the bots are compiled', async ({
  page,
}) => {
  const mock = await mockApi(
    page,
    botGoing(0, { status: 'compiling', frames: [], turns: [], to_act: [] }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('board-overlay')).toContainText('Compiling');
  await expect(pace(page)).toHaveCount(0);
  await expect(page.getByTestId('playback-slot')).toBeAttached();

  mock.current = botGoing(1);
  await expect(pace(page)).toBeVisible();

  mock.current = botGoing(1, {
    seats: [
      { kind: 'human', name: 'Alice' },
      { kind: 'human', name: 'Bob' },
    ],
    status: 'waiting_human',
  });
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(pace(page)).toHaveCount(0);
  await expect(page.getByTestId('playback-slot')).toBeAttached();
});

test('moves are shown one by one; the result comes when the shown game ends, not the real one', async ({
  page,
}) => {
  const mock = await mockApi(page, botGame());
  await startBots(page);
  // The server's game is over already; the page shows it from its first move.
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(0);
  await expect(status(page)).toHaveText('uttt-v010 is thinking…');
  await expect(page.locator('.info dt:text-is("Move") + dd')).toHaveText('1');
  await expect(page.getByTestId('result')).toHaveCount(0);
  await expect(page.getByTestId('playback')).toHaveCount(0);
  await expect(page.getByTestId('end-block')).toHaveCount(0);
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);

  // One move at a time, the board with it.
  await expect(moves(page)).toHaveCount(1);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);
  await expect(status(page)).toHaveText('uttt-v009 is thinking…');
  await expect(page.locator('.info dt:text-is("Move") + dd')).toHaveText('2');
  await expect(page.getByTestId('result')).toHaveCount(0);
  await expect(page.getByTestId('end-block')).toHaveCount(0);

  // Saving while the shown game lags saves the real game, which is over; the page says
  // "In your history" only when the shown game is over too.
  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect.poll(() => mock.saves).toBe(1);
  await expect(page.getByRole('button', { name: 'In your history' })).toHaveCount(0);

  await setPace(page, 'Max');
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('end-block')).toBeVisible();
  await expect(moves(page)).toHaveCount(4);
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();
  await expect(pace(page)).toHaveCount(0);
});

test('a pace shorter than the wait is timed from the last move, and a slower pace takes effect at once', async ({
  page,
}) => {
  await mockApi(page, botGame());
  await startBots(page);
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(0);
  // Faster: the moves follow each other at 0.25 s; at 2 s the four would take eight seconds.
  await setPace(page, '0.25 s');
  await expect(page.getByTestId('playback')).toBeVisible({ timeout: 4000 });
  await expect(moves(page)).toHaveCount(4);
});

test('Max shows the latest frame at once', async ({ page }) => {
  const mock = await mockApi(page, botGoing(3));
  await startBots(page);
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(0);
  await setPace(page, 'Max');
  await expect(moves(page)).toHaveCount(3);
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  // In Max the frames come as they are played.
  mock.current = botGoing(4);
  await expect(moves(page)).toHaveCount(4);
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(page.getByTestId('result')).toHaveCount(0);

  // Back to a pace: the shown game stays where it is.
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(4);
  mock.current = botGame();
  await expect(page.getByTestId('result')).toBeVisible();
});

test('a frame that arrives late is shown as it arrives', async ({ page }) => {
  const mock = await mockApi(page, botGoing(0));
  await startBots(page);
  await setPace(page, '1 s');
  await expect(moves(page)).toHaveCount(0);
  // The bot is slower than the pace: well past one second, the next frame arrives.
  await page.waitForTimeout(1500);
  mock.current = botGoing(1);
  // Shown after the next poll (every 0.2 s), not one more pace later.
  await expect(moves(page)).toHaveCount(1, { timeout: 700 });
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  mock.current = botGoing(2);
  await expect(moves(page)).toHaveCount(2, { timeout: 1900 });
});

test('a game that fails while pacing is shown failed at once', async ({ page }) => {
  const mock = await mockApi(page, botGoing(4));
  await startBots(page);
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(0);
  mock.current = botGoing(2, { status: 'failed', to_act: [], error: 'the bot crashed' });
  await expect(page.getByRole('alert')).toHaveText('the bot crashed');
  await expect(pace(page)).toHaveCount(0);
  await expect(page.getByTestId('playback')).toHaveCount(0);
  await expect(status(page)).toHaveText('Failed: the bot crashed');
});

test('a game already over when opened is its review at once', async ({ page }) => {
  await mockApi(page, botGame());
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await expect(moves(page)).toHaveCount(4);
  await expect(pace(page)).toHaveCount(0);
});

test('a game opened while it goes on is shown from its latest frame, then paced', async ({
  page,
}) => {
  const mock = await mockApi(page, botGoing(2));
  await page.goto('/#/game/uttt/ab12');
  await expect(moves(page)).toHaveCount(2);
  await setPace(page, '2 s');
  mock.current = botGame();
  await expect(status(page)).toHaveText('uttt-v010 is thinking…');
  await expect(page.getByTestId('result')).toHaveCount(0);
  await expect(moves(page)).toHaveCount(2);
  await expect(moves(page)).toHaveCount(3);
  await expect(page.getByTestId('result')).toHaveCount(0);
  await setPace(page, 'Max');
  await expect(page.getByTestId('result')).toBeVisible();
});

test('the keys look back from the shown frame and End follows the pace again', async ({ page }) => {
  await mockApi(page, botGame());
  await startBots(page);
  await setPace(page, '2 s');
  await expect(moves(page)).toHaveCount(1);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
  await page.keyboard.press('Home');
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  // The shown game goes on behind, and the board stays where it was put.
  await expect(moves(page)).toHaveCount(2);
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
});

test('the pacing goes on while another tab is shown', async ({ page }) => {
  await mockApi(page, botGame());
  await startBots(page);
  await page.getByRole('link', { name: 'History' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history/);
  // Four moves at 0.5 s take two seconds; the game is over and shown when the tab is back.
  await page.waitForTimeout(3000);
  await page.getByRole('link', { name: 'Play' }).click();
  await expect(shown(page.getByTestId('playback'))).toBeVisible();
  await expect(shown(page.getByTestId('result'))).toContainText('0–1 · O wins');
  await expect(shown(moves(page))).toHaveCount(4);
});

test('a new game on the page keeps the pace and starts from its first move', async ({ page }) => {
  const mock = await mockApi(page, botGame());
  await startBots(page);
  await setPace(page, 'Max');
  await expect(page.getByTestId('end-block')).toBeVisible();
  mock.current = botGoing(1);
  await page.getByRole('button', { name: 'Rematch' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/cd34$/);
  await expect(pace(page)).toBeVisible();
  await expect(moves(page)).toHaveCount(1);
  await expect(pace(page).getByRole('button', { name: 'Max' })).toHaveAttribute(
    'aria-pressed',
    'true',
  );
});
