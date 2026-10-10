import { describe, expect, it } from 'vitest';
import type { Answer, Session } from './api';
import {
  canPlay,
  controlsFor,
  cursorAfterKey,
  deepEqual,
  exportFileName,
  findMoveIndex,
  frameAfterTurn,
  rematchRequest,
  runningSeat,
  sessionMode,
  takebackKeep,
  timeUsed,
  turnOfFrame,
  turnSeats,
} from './play';

const human = { kind: 'human', name: 'You' } as const;
const friend = { kind: 'human', name: 'Friend' } as const;
const bot = {
  kind: 'bot',
  name: 'uttt-v010',
  release: 'uttt-v010',
  think_ms: 100,
  mode: 'fixed',
  fixed_iters: 2500,
} as const;
const botRealtime = { ...bot, name: 'uttt-v009', release: 'uttt-v009', mode: 'realtime' } as const;

const turn = (seat: number, line = '0 0', ms = 0): Answer[] => [{ seat, lines: [line], ms }];

function session(patch: Partial<Session>): Session {
  return {
    id: 'ab',
    game: 'uttt',
    seed: 7,
    opening_plies: 0,
    seats: [human, friend],
    status: 'waiting_human',
    error: null,
    to_act: [0],
    opening_turns: 0,
    turns: [],
    frames: [],
    human_moves: [],
    result: null,
    progress: null,
    ...patch,
  };
}

describe('findMoveIndex', () => {
  const moves = [
    { action: { row: 3, col: 3 }, lines: ['3 3'] },
    { action: { row: 3, col: 4 }, lines: ['3 4'] },
  ];

  it('finds the entry whose action equals the clicked one', () => {
    expect(findMoveIndex(moves, { row: 3, col: 4 })).toBe(1);
    expect(findMoveIndex(moves, { col: 3, row: 3 })).toBe(0);
  });

  it('is -1 for an action that is not offered', () => {
    expect(findMoveIndex(moves, { row: 0, col: 0 })).toBe(-1);
    expect(findMoveIndex(moves, { row: 3, col: 3, extra: 1 })).toBe(-1);
    expect(findMoveIndex([], { row: 3, col: 3 })).toBe(-1);
    expect(findMoveIndex([null, 4], 4)).toBe(-1);
  });
});

describe('deepEqual', () => {
  it('compares nested JSON', () => {
    expect(deepEqual({ a: [1, { b: 2 }] }, { a: [1, { b: 2 }] })).toBe(true);
    expect(deepEqual({ a: [1, 2] }, { a: [2, 1] })).toBe(false);
    expect(deepEqual([1], { 0: 1 })).toBe(false);
    expect(deepEqual(null, {})).toBe(false);
  });
});

describe('takebackKeep', () => {
  it('keeps all but the last turn in a friend game', () => {
    expect(takebackKeep(session({ turns: [turn(0), turn(1), turn(0)] }))).toBe(2);
    expect(takebackKeep(session({ turns: [turn(0)] }))).toBe(0);
  });

  it('has nothing to take back at the start', () => {
    expect(takebackKeep(session({ turns: [] }))).toBeNull();
  });

  it('removes the bot reply too against the computer', () => {
    // The human is X (seat 0) and moved first: X O X O, X to move.
    const turns = [turn(0), turn(1), turn(0), turn(1)];
    expect(takebackKeep(session({ seats: [human, bot], turns }))).toBe(2);
  });

  it('works when the bot moved first', () => {
    // The bot is X: X(bot) O(human) X(bot), the human to move.
    const turns = [turn(0), turn(1), turn(0)];
    expect(takebackKeep(session({ seats: [bot, human], to_act: [1], turns }))).toBe(1);
  });

  it('has nothing to take back before the human moved', () => {
    expect(
      takebackKeep(session({ seats: [bot, human], to_act: [1], turns: [turn(0)] })),
    ).toBeNull();
    expect(takebackKeep(session({ seats: [human, bot], turns: [] }))).toBeNull();
  });

  it('has no takeback for bot against bot', () => {
    expect(
      takebackKeep(session({ seats: [bot, botRealtime], turns: [turn(0), turn(1)] })),
    ).toBeNull();
  });

  it('never goes below the opening turns', () => {
    expect(takebackKeep(session({ opening_turns: 1, turns: [turn(0)] }))).toBeNull();
    expect(takebackKeep(session({ opening_turns: 1, turns: [turn(0), turn(1)] }))).toBe(1);
  });
});

describe('controlsFor', () => {
  const played = [turn(0), turn(1)];

  it('lets the side to move resign in a friend game', () => {
    expect(controlsFor(session({ turns: played }), false)).toEqual({
      takebackTo: 1,
      resignSeat: 0,
    });
    expect(controlsFor(session({ turns: played, to_act: [1] }), false).resignSeat).toBe(1);
  });

  it('lets the human resign against the computer, also while the bot thinks', () => {
    const base = { seats: [bot, human] as Session['seats'], turns: [turn(0), turn(1), turn(0)] };
    expect(controlsFor(session({ ...base, to_act: [1] }), false)).toEqual({
      takebackTo: 1,
      resignSeat: 1,
    });
    expect(controlsFor(session({ ...base, status: 'bot_thinking', to_act: [0] }), false)).toEqual({
      takebackTo: null,
      resignSeat: 1,
    });
  });

  it('is disabled while busy, rewinding, over, or bot against bot', () => {
    const turns = played;
    const off = { takebackTo: null, resignSeat: null };
    expect(controlsFor(session({ turns }), true)).toEqual(off);
    expect(controlsFor(session({ turns, status: 'rewinding' }), false)).toEqual(off);
    expect(controlsFor(session({ turns, status: 'over' }), false)).toEqual(off);
    expect(
      controlsFor(session({ seats: [bot, botRealtime], status: 'bot_thinking' }), false),
    ).toEqual(off);
    expect(controlsFor(null, false)).toEqual(off);
  });
});

