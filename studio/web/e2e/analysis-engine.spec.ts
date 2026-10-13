import { expect, test, type Locator, type Page } from '@playwright/test';
import { mockAnalysis, MOCK_LINES } from './analysisMock';
import { frame, mockGames, pageHeights, record, recordView } from './fixtures';

// The engine's panel, the bar and the chart on the analysis board, against the server's analysis
// routes mocked by `mockAnalysis` (the real program runs in browser-engine.spec.ts). The saved game
// has four moves, X first; the mocked program calls a position of four moves finished.

const SAVED = 'uttt-1760000000-abcdef123456';
const BOARD = `/#/game/uttt/analysis/saved/${SAVED}`;

async function open(
  page: Page,
  options: { analysis?: boolean; over?: boolean; chartReads?: number; failChart?: boolean } = {},
) {
  await mockGames(page, options.analysis ?? true);
  await page.route(`**/api/history/${SAVED}/view`, (route) =>
    route.fulfill({ json: recordView() }),
  );
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
  const mock = await mockAnalysis(page, {
    overAt: options.over === false ? undefined : 4,
    chance: (moves) => moves / 4,
    chartReads: options.chartReads,
    failChart: options.failChart,
  });
  await page.goto(BOARD);
  await expect(page.getByRole('button', { name: 'Next move' })).toBeVisible();
  return mock;
}

const panel = (page: Page) => page.getByTestId('engine-panel');
const lines = (page: Page) => panel(page).getByTestId('engine-line');
const bar = (page: Page) => page.getByTestId('evalbar');
const chart = (page: Page) => page.getByTestId('eval-chart');

/** The id the mocked server gave to its `n`th request (from 0). */
const idOf = (n: number) => `m${n + 1}`;
/** The part of a locator that is on screen: the Play tab stays in the page, hidden, behind the others. */
const shown = (locator: Locator) => locator.filter({ visible: true });

async function box(locator: Locator) {
  const found = await shown(locator).boundingBox();
  expect(found).not.toBeNull();
  return found!;
}

test('the panel, the bar and a line appear, named after the game’s latest release', async ({
  page,
}) => {
  const mock = await open(page);
  await expect(panel(page).getByText('uttt-v010 engine')).toBeVisible();
  await expect(panel(page).getByRole('checkbox', { name: 'uttt-v010 engine' })).toBeChecked();
  await expect(lines(page)).toHaveCount(1);
  await expect(lines(page).getByTestId('engine-chip')).toHaveText('X 58%');
  await expect(lines(page).getByTestId('engine-chip')).toHaveAttribute(
    'title',
    'X 58% · draw 2% · O 40% · 9,012 visits',
  );
  // The line is numbered from the start of the game: the first position, X to move.
  await expect(lines(page).getByTestId('engine-moves-0')).toHaveText('1.4 4 4 3 2.3 1 1 5');
  await expect(panel(page).getByTestId('engine-position')).toHaveText(
    'X to move · X 58% · draw 2% · O 40%',
  );
  await expect(panel(page).getByTestId('engine-stat')).toHaveText(/^\d\.\d s · [\d,]+ iterations$/);
  await expect(bar(page)).toHaveAttribute(
    'aria-label',
    'Win, draw and loss: X 58% · draw 2% · O 40%',
  );

  expect(mock.posts[0]).toEqual({
    game: 'uttt',
    moves: [],
    iterations: 100_000_000,
    report_every: 20_000,
    depth: 10,
    seed: '1',
  });

  // The next position: the old analysis is stopped, the new one asks for the moves so far, and the
  // line starts with O's move.
  await page.keyboard.press('ArrowRight');
  await expect(lines(page).getByTestId('engine-moves-0')).toHaveText('1…4 4 2.4 3 3 1 3.1 5');
  await expect(panel(page).getByTestId('engine-position')).toHaveText(
    'O to move · X 58% · draw 2% · O 40%',
  );
  expect(mock.deletes).toContain(idOf(0));
  expect(
    mock.posts.some((post) => post.iterations === 100_000_000 && post.moves.join() === '4 4'),
  ).toBe(true);
});

test('a game without analysis has no panel, bar, chart or heat map switch', async ({ page }) => {
  const mock = await open(page, { analysis: false });
  await expect(shown(page.getByTestId('player-0'))).toBeVisible();
  await expect(panel(page)).toHaveCount(0);
  await expect(bar(page)).toHaveCount(0);
  await expect(chart(page)).toHaveCount(0);
  await expect(page.getByLabel('Heatmap of best moves')).toHaveCount(0);
  expect(mock.posts).toEqual([]);
});

