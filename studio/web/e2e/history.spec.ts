import { expect, test, type Page } from '@playwright/test';
import { mockGames, record, recordView } from './fixtures';

// The server is not run by this job: the API is mocked with page.route.

interface Entry {
  id: string;
  game: string;
  unix_time: number;
  players: [string, string];
  winner: number | null;
  end: string;
  turns: number;
  source: string;
}

const entry = (patch: Partial<Entry>): Entry => ({
  id: 'uttt-1760000000-aaaaaaaaaaaa',
  game: 'uttt',
  unix_time: 1_760_000_000,
  players: ['uttt-v010', 'uttt-v009'],
  winner: 0,
  end: 'finished',
  turns: 41,
  source: 'arena match',
  ...patch,
});

const ROWS: Entry[] = [
  entry({}),
  entry({
    id: 'uttt-1759990000-bbbbbbbbbbbb',
    unix_time: 1_759_990_000,
    players: ['You', 'uttt-v010'],
    winner: 1,
    turns: 30,
    source: 'studio',
  }),
  entry({
    id: 'uttt-1759980000-cccccccccccc',
    unix_time: 1_759_980_000,
    winner: null,
    turns: 81,
  }),
  entry({
    id: 'uttt-1759970000-dddddddddddd',
    unix_time: 1_759_970_000,
    winner: 0,
    end: 'timeout',
    turns: 12,
  }),
  entry({
    id: 'uttt-1759960000-eeeeeeeeeeee',
    unix_time: 1_759_960_000,
    winner: null,
    end: 'aborted',
    turns: 5,
    source: 'studio',
  }),
];

interface Mock {
  rows: Entry[];
  queries: string[];
  deleted: string[];
  posted: unknown[];
  /** What POST /api/history answers for a record, by its seed. */
  answer: (record: { seed: number }) => { status: number; json: unknown };
  viewed: unknown[];
}

async function mockHistory(page: Page, rows: Entry[] = ROWS): Promise<Mock> {
  const mock: Mock = {
    rows: [...rows],
    queries: [],
    deleted: [],
    posted: [],
    answer: () => ({ status: 200, json: { id: 'x', duplicate: false } }),
    viewed: [],
  };
  await mockGames(page);
  await page.route(
    (url) => url.pathname === '/api/history',
    async (route) => {
      const request = route.request();
      if (request.method() === 'POST') {
        const body = request.postDataJSON() as { seed: number };
        mock.posted.push(body);
        const { status, json } = mock.answer(body);
        await route.fulfill({ status, json });
      } else {
        mock.queries.push(new URL(request.url()).search);
        await route.fulfill({ json: mock.rows });
      }
    },
  );
  await page.route(/\/api\/history\/[a-z0-9-]+$/, async (route) => {
    const request = route.request();
    const id = new URL(request.url()).pathname.split('/').pop() ?? '';
    if (request.method() === 'DELETE') {
      mock.deleted.push(id);
      mock.rows = mock.rows.filter((row) => row.id !== id);
      await route.fulfill({ status: 204, body: '' });
    } else {
      await route.fulfill({ json: record({ seed: 4242 }) });
    }
  });
  await page.route(/\/api\/history\/[a-z0-9-]+\/view$/, (route) =>
    route.fulfill({ json: recordView() }),
  );
  await page.route('**/api/view', async (route) => {
    mock.viewed.push(route.request().postDataJSON());
    await route.fulfill({ json: recordView() });
  });
  return mock;
}

const rows = (page: Page) => page.getByTestId('history-row');

test('the table shows the saved games with their results and tags', async ({ page }) => {
  await mockHistory(page);
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(5);
  await expect(page.getByRole('link', { name: 'History' })).toHaveAttribute('aria-current', 'page');

  const first = rows(page).nth(0);
  await expect(first).toContainText('uttt-v010');
  await expect(first).toContainText('uttt-v009');
  await expect(first).toContainText('1–0');
  await expect(first).toContainText('41');
  await expect(first).toContainText('arena');
  await expect(first).toContainText(/\d{4}-\d{2}-\d{2} \d{2}:\d{2}/);

  await expect(rows(page).nth(1)).toContainText('0–1');
  await expect(rows(page).nth(1)).toContainText('studio');
  await expect(rows(page).nth(2)).toContainText('½–½');
  await expect(rows(page).nth(3)).toContainText('timeout');
  await expect(rows(page).nth(4)).toContainText('—');
  await expect(first.locator('.tag.keep')).toHaveCount(0);
});

test('the filters send their query, and an empty answer says nothing matches', async ({ page }) => {
  const mock = await mockHistory(page);
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(5);
  expect(mock.queries).toEqual(['']);

  await page.getByLabel('Release').selectOption('uttt-v009');
  await expect.poll(() => mock.queries.at(-1)).toBe('?release=uttt-v009');
  await page.getByLabel('Result').selectOption({ label: 'Faults' });
  await expect.poll(() => mock.queries.at(-1)).toBe('?release=uttt-v009&result=fault');
  await page.getByLabel('Source').selectOption({ label: 'Arena samples' });
  await page.getByLabel('From').fill('2026-10-01');
  await page.getByLabel('To').fill('2026-10-10');
  await expect
    .poll(() => mock.queries.at(-1))
    .toBe('?release=uttt-v009&result=fault&source=arena&from=2026-10-01&to=2026-10-10');

  mock.rows = [];
  await page.getByLabel('Result').selectOption({ label: 'X wins' });
  await expect.poll(() => mock.queries.at(-1)).toContain('result=x');
  await expect(page.getByTestId('history-empty')).toHaveText(
    'No saved game matches these filters.',
  );

  await page.getByLabel('Release').selectOption('');
  await page.getByLabel('Result').selectOption('');
  await page.getByLabel('Source').selectOption('');
  await page.getByLabel('From').fill('');
  await page.getByLabel('To').fill('');
  await expect.poll(() => mock.queries.at(-1)).toBe('');
  await expect(page.getByTestId('history-empty')).toContainText('No saved games yet');
});

