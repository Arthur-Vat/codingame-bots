import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { renderers, resultDetails } from '../registry';
import type { RendererProps } from '../registry';
import Board, { type BoardProps } from './Board';
import { sample } from './sample';
import { emptyFrame } from './view';

const props = (extra: Partial<BoardProps> = {}): BoardProps => ({
  frame: sample(),
  interactive: false,
  showCoordinates: true,
  ...extra,
});

const render = (extra: Partial<BoardProps> = {}): string =>
  renderToStaticMarkup(createElement(Board, props(extra)));
const count = (html: string, pattern: RegExp): number => (html.match(pattern) ?? []).length;

describe('Board markup', () => {
  it('describes the board, with the turn and the gold, for a screen reader', () => {
    const html = render();
    expect(html).toContain('role="group"');
    expect(html).toContain(
      'aria-label="A Code of Ice and Fire board, turn 4 / 100, Blue to move, gold 12 red, 7 blue',
    );
  });

  it('draws a tile for every cell that is not void, and none for a void cell', () => {
    const html = render();
    expect(count(html, /<rect class="tile /g)).toBe(143);
    expect(count(html, /class="tile neutral"/g)).toBe(143 - 4);
    expect(count(html, /class="tile r"/g)).toBe(2);
    expect(count(html, /class="tile r off"/g)).toBe(1);
    expect(count(html, /class="tile b"/g)).toBe(1);
  });

  it('draws a bare map of 144 tiles for the empty frame, and nothing on it', () => {
    const html = render({ frame: emptyFrame() });
    expect(count(html, /class="tile neutral"/g)).toBe(144);
    expect(count(html, /class="unit /g)).toBe(0);
    expect(count(html, /class="building /g)).toBe(0);
    expect(html).toContain('turn 1 / 100');
    expect(html).not.toContain('class="last"');
    expect(html).not.toContain('NaN');
  });

  it('marks the cells cut off from their HQ with hatching, one path per owner', () => {
    const html = render();
    expect(html).toContain('class="hatch r" d="M0.12 2.88');
    expect(html).toContain('class="hatch b" d=""');
  });

  it('draws mine spots that hold no mine, with a title', () => {
    const frame = sample();
    frame.buildings.push({ owner: 0, kind: 'mine', x: 3, y: 3 });
    const html = render({ frame });
    expect(count(html, /class="spot"/g)).toBe(1);
    expect(html).toContain('Mine spot at 8,8');
    expect(html).not.toContain('Mine spot at 3,3');
  });

  it('draws an HQ with its owner letter, a mine and a tower in different shapes', () => {
    const frame = sample();
    frame.buildings.push(
      { owner: 0, kind: 'mine', x: 3, y: 3 },
      { owner: 1, kind: 'tower', x: 10, y: 10 },
    );
    const html = render({ frame });
    expect(html).toContain('Red HQ at 0,0');
    expect(html).toContain('Blue HQ at 11,11');
    expect(html).toContain('>R<');
    expect(html).toContain('>B<');
    expect(html).toContain('Red mine at 3,3');
    expect(html).toContain('Blue tower at 10,10');
    const shape = (kind: string): string =>
      new RegExp(
        `class="building ${kind} [rb]"[^>]*>(?:<title>[^<]*</title>)?<path class="shape" d="([^"]*)"`,
      ).exec(html)?.[1] ?? '';
    expect(new Set([shape('hq'), shape('mine'), shape('tower')]).size).toBe(3);
  });

  it('draws a unit as a disc with its level and owner, blue ones with a ring', () => {
    const html = render();
    expect(count(html, /class="unit /g)).toBe(2);
    expect(html).toContain('Red unit level 1 at 1,0');
    expect(html).toContain('Blue unit level 3 at 11,10');
    expect(count(html, /class="halo"/g)).toBe(1);
    expect(html).toMatch(/<text class="level"[^>]*>3<\/text>/);
  });

  it('draws up to a unit on every cell', () => {
    const frame = sample();
    frame.units = Array.from({ length: 144 }, (_, id) => ({
      id,
      owner: (id % 2) as 0 | 1,
      level: (1 + (id % 3)) as 1 | 2 | 3,
      x: id % 12,
      y: Math.floor(id / 12),
    }));
    expect(count(render({ frame }), /class="unit /g)).toBe(144);
  });

  it('shows the last turn: an arrow per move, a ring per training, a mark per building, a fade per removal', () => {
    const html = render();
    expect(count(html, /class="move"/g)).toBe(1);
    expect(html).toContain('Moved from 0,0 to 1,0');
    expect(count(html, /class="trained"/g)).toBe(1);
    expect(html).toContain('Trained level 1 unit at 0,1');
    expect(count(html, /class="built"/g)).toBe(1);
    expect(html).toContain('Built mine at 3,3');
    expect(count(html, /class="removed b"/g)).toBe(1);
    expect(html).toContain('Blue unit level 2 removed at 5,5');
  });

  it('shows no last turn marks in frame 0', () => {
    const html = render({ frame: { ...sample(), last: null } });
    expect(count(html, /class="(move|trained|built|removed )/g)).toBe(0);
  });

  it('draws no arrow for a move that stays in its cell', () => {
    const frame = sample();
    frame.last!.moved = [{ id: 1, from: [2, 2], to: [2, 2] }];
    expect(count(render({ frame }), /class="move"/g)).toBe(0);
  });

  it('shows both seats: gold, a signed income and military power', () => {
    const html = render();
    expect(html).toContain('>12 gold<');
    expect(html).toContain('>+5<');
    expect(html).toContain('>-2<');
    expect(html).toContain('>power 1<');
    expect(html).toContain('>power 3<');
    expect(html).toContain('>turn 4 / 100<');
    expect(html).toContain('>Blue to move<');
  });

  it('shows the last answer, whole in the title', () => {
    const frame = sample();
    const long = 'MOVE 1 11 11;'.repeat(30);
    frame.last!.answer = long;
    const html = render({ frame });
    expect(html).toContain(`title="Red: ${long}"`);
    expect(html).toContain(`>Red: ${long}</div>`);
  });

  it('keeps a line for the answer in frame 0, without a title', () => {
    const html = render({ frame: { ...sample(), last: null } });
    expect(html).toContain('<div class="iaf-answer"></div>');
  });

  it('says the game is over when it is', () => {
    const over = { ...sample(), to_move: null, result: { winner: 0 as const, end: 'hq' as const } };
    const html = render({ frame: over });
    expect(html).toContain('>Game over<');
    expect(html).not.toContain('to move');
  });

  it('keeps the coordinates whether they are shown or not, x along the top and y down the side', () => {
    const shown = render({ showCoordinates: true });
    const hidden = render({ showCoordinates: false });
    for (const html of [shown, hidden]) {
      expect(count(html, /class="coord"/g)).toBe(24);
      expect(html).toContain('class="coord" x="11.5" y="-0.4">11<');
      expect(html).toContain('class="coord" x="-0.45" y="11.5">11<');
    }
    expect(shown).not.toContain('coords-off');
    expect(hidden).toContain('coords-off');
  });

  it('takes the shell props and ignores them: no buttons, no roles to click', () => {
    const html = render({
      interactive: true,
      onAction: () => undefined,
      heat: new Map([['0,0', 0.5]]),
      highlight: [1, 1],
    });
    expect(count(html, /<button/g)).toBe(0);
    expect(html).toBe(render());
  });
});

describe('registry', () => {
  it('renders the game through the generic props', () => {
    const Renderer = renderers['ice-and-fire']!;
    const rendered = (frame: unknown): string => {
      const shell: RendererProps = { frame, interactive: false, showCoordinates: false };
      return renderToStaticMarkup(createElement(Renderer, shell));
    };
    expect(rendered(sample())).toContain('class="iaf coords-off"');
    // A frame that is not one draws nothing, and never throws.
    for (const bad of [null, undefined, 4, 'x', {}, { ...sample(), turn: 0 }, { void: [] }]) {
      expect(rendered(bad), JSON.stringify(bad)).toBe('');
    }
  });

  it('words the end of a game from the frame, and nothing for a bad frame', () => {
    const detail = resultDetails['ice-and-fire']!;
    const over = { ...sample(), to_move: null, result: { winner: 1, end: 'hq' } };
    expect(detail(over)).toBe("Blue captured Red's HQ.");
    expect(detail(sample())).toBeNull();
    expect(detail({ nope: true })).toBeNull();
    expect(detail(null)).toBeNull();
  });
});