test('the engine thinks for 2 seconds, then stops and keeps what it found', async ({ page }) => {
  const mock = await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await expect(panel(page).getByRole('button', { name: 'Keep thinking' })).toBeVisible();
  await expect.poll(() => mock.deletes.includes(idOf(0)), { timeout: 8000 }).toBe(true);
  await expect(panel(page).getByTestId('engine-stat')).toHaveText(/^2\.\d s · /);
  // What was found stays, and nothing is read any more (a read already on its way may arrive).
  await expect(lines(page)).toHaveCount(1);
  await expect(bar(page)).toBeVisible();
  const reads = mock.reads.get(idOf(0));
  await page.waitForTimeout(500);
  expect(mock.reads.get(idOf(0))).toBe(reads);
  expect(mock.lateReads).toBeLessThanOrEqual(1);
});

test('Keep thinking goes on past 2 seconds until Stop', async ({ page }) => {
  const mock = await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await panel(page).getByRole('button', { name: 'Keep thinking' }).click();
  await expect(panel(page).getByRole('button', { name: 'Stop' })).toBeVisible();
  await expect(panel(page).getByTestId('engine-stat')).toHaveText(/^(2\.[5-9]|[3-9]\.\d) s · /, {
    timeout: 8000,
  });
  expect(mock.deletes).not.toContain(idOf(0));

  await panel(page).getByRole('button', { name: 'Stop' }).click();
  await expect(panel(page).getByRole('button', { name: 'Keep thinking' })).toBeVisible();
  await expect.poll(() => mock.deletes.includes(idOf(0))).toBe(true);
  await expect(lines(page)).toHaveCount(1);
});

test('a position left behind stops its analysis, and a new one starts on the new position', async ({
  page,
}) => {
  const mock = await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect.poll(() => mock.deletes.includes(idOf(0))).toBe(true);
  await expect
    .poll(() => mock.posts.some((post) => post.moves.length === 4 && post.iterations > 50_000))
    .toBe(true);
  await expect(lines(page)).toHaveCount(1);
});

test('three lines can be shown, and the choice is remembered', async ({ page }) => {
  await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await panel(page).getByRole('button', { name: 'Show 3 lines ▾' }).click();
  await expect(lines(page)).toHaveCount(3);
  await expect(panel(page).getByTestId('engine-chip')).toHaveText(['X 58%', 'X 55%', 'O 51%']);
  await expect(panel(page).getByTestId('engine-chip').nth(2)).toHaveClass(/\bo\b/);
  await expect(panel(page).getByRole('button', { name: 'Show 1 line ▴' })).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('studio.analysis.lines'))).toBe('3');

  await page.reload();
  await expect(lines(page)).toHaveCount(3);
  await panel(page).getByRole('button', { name: 'Show 1 line ▴' }).click();
  await expect(lines(page)).toHaveCount(1);
  expect(await page.evaluate(() => localStorage.getItem('studio.analysis.lines'))).toBe('1');
});

test('hovering a line outlines its first move on the board', async ({ page }) => {
  await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await expect(shown(page.locator('.uttt-cell.hl'))).toHaveCount(0);
  await lines(page).first().hover();
  await expect(shown(page.locator('.uttt-cell.hl'))).toHaveCount(1);
  await expect(shown(page.locator('.uttt-cell.hl'))).toHaveAttribute(
    'aria-label',
    /^row 4 column 4/,
  );
  await shown(page.getByTestId('player-0')).hover();
  await expect(shown(page.locator('.uttt-cell.hl'))).toHaveCount(0);
  expect(MOCK_LINES[0]?.move).toBe('4 4');
});

test('the switch turns the engine off: nothing is shown, nothing runs, and it stays off', async ({
  page,
}) => {
  const mock = await open(page, { over: false });
  await expect(lines(page)).toHaveCount(1);
  await expect(chart(page)).toBeVisible();
  await expect(page.getByLabel('Heatmap of best moves')).toBeEnabled();

  await panel(page).getByRole('checkbox', { name: 'uttt-v010 engine' }).uncheck();
  await expect(lines(page)).toHaveCount(0);
  await expect(bar(page)).toHaveCount(0);
  await expect(chart(page)).toHaveCount(0);
  await expect(panel(page).getByRole('button', { name: 'Keep thinking' })).toHaveCount(0);
  await expect(page.getByLabel('Heatmap of best moves')).toBeDisabled();
  await expect.poll(() => mock.live()).toEqual([]);
  expect(await page.evaluate(() => localStorage.getItem('studio.analysis.on'))).toBe('0');

  // Stepping asks nothing.
  const posts = mock.posts.length;
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('button', { name: '4 4' })).toHaveAttribute('aria-current', 'true');
  expect(mock.posts).toHaveLength(posts);

  await page.reload();
  await expect(panel(page).getByRole('checkbox', { name: 'uttt-v010 engine' })).not.toBeChecked();
  expect(mock.posts).toHaveLength(posts);

  await panel(page).getByRole('checkbox', { name: 'uttt-v010 engine' }).check();
  await expect(lines(page)).toHaveCount(1);
  expect(await page.evaluate(() => localStorage.getItem('studio.analysis.on'))).toBe('1');
});

