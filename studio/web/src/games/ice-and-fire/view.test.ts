import { describe, expect, it } from 'vitest';
import {
  answerText,
  arrow,
  asIceAndFireFrame,
  boardLabel,
  emptyFrame,
  hatchPath,
  inactiveCells,
  resultDetail,
  signed,
  toMoveText,
  turnText,
  type IceAndFireFrame,
} from './view';
import { sample } from './sample';

const copy = (): Record<string, unknown> =>
  JSON.parse(JSON.stringify(sample())) as Record<string, unknown>;

describe('asIceAndFireFrame', () => {
  it('accepts a frame, the start, an empty one and a finished game', () => {
    expect(asIceAndFireFrame(copy())).not.toBeNull();
    expect(asIceAndFireFrame({ ...copy(), last: null })).not.toBeNull();
    expect(asIceAndFireFrame(emptyFrame())).not.toBeNull();
    const over = { ...copy(), to_move: null, result: { winner: 0, end: 'hq' } };
    expect(asIceAndFireFrame(over)).not.toBeNull();
    const draw = { ...copy(), turn: 100, to_move: null, result: { winner: null, end: 'turns' } };
    expect(asIceAndFireFrame(draw)).not.toBeNull();
  });

  it('accepts as many units as the map has cells', () => {
    const frame = copy();
    frame['units'] = Array.from({ length: 144 }, (_, id) => ({
      id,
      owner: id % 2,
      level: 1 + (id % 3),
      x: id % 12,
      y: Math.floor(id / 12),
    }));
    expect(asIceAndFireFrame(frame)).not.toBeNull();
  });

  it('rejects what is no object', () => {
    for (const value of [null, undefined, 3, 'frame', [], true, () => 1]) {
      expect(asIceAndFireFrame(value), String(value)).toBeNull();
    }
    expect(asIceAndFireFrame({})).toBeNull();
    expect(asIceAndFireFrame({ cells: [], small: [], playable: [] })).toBeNull();
  });

  it('rejects a grid of the wrong size or content', () => {
    for (const key of ['void', 'owner', 'active']) {
      const short = copy();
      (short[key] as unknown[]).pop();
      expect(asIceAndFireFrame(short), `${key} short`).toBeNull();
      const narrow = copy();
      (narrow[key] as unknown[][])[3]!.pop();
      expect(asIceAndFireFrame(narrow), `${key} narrow`).toBeNull();
      const notRows = copy();
      (notRows[key] as unknown[])[0] = 'row';
      expect(asIceAndFireFrame(notRows), `${key} row`).toBeNull();
      const gone = copy();
      delete gone[key];
      expect(asIceAndFireFrame(gone), `${key} missing`).toBeNull();
    }
    const voidNumbers = copy();
    (voidNumbers['void'] as unknown[][])[0]![0] = 1;
    expect(asIceAndFireFrame(voidNumbers)).toBeNull();
    const owner2 = copy();
    (owner2['owner'] as unknown[][])[0]![0] = 2;
    expect(asIceAndFireFrame(owner2)).toBeNull();
    const ownerText = copy();
    (ownerText['owner'] as unknown[][])[0]![0] = '0';
    expect(asIceAndFireFrame(ownerText)).toBeNull();
  });

  it('rejects coordinates that are not whole numbers inside the map', () => {
    const bad = [12, -1, 1.5, '3', null, Number.NaN, Infinity];
    for (const value of bad) {
      const unit = copy();
      (unit['units'] as Record<string, unknown>[])[0]!['x'] = value;
      expect(asIceAndFireFrame(unit), `unit x ${String(value)}`).toBeNull();
      const building = copy();
      (building['buildings'] as Record<string, unknown>[])[0]!['y'] = value;
      expect(asIceAndFireFrame(building), `building y ${String(value)}`).toBeNull();
      const spot = copy();
      (spot['mine_spots'] as unknown[])[0] = [value, 0];
      expect(asIceAndFireFrame(spot), `spot ${String(value)}`).toBeNull();
      const last = copy();
      (last['last'] as { moved: { to: unknown[] }[] }).moved[0]!.to = [0, value];
      expect(asIceAndFireFrame(last), `move ${String(value)}`).toBeNull();
    }
    const spot3 = copy();
    (spot3['mine_spots'] as unknown[])[0] = [1, 2, 3];
    expect(asIceAndFireFrame(spot3)).toBeNull();
  });

  it('rejects units and buildings that are malformed', () => {
    const level = copy();
    (level['units'] as Record<string, unknown>[])[0]!['level'] = 4;
    expect(asIceAndFireFrame(level)).toBeNull();
    const owner = copy();
    (owner['units'] as Record<string, unknown>[])[0]!['owner'] = 2;
    expect(asIceAndFireFrame(owner)).toBeNull();
    const id = copy();
    (id['units'] as Record<string, unknown>[])[0]!['id'] = 'a';
    expect(asIceAndFireFrame(id)).toBeNull();
    const kind = copy();
    (kind['buildings'] as Record<string, unknown>[])[0]!['kind'] = 'castle';
    expect(asIceAndFireFrame(kind)).toBeNull();
    const notList = copy();
    notList['units'] = { length: 0 };
    expect(asIceAndFireFrame(notList)).toBeNull();
    const withNull = copy();
    (withNull['units'] as unknown[]).push(null);
    expect(asIceAndFireFrame(withNull)).toBeNull();
  });

  it('rejects more items than the map has cells', () => {
    const frame = copy();
    frame['mine_spots'] = Array.from({ length: 145 }, () => [0, 0]);
    expect(asIceAndFireFrame(frame)).toBeNull();
  });

  it('rejects numbers that are not whole, and pairs of the wrong length', () => {
    for (const key of ['gold', 'income', 'military']) {
      for (const value of [[1], [1, 2, 3], [1.5, 2], ['1', 2], [null, 2], 5, null]) {
        const frame = copy();
        frame[key] = value;
        expect(asIceAndFireFrame(frame), `${key} ${JSON.stringify(value)}`).toBeNull();
      }
    }
  });

  it('rejects a turn outside 1..100, a bad seat to move and a bad result', () => {
    for (const turn of [0, 101, 1.5, '4', null, Number.NaN]) {
      expect(asIceAndFireFrame({ ...copy(), turn }), String(turn)).toBeNull();
    }
    for (const to_move of [2, '0', undefined, -1]) {
      expect(asIceAndFireFrame({ ...copy(), to_move }), String(to_move)).toBeNull();
    }
    for (const result of [
      {},
      { winner: 0 },
      { end: 'hq' },
      { winner: 2, end: 'hq' },
      { winner: 0, end: 'gold' },
      'over',
      undefined,
    ]) {
      expect(asIceAndFireFrame({ ...copy(), result }), JSON.stringify(result)).toBeNull();
    }
  });

  it('rejects a last turn that is malformed', () => {
    const base = sample().last!;
    for (const last of [
      undefined,
      'x',
      { ...base, seat: 2 },
      { ...base, answer: 3 },
      { ...base, moved: null },
      { ...base, moved: [{ id: 1, from: [0, 0] }] },
      { ...base, trained: [{ id: 1, level: 0, x: 0, y: 0 }] },
      { ...base, built: [{ kind: 'hq', x: 0, y: 0 }] },
      { ...base, removed: [{ id: 1, owner: 0, level: 1, x: 12, y: 0 }] },
    ]) {
      expect(asIceAndFireFrame({ ...copy(), last }), JSON.stringify(last)).toBeNull();
    }
  });
});

