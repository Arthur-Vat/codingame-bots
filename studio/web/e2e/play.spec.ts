import { expect, test, type Page } from '@playwright/test';

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
}

function frame({
  marks = [],
  playable = 'all',
  last = null,
  points = [0, 0],
  winner,
}: FrameOptions = {}) {
  const cells: Mark[][] = Array.from({ length: 9 }, () => Array<Mark>(9).fill(null));
  for (const [row, col, seat] of marks) cells[row]![col] = seat;
  const all: [number, number][] = [];
  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++) if (cells[r]![c] === null) all.push([r, c]);
  const list = playable === 'all' ? all : playable;
  return {
    cells,
    small: Array.from({ length: 3 }, () => Array<Mark>(3).fill(null)),
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

test('bot against bot shows a compact line over the board, then the end block', async ({
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
  await expect(page.getByTestId('board-overlay')).toHaveText('uttt-v010 vs uttt-v009 · move 2');
  await expect(page.getByRole('img', { name: 'row 4 column 4 X' })).toBeVisible();
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
  await expect(page.getByTestId('result')).toContainText('0–1 · O wins');
  await expect(page.getByTestId('result')).toContainText('Small boards won: 2–3');
  await expect(page.getByTestId('board-overlay')).toHaveCount(0);
  await expect(page.getByTestId('clock-1')).toHaveText('0.0 s');
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

  await page.clock.runFor(31_000);
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