test('the heat map switch is remembered, and needs the engine', async ({ page }) => {
  await open(page, { over: false });
  const heat = page.getByLabel('Heatmap of best moves');
  await expect(heat).not.toBeChecked();
  await heat.check();
  expect(await page.evaluate(() => localStorage.getItem('studio.analysis.heat'))).toBe('1');
  await page.reload();
  await expect(page.getByLabel('Heatmap of best moves')).toBeChecked();
});

test('a failed analysis shows the program’s message in the panel', async ({ page }) => {
  await mockGames(page, true);
  await page.route(`**/api/history/${SAVED}/view`, (route) =>
    route.fulfill({ json: recordView() }),
  );
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
  const mock = await mockAnalysis(page);
  mock.failWith('move 1 ("9 9") is not legal');
  await page.goto(BOARD);
  await expect(panel(page).getByRole('alert')).toHaveText('move 1 ("9 9") is not legal');
  await expect(lines(page)).toHaveCount(0);
});

test('a finished game shows its result, no lines, and the bar at the result', async ({ page }) => {
  await open(page);
  await page.keyboard.press('End');
  await expect(panel(page).getByTestId('engine-position')).toHaveText('Game over · O wins');
  await expect(lines(page)).toHaveCount(0);
  await expect(panel(page).getByRole('button', { name: /Show/ })).toHaveCount(0);
  await expect(panel(page).getByTestId('engine-stat')).toHaveCount(0);
  await expect(panel(page).getByRole('button', { name: 'Keep thinking' })).toBeDisabled();
  await expect(bar(page)).toHaveAttribute(
    'aria-label',
    'Win, draw and loss: X 0% · draw 0% · O 100%',
  );
});

test('the chart fills in the background, one analysis at a time, and jumps to a frame when clicked', async ({
  page,
}) => {
  const mock = await open(page);
  await expect(chart(page)).toBeVisible();
  await expect(chart(page).locator('svg')).toHaveAttribute(
    'aria-label',
    "Evaluation chart: X's win chance by move",
  );
  // No text on it.
  await expect(chart(page)).toHaveText('');
  await expect(page.getByTestId('chart-pending')).toHaveCount(0);

  const asked = mock.posts.filter((post) => post.iterations === 50_000);
  expect(asked.map((post) => post.moves.length)).toEqual([0, 1, 2, 3, 4]);
  expect(asked.every((post) => post.report_every === 50_000 && post.seed === '1')).toBe(true);

  // Clicking in the middle goes to the middle frame; the cursor line follows.
  await expect(page.getByTestId('chart-cursor')).toHaveAttribute('x1', '8');
  await chart(page).scrollIntoViewIfNeeded();
  const svg = await box(chart(page).locator('svg'));
  await page.mouse.click(svg.x + svg.width / 2, svg.y + svg.height / 2);
  await expect(page.getByRole('button', { name: '4 5' })).toHaveAttribute('aria-current', 'true');
  await expect(page.getByTestId('chart-cursor')).toHaveAttribute('x1', '500');
  await expect(page.getByTestId('chart-dot')).toHaveAttribute('d', /^M 500 /);

  // Returning to the game later computes nothing again.
  await page.goto('/#/game/uttt/history');
  await page.goto(BOARD);
  await expect(chart(page)).toBeVisible();
  await expect(page.getByTestId('chart-pending')).toHaveCount(0);
  expect(mock.posts.filter((post) => post.iterations === 50_000)).toHaveLength(5);
});

test('at most two analyses run at once, the live one and the chart, however fast the positions change', async ({
  page,
}) => {
  // Each position of the chart takes a few reads, so that it is still going while the keys are pressed.
  const mock = await open(page, { over: false, chartReads: 4 });
  await expect(page.getByTestId('chart-pending')).toHaveCount(1);
  for (let step = 0; step < 4; step++) await page.keyboard.press('ArrowRight');
  await expect(lines(page)).toHaveCount(1);
  await expect(page.getByTestId('chart-pending')).toHaveCount(0, { timeout: 15_000 });
  // Both ran together, and never more.
  expect(mock.maxLive).toBe(2);
});

test('a chart that fails twice says so in the panel, and the lines go on', async ({ page }) => {
  const mock = await open(page, { over: false, failChart: true });
  await expect(page.getByTestId('chart-problem')).toHaveText('Chart stopped: unknown analysis');
  await expect(lines(page)).toHaveCount(1);
  await expect(panel(page).getByRole('alert')).toHaveCount(0);
  // The first position was tried twice, and the chart gave up.
  expect(mock.posts.filter((post) => post.iterations === 50_000)).toHaveLength(2);
  // No text on the chart itself.
  await expect(chart(page)).toHaveText('');
  // The live analysis still follows the position.
  await page.keyboard.press('ArrowRight');
  await expect(lines(page).getByTestId('engine-moves-0')).toHaveText(/^1…4 4/);
});

