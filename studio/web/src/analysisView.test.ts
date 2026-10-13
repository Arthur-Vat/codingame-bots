import { describe, expect, it } from 'vitest';
import type { Answer, AnalysisMove, AnalysisReport } from './api';
import {
  barLabel,
  barShares,
  CHART_PAD_X,
  CHART_WIDTH,
  chartPaths,
  chartRequests,
  chartX,
  chartY,
  chipOf,
  chipTitle,
  computedCount,
  formatCount,
  formatThinking,
  heatShares,
  lineTokens,
  nearestFrame,
  percent,
  positionText,
  requestMoves,
  selectLines,
  statLine,
  validWdl,
  wdlText,
} from './analysisView';
import { moveCellFor } from './games/registry';

const turn = (seat: number, ...lines: string[]): Answer[] => [{ seat, lines, ms: 1 }];

function entry(move: string, visits: number, wdl: AnalysisMove['wdl'], line?: string[]) {
  return { move, visits, wdl, proven: null, line: line ?? [move] } satisfies AnalysisMove;
}

function report(patch: Partial<AnalysisReport> = {}): AnalysisReport {
  return {
    iterations: 20000,
    nodes: 100,
    to_move: 1,
    proven: null,
    wdl: [0.58, 0.02, 0.4],
    moves: [],
    done: false,
    stop: null,
    ...patch,
  };
}

describe('requestMoves', () => {
  const model = {
    turns: [turn(0, '4 4'), turn(1, '4 5'), turn(0, '5 3'), turn(1, '3 7')],
    openingTurns: 0,
  };

  it('is the turns up to the frame, each as its answer', () => {
    expect(requestMoves(model, 0)).toEqual([]);
    expect(requestMoves(model, 2)).toEqual(['4 4', '4 5']);
    expect(requestMoves(model, 4)).toEqual(['4 4', '4 5', '5 3', '3 7']);
  });

  it('counts the opening turns, which frame 0 already shows', () => {
    const opened = { turns: model.turns, openingTurns: 2 };
    expect(requestMoves(opened, 0)).toEqual(['4 4', '4 5']);
    expect(requestMoves(opened, 2)).toEqual(['4 4', '4 5', '5 3', '3 7']);
  });

  it('joins the lines of an answer with a newline', () => {
    expect(requestMoves({ turns: [turn(0, '1', '2')], openingTurns: 0 }, 1)).toEqual(['1\n2']);
  });

  it('is null for a frame outside the game, or a turn without exactly one answer', () => {
    expect(requestMoves(model, 5)).toBeNull();
    expect(requestMoves(model, -1)).toBeNull();
    expect(requestMoves(model, 1.5)).toBeNull();
    expect(requestMoves({ turns: [[]], openingTurns: 0 }, 1)).toBeNull();
    expect(
      requestMoves({ turns: [[...turn(0, 'a'), ...turn(1, 'b')]], openingTurns: 0 }, 1),
    ).toBeNull();
  });

  it('is null for more moves than the program takes', () => {
    const long = {
      turns: Array.from({ length: 201 }, (_, i) => turn(i % 2, '4 4')),
      openingTurns: 0,
    };
    expect(requestMoves(long, 200)).toHaveLength(200);
    expect(requestMoves(long, 201)).toBeNull();
  });

  it('gives the requests of every frame, or none when one cannot be asked', () => {
    expect(chartRequests(model, 4)).toHaveLength(5);
    expect(chartRequests(model, 5)).toBeNull();
  });
});

describe('chances', () => {
  it('reads three finite numbers only', () => {
    expect(validWdl([0.5, 0.25, 0.25])).toEqual([0.5, 0.25, 0.25]);
    for (const bad of [null, undefined, [], [1, 2], [1, 2, 3, 4], [1, null, 0], [NaN, 0, 0], 'x']) {
      expect(validWdl(bad), String(bad)).toBeNull();
    }
  });

  it('rounds to whole percents within 0 to 100', () => {
    expect(percent(0.584)).toBe(58);
    expect(percent(0.585)).toBe(59);
    expect(percent(-1)).toBe(0);
    expect(percent(2)).toBe(100);
    expect(percent(NaN)).toBe(0);
  });

  it('writes the chances of a position', () => {
    expect(wdlText([0.5812, 0.0201, 0.3987])).toBe('X 58% · draw 2% · O 40%');
  });

  it('chips the side that is ahead, X when equal', () => {
    expect(chipOf([0.58, 0.02, 0.4])).toEqual({ side: 'X', percent: 58, text: 'X 58%' });
    expect(chipOf([0.4, 0.09, 0.51])).toEqual({ side: 'O', percent: 51, text: 'O 51%' });
    expect(chipOf([0.45, 0.1, 0.45]).side).toBe('X');
    expect(chipOf([0, 0, 1]).text).toBe('O 100%');
  });

  it('gives the chip a tooltip with the draw, the other side and the visits', () => {
    expect(chipTitle({ wdl: [0.58, 0.02, 0.4], visits: 9012 })).toBe(
      'X 58% · draw 2% · O 40% · 9,012 visits',
    );
    expect(chipTitle({ wdl: [0.58, 0.02, 0.4], visits: 1 })).toBe(
      'X 58% · draw 2% · O 40% · 1 visit',
    );
  });

  it('counts with commas', () => {
    expect(formatCount(812000)).toBe('812,000');
    expect(formatCount(999)).toBe('999');
    expect(formatCount(1234567)).toBe('1,234,567');
    expect(formatCount(0)).toBe('0');
  });
});

