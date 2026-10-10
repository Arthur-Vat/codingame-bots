import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { renderers, resultDetails } from '../registry';
import type { RendererProps } from '../registry';
import Board, { type BoardProps } from './Board';
import type { Cell, UtttFrame } from './view';

function frame(): UtttFrame {
  const playable: Cell[] = [];
  for (let r = 3; r < 6; r++) {
    for (let c = 3; c < 6; c++) if (r !== 4 || c !== 4) playable.push([r, c]);
  }
  const cells = Array.from({ length: 9 }, () => Array<0 | 1 | null>(9).fill(null));
  cells[4]![4] = 0;
  cells[0]![0] = 1;
  const small = Array.from({ length: 3 }, () => Array<0 | 1 | 'draw' | null>(3).fill(null));
  small[0]![2] = 1;
  small[2]![0] = 'draw';
  return { cells, small, last: [4, 4], playable, to_move: 1, points: [0, 1], result: null };
}

const props = (extra: Partial<BoardProps> = {}): BoardProps => ({
  frame: frame(),
  interactive: false,
  showCoordinates: true,
  ...extra,
});

const count = (html: string, pattern: RegExp): number => (html.match(pattern) ?? []).length;

describe('Board markup', () => {
  it('has 81 cells in 9 small boards', () => {
    const html = renderToStaticMarkup(createElement(Board, props()));
    expect(count(html, /class="uttt-cell[ "]/g)).toBe(81);
    expect(count(html, /class="uttt-sb[ "]/g)).toBe(9);
    expect(html).toContain('role="group" aria-label="Ultimate Tic-Tac-Toe board"');
    expect(count(html, /role="group"/g)).toBe(10);
  });

  it('keeps the coordinate gutters whether coordinates are shown or not', () => {
    const shown = renderToStaticMarkup(createElement(Board, props({ showCoordinates: true })));
    const hidden = renderToStaticMarkup(createElement(Board, props({ showCoordinates: false })));
    for (const html of [shown, hidden]) {
      expect(count(html, /uttt-coords-top/g)).toBe(1);
      expect(count(html, /uttt-coords-left/g)).toBe(1);
      expect(count(html, /<span>\d<\/span>/g)).toBe(18);
    }
    expect(shown).not.toContain('coords-off');
    expect(hidden).toContain('coords-off');
  });

  it('labels cells for screen readers', () => {
    const html = renderToStaticMarkup(createElement(Board, props()));
    expect(html).toContain('aria-label="row 4 column 4 X"');
    expect(html).toContain('aria-label="row 0 column 0 O"');
    expect(html).toContain('aria-label="row 3 column 3 playable"');
    expect(html).toContain('aria-label="row 8 column 8"');
  });

  it('places each cell in its small board, in reading order', () => {
    const html = renderToStaticMarkup(createElement(Board, props()));
    const boards = html.split('<div class="uttt-sb').slice(1);
    expect(boards).toHaveLength(9);
    const cellsOf = (board: string): string[] =>
      [...board.matchAll(/class="uttt-cell[^"]*"[^>]*role="img" aria-label="([^"]*)"/g)].map(
        (m) => m[1] ?? '',
      );
    expect(boards[4]).toContain('aria-label="small board 1 1"');
    expect(cellsOf(boards[4]!)[1]).toMatch(/^row 3 column 4/);
    expect(boards[6]).toContain('aria-label="small board 2 0"');
    expect(cellsOf(boards[6]!)[6]).toMatch(/^row 8 column 0/);
    for (const board of boards) expect(cellsOf(board)).toHaveLength(9);
  });

  it('uses no grid roles, which need rows the layout does not have', () => {
    const html = renderToStaticMarkup(createElement(Board, props({ interactive: true })));
    expect(html).not.toMatch(/role="(grid|row|gridcell)"|aria-(row|col)(index|count)/);
  });

  it('draws buttons only on playable cells of an interactive board', () => {
    const still = renderToStaticMarkup(createElement(Board, props()));
    expect(count(still, /<button/g)).toBe(0);
    const live = renderToStaticMarkup(createElement(Board, props({ interactive: true })));
    expect(count(live, /<button/g)).toBe(8);
  });

  it('draws won and drawn small boards, the last move and the highlight', () => {
    const html = renderToStaticMarkup(createElement(Board, props({ highlight: [3, 4] })));
    expect(html).toContain('uttt-sb won-o');
    expect(html).toContain('uttt-sb drawn');
    expect(count(html, /uttt-cell last/g)).toBe(1);
    expect(count(html, /uttt-cell play hl/g)).toBe(1);
  });

  it('draws heat on legal cells only, with a label from 4%', () => {
    const heat = new Map([
      ['3,3', 0.6],
      ['3,4', 0.02],
      ['0,5', 0.9],
    ]);
    const html = renderToStaticMarkup(createElement(Board, props({ heat })));
    expect(count(html, /class="pct"/g)).toBe(1);
    expect(html).toContain('>60<');
    expect(count(html, /--h:/g)).toBe(2);
  });
});

describe('registry', () => {
  it('renders uttt through the generic props', () => {
    const Renderer = renderers['uttt']!;
    const html = renderToStaticMarkup(
      createElement(Renderer, { frame: frame(), interactive: false, showCoordinates: false }),
    );
    expect(count(html, /class="uttt-cell[ "]/g)).toBe(81);
  });

  it('renders nothing for a frame it cannot read', () => {
    const Renderer = renderers['uttt']!;
    const html = renderToStaticMarkup(
      createElement(Renderer, { frame: { nope: true }, interactive: false, showCoordinates: true }),
    );
    expect(html).toBe('');
  });
});

describe('registry actions and result detail', () => {
  it('reports a clicked cell as the action {row, col}', () => {
    const actions: unknown[] = [];
    const Renderer = renderers['uttt']!;
    const element = (Renderer as (props: RendererProps) => ReturnType<typeof createElement>)({
      frame: frame(),
      interactive: true,
      showCoordinates: false,
      onAction: (action) => actions.push(action),
    }) as unknown as { props: BoardProps };
    element.props.onCell?.(3, 5);
    expect(actions).toEqual([{ row: 3, col: 5 }]);
  });

  it('writes the result detail from a frame, or nothing for an unreadable one', () => {
    expect(resultDetails['uttt']?.(frame())).toBe('Small boards won: 0–1');
    expect(resultDetails['uttt']?.({ nope: true })).toBeNull();
  });
});
