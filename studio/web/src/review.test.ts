import { describe, expect, it } from 'vitest';
import type { Answer, GameRecord, RecordPlayer, RecordView, Session } from './api';
import { recallFile, rememberFile } from './loadedFiles';
import {
  exportName,
  formatDate,
  lastFrame,
  playbackInterval,
  playbackReducer,
  PLAYBACK_START,
  reviewKey,
  reviewOfRecord,
  reviewOfSession,
  SPEEDS,
  speedLabel,
  type PlaybackState,
} from './review';

const turn = (seat: number, line: string, ms = 5): Answer[] => [{ seat, lines: [line], ms }];

const sessionSeat = {
  kind: 'bot',
  name: 'uttt-v010',
  release: 'uttt-v010',
  think_ms: 100,
  mode: 'fixed',
  fixed_iters: 2500,
} as const;

function session(patch: Partial<Session> = {}): Session {
  return {
    id: 'ab12',
    game: 'uttt',
    seed: 7,
    opening_plies: 0,
    seats: [{ kind: 'human', name: 'You' }, sessionSeat],
    status: 'over',
    error: null,
    to_act: [],
    opening_turns: 0,
    turns: [turn(0, '4 4'), turn(1, '4 5')],
    frames: ['f0', 'f1', 'f2'],
    human_moves: [],
    result: { winner: 1, end: { kind: 'finished' } },
    progress: null,
    ...patch,
  };
}

const botPlayer = (name: string, patch: Partial<RecordPlayer> = {}): RecordPlayer => ({
  name,
  kind: 'bot',
  bot_seed: 1,
  command: name,
  time_scale: 1,
  fixed_iters: null,
  ...patch,
});

function record(patch: Partial<GameRecord> = {}): GameRecord {
  return {
    format: 1,
    game: 'uttt',
    seed: 99,
    opening_plies: 0,
    unix_time: 1_760_000_000,
    source: 'arena match',
    players: [botPlayer('uttt-v010'), botPlayer('uttt-v009', { time_scale: 2 })],
    turns: [turn(0, '4 4'), turn(1, '4 5'), turn(0, '5 5')],
    end: { kind: 'finished' },
    winner: 0,
    ...patch,
  };
}

function view(patch: Partial<RecordView> = {}, recordPatch: Partial<GameRecord> = {}): RecordView {
  return {
    record: record(recordPatch),
    frames: ['f0', 'f1', 'f2', 'f3'],
    opening_turns: 0,
    shown_turns: 3,
    ...patch,
  };
}

describe('reviewOfSession', () => {
  it('turns a finished session into the model', () => {
    const model = reviewOfSession(session());
    expect(model.kind).toBe('session');
    expect(model.id).toBe('ab12');
    expect(model.game).toBe('uttt');
    expect(model.players[0]).toEqual({ name: 'You', kind: 'human', settings: null });
    expect(model.players[1]).toEqual({ name: 'uttt-v010', kind: 'bot', settings: '≈ 100 ms' });
    expect(model.turns).toHaveLength(2);
    expect(model.frames).toHaveLength(3);
    expect(model.result).toEqual({ winner: 1, end: { kind: 'finished' } });
    expect(model.seed).toBe(7);
    expect(model.source).toBe('Played here');
    expect(model.unixTime).toBeNull();
    expect(model.saved).toBe(false);
    expect(model.hiddenTurns).toBe(0);
    expect(model.error).toBeNull();
  });

  it('says real time for a bot that plays on a clock', () => {
    const model = reviewOfSession(
      session({ seats: [{ ...sessionSeat, mode: 'realtime' }, sessionSeat] }),
    );
    expect(model.players[0].settings).toBe('≈ 100 ms, real time');
  });

  it('has no result while the game goes on, and carries the error of a failed one', () => {
    const going = reviewOfSession(session({ status: 'bot_thinking', result: null }));
    expect(going.result).toBeNull();
    const failed = reviewOfSession(session({ status: 'failed', error: 'the bot crashed' }));
    expect(failed.error).toBe('the bot crashed');
  });
});

