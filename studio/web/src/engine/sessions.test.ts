/* The sessions with bots, over the real rules and bots played by scripts (the real bots run in
   workers, which the browser tests cover). */

import { describe, expect, it } from 'vitest';
import { BotError } from './bot';
import type { Engine } from './routes';
import {
  firstLegalMove,
  request,
  scriptedFactory,
  testEngine,
  until,
  type Script,
} from './testSupport';

const legal: Script = async (input) => [firstLegalMove(input)];

const bot = (release: string, mode: 'fixed' | 'realtime', thinkMs = 100) => ({
  kind: 'bot',
  release,
  think_ms: thinkMs,
  mode,
});
const human = { kind: 'human', name: 'me' };

function game(...seats: unknown[]) {
  return { game: 'uttt', seed: 11, opening_plies: 0, seats };
}

interface View {
  status: string;
  error: string | null;
  to_act: number[];
  turns: { seat: number; lines: string[] }[][];
  frames: unknown[];
  human_moves: unknown[];
  result: { winner: number | null; end: { kind: string; [key: string]: unknown } } | null;
  progress: { done: number; total: number | null } | null;
  seats: { fixed_iters?: number | null }[];
}

async function start(engine: Engine, body: unknown): Promise<string> {
  const created = await request(engine, 'POST', '/api/sessions', body);
  expect(created.status).toBe(200);
  return (created.body as { id: string }).id;
}

const view = async (engine: Engine, id: string): Promise<View> =>
  (await request(engine, 'GET', `/api/sessions/${id}`)).body as View;

const untilStatus = (engine: Engine, id: string, ...statuses: string[]) =>
  until(
    () => view(engine, id),
    (current) => statuses.includes(current.status),
  );

/** A promise settled by hand. */
function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}

