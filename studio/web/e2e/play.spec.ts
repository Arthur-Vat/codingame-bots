import { expect, test, type Page } from '@playwright/test';
import { pageHeights } from './fixtures';

// The server is not run by this job: the API is mocked with page.route. The session below is
// what the server would send; the mock changes it by hand where the server would.

const GAMES = [
  { id: 'uttt', name: 'Ultimate Tic-Tac-Toe', releases: ['uttt-v010', 'uttt-v009', 'uttt-v001'] },
];

type Mark = 0 | 1 | null;

interface FrameOptions {
  marks?: [number, number, 0 | 1][];
  playable?: [number, number][] | 'all';
  last?: [number, number] | null;
  points?: [number, number];
  winner?: 0 | 1 | null | undefined;
  /** The small boards won, `[row, col, seat]`. */
  small?: [number, number, 0 | 1][];
}

function frame({
  marks = [],
  playable = 'all',
  last = null,
  points = [0, 0],
  winner,
  small = [],
}: FrameOptions = {}) {
  const cells: Mark[][] = Array.from({ length: 9 }, () => Array<Mark>(9).fill(null));
  for (const [row, col, seat] of marks) cells[row]![col] = seat;
  const all: [number, number][] = [];
  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++) if (cells[r]![c] === null) all.push([r, c]);
  const list = playable === 'all' ? all : playable;
  const smallBoards: Mark[][] = Array.from({ length: 3 }, () => Array<Mark>(3).fill(null));
  for (const [row, col, seat] of small) smallBoards[row]![col] = seat;
  return {
    cells,
    small: smallBoards,
    last,
    playable: winner === undefined ? list : [],
    to_move: winner === undefined ? marks.length % 2 : null,
    points,
    result: winner === undefined ? null : { winner },
  };
}

function humanMoves(f: ReturnType<typeof frame>) {
  return f.playable.map(([row, col]) => ({ action: { row, col }, lines: [`${row} ${col}`] }));
}

const human = (name: string) => ({ kind: 'human', name });
const bot = (release: string, mode: 'fixed' | 'realtime' = 'fixed') => ({
  kind: 'bot',
  name: release,
  release,
  think_ms: 100,
  mode,
  fixed_iters: mode === 'fixed' ? 2500 : null,
});
const answer = (seat: number, line: string, ms = 0) => [{ seat, lines: [line], ms }];

function session(patch: Record<string, unknown>) {
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
    human_moves: humanMoves(frame()),
    result: null,
    progress: null,
    ...patch,
  };
}

interface Mock {
  current: Record<string, unknown>;
  posts: { path: string; body: unknown }[];
  /** What the next move, takeback or end answers with. */
  next: Record<string, unknown> | null;
}

async function mockApi(page: Page, initial: Record<string, unknown>): Promise<Mock> {
  const mock: Mock = { current: initial, posts: [], next: null };
  await page.route('**/api/games', (route) => route.fulfill({ json: GAMES }));
  await page.route('**/api/sessions', (route) => route.fulfill({ json: { id: 'ab12' } }));
  await page.route(/\/api\/sessions\/ab12(\/.*)?$/, async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname.replace('/api/sessions/ab12', '') || '/';
    if (request.method() === 'GET' && path === '/') {
      await route.fulfill({ json: mock.current });
    } else if (request.method() === 'GET' && path === '/record') {
      await route.fulfill({ json: { format: 1, game: 'uttt', seed: 5 } });
    } else if (request.method() === 'POST' && path === '/save') {
      mock.posts.push({ path, body: request.postDataJSON() });
      await route.fulfill({ json: { id: 'uttt-1-abc', duplicate: false } });
    } else if (request.method() === 'POST') {
      mock.posts.push({ path, body: request.postDataJSON() });
      if (mock.next !== null) mock.current = mock.next;
      await route.fulfill({ json: mock.current });
    } else {
      await route.fulfill({ status: 204, body: '' });
    }
  });
  return mock;
}

async function box(page: Page) {
  const slot = await page.getByTestId('board-slot').boundingBox();
  expect(slot).not.toBeNull();
  return slot;
}