describe('reviewOfRecord', () => {
  it('turns a saved game into the model', () => {
    const model = reviewOfRecord(view(), { kind: 'saved', id: 'uttt-1-abc' });
    expect(model.kind).toBe('saved');
    expect(model.id).toBe('uttt-1-abc');
    expect(model.players[0]).toEqual({ name: 'uttt-v010', kind: 'bot', settings: null });
    expect(model.players[1].settings).toBe('time × 2');
    expect(model.turns).toHaveLength(3);
    expect(model.frames).toHaveLength(4);
    expect(model.result).toEqual({ winner: 0, end: { kind: 'finished' } });
    expect(model.seed).toBe(99);
    expect(model.source).toBe('Saved (arena match)');
    expect(model.unixTime).toBe(1_760_000_000);
    expect(model.saved).toBe(true);
  });

  it('shows the turns that replay when the last answer of an arena game was invalid', () => {
    const invalid = view(
      { shown_turns: 2, frames: ['f0', 'f1', 'f2'] },
      {
        end: { kind: 'invalid', seat: 0, reason: 'cell taken' },
        winner: 1,
      },
    );
    const model = reviewOfRecord(invalid, { kind: 'saved', id: 'x' });
    expect(model.turns).toHaveLength(2);
    expect(model.hiddenTurns).toBe(1);
    expect(model.result).toEqual({
      winner: 1,
      end: { kind: 'invalid', seat: 0, reason: 'cell taken' },
    });
    expect(lastFrame(model)).toBe(2);
  });

  it('keeps every turn when all of them replay, and never reads past the list', () => {
    expect(reviewOfRecord(view(), { kind: 'saved', id: 'x' }).hiddenTurns).toBe(0);
    const model = reviewOfRecord(view({ shown_turns: 9 }), { kind: 'saved', id: 'x' });
    expect(model.turns).toHaveLength(3);
    expect(model.hiddenTurns).toBe(0);
  });

  it('shows a loaded file under its name, not saved yet', () => {
    const model = reviewOfRecord(view(), { kind: 'file', id: 'f1', name: 'sprt-sample.json' });
    expect(model.kind).toBe('file');
    expect(model.id).toBe('f1');
    expect(model.source).toBe('sprt-sample.json');
    expect(model.saved).toBe(false);
  });

  it('describes iterations for a fixed bot, and nothing for a human', () => {
    const model = reviewOfRecord(
      view(
        {},
        {
          players: [
            botPlayer('uttt-v010', { fixed_iters: 2500 }),
            { ...botPlayer('You'), kind: 'human', command: null, bot_seed: null },
          ],
        },
      ),
      { kind: 'saved', id: 'x' },
    );
    expect(model.players[0].settings).toBe('2500 iterations');
    expect(model.players[1]).toEqual({ name: 'You', kind: 'human', settings: null });
  });

  it('keeps the opening turns of the replay', () => {
    const model = reviewOfRecord(view({ opening_turns: 2 }), { kind: 'saved', id: 'x' });
    expect(model.openingTurns).toBe(2);
  });
});

describe('playbackInterval', () => {
  it('makes the whole game last 120 seconds at 1×', () => {
    expect(playbackInterval(121, 1)).toBe(1000);
    expect(playbackInterval(41, 1)).toBe(3000);
    // 120 steps of 1 s each.
    expect((playbackInterval(121, 1) ?? 0) * 120).toBe(120_000);
  });

  it('divides by the speed', () => {
    expect(playbackInterval(121, 4)).toBe(250);
    expect(playbackInterval(121, 2)).toBe(500);
    expect(playbackInterval(121, 0.5)).toBe(2000);
    expect(playbackInterval(121, 0.25)).toBe(4000);
  });

  it('is null when there is no step to play', () => {
    expect(playbackInterval(0, 1)).toBeNull();
    expect(playbackInterval(1, 1)).toBeNull();
    expect(playbackInterval(5, 0)).toBeNull();
  });

  it('offers the five speeds', () => {
    expect([...SPEEDS]).toEqual([0.25, 0.5, 1, 2, 4]);
    expect(speedLabel(0.25)).toBe('0.25×');
    expect(speedLabel(4)).toBe('4×');
  });
});

