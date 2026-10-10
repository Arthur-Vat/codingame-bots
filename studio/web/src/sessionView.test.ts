import { describe, expect, it } from 'vitest';
import type { Answer, Session } from './api';
import {
  bottomSeat,
  moveRows,
  overlayFor,
  resultText,
  sessionTitle,
  statusText,
} from './sessionView';

const human = { kind: 'human', name: 'You' } as const;
const bot = {
  kind: 'bot',
  name: 'uttt-v010',
  release: 'uttt-v010',
  think_ms: 100,
  mode: 'fixed',
  fixed_iters: null,
} as const;

function session(patch: Partial<Session>): Session {
  return {
    id: 'ab',
    game: 'uttt',
    seed: 1,
    opening_plies: 0,
    seats: [bot, human],
    status: 'waiting_human',
    error: null,
    to_act: [1],
    opening_turns: 0,
    turns: [],
    frames: [],
    human_moves: [],
    result: null,
    progress: null,
    ...patch,
  };
}

const turn = (seat: number, line: string): Answer[] => [{ seat, lines: [line], ms: 0 }];

describe('moveRows', () => {
  it('pairs the turns and numbers the pairs', () => {
    const rows = moveRows([turn(0, '4 4'), turn(1, '4 5'), turn(0, '5 0')]);
    expect(rows).toEqual([
      {
        number: 1,
        moves: [
          { turn: 0, text: '4 4' },
          { turn: 1, text: '4 5' },
        ],
      },
      { number: 2, moves: [{ turn: 2, text: '5 0' }, null] },
    ]);
  });

  it('is empty without turns', () => {
    expect(moveRows([])).toEqual([]);
  });
});

describe('statusText', () => {
  it('names who has to move', () => {
    expect(statusText(session({}))).toBe('You to move (O)');
    expect(statusText(session({ status: 'bot_thinking', to_act: [0] }))).toBe(
      'uttt-v010 is thinking…',
    );
  });

  it('shows the progress of a replay and the error of a failure', () => {
    expect(statusText(session({ status: 'rewinding', progress: { done: 2, total: 5 } }))).toBe(
      'Replaying the bot: 2 of 5',
    );
    expect(statusText(session({ status: 'measuring', progress: { done: 0, total: null } }))).toBe(
      'Measuring the bot’s speed…',
    );
    expect(statusText(session({ status: 'failed', error: 'no rustc' }))).toBe('Failed: no rustc');
    expect(statusText(session({ status: 'over' }))).toBe('Game over');
  });
});

describe('sessionTitle and bottomSeat', () => {
  it('tells the kind of game from the seats', () => {
    expect(sessionTitle(session({}))).toBe('Against the computer');
    expect(sessionTitle(session({ seats: [human, human] }))).toBe('Friend game');
    expect(sessionTitle(session({ seats: [bot, bot] }))).toBe('Bot against bot');
  });

  it('puts the first human at the bottom', () => {
    expect(bottomSeat(session({}))).toBe(1);
    expect(bottomSeat(session({ seats: [bot, bot] }))).toBe(0);
    expect(bottomSeat(null)).toBe(0);
  });
});

describe('resultText', () => {
  const seats = [bot, human] as Session['seats'];

  it('writes the score and the winner', () => {
    const finished = { kind: 'finished' } as const;
    expect(resultText({ winner: 0, end: finished }, seats, null).headline).toBe('1–0 · X wins');
    expect(resultText({ winner: 1, end: finished }, seats, null).headline).toBe('0–1 · O wins');
    expect(resultText({ winner: null, end: finished }, seats, null).headline).toBe('½–½ · Draw');
  });

  it('takes the reason of a finished game from the renderer', () => {
    const result = { winner: 0, end: { kind: 'finished' } } as const;
    expect(resultText(result, seats, 'Small boards won: 4–3').detail).toBe('Small boards won: 4–3');
    expect(resultText(result, seats, null).detail).toBe('');
  });

  it('says out of time for a human and timeout for a bot', () => {
    const human1 = { winner: 0, end: { kind: 'timeout', seat: 1, limit_ms: 0 } } as const;
    const bot0 = { winner: 1, end: { kind: 'timeout', seat: 0, limit_ms: 100 } } as const;
    expect(resultText(human1, seats, null).detail).toBe('Out of time');
    expect(resultText(bot0, seats, null).detail).toBe('Timeout');
  });

  it('names the side that resigned, and the invalid answer and the crash', () => {
    expect(resultText({ winner: 1, end: { kind: 'resigned', seat: 0 } }, seats, null)).toEqual({
      headline: '0–1 · O wins',
      detail: 'X resigned',
    });
    expect(resultText({ winner: 0, end: { kind: 'resigned', seat: 1 } }, seats, null).detail).toBe(
      'O resigned',
    );
    expect(
      resultText({ winner: 1, end: { kind: 'invalid', seat: 0, reason: 'x' } }, seats, null).detail,
    ).toBe('Invalid answer');
    expect(
      resultText({ winner: 1, end: { kind: 'crash', seat: 0, detail: 'x' } }, seats, null).detail,
    ).toBe('Crash');
  });
});

describe('overlayFor', () => {
  it('shows the progress of a rewind', () => {
    expect(
      overlayFor(session({ status: 'rewinding', seed: 42, progress: { done: 3, total: 12 } })),
    ).toEqual({
      kind: 'notice',
      title: 'Rewinding uttt-v010',
      text: 'Replaying 12 moves with seed 42.',
      fraction: 0.25,
    });
  });

  it('restarts from the first move when the bot has nothing to replay', () => {
    expect(overlayFor(session({ status: 'rewinding', progress: { done: 0, total: 0 } }))).toEqual({
      kind: 'notice',
      title: 'Rewinding uttt-v010',
      text: 'Restarting uttt-v010 from the first move.',
      fraction: 0,
    });
  });

  it('says what is compiled and measured', () => {
    expect(overlayFor(session({ status: 'compiling' }))).toMatchObject({
      title: 'Compiling uttt-v010…',
    });
    expect(
      overlayFor(session({ status: 'measuring', progress: { done: 0, total: null } })),
    ).toMatchObject({
      title: "Measuring uttt-v010's speed on this computer…",
      fraction: null,
    });
  });

  it('shows a compact line while two bots play, and nothing in other games', () => {
    const other = { ...bot, name: 'uttt-v009', release: 'uttt-v009' };
    const bots = session({
      seats: [bot, other],
      status: 'bot_thinking',
      to_act: [0],
      turns: [turn(0, '4 4'), turn(1, '4 5')],
    });
    expect(overlayFor(bots)).toEqual({ kind: 'pill', text: 'uttt-v010 vs uttt-v009 · move 3' });
    expect(overlayFor(session({ status: 'bot_thinking', to_act: [0] }))).toBeNull();
    expect(overlayFor(session({}))).toBeNull();
    expect(overlayFor(session({ status: 'over' }))).toBeNull();
  });
});
