import { describe, expect, it } from 'vitest';
import type { Answer, GameRecord, RecordPlayer, RecordView } from './api';
import { recallFile, rememberFile } from './loadedFiles';
import {
  exportName,
  formatDate,
  lastFrame,
  NO_PLAYBACK,
  playbackFor,
  playbackInterval,
  playbackReducer,
  PLAYBACK_START,
  retime,
  analysisKey,
  analysisOfRecord,
  shownCursor,
  SPEEDS,
  speedLabel,
  type PlaybackState,
} from './analysis';

const turn = (seat: number, line: string, ms = 5): Answer[] => [{ seat, lines: [line], ms }];

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

describe('analysisOfRecord', () => {
  it('turns a saved game into the model', () => {
    const model = analysisOfRecord(view(), { kind: 'saved', id: 'uttt-1-abc' });
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
    const model = analysisOfRecord(invalid, { kind: 'saved', id: 'x' });
    expect(model.turns).toHaveLength(2);
    expect(model.hiddenTurns).toBe(1);
    expect(model.result).toEqual({
      winner: 1,
      end: { kind: 'invalid', seat: 0, reason: 'cell taken' },
    });
    expect(lastFrame(model)).toBe(2);
  });

  it('keeps every turn when all of them replay, and never reads past the list', () => {
    expect(analysisOfRecord(view(), { kind: 'saved', id: 'x' }).hiddenTurns).toBe(0);
    const model = analysisOfRecord(view({ shown_turns: 9 }), { kind: 'saved', id: 'x' });
    expect(model.turns).toHaveLength(3);
    expect(model.hiddenTurns).toBe(0);
  });

  it('shows a loaded file under its name, not saved yet', () => {
    const model = analysisOfRecord(view(), { kind: 'file', id: 'f1', name: 'sprt-sample.json' });
    expect(model.kind).toBe('file');
    expect(model.id).toBe('f1');
    expect(model.source).toBe('sprt-sample.json');
    expect(model.saved).toBe(false);
  });

  it('shows an arena game as not saved, from the arena', () => {
    const model = analysisOfRecord(view(), { kind: 'arena', id: 'uttt-1-abc' });
    expect(model.kind).toBe('arena');
    expect(model.id).toBe('uttt-1-abc');
    expect(model.source).toBe('Arena game (arena match)');
    expect(model.saved).toBe(false);
  });

  it('shows a game played here as played here, not saved yet', () => {
    const model = analysisOfRecord(view(), { kind: 'session', id: 'ab12' });
    expect(model.kind).toBe('session');
    expect(model.id).toBe('ab12');
    expect(model.source).toBe('Played here');
    expect(model.saved).toBe(false);
  });

  it('describes iterations for a fixed bot, and nothing for a human', () => {
    const model = analysisOfRecord(
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
    const model = analysisOfRecord(view({ opening_turns: 2 }), { kind: 'saved', id: 'x' });
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

describe('retime', () => {
  it('keeps the delay in use while the wanted one is close to it', () => {
    // A live game gains a frame: 120 s over 40 steps, then over 41.
    expect(retime(3000, playbackInterval(41, 1))).toBe(3000);
    expect(retime(3000, 2926)).toBe(3000);
  });

  it('takes the wanted delay when it differs by more than a quarter, as a new speed does', () => {
    expect(retime(3000, 750)).toBe(750);
    expect(retime(3000, 6000)).toBe(6000);
    expect(retime(1000, 1300)).toBe(1300);
    expect(retime(1000, 1250)).toBe(1000);
  });

  it('follows a delay that appears or goes away', () => {
    expect(retime(null, 1000)).toBe(1000);
    expect(retime(1000, null)).toBeNull();
    expect(retime(null, null)).toBeNull();
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

  it('stops playing when a jump lands on the last frame', () => {
    expect(
      playbackReducer(at({ cursor: 1, playing: true }), { type: 'goto', frame: 5, last: 5 }),
    ).toEqual({
      cursor: 5,
      playing: false,
      speed: 1,
    });
    // A jump past the end lands on the last frame too.
    expect(
      playbackReducer(at({ playing: true }), { type: 'goto', frame: 99, last: 5 }).playing,
    ).toBe(false);
    expect(
      playbackReducer(at({ playing: true }), { type: 'goto', frame: 4, last: 5 }).playing,
    ).toBe(true);
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

describe('playbackFor', () => {
  it('starts a new playback on the frame that was on the board', () => {
    const fresh = playbackFor(NO_PLAYBACK, 'ab12', 7);
    expect(fresh).toEqual({ key: 'ab12', state: { cursor: 7, playing: false, speed: 1 } });
  });

  it('keeps the playback held for the same game, so a poll does not move the cursor', () => {
    const held = playbackFor(NO_PLAYBACK, 'ab12', 7);
    const moved = { key: 'ab12', state: { ...held.state, cursor: 2, playing: true, speed: 4 } };
    expect(playbackFor(moved, 'ab12', 9)).toBe(moved);
  });

  it('starts again for another game, and when the game goes on again (key null)', () => {
    const moved = { key: 'ab12', state: { cursor: 2, playing: true, speed: 4 } };
    expect(playbackFor(moved, 'cd34', 5).state).toEqual({ cursor: 5, playing: false, speed: 1 });
    expect(playbackFor(moved, null, 5)).toEqual({
      key: null,
      state: { cursor: 5, playing: false, speed: 1 },
    });
  });

  it('never starts before the first frame', () => {
    expect(playbackFor(NO_PLAYBACK, 'x', -3).state.cursor).toBe(0);
  });

  it('plays from the last frame by starting over, as the analysis board does', () => {
    const atEnd = playbackFor(NO_PLAYBACK, 'x', 4).state;
    expect(playbackReducer(atEnd, { type: 'toggle', last: 4 })).toEqual({
      cursor: 0,
      playing: true,
      speed: 1,
    });
  });
});

describe('shownCursor', () => {
  it('keeps the cursor within the frames there are', () => {
    expect(shownCursor({ cursor: 3, playing: false, speed: 1 }, 5)).toBe(3);
    expect(shownCursor({ cursor: 9, playing: false, speed: 1 }, 5)).toBe(5);
    expect(shownCursor({ cursor: 2, playing: false, speed: 1 }, -1)).toBe(0);
  });
});

describe('analysisKey', () => {
  it('steps with the arrows, within the frames', () => {
    expect(analysisKey('ArrowRight', 1, 5)).toBe(2);
    expect(analysisKey('ArrowLeft', 1, 5)).toBe(0);
    expect(analysisKey('ArrowLeft', 0, 5)).toBe(0);
    expect(analysisKey('ArrowRight', 5, 5)).toBe(5);
  });

  it('jumps with Home and End', () => {
    expect(analysisKey('Home', 3, 5)).toBe(0);
    expect(analysisKey('End', 3, 5)).toBe(5);
  });

  it('plays and pauses with Space, and ignores other keys', () => {
    expect(analysisKey(' ', 3, 5)).toBe('toggle');
    expect(analysisKey('a', 3, 5)).toBeUndefined();
    expect(analysisKey('Enter', 3, 5)).toBeUndefined();
  });
});

describe('formatDate and exportName', () => {
  it('writes the local date and minute', () => {
    const local = new Date(2026, 9, 10, 14, 5, 59);
    expect(formatDate(local.getTime() / 1000)).toBe('2026-10-10 14:05');
  });

  it('names an export after the saved or arena id, or the file', () => {
    const saved = analysisOfRecord(view(), { kind: 'saved', id: 'uttt-1-abc' });
    expect(exportName(saved)).toBe('uttt-1-abc.json');
    const arena = analysisOfRecord(view(), { kind: 'arena', id: 'uttt-2-def' });
    expect(exportName(arena)).toBe('uttt-2-def.json');
    const file = analysisOfRecord(view(), { kind: 'file', id: 'f1', name: 'sample' });
    expect(exportName(file)).toBe('sample.json');
    const named = analysisOfRecord(view(), { kind: 'file', id: 'f1', name: 'a.json' });
    expect(exportName(named)).toBe('a.json');
    // A game played here is named as the Play tab names it: the game, the UTC day, the seed.
    const played = analysisOfRecord(view(), { kind: 'session', id: 'ab12' });
    expect(exportName(played, new Date('2026-10-12T10:00:00Z'))).toBe('uttt-2026-10-12-99.json');
  });
});

describe('loaded files', () => {
  it('keeps a file under a generated id, one id per file', () => {
    const first = rememberFile({ name: 'a.json', record: { a: 1 } });
    const second = rememberFile({ name: 'a.json', record: { a: 1 } });
    expect(first).not.toBe(second);
    // Not a counter that starts again after a reload.
    expect(first).toMatch(/^f[0-9a-z]{8,}$/);
    expect(recallFile(first)).toEqual({ name: 'a.json', record: { a: 1 } });
    expect(recallFile('missing')).toBeNull();
  });
});
