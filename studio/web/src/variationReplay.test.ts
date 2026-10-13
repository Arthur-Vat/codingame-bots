import { describe, expect, it } from 'vitest';
import type { Answer, GameRecord, RecordView } from './api';
import { requestMoves } from './analysisView';
import { request, testEngine } from './engine/testSupport';
import { fetchVariation, variationRecord } from './variation';

// The side line's record against the real rules program (built to WebAssembly by `npm run wasm`,
// the same program the browser engine runs; the local server runs the same Rust code). The point:
// `opening_plies: 0` with the opening's turns as ordinary answers replays to the positions of the
// game that has an opening, whatever its seed.

const turn = (seat: number, line: string): Answer[] => [{ seat, lines: [line], ms: 0 }];

const players = ['X', 'O'].map((name) => ({
  name,
  kind: 'human' as const,
  bot_seed: null,
  command: null,
  time_scale: 1,
  fixed_iters: null,
}));

function game(patch: Partial<GameRecord>): GameRecord {
  return {
    format: 1,
    game: 'uttt',
    seed: 7,
    opening_plies: 0,
    unix_time: 0,
    source: 'test',
    players: [players[0]!, players[1]!],
    turns: [],
    end: { kind: 'aborted', reason: 'test' },
    winner: null,
    ...patch,
  } as GameRecord;
}

async function post(record: unknown): Promise<RecordView> {
  const { engine } = await testEngine();
  const answer = await request(engine, 'POST', '/api/view', record);
  if (answer.status !== 200) throw new Error((answer.body as { error: string }).error);
  return answer.body as RecordView;
}

/** The legal moves of a position, from its frame. */
function playable(frame: unknown): string[] {
  return (frame as { playable: [number, number][] }).playable.map(([row, col]) => `${row} ${col}`);
}

/**
 * The turns of a game that has an opening of `plies`: the opening's own moves, which are not
 * written anywhere but are the only ones the rules take, found by trying; then `extra` legal moves.
 */
async function openedTurns(plies: number, seed: number, extra: number): Promise<Answer[][]> {
  const opening = (await post(game({ seed, opening_plies: plies }))).opening_turns;
  expect(opening).toBe(plies);
  const cells: string[] = [];
  for (let row = 0; row < 9; row++) for (let col = 0; col < 9; col++) cells.push(`${row} ${col}`);
  const search = async (turns: Answer[][]): Promise<Answer[][] | null> => {
    if (turns.length === opening) return turns;
    for (const cell of cells) {
      const next = [...turns, turn(turns.length % 2, cell)];
      try {
        await post(game({ seed, opening_plies: plies, turns: next }));
      } catch {
        continue;
      }
      const found = await search(next);
      if (found !== null) return found;
    }
    return null;
  };
  const found = await search([]);
  expect(found).not.toBeNull();
  const turns = found!;
  for (let i = 0; i < extra; i++) {
    const view = await post(game({ seed, opening_plies: plies, turns }));
    const frame = view.frames[view.frames.length - 1];
    turns.push(turn(turns.length % 2, playable(frame)[0]!));
  }
  return turns;
}

describe('a side line on a game with an opening', () => {
  it('replays to the positions of the game itself, whatever its seed', async () => {
    const seed = 123456789;
    const turns = await openedTurns(4, seed, 3);
    const real = await post(game({ seed, opening_plies: 4, turns }));
    expect(real.opening_turns).toBe(4);
    expect(real.frames).toHaveLength(4);

    const model = { game: 'uttt', turns, openingTurns: 4 };
    // Branching from the first frame replays the game's own moves: the same frames.
    const same = turns.slice(4).map((t) => t[0]!.lines[0]!);
    const asGame = await fetchVariation(model, { from: 0, moves: same, cursor: 3 }, post);
    expect(asGame.frames).toEqual(real.frames.slice(1));

    // Branching from a later frame, and playing something else there.
    const frame = real.frames[1];
    const other = playable(frame).find((move) => move !== turns[5]![0]!.lines[0]!)!;
    const branch = await fetchVariation(model, { from: 1, moves: [other], cursor: 1 }, post);
    const check = await post(
      game({ seed, opening_plies: 4, turns: [...turns.slice(0, 5), turn(1, other)] }),
    );
    expect(branch.frames).toEqual([check.frames[2]]);
    // The engine is asked for the opening turns and the moves, as the record holds them.
    expect(requestMoves(model, 1)).toHaveLength(5);
  });

  it('is refused with the engine’s message when a move is illegal', async () => {
    const turns = await openedTurns(2, 5, 1);
    const model = { game: 'uttt', turns, openingTurns: 2 };
    const taken = turns[0]![0]!.lines[0]!;
    await expect(
      fetchVariation(model, { from: 1, moves: [taken], cursor: 1 }, post),
    ).rejects.toThrow(/turn 3/);
  });

  it('from the first frame of a game with no opening starts with seat 0', async () => {
    const model = { game: 'uttt', turns: [] as Answer[][], openingTurns: 0 };
    const next = await fetchVariation(model, { from: 0, moves: ['4 4', '4 5'], cursor: 1 }, post);
    expect(next.frames).toHaveLength(2);
    expect(variationRecord(model, 0, ['4 4']).turns[0]?.[0]?.seat).toBe(0);
  });
});
