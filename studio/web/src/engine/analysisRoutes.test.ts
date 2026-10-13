import { describe, expect, it } from 'vitest';
import { ANALYSIS_IDLE_MS, MAX_ANALYSES } from './analyses';
import { request, scriptedAnalysis, testEngine, until } from './testSupport';

const small = {
  game: 'uttt',
  moves: ['4 4'],
  iterations: 2000,
  report_every: 1000,
};

const STEP = JSON.stringify({ report: { iterations: 1000, done: false, stop: null } });
const LAST = JSON.stringify({ report: { iterations: 2000, done: true, stop: 'iterations' } });

interface View {
  status: string;
  report: { iterations: number; done: boolean; stop: string | null; moves: unknown[] } | null;
  error: string | null;
}

async function start(engine: Parameters<typeof request>[0], body: unknown = small) {
  const created = await request(engine, 'POST', '/api/analysis', body);
  expect(created.status).toBe(200);
  return (created.body as { id: string }).id;
}

async function view(engine: Parameters<typeof request>[0], id: string): Promise<View> {
  const got = await request(engine, 'GET', `/api/analysis/${id}`);
  expect(got.status).toBe(200);
  return got.body as View;
}

describe('the analysis routes', () => {
  it('says in the games list which games have an analysis program', async () => {
    const without = await testEngine({ analysis: false });
    const games = await request(without.engine, 'GET', '/api/games');
    expect((games.body as { analysis: boolean }[])[0]?.analysis).toBe(false);
  });

  it('refuses a body that is not a request with a 400', async () => {
    const { engine } = await testEngine();
    const refused = async (body: unknown, message: string) => {
      const answer = await request(engine, 'POST', '/api/analysis', body);
      expect(answer.status, JSON.stringify(body)).toBe(400);
      expect((answer.body as { error: string }).error).toBe(message);
    };
    const notObject = 'invalid request body: the request must be a JSON object';
    await refused([], notObject);
    await refused('3', notObject);
    await refused('null', notObject);
    await refused('"uttt"', notObject);
    const noGame = 'invalid request body: game must be a string';
    await refused({ moves: [] }, noGame);
    await refused({ game: 3, moves: [] }, noGame);
    await refused({ game: null }, noGame);
    await refused({ game: 'chess', moves: [] }, 'unknown game "chess"');
    const bad = await request(engine, 'POST', '/api/analysis', 'not json');
    expect(bad.status).toBe(400);
    expect((bad.body as { error: string }).error).toMatch(/^invalid request body: /);
    const empty = await request(engine, 'POST', '/api/analysis');
    expect(empty.status).toBe(400);
    expect(engine.analyses.ids()).toEqual([]);
  });

  it('refuses a game that has no analysis program', async () => {
    const { engine } = await testEngine({ analysis: false });
    const answer = await request(engine, 'POST', '/api/analysis', small);
    expect(answer.status).toBe(400);
    expect(answer.body).toEqual({ error: 'the game "uttt" has no analysis' });
  });

  it('answers 405 to a wrong method and 404 to a longer path', async () => {
    const { engine } = await testEngine();
    expect((await request(engine, 'PUT', '/api/analysis')).status).toBe(405);
    expect((await request(engine, 'GET', '/api/analysis')).status).toBe(405);
    expect((await request(engine, 'POST', '/api/analysis/abc')).status).toBe(405);
    expect((await request(engine, 'GET', '/api/analysis/a/b')).status).toBe(404);
  });

  it('runs the real program to a done report, and keeps it while it is read', async () => {
    const { engine, clock } = await testEngine();
    const id = await start(engine);
    expect(id).toMatch(/^s\d+$/);
    const done = await until(
      () => view(engine, id),
      (current) => current.status !== 'running',
    );
    expect(done.status).toBe('done');
    expect(done.error).toBeNull();
    expect(done.report?.iterations).toBe(2000);
    expect(done.report?.done).toBe(true);
    expect(done.report?.stop).toBe('iterations');
    expect(done.report?.moves).toHaveLength(8);
    for (let read = 0; read < 3; read++) {
      clock.now += ANALYSIS_IDLE_MS - 1000;
      expect((await view(engine, id)).status).toBe('done');
    }
  });

  it('gives the program its own errors through GET, as status error', async () => {
    const { engine } = await testEngine();
    const id = await start(engine, { game: 'uttt', moves: ['9 9'] });
    const failed = await until(
      () => view(engine, id),
      (current) => current.status !== 'running',
    );
    expect(failed).toEqual({
      status: 'error',
      report: null,
      error: 'move 1 ("9 9") is not legal',
    });
    const other = await start(engine, { game: 'uttt', moves: [], iterations: 0 });
    const invalid = await until(
      () => view(engine, other),
      (current) => current.status !== 'running',
    );
    expect(invalid.status).toBe('error');
    expect(invalid.error).toContain('iterations');
  });

  it('sends the program every field but game, unchanged', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    await start(engine, {
      game: 'uttt',
      moves: ['4 4', '4 3'],
      iterations: 500,
      seed: '9',
      extra: { a: [1, null] },
    });
    expect(JSON.parse(runs[0]?.request ?? '')).toEqual({
      moves: ['4 4', '4 3'],
      iterations: 500,
      seed: '9',
      extra: { a: [1, null] },
    });
  });

  it('is running with no report before the first, then follows the lines', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    const id = await start(engine);
    expect(await view(engine, id)).toEqual({ status: 'running', report: null, error: null });
    runs[0]?.line(STEP);
    const running = await view(engine, id);
    expect(running.status).toBe('running');
    expect(running.report?.iterations).toBe(1000);
    runs[0]?.line(LAST);
    const done = await view(engine, id);
    expect(done.status).toBe('done');
    expect(done.report?.iterations).toBe(2000);
    // Done: the worker is ended, and later lines change nothing.
    expect(runs[0]?.terminated).toBe(1);
    runs[0]?.line(STEP);
    expect((await view(engine, id)).report?.iterations).toBe(2000);
  });

  it('turns an error line, even the first, into status error', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    const first = await start(engine);
    runs[0]?.line('{"error":"bad moves"}');
    expect(await view(engine, first)).toEqual({
      status: 'error',
      report: null,
      error: 'bad moves',
    });
    expect(runs[0]?.terminated).toBe(1);
    const second = await start(engine);
    runs[1]?.line(STEP);
    runs[1]?.line('{"error":"late"}');
    const late = await view(engine, second);
    expect([late.status, late.error, late.report?.iterations]).toEqual(['error', 'late', 1000]);
    for (const [index, line] of ['nonsense', '[]', '{"other":1}', '{"report":3}'].entries()) {
      const id = await start(engine);
      runs[2 + index]?.line(line);
      expect(await view(engine, id), line).toEqual({
        status: 'error',
        report: null,
        error: 'the analysis program wrote an unexpected line',
      });
    }
  });

  it('says so when the program crashes or ends without a final report', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    const trapped = await start(engine);
    runs[0]?.line(STEP);
    runs[0]?.end({ error: 'unreachable' });
    const crashed = await view(engine, trapped);
    expect([crashed.status, crashed.error]).toEqual(['error', 'the analysis program crashed']);
    expect(crashed.report?.iterations).toBe(1000);
    const failed = await start(engine);
    runs[1]?.end({ exit: 1 });
    expect((await view(engine, failed)).error).toBe('the analysis program crashed');
    const early = await start(engine);
    runs[2]?.end({ exit: 0 });
    expect((await view(engine, early)).error).toBe(
      'the analysis program ended without a final report',
    );
  });

  it('knows no analysis by an id nobody gave, and a second delete is a 404', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    for (const id of ['zzz', 'a%2Fb', 'a b', '..', '%00', 'x'.repeat(5000)]) {
      for (const method of ['GET', 'DELETE']) {
        const answer = await request(engine, method, `/api/analysis/${id}`);
        expect([answer.status, answer.body], `${method} ${id}`).toEqual([
          404,
          { error: 'unknown analysis' },
        ]);
      }
    }
    const id = await start(engine);
    const deleted = await request(engine, 'DELETE', `/api/analysis/${id}`);
    expect([deleted.status, deleted.body]).toEqual([200, { deleted: true }]);
    expect(runs[0]?.terminated).toBe(1);
    const again = await request(engine, 'DELETE', `/api/analysis/${id}`);
    expect([again.status, again.body]).toEqual([404, { error: 'unknown analysis' }]);
    expect((await request(engine, 'GET', `/api/analysis/${id}`)).status).toBe(404);
    // Lines after the stop are ignored.
    runs[0]?.line(STEP);
    expect(engine.analyses.ids()).toEqual([]);
  });

  it('stops and removes the oldest when a fifth starts', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine } = await testEngine({ makeAnalysis: factory });
    const ids: string[] = [];
    for (let count = 0; count < MAX_ANALYSES + 1; count++) ids.push(await start(engine));
    expect(runs.map((run) => run.terminated)).toEqual([1, 0, 0, 0, 0]);
    expect((await request(engine, 'GET', `/api/analysis/${ids[0]}`)).status).toBe(404);
    for (const id of ids.slice(1)) {
      expect((await request(engine, 'GET', `/api/analysis/${id}`)).status).toBe(200);
    }
    expect(engine.analyses.ids()).toEqual(ids.slice(1));
  });

  it('stops and removes an analysis nobody has read for ten seconds', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine, clock } = await testEngine({ makeAnalysis: factory });
    const read = await start(engine);
    const unread = await start(engine);
    clock.now += ANALYSIS_IDLE_MS - 1;
    expect((await view(engine, read)).status).toBe('running');
    // Exactly ten seconds is not too long yet; one millisecond more is.
    clock.now += 1;
    expect(engine.analyses.ids()).toEqual([read, unread]);
    clock.now += 1;
    expect((await request(engine, 'GET', `/api/analysis/${unread}`)).status).toBe(404);
    expect(runs.map((run) => run.terminated)).toEqual([0, 1]);
    expect(await view(engine, read)).toMatchObject({ status: 'running' });
    // Any analysis request sweeps: a start removes what has been left unread, here the first.
    clock.now += ANALYSIS_IDLE_MS + 1;
    const fresh = await start(engine);
    expect(engine.analyses.ids()).toEqual([fresh]);
    expect(runs.map((run) => run.terminated)).toEqual([1, 1, 0]);
  });

  it('stops a running analysis that is not read when its next report arrives', async () => {
    const { factory, runs } = scriptedAnalysis();
    const { engine, clock } = await testEngine({ makeAnalysis: factory });
    const id = await start(engine);
    runs[0]?.line(STEP);
    clock.now += ANALYSIS_IDLE_MS + 1;
    runs[0]?.line(STEP);
    expect(runs[0]?.terminated).toBe(1);
    expect(engine.analyses.ids()).toEqual([]);
    expect((await request(engine, 'GET', `/api/analysis/${id}`)).status).toBe(404);
  });

  it('fails the analysis when the program cannot start', async () => {
    const { engine } = await testEngine({
      makeAnalysis: () => {
        throw new Error('no workers here');
      },
    });
    const id = await start(engine);
    expect(await view(engine, id)).toEqual({
      status: 'error',
      report: null,
      error: 'cannot start the analysis: no workers here',
    });
  });
});