describe('the texts', () => {
  it('writes the turn, whose move it is and the status line', () => {
    const frame = sample();
    expect(turnText(frame)).toBe('turn 4 / 100');
    expect(toMoveText(frame)).toBe('Blue to move');
    expect(toMoveText({ ...frame, to_move: 0 })).toBe('Red to move');
    expect(toMoveText({ ...frame, to_move: null, result: { winner: 0, end: 'hq' } })).toBe(
      'Game over',
    );
    expect(toMoveText(emptyFrame())).toBe('');
  });

  it('signs a number', () => {
    expect([signed(5), signed(0), signed(-3)]).toEqual(['+5', '+0', '-3']);
  });

  it('writes the last answer with its seat, and nothing at the start', () => {
    expect(answerText(sample())).toBe('Red: TRAIN 1 1 0;MOVE 1 1 0');
    expect(answerText({ ...sample(), last: null })).toBe('');
    const last = { ...sample().last!, seat: 1 as const, answer: '  ' };
    expect(answerText({ ...sample(), last })).toBe('Blue: (empty answer)');
  });

  it('describes the board for a screen reader', () => {
    expect(boardLabel(sample())).toBe(
      'A Code of Ice and Fire board, turn 4 / 100, Blue to move, gold 12 red, 7 blue, military power 1 red, 3 blue',
    );
  });
});

