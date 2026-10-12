import { expect, test, type Page } from '@playwright/test';
import { gameFrames, gameTurns, mockGames, record, recordView } from './fixtures';

// One page per game, with the Play and History tabs and the analysis board. The API is mocked with page.route.

const SAVED = 'uttt-1760000000-abcdef123456';

const seat = (name: string) => ({ kind: 'human', name });

function running(patch: Record<string, unknown> = {}) {
  return {
    id: 'ab12',
    game: 'uttt',
    seed: 5,
    opening_plies: 0,
    seats: [seat('Alice'), seat('Bob')],
    status: 'waiting_human',
    error: null,
    to_act: [0],
    opening_turns: 0,
    turns: gameTurns(2),
    frames: gameFrames(2),
    human_moves: [],
    result: null,
    progress: null,
    ...patch,
  };
}

function over(patch: Record<string, unknown> = {}) {
  return running({
    status: 'over',
    to_act: [],
    turns: gameTurns(),
    frames: gameFrames(),
    result: { winner: 1, end: { kind: 'finished' } },
    ...patch,
  });
}

const savedRow = {
  id: SAVED,
  game: 'uttt',
  unix_time: 1_760_000_000,
  players: ['uttt-v010', 'uttt-v009'],
  winner: 1,
  end: 'finished',
  turns: 4,
  source: 'studio',
};

interface Calls {
  sessions: number;
  history: string[];
}

async function mockAll(page: Page, session: unknown = running()): Promise<Calls> {
  const calls: Calls = { sessions: 0, history: [] };
  await mockGames(page);
  await page.route('**/api/sessions/ab12', (route) => {
    calls.sessions += 1;
    return route.fulfill({ json: session });
  });
  await page.route(
    (url) => url.pathname === '/api/history',
    (route) => {
      calls.history.push(new URL(route.request().url()).search);
      return route.fulfill({ json: [savedRow] });
    },
  );
  await page.route(`**/api/history/${SAVED}/view`, (route) =>
    route.fulfill({ json: recordView() }),
  );
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
  return calls;
}

const tab = (page: Page, name: 'Play' | 'History') =>
  page.getByRole('navigation', { name: 'Game' }).getByRole('link', { name });

/** What is on screen: the Play tab stays in the page, hidden, behind the others. */
const onScreen = { visible: true };

const cell = (page: Page, row: number, col: number, mark: 'X' | 'O') =>
  page.getByRole('img', { name: `row ${row} column ${col} ${mark}` });

test('a game page has two tabs in the top bar, and the home page has none', async ({ page }) => {
  await mockAll(page);
  await page.goto('/');
  await expect(page.getByRole('navigation', { name: 'Game' })).toHaveCount(0);
  await expect(page.getByRole('link', { name: 'History' })).toHaveCount(0);
  await expect(page.getByRole('link', { name: 'Games' })).toHaveAttribute('aria-current', 'page');

  await page.goto('/#/game/uttt');
  await expect(page.locator('.crumb')).toHaveText('Ultimate Tic-Tac-Toe');
  const nav = page.getByRole('navigation', { name: 'Game' });
  await expect(nav.getByRole('link')).toHaveText(['Play', 'History']);
  await expect(tab(page, 'Play')).toHaveAttribute('aria-current', 'page');
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt');
  // The Play tab opened the setup menu, which covers the page until it is closed.
  await expect(page.getByRole('dialog', { name: 'Game setup' })).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(tab(page, 'History')).toHaveAttribute('href', '#/game/uttt/history');
  await expect(page.getByRole('link', { name: 'Games' })).not.toHaveAttribute(
    'aria-current',
    'page',
  );

  await tab(page, 'History').click();
  await expect(tab(page, 'History')).toHaveAttribute('aria-current', 'page');
  await expect(tab(page, 'Play')).not.toHaveAttribute('aria-current', 'page');
  // The Review tab is gone.
  await expect(page.getByRole('link', { name: 'Review' })).toHaveCount(0);
});

test('the tabs fit the top bar and the page does not scroll sideways', async ({ page }) => {
  await mockAll(page);
  for (const width of [1366, 1000, 700, 390]) {
    await page.setViewportSize({ width, height: 800 });
    await page.goto('/#/game/uttt/history');
    await expect(tab(page, 'History')).toBeVisible();
    const sizes = await page.evaluate<{ scroll: number; inner: number; top: number }>(
      '({ scroll: document.documentElement.scrollWidth, inner: window.innerWidth, top: document.querySelector("header.top").getBoundingClientRect().height })',
    );
    expect(sizes.scroll, `${width}`).toBeLessThanOrEqual(sizes.inner);
    if (width >= 1000) expect(sizes.top, `${width}`).toBeLessThanOrEqual(53);
  }
});

