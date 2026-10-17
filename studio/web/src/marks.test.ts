import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { SeatMark } from './Marks';

const render = (seat: number, game?: string): string =>
  renderToStaticMarkup(createElement(SeatMark, { seat, className: 'chip', game }));

describe('SeatMark', () => {
  it('draws X and O, for a game without names and when no game is given', () => {
    for (const game of [undefined, 'uttt', 'nope']) {
      expect(render(0, game)).toContain('mark-x seat-0 chip');
      expect(render(1, game)).toContain('mark-o seat-1 chip');
    }
  });

  it('draws a square with the first letter of the name, for a game that names its seats', () => {
    const red = render(0, 'ice-and-fire');
    const blue = render(1, 'ice-and-fire');
    expect(red).toContain('mark-sq seat-0 chip');
    expect(red).toContain('>R</text>');
    expect(blue).toContain('mark-sq seat-1 chip');
    expect(blue).toContain('>B</text>');
    expect(red).not.toContain('mark-x');
  });
});
