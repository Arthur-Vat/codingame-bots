import { expect, test, type Page } from '@playwright/test';

// The server is not run by this job: the API is mocked with page.route.

const GAMES = [
  {
    id: 'uttt',
    name: 'Ultimate Tic-Tac-Toe',
    releases: ['uttt-v003', 'uttt-v002', 'uttt-v001'],
  },
];

const SESSION = {
  id: 'ab12',
  game: 'uttt',
  seed: 5,
  opening_plies: 0,
  seats: [
    { kind: 'human', name: 'You' },
    {
      kind: 'bot',
      name: 'uttt-v003',
      release: 'uttt-v003',
      think_ms: 100,
      mode: 'fixed',
      fixed_iters: 1000,
    },
  ],
  status: 'bot_thinking',
  error: null,
  to_act: [1],
  opening_turns: 0,
  turns: [
    [{ seat: 0, lines: ['4 4'], ms: 0 }],
    [{ seat: 1, lines: ['4 5'], ms: 12 }],
    [{ seat: 0, lines: ['5 0'], ms: 0 }],
  ],
  frames: [],
  human_moves: [],
  result: null,
  progress: null,
};

async function mockGames(page: Page) {
  await page.route('**/api/games', (route) => route.fulfill({ json: GAMES }));
}

test('the home screen lists the games the server offers', async ({ page }) => {
  await mockGames(page);
  await page.goto('/');
  await expect(page).toHaveTitle('studio');
  await expect(page.getByRole('heading', { name: 'Choose a game' })).toBeVisible();
  const card = page.getByRole('link', { name: /Ultimate Tic-Tac-Toe/ });
  await expect(card).toContainText('3 releases');
  await expect(card).toContainText('Latest: uttt-v003');
  await expect(page.getByText('Next game')).toBeVisible();
  await expect(page.getByText('More games')).toBeVisible();
});