test('Play, History and back: the same session, nothing refetched from scratch', async ({
  page,
}) => {
  await mockAll(page);
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await page.getByLabel('Show coordinates').check();
  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');

  await tab(page, 'History').click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history$/);
  await expect(page.getByTestId('history-row')).toHaveCount(1);
  // The Play tab is still in the page, hidden, with its state.
  await expect(page.getByTestId('play-tab')).toBeHidden();
  await expect(page.getByTestId('play-tab')).toBeAttached();
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');

  await tab(page, 'Play').click();
  // The tab is in front once the Play tab is shown and the others are gone: before that, both
  // tabs' elements can be in the page at once.
  await expect(page.getByTestId('play-tab')).toBeVisible();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(page.getByTestId('play-tab')).toBeVisible();
  // The pinned frame and the coordinates switch are as they were.
  await expect(page.getByLabel('Show coordinates')).toBeChecked();
  await expect(cell(page, 4, 5, 'O')).toHaveCount(0);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
});

test('the Play tab is the remembered session while another tab is shown', async ({ page }) => {
  await mockAll(page);
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  // Straight to another tab by address: Play leads back to the session.
  await page.goto(`/#/game/uttt/analysis/saved/${SAVED}`);
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');
  await page.goto('/#/game/uttt/history/arena');
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');
  await expect(tab(page, 'History')).toHaveAttribute('href', '#/game/uttt/history/arena');
});

test('a history row’s Analysis board: History stays current, no Rematch, and the way back', async ({
  page,
}) => {
  await mockAll(page);
  await page.goto('/#/game/uttt/ab12');
  await tab(page, 'History').click();
  await page.getByTestId('history-row').getByRole('link', { name: 'Analysis board' }).click();
  await expect(page).toHaveURL(new RegExp(`#/game/uttt/analysis/saved/${SAVED}$`));
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Analysis board' })).toBeVisible();
  await expect(tab(page, 'History')).toHaveAttribute('aria-current', 'page');
  await expect(tab(page, 'Play')).not.toHaveAttribute('aria-current', 'page');
  // A game from the history offers neither Rematch nor New game (the hidden Play tab has its own).
  await expect(page.getByRole('button', { name: 'Rematch' }).filter(onScreen)).toHaveCount(0);
  await expect(
    page.getByRole('button', { name: 'New game', exact: true }).filter(onScreen),
  ).toHaveCount(0);
  await expect(page.getByText("The engine's analysis comes in a later version.")).toBeVisible();
  await expect(page.getByText('Saved (arena match)')).toBeVisible();

  // The Play tab still leads to the session, and the way back to the history.
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');
  await page.getByRole('link', { name: 'Back to history' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history$/);
  await expect(page.getByTestId('history-row')).toHaveCount(1);

  await tab(page, 'Play').click();
  // The tab is in front once the Play tab is shown and the others are gone: before that, both
  // tabs' elements can be in the page at once.
  await expect(page.getByTestId('play-tab')).toBeVisible();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(page.getByRole('button', { name: 'First move' })).toHaveCount(0);
});

test('a finished game keeps its frame while another tab is shown, and takes no key there', async ({
  page,
}) => {
  await mockAll(page, over());
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toBeVisible();
  await page.keyboard.press('ArrowLeft');
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(cell(page, 5, 3, 'X')).toBeVisible();

  await tab(page, 'History').click();
  await page.getByTestId('history-row').getByRole('link', { name: 'Analysis board' }).click();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  // The saved game starts on its first frame. Keys move that analysis board, not the hidden game.
  await page.keyboard.press('End');
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

  await tab(page, 'Play').click();
  // The tab is in front once the Play tab is shown and the others are gone: before that, both
  // tabs' elements can be in the page at once.
  await expect(page.getByTestId('play-tab')).toBeVisible();
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Last move' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Pause' })).toHaveCount(0);
});

