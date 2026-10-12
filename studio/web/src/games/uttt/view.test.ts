import { describe, expect, it } from 'vitest';
import {
  asUtttFrame,
  cellState,
  emptyFrame as blankFrame,
  prepareHeat,
  resultDetail,
  smallBoardState,
  winningLines,
  type Cell,
  type UtttFrame,
} from './view';

function emptyFrame(): UtttFrame {
  const playable: Cell[] = [];
  for (let r = 0; r < 9; r++) for (let c = 0; c < 9; c++) playable.push([r, c]);
  return {
    cells: Array.from({ length: 9 }, () => Array<null>(9).fill(null)),
    small: Array.from({ length: 3 }, () => Array<null>(3).fill(null)),
    last: null,
    playable,
    to_move: 0,
    points: [0, 0],
    result: null,
  };
}

/** X played (4,4); O must answer in the centre small board. */
function afterCentre(): UtttFrame {
  const frame = emptyFrame();
  frame.cells[4]![4] = 0;
  frame.last = [4, 4];
  frame.to_move = 1;
  frame.playable = [];
  for (let r = 3; r < 6; r++) {
    for (let c = 3; c < 6; c++) if (r !== 4 || c !== 4) frame.playable.push([r, c]);
  }
  return frame;
}

describe('cellState', () => {
  it('marks all 81 cells of an empty frame as playable', () => {
    const frame = emptyFrame();
    let count = 0;
    for (let r = 0; r < 9; r++) {
      for (let c = 0; c < 9; c++) {
        const s = cellState(frame, r, c);
        expect(s.mark).toBeNull();
        if (s.playable && s.status === 'play') count++;
      }
    }
    expect(count).toBe(81);
  });

  it('shows the last move and the cells playable next', () => {
    const frame = afterCentre();
    const last = cellState(frame, 4, 4);
    expect(last.status).toBe('last');
    expect(last.mark).toBe(0);
    expect(last.playable).toBe(false);
    expect(last.label).toBe('row 4 column 4 X');
    let play = 0;
    for (let r = 0; r < 9; r++) {
      for (let c = 0; c < 9; c++) if (cellState(frame, r, c).status === 'play') play++;
    }
    expect(play).toBe(8);
    expect(cellState(frame, 3, 3).status).toBe('play');
    expect(cellState(frame, 3, 3).label).toBe('row 3 column 3 playable');
    expect(cellState(frame, 0, 0).status).toBeNull();
    expect(cellState(frame, 0, 0).label).toBe('row 0 column 0');
  });

  it('names an O', () => {
    const frame = afterCentre();
    frame.cells[3]![3] = 1;
    expect(cellState(frame, 3, 3).label).toBe('row 3 column 3 O');
  });

  it('ignores the heat of cells that are not playable', () => {
    const frame = afterCentre();
    const heat = new Map([
      ['3,3', 0.5],
      ['0,0', 0.3],
      ['4,4', 0.2],
    ]);
    const prepared = prepareHeat(frame, heat);
    expect(prepared?.shares.size).toBe(1);
    expect(cellState(frame, 0, 0, prepared).heat).toBeNull();
    expect(cellState(frame, 0, 0, prepared).heatAlpha).toBe(0);
    expect(cellState(frame, 4, 4, prepared).heat).toBeNull();
    const hot = cellState(frame, 3, 3, prepared);
    expect(hot.heat).toBe(0.5);
    expect(hot.heatLabel).toBe('50');
    expect(hot.heatAlpha).toBeCloseTo(0.82);
  });

  it('writes a percentage only from 4%', () => {
    const frame = afterCentre();
    const heat = new Map([
      ['3,3', 0.9],
      ['3,4', 0.04],
      ['3,5', 0.039],
    ]);
    const prepared = prepareHeat(frame, heat);
    expect(cellState(frame, 3, 4, prepared).heatLabel).toBe('4');
    expect(cellState(frame, 3, 5, prepared).heatLabel).toBeNull();
    expect(cellState(frame, 3, 5, prepared).heat).toBe(0.039);
  });

  it('has no heat without a map or without a legal cell in it', () => {
    const frame = afterCentre();
    expect(prepareHeat(frame, null)).toBeNull();
    expect(prepareHeat(frame, new Map([['0,0', 1]]))).toBeNull();
  });
});

