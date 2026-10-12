import { expect, test, type Locator, type Page } from '@playwright/test';
import {
  chooseMode,
  frame,
  gameFrames,
  gameTurns,
  mockGames,
  openSetup,
  setupMenu,
} from './fixtures';

// The setup menu (Game setup): one modal for the three modes. The API is mocked with page.route.

const human = (name: string) => ({ kind: 'human', name });
const bot = (release: string) => ({
  kind: 'bot',
  name: release,
  release,
  think_ms: 100,
  mode: 'realtime',
  fixed_iters: null,
});

function session(patch: Record<string, unknown> = {}) {
  return {
    id: 'ab12',
    game: 'uttt',
    seed: 5,
    opening_plies: 0,
    seats: [human('Player 1'), human('Player 2')],
    status: 'waiting_human',
    error: null,
    to_act: [0],
    opening_turns: 0,
    turns: [],
    frames: [frame()],
    human_moves: [],
    result: null,
    progress: null,
    ...patch,
  };
}

/** A finished game of two bots (the default) or of two people. */
function over(patch: Record<string, unknown> = {}) {
  return session({
    seats: [bot('uttt-v010'), bot('uttt-v009')],
    status: 'over',
    to_act: [],
    turns: gameTurns(),
    frames: gameFrames(),
    result: { winner: 1, end: { kind: 'finished' } },
    ...patch,
  });
}

/** The games, a session `ab12` as given, and a collector of the requests that start a game. */
async function mockApi(page: Page, current = session()): Promise<Record<string, unknown>[]> {
  const created: Record<string, unknown>[] = [];
  await mockGames(page);
  await page.route('**/api/sessions/ab12', (route) =>
    route.request().method() === 'DELETE'
      ? route.fulfill({ status: 204, body: '' })
      : route.fulfill({ json: current }),
  );
  await page.route('**/api/sessions', (route) => {
    created.push(route.request().postDataJSON() as Record<string, unknown>);
    return route.fulfill({ json: { id: 'ab12' } });
  });
  await page.route(
    (url) => url.pathname === '/api/history',
    (route) => route.fulfill({ json: [] }),
  );
  return created;
}

const tabLink = (page: Page, name: string) =>
  page.getByRole('link', { name, exact: true }).filter({ visible: true });

const pressed = (menu: Locator, mode: string) =>
  menu.getByRole('button', { name: mode, exact: true });

test('Play opens the setup menu by itself, Escape closes it, and the next visit opens it again', async ({
  page,
}) => {
  await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = setupMenu(page);
  await expect(menu).toBeVisible();
  await expect(menu).toHaveAttribute('aria-modal', 'true');
  await expect(menu.getByRole('heading', { name: 'Game setup' })).toBeVisible();
  const modes = menu.getByRole('group', { name: 'Play against' }).getByRole('button');
  await expect(modes).toHaveText([
    /A friend\s*Two people, one screen/,
    /A release\s*Any release, any think time/,
    /Bot against bot\s*Watch the game once it is played/,
  ]);
  // The studio's main use is selected: a release.
  await expect(pressed(menu, 'A release')).toHaveAttribute('aria-pressed', 'true');
  await expect(pressed(menu, 'A friend')).toHaveAttribute('aria-pressed', 'false');

  await page.keyboard.press('Escape');
  await expect(menu).toHaveCount(0);
  // It stays closed on Play: the New game button opens it again, by hand.
  await page.waitForTimeout(300);
  await expect(menu).toHaveCount(0);

  // Another tab shows no menu; the Play tab shown again opens it.
  await tabLink(page, 'History').click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history$/);
  await expect(menu).toHaveCount(0);
  await tabLink(page, 'Play').click();
  await expect(menu).toBeVisible();

  // The menu is left open on Play and the page goes to History: it closes.
  await page.goto('/#/game/uttt/history');
  await expect(menu).toHaveCount(0);
});