test('a playing game stops when its tab is left and goes on when it is shown again', async ({
  page,
}) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  await mockAll(page, over());
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('playback')).toBeVisible();
  await page.getByRole('button', { name: 'First move' }).click();
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

  await tab(page, 'History').click();
  await expect(page.getByTestId('play-tab')).toBeHidden();
  // 120 s over 4 steps: a full minute would have moved two frames; nothing moves while hidden.
  await page.clock.runFor(60_000);
  await tab(page, 'Play').click();
  // The tab is in front once the Play tab is shown and the others are gone: before that, both
  // tabs' elements can be in the page at once.
  await expect(page.getByTestId('play-tab')).toBeVisible();
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.clock.runFor(31_000);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
});

test('the addresses of the time before the tabs, and of the Review tab, are unknown: the home page', async ({
  page,
}) => {
  await mockAll(page, over());
  for (const hash of [
    '/#/history',
    '/#/history/arena',
    `/#/review/saved/${SAVED}`,
    '/#/review/session/ab12',
    '/#/game/uttt/review',
    `/#/game/uttt/review/saved/${SAVED}`,
    '/#/game/uttt/review/session/ab12',
    '/#/game/uttt/analysis',
    '/#/game/uttt/analysis/bogus/x',
  ]) {
    await page.goto(hash);
    await expect(page.getByRole('heading', { name: 'Choose a game' }), hash).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Game' })).toHaveCount(0);
  }
});

test('the analysis board of another game’s record moves to that game’s page', async ({ page }) => {
  await mockGames(page);
  await page.route('**/api/games', (route) =>
    route.fulfill({
      json: [
        { id: 'uttt', name: 'Ultimate Tic-Tac-Toe', releases: [] },
        { id: 'other', name: 'Other', releases: [] },
      ],
    }),
  );
  await page.route(`**/api/history/${SAVED}/view`, (route) =>
    route.fulfill({ json: recordView({ game: 'other' }) }),
  );
  await page.addInitScript(
    'window.seen = []; window.addEventListener("hashchange", (event) => window.seen.push(new URL(event.newURL).hash));',
  );
  await page.goto('/#/game/uttt');
  await page.goto(`/#/game/uttt/analysis/saved/${SAVED}`);
  await expect(page).toHaveURL(new RegExp(`#/game/other/analysis/saved/${SAVED}$`));
  await expect(page.locator('.crumb')).toHaveText('Other');
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  // Replaced once, with no loop between two routes, and Back leaves it behind.
  const seen = await page.evaluate<string[]>('window.seen');
  expect(seen).toEqual([
    `#/game/uttt/analysis/saved/${SAVED}`,
    `#/game/other/analysis/saved/${SAVED}`,
  ]);
  await page.goBack();
  await expect(page).toHaveURL(/#\/game\/uttt$/);
});

test('a game the server does not have is said on every tab', async ({ page }) => {
  await mockGames(page);
  for (const hash of ['/#/game/nope', '/#/game/nope/analysis/saved/x', '/#/game/nope/history']) {
    await page.goto(hash);
    await expect(
      page.getByText('The server has no game called nope.').filter({ visible: true }).first(),
    ).toBeVisible();
  }
});

test('an empty history says so inside the game’s tab', async ({ page }) => {
  await mockGames(page);
  await page.route(
    (url) => url.pathname === '/api/history',
    (route) => route.fulfill({ json: [] }),
  );
  await page.goto('/#/game/uttt/history');
  await expect(page.getByTestId('history-empty')).toContainText('No saved games yet');
});

test('an id that every object inherits is not a game, and the page still renders', async ({
  page,
}) => {
  await mockGames(page);
  for (const hash of [
    '/#/game/constructor',
    '/#/game/constructor/history',
    '/#/game/__proto__/analysis/saved/x',
    '/#/game/toString/history/arena',
  ]) {
    await page.goto(hash);
    await expect(
      page
        .getByText(/The server has no game called /)
        .filter({ visible: true })
        .first(),
    ).toBeVisible();
    await expect(page.locator('header.top')).toBeVisible();
    await expect(page.locator('#root > *').first()).toBeAttached();
  }
});

test('a file that names an inherited id opens on the current game', async ({ page }) => {
  await mockAll(page);
  await page.goto('/#/game/uttt/history');
  await page.getByLabel('Open a file').setInputFiles({
    name: 'odd.json',
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify(record({ game: 'constructor' }))),
  });
  await expect(page).toHaveURL(/#\/game\/uttt\/analysis\/file\/f[0-9a-z]+$/);
});
