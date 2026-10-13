import { describe, expect, it } from 'vitest';
import type { Answer, RecordView } from './api';
import { MAX_REQUEST_MOVES } from './analysisView';
import {
  fetchVariation,
  mainNext,
  planPath,
  shownFrame,
  shownMoves,
  stripTokens,
  variationFrames,
  variationKey,
  variationRecord,
  withCursor,
  type Place,
  type Variation,
} from './variation';

const turn = (seat: number, line: string): Answer[] => [{ seat, lines: [line], ms: 5 }];

// A game of four moves, X first, with frames 0 to 4 (no opening).
const MOVES = ['4 4', '4 5', '5 3', '3 7'];
const model = {
  game: 'uttt',
  turns: MOVES.map((move, index) => turn(index % 2, move)),
  openingTurns: 0,
  frames: ['f0', 'f1', 'f2', 'f3', 'f4'],
};
// The same game with an opening of two turns: frame 0 shows the position after them.
const opened = {
  ...model,
  turns: ['0 0', '1 1', ...MOVES].map((move, index) => turn(index % 2, move)),
  openingTurns: 2,
};

const side = (patch: Partial<Variation> = {}): Variation => ({
  from: 1,
  moves: ['0 0', '1 1', '2 2'],
  frames: ['v1', 'v2', 'v3'],
  cursor: 1,
  ...patch,
});
const onGame = (frame: number): Place => ({ frame, variation: null });
const inSide = (variation: Variation): Place => ({ frame: variation.from, variation });

describe('mainNext', () => {
  it('is the answer of the turn after the frame, openings counted', () => {
    expect(mainNext(model, 0)).toBe('4 4');
    expect(mainNext(model, 3)).toBe('3 7');
    expect(mainNext(opened, 0)).toBe('4 4');
    expect(mainNext(opened, 2)).toBe('5 3');
  });

  it('is null at the last frame, and for a turn that is not a single answer', () => {
    expect(mainNext(model, 4)).toBeNull();
    expect(mainNext({ turns: [[], turn(1, 'a')], openingTurns: 0 }, 0)).toBeNull();
    expect(
      mainNext({ turns: [[...turn(0, 'a'), ...turn(1, 'b')]], openingTurns: 0 }, 0),
    ).toBeNull();
  });
});

describe('playing a move on the game', () => {
  it("steps along the game when it is the game's next move", () => {
    expect(planPath(model, onGame(1), ['4 5'])).toEqual({ kind: 'frame', frame: 2 });
    expect(planPath(opened, onGame(0), ['4 4'])).toEqual({ kind: 'frame', frame: 1 });
  });

  it('starts a side line with any other move, from the frame on the board', () => {
    expect(planPath(model, onGame(1), ['0 0'])).toEqual({
      kind: 'branch',
      branch: { from: 1, moves: ['0 0'], cursor: 1 },
    });
    // At the last frame there is no next move at all.
    expect(planPath(model, onGame(4), ['4 4'])).toEqual({
      kind: 'branch',
      branch: { from: 4, moves: ['4 4'], cursor: 1 },
    });
  });

  it('ignores a move written with spaces around as the same move', () => {
    expect(planPath(model, onGame(0), [' 4 4 '])).toEqual({ kind: 'frame', frame: 1 });
  });
});

describe('playing a move in a side line', () => {
  it('steps when it is the side line’s next move', () => {
    expect(planPath(model, inSide(side()), ['1 1'])).toEqual({ kind: 'cursor', cursor: 2 });
    expect(planPath(model, inSide(side({ cursor: 0 })), ['0 0'])).toEqual({
      kind: 'cursor',
      cursor: 1,
    });
  });

  it('drops the moves after the cursor and appends the move otherwise', () => {
    expect(planPath(model, inSide(side()), ['8 8'])).toEqual({
      kind: 'branch',
      branch: { from: 1, moves: ['0 0', '8 8'], cursor: 2 },
    });
    // At the end of the side line the move extends it.
    expect(planPath(model, inSide(side({ cursor: 3 })), ['8 8'])).toEqual({
      kind: 'branch',
      branch: { from: 1, moves: ['0 0', '1 1', '2 2', '8 8'], cursor: 4 },
    });
    // At its start it is another first move, not the game's next move.
    expect(planPath(model, inSide(side({ cursor: 0 })), ['4 5'])).toEqual({
      kind: 'branch',
      branch: { from: 1, moves: ['4 5'], cursor: 1 },
    });
  });
});

