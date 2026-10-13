import { describe, expect, it } from 'vitest';
import { createEngine, validHistoryId, type Engine } from './routes';
import { createRules } from './rules';
import { Speed, memorySpeedCache } from './speed';
import { BOTS_UNAVAILABLE } from './bot';
import { STORE_UNAVAILABLE, memoryGameStore } from './storage';
import { fakeWasm, inProcessAnalysis, request, rulesModule, testEngine } from './testSupport';

const friends = {
  game: 'uttt',
  seed: 7,
  opening_plies: 0,
  seats: [
    { kind: 'human', name: 'Ann' },
    { kind: 'human', name: 'Bob' },
  ],
};

interface View {
  id: string;
  status: string;
  to_act: number[];
  turns: unknown[][];
  frames: unknown[];
  human_moves: { lines: string[] }[];
  result: { winner: number | null; end: { kind: string } } | null;
  error: string | null;
  seats: { kind: string; name: string }[];
  progress: unknown;
  opening_turns: number;
}

async function start(engine: Engine, body: unknown = friends): Promise<string> {
  const created = await request(engine, 'POST', '/api/sessions', body);
  expect(created.status).toBe(200);
  return (created.body as { id: string }).id;
}

async function view(engine: Engine, id: string): Promise<View> {
  const got = await request(engine, 'GET', `/api/sessions/${id}`);
  expect(got.status).toBe(200);
  return got.body as View;
}

async function move(engine: Engine, id: string, seat: number, index: number) {
  return request(engine, 'POST', `/api/sessions/${id}/move`, { seat, index });
}

/** Plays the first legal move for whoever is to act, until the game is over. */
async function playOut(engine: Engine, id: string): Promise<View> {
  for (let guard = 0; guard < 100; guard++) {
    const current = await view(engine, id);
    if (current.status === 'over') return current;
    const answer = await move(engine, id, current.to_act[0] ?? 0, 0);
    expect(answer.status).toBe(200);
  }
  throw new Error('the game did not end');
}

describe('the routes', () => {
  it('answers 404 for a path that is not a route and 405 for a wrong method', async () => {
    const { engine } = await testEngine();
    expect((await request(engine, 'GET', '/api/nope')).status).toBe(404);
    expect((await request(engine, 'GET', '/api')).status).toBe(404);
    expect((await request(engine, 'GET', '/index.html')).status).toBe(404);
    expect((await request(engine, 'GET', '/api/sessions/a/b/c')).status).toBe(404);
    const wrong = await request(engine, 'PUT', '/api/games');
    expect(wrong.status).toBe(405);
    expect(wrong.body).toEqual({ error: 'method not allowed' });
    expect((await request(engine, 'GET', '/api/sessions/x/move')).status).toBe(405);
    expect((await request(engine, 'GET', '/api/sessions')).status).toBe(405);
  });

  it('lists the games with their releases, newest first, as the manifest has them', async () => {
    const { engine } = await testEngine({ releases: ['uttt-v002', 'uttt-v001'] });
    const games = await request(engine, 'GET', '/api/games');
    expect(games.status).toBe(200);
    expect(games.body).toEqual([
      {
        id: 'uttt',
        name: 'Ultimate Tic-Tac-Toe',
        releases: ['uttt-v002', 'uttt-v001'],
        analysis: true,
      },
    ]);
  });

  it('gives a game that is not in the manifest no releases', async () => {
    const { engine } = await testEngine({ releases: [] });
    const games = await request(engine, 'GET', '/api/games');
    expect(games.body).toEqual([
      { id: 'uttt', name: 'Ultimate Tic-Tac-Toe', releases: [], analysis: true },
    ]);
  });
});

describe('when the WebAssembly files are missing', () => {
  it('answers 500 saying which file and how to build them, and recovers when it is there', async () => {
    let available = false;
    const rules = createRules(async () => {
      if (!available) throw new Error('cannot load wasm/rules.wasm (HTTP 404)');
      return rulesModule();
    });
    const engine = createEngine({
      rules,
      wasm: fakeWasm(['uttt-v001']),
      speed: new Speed(memorySpeedCache()),
      makeBot: null,
      now: () => 0,
      newId: () => 'a',
      makeAnalysis: inProcessAnalysis,
      store: async () => memoryGameStore(),
    });
    const failed = await request(engine, 'GET', '/api/games');
    expect(failed.status).toBe(500);
    expect((failed.body as { error: string }).error).toBe(
      'cannot load the rules program: cannot load wasm/rules.wasm (HTTP 404) (run scripts/build-studio-wasm.sh to build them)',
    );
    available = true;
    expect((await request(engine, 'GET', '/api/games')).status).toBe(200);
  });
});

