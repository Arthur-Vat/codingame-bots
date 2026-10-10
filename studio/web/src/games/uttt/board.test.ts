import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { renderers } from '../registry';
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
  it('has 81 grid cells in 9 small boards', () => {
    const html = renderToStaticMarkup(createElement(Board, props()));
    expect(count(html, /role="gridcell"/g)).toBe(81);
    expect(count(html, /class="uttt-sb[ "]/g)).toBe(9);
    expect(html).toContain('role="grid"');
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
    expect(count(html, /role="gridcell"/g)).toBe(81);
  });

  it('renders nothing for a frame it cannot read', () => {
    const Renderer = renderers['uttt']!;
    const html = renderToStaticMarkup(
      createElement(Renderer, { frame: { nope: true }, interactive: false, showCoordinates: true }),
    );
    expect(html).toBe('');
  });
});