describe('selectLines', () => {
  const moves = [
    entry('4 4', 500, [0.6, 0, 0.4], ['4 4', '4 3']),
    entry('1 1', 300, [0.5, 0.1, 0.4]),
    entry('2 2', 200, null),
    entry('3 3', 0, null),
    entry('5 5', 100, [0.3, 0.1, 0.6]),
    entry('6 6', 50, [0.3, 0.1, 0.6]),
  ];

  it('takes the first moves that have visits and chances, in the report order', () => {
    expect(selectLines(report({ moves }), 1).map((l) => l.move)).toEqual(['4 4']);
    expect(selectLines(report({ moves }), 3).map((l) => l.move)).toEqual(['4 4', '1 1', '5 5']);
  });

  it('gives a move with no line a line of itself', () => {
    const [, second] = selectLines(report({ moves }), 3);
    expect(second?.line).toEqual(['1 1']);
    const missing = { ...entry('0 0', 10, [1, 0, 0]), line: undefined } as unknown as AnalysisMove;
    expect(selectLines(report({ moves: [missing] }), 1)[0]?.line).toEqual(['0 0']);
  });

  it('shows nothing without a report, moves, or any visit', () => {
    expect(selectLines(null, 3)).toEqual([]);
    expect(selectLines(report({ moves: [] }), 3)).toEqual([]);
    expect(selectLines(report({ moves: undefined as unknown as AnalysisMove[] }), 3)).toEqual([]);
    expect(selectLines(report({ moves: [entry('3 3', 0, null)] }), 3)).toEqual([]);
  });
});

describe('lineTokens', () => {
  const text = (line: string[], ply: number) =>
    lineTokens(line, ply).map((t) => (t.kind === 'number' ? t.text : `[${t.text}]`));

  it('numbers X moves with a point, from the start of the game', () => {
    expect(text(['4 4', '4 5', '5 3'], 0)).toEqual(['1.', '[4 4]', '[4 5]', '2.', '[5 3]']);
    expect(text(['4 4'], 16)).toEqual(['9.', '[4 4]']);
  });

  it('starts a line that starts with O with the number and an ellipsis', () => {
    expect(text(['6 1', '1 4', '4 3'], 15)).toEqual(['8…', '[6 1]', '9.', '[1 4]', '[4 3]']);
  });

  it('marks only the first move', () => {
    const tokens = lineTokens(['a', 'b'], 1);
    expect(tokens.filter((t) => t.kind === 'move' && t.first)).toHaveLength(1);
  });

  it('counts the opening turns like any other', () => {
    expect(text(['a', 'b'], 2)).toEqual(['2.', '[a]', '[b]']);
  });
});

describe('heatShares', () => {
  const cellOf = (text: string) => moveCellFor('uttt', text);

  it('is each move’s share of the visits, by cell key', () => {
    const shares = heatShares(
      report({
        moves: [entry('4 4', 750, [1, 0, 0]), entry('0 1', 250, [1, 0, 0]), entry('2 2', 0, null)],
      }),
      cellOf,
    );
    expect(shares?.get('4,4')).toBe(0.75);
    expect(shares?.get('0,1')).toBe(0.25);
    expect(shares?.has('2,2')).toBe(false);
  });

  it('is null without visits, a report or a readable move', () => {
    expect(heatShares(null, cellOf)).toBeNull();
    expect(heatShares(report({ moves: [] }), cellOf)).toBeNull();
    expect(heatShares(report({ moves: [entry('4 4', 0, null)] }), cellOf)).toBeNull();
    expect(heatShares(report({ moves: [entry('x', 5, [1, 0, 0])] }), cellOf)).toBeNull();
  });
});