test('the board, the bar, the panel and the chart are level with the squares', async ({ page }) => {
  await open(page, { over: false });
  await page.setViewportSize({ width: 1536, height: 730 });
  await expect(lines(page)).toHaveCount(1);
  await expect(chart(page)).toBeVisible();

  const squares = await box(page.locator('.uttt-board'));
  const bars = await box(bar(page));
  const engine = await box(panel(page));
  const table = await box(page.locator('.panel.table'));
  const side = await box(page.locator('.side-right'));
  const playback = await box(page.getByTestId('playback'));
  const charts = await box(chart(page));

  // The bar: right of the board, as tall as the squares and level with them, before the column.
  expect(bars.x).toBeGreaterThanOrEqual(squares.x + squares.width);
  expect(bars.x + bars.width).toBeLessThanOrEqual(side.x);
  expect(Math.abs(bars.y - squares.y)).toBeLessThanOrEqual(1);
  expect(Math.abs(bars.y + bars.height - (squares.y + squares.height))).toBeLessThanOrEqual(1);
  // The panel at the top of the column, the table down to the bottom of the squares.
  expect(Math.abs(engine.y - squares.y)).toBeLessThanOrEqual(1);
  expect(table.y).toBeGreaterThanOrEqual(engine.y + engine.height);
  expect(Math.abs(table.y + table.height - (squares.y + squares.height))).toBeLessThanOrEqual(1);
  // The window shows the board and its playback bar without scrolling; the chart is below.
  expect(squares.y + squares.height).toBeLessThanOrEqual(730);
  expect(playback.y + playback.height).toBeLessThanOrEqual(730);
  expect(charts.y).toBeGreaterThanOrEqual(playback.y + playback.height);
  const heights = await pageHeights(page);
  expect(heights.scroll).toBeGreaterThan(heights.inner);
  // The chart is exactly as wide as the squares, and aligned with them.
  expect(Math.abs(charts.x - squares.x)).toBeLessThanOrEqual(1);
  expect(Math.abs(charts.width - squares.width)).toBeLessThanOrEqual(1);

  // The page scrolls down to it.
  await chart(page).scrollIntoViewIfNeeded();
  expect(await page.evaluate<number>('window.scrollY')).toBeGreaterThan(0);
  const shownChart = await box(chart(page));
  expect(shownChart.y + shownChart.height).toBeLessThanOrEqual(730);
});

test('in a narrower window the bar is right of the board, the panel above the table, the chart under the board', async ({
  page,
}) => {
  await open(page, { over: false });
  for (const viewport of [
    { width: 1100, height: 700 },
    { width: 1024, height: 768 },
  ]) {
    await page.setViewportSize(viewport);
    await expect(lines(page)).toHaveCount(1);
    const name = `${viewport.width}x${viewport.height}`;
    const squares = await box(page.locator('.uttt-board'));
    const bars = await box(bar(page));
    const engine = await box(panel(page));
    const table = await box(page.locator('.panel.table'));
    const charts = await box(chart(page));
    const playback = await box(page.getByTestId('playback'));
    expect(squares.y + squares.height, name).toBeLessThanOrEqual(viewport.height);
    expect(bars.x, name).toBeGreaterThanOrEqual(squares.x + squares.width);
    expect(Math.abs(bars.height - squares.height), name).toBeLessThanOrEqual(1);
    expect(table.y, name).toBeGreaterThanOrEqual(engine.y + engine.height);
    expect(charts.y, name).toBeGreaterThanOrEqual(playback.y + playback.height);
    expect(Math.abs(charts.x - squares.x), name).toBeLessThanOrEqual(1);
    expect(Math.abs(charts.width - squares.width), name).toBeLessThanOrEqual(1);
  }
});

// Side lines (one at a time): the mocked rules answer `POST /api/view` with the frames of the path
// they are sent, every cell not yet marked being playable and four moves ending the game, as the
// mocked program says. The saved game's own frames get the same playable cells.

/** The frames of a path of moves from the empty board: the board, then one more mark per move. */
function pathFrames(moves: string[], endsAt: number) {
  const marked: [number, number, 0 | 1][] = [];
  const frames = [withPlayable(frame(), true)];
  moves.forEach((text, index) => {
    const [row, col] = text.split(' ').map(Number);
    marked.push([row!, col!, (index % 2) as 0 | 1]);
    const over = marked.length >= endsAt;
    frames.push(
      over ? withPlayable(frame(marked, 1, [1, 2]), false) : withPlayable(frame(marked), true),
    );
  });
  return frames;
}