describe('starting a game', () => {
  it('refuses what the server refuses, with a 400 and the rules own message', async () => {
    const { engine } = await testEngine({ makeBot: null });
    const refused = async (body: unknown, message: string) => {
      const answer = await request(engine, 'POST', '/api/sessions', body);
      expect(answer.status).toBe(400);
      expect((answer.body as { error: string }).error).toContain(message);
    };
    await refused({ ...friends, game: 'chess' }, 'unknown game "chess"');
    await refused({ ...friends, seed: undefined }, 'seed is required');
    await refused({ ...friends, opening_plies: 3 }, 'not supported');
    await refused({ ...friends, seats: [friends.seats[0]] }, 'exactly 2 seats');
    await refused(
      {
        ...friends,
        seats: [
          friends.seats[0],
          { kind: 'bot', release: 'uttt-v999', think_ms: 100, mode: 'fixed' },
        ],
      },
      'unknown release "uttt-v999"',
    );
    await refused(
      {
        ...friends,
        seats: [
          friends.seats[0],
          { kind: 'bot', release: 'uttt-v001', think_ms: 5, mode: 'fixed' },
        ],
      },
      'think_ms must be a number from 10 to 10000',
    );
    await refused(
      {
        ...friends,
        seats: [friends.seats[0], { kind: 'bot', release: 'uttt-v001', think_ms: 50 }],
      },
      'mode must be',
    );
  });

  it('refuses a body that is not JSON with a 400', async () => {
    const { engine } = await testEngine();
    const answer = await request(engine, 'POST', '/api/sessions', '{not json');
    expect(answer.status).toBe(400);
    expect((answer.body as { error: string }).error).toMatch(/^invalid request body: /);
  });

  it('refuses a seat with a bot where bots cannot run, with a 409, and still plays friends', async () => {
    const { engine } = await testEngine({ makeBot: null });
    const withBot = {
      ...friends,
      seats: [friends.seats[0], { kind: 'bot', release: 'uttt-v001', think_ms: 50, mode: 'fixed' }],
    };
    const refused = await request(engine, 'POST', '/api/sessions', withBot);
    expect(refused.status).toBe(409);
    expect(refused.body).toEqual({ error: BOTS_UNAVAILABLE });
    expect(BOTS_UNAVAILABLE).toBe(
      'This browser does not let bots run on this page (cross-origin isolation is unavailable)',
    );
    expect(engine.sessions.size).toBe(0);
    const id = await start(engine);
    expect((await view(engine, id)).status).toBe('waiting_human');
  });

  it('starts a game between humans at once, waiting for the first', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    expect(id).toMatch(/^s[0-9a-f]+$/);
    const current = await view(engine, id);
    expect(current).toMatchObject({
      id,
      game: 'uttt',
      seed: 7,
      opening_plies: 0,
      status: 'waiting_human',
      error: null,
      to_act: [0],
      opening_turns: 0,
      turns: [],
      result: null,
      progress: null,
    });
    expect(current.seats).toEqual([
      { kind: 'human', name: 'Ann' },
      { kind: 'human', name: 'Bob' },
    ]);
    expect(current.frames).toHaveLength(1);
    expect(current.human_moves).toHaveLength(81);
  });

  it('names an unnamed human Player 1 and Player 2, as the server does', async () => {
    const { engine } = await testEngine();
    const id = await start(engine, {
      ...friends,
      seats: [{ kind: 'human' }, { kind: 'human', name: '   ' }],
    });
    expect((await view(engine, id)).seats.map((seat) => seat.name)).toEqual([
      'Player 1',
      'Player 2',
    ]);
  });
});

