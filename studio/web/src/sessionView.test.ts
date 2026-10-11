import { describe, expect, it } from 'vitest';
import type { Answer, Session } from './api';
import { bottomSeat, moveRows, resultText, sessionTitle, statusText } from './sessionView';

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
      { number: 1, moves: ['4 4', '4 5'] },
      { number: 2, moves: ['5 0', null] },
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
  it('writes the score and the way the game ended', () => {
    expect(resultText({ winner: 0, end: { kind: 'finished' } }).headline).toBe('1–0 · X wins');
    expect(resultText({ winner: 1, end: { kind: 'resigned', seat: 0 } })).toEqual({
      headline: '0–1 · O wins',
      detail: 'Resigned',
    });
    expect(resultText({ winner: null, end: { kind: 'finished' } }).headline).toBe('½–½ · Draw');
  });
});