describe('playbackReducer', () => {
  const at = (patch: Partial<PlaybackState>): PlaybackState => ({ ...PLAYBACK_START, ...patch });

  it('starts at the first frame, paused, at 1×', () => {
    expect(PLAYBACK_START).toEqual({ cursor: 0, playing: false, speed: 1 });
  });

  it('plays, steps, and stops at the last frame', () => {
    let state = playbackReducer(PLAYBACK_START, { type: 'toggle', last: 2 });
    expect(state.playing).toBe(true);
    state = playbackReducer(state, { type: 'tick', last: 2 });
    expect(state).toEqual({ cursor: 1, playing: true, speed: 1 });
    state = playbackReducer(state, { type: 'tick', last: 2 });
    expect(state).toEqual({ cursor: 2, playing: false, speed: 1 });
    // A late tick does nothing.
    expect(playbackReducer(state, { type: 'tick', last: 2 })).toBe(state);
  });

  it('pauses with the cursor where it is', () => {
    const state = playbackReducer(at({ cursor: 1, playing: true }), { type: 'toggle', last: 3 });
    expect(state).toEqual({ cursor: 1, playing: false, speed: 1 });
  });

  it('starts again from the first frame when played at the end', () => {
    const state = playbackReducer(at({ cursor: 3 }), { type: 'toggle', last: 3 });
    expect(state).toEqual({ cursor: 0, playing: true, speed: 1 });
  });

  it('does not play a game with no step', () => {
    expect(playbackReducer(PLAYBACK_START, { type: 'toggle', last: 0 })).toBe(PLAYBACK_START);
    expect(playbackReducer(PLAYBACK_START, { type: 'toggle', last: -1 })).toBe(PLAYBACK_START);
  });

  it('keeps playing when the speed changes, and keeps the cursor', () => {
    const state = playbackReducer(at({ cursor: 2, playing: true }), { type: 'speed', speed: 4 });
    expect(state).toEqual({ cursor: 2, playing: true, speed: 4 });
    const paused = playbackReducer(at({ cursor: 2 }), { type: 'speed', speed: 0.5 });
    expect(paused).toEqual({ cursor: 2, playing: false, speed: 0.5 });
  });

  it('jumps within the frames and keeps the playing state', () => {
    expect(playbackReducer(at({ playing: true }), { type: 'goto', frame: 2, last: 5 })).toEqual({
      cursor: 2,
      playing: true,
      speed: 1,
    });
    expect(playbackReducer(at({}), { type: 'goto', frame: -3, last: 5 }).cursor).toBe(0);
    expect(playbackReducer(at({}), { type: 'goto', frame: 9, last: 5 }).cursor).toBe(5);
    expect(playbackReducer(at({}), { type: 'goto', frame: 1, last: -1 }).cursor).toBe(0);
  });
});

describe('reviewKey', () => {
  it('steps with the arrows, within the frames', () => {
    expect(reviewKey('ArrowRight', 1, 5)).toBe(2);
    expect(reviewKey('ArrowLeft', 1, 5)).toBe(0);
    expect(reviewKey('ArrowLeft', 0, 5)).toBe(0);
    expect(reviewKey('ArrowRight', 5, 5)).toBe(5);
  });

  it('jumps with Home and End', () => {
    expect(reviewKey('Home', 3, 5)).toBe(0);
    expect(reviewKey('End', 3, 5)).toBe(5);
  });

  it('plays and pauses with Space, and ignores other keys', () => {
    expect(reviewKey(' ', 3, 5)).toBe('toggle');
    expect(reviewKey('a', 3, 5)).toBeUndefined();
    expect(reviewKey('Enter', 3, 5)).toBeUndefined();
  });
});

describe('formatDate and exportName', () => {
  it('writes the local date and minute', () => {
    const local = new Date(2026, 9, 10, 14, 5, 59);
    expect(formatDate(local.getTime() / 1000)).toBe('2026-10-10 14:05');
  });

  it('names an export after the saved id, the file, or the game', () => {
    const now = new Date('2026-10-10T12:00:00Z');
    const base = reviewOfSession(session());
    expect(exportName(base, now)).toBe('uttt-2026-10-10-7.json');
    const saved = reviewOfRecord(view(), { kind: 'saved', id: 'uttt-1-abc' });
    expect(exportName(saved, now)).toBe('uttt-1-abc.json');
    const file = reviewOfRecord(view(), { kind: 'file', id: 'f1', name: 'sample' });
    expect(exportName(file, now)).toBe('sample.json');
    const named = reviewOfRecord(view(), { kind: 'file', id: 'f1', name: 'a.json' });
    expect(exportName(named, now)).toBe('a.json');
  });
});

describe('loaded files', () => {
  it('keeps a file under a generated id, one id per file', () => {
    const first = rememberFile({ name: 'a.json', record: { a: 1 } });
    const second = rememberFile({ name: 'a.json', record: { a: 1 } });
    expect(first).not.toBe(second);
    expect(recallFile(first)).toEqual({ name: 'a.json', record: { a: 1 } });
    expect(recallFile('missing')).toBeNull();
  });
});