describe('a game between friends', () => {
  it('plays to the end by the rules and shows the result', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    const over = await playOut(engine, id);
    expect(over.result?.end.kind).toBe('finished');
    expect(over.to_act).toEqual([]);
    expect(over.human_moves).toEqual([]);
    expect(over.frames).toHaveLength(over.turns.length + 1);
  });

  it('refuses a move out of turn, a wrong index and a move after the end, with a 409', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    const outOfTurn = await move(engine, id, 1, 0);
    expect(outOfTurn.status).toBe(409);
    expect(outOfTurn.body).toEqual({ error: "it is not that human's turn" });
    const noSeat = await move(engine, id, 5, 0);
    expect(noSeat.status).toBe(409);
    const index = await move(engine, id, 0, 999);
    expect(index.status).toBe(409);
    expect(index.body).toEqual({ error: 'index 999 is not one of the 81 moves' });
    await playOut(engine, id);
    const late = await move(engine, id, 0, 0);
    expect(late.status).toBe(409);
    expect(late.body).toEqual({ error: 'not waiting for a move (status over)' });
  });

  it('refuses a body that is not a seat and an index with a 400', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    expect((await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0 })).status).toBe(
      400,
    );
    expect(
      (await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: -1, index: 0 })).status,
    ).toBe(400);
    expect(
      (await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 1.5 })).status,
    ).toBe(400);
    expect((await request(engine, 'POST', `/api/sessions/${id}/move`, 'x')).status).toBe(400);
  });

  it('knows nothing of a session that does not exist, with a 404', async () => {
    const { engine } = await testEngine();
    for (const [method, path] of [
      ['GET', '/api/sessions/nope'],
      ['DELETE', '/api/sessions/nope'],
      ['GET', '/api/sessions/nope/record'],
      ['POST', '/api/sessions/nope/save'],
    ] as const) {
      const answer = await request(engine, method, path, method === 'POST' ? {} : undefined);
      expect(answer.status).toBe(404);
      expect(answer.body).toEqual({ error: 'unknown session' });
    }
    expect(
      (await request(engine, 'POST', '/api/sessions/nope/move', { seat: 0, index: 0 })).status,
    ).toBe(404);
  });

  it('takes moves back to any number of turns from the opening to the last valid one', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    for (let turn = 0; turn < 4; turn++) {
      const current = await view(engine, id);
      await move(engine, id, current.to_act[0] ?? 0, 0);
    }
    const back = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 1 });
    expect(back.status).toBe(200);
    const after = back.body as View;
    expect(after.turns).toHaveLength(1);
    expect(after.status).toBe('waiting_human');
    expect(after.to_act).toEqual([1]);
    expect(after.frames).toHaveLength(2);
    const tooFar = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 2 });
    expect(tooFar.status).toBe(409);
    expect(tooFar.body).toEqual({ error: 'turns must be from 0 to 1, got 2' });
    const toStart = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 0 });
    expect((toStart.body as View).turns).toHaveLength(0);
  });

  it('takes back a finished game and reopens it', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    const over = await playOut(engine, id);
    const back = await request(engine, 'POST', `/api/sessions/${id}/takeback`, {
      turns: over.turns.length - 2,
    });
    const reopened = back.body as View;
    expect(reopened.status).toBe('waiting_human');
    expect(reopened.result).toBeNull();
  });

  it('ends by resignation or timeout, humans only, once', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    expect(
      (await request(engine, 'POST', `/api/sessions/${id}/end`, { seat: 2, reason: 'resign' }))
        .status,
    ).toBe(400);
    const badReason = await request(engine, 'POST', `/api/sessions/${id}/end`, {
      seat: 0,
      reason: 'quit',
    });
    expect(badReason.status).toBe(400);
    expect(badReason.body).toEqual({ error: 'reason must be "resign" or "timeout"' });
    const resigned = await request(engine, 'POST', `/api/sessions/${id}/end`, {
      seat: 0,
      reason: 'resign',
    });
    expect(resigned.status).toBe(200);
    expect((resigned.body as View).result).toEqual({
      winner: 1,
      end: { kind: 'resigned', seat: 0 },
    });
    expect((resigned.body as View).status).toBe('over');
    const again = await request(engine, 'POST', `/api/sessions/${id}/end`, {
      seat: 1,
      reason: 'resign',
    });
    expect(again.status).toBe(409);
    expect(again.body).toEqual({ error: 'the game is already over' });

    const second = await start(engine);
    const timeout = await request(engine, 'POST', `/api/sessions/${second}/end`, {
      seat: 1,
      reason: 'timeout',
    });
    expect((timeout.body as View).result).toEqual({
      winner: 0,
      end: { kind: 'timeout', seat: 1, limit_ms: 0 },
    });
  });

  it('forgets a deleted session', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    const deleted = await request(engine, 'DELETE', `/api/sessions/${id}`);
    expect(deleted.status).toBe(200);
    expect((await request(engine, 'GET', `/api/sessions/${id}`)).status).toBe(404);
    expect((await request(engine, 'DELETE', `/api/sessions/${id}`)).status).toBe(404);
  });

  it('drops sessions untouched for two hours', async () => {
    const { engine, clock } = await testEngine();
    const old = await start(engine);
    clock.now += 2 * 60 * 60 * 1000 + 1;
    const fresh = await start(engine);
    expect((await request(engine, 'GET', `/api/sessions/${old}`)).status).toBe(404);
    expect((await request(engine, 'GET', `/api/sessions/${fresh}`)).status).toBe(200);
  });

  it('gives its record as text of format 1, source studio, ended unfinished while going on', async () => {
    const { engine, clock } = await testEngine();
    const id = await start(engine);
    await move(engine, id, 0, 0);
    const record = await request(engine, 'GET', `/api/sessions/${id}/record`);
    expect(record.status).toBe(200);
    const parsed = record.body as Record<string, unknown>;
    expect(parsed).toMatchObject({
      format: 1,
      game: 'uttt',
      seed: 7,
      source: 'studio',
      unix_time: Math.floor(clock.now / 1000),
      end: { kind: 'aborted', reason: 'unfinished' },
      winner: null,
    });
    expect((parsed.players as { name: string }[]).map((player) => player.name)).toEqual([
      'Ann',
      'Bob',
    ]);
  });
});

