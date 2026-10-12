import { describe, expect, it } from 'vitest';
import type { Answer, Session, SessionStatus } from './api';
import { botsPlaying, unplayedView } from './botGame';

const human = { kind: 'human', name: 'You' } as const;
const bot = (name: string) =>
  ({
    kind: 'bot',
    name,
    release: name,
    think_ms: 100,
    mode: 'realtime',
    fixed_iters: null,
  }) as const;

const turn = (seat: number, line: string): Answer[] => [{ seat, lines: [line], ms: 5 }];

function game(patch: Partial<Session> = {}): Session {
  return {
    id: 'ab',
    game: 'uttt',
    seed: 1,
    opening_plies: 0,
    seats: [bot('a'), bot('b')],
    status: 'bot_thinking',
    error: null,
    to_act: [0],
    opening_turns: 0,
    turns: [turn(0, '4 4'), turn(1, '4 5')],
    frames: ['f0', 'f1', 'f2'],
    human_moves: [],
    result: null,
    progress: null,
    ...patch,
  };
}

describe('unplayedView', () => {
  it('cuts a game the bots are playing to its first frame, with no moves and no result', () => {
    const view = unplayedView(game());
    expect(view.frames).toEqual(['f0']);
    expect(view.turns).toEqual([]);
    expect(view.result).toBeNull();
    expect(view.human_moves).toEqual([]);
    // The rest is the session's: the bot to move, the status, the seats.
    expect(view.status).toBe('bot_thinking');
    expect(view.to_act).toEqual([0]);
    expect(view.seats).toEqual(game().seats);
  });

  it('keeps the turns of the opening', () => {
    const view = unplayedView(
      game({
        opening_turns: 2,
        turns: [turn(0, 'o1'), turn(1, 'o2'), turn(0, '4 4')],
        frames: ['f0', 'f1'],
      }),
    );
    expect(view.frames).toEqual(['f0']);
    expect(view.turns).toEqual([turn(0, 'o1'), turn(1, 'o2')]);
  });

  it('does not change the session it is given', () => {
    const session = game();
    unplayedView(session);
    expect(session.frames).toHaveLength(3);
    expect(session.turns).toHaveLength(2);
  });

  it('gives back the same session for every other status of a game between bots', () => {
    const others: SessionStatus[] = ['over', 'failed', 'compiling', 'measuring', 'rewinding'];
    for (const status of others) {
      const session = game({
        status,
        result: status === 'over' ? { winner: 1, end: { kind: 'finished' } } : null,
        error: status === 'failed' ? 'x' : null,
      });
      expect(unplayedView(session)).toBe(session);
    }
  });

  it('gives back the same session before any frame exists', () => {
    const session = game({ frames: [], turns: [] });
    expect(unplayedView(session)).toBe(session);
  });

  it('gives back the same session for a friend game and a game against the computer', () => {
    const friends = game({ seats: [human, { kind: 'human', name: 'Friend' }] });
    expect(unplayedView(friends)).toBe(friends);
    const computer = game({ seats: [human, bot('b')] });
    expect(unplayedView(computer)).toBe(computer);
    const waiting = game({ seats: [human, bot('b')], status: 'waiting_human' });
    expect(unplayedView(waiting)).toBe(waiting);
  });
});

describe('botsPlaying', () => {
  it('is true only for two bots with frames and a game going on', () => {
    expect(botsPlaying(game())).toBe(true);
    expect(botsPlaying(game({ status: 'over' }))).toBe(false);
    expect(botsPlaying(game({ frames: [] }))).toBe(false);
    expect(botsPlaying(game({ seats: [human, bot('b')] }))).toBe(false);
  });
});