describe('a game against a bot in fixed mode', () => {
  it('loads, measures, converts the think time to iterations and starts the bot with its settings', async () => {
    const { factory, envs } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, measure: async () => 2.5 });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed', 100)));
    const ready = await untilStatus(engine, id, 'waiting_human');
    expect(ready.seats[1]?.fixed_iters).toBe(250);
    expect(envs).toHaveLength(1);
    expect(envs[0]?.CG_FIXED_ITERS).toBe('250');
    expect(envs[0]?.CG_SEED).toMatch(/^\d{17,}$/);
    expect(envs[0]).not.toHaveProperty('CG_TIME_SCALE');
  });

  it('shows measuring, with the progress of the measuring game', async () => {
    const { factory } = scriptedFactory([legal]);
    const gate = deferred<number>();
    const { engine } = await testEngine({
      makeBot: factory,
      measure: async (options) => {
        options.progress(3);
        return gate.promise;
      },
    });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    const measuring = await untilStatus(engine, id, 'measuring');
    await until(
      async () => (await view(engine, id)).progress,
      (progress) => progress?.done === 3,
    );
    expect(measuring.to_act).toEqual([0]);
    gate.resolve(1);
    await untilStatus(engine, id, 'waiting_human');
  });

  it('plays the human move, thinks, answers, and waits again', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    const sent = await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
    expect((sent.body as View).status).toBe('bot_thinking');
    expect((sent.body as View).human_moves).toEqual([]);
    const after = await untilStatus(engine, id, 'waiting_human');
    expect(after.turns).toHaveLength(2);
    expect(after.turns[1]?.[0]?.seat).toBe(1);
    expect(after.to_act).toEqual([0]);
    expect(after.frames).toHaveLength(3);
  });

  it('takes a move back by restarting the bot and replaying its answers', async () => {
    const { factory, envs, bots } = scriptedFactory([legal, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    for (let move = 0; move < 2; move++) {
      await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
      await untilStatus(engine, id, 'waiting_human');
    }
    expect((await view(engine, id)).turns).toHaveLength(4);

    const back = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 2 });
    const rewinding = back.body as View;
    expect(rewinding.status).toBe('rewinding');
    expect(rewinding.progress).toEqual({ done: 0, total: 1 });
    const done = await untilStatus(engine, id, 'waiting_human');
    expect(done.turns).toHaveLength(2);
    expect(done.progress).toBeNull();
    expect(envs).toHaveLength(2);
    // The same seed and iterations again, and the first bot was stopped.
    expect(envs[1]).toEqual(envs[0]);
    expect(bots).toHaveLength(2);
    // It plays on from there.
    await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 1 });
    expect((await untilStatus(engine, id, 'waiting_human')).turns).toHaveLength(4);
  });

  it('fails the session, naming the release, when the bot does not replay identically', async () => {
    const other: Script = async (input) => [input.split('\n')[3] ?? ''];
    const { factory } = scriptedFactory([legal, other]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 2 });
    const failed = await untilStatus(engine, id, 'failed');
    expect(failed.error).toBe(
      'uttt-v001 did not replay identically; takebacks are unavailable against it',
    );
    expect(failed.to_act).toEqual([]);
  });

  it('fails the session when the replay cannot even be asked', async () => {
    const broken: Script = async () => {
      throw new BotError('crash', 'exited with code 1');
    };
    const { factory } = scriptedFactory([legal, broken]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 2 });
    const failed = await untilStatus(engine, id, 'failed');
    expect(failed.error).toBe('cannot restart uttt-v001: exited with code 1');
  });

  it('lets a takeback cancel a bot that is thinking, and drops its late answer', async () => {
    const gate = deferred<string[]>();
    const slow: Script = async () => gate.promise;
    const { factory, bots } = scriptedFactory([slow, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
    expect((await view(engine, id)).status).toBe('bot_thinking');
    const back = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 0 });
    expect(back.status).toBe(200);
    await untilStatus(engine, id, 'waiting_human');
    // The cancelled bot answers late: nothing changes.
    gate.resolve(['4 4']);
    await new Promise((resolve) => setTimeout(resolve, 20));
    const now = await view(engine, id);
    expect(now.turns).toHaveLength(0);
    expect(now.status).toBe('waiting_human');
    expect(bots).toHaveLength(2);
  });

  it('stops its bot when the session is deleted', async () => {
    const gate = deferred<string[]>();
    const { factory, bots } = scriptedFactory([async () => gate.promise]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    await request(engine, 'POST', `/api/sessions/${id}/move`, { seat: 0, index: 0 });
    let stopped = false;
    const bot0 = bots[0];
    if (bot0 === undefined) throw new Error('no bot');
    const terminate = bot0.terminate;
    bot0.terminate = () => {
      stopped = true;
      terminate();
    };
    expect((await request(engine, 'DELETE', `/api/sessions/${id}`)).status).toBe(200);
    expect(stopped).toBe(true);
    gate.resolve(['4 4']);
    expect((await request(engine, 'GET', `/api/sessions/${id}`)).status).toBe(404);
  });

  it('refuses a takeback while the session is busy', async () => {
    const gate = deferred<number>();
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, measure: async () => gate.promise });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'measuring');
    const refused = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 0 });
    expect(refused.status).toBe(409);
    expect(refused.body).toEqual({ error: 'no takeback now (status measuring)' });
    gate.resolve(1);
  });

  it('ends only a human seat by hand', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    const refused = await request(engine, 'POST', `/api/sessions/${id}/end`, {
      seat: 1,
      reason: 'resign',
    });
    expect(refused.status).toBe(409);
    expect(refused.body).toEqual({ error: 'only a human seat can resign or run out of time' });
    const ended = await request(engine, 'POST', `/api/sessions/${id}/end`, {
      seat: 0,
      reason: 'resign',
    });
    expect((ended.body as View).status).toBe('over');
  });

  it('refuses a takeback beyond the valid turns and below the opening', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    await untilStatus(engine, id, 'waiting_human');
    const refused = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 1 });
    expect(refused.status).toBe(409);
    expect(refused.body).toEqual({ error: 'turns must be from 0 to 0, got 1' });
  });
});

