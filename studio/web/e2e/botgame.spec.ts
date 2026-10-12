import { expect, test, type Page } from '@playwright/test';
import { chooseMode, gameFrames, gameTurns, mockGames, openSetup } from './fixtures';

// A game between two bots is shown once it is played: a spinner over the empty board while the
// bots play, then the game from its first move, paused. The API is mocked with page.route.

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

/** The game after `count` moves, the bots still playing. */
function botGoing(count: number, patch: Record<string, unknown> = {}) {
  return botGame({
    status: 'bot_thinking',
    to_act: [count % 2],
    result: null,
    turns: gameTurns(count).map((turn) => turn.map((answer) => ({ ...answer, ms: 1500 }))),
    frames: gameFrames(count).slice(0, count + 1),
    ...patch,
  });
}

async function mockApi(page: Page, initial: Record<string, unknown>) {
  const mock = { current: initial };
  await mockGames(page);
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: mock.current }));
  return mock;
}

const moves = (page: Page) => page.getByLabel('Moves', { exact: true }).locator('button');
const cell = (page: Page, row: number, col: number, mark: 'X' | 'O') =>
  page.getByRole('img', { name: `row ${row} column ${col} ${mark}` });
const marks = (page: Page) => page.getByRole('img', { name: /^row \d column \d [XO]$/ });
const moveRow = (page: Page) => page.locator('.info dt:text-is("Move") + dd');

async function boardBox(page: Page) {
  const box = await page.getByTestId('board-slot').boundingBox();
  expect(box).not.toBeNull();
  return box;
}

test('while the bots play: a spinner over the empty board, nothing of the game, the move advancing', async ({
  page,
}) => {
  const mock = await mockApi(page, botGoing(2));
  await page.goto('/#/game/uttt/ab12');
  const spinner = page.getByTestId('playing');
  await expect(spinner).toBeVisible();
  await expect(spinner).toHaveAttribute('role', 'status');
  await expect(spinner).toHaveAttribute('aria-label', 'The bots are playing');
  await expect(spinner).toContainText('Playing…');
  await expect(page.getByTestId('board-overlay')).toHaveCount(0);

  // Nothing of the game: the first position, no move, no result, no time used, no playback.
  await expect(marks(page)).toHaveCount(0);
  await expect(moves(page)).toHaveCount(0);
  await expect(page.getByTestId('result')).toHaveCount(0);
  await expect(page.getByTestId('end-block')).toHaveCount(0);
  await expect(page.getByTestId('playback')).toHaveCount(0);
  await expect(page.getByTestId('playback-slot')).toBeAttached();
  await expect(page.getByTestId('player-0')).toContainText('0.0 s');
  await expect(page.getByTestId('player-1')).toContainText('0.0 s');
  await expect(page.getByRole('button', { name: /Takeback|Resign/ })).toHaveCount(0);
  // Saving or exporting would show the game before it ends.
  await expect(page.getByRole('button', { name: 'Save', exact: true })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Export file' })).toBeDisabled();
  // The status and the move being played are the only hints of progress.
  await expect(page.locator('.info dd').first()).toHaveText('uttt-v010 is thinking…');
  await expect(moveRow(page)).toHaveText('3');

  mock.current = botGoing(3);
  await expect(moveRow(page)).toHaveText('4');
  await expect(marks(page)).toHaveCount(0);
  await expect(moves(page)).toHaveCount(0);
  await expect(spinner).toBeVisible();
  // The keys have nothing to look back at.
  for (const key of ['ArrowLeft', 'Home', 'ArrowRight', 'End', ' ']) {
    await page.keyboard.press(key);
  }
  await expect(marks(page)).toHaveCount(0);
  await expect(moves(page)).toHaveCount(0);
  await expect(spinner).toBeVisible();
});

test('the game ends: the spinner goes, the moves and the result come, the playback opens on its first move', async ({
  page,
}) => {
  const mock = await mockApi(page, botGoing(3));
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playing')).toBeVisible();
  const playing = await boardBox(page);

  mock.current = botGame();
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('playing')).toHaveCount(0);
  await expect(moves(page)).toHaveCount(4);
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('end-block')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save', exact: true })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Export file' })).toBeEnabled();
  await expect(page.getByTestId('player-1')).toContainText('0.0 s');
  // The first move, paused: the board is still empty.
  await expect(marks(page)).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Previous move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Play', exact: true })).toBeVisible();
  // The board did not move when the playback bar took the room kept for it.
  expect(await boardBox(page)).toEqual(playing);

  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.getByRole('button', { name: 'Pause' }).click();
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
});

test('a game already over when opened opens on its first move', async ({ page }) => {
  await mockApi(page, botGame());
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page.getByTestId('playing')).toHaveCount(0);
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(moves(page)).toHaveCount(4);
  await expect(marks(page)).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Previous move' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Last move' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Play', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
});

test('a game that fails is shown as it is, without the spinner', async ({ page }) => {
  await mockApi(page, botGoing(2, { status: 'failed', to_act: [], error: 'the bot crashed' }));
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByRole('alert')).toHaveText('the bot crashed');
  await expect(page.getByTestId('playing')).toHaveCount(0);
  await expect(moves(page)).toHaveCount(2);
  await expect(cell(page, 4, 5, 'O')).toBeVisible();
});

test('no spinner while the bots are compiled, nor in a game with a person', async ({ page }) => {
  const mock = await mockApi(
    page,
    botGoing(0, { status: 'compiling', frames: [], turns: [], to_act: [] }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('board-overlay')).toContainText('Compiling');
  await expect(page.getByTestId('playing')).toHaveCount(0);

  mock.current = botGoing(2, {
    seats: [
      { kind: 'human', name: 'Alice' },
      { kind: 'human', name: 'Bob' },
    ],
    status: 'waiting_human',
  });
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(moves(page)).toHaveCount(2);
  await expect(page.getByTestId('playing')).toHaveCount(0);

  mock.current = botGoing(2, { seats: [{ kind: 'human', name: 'Alice' }, bot('uttt-v009')] });
  await page.goto('/#/game/uttt/ab12');
  // The same address does not reload the page: wait for the game against the computer.
  await expect(page.getByTestId('player-1')).toContainText('uttt-v009');
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(moves(page)).toHaveCount(2);
  await expect(page.getByTestId('playing')).toHaveCount(0);
});

test('starting another game while the bots play still warns that it ends this one', async ({
  page,
}) => {
  await mockApi(page, botGoing(2));
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playing')).toBeVisible();
  await chooseMode(await openSetup(page), 'Bot vs bot');
  await expect(page.getByText('Starting a new game ends the one in progress.')).toBeVisible();
});

test('the ring spins, and stands still for those who ask for less motion', async ({ page }) => {
  await mockApi(page, botGoing(2));
  await page.goto('/#/game/uttt/ab12');
  const animation = () =>
    page.evaluate<string>("getComputedStyle(document.querySelector('.spinner')).animationName");
  await expect(page.getByTestId('playing')).toBeVisible();
  expect(await animation()).toBe('spin');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  expect(await animation()).toBe('none');
});