describe('playing an engine line', () => {
  it('steps one move when its first move is the next move of the line the board is on', () => {
    expect(planPath(model, onGame(0), ['4 4', '4 3', '3 1'])).toEqual({ kind: 'frame', frame: 1 });
    expect(planPath(model, inSide(side()), ['1 1', '5 5'])).toEqual({ kind: 'cursor', cursor: 2 });
  });

  it('makes a side line of the whole line, the cursor after its first move', () => {
    expect(planPath(model, onGame(2), ['0 0', '1 1', '2 2'])).toEqual({
      kind: 'branch',
      branch: { from: 2, moves: ['0 0', '1 1', '2 2'], cursor: 1 },
    });
  });

  it('replaces the moves after the cursor in a side line', () => {
    expect(planPath(model, inSide(side()), ['7 7', '6 6'])).toEqual({
      kind: 'branch',
      branch: { from: 1, moves: ['0 0', '7 7', '6 6'], cursor: 2 },
    });
  });

  it('makes a side line of a line equal to the rest of the game only after its first move', () => {
    // The first move is the game's next move: a step, whatever follows.
    expect(planPath(model, onGame(1), ['4 5', '5 3', '3 7'])).toEqual({ kind: 'frame', frame: 2 });
  });

  it('ignores an empty line and a line with something that is not a move', () => {
    expect(planPath(model, onGame(0), [])).toEqual({ kind: 'ignored' });
    expect(planPath(model, onGame(0), ['4 4', 5 as unknown as string])).toEqual({
      kind: 'ignored',
    });
    expect(planPath(model, onGame(0), ['4 4', ''])).toEqual({ kind: 'ignored' });
    expect(planPath(model, onGame(0), [null as unknown as string])).toEqual({ kind: 'ignored' });
  });
});

describe('the length of a side line', () => {
  const moves = (count: number) => Array.from({ length: count }, () => '0 0');

  it('may fill the 200 turns the engine takes, the game before it counted', () => {
    // One turn of the game before the frame, so 199 moves fit.
    const long = side({ from: 1, moves: moves(198), cursor: 198 });
    const plan = planPath(model, inSide(long), ['1 1']);
    expect(plan.kind).toBe('branch');
    expect(plan.kind === 'branch' && plan.branch.moves).toHaveLength(199);
  });

  it('is refused one move past them, with a message that says so', () => {
    const full = side({ from: 1, moves: moves(199), cursor: 199 });
    const plan = planPath(model, inSide(full), ['1 1']);
    expect(plan.kind).toBe('refused');
    expect(plan.kind === 'refused' && plan.message).toContain(String(MAX_REQUEST_MOVES));
    // A whole line counts too.
    expect(planPath(model, onGame(1), moves(200)).kind).toBe('refused');
    expect(planPath(model, onGame(1), moves(199)).kind).toBe('branch');
    // The opening turns count.
    expect(planPath(opened, onGame(1), moves(197)).kind).toBe('branch');
    expect(planPath(opened, onGame(1), moves(198)).kind).toBe('refused');
  });
});

describe('the record of a side line', () => {
  it('replays the game up to the frame, then the moves, with no opening of its own', () => {
    const record = variationRecord(model, 2, ['0 0', '1 1']);
    expect(record).toMatchObject({
      format: 1,
      game: 'uttt',
      seed: 0,
      opening_plies: 0,
      unix_time: 0,
      source: 'studio',
      end: { kind: 'aborted', reason: 'variation' },
      winner: null,
    });
    expect(record.players.map((p) => [p.name, p.kind, p.bot_seed, p.command])).toEqual([
      ['X', 'human', null, null],
      ['O', 'human', null, null],
    ]);
    expect(record.turns.map((t) => [t[0]?.seat, t[0]?.lines])).toEqual([
      [0, ['4 4']],
      [1, ['4 5']],
      [0, ['0 0']],
      [1, ['1 1']],
    ]);
    expect(record.turns[2]).toEqual([{ seat: 0, lines: ['0 0'], ms: 0 }]);
  });

  it('starts with the seat after the last turn before it, seat 0 without one', () => {
    expect(variationRecord(model, 1, ['0 0']).turns[1]?.[0]?.seat).toBe(1);
    expect(variationRecord(model, 0, ['0 0', '1 1']).turns.map((t) => t[0]?.seat)).toEqual([0, 1]);
  });

  it('holds the opening turns as ordinary turns, which a side line from the first frame follows', () => {
    const record = variationRecord(opened, 0, ['0 0']);
    expect(record.turns.map((t) => t[0]?.lines[0])).toEqual(['0 0', '1 1', '0 0']);
    expect(record.turns.map((t) => t[0]?.seat)).toEqual([0, 1, 0]);
    expect(variationRecord(opened, 2, ['9 9']).turns).toHaveLength(5);
  });

  it('starts from the seat after the last turn, as a game where the same seat moved twice would', () => {
    const odd = { ...model, turns: [turn(0, 'a'), turn(0, 'b')] };
    expect(variationRecord(odd, 2, ['c', 'd']).turns.map((t) => t[0]?.seat)).toEqual([0, 0, 1, 0]);
  });
});