/** A frame with the cells nobody marked as the ones to play (none when the game is over). */
function withPlayable<T extends { cells: (0 | 1 | null)[][] }>(f: T, live: boolean) {
  const playable: [number, number][] = [];
  if (live) {
    f.cells.forEach((cells, row) =>
      cells.forEach((cellMark, col) => {
        if (cellMark === null) playable.push([row, col]);
      }),
    );
  }
  return { ...f, playable };
}

interface MockedRules {
  /** The records posted to `/api/view`, in order. */
  records: { turns: { seat: number; lines: string[] }[][]; opening_plies: number }[];
  /** The next answers are refusals with this message (null: they are views again). */
  refuse: (message: string | null) => void;
  /** The next answer waits until `release` is called. */
  hold: () => () => void;
}

async function mockRules(page: Page, endsAt: number): Promise<MockedRules> {
  let refusal: string | null = null;
  let gate: Promise<void> | null = null;
  const mock: MockedRules = {
    records: [],
    refuse: (message) => {
      refusal = message;
    },
    hold: () => {
      let release = () => {};
      gate = new Promise<void>((resolve) => {
        release = () => {
          gate = null;
          resolve();
        };
      });
      return release;
    },
  };
  await page.route('**/api/view', async (route) => {
    const body = route.request().postDataJSON() as MockedRules['records'][number];
    mock.records.push(body);
    await gate;
    if (refusal !== null) {
      await route.fulfill({ status: 400, json: { error: refusal } });
      return;
    }
    const moves = body.turns.map((turn) => turn[0]?.lines.join('\n') ?? '');
    await route.fulfill({
      json: {
        record: body,
        frames: pathFrames(moves, endsAt),
        opening_turns: 0,
        shown_turns: moves.length,
      },
    });
  });
  return mock;
}

/** The saved game of four moves with playable cells on its frames, and the rules that replay a side line. */
async function openSide(
  page: Page,
  options: { analysis?: boolean; over?: boolean } = {},
): Promise<{ analysis: Awaited<ReturnType<typeof open>>; rules: MockedRules }> {
  const rules = await mockRules(page, options.over === false ? Infinity : 4);
  await mockGames(page, options.analysis ?? true);
  const view = recordView();
  view.frames = view.frames.map((f, index) => withPlayable(f as never, index < 4));
  await page.route(`**/api/history/${SAVED}/view`, (route) => route.fulfill({ json: view }));
  await page.route(`**/api/history/${SAVED}`, (route) => route.fulfill({ json: record() }));
  const analysis = await mockAnalysis(page, {
    overAt: options.over === false ? undefined : 4,
    chance: (moves) => moves / 4,
  });
  await page.goto(BOARD);
  await expect(page.getByRole('button', { name: 'Next move' })).toBeVisible();
  return { analysis, rules };
}

const sideLine = (page: Page) => page.getByTestId('side-line');
const sideMoves = (page: Page) => sideLine(page).locator('.sideline-moves button');
const playCell = (page: Page, row: number, col: number) =>
  shown(page.getByRole('button', { name: `play row ${row} column ${col}` }));
const mark = (page: Page, row: number, col: number, who: 'X' | 'O') =>
  shown(page.getByRole('img', { name: `row ${row} column ${col} ${who}` }));
const gameMoves = (page: Page) => page.getByLabel('Moves', { exact: true }).locator('button');