test('a friend game: a click plays the matching move and the new frame shows the mark', async ({
  page,
}) => {
  const mock = await mockApi(page, session({}));
  const after = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  mock.next = session({
    to_act: [1],
    turns: [answer(0, '4 4')],
    frames: [frame(), after],
    human_moves: humanMoves(after),
  });
  await page.goto('/#/game/uttt/ab12');
  const before = await box(page);
  await expect(page.getByRole('button', { name: 'play row 4 column 4' })).toBeVisible();

  await page.getByRole('button', { name: 'play row 4 column 4' }).click();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  expect(mock.posts).toEqual([{ path: '/move', body: { seat: 0, index: 40 } }]);
  expect(await box(page)).toEqual(before);
  await expect(page.getByLabel('Moves', { exact: true })).toContainText('4 4');
  // O is next; only the cells of the small board 1,1 are legal in the mock's frame list, here all.
  await expect(page.getByRole('button', { name: 'play row 0 column 0' })).toBeVisible();
});

test('the move list and the arrow keys look back, and input waits for the latest frame', async ({
  page,
}) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const f2 = frame({
    marks: [
      [4, 4, 0],
      [4, 5, 1],
    ],
    last: [4, 5],
  });
  const mock = await mockApi(
    page,
    session({
      turns: [answer(0, '4 4'), answer(1, '4 5')],
      frames: [frame(), f1, f2],
      human_moves: humanMoves(f2),
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  const before = await box(page);
  await expect(page.getByRole('img', { name: 'row 4 column 5 O' })).toBeVisible();

  await page.getByRole('button', { name: '4 4', exact: true }).click();
  await expect(page.getByRole('img', { name: 'row 4 column 5 O' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /^play row/ })).toHaveCount(0);
  expect(await box(page)).toEqual(before);

  await page.keyboard.press('Home');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toHaveCount(0);
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  await page.keyboard.press('End');
  await expect(page.getByRole('img', { name: 'row 4 column 5 O' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'play row 0 column 0' })).toBeVisible();
  expect(mock.posts).toEqual([]);
});

test('a takeback keeps one turn fewer in a friend game', async ({ page }) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const f2 = frame({
    marks: [
      [4, 4, 0],
      [4, 5, 1],
    ],
    last: [4, 5],
  });
  const f3 = frame({
    marks: [
      [4, 4, 0],
      [4, 5, 1],
      [5, 0, 0],
    ],
    last: [5, 0],
  });
  const mock = await mockApi(
    page,
    session({
      to_act: [1],
      turns: [answer(0, '4 4'), answer(1, '4 5'), answer(0, '5 0')],
      frames: [frame(), f1, f2, f3],
      human_moves: humanMoves(f3),
    }),
  );
  mock.next = session({
    turns: [answer(0, '4 4'), answer(1, '4 5')],
    frames: [frame(), f1, f2],
    human_moves: humanMoves(f2),
  });
  await page.goto('/#/game/uttt/ab12');
  await page.getByRole('button', { name: '↶ Takeback' }).click();
  await expect.poll(() => mock.posts).toEqual([{ path: '/takeback', body: { turns: 2 } }]);
  await expect(page.getByRole('img', { name: 'row 5 column 0 X' })).toHaveCount(0);
});

test('a takeback against the computer removes the reply too, and resigning is the human', async ({
  page,
}) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const f2 = frame({
    marks: [
      [4, 4, 0],
      [4, 5, 1],
    ],
    last: [4, 5],
  });
  const f3 = frame({
    marks: [
      [4, 4, 0],
      [4, 5, 1],
      [5, 0, 0],
    ],
    last: [5, 0],
  });
  const mock = await mockApi(
    page,
    session({
      seats: [bot('uttt-v010'), human('You')],
      to_act: [1],
      turns: [answer(0, '4 4', 40), answer(1, '4 5'), answer(0, '5 0', 35)],
      frames: [frame(), f1, f2, f3],
      human_moves: humanMoves(f3),
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('clock-0')).toHaveText('0.1 s');
  await page.getByRole('button', { name: '↶ Takeback' }).click();
  await expect.poll(() => mock.posts).toEqual([{ path: '/takeback', body: { turns: 1 } }]);
  await page.getByRole('button', { name: '⚑ Resign' }).click();
  await page.getByRole('button', { name: 'Confirm resign' }).click();
  await expect.poll(() => mock.posts.length).toBe(2);
  expect(mock.posts[1]).toEqual({ path: '/end', body: { seat: 1, reason: 'resign' } });
});

test('a computer session that is rewinding shows the overlay with its progress', async ({
  page,
}) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const mock = await mockApi(
    page,
    session({
      seats: [human('You'), bot('uttt-v010')],
      to_act: [],
      turns: [answer(0, '4 4')],
      frames: [frame(), f1],
      human_moves: [],
      status: 'rewinding',
      progress: { done: 3, total: 12 },
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  const overlay = page.getByTestId('board-overlay');
  await expect(overlay).toContainText('Rewinding uttt-v010');
  await expect(overlay).toContainText('Replaying 12 moves with seed 5.');
  await expect(overlay.getByRole('progressbar')).toHaveAttribute('aria-valuenow', '25');
  const rewinding = await box(page);
  await expect(page.getByRole('button', { name: /^play row/ })).toHaveCount(0);

  mock.current = { ...mock.current, status: 'waiting_human', to_act: [0], progress: null };
  await expect(overlay).toHaveCount(0);
  expect(await box(page)).toEqual(rewinding);
});

test('compiling and measuring say what the server does', async ({ page }) => {
  const mock = await mockApi(
    page,
    session({
      seats: [human('You'), bot('uttt-v010')],
      status: 'compiling',
      to_act: [],
      frames: [],
      human_moves: [],
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('board-overlay')).toContainText('Compiling uttt-v010…');
  mock.current = { ...mock.current, status: 'measuring', progress: { done: 0, total: null } };
  await expect(page.getByTestId('board-overlay')).toContainText(
    "Measuring uttt-v010's speed on this computer…",
  );
});

test('bot against bot shows the move number in the info card, then becomes its review', async ({
  page,
}) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const mock = await mockApi(
    page,
    session({
      seats: [bot('uttt-v010', 'realtime'), bot('uttt-v009', 'realtime')],
      status: 'bot_thinking',
      to_act: [1],
      turns: [answer(0, '4 4', 12)],
      frames: [frame(), f1],
      human_moves: [],
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  // The game is not shown while the bots play: the empty board with a spinner over it; the card
  // tells the move being played.
  await expect(page.locator('.info dt:text-is("Move") + dd')).toHaveText('2');
  await expect(page.getByTestId('board-overlay')).toHaveCount(0);
  await expect(page.getByTestId('playing')).toBeVisible();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /Takeback|Resign/ })).toHaveCount(0);
  const live = await box(page);

  mock.current = {
    ...mock.current,
    status: 'over',
    to_act: [],
    result: { winner: 1, end: { kind: 'finished' } },
    frames: [frame(), f1, frame({ marks: [[4, 4, 0]], points: [2, 3], winner: 1 })],
    turns: [answer(0, '4 4', 12), answer(1, '4 5', 9)],
  };
  // The game ended: it becomes its review in place, from its first move, with the board in
  // place; the address stays.
  await expect(page.getByTestId('playback')).toBeVisible();
  await expect(page.getByTestId('playing')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'First move' })).toBeDisabled();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toHaveCount(0);
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('result')).toContainText('Small boards won: 2–3');
  await expect(page.getByTestId('board-overlay')).toHaveCount(0);
  await expect(page.getByTestId('player-1')).toContainText('0.0 s');
  expect(await box(page)).toEqual(live);
});

test('a finished game shows the end block with Rematch, New game, Save and Export', async ({
  page,
}) => {
  const last = frame({ marks: [[4, 4, 0]], points: [5, 3], winner: 0 });
  const mock = await mockApi(
    page,
    session({
      status: 'over',
      to_act: [],
      turns: [answer(0, '4 4')],
      frames: [frame(), last],
      human_moves: [],
      result: { winner: 0, end: { kind: 'finished' } },
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  const result = page.getByTestId('result');
  await expect(result).toContainText('1–0 · X wins');
  await expect(result).toContainText('Small boards won: 5–3');
  await expect(page.getByRole('button', { name: 'Rematch' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'New game' })).toBeVisible();

  await page.getByRole('button', { name: 'Save' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  expect(mock.posts).toEqual([{ path: '/save', body: {} }]);

  const download = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export file' }).click();
  expect((await download).suggestedFilename()).toMatch(/^uttt-\d{4}-\d{2}-\d{2}-5\.json$/);

  await page.getByRole('button', { name: 'New game' }).click();
  await expect(page.getByRole('dialog', { name: 'Play with a friend' })).toBeVisible();
});

test('a duplicate save and the reasons of an end', async ({ page }) => {
  await page.route('**/api/games', (route) => route.fulfill({ json: GAMES }));
  await page.route('**/api/sessions/ab12/save', (route) =>
    route.fulfill({ json: { id: 'x', duplicate: true } }),
  );
  await page.route('**/api/sessions/ab12', (route) =>
    route.fulfill({
      json: session({
        seats: [human('You'), bot('uttt-v010')],
        status: 'over',
        to_act: [],
        turns: [answer(0, '4 4')],
        frames: [frame(), frame({ marks: [[4, 4, 0]], winner: 1 })],
        human_moves: [],
        result: { winner: 1, end: { kind: 'timeout', seat: 0, limit_ms: 0 } },
      }),
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('result')).toContainText('Out of time');
  await page.getByRole('button', { name: 'Save' }).click();
  await expect(page.getByText('Already in your history')).toBeVisible();
});

test('a failed session shows the server error under the moves', async ({ page }) => {
  await mockApi(
    page,
    session({
      seats: [human('You'), bot('uttt-v010')],
      status: 'failed',
      error: 'uttt-v010 did not replay identically',
      to_act: [],
      human_moves: [],
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByRole('alert')).toHaveText('uttt-v010 did not replay identically');
});

test('a human clock that reaches zero ends the game with a timeout, once', async ({ page }) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const mock = await mockApi(page, session({}));
  mock.next = session({
    status: 'over',
    to_act: [],
    human_moves: [],
    result: { winner: 1, end: { kind: 'timeout', seat: 0, limit_ms: 0 } },
  });
  await page.goto('/#/game/uttt');
  // From here on, time passes only when the test says so.
  await page.clock.pauseAt(new Date('2026-01-01T00:00:10Z'));
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Play with a friend' });
  await dialog.getByLabel(/Minutes per side/).fill('0');
  await dialog.getByRole('button', { name: 'Start' }).click();

  await expect(page.getByTestId('clock-0')).toHaveText('0:30');
  await expect(page.getByTestId('clock-1')).toHaveText('0:30');
  await page.clock.runFor(12_000);
  await expect(page.getByTestId('clock-0')).toHaveText('0:18');
  await expect(page.getByTestId('clock-0')).toHaveClass(/low/);
  await expect(page.getByTestId('clock-1')).toHaveText('0:30');
  await page.clock.runFor(9_000);
  await expect(page.getByTestId('clock-0')).toHaveText('0:09.0');

  await page.clock.runFor(10_000);
  await expect.poll(() => mock.posts.length).toBe(1);
  expect(mock.posts[0]).toEqual({ path: '/end', body: { seat: 0, reason: 'timeout' } });
  await page.clock.runFor(10_000);
  expect(mock.posts.length).toBe(1);
});

test('the clocks wait while a bot thinks and while the server rewinds', async ({ page }) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const mock = await mockApi(
    page,
    session({
      seats: [human('You'), bot('uttt-v010')],
      status: 'bot_thinking',
      to_act: [1],
      turns: [answer(0, '4 4')],
      frames: [frame(), frame({ marks: [[4, 4, 0]] })],
      human_moves: [],
    }),
  );
  await page.goto('/#/game/uttt');
  await page.clock.pauseAt(new Date('2026-01-01T00:00:10Z'));
  await page.getByRole('button', { name: /Play the computer/ }).click();
  await page
    .getByRole('dialog', { name: 'Play the computer' })
    .getByRole('button', { name: 'Start' })
    .click();
  await expect(page.getByTestId('clock-0')).toHaveText('10:00');
  await page.clock.runFor(5_000);
  await expect(page.getByTestId('clock-0')).toHaveText('10:00');

  // The page learns of a new status by polling, and the next poll is a fake timer that is armed
  // only when the previous answer has arrived, in real time. So one runFor may fire nothing:
  // advance the clock in small steps until the page shows the status, and only then check.
  mock.current = { ...mock.current, status: 'rewinding', progress: { done: 0, total: 1 } };
  await expect
    .poll(async () => {
      await page.clock.runFor(250);
      return page.getByTestId('board-overlay').count();
    })
    .toBe(1);
  await expect(page.getByTestId('board-overlay')).toContainText('Rewinding uttt-v010');
  await page.clock.runFor(5_000);
  await expect(page.getByTestId('clock-0')).toHaveText('10:00');

  mock.current = { ...mock.current, status: 'waiting_human', to_act: [0], progress: null };
  await expect
    .poll(async () => {
      await page.clock.runFor(250);
      return page.getByTestId('board-overlay').count();
    })
    .toBe(0);
  // The clock runs again: it went down, by a few seconds at most (how many steps it took the
  // page to see the status is not the test's business).
  await expect
    .poll(async () => {
      await page.clock.runFor(100);
      return page.getByTestId('clock-0').innerText();
    })
    .toMatch(/^9:5\d$/);
  await expect(page.getByTestId('clock-0')).toHaveClass(/run/);
});

const OVER_BY_TIMEOUT = () =>
  session({
    status: 'over',
    to_act: [],
    human_moves: [],
    result: { winner: 1, end: { kind: 'timeout', seat: 0, limit_ms: 0 } },
  });

async function startTimedFriendGame(page: Page) {
  await page.goto('/#/game/uttt');
  // From here on, time passes only when the test says so.
  await page.clock.pauseAt(new Date('2026-01-01T00:00:10Z'));
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Play with a friend' });
  await dialog.getByLabel(/Minutes per side/).fill('0');
  await dialog.getByRole('button', { name: 'Start' }).click();
  await expect(page.getByTestId('clock-0')).toHaveText('0:30');
}

test('a clock that hits zero while a move is in flight reports the timeout after the move failed', async ({
  page,
}) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const mock = await mockApi(page, session({}));
  mock.next = OVER_BY_TIMEOUT();
  let release: () => void = () => undefined;
  const gate = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(/\/api\/sessions\/ab12\/move$/, async (route) => {
    await gate;
    await route.fulfill({ status: 409, json: { error: "it is not that human's turn" } });
  });
  await startTimedFriendGame(page);

  await page.getByRole('button', { name: 'play row 4 column 4' }).click();
  await page.clock.runFor(31_000);
  await expect(page.getByTestId('clock-0')).toHaveText('0:00.0');
  expect(mock.posts).toEqual([]);

  release();
  await expect
    .poll(() => mock.posts)
    .toEqual([{ path: '/end', body: { seat: 0, reason: 'timeout' } }]);
  await expect(page.getByTestId('result')).toContainText('Out of time');
});

test('a timeout report that fails is made again', async ({ page }) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const mock = await mockApi(page, session({}));
  const ends: unknown[] = [];
  await page.route(/\/api\/sessions\/ab12\/end$/, async (route) => {
    ends.push(route.request().postDataJSON());
    if (ends.length === 1) {
      await route.fulfill({ status: 500, json: { error: 'disk full' } });
    } else {
      mock.current = OVER_BY_TIMEOUT();
      await route.fulfill({ json: mock.current });
    }
  });
  await startTimedFriendGame(page);

  // Just past the 30 s on the clock, and well short of the 1 s before the
  // retry: the failed report's answer arrives in real time, so a longer step
  // could also run the retry's timer.
  await page.clock.runFor(30_050);
  await expect.poll(() => ends.length).toBe(1);
  await expect(page.getByText('disk full')).toBeVisible();
  await page.clock.runFor(1_500);
  await expect.poll(() => ends.length).toBe(2);
  expect(ends).toEqual([
    { seat: 0, reason: 'timeout' },
    { seat: 0, reason: 'timeout' },
  ]);
  await expect(page.getByTestId('result')).toContainText('Out of time');
  await page.clock.runFor(5_000);
  expect(ends.length).toBe(2);
});

test('the Play tab goes on while another tab is shown: its clock runs and its timeout is reported', async ({
  page,
}) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const mock = await mockApi(page, session({}));
  mock.next = OVER_BY_TIMEOUT();
  await page.route(
    (url) => url.pathname === '/api/history',
    (route) => route.fulfill({ json: [] }),
  );
  await startTimedFriendGame(page);
  await page.getByLabel('Show coordinates').check();
  await page.clock.runFor(5_000);
  await expect(page.getByTestId('clock-0')).toHaveText('0:25');

  const tabs = page.getByRole('navigation', { name: 'Game' });
  await tabs.getByRole('link', { name: 'History' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/history$/);
  await expect(page.getByTestId('play-tab')).toBeHidden();
  await page.clock.runFor(10_000);
  await tabs.getByRole('link', { name: 'Play' }).click();
  await expect(page).toHaveURL(/#\/game\/uttt\/ab12$/);
  // The clock kept running while the tab was hidden: it was not started again from the settings.
  await expect(page.getByTestId('clock-0')).toHaveText('0:15');
  await expect(page.getByLabel('Show coordinates')).toBeChecked();
  expect(mock.posts).toEqual([]);

  // A clock that runs out while another tab is shown is reported all the same.
  await tabs.getByRole('link', { name: 'History' }).click();
  await page.clock.runFor(16_000);
  await expect
    .poll(() => mock.posts)
    .toEqual([{ path: '/end', body: { seat: 0, reason: 'timeout' } }]);
  await tabs.getByRole('link', { name: 'Play' }).click();
  await expect(page.getByTestId('result')).toContainText('Out of time');
  await expect(page.getByTestId('playback')).toBeVisible();
});

test('a double click on Rematch creates one session', async ({ page }) => {
  await mockApi(
    page,
    session({
      status: 'over',
      to_act: [],
      turns: [answer(0, '4 4')],
      frames: [frame(), frame({ marks: [[4, 4, 0]], points: [5, 3], winner: 0 })],
      human_moves: [],
      result: { winner: 0, end: { kind: 'finished' } },
    }),
  );
  let created = 0;
  await page.route('**/api/sessions', async (route) => {
    created += 1;
    await new Promise((resolve) => setTimeout(resolve, 300));
    await route.fulfill({ json: { id: 'ab12' } });
  });
  await page.goto('/#/game/uttt/ab12');
  await page.getByRole('button', { name: 'Rematch' }).dblclick();
  await expect(page.getByRole('button', { name: 'Rematch' })).toBeEnabled();
  expect(created).toBe(1);
});

test('the arrow keys look back when a switch has the focus', async ({ page }) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  await mockApi(
    page,
    session({ to_act: [1], turns: [answer(0, '4 4')], frames: [frame(), f1], human_moves: [] }),
  );
  await page.goto('/#/game/uttt/ab12');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  await page.getByLabel('Show coordinates').focus();
  await page.keyboard.press('ArrowLeft');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toHaveCount(0);
  await page.keyboard.press('End');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
});

const VIEWPORTS = [
  { width: 1536, height: 730 },
  { width: 1366, height: 650 },
  { width: 1920, height: 970 },
];

// The squares of the board, without the room kept for the coordinates.
const board = (page: Page) => page.locator('.uttt-board');

test('the game page fits the window, and its table is as tall as the board', async ({ page }) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  await mockApi(
    page,
    session({
      to_act: [1],
      turns: [answer(0, '4 4')],
      frames: [frame(), f1],
      human_moves: humanMoves(f1),
    }),
  );
  for (const viewport of VIEWPORTS) {
    await page.setViewportSize(viewport);
    await page.goto('/#/game/uttt/ab12');
    await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
    const fit = await pageHeights(page);
    expect(fit.scroll, `${viewport.width}x${viewport.height}`).toBeLessThanOrEqual(fit.inner);
    const squares = await board(page).boundingBox();
    const table = await page.locator('.panel.table').boundingBox();
    expect(squares).not.toBeNull();
    expect(table).not.toBeNull();
    expect(Math.abs(table!.y - squares!.y)).toBeLessThanOrEqual(1);
    expect(Math.abs(table!.y + table!.height - (squares!.y + squares!.height))).toBeLessThanOrEqual(
      1,
    );
    expect(Math.abs(squares!.width - squares!.height)).toBeLessThanOrEqual(1);
    // It uses the room: the board is more than the old 640 px where the window is tall enough.
    if (viewport.height === 970) expect(squares!.height).toBeGreaterThan(640);
  }
});

test('in a narrower window the board still fits the window’s height', async ({ page }) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  await mockApi(
    page,
    session({
      to_act: [1],
      turns: [answer(0, '4 4')],
      frames: [frame(), f1],
      human_moves: humanMoves(f1),
    }),
  );
  for (const viewport of [
    { width: 1100, height: 700 },
    { width: 1024, height: 768 },
  ]) {
    await page.setViewportSize(viewport);
    await page.goto('/#/game/uttt/ab12');
    await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
    const squares = await board(page).boundingBox();
    const { inner } = await pageHeights(page);
    expect(
      squares!.y + squares!.height,
      `${viewport.width}x${viewport.height}`,
    ).toBeLessThanOrEqual(inner);
  }
});

test('resigning takes a second click, and the question goes away', async ({ page }) => {
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const mock = await mockApi(
    page,
    session({
      to_act: [1],
      turns: [answer(0, '4 4')],
      frames: [frame(), f1],
      human_moves: humanMoves(f1),
    }),
  );
  mock.next = session({
    status: 'over',
    to_act: [],
    human_moves: [],
    result: { winner: 0, end: { kind: 'resigned', seat: 1 } },
  });
  await page.goto('/#/game/uttt/ab12');
  const resign = page.getByRole('button', { name: '⚑ Resign' });
  const confirm = page.getByRole('button', { name: 'Confirm resign' });

  await resign.click();
  await expect(confirm).toBeVisible();
  expect(mock.posts).toEqual([]);
  await page.keyboard.press('Escape');
  await expect(resign).toBeVisible();
  await expect(confirm).toHaveCount(0);

  await resign.click();
  await expect(confirm).toBeVisible();
  await page.clock.runFor(4_100);
  await expect(resign).toBeVisible();
  expect(mock.posts).toEqual([]);

  await resign.click();
  await confirm.click();
  await expect
    .poll(() => mock.posts)
    .toEqual([{ path: '/end', body: { seat: 1, reason: 'resign' } }]);
});

test('a new game asks first only while a game is in progress', async ({ page }) => {
  const f1 = frame({ marks: [[4, 4, 0]], last: [4, 4] });
  const running = session({
    to_act: [1],
    turns: [answer(0, '4 4')],
    frames: [frame(), f1],
    human_moves: humanMoves(f1),
  });
  const mock = await mockApi(page, running);
  const warning = 'Starting a new game ends the one in progress.';

  await page.goto('/#/game/uttt/ab12');
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  const dialog = page.getByRole('dialog', { name: 'Play with a friend' });
  await expect(dialog.getByText(warning)).toBeVisible();
  await expect(dialog.getByRole('button', { name: 'End this game and start' })).toBeVisible();
  // Closing the dialog keeps the game.
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  expect(mock.posts).toEqual([]);

  // A game with no move yet is not worth a warning.
  mock.current = session({});
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toHaveCount(0);
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  await expect(dialog.getByText(warning)).toHaveCount(0);
  await expect(dialog.getByRole('button', { name: 'Start', exact: true })).toBeVisible();
  await page.keyboard.press('Escape');

  // Nor is a game that is over.
  mock.current = session({
    status: 'over',
    to_act: [],
    turns: [answer(0, '4 4')],
    frames: [frame(), f1],
    human_moves: [],
    result: { winner: 0, end: { kind: 'resigned', seat: 1 } },
  });
  await expect(page.getByTestId('end-block')).toBeVisible();
  await page.getByRole('button', { name: 'New game', exact: true }).click();
  await expect(dialog.getByText(warning)).toHaveCount(0);
  await expect(dialog.getByRole('button', { name: 'Start', exact: true })).toBeVisible();

  // No game at all.
  await page.goto('/#/game/uttt');
  await page.keyboard.press('Escape');
  await page.getByRole('button', { name: /Play with a friend/ }).click();
  await expect(dialog.getByText(warning)).toHaveCount(0);
});

test('without a frame the game’s own empty board is drawn, and nothing can be played', async ({
  page,
}) => {
  const mock = await mockApi(page, session({}));
  await page.goto('/#/game/uttt');
  await expect(page.getByText('The board appears here.')).toHaveCount(0);
  const cells = page.getByRole('img', { name: /^row \d column \d$/ });
  await expect(cells).toHaveCount(81);
  await expect(page.getByRole('button', { name: /^play row/ })).toHaveCount(0);
  await cells.nth(40).click({ force: true });
  expect(mock.posts).toEqual([]);

  // A session still compiling has no frame either: the notice covers the empty board.
  await page.goto('/#/game/uttt/ab12');
  mock.current = session({
    seats: [human('You'), bot('uttt-v010')],
    status: 'compiling',
    to_act: [],
    frames: [],
    human_moves: [],
  });
  await page.reload();
  await expect(page.getByTestId('board-overlay')).toContainText('Compiling uttt-v010…');
  await expect(cells).toHaveCount(81);
  await expect(page.getByText('The board appears here.')).toHaveCount(0);
});

test('a game won with three small boards in a row has a dashed line through them', async ({
  page,
}) => {
  const won = (small: [number, number, 0 | 1][], winner: 0 | 1 | null) => [
    frame(),
    frame({ marks: [[4, 4, 0]], small, winner, points: [small.length, 0] }),
  ];
  const diagonal: [number, number, 0 | 1][] = [
    [0, 0, 1],
    [1, 1, 1],
    [2, 2, 1],
    [0, 2, 1],
  ];
  const mock = await mockApi(
    page,
    session({
      status: 'over',
      to_act: [],
      turns: [answer(0, '4 4')],
      frames: won(diagonal, 1),
      human_moves: [],
      result: { winner: 1, end: { kind: 'finished' } },
    }),
  );
  await page.goto('/#/game/uttt/ab12');
  const lines = page.getByTestId('win-lines');
  await expect(lines).toBeVisible();
  await expect(lines).toHaveAttribute('aria-hidden', 'true');
  // One line: an edge and a dash.
  await expect(lines.locator('line')).toHaveCount(2);
  const squares = await board(page).boundingBox();
  const drawn = await lines.boundingBox();
  expect(Math.abs(drawn!.x - squares!.x)).toBeLessThanOrEqual(1);
  expect(Math.abs(drawn!.width - squares!.width)).toBeLessThanOrEqual(1);
  // The line takes no click: the cell under it is still the target.
  await expect(lines).toHaveCSS('pointer-events', 'none');

  // A win on the count of small boards has no line, and neither has a game in progress, even
  // with three small boards in a row.
  mock.current = {
    ...mock.current,
    frames: won(
      [
        [0, 0, 1],
        [0, 1, 1],
        [2, 0, 1],
      ],
      1,
    ),
  };
  await page.reload();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  await expect(lines).toHaveCount(0);

  const inProgress = frame({ marks: [[4, 4, 0]], small: diagonal });
  mock.current = session({
    to_act: [1],
    turns: [answer(0, '4 4')],
    frames: [frame(), inProgress],
    human_moves: humanMoves(inProgress),
  });
  await page.reload();
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
  await expect(lines).toHaveCount(0);
});
