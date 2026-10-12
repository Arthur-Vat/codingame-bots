import { expect, test, type Page } from '@playwright/test';
import { serveArena } from './arena';
import { gameFrames, gameTurns, mockGames, record, recordView } from './fixtures';

// One page per game, with the Play, Review and History tabs. The API is mocked with page.route.

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

const tab = (page: Page, name: 'Play' | 'Review' | 'History') =>
  page.getByRole('navigation', { name: 'Game' }).getByRole('link', { name });

const cell = (page: Page, row: number, col: number, mark: 'X' | 'O') =>
  page.getByRole('img', { name: `row ${row} column ${col} ${mark}` });

test('a game page has three tabs in the top bar, and the home page has none', async ({ page }) => {
  await mockAll(page);
  await page.goto('/');
  await expect(page.getByRole('navigation', { name: 'Game' })).toHaveCount(0);
  await expect(page.getByRole('link', { name: 'History' })).toHaveCount(0);
  await expect(page.getByRole('link', { name: 'Games' })).toHaveAttribute('aria-current', 'page');

  await page.goto('/#/game/uttt');
  await expect(page.locator('.crumb')).toHaveText('Ultimate Tic-Tac-Toe');
  const nav = page.getByRole('navigation', { name: 'Game' });
  await expect(nav.getByRole('link')).toHaveText(['Play', 'Review', 'History']);
  await expect(tab(page, 'Play')).toHaveAttribute('aria-current', 'page');
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt');
  await expect(tab(page, 'Review')).toHaveAttribute('href', '#/game/uttt/review');
  await expect(tab(page, 'History')).toHaveAttribute('href', '#/game/uttt/history');
  await expect(page.getByRole('link', { name: 'Games' })).not.toHaveAttribute(
    'aria-current',
    'page',
  );

  await tab(page, 'Review').click();
  await expect(tab(page, 'Review')).toHaveAttribute('aria-current', 'page');
  await expect(tab(page, 'Play')).not.toHaveAttribute('aria-current', 'page');
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
  await page.goto('/#/game/uttt/review');
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');
  await page.goto('/#/game/uttt/history/arena');
  await expect(tab(page, 'Play')).toHaveAttribute('href', '#/game/uttt/ab12');
  await expect(tab(page, 'History')).toHaveAttribute('href', '#/game/uttt/history/arena');
});

test('Review with nothing open says so and leads to the history', async ({ page }) => {
  await mockAll(page);
  await serveArena(page.context(), { index: null });
  await page.goto('/#/game/uttt/ab12');
  await tab(page, 'Review').click();
  await expect(page).toHaveURL(/#\/game\/uttt\/review$/);
  await expect(page.getByText('No game open for review.')).toBeVisible();
  await page.getByRole('link', { name: 'Open one from the history' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history$/);
  await expect(page.getByTestId('history-row')).toHaveCount(1);
});

test('History, a row’s Review, and the tabs remember it', async ({ page }) => {
  await mockAll(page);
  await page.goto('/#/game/uttt/ab12');
  await tab(page, 'History').click();
  await page.getByTestId('history-row').getByRole('link', { name: 'Review' }).click();
  await expect(page).toHaveURL(new RegExp(`#/game/uttt/review/saved/${SAVED}$`));
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(tab(page, 'Review')).toHaveAttribute('aria-current', 'page');

  await tab(page, 'Play').click();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('player-0')).toContainText('Alice');
  await expect(page.getByRole('button', { name: 'First move' })).toHaveCount(0);

  await tab(page, 'Review').click();
  await expect(page).toHaveURL(new RegExp(`#/game/uttt/review/saved/${SAVED}$`));
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByText('Saved (arena match)')).toBeVisible();
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

  await tab(page, 'Review').click();
  await page.getByRole('link', { name: 'Open one from the history' }).click();
  await page.getByTestId('history-row').getByRole('link', { name: 'Review' }).click();
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  // The saved game starts on its first frame. Keys move that review, not the hidden game.
  await page.keyboard.press('End');
  await expect(page.getByRole('button', { name: 'Last move' })).toBeDisabled();
  await page.keyboard.press('Space');
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

  await tab(page, 'Play').click();
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(cell(page, 5, 3, 'X')).toBeVisible();
  await expect(cell(page, 3, 7, 'O')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Last move' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Pause' })).toHaveCount(0);
});

test('a playing review stops when its tab is left and goes on when it is shown again', async ({
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
  await expect(cell(page, 4, 4, 'X')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();
  await page.clock.runFor(31_000);
  await expect(cell(page, 4, 4, 'X')).toBeVisible();
});

test('the old addresses redirect once, and Back does not return to them', async ({ page }) => {
  await mockAll(page, over());
  await serveArena(page.context(), { index: null });
  await page.addInitScript(
    'window.seen = []; window.addEventListener("hashchange", (event) => window.seen.push(new URL(event.newURL).hash));',
  );
  const seen = () => page.evaluate<string[]>('window.seen');
  const settled = () =>
    page.evaluate(
      'new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 50))))',
    );

  await page.goto('/#/game/uttt');
  const redirects: [string, RegExp][] = [
    ['/#/history', /#\/game\/uttt\/history$/],
    ['/#/history/arena', /#\/game\/uttt\/history\/arena$/],
    [`/#/review/saved/${SAVED}`, new RegExp(`#/game/uttt/review/saved/${SAVED}$`)],
    ['/#/review/session/ab12', /#\/game\/uttt\/ab12$/],
    ['/#/game/uttt/review/session/ab12', /#\/game\/uttt\/ab12$/],
  ];
  for (const [old, target] of redirects) {
    await page.goto('/#/game/uttt');
    const before = (await seen()).length;
    await page.goto(old);
    await expect(page).toHaveURL(target);
    await settled();
    // The old address, then the new one: no loop between two routes.
    expect((await seen()).slice(before), old).toHaveLength(2);
    // Back leaves the redirect behind: it goes to where the visitor came from.
    await page.goBack();
    await expect(page).toHaveURL(/#\/game\/uttt$/);
  }
});

test('an old review of a game that cannot be shown says why, and does not redirect', async ({
  page,
}) => {
  await mockGames(page);
  await page.route('**/api/history/nope/view', (route) =>
    route.fulfill({ status: 404, json: { error: 'no saved game "nope"' } }),
  );
  await page.goto('/#/review/saved/nope');
  await expect(page.getByRole('alert')).toHaveText('no saved game "nope"');
  await expect(page).toHaveURL(/#\/review\/saved\/nope$/);
});

test('an old history address shows the home page’s notice when the games cannot be loaded', async ({
  page,
}) => {
  await page.route('**/api/games', (route) => route.abort());
  await page.goto('/#/history');
  await expect(page.getByText('Start the server: cargo run --release -p studio')).toBeVisible();
  await expect(page).toHaveURL(/#\/history$/);
});

test('a review of another game’s record moves to that game’s page', async ({ page }) => {
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
  await page.goto(`/#/game/uttt/review/saved/${SAVED}`);
  await expect(page).toHaveURL(new RegExp(`#/game/other/review/saved/${SAVED}$`));
  await expect(page.locator('.crumb')).toHaveText('Other');
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
});

test('a game the server does not have is said on every tab', async ({ page }) => {
  await mockGames(page);
  for (const hash of ['/#/game/nope', '/#/game/nope/review', '/#/game/nope/history']) {
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