describe('a game with a baseline bot', () => {
  it('loads the baseline bot like a release and starts it with its settings', async () => {
    const { factory, envs } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, measure: async () => 2.5 });
    const id = await start(engine, game(human, bot('uttt-greedy', 'fixed', 100)));
    const ready = await untilStatus(engine, id, 'waiting_human');
    expect(ready.seats[1]?.fixed_iters).toBe(250);
    expect(envs[0]?.CG_FIXED_ITERS).toBe('250');
  });

  it('plays a game that has no release, two baseline bots of its own', async () => {
    const { factory, envs } = scriptedFactory([legal, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const iceAndFire = {
      game: 'ice-and-fire',
      seed: 3,
      opening_plies: 0,
      seats: [
        bot('ice-and-fire-bronze', 'realtime', 10),
        bot('ice-and-fire-random', 'realtime', 10),
      ],
    };
    const id = await start(engine, iceAndFire);
    await until(
      async () => envs.length,
      (started) => started === 2,
    );
    const current = await view(engine, id);
    expect(current.status).not.toBe('failed');
  });

  it('refuses a baseline bot of another game, and one that is not built', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, baselines: { uttt: ['uttt-random'] } });
    const refused = async (body: unknown) => request(engine, 'POST', '/api/sessions', body);
    const other = await refused(game(human, bot('ice-and-fire-bronze', 'fixed')));
    expect(other.status).toBe(400);
    expect(other.body).toEqual({ error: 'unknown release "ice-and-fire-bronze"' });
    // The adapter names it, but its file is not in the manifest: the load fails when it starts.
    const missing = await start(engine, game(human, bot('uttt-greedy', 'fixed')));
    const failed = await untilStatus(engine, missing, 'failed');
    expect(failed.error).toBe(
      'cannot load uttt-greedy: cannot load wasm/uttt/uttt-greedy.wasm (HTTP 404)',
    );
  });
});