describe('runningSeat and canPlay', () => {
  it('runs only for the human to move, with nothing in flight', () => {
    expect(runningSeat(session({}), false)).toBe(0);
    expect(runningSeat(session({}), true)).toBeNull();
    expect(runningSeat(session({ status: 'bot_thinking' }), false)).toBeNull();
    expect(runningSeat(session({ status: 'rewinding' }), false)).toBeNull();
    expect(runningSeat(session({ status: 'compiling' }), false)).toBeNull();
    expect(runningSeat(session({ status: 'measuring' }), false)).toBeNull();
    expect(runningSeat(session({ status: 'over' }), false)).toBeNull();
    expect(runningSeat(session({ seats: [bot, human], to_act: [0] }), false)).toBeNull();
    expect(runningSeat(null, false)).toBeNull();
  });

  it('accepts input on the latest frame only, when a human is to move', () => {
    const moves = [{ action: { row: 0, col: 0 }, lines: ['0 0'] }];
    const s = session({ human_moves: moves });
    expect(canPlay(s, false, true)).toBe(true);
    expect(canPlay(s, false, false)).toBe(false);
    expect(canPlay(s, true, true)).toBe(false);
    expect(canPlay(session({ human_moves: moves, status: 'bot_thinking' }), false, true)).toBe(
      false,
    );
    expect(canPlay(session({}), false, true)).toBe(false);
  });
});

describe('rematchRequest', () => {
  it('keeps a friend game as it was', () => {
    expect(rematchRequest(session({}), 99)).toEqual({
      game: 'uttt',
      seed: 99,
      opening_plies: 0,
      seats: [
        { kind: 'human', name: 'You' },
        { kind: 'human', name: 'Friend' },
      ],
    });
  });

  it('swaps the sides against the computer', () => {
    const request = rematchRequest(session({ seats: [human, bot] }), 5);
    expect(request.seats).toEqual([
      { kind: 'bot', release: 'uttt-v010', think_ms: 100, mode: 'fixed' },
      { kind: 'human', name: 'You' },
    ]);
  });

  it('keeps the sides of two bots', () => {
    const request = rematchRequest(session({ seats: [bot, botRealtime] }), 5);
    expect(request.seats.map((seat) => (seat.kind === 'bot' ? seat.release : ''))).toEqual([
      'uttt-v010',
      'uttt-v009',
    ]);
  });
});

describe('small helpers', () => {
  it('names the mode of a session', () => {
    expect(sessionMode(session({}))).toBe('friend');
    expect(sessionMode(session({ seats: [human, bot] }))).toBe('computer');
    expect(sessionMode(session({ seats: [bot, bot] }))).toBe('bots');
  });

  it('lists the seat of each turn', () => {
    expect(turnSeats([turn(0), turn(1), []])).toEqual([0, 1, null]);
  });

  it('adds up the time a seat used', () => {
    const s = session({ turns: [turn(0, '1 1', 10), turn(1, '2 2', 5), turn(0, '3 3', 20)] });
    expect(timeUsed(s, 0)).toBe(30);
    expect(timeUsed(s, 1)).toBe(5);
  });

  it('names the file of an export', () => {
    expect(exportFileName('uttt', 123, new Date('2026-10-10T23:59:00Z'))).toBe(
      'uttt-2026-10-10-123.json',
    );
  });

  it('maps turns to frames and back, past an opening', () => {
    expect(frameAfterTurn(0, 0)).toBe(1);
    expect(frameAfterTurn(3, 0)).toBe(4);
    expect(frameAfterTurn(0, 2)).toBe(0);
    expect(frameAfterTurn(2, 2)).toBe(1);
    expect(turnOfFrame(0, 0)).toBe(-1);
    expect(turnOfFrame(4, 0)).toBe(3);
    expect(turnOfFrame(1, 2)).toBe(2);
  });
});

describe('cursorAfterKey', () => {
  it('steps back and forward, and follows the latest frame at its end', () => {
    expect(cursorAfterKey('ArrowLeft', 5, 5)).toBe(4);
    expect(cursorAfterKey('ArrowLeft', 0, 5)).toBe(0);
    expect(cursorAfterKey('ArrowRight', 3, 5)).toBe(4);
    expect(cursorAfterKey('ArrowRight', 4, 5)).toBeNull();
    expect(cursorAfterKey('ArrowRight', 5, 5)).toBeNull();
  });

  it('jumps to the first and the latest frame', () => {
    expect(cursorAfterKey('Home', 3, 5)).toBe(0);
    expect(cursorAfterKey('End', 3, 5)).toBeNull();
    expect(cursorAfterKey('Home', 0, 0)).toBeNull();
  });

  it('ignores other keys', () => {
    expect(cursorAfterKey('a', 3, 5)).toBeUndefined();
  });
});