test('the release filter offers the releases of the games', async ({ page }) => {
  await mockHistory(page);
  await page.goto('/#/history');
  await expect(page.getByLabel('Release').locator('option')).toHaveText([
    'All releases',
    'uttt-v010',
    'uttt-v009',
    'uttt-v001',
  ]);
});

test('Review opens the saved game', async ({ page }) => {
  await mockHistory(page);
  await page.goto('/#/history');
  await rows(page).nth(1).getByRole('link', { name: 'Review' }).click();
  await expect(page).toHaveURL(/#\/review\/saved\/uttt-1759990000-bbbbbbbbbbbb$/);
  await expect(page.getByRole('button', { name: 'First move' })).toBeVisible();
  await expect(page.getByTestId('result')).toBeVisible();
});

test('Export downloads the record', async ({ page }) => {
  await mockHistory(page);
  await page.goto('/#/history');
  const exported = page.waitForEvent('download');
  await rows(page).nth(2).getByRole('button', { name: 'Export' }).click();
  const download = await exported;
  expect(download.suggestedFilename()).toBe('uttt-1759980000-cccccccccccc.json');
  const path = await download.path();
  expect(path).not.toBeNull();
});

test('Delete asks again in the row, and only the second click deletes', async ({ page }) => {
  const mock = await mockHistory(page);
  let dialogs = 0;
  page.on('dialog', (dialog) => {
    dialogs += 1;
    void dialog.dismiss();
  });
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(5);

  const second = rows(page).nth(1);
  await second.getByRole('button', { name: 'Delete', exact: true }).click();
  await expect(second.getByRole('button', { name: 'Delete for good' })).toBeVisible();
  expect(mock.deleted).toEqual([]);

  // Cancel puts the row back.
  await second.getByRole('button', { name: 'Cancel' }).click();
  await expect(second.getByRole('button', { name: 'Delete', exact: true })).toBeVisible();
  await expect(second.getByRole('button', { name: 'Delete for good' })).toHaveCount(0);
  expect(mock.deleted).toEqual([]);

  await second.getByRole('button', { name: 'Delete', exact: true }).click();
  await second.getByRole('button', { name: 'Delete for good' }).click();
  await expect(rows(page)).toHaveCount(4);
  expect(mock.deleted).toEqual(['uttt-1759990000-bbbbbbbbbbbb']);
  await expect(page.getByText('uttt-1759990000')).toHaveCount(0);
  expect(dialogs).toBe(0);
});

test('loading three files shows what was imported, what was there, and what was refused', async ({
  page,
}) => {
  const mock = await mockHistory(page);
  mock.answer = ({ seed }) => {
    if (seed === 2) return { status: 200, json: { id: 'x', duplicate: true } };
    if (seed === 3) {
      return {
        status: 400,
        json: { error: 'invalid game record: the record does not replay: turn 4 is not legal' },
      };
    }
    return { status: 200, json: { id: 'x', duplicate: false } };
  };
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(5);
  await expect(page.getByText('Workflow artifacts come zipped: unzip them first.')).toBeVisible();

  const file = (name: string, seed: number) => ({
    name,
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify(record({ seed }))),
  });
  await page
    .getByLabel('Load game files')
    .setInputFiles([file('one.json', 1), file('two.json', 2), file('three.json', 3)]);

  await expect(
    page.getByRole('status').filter({ hasText: 'Imported 1, 1 already there, 1 refused' }),
  ).toBeVisible();
  const refused = page.getByRole('alert', { name: 'Refused files' });
  await expect(refused).toContainText('three.json');
  await expect(refused).toContainText('the record does not replay: turn 4 is not legal');
  await expect(refused).not.toContainText('one.json');
  expect(mock.posted).toHaveLength(3);
  // The list is read again.
  await expect.poll(() => mock.queries.length).toBeGreaterThan(1);
});

test('a file that is not JSON is refused without asking the server', async ({ page }) => {
  const mock = await mockHistory(page);
  await page.goto('/#/history');
  await expect(rows(page)).toHaveCount(5);
  await page.getByLabel('Load game files').setInputFiles({
    name: 'artifact.zip.json',
    mimeType: 'application/json',
    buffer: Buffer.from('PK\u0003\u0004 not json'),
  });
  await expect(
    page.getByRole('status').filter({ hasText: 'Imported 0, 0 already there, 1 refused' }),
  ).toBeVisible();
  await expect(page.getByRole('alert', { name: 'Refused files' })).toContainText(
    'artifact.zip.json: not a JSON file',
  );
  expect(mock.posted).toEqual([]);
});

test('a file can be reviewed without being saved, then saved from the review', async ({ page }) => {
  const mock = await mockHistory(page);
  await page.goto('/#/history');
  const loaded = record({ seed: 777 });
  await page.getByLabel('Review a file').setInputFiles({
    name: 'sample.json',
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify(loaded)),
  });
  await expect(page).toHaveURL(/#\/review\/file\/f\d+$/);
  await expect(page.getByText('sample.json')).toBeVisible();
  expect(mock.viewed).toEqual([loaded]);
  expect(mock.posted).toEqual([]);

  await page.getByRole('button', { name: 'Save to history' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Saved to your history' })).toBeVisible();
  expect(mock.posted).toEqual([loaded]);
  await expect(page.getByRole('button', { name: 'In your history' })).toBeDisabled();

  const exported = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export file' }).click();
  expect((await exported).suggestedFilename()).toBe('sample.json');
});