describe('resultDetail', () => {
  const finished = (winner: 0 | 1 | null, end: 'hq' | 'turns'): IceAndFireFrame => ({
    ...sample(),
    to_move: null,
    result: { winner, end },
  });

  it('says who captured whose HQ', () => {
    expect(resultDetail(finished(0, 'hq'))).toBe("Red captured Blue's HQ.");
    expect(resultDetail(finished(1, 'hq'))).toBe("Blue captured Red's HQ.");
  });

  it('says who had more military power after 100 turns, or that it was equal', () => {
    expect(resultDetail(finished(1, 'turns'))).toBe(
      'Blue had more military power after 100 turns.',
    );
    expect(resultDetail(finished(0, 'turns'))).toBe('Red had more military power after 100 turns.');
    expect(resultDetail(finished(null, 'turns'))).toBe('Equal military power after 100 turns.');
  });

  it('has nothing to say while the game is on, or for a capture without a winner', () => {
    expect(resultDetail(sample())).toBeNull();
    expect(resultDetail(finished(null, 'hq'))).toBeNull();
  });
});

describe('the geometry', () => {
  it('draws an arrow between two cell centres, off the disc, with a head at the target', () => {
    const a = arrow([0, 0], [3, 0])!;
    expect(a.y1).toBeCloseTo(0.5);
    expect(a.y2).toBeCloseTo(0.5);
    expect(a.x1).toBeGreaterThan(0.5);
    expect(a.x2).toBeLessThan(3.5);
    expect(a.x2).toBeGreaterThan(a.x1);
    expect(a.head.split(' ')).toHaveLength(3);
    expect(arrow([2, 2], [2, 2])).toBeNull();
  });

  it('draws an arrow along a diagonal and upward', () => {
    const a = arrow([4, 4], [2, 2])!;
    expect(a.x2).toBeLessThan(a.x1);
    expect(a.y2).toBeLessThan(a.y1);
    expect(a.head).not.toContain('NaN');
  });

  it('finds the owned cells that are cut off, per owner', () => {
    const frame = sample();
    expect(inactiveCells(frame, 0)).toEqual([[0, 2]]);
    expect(inactiveCells(frame, 1)).toEqual([]);
  });

  it('writes one stroke group per cell and none for no cells', () => {
    expect(hatchPath([])).toBe('');
    expect(hatchPath([[0, 2]]).match(/M/g)).toHaveLength(3);
    expect(hatchPath([[0, 2]])).not.toContain('NaN');
  });

  it('has an empty frame with nothing on a 12 x 12 map', () => {
    const frame = emptyFrame();
    expect(frame.void).toHaveLength(12);
    expect(frame.void.every((row) => row.length === 12 && row.every((v) => !v))).toBe(true);
    expect(frame.owner.flat().every((o) => o === null)).toBe(true);
    expect([frame.buildings, frame.units, frame.mine_spots].every((l) => l.length === 0)).toBe(
      true,
    );
    expect(frame).toMatchObject({
      gold: [0, 0],
      income: [0, 0],
      military: [0, 0],
      turn: 1,
      to_move: null,
      last: null,
      result: null,
    });
  });
});