describe('the frames of a side line', () => {
  const view = (frames: unknown[], shown: number): RecordView => ({
    record: variationRecord(model, 2, ['0 0', '1 1']),
    frames,
    opening_turns: 0,
    shown_turns: shown,
  });

  it('are the last ones of the view', () => {
    expect(variationFrames(view(['a', 'b', 'c', 'd', 'e'], 4), 2, 2)).toEqual(['d', 'e']);
    expect(variationFrames(view(['a', 'b', 'c', 'd', 'e'], 4), 1, 3)).toEqual(['e']);
  });

  it('are refused when the view is not of the whole path', () => {
    expect(() => variationFrames(view(['a', 'b', 'c', 'd'], 4), 2, 2)).toThrow(
      'could not be shown',
    );
    expect(() => variationFrames(view(['a', 'b', 'c', 'd', 'e'], 3), 2, 2)).toThrow(
      'could not be shown',
    );
    expect(() => variationFrames(view(undefined as unknown as unknown[], 4), 2, 2)).toThrow(
      'could not be shown',
    );
  });

  it('come from the view of the record, which is asked for once', async () => {
    const asked: unknown[] = [];
    const next = await fetchVariation(model, { from: 2, moves: ['0 0', '1 1'], cursor: 1 }, (r) => {
      asked.push(r);
      return Promise.resolve(view(['a', 'b', 'c', 'd', 'e'], 4));
    });
    expect(asked).toEqual([variationRecord(model, 2, ['0 0', '1 1'])]);
    expect(next).toEqual({ from: 2, moves: ['0 0', '1 1'], frames: ['d', 'e'], cursor: 1 });
  });

  it('fail with the message of a refused move', async () => {
    await expect(
      fetchVariation(model, { from: 2, moves: ['9 9'], cursor: 1 }, () =>
        Promise.reject(new Error('the record does not replay: turn 2 is invalid')),
      ),
    ).rejects.toThrow('turn 2 is invalid');
  });
});

describe('the position of a side line', () => {
  it('is the game’s frame at cursor 0, else the frame after the move at the cursor', () => {
    expect(shownFrame(model, side({ cursor: 0 }))).toBe('f1');
    expect(shownFrame(model, side({ cursor: 1 }))).toBe('v1');
    expect(shownFrame(model, side({ cursor: 3 }))).toBe('v3');
  });

  it('asks the engine for the game up to the frame, then the moves up to the cursor', () => {
    expect(shownMoves(model, side({ cursor: 0 }))).toEqual(['4 4']);
    expect(shownMoves(model, side({ cursor: 2 }))).toEqual(['4 4', '0 0', '1 1']);
    expect(shownMoves(opened, side({ from: 0, cursor: 1 }))).toEqual(['0 0', '1 1', '0 0']);
  });

  it('cannot be asked for a game with turns of several answers', () => {
    const together = { ...model, turns: [[...turn(0, 'a'), ...turn(1, 'b')], ...model.turns] };
    expect(shownMoves(together, side())).toBeNull();
  });

  it('moves its cursor within its moves', () => {
    expect(withCursor(side(), 2).cursor).toBe(2);
    expect(withCursor(side(), -1).cursor).toBe(0);
    expect(withCursor(side(), 9).cursor).toBe(3);
    const same = side();
    expect(withCursor(same, 1)).toBe(same);
  });

  it('is stepped by ← → Home End, and not played by Space', () => {
    const at = side({ cursor: 1 });
    expect(variationKey('ArrowLeft', at)).toBe(0);
    expect(variationKey('ArrowRight', at)).toBe(2);
    expect(variationKey('Home', at)).toBe(0);
    expect(variationKey('End', at)).toBe(3);
    expect(variationKey('ArrowLeft', side({ cursor: 0 }))).toBe(0);
    expect(variationKey('ArrowRight', side({ cursor: 3 }))).toBe(3);
    expect(variationKey(' ', at)).toBe('toggle');
    expect(variationKey('a', at)).toBeUndefined();
  });
});

describe('the strip', () => {
  it('numbers the moves from the start of the game, as the engine’s lines are', () => {
    // From frame 1, X to move... no: after 1 turn it is O who moves, so the line starts `1…`.
    expect(stripTokens(model, { from: 1, moves: ['0 0', '1 1', '2 2'] })).toEqual([
      { kind: 'number', text: '1…' },
      { kind: 'move', text: '0 0', index: 0 },
      { kind: 'number', text: '2.' },
      { kind: 'move', text: '1 1', index: 1 },
      { kind: 'move', text: '2 2', index: 2 },
    ]);
    expect(stripTokens(model, { from: 2, moves: ['0 0'] })).toEqual([
      { kind: 'number', text: '2.' },
      { kind: 'move', text: '0 0', index: 0 },
    ]);
  });

  it('counts the opening turns', () => {
    expect(stripTokens(opened, { from: 0, moves: ['0 0'] })).toEqual([
      { kind: 'number', text: '2.' },
      { kind: 'move', text: '0 0', index: 0 },
    ]);
  });
});