describe('smallBoardState', () => {
  it('reports a board won by a seat', () => {
    const frame = emptyFrame();
    frame.small[0]![2] = 1;
    frame.small[1]![1] = 0;
    expect(smallBoardState(frame, 0, 2)).toEqual({ won: 1, drawn: false });
    expect(smallBoardState(frame, 1, 1)).toEqual({ won: 0, drawn: false });
  });

  it('reports a drawn board', () => {
    const frame = emptyFrame();
    frame.small[2]![0] = 'draw';
    expect(smallBoardState(frame, 2, 0)).toEqual({ won: null, drawn: true });
  });

  it('reports an open board', () => {
    expect(smallBoardState(emptyFrame(), 1, 0)).toEqual({ won: null, drawn: false });
  });
});

describe('asUtttFrame', () => {
  it('accepts a frame and refuses what is not one', () => {
    expect(asUtttFrame(emptyFrame())).not.toBeNull();
    expect(asUtttFrame(null)).toBeNull();
    expect(asUtttFrame({})).toBeNull();
    expect(asUtttFrame({ cells: [], small: [], playable: [] })).toBeNull();
  });
});

describe('resultDetail', () => {
  it('counts the small boards each seat won', () => {
    expect(resultDetail({ ...emptyFrame(), points: [4, 3] })).toBe('Small boards won: 4–3');
  });

  it('says nothing when the frame has no points', () => {
    const frame = { ...emptyFrame(), points: undefined } as unknown as UtttFrame;
    expect(resultDetail(frame)).toBeNull();
  });
});

describe('emptyFrame', () => {
  it('is a frame with nothing played and nothing playable', () => {
    const frame = blankFrame();
    expect(asUtttFrame(frame)).not.toBeNull();
    expect(frame.cells.flat().every((cell) => cell === null)).toBe(true);
    expect(frame.small.flat().every((cell) => cell === null)).toBe(true);
    expect(frame).toMatchObject({ last: null, playable: [], to_move: null, result: null });
    expect(cellState(frame, 4, 4)).toMatchObject({ mark: null, status: null, playable: false });
  });
});

describe('winningLines', () => {
  const won = (winner: 0 | 1 | null, ...owned: [number, number][]): UtttFrame => {
    const frame = emptyFrame();
    frame.result = { winner };
    for (const [r, c] of owned) frame.small[r]![c] = winner;
    return frame;
  };

  it('finds each row', () => {
    for (const r of [0, 1, 2]) {
      expect(winningLines(won(0, [r, 0], [r, 1], [r, 2]))).toEqual([{ from: [r, 0], to: [r, 2] }]);
    }
  });

  it('finds each column', () => {
    for (const c of [0, 1, 2]) {
      expect(winningLines(won(1, [0, c], [1, c], [2, c]))).toEqual([{ from: [0, c], to: [2, c] }]);
    }
  });

  it('finds both diagonals', () => {
    expect(winningLines(won(0, [0, 0], [1, 1], [2, 2]))).toEqual([{ from: [0, 0], to: [2, 2] }]);
    expect(winningLines(won(1, [0, 2], [1, 1], [2, 0]))).toEqual([{ from: [0, 2], to: [2, 0] }]);
  });

  it('gives every line when several exist', () => {
    const frame = won(0, [0, 0], [0, 1], [0, 2], [1, 1], [2, 2]);
    expect(winningLines(frame)).toEqual([
      { from: [0, 0], to: [0, 2] },
      { from: [0, 0], to: [2, 2] },
    ]);
  });

  it('ignores the other seat, drawn boards and open ones', () => {
    const frame = won(0, [0, 0], [0, 1]);
    frame.small[0]![2] = 1;
    expect(winningLines(frame)).toEqual([]);
    frame.small[0]![2] = 'draw';
    expect(winningLines(frame)).toEqual([]);
    frame.small[0]![2] = null;
    expect(winningLines(frame)).toEqual([]);
  });

  it('draws no line for a win on the count of small boards', () => {
    expect(winningLines(won(1, [0, 0], [0, 1], [1, 2], [2, 0]))).toEqual([]);
  });

  it('draws no line for a draw, a game in progress or no result', () => {
    const owned: [number, number][] = [
      [0, 0],
      [0, 1],
      [0, 2],
    ];
    expect(winningLines(won(null, ...owned))).toEqual([]);
    const running = won(0, ...owned);
    running.result = null;
    expect(winningLines(running)).toEqual([]);
  });

  it('does not throw on a malformed frame', () => {
    const base = won(0, [0, 0], [0, 1], [0, 2]);
    const bad: unknown[] = [
      { ...base, small: undefined },
      { ...base, small: [] },
      { ...base, small: [null, 5, 'x'] },
      { ...base, small: [[0], [], [0, 0, 0]] },
      { ...base, result: undefined },
      { ...base, result: { winner: 7 } },
      {},
      null,
    ];
    for (const frame of bad) expect(winningLines(frame as UtttFrame)).toEqual([]);
  });
});
