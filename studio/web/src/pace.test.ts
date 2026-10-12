import { describe, expect, it } from 'vitest';
import type { Answer, Session } from './api';
import { DEFAULT_PACE, PACES, nextPacedCursor, paceDelay, paceLabel, pacedSession } from './pace';

const bot = (name: string) =>
  ({
    kind: 'bot',
    name,
    release: name,
    think_ms: 100,
    mode: 'realtime',
    fixed_iters: null,
  }) as const;
const human = { kind: 'human', name: 'You' } as const;

const turn = (seat: number, line: string): Answer[] => [{ seat, lines: [line], ms: 5 }];

/** A finished game of four turns between two bots, with its five frames. */
function finished(patch: Partial<Session> = {}): Session {
  return {
    id: 'ab',
    game: 'uttt',
    seed: 1,
    opening_plies: 0,
    seats: [bot('a'), bot('b')],
    status: 'over',
    error: null,
    to_act: [],
    opening_turns: 0,
    turns: [turn(0, '4 4'), turn(1, '4 5'), turn(0, '5 3'), turn(1, '3 7')],
    frames: ['f0', 'f1', 'f2', 'f3', 'f4'],
    human_moves: [],
    result: { winner: 1, end: { kind: 'finished' } },
    progress: null,
    ...patch,
  };
}

describe('paces', () => {
  it('are the five the bar offers, 0.5 s by default', () => {
    expect(PACES).toEqual([250, 500, 1000, 2000, null]);
    expect(DEFAULT_PACE).toBe(500);
    expect(PACES.map(paceLabel)).toEqual(['0.25 s', '0.5 s', '1 s', '2 s', 'Max']);
  });
});

describe('pacedSession', () => {
  it('is the session as it looked when the frame was its latest', () => {
    const shown = pacedSession(finished(), 2);
    expect(shown.frames).toEqual(['f0', 'f1', 'f2']);
    expect(shown.turns).toEqual([turn(0, '4 4'), turn(1, '4 5')]);
    expect(shown.status).toBe('bot_thinking');
    expect(shown.result).toBeNull();
    expect(shown.human_moves).toEqual([]);
    expect(shown.error).toBeNull();
    // The next turn is seat 0's.
    expect(shown.to_act).toEqual([0]);
    expect(pacedSession(finished(), 3).to_act).toEqual([1]);
  });

  it('shows the first frame with no turn and the first bot to move', () => {
    const shown = pacedSession(finished(), 0);
    expect(shown.frames).toEqual(['f0']);
    expect(shown.turns).toEqual([]);
    expect(shown.to_act).toEqual([0]);
  });

  it('clamps a negative cursor to the first frame', () => {
    expect(pacedSession(finished(), -3).frames).toEqual(['f0']);
  });

  it('counts the opening turns, which no frame ends with', () => {
    const opening = finished({
      opening_turns: 2,
      turns: [turn(0, 'o1'), turn(1, 'o2'), turn(0, '4 4'), turn(1, '4 5')],
      frames: ['f0', 'f1', 'f2'],
    });
    const shown = pacedSession(opening, 1);
    expect(shown.frames).toEqual(['f0', 'f1']);
    expect(shown.turns).toEqual([turn(0, 'o1'), turn(1, 'o2'), turn(0, '4 4')]);
    expect(shown.to_act).toEqual([1]);
    expect(pacedSession(opening, 0).turns).toHaveLength(2);
  });

  it('keeps the real to_act when the next turn is not known yet', () => {
    const running = finished({
      status: 'bot_thinking',
      result: null,
      to_act: [1],
      turns: [turn(0, '4 4'), turn(1, '4 5')],
      frames: ['f0', 'f1', 'f2', 'f3'],
    });
    expect(pacedSession(running, 2).to_act).toEqual([1]);
  });

  it('gives back the same session once the cursor is at the latest frame, or past it', () => {
    const session = finished();
    expect(pacedSession(session, 4)).toBe(session);
    expect(pacedSession(session, 9)).toBe(session);
    const running = finished({ status: 'bot_thinking', result: null });
    expect(pacedSession(running, 4)).toBe(running);
  });

  it('gives back the same session when it is not a game between two bots', () => {
    const friends = finished({ seats: [human, { kind: 'human', name: 'Friend' }] });
    expect(pacedSession(friends, 1)).toBe(friends);
    const computer = finished({ seats: [human, bot('b')] });
    expect(pacedSession(computer, 1)).toBe(computer);
  });

  it('gives back the same session while it fails, compiles, measures or rewinds', () => {
    for (const status of ['failed', 'compiling', 'measuring', 'rewinding'] as const) {
      const session = finished({ status, result: null, error: status === 'failed' ? 'x' : null });
      expect(pacedSession(session, 1)).toBe(session);
    }
  });

  it('does not change the session it is given', () => {
    const session = finished();
    pacedSession(session, 1);
    expect(session.frames).toHaveLength(5);
    expect(session.status).toBe('over');
  });
});

describe('nextPacedCursor', () => {
  it('steps one frame, never past the latest', () => {
    expect(nextPacedCursor(0, 4, 500)).toBe(1);
    expect(nextPacedCursor(3, 4, 500)).toBe(4);
    expect(nextPacedCursor(4, 4, 500)).toBe(4);
    expect(nextPacedCursor(0, -1, 500)).toBe(0);
  });

  it('is the latest frame with Max', () => {
    expect(nextPacedCursor(0, 7, null)).toBe(7);
    expect(nextPacedCursor(7, 7, null)).toBe(7);
    expect(nextPacedCursor(0, -1, null)).toBe(0);
  });
});

describe('paceDelay', () => {
  it('waits the pace from the previous frame, not from the arrival of the next', () => {
    expect(paceDelay(500, 1000, 1000)).toBe(500);
    expect(paceDelay(500, 1000, 1200)).toBe(300);
  });

  it('is zero once that moment is past, as when a slow bot is late', () => {
    expect(paceDelay(500, 1000, 1500)).toBe(0);
    expect(paceDelay(500, 1000, 9000)).toBe(0);
  });

  it('takes a changed pace from the same step: a shorter one may already be past', () => {
    expect(paceDelay(2000, 1000, 1700)).toBe(1300);
    expect(paceDelay(250, 1000, 1700)).toBe(0);
  });

  it('is zero for Max', () => {
    expect(paceDelay(null, 1000, 1000)).toBe(0);
  });
});