describe('the header and the footer', () => {
  it('writes the time with one decimal', () => {
    expect(formatThinking(2000)).toBe('2.0 s');
    expect(formatThinking(2049)).toBe('2.0 s');
    expect(formatThinking(12_340)).toBe('12.3 s');
    expect(formatThinking(-5)).toBe('0.0 s');
  });

  it('says the time and the latest report’s iterations', () => {
    expect(statLine({ elapsedMs: 2000, report: report({ iterations: 812000 }) })).toBe(
      '2.0 s · 812,000 iterations',
    );
    expect(statLine({ elapsedMs: 300, report: null })).toBe('0.3 s');
  });

  it('says who is proven to win, and when the memory limit was reached', () => {
    expect(
      statLine({ elapsedMs: 1500, report: report({ proven: 0, stop: 'proven', done: true }) }),
    ).toBe('1.5 s · proven: X wins');
    expect(statLine({ elapsedMs: 1500, report: report({ proven: 1 }) })).toBe(
      '1.5 s · proven: O wins',
    );
    expect(statLine({ elapsedMs: 9000, report: report({ stop: 'nodes', done: true }) })).toBe(
      '9.0 s · memory limit reached',
    );
  });

  it('says nothing for a finished game', () => {
    expect(
      statLine({ elapsedMs: 10, report: report({ stop: 'over', done: true, to_move: null }) }),
    ).toBe('');
  });

  it('says who moves and the chances, or the result of a finished game', () => {
    expect(positionText(report())).toBe('O to move · X 58% · draw 2% · O 40%');
    expect(positionText(report({ to_move: 0, wdl: [0.4, 0.09, 0.51] }))).toBe(
      'X to move · X 40% · draw 9% · O 51%',
    );
    const over = (wdl: [number, number, number]) =>
      positionText(report({ stop: 'over', to_move: null, wdl }));
    expect(over([1, 0, 0])).toBe('Game over · X wins');
    expect(over([0, 0, 1])).toBe('Game over · O wins');
    expect(over([0, 1, 0])).toBe('Game over · draw');
    expect(positionText(null)).toBeNull();
    expect(positionText(report({ wdl: null as unknown as [number, number, number] }))).toBeNull();
  });
});

describe('the bar', () => {
  it('splits the height by the chances, normalised', () => {
    expect(barShares([0.58, 0.02, 0.4])).toEqual({ o: 0.4, draw: 0.02, x: 0.58 });
    const odd = barShares([0.5, 0.5, 0.5]);
    expect(odd.x + odd.draw + odd.o).toBeCloseTo(1);
    expect(barShares([1, 0, 0])).toEqual({ o: 0, draw: 0, x: 1 });
    expect(barShares([0, 0, 0])).toEqual({ o: 0, draw: 1, x: 0 });
  });

  it('labels itself with the chances', () => {
    expect(barLabel([0.58, 0.02, 0.4])).toBe('Win, draw and loss: X 58% · draw 2% · O 40%');
  });
});

describe('the chart’s scales', () => {
  it('puts the first frame and the last at the plot’s ends, 100% above 0%', () => {
    expect(chartX(0, 10)).toBe(CHART_PAD_X);
    expect(chartX(10, 10)).toBe(CHART_WIDTH - CHART_PAD_X);
    expect(chartX(5, 10)).toBe(CHART_WIDTH / 2);
    expect(chartY(1)).toBeLessThan(chartY(0.5));
    expect(chartY(0.5)).toBeLessThan(chartY(0));
    expect(chartY(0.5)).toBeCloseTo((chartY(0) + chartY(1)) / 2);
    expect(chartY(5)).toBe(chartY(1));
    expect(chartY(-1)).toBe(chartY(0));
  });

  it('finds the frame nearest to a point', () => {
    expect(nearestFrame(0, 10)).toBe(0);
    expect(nearestFrame(1, 10)).toBe(10);
    expect(nearestFrame(0.5, 10)).toBe(5);
    expect(nearestFrame(chartX(7, 10) / CHART_WIDTH + 0.01, 10)).toBe(7);
    expect(nearestFrame(0.5, 0)).toBe(0);
    expect(nearestFrame(NaN, 10)).toBe(0);
    expect(nearestFrame(-3, 10)).toBe(0);
    expect(nearestFrame(7, 10)).toBe(10);
  });

  it('counts the values computed from the start', () => {
    expect(computedCount([])).toBe(0);
    expect(computedCount([null, null])).toBe(0);
    expect(computedCount([0.5, 0.6, null, null])).toBe(2);
    expect(computedCount([0.5, 0.6])).toBe(2);
  });

  it('draws the curve of what is computed, closed under it for the fill', () => {
    expect(chartPaths([null, null, null], 2)).toEqual({ line: '', area: '' });
    expect(chartPaths([0.5, null, null], 2)).toEqual({ line: '', area: '' });
    const { line, area } = chartPaths([0.5, 1, null], 2);
    expect(line).toBe(
      `M ${CHART_PAD_X}.0 ${chartY(0.5).toFixed(1)} L ${CHART_WIDTH / 2}.0 ${chartY(1).toFixed(1)}`,
    );
    expect(area.startsWith(line)).toBe(true);
    expect(area.endsWith('Z')).toBe(true);
  });
});
