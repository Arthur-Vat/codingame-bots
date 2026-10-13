import { describe, expect, it } from 'vitest';
import {
  createRules,
  RulesError,
  botSeeds,
  type Rules,
  type RulesState,
  type Settled,
} from './rules';
import { loadedRules, rulesModule } from './testSupport';

const bot = { kind: 'bot', release: 'uttt-v001', think_ms: 10, mode: 'fixed' };
const human = (name: string) => ({ kind: 'human', name });

async function newState(seats: unknown[], seed = 5): Promise<{ state: RulesState; rules: Rules }> {
  const rules = await loadedRules();
  const state = rules.call<RulesState>({
    op: 'new',
    request: { game: 'uttt', seed, opening_plies: 0, seats },
    releases: ['uttt-v001'],
  });
  return { state, rules };
}

describe('the rules program', () => {
  it('lists the games', async () => {
    const rules = await loadedRules();
    expect(rules.call({ op: 'games' })).toEqual([{ id: 'uttt', name: 'Ultimate Tic-Tac-Toe' }]);
  });

  it('refuses a bad request with its own message', async () => {
    const rules = await loadedRules();
    expect(() => rules.call({ op: 'nope' })).toThrow(RulesError);
    expect(() => rules.call({ op: 'nope' })).toThrow('unknown op "nope"');
    expect(() =>
      rules.call({
        op: 'new',
        request: { game: 'uttt', seed: 1, seats: [bot, human('me')] },
        releases: ['uttt-v002'],
      }),
    ).toThrow('unknown release "uttt-v001"');
  });

  it('settles a new game between humans on the first move', async () => {
    const { state, rules } = await newState([human('a'), human('b')]);
    const settled = rules.call<Settled>({ op: 'settle', state });
    expect(settled.next).toEqual({ kind: 'human', seat: 0 });
    expect(settled.to_act).toEqual([0]);
    expect(settled.human_moves).toHaveLength(81);
    expect(settled.frames).toHaveLength(1);
  });

  it('asks a bot for its input, with its seed as a string that keeps every digit', async () => {
    const { state, rules } = await newState([bot, human('me')]);
    const { next } = rules.call<Settled>({ op: 'settle', state });
    if (next.kind !== 'bot') throw new Error('a bot was expected');
    expect(next.answer_lines).toBe(1);
    expect(next.input.length).toBeGreaterThan(0);
    expect(typeof next.bot_seed).toBe('string');
    expect(next.bot_seed).toMatch(/^\d+$/);
    // Above 2^53: a number would have lost digits.
    expect(BigInt(next.bot_seed)).toBeGreaterThan(2n ** 53n);
  });

  it('plays an answer, and keeps one the rules reject as a loss for its seat', async () => {
    const { state, rules } = await newState([human('a'), human('b')]);
    interface Played {
      turns: unknown[][];
      result: { winner: number; end: { kind: string } } | null;
    }
    const played = rules.call<Played>({
      op: 'play',
      state,
      answer: { seat: 0, lines: ['0 0'], ms: 0 },
    });
    expect(played.turns).toHaveLength(1);
    expect(played.result).toBeNull();
    const bad = rules.call<Played>({
      op: 'play',
      state,
      answer: { seat: 0, lines: ['not a move'], ms: 0 },
    });
    expect(bad.turns).toHaveLength(1);
    expect(bad.result?.winner).toBe(1);
    expect(bad.result?.end.kind).toBe('invalid');
  });

  it('writes the record as text, and checks and views it', async () => {
    const { state, rules } = await newState([human('a'), human('b')]);
    state.turns = rules.call<{ turns: RulesState['turns'] }>({
      op: 'play',
      state,
      answer: { seat: 0, lines: ['4 4'], ms: 0 },
    }).turns;
    const { record_text } = rules.call<{ record_text: string }>({
      op: 'record',
      state,
      unix_time: 1_760_000_000,
    });
    expect(typeof record_text).toBe('string');
    const checked = rules.call<{ id: string; row: { turns: number } }>({
      op: 'check',
      record_text,
    });
    expect(checked.id).toMatch(/^uttt-1760000000-[0-9a-f]{12}$/);
    expect(checked.row.turns).toBe(1);
    const view = rules.call<{ frames: unknown[]; shown_turns: number }>({
      op: 'view',
      record_text,
    });
    expect(view.frames).toHaveLength(2);
    expect(view.shown_turns).toBe(1);
    expect(() => rules.call({ op: 'check', record_text: '{"nope":1}' })).toThrow(RulesError);
  });
});

describe('the bot seeds', () => {
  it('are read from the record as text for both bots, and null for a human', async () => {
    const both = await newState([bot, bot]);
    const [first, second] = botSeeds(both.rules, both.state);
    expect(first).toMatch(/^\d{17,}$/);
    expect(second).toMatch(/^\d{17,}$/);
    expect(first).not.toBe(second);
    const one = await newState([bot, human('me')]);
    expect(botSeeds(one.rules, one.state)).toEqual([first, null]);
  });

  it('are not confused by a name that looks like a key', async () => {
    const odd = await newState([human('"bot_seed":7,'), bot]);
    const seeds = botSeeds(odd.rules, odd.state);
    expect(seeds[0]).toBeNull();
    expect(seeds[1]).toMatch(/^\d+$/);
  });
});

describe('the runner', () => {
  it('compiles the module once and only when first needed', async () => {
    let loads = 0;
    const rules = createRules(async () => {
      loads += 1;
      return rulesModule();
    });
    expect(() => rules.call({ op: 'games' })).toThrow('not loaded');
    await rules.ready();
    await rules.ready();
    expect(loads).toBe(1);
    expect(rules.call({ op: 'games' })).toHaveLength(1);
  });

  it('gives an answer as the text the program wrote, and refuses as call does', async () => {
    const rules = createRules(rulesModule);
    expect(() => rules.callText({ op: 'games' })).toThrow('not loaded');
    await rules.ready();
    const text = rules.callText({ op: 'games' });
    expect(text.startsWith('[{')).toBe(true);
    expect(JSON.parse(text)).toEqual(rules.call({ op: 'games' }));
    expect(() => rules.callText({ op: 'nope' })).toThrow(RulesError);
    expect(() => rules.call({ op: 'nope' })).toThrow(RulesError);
  });

  it('tries again after a failed load', async () => {
    let attempts = 0;
    const rules = createRules(async () => {
      attempts += 1;
      if (attempts === 1) throw new Error('offline');
      return rulesModule();
    });
    await expect(rules.ready()).rejects.toThrow('offline');
    await rules.ready();
    expect(rules.call({ op: 'games' })).toHaveLength(1);
  });
});
