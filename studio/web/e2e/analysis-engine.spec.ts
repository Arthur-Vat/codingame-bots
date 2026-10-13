import { expect, test, type Locator, type Page } from '@playwright/test';
import { mockAnalysis, MOCK_LINES } from './analysisMock';
import { mockGames, pageHeights, record, recordView } from './fixtures';

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