test('a card opens the game page with the three ways to start', async ({ page }) => {
  await mockGames(page);
  await page.goto('/');
  await page.getByRole('link', { name: /Ultimate Tic-Tac-Toe/ }).click();
  await expect(page).toHaveURL(/#\/game\/uttt$/);
  await expect(page.locator('.crumb')).toHaveText('Ultimate Tic-Tac-Toe');
  for (const name of ['Play with a friend', 'Play the computer', 'Bot against bot']) {
    await expect(page.getByRole('button', { name: new RegExp(name) })).toBeVisible();
  }
  await expect(page.getByText('Choose a mode on the left to start a game.')).toBeVisible();
  await expect(page.getByLabel('Heatmap of best moves')).toBeDisabled();
});

test('playing the computer sends a human seat and a fixed bot seat', async ({ page }) => {
  await mockGames(page);
  let body: unknown = null;
  await page.route('**/api/sessions', async (route) => {
    body = route.request().postDataJSON();
    await route.fulfill({ json: { id: 'ab12' } });
  });
  await page.route('**/api/sessions/ab12', (route) => route.fulfill({ json: SESSION }));

  await page.goto('/#/game/uttt');
  await page.getByRole('button', { name: /Play the computer/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Play the computer' });
  await expect(dialog).toBeVisible();
  await expect(dialog.getByRole('button', { name: 'Start' })).toBeFocused();
  await expect(dialog.getByLabel('Release')).toHaveValue('uttt-v003');
  await expect(dialog.getByText('uttt-v003 (latest)')).toBeAttached();
  await expect(dialog.getByText('≈ 100 ms')).toBeVisible();
  await dialog.getByRole('button', { name: 'Start' }).click();

  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  expect(body).toMatchObject({
    game: 'uttt',
    opening_plies: 0,
    seats: [
      { kind: 'human', name: 'You' },
      { kind: 'bot', release: 'uttt-v003', think_ms: 100, mode: 'fixed' },
    ],
  });
  expect(typeof (body as { seed: unknown }).seed).toBe('number');

  // The session's seats and answers are shown.
  await expect(dialog).toBeHidden();
  await expect(page.getByTestId('player-0')).toContainText('You');
  await expect(page.getByTestId('player-1')).toContainText('uttt-v003');
  await expect(page.getByLabel('Moves', { exact: true })).toContainText('4 4');
  await expect(page.getByLabel('Moves', { exact: true })).toContainText('4 5');
  await expect(page.getByLabel('Moves', { exact: true })).toContainText('5 0');
  await expect(page.getByText('uttt-v003 is thinking…')).toBeVisible();
});

test('Escape closes the dialog and the advanced mode shows the seed', async ({ page }) => {
  await mockGames(page);
  await page.goto('/#/game/uttt');
  await page.getByRole('button', { name: /Bot against bot/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Bot against bot' });
  await expect(dialog.getByLabel(/^X \(moves first\)/)).toHaveValue('uttt-v003');
  await expect(dialog.getByLabel('O', { exact: true })).toHaveValue('uttt-v002');
  await expect(dialog.getByLabel('Seed')).toBeHidden();
  await dialog.getByLabel('Advanced mode').check();
  await expect(dialog.getByLabel('Seed')).toBeVisible();
  await expect(dialog.getByText('Otherwise taken from the clock.')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(dialog).toBeHidden();
});

test('the board slot keeps its size when the coordinates are shown', async ({ page }) => {
  await mockGames(page);
  await page.goto('/#/game/uttt');
  const slot = page.getByTestId('board-slot');
  await expect(slot).toBeVisible();
  const before = await slot.boundingBox();
  await page.getByLabel('Show coordinates').check();
  await expect(page.getByTestId('board-wrap')).toHaveClass(/coords/);
  const after = await slot.boundingBox();
  expect(before).not.toBeNull();
  expect(after).toEqual(before);
  expect(before?.width).toBeCloseTo(before?.height ?? 0, 1);
});

test('the history page is a placeholder', async ({ page }) => {
  await mockGames(page);
  await page.goto('/#/history');
  await expect(page.getByText('Saved games appear here')).toBeVisible();
  await expect(page.getByRole('link', { name: 'History' })).toHaveAttribute('aria-current', 'page');
});

test('an unreachable server asks to start it', async ({ page }) => {
  await page.route('**/api/games', (route) => route.abort());
  await page.goto('/');
  await expect(page.getByText('Start the server: cargo run --release -p studio')).toBeVisible();
});

test('the home screen connects by itself once the server answers', async ({ page }) => {
  let calls = 0;
  await page.route('**/api/games', (route) =>
    ++calls === 1 ? route.abort() : route.fulfill({ json: GAMES }),
  );
  await page.goto('/');
  await expect(page.getByText('The page connects by itself')).toBeVisible();
  await expect(page.getByRole('link', { name: /Ultimate Tic-Tac-Toe/ })).toBeVisible({
    timeout: 8000,
  });
  await expect(page.getByText('Start the server')).toBeHidden();
});

test('starting a second game deletes the first session', async ({ page }) => {
  await mockGames(page);
  const deleted: string[] = [];
  await page.route('**/api/sessions', (route) => route.fulfill({ json: { id: 'cd34' } }));
  await page.route('**/api/sessions/*', (route) => {
    const id = new URL(route.request().url()).pathname.split('/').pop();
    if (route.request().method() === 'DELETE') {
      deleted.push(id ?? '');
      return route.fulfill({ json: { deleted: true } });
    }
    return route.fulfill({ json: { ...SESSION, id } });
  });

  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('You');
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  await page.getByRole('button', { name: 'Start' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/cd34$/);
  await expect.poll(() => deleted).toEqual(['ab12']);
});

test('the dialog holds still while the game is created, and shows a refusal', async ({ page }) => {
  await mockGames(page);
  let release: () => void = () => {};
  const gate = new Promise<void>((resolve) => (release = resolve));
  await page.route('**/api/sessions', async (route) => {
    await gate;
    await route.fulfill({ status: 400, json: { error: 'unknown release "uttt-v003"' } });
  });
  await page.goto('/#/game/uttt');
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Play with a friend' });
  await dialog.getByRole('button', { name: 'Start' }).click();
  await expect(dialog.getByRole('button', { name: 'Starting…' })).toBeDisabled();
  await page.keyboard.press('Escape');
  await dialog.getByRole('button', { name: 'Close' }).click();
  await page.mouse.click(5, 5);
  await expect(dialog).toBeVisible();
  release();
  await expect(dialog.getByRole('alert')).toHaveText('unknown release "uttt-v003"');
  await expect(dialog.getByRole('button', { name: 'Start' })).toBeEnabled();
  await page.keyboard.press('Escape');
  await expect(dialog).toBeHidden();
});

test('a lost connection shows while a session is open, and clears', async ({ page }) => {
  await mockGames(page);
  let fail = false;
  await page.route('**/api/sessions/ab12', (route) =>
    fail ? route.abort() : route.fulfill({ json: SESSION }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('player-0')).toContainText('You');
  await expect(page.getByText('Connection lost')).toBeHidden();
  fail = true;
  await expect(page.getByText('Connection lost — retrying')).toBeVisible();
  // The turns already shown stay.
  await expect(page.getByLabel('Moves', { exact: true })).toContainText('4 5');
  fail = false;
  await expect(page.getByText('Connection lost')).toBeHidden({ timeout: 5000 });
});