test('the menu opens with the mode last chosen, also from the New game button', async ({
  page,
}) => {
  await mockApi(page);
  await page.goto('/#/game/uttt');
  await chooseMode(setupMenu(page), 'Bot against bot');
  await page.keyboard.press('Escape');
  await expect(setupMenu(page)).toHaveCount(0);

  const menu = await openSetup(page);
  await expect(pressed(menu, 'Bot against bot')).toHaveAttribute('aria-pressed', 'true');
  await page.keyboard.press('Escape');

  await tabLink(page, 'History').click();
  await tabLink(page, 'Play').click();
  await expect(pressed(setupMenu(page), 'Bot against bot')).toHaveAttribute('aria-pressed', 'true');
});

test('an address with a session, running or finished, does not open the menu', async ({ page }) => {
  await mockApi(page, session());
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('Player 1');
  await page.waitForTimeout(300);
  await expect(setupMenu(page)).toHaveCount(0);

  const finished = await page.context().newPage();
  await mockApi(finished, over());
  await finished.goto('/#/game/uttt/ab12');
  await expect(finished.getByTestId('end-block')).toBeVisible();
  await finished.waitForTimeout(300);
  await expect(setupMenu(finished)).toHaveCount(0);

  // A game on the page, the History and back: the game is still shown, no menu.
  await tabLink(finished, 'History').click();
  await tabLink(finished, 'Play').click();
  await expect(finished.getByTestId('end-block')).toBeVisible();
  await expect(setupMenu(finished)).toHaveCount(0);
});

test('the menu waits for the games, and a game the server lacks opens none', async ({ page }) => {
  let release: () => void = () => {};
  const gate = new Promise<void>((resolve) => (release = resolve));
  await page.route('**/api/games', async (route) => {
    await gate;
    await route.fulfill({
      json: [{ id: 'uttt', name: 'Ultimate Tic-Tac-Toe', releases: ['uttt-v010', 'uttt-v009'] }],
    });
  });
  await page.goto('/#/game/uttt');
  await page.waitForTimeout(300);
  await expect(setupMenu(page)).toHaveCount(0);
  release();
  await expect(setupMenu(page)).toBeVisible();

  const other = await page.context().newPage();
  await mockGames(other);
  await other.goto('/#/game/nothing');
  await expect(other.getByText('The server has no game called nothing.').first()).toBeVisible();
  await other.waitForTimeout(300);
  await expect(setupMenu(other)).toHaveCount(0);
});

test('switching modes keeps what was chosen in each mode', async ({ page }) => {
  await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = setupMenu(page);
  await expect(menu).toBeVisible();

  await menu.getByLabel('Release', { exact: true }).selectOption('uttt-v009');
  await menu.getByLabel(/^Think time/).fill('4');
  await menu.getByLabel(/^Your minutes/).fill('2');
  await menu.getByRole('button', { name: 'Play O' }).click();

  await chooseMode(menu, 'A friend');
  await expect(menu.getByLabel('Release', { exact: true })).toHaveCount(0);
  await expect(menu.getByLabel(/^Minutes per side/)).toHaveValue('5');
  await menu.getByLabel(/^Minutes per side/).fill('3');

  await chooseMode(menu, 'Bot against bot');
  await expect(menu.getByLabel(/^X \(moves first\)/)).toHaveValue('uttt-v010');
  await menu.getByLabel(/^X \(moves first\)/).selectOption('uttt-v001');
  await menu.getByLabel('O', { exact: true }).selectOption('uttt-v010');
  await menu.getByLabel(/^Think time/).fill('6');

  await chooseMode(menu, 'A release');
  await expect(menu.getByLabel('Release', { exact: true })).toHaveValue('uttt-v009');
  await expect(menu.getByLabel(/^Think time/)).toHaveValue('4');
  await expect(menu.getByLabel(/^Your minutes/)).toHaveValue('2');
  await expect(menu.getByRole('button', { name: 'Play O' })).toHaveAttribute(
    'aria-pressed',
    'true',
  );

  await chooseMode(menu, 'A friend');
  await expect(menu.getByLabel(/^Minutes per side/)).toHaveValue('3');

  await chooseMode(menu, 'Bot against bot');
  await expect(menu.getByLabel(/^X \(moves first\)/)).toHaveValue('uttt-v001');
  await expect(menu.getByLabel('O', { exact: true })).toHaveValue('uttt-v010');
  await expect(menu.getByLabel(/^Think time/)).toHaveValue('6');
});