describe('the saved games', () => {
  it('lists nothing at first', async () => {
    const { engine } = await testEngine();
    const list = await request(engine, 'GET', '/api/history');
    expect(list.status).toBe(200);
    expect(list.body).toEqual([]);
  });

  it('saves a session once, lists it, gives its record back exactly, views and deletes it', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    await playOut(engine, id);
    const recordText = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;

    const saved = await request(engine, 'POST', `/api/sessions/${id}/save`, {});
    expect(saved.status).toBe(200);
    const { id: gameId, duplicate } = saved.body as { id: string; duplicate: boolean };
    expect(gameId).toMatch(/^uttt-\d+-[0-9a-f]{12}$/);
    expect(validHistoryId(gameId)).toBe(true);
    expect(duplicate).toBe(false);
    const again = await request(engine, 'POST', `/api/sessions/${id}/save`, {});
    expect(again.body).toEqual({ id: gameId, duplicate: true });

    const list = (await request(engine, 'GET', '/api/history')).body as {
      id: string;
      end: string;
    }[];
    expect(list).toHaveLength(1);
    expect(list[0]).toMatchObject({ id: gameId, game: 'uttt', end: 'finished', source: 'studio' });

    const exported = await request(engine, 'GET', `/api/history/${gameId}`);
    expect(exported.status).toBe(200);
    expect(exported.text).toBe(recordText);

    const shown = await request(engine, 'GET', `/api/history/${gameId}/view`);
    expect(shown.status).toBe(200);
    expect(shown.body).toMatchObject({ opening_turns: 0 });
    expect((shown.body as { frames: unknown[] }).frames.length).toBeGreaterThan(1);

    const removed = await request(engine, 'DELETE', `/api/history/${gameId}`);
    expect(removed.status).toBe(204);
    expect(removed.text).toBe('');
    expect((await request(engine, 'DELETE', `/api/history/${gameId}`)).status).toBe(404);
    expect((await request(engine, 'GET', `/api/history/${gameId}`)).status).toBe(404);
    expect((await request(engine, 'GET', '/api/history')).body).toEqual([]);
  });

  it('saves a record file as given, and the same one twice only once', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    await playOut(engine, id);
    const text = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;
    const first = await request(engine, 'POST', '/api/history', text);
    const second = await request(engine, 'POST', '/api/history', text);
    expect((first.body as { duplicate: boolean }).duplicate).toBe(false);
    expect((second.body as { duplicate: boolean }).duplicate).toBe(true);
    expect(((await request(engine, 'GET', '/api/history')).body as unknown[]).length).toBe(1);
  });

  it('refuses a record that breaks the rules or is not one, with a 400 and the reason', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    await move(engine, id, 0, 0);
    const text = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;
    const broken = text.replace(
      '"end":{"kind":"aborted","reason":"unfinished"}',
      '"end":{"kind":"finished"}',
    );
    expect(broken).not.toBe(text);
    const refusedEnd = await request(engine, 'POST', '/api/history', broken);
    expect(refusedEnd.status).toBe(400);
    expect((refusedEnd.body as { error: string }).error.length).toBeGreaterThan(0);
    for (const body of ['', 'null', '[]', '{"format":2}', 'not json']) {
      const refused = await request(engine, 'POST', '/api/history', body);
      expect(refused.status).toBe(400);
      expect(typeof (refused.body as { error: string }).error).toBe('string');
    }
    expect((await request(engine, 'POST', '/api/view', '{"nope":1}')).status).toBe(400);
    expect((await request(engine, 'GET', '/api/history')).body).toEqual([]);
  });

  it('views a record that is not saved, and filters the list', async () => {
    const { engine } = await testEngine();
    const id = await start(engine);
    await playOut(engine, id);
    const text = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;
    const shown = await request(engine, 'POST', '/api/view', text);
    expect(shown.status).toBe(200);
    expect((shown.body as { shown_turns: number }).shown_turns).toBeGreaterThan(0);
    expect(((await request(engine, 'GET', '/api/history')).body as unknown[]).length).toBe(0);

    await request(engine, 'POST', '/api/history', text);
    const ann = await request(engine, 'GET', '/api/history?release=Ann');
    expect((ann.body as unknown[]).length).toBe(1);
    const nobody = await request(engine, 'GET', '/api/history?release=Zed');
    expect(nobody.body).toEqual([]);
    const fault = await request(engine, 'GET', '/api/history?result=fault');
    expect(fault.body).toEqual([]);
    const bad = await request(engine, 'GET', '/api/history?result=bogus');
    expect(bad.status).toBe(400);
  });

  it('filters the rows of the body like the list of saved games, without the store', async () => {
    const { engine } = await testEngine({ store: null });
    const row = (
      id: string,
      unixTime: number,
      players: [string, string],
      winner: number | null,
    ) => ({
      id,
      game: 'uttt',
      unix_time: unixTime,
      players,
      winner,
      end: 'finished',
      turns: 40,
      source: 'arena match',
    });
    const rows = [
      row('uttt-100-aaaaaaaaaaaa', 100, ['uttt-v001', 'uttt-v002'], 0),
      row('uttt-300-cccccccccccc', 300, ['uttt-v002', 'uttt-v003'], 1),
      row('uttt-200-bbbbbbbbbbbb', 200, ['uttt-v001', 'uttt-v003'], null),
    ];
    const ids = async (query: string) =>
      ((await request(engine, 'POST', `/api/filter${query}`, rows)).body as { id: string }[]).map(
        (found) => found.id,
      );
    // Newest first.
    expect(await ids('')).toEqual([
      'uttt-300-cccccccccccc',
      'uttt-200-bbbbbbbbbbbb',
      'uttt-100-aaaaaaaaaaaa',
    ]);
    expect(await ids('?release=uttt-v002')).toEqual([
      'uttt-300-cccccccccccc',
      'uttt-100-aaaaaaaaaaaa',
    ]);
    expect(await ids('?result=draw')).toEqual(['uttt-200-bbbbbbbbbbbb']);
    expect(await ids('?source=studio')).toEqual([]);
    expect(await ids('?from=2030-01-01')).toEqual([]);
    expect((await request(engine, 'POST', '/api/filter', rows)).body).toEqual([
      rows[1],
      rows[2],
      rows[0],
    ]);
    expect((await request(engine, 'POST', '/api/filter', [])).body).toEqual([]);
  });

  it('lets the release filter find the league names, which leave out the game', async () => {
    const { engine } = await testEngine({ store: null });
    const row = (id: string, players: [string, string]) => ({
      id,
      game: 'uttt',
      unix_time: 1,
      players,
      winner: 0,
      end: 'finished',
      turns: 40,
      source: 'arena league',
    });
    const rows = [row('league', ['v009', 'v010']), row('sprt', ['uttt-v010', 'uttt-v009'])];
    const found = await request(engine, 'POST', '/api/filter?release=uttt-v009', rows);
    expect((found.body as { id: string }[]).map((entry) => entry.id)).toEqual(['sprt', 'league']);
    const none = await request(engine, 'POST', '/api/filter?release=uttt-v001', rows);
    expect(none.body).toEqual([]);
  });

  it('refuses a filter with a bad query or body with a 400, and other methods with a 405', async () => {
    const { engine } = await testEngine();
    // The texts are the server's (studio/server/src/history_tests.rs checks the same ones): the
    // query is read first, then the body.
    const queries: [string, string][] = [
      ['?result=bogus', 'result must be x, o, draw, fault or unfinished'],
      ['?source=web', 'source must be studio or arena'],
    ];
    for (const [query, message] of queries) {
      for (const body of [[], 'not json']) {
        const refused = await request(engine, 'POST', `/api/filter${query}`, body);
        expect(refused.status).toBe(400);
        expect(refused.body).toEqual({ error: message });
      }
    }
    for (const query of ['?from=yesterday', '?release=%zz']) {
      const refused = await request(engine, 'POST', `/api/filter${query}`, []);
      expect(refused.status).toBe(400);
      expect(typeof (refused.body as { error: string }).error).toBe('string');
    }
    const bodies: [string | undefined, string][] = [
      [undefined, 'EOF while parsing a value at line 1 column 0'],
      ['', 'EOF while parsing a value at line 1 column 0'],
      ['not json', 'expected ident at line 1 column 2'],
      ['{"rows":[]}', 'invalid type: map, expected a sequence at line 1 column 0'],
      ['[1]', 'invalid type: integer `1`, expected struct Row at line 1 column 2'],
      ['[{"id":"x"}]', 'missing field `game` at line 1 column 11'],
    ];
    for (const [body, message] of bodies) {
      const refused = await request(engine, 'POST', '/api/filter', body);
      expect(refused.status).toBe(400);
      expect(refused.body).toEqual({ error: `invalid request body: ${message}` });
    }
    for (const method of ['GET', 'PUT', 'DELETE']) {
      const wrong = await request(engine, method, '/api/filter');
      expect(wrong.status).toBe(405);
      expect(wrong.body).toEqual({ error: 'method not allowed' });
    }
    expect((await request(engine, 'POST', '/api/filter/x', [])).status).toBe(404);
  });

  it('checks the id the way the server does', async () => {
    expect(validHistoryId('uttt-1760000000-0123456789ab')).toBe(true);
    for (const id of ['', 'A', 'a/b', '../x', 'a b', 'a.json', 'é']) {
      expect(validHistoryId(id)).toBe(false);
    }
    const { engine } = await testEngine();
    for (const path of ['/api/history/Bad', '/api/history/..%2Fx', '/api/history/A/view']) {
      const got = await request(engine, 'GET', path);
      expect(got.status).toBe(400);
      expect(got.body).toEqual({ error: 'invalid game id' });
    }
    expect((await request(engine, 'DELETE', '/api/history/Bad')).status).toBe(400);
    expect((await request(engine, 'GET', '/api/history/nope')).status).toBe(404);
    expect((await request(engine, 'GET', '/api/history/nope/view')).status).toBe(404);
  });

  it('answers 409 to the routes that need the store when the browser has none, and still plays', async () => {
    const { engine } = await testEngine({ store: null });
    const id = await start(engine);
    await playOut(engine, id);
    const text = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;
    for (const [method, path, body] of [
      ['GET', '/api/history', undefined],
      ['POST', '/api/history', text],
      ['POST', `/api/sessions/${id}/save`, {}],
      ['GET', '/api/history/uttt-1-aa', undefined],
      ['GET', '/api/history/uttt-1-aa/view', undefined],
      ['DELETE', '/api/history/uttt-1-aa', undefined],
    ] as const) {
      const answer = await request(engine, method, path, body);
      expect(answer.status).toBe(409);
      expect(answer.body).toEqual({ error: STORE_UNAVAILABLE });
    }
    // Reading a file without saving it does not need the store.
    expect((await request(engine, 'POST', '/api/view', text)).status).toBe(200);
    // A bad record is still the caller's fault, whatever the store.
    expect((await request(engine, 'POST', '/api/history', 'nope')).status).toBe(400);
  });

  it('tries the store again after it failed to open', async () => {
    let attempts = 0;
    const store = memoryGameStore();
    const { engine } = await testEngine({
      store: async () => {
        attempts += 1;
        if (attempts === 1) throw new Error('blocked');
        return store;
      },
    });
    expect((await request(engine, 'GET', '/api/history')).status).toBe(409);
    expect((await request(engine, 'GET', '/api/history')).status).toBe(200);
    expect((await request(engine, 'GET', '/api/history')).status).toBe(200);
    expect(attempts).toBe(2);
  });
});