test('clicking an engine line enters a side line; the board, the bar and the panel follow it', async ({
  page,
}) => {
  const { analysis, rules } = await openSide(page);
  await panel(page).getByRole('button', { name: 'Show 3 lines ▾' }).click();
  await expect(lines(page)).toHaveCount(3);
  await expect(sideLine(page)).toHaveCount(0);

  // The second line: 2 2, 1 1, 0 0. The board shows the position after its first move.
  await lines(page).nth(1).click();
  await expect(sideMoves(page)).toHaveText(['2 2', '1 1', '0 0']);
  await expect(sideLine(page).locator('.n')).toHaveText(['1.', '2.']);
  await expect(sideMoves(page).first()).toHaveAttribute('aria-current', 'true');
  await expect(sideLine(page).getByRole('button', { name: 'Back to the game' })).toBeVisible();
  await expect(mark(page, 2, 2, 'X')).toBeVisible();
  await expect(mark(page, 1, 1, 'O')).toHaveCount(0);
  // The record the rules were asked to replay: no opening, the moves as ordinary answers, seats alternating.
  expect(rules.records).toHaveLength(1);
  expect(rules.records[0]?.opening_plies).toBe(0);
  expect(rules.records[0]?.turns).toMatchObject([
    [{ seat: 0, lines: ['2 2'] }],
    [{ seat: 1, lines: ['1 1'] }],
    [{ seat: 0, lines: ['0 0'] }],
  ]);
  // The engine follows: the position after 2 2, with O to move; the lines start with O's move.
  await expect
    .poll(() =>
      analysis.posts.some((post) => post.iterations > 50_000 && post.moves.join('|') === '2 2'),
    )
    .toBe(true);
  await expect(panel(page).getByTestId('engine-position')).toHaveText(
    'O to move · X 58% · draw 2% · O 40%',
  );
  await expect(lines(page).first().getByTestId('engine-moves-0')).toHaveText(
    '1…4 4 2.4 3 3 1 3.1 5',
  );
  // No move of the game is current, and the chart's marker stays on the frame it branches from.
  await expect(gameMoves(page).locator('[aria-current="true"]')).toHaveCount(0);
  await expect(page.getByTestId('chart-cursor')).toHaveAttribute('x1', '8');

  // → and ← step through the side line; Home and End go to its ends; ← at the start stays.
  await page.keyboard.press('ArrowRight');
  await expect(sideMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  await expect(mark(page, 1, 1, 'O')).toBeVisible();
  await expect
    .poll(() => analysis.posts.some((post) => post.moves.join('|') === '2 2|1 1'))
    .toBe(true);
  await page.keyboard.press('End');
  await expect(sideMoves(page).nth(2)).toHaveAttribute('aria-current', 'true');
  await expect(mark(page, 0, 0, 'X')).toBeVisible();
  await page.keyboard.press('Home');
  await expect(sideMoves(page).locator('[aria-current="true"]')).toHaveCount(0);
  await expect(mark(page, 2, 2, 'X')).toHaveCount(0);
  await page.keyboard.press('ArrowLeft');
  await expect(sideLine(page)).toBeVisible();
  await expect(mark(page, 2, 2, 'X')).toHaveCount(0);

  // The strip's moves are buttons, and so are the playback bar's.
  await sideMoves(page).nth(1).click();
  await expect(sideMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  await expect(mark(page, 1, 1, 'O')).toBeVisible();
  await page.getByRole('button', { name: 'Last move' }).click();
  await expect(sideMoves(page).nth(2)).toHaveAttribute('aria-current', 'true');
  await page.getByRole('button', { name: 'Previous move' }).click();
  await expect(sideMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  await page.getByRole('button', { name: 'First move' }).click();
  await expect(sideMoves(page).locator('[aria-current="true"]')).toHaveCount(0);
  await page.getByRole('button', { name: 'Next move' }).click();
  await expect(sideMoves(page).first()).toHaveAttribute('aria-current', 'true');
  // Nothing plays along in a side line.
  await expect(page.getByRole('button', { name: 'Play', exact: true })).toBeDisabled();
  // (Space on a focused button is the button's own, so the focus is taken off the bar first.)
  await page.evaluate('document.activeElement?.blur()');
  await page.keyboard.press('Space');
  await expect(sideMoves(page).first()).toHaveAttribute('aria-current', 'true');
  await expect(page.getByTestId('playback')).toBeVisible();
  expect(rules.records).toHaveLength(1);
});

test('a move on the board enters a side line and extends it; a move it has already is a step', async ({
  page,
}) => {
  const { analysis, rules } = await openSide(page);
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  await expect(gameMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');

  await playCell(page, 0, 0).click();
  await expect(sideMoves(page)).toHaveText(['0 0']);
  // From frame 2, X to move: `2.`; the marker stays at frame 2 and no move of the game is current.
  await expect(sideLine(page).locator('.n')).toHaveText(['2.']);
  await expect(mark(page, 0, 0, 'X')).toBeVisible();
  await expect(page.getByTestId('chart-cursor')).toHaveAttribute('x1', '500');
  await expect(gameMoves(page).locator('[aria-current="true"]')).toHaveCount(0);
  await expect
    .poll(() =>
      analysis.posts.some(
        (post) => post.iterations > 50_000 && post.moves.join('|') === '4 4|4 5|0 0',
      ),
    )
    .toBe(true);

  // Playing inside it extends it.
  await playCell(page, 1, 1).click();
  await expect(sideMoves(page)).toHaveText(['0 0', '1 1']);
  await expect(sideMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  expect(rules.records).toHaveLength(2);
  expect(rules.records[1]?.turns.map((turn) => turn[0]?.lines[0])).toEqual([
    '4 4',
    '4 5',
    '0 0',
    '1 1',
  ]);

  // A different move from the middle drops what followed.
  await page.keyboard.press('ArrowLeft');
  await playCell(page, 8, 8).click();
  await expect(sideMoves(page)).toHaveText(['0 0', '8 8']);
  await expect(mark(page, 1, 1, 'O')).toHaveCount(0);
  expect(rules.records).toHaveLength(3);

  // The move it has already is a step: no new replay.
  await page.keyboard.press('Home');
  await expect(sideMoves(page).locator('[aria-current="true"]')).toHaveCount(0);
  await playCell(page, 0, 0).click();
  await expect(sideMoves(page).first()).toHaveAttribute('aria-current', 'true');
  await expect(sideMoves(page)).toHaveText(['0 0', '8 8']);
  expect(rules.records).toHaveLength(3);
});

test('the game’s own next move, from the board or from a line, steps without a side line', async ({
  page,
}) => {
  const { rules } = await openSide(page);
  await playCell(page, 4, 4).click();
  await expect(gameMoves(page).nth(0)).toHaveAttribute('aria-current', 'true');
  await expect(sideLine(page)).toHaveCount(0);
  await expect(mark(page, 4, 4, 'X')).toBeVisible();

  // On the next frame the first line (4 4) is no longer the game's move; the panel offers 4 4 again
  // at frame 0, and a line whose first move is the game's next move steps one move.
  await page.keyboard.press('Home');
  await expect(lines(page)).toHaveCount(1);
  await lines(page).first().click();
  await expect(gameMoves(page).nth(0)).toHaveAttribute('aria-current', 'true');
  await expect(sideLine(page)).toHaveCount(0);
  // The line now starts at O's move.
  await expect(lines(page).first().getByTestId('engine-moves-0')).toHaveText(
    '1…4 4 2.4 3 3 1 3.1 5',
  );
  expect(rules.records).toEqual([]);
});

test('Back to the game, Escape, a move of the list and the chart leave the side line', async ({
  page,
}) => {
  const { rules } = await openSide(page);
  await expect(chart(page)).toBeVisible();
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');

  // Back to the game: frame 2, where it branched.
  await playCell(page, 0, 0).click();
  await expect(sideLine(page)).toBeVisible();
  await sideLine(page).getByRole('button', { name: 'Back to the game' }).click();
  await expect(sideLine(page)).toHaveCount(0);
  await expect(gameMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  await expect(mark(page, 0, 0, 'X')).toHaveCount(0);

  // Escape.
  await playCell(page, 0, 0).click();
  await expect(sideLine(page)).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(sideLine(page)).toHaveCount(0);
  await expect(gameMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');

  // A move of the list shows its position.
  await playCell(page, 0, 0).click();
  await expect(sideLine(page)).toBeVisible();
  await gameMoves(page).nth(3).click();
  await expect(sideLine(page)).toHaveCount(0);
  await expect(gameMoves(page).nth(3)).toHaveAttribute('aria-current', 'true');

  // The chart: a click on it goes to the frame it names.
  await page.keyboard.press('Home');
  await playCell(page, 0, 0).click();
  await expect(sideLine(page)).toBeVisible();
  await chart(page).scrollIntoViewIfNeeded();
  const plot = await box(chart(page).locator('svg'));
  await page.mouse.click(plot.x + plot.width * 0.75, plot.y + plot.height / 2);
  await expect(sideLine(page)).toHaveCount(0);
  await expect(gameMoves(page).nth(2)).toHaveAttribute('aria-current', 'true');
  expect(rules.records).toHaveLength(4);

  // Nothing is left: the keys are the game's again.
  await page.keyboard.press('ArrowLeft');
  await expect(gameMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
  await page.keyboard.press('Escape');
  await expect(gameMoves(page).nth(1)).toHaveAttribute('aria-current', 'true');
});

test('Escape leaves a dialog alone, and the side line with it', async ({ page }) => {
  await openSide(page);
  await playCell(page, 0, 0).click();
  await expect(sideLine(page)).toBeVisible();
  await page.evaluate(
    'document.body.insertAdjacentHTML(\'beforeend\', \'<div role="dialog" id="fake-dialog"></div>\')',
  );
  await page.keyboard.press('Escape');
  await expect(sideLine(page)).toBeVisible();
  await page.evaluate("document.getElementById('fake-dialog')?.remove()");
  await page.keyboard.press('Escape');
  await expect(sideLine(page)).toHaveCount(0);
});

test('a side line can be played without the engine: switched off, or a game without analysis', async ({
  page,
}) => {
  const { analysis, rules } = await openSide(page, { analysis: false });
  await expect(panel(page)).toHaveCount(0);
  await playCell(page, 4, 4).click();
  await expect(gameMoves(page).nth(0)).toHaveAttribute('aria-current', 'true');
  await playCell(page, 0, 0).click();
  await expect(sideMoves(page)).toHaveText(['0 0']);
  await playCell(page, 1, 1).click();
  await expect(sideMoves(page)).toHaveText(['0 0', '1 1']);
  await page.keyboard.press('ArrowLeft');
  await expect(sideMoves(page).first()).toHaveAttribute('aria-current', 'true');
  await page.keyboard.press('Escape');
  await expect(sideLine(page)).toHaveCount(0);
  expect(rules.records).toHaveLength(2);
  expect(analysis.posts).toEqual([]);
});

test('with the engine switched off a move on the board still makes a side line', async ({
  page,
}) => {
  const { analysis } = await openSide(page);
  await expect(lines(page)).toHaveCount(1);
  await panel(page).getByRole('checkbox', { name: 'uttt-v010 engine' }).uncheck();
  await expect(lines(page)).toHaveCount(0);
  const posts = analysis.posts.length;
  await playCell(page, 0, 0).click();
  await expect(sideMoves(page)).toHaveText(['0 0']);
  await expect(lines(page)).toHaveCount(0);
  expect(analysis.posts).toHaveLength(posts);
});

test('a refused side line says why and leaves the board as it was', async ({ page }) => {
  const { rules } = await openSide(page);
  rules.refuse('the record does not replay: turn 0 (seat 0) is invalid');
  await playCell(page, 0, 0).click();
  await expect(page.getByRole('status')).toHaveText(
    'the record does not replay: turn 0 (seat 0) is invalid',
  );
  await expect(sideLine(page)).toHaveCount(0);
  await expect(mark(page, 0, 0, 'X')).toHaveCount(0);
  // The board can be played again.
  rules.refuse(null);
  await playCell(page, 0, 0).click();
  await expect(sideMoves(page)).toHaveText(['0 0']);
});

test('while a side line is on its way the board and the lines are not clickable, and leaving drops its answer', async ({
  page,
}) => {
  const { rules } = await openSide(page);
  await expect(lines(page)).toHaveCount(1);
  const release = rules.hold();
  await playCell(page, 0, 0).click();
  await expect(playCell(page, 1, 1)).toHaveCount(0);
  await expect(lines(page).locator('button')).toBeDisabled();
  await lines(page).locator('button').click({ force: true });
  expect(rules.records).toHaveLength(1);

  // Leaving before the answer: the answer comes and is ignored.
  await page.keyboard.press('Escape');
  release();
  await expect(playCell(page, 1, 1)).toBeVisible();
  await page.waitForTimeout(200);
  await expect(sideLine(page)).toHaveCount(0);
  await expect(mark(page, 0, 0, 'X')).toHaveCount(0);
  expect(rules.records).toHaveLength(1);
});

test('a side line that ends the game shows the engine’s result, not the game’s', async ({
  page,
}) => {
  await openSide(page);
  const result = page.getByTestId('result');
  const before = await shown(result).textContent();
  for (const [row, col] of [
    [0, 0],
    [1, 1],
    [2, 2],
    [3, 3],
  ]) {
    await playCell(page, row!, col!).click();
  }
  await expect(sideMoves(page)).toHaveCount(4);
  await expect(panel(page).getByTestId('engine-position')).toHaveText('Game over · O wins');
  await expect(lines(page)).toHaveCount(0);
  // Nothing is left to play.
  await expect(shown(page.getByRole('button', { name: /^play row/ }))).toHaveCount(0);
  expect(await shown(result).textContent()).toBe(before);
  // The line of the game stays as it was.
  await page.keyboard.press('ArrowLeft');
  await expect(playCell(page, 3, 3)).toBeVisible();
});

test('a long side line wraps in its strip: the board keeps its place and the page does not grow', async ({
  page,
}) => {
  await openSide(page, { over: false });
  await page.setViewportSize({ width: 1536, height: 730 });
  await expect(lines(page)).toHaveCount(1);
  const squares = await box(page.locator('.uttt-board'));
  const table = await box(page.locator('.panel.table'));
  const heights = await pageHeights(page);

  // Twenty-four moves on the first rows of the board.
  for (let n = 0; n < 24; n++) {
    await playCell(page, Math.floor(n / 9), n % 9).click();
    await expect(sideMoves(page)).toHaveCount(n + 1);
  }
  await expect(sideMoves(page).nth(23)).toHaveAttribute('aria-current', 'true');

  const after = await box(page.locator('.uttt-board'));
  expect(after).toEqual(squares);
  const tableAfter = await box(page.locator('.panel.table'));
  expect(tableAfter).toEqual(table);
  expect(await pageHeights(page)).toEqual(heights);
  // The strip has a room of its own, with its own scroll; the move list keeps some.
  const strip = await box(sideLine(page));
  expect(strip.height).toBeLessThanOrEqual(120);
  expect(strip.y + strip.height).toBeLessThanOrEqual(table.y + table.height);
  const list = await box(page.getByLabel('Moves', { exact: true }));
  expect(list.height).toBeGreaterThan(40);
  const scrolls = await shown(sideLine(page).locator('.sideline-moves')).evaluate(
    (element) => element.scrollHeight > element.clientHeight,
  );
  expect(scrolls).toBe(true);
  await sideLine(page).getByRole('button', { name: 'Back to the game' }).click();
  await expect(sideLine(page)).toHaveCount(0);
  expect(await box(page.locator('.panel.table'))).toEqual(table);
});