test('bot against bot: one think time for both, or one for each in advanced mode', async ({
  page,
}) => {
  const created = await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = await chooseMode(setupMenu(page), 'Bot against bot');
  await expect(menu.getByLabel(/^X think/)).toHaveCount(0);
  await expect(menu.getByLabel(/^Think time/)).toHaveValue('2');
  await expect(menu.getByText('≈ 100 ms')).toBeVisible();

  // Advanced mode off: both bots take the shared time.
  await menu.getByLabel(/^Think time/).fill('4');
  await menu.getByRole('button', { name: 'Start' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  expect(created).toHaveLength(1);
  expect(created[0]).toMatchObject({
    game: 'uttt',
    opening_plies: 0,
    seats: [
      { kind: 'bot', release: 'uttt-v010', think_ms: 500, mode: 'realtime' },
      { kind: 'bot', release: 'uttt-v009', think_ms: 500, mode: 'realtime' },
    ],
  });

  // Advanced mode on: each bot has its own, starting from the shared one.
  await openSetup(page);
  await chooseMode(menu, 'Bot against bot');
  await menu.getByLabel(/^Think time/).fill('5');
  await menu.getByLabel('Advanced mode').check();
  await expect(menu.getByLabel(/^X think/)).toHaveValue('5');
  await expect(menu.getByLabel(/^O think/)).toHaveValue('5');
  await menu.getByLabel(/^X think/).fill('0');
  await menu.getByLabel(/^O think/).fill('6');
  await menu.getByRole('button', { name: 'Start' }).click();
  await expect.poll(() => created.length).toBe(2);
  expect(created[1]).toMatchObject({
    seats: [
      { kind: 'bot', release: 'uttt-v010', think_ms: 25, mode: 'realtime' },
      { kind: 'bot', release: 'uttt-v009', think_ms: 2000, mode: 'realtime' },
    ],
  });

  // Turned off again, the shared time is back for both.
  await openSetup(page);
  await chooseMode(menu, 'Bot against bot');
  await menu.getByLabel('Advanced mode').check();
  await menu.getByLabel('Advanced mode').uncheck();
  await expect(menu.getByLabel(/^Think time/)).toHaveValue('2');
});

test('the times of the two bots survive a switch of mode and advanced mode off and on', async ({
  page,
}) => {
  await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = await chooseMode(setupMenu(page), 'Bot against bot');
  await menu.getByLabel('Advanced mode').check();
  await menu.getByLabel(/^X think/).fill('0');
  await menu.getByLabel(/^O think/).fill('6');

  await chooseMode(menu, 'A release');
  await menu.getByLabel('Advanced mode').uncheck();
  await menu.getByLabel('Advanced mode').check();
  await chooseMode(menu, 'Bot against bot');
  await expect(menu.getByLabel(/^X think/)).toHaveValue('0');
  await expect(menu.getByLabel(/^O think/)).toHaveValue('6');
});

test('a game against a release sends the request of the choices made', async ({ page }) => {
  const created = await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = setupMenu(page);
  await expect(menu).toBeVisible();
  await expect(menu.getByLabel('Release', { exact: true })).toHaveValue('uttt-v010');
  await menu.getByLabel('Release', { exact: true }).selectOption('uttt-v009');
  await menu.getByRole('button', { name: 'Play O' }).click();
  await menu.getByLabel(/^Think time/).fill('4');
  await menu.getByLabel(/^Your minutes/).fill('2');
  await menu.getByLabel(/^Increment/).fill('4');
  await menu.getByLabel('Advanced mode').check();
  await menu.getByLabel('Seed').fill('1234');
  await menu.getByRole('button', { name: 'Start' }).click();

  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  expect(created).toEqual([
    {
      game: 'uttt',
      seed: 1234,
      opening_plies: 0,
      seats: [
        { kind: 'bot', release: 'uttt-v009', think_ms: 500, mode: 'fixed' },
        { kind: 'human', name: 'You' },
      ],
    },
  ]);
  // The clock is the one chosen: 2 minutes and 5 seconds.
  await expect(page.locator('.info dt:text-is("Clock") + dd')).toHaveText('2 + 5');
});

test('a friend game sends two human seats, and the seed only in advanced mode', async ({
  page,
}) => {
  const created = await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = await chooseMode(setupMenu(page), 'A friend');
  await expect(menu.getByLabel('Seed')).toHaveCount(0);
  await menu.getByLabel('Advanced mode').check();
  await expect(menu.getByLabel('Seed')).toBeVisible();
  await expect(menu.getByLabel(/think/i)).toHaveCount(0);
  await menu.getByLabel('Seed').fill('77');
  await menu.getByRole('button', { name: 'Start' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  expect(created).toEqual([
    {
      game: 'uttt',
      seed: 77,
      opening_plies: 0,
      seats: [human('Player 1'), human('Player 2')],
    },
  ]);
});

test('New game in the end block opens the menu with the mode just played', async ({ page }) => {
  await mockApi(page, over());
  await page.goto('/#/game/uttt/ab12');
  await page.getByTestId('end-block').getByRole('button', { name: 'New game' }).click();
  await expect(pressed(setupMenu(page), 'Bot against bot')).toHaveAttribute('aria-pressed', 'true');

  const friends = await page.context().newPage();
  await mockApi(friends, over({ seats: [human('Player 1'), human('Player 2')] }));
  await friends.goto('/#/game/uttt/ab12');
  await friends.getByTestId('end-block').getByRole('button', { name: 'New game' }).click();
  await expect(pressed(setupMenu(friends), 'A friend')).toHaveAttribute('aria-pressed', 'true');
  // The left column's New game button opens it with the mode last chosen, the finished game's.
  await friends.keyboard.press('Escape');
  await expect(setupMenu(friends)).toHaveCount(0);
  await openSetup(friends);
  await expect(pressed(setupMenu(friends), 'A friend')).toHaveAttribute('aria-pressed', 'true');
});

test('the menu fits a 1536x730 window in every mode, also in advanced mode', async ({ page }) => {
  await page.setViewportSize({ width: 1536, height: 730 });
  await mockApi(page);
  await page.goto('/#/game/uttt');
  const menu = setupMenu(page);
  await expect(menu).toBeVisible();
  for (const advanced of [false, true]) {
    for (const mode of ['A friend', 'A release', 'Bot against bot'] as const) {
      await chooseMode(menu, mode);
      const advancedMode = menu.getByLabel('Advanced mode');
      if (advanced) await advancedMode.check();
      else await advancedMode.uncheck();
      const box = await menu.boundingBox();
      expect(box).not.toBeNull();
      expect(box!.x).toBeGreaterThanOrEqual(0);
      expect(box!.y).toBeGreaterThanOrEqual(0);
      expect(box!.x + box!.width).toBeLessThanOrEqual(1536);
      expect(box!.y + box!.height).toBeLessThanOrEqual(730);
      // Nothing inside scrolls: the whole menu is shown.
      const [scroll, client] = await menu.evaluate((element) => [
        element.scrollHeight,
        element.clientHeight,
      ]);
      expect(scroll, `${mode}, advanced ${advanced}`).toBeLessThanOrEqual(client);
      await expect(menu.getByRole('button', { name: 'Start' })).toBeInViewport();
    }
  }
});