describe('a game against a bot in real time', () => {
  it('starts the bot with a time scale of think_ms over 100, unset when it is 1', async () => {
    const { factory, envs } = scriptedFactory([legal, legal, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    await untilStatus(
      engine,
      await start(engine, game(human, bot('uttt-v001', 'realtime', 10))),
      'waiting_human',
    );
    await untilStatus(
      engine,
      await start(engine, game(human, bot('uttt-v001', 'realtime', 250))),
      'waiting_human',
    );
    await untilStatus(
      engine,
      await start(engine, game(human, bot('uttt-v001', 'realtime', 100))),
      'waiting_human',
    );
    expect(envs.map((env) => env.CG_TIME_SCALE)).toEqual(['0.1', '2.5', undefined]);
    for (const env of envs) expect(env).not.toHaveProperty('CG_FIXED_ITERS');
  });

  it('refuses a takeback', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(human, bot('uttt-v001', 'realtime', 10)));
    await untilStatus(engine, id, 'waiting_human');
    const refused = await request(engine, 'POST', `/api/sessions/${id}/takeback`, { turns: 0 });
    expect(refused.status).toBe(409);
    expect(refused.body).toEqual({
      error: 'takebacks are unavailable against a bot playing in real time',
    });
  });

  it('plays two bots to the end, and the polled views stay consistent meanwhile', async () => {
    const { factory } = scriptedFactory([legal, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(
      engine,
      game(bot('uttt-v002', 'realtime', 10), bot('uttt-v001', 'realtime', 10)),
    );
    const seen = new Set<string>();
    const over = await until(
      async () => {
        const current = await view(engine, id);
        seen.add(current.status);
        // The frames always match the turns that replay; a bot's turn has one to act.
        expect(current.frames.length).toBe(current.turns.length + 1);
        if (current.status === 'over') expect(current.result).not.toBeNull();
        else expect(current.result).toBeNull();
        if (current.status === 'bot_thinking') expect(current.to_act).toHaveLength(1);
        return current;
      },
      (current) => current.status === 'over',
    );
    expect(over.result?.end.kind).toBe('finished');
    expect(seen.has('bot_thinking') || over.turns.length > 0).toBe(true);
  });

  it('saves the finished game with the bots seeds as exact digits, and gives them back', async () => {
    const { factory, envs } = scriptedFactory([legal, legal]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(
      engine,
      game(bot('uttt-v002', 'realtime', 10), bot('uttt-v001', 'realtime', 10)),
    );
    await untilStatus(engine, id, 'over');
    const text = (await request(engine, 'GET', `/api/sessions/${id}/record`)).text;
    const seeds = [...text.matchAll(/"bot_seed":(\d+)/g)].map((match) => match[1]);
    expect(seeds).toEqual([envs[0]?.CG_SEED, envs[1]?.CG_SEED]);
    const saved = (await request(engine, 'POST', `/api/sessions/${id}/save`, {})).body as {
      id: string;
    };
    const exported = await request(engine, 'GET', `/api/history/${saved.id}`);
    expect(exported.text).toBe(text);
    const players = (
      JSON.parse(text) as { players: { time_scale: number; fixed_iters: number | null }[] }
    ).players;
    expect(players.map((player) => player.time_scale)).toEqual([0.1, 0.1]);
    expect(players.map((player) => player.fixed_iters)).toEqual([null, null]);
  });
});

describe('a bot that loses', () => {
  const lost = async (script: Script) => {
    const { factory } = scriptedFactory([script]);
    const { engine } = await testEngine({ makeBot: factory });
    const id = await start(engine, game(bot('uttt-v001', 'realtime', 10), human));
    return untilStatus(engine, id, 'over', 'failed');
  };

  it('times out: the seat loses with the limit it had', async () => {
    const over = await lost(async () => {
      throw new BotError('timeout', 'the bot did not answer in time');
    });
    expect(over.status).toBe('over');
    expect(over.result?.winner).toBe(1);
    expect(over.result?.end).toMatchObject({ kind: 'timeout', seat: 0 });
    expect(over.result?.end.limit_ms).toBeGreaterThan(0);
  });

  it('crashes, with the detail', async () => {
    const over = await lost(async () => {
      throw new BotError('crash', 'exited with code 3');
    });
    expect(over.result).toMatchObject({
      winner: 1,
      end: { kind: 'crash', seat: 0, detail: 'exited with code 3' },
    });
  });

  it('answers against the rules: the turn is kept and the seat loses', async () => {
    const over = await lost(async () => ['9 9']);
    expect(over.result?.winner).toBe(1);
    expect(over.result?.end).toMatchObject({ kind: 'invalid', seat: 0 });
    expect(over.turns).toHaveLength(1);
    expect(over.frames).toHaveLength(1);
  });

  it('fails the session on an error that is not the bot', async () => {
    const failed = await lost(async () => {
      throw new Error('boom');
    });
    expect(failed.status).toBe('failed');
    expect(failed.error).toBe('boom');
  });
});

describe('what goes wrong before the bots play', () => {
  it('fails the session with a clear error when a release file cannot be fetched', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, broken: ['uttt-v001'] });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    const failed = await untilStatus(engine, id, 'failed');
    expect(failed.error).toBe(
      'cannot load uttt-v001: cannot load wasm/uttt/uttt-v001.wasm (HTTP 404)',
    );
    expect(failed.to_act).toEqual([]);
    expect(failed.human_moves).toEqual([]);
  });

  it('fails the session when the speed cannot be measured', async () => {
    const { factory } = scriptedFactory([legal]);
    const { engine } = await testEngine({
      makeBot: factory,
      measure: async () => {
        throw new Error('the game was too short to measure');
      },
    });
    const id = await start(engine, game(human, bot('uttt-v001', 'fixed')));
    const failed = await untilStatus(engine, id, 'failed');
    expect(failed.error).toBe(
      'cannot measure the speed of uttt-v001: the game was too short to measure',
    );
  });

  it('refuses a release that is not in the manifest with a 400, before anything is loaded', async () => {
    const { factory, envs } = scriptedFactory([legal]);
    const { engine } = await testEngine({ makeBot: factory, releases: ['uttt-v002'] });
    const answer = await request(
      engine,
      'POST',
      '/api/sessions',
      game(human, bot('uttt-v001', 'fixed')),
    );
    expect(answer.status).toBe(400);
    expect(answer.body).toEqual({ error: 'unknown release "uttt-v001"' });
    expect(envs).toHaveLength(0);
  });
});
