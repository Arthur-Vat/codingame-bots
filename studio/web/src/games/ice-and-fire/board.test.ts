import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { renderers, resultDetails } from '../registry';
import type { RendererProps } from '../registry';
import Board, { type BoardProps } from './Board';
import { forgetChosenThemes, setGameTheme } from '../../gameThemes';
import { sample } from './sample';
import { groundUrl, pieceUrl } from './themes';
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
/** How many ground images of a board theme's tile. */
const tiles = (html: string, theme: string, name: string): number =>
  count(html, new RegExp(`<image class="tile" href="${groundUrl(theme, name)}"`, 'g'));

describe('Board markup', () => {
  it('describes the board, with the turn and the gold, for a screen reader', () => {
    const html = render();
    expect(html).toContain('role="group"');
    expect(html).toContain(
      'aria-label="A Code of Ice and Fire board, turn 4 / 100, Blue to move, gold 12 red, 7 blue',
    );
  });

  it('draws a ground image for every cell, a void cell included, from the default board', () => {
    const html = render();
    expect(html).toContain('data-board="scandinavian"');
    expect(count(html, /<image class="tile"/g)).toBe(144);
    expect(tiles(html, 'scandinavian', 'ground-void')).toBe(1);
    expect(tiles(html, 'scandinavian', 'ground-neutral')).toBe(144 - 1 - 4 - 2);
    expect(tiles(html, 'scandinavian', 'ground-mine-spot')).toBe(2);
    expect(tiles(html, 'scandinavian', 'ground-fire')).toBe(2);
    expect(tiles(html, 'scandinavian', 'ground-fire-cut-off')).toBe(1);
    expect(tiles(html, 'scandinavian', 'ground-ice')).toBe(1);
    expect(tiles(html, 'scandinavian', 'ground-ice-cut-off')).toBe(0);
    expect(count(html, /preserveAspectRatio="none"/g)).toBe(144);
  });

  it('draws a bare map of 144 neutral tiles for the empty frame, and nothing on it', () => {
    const html = render({ frame: emptyFrame() });
    expect(tiles(html, 'scandinavian', 'ground-neutral')).toBe(144);
    expect(count(html, /class="unit /g)).toBe(0);
    expect(count(html, /class="building /g)).toBe(0);
    expect(html).toContain('turn 1 / 100');
    expect(html).not.toContain('class="last"');
    expect(html).not.toContain('NaN');
  });

  it('mirrors the tiles of a board theme that asks for it on some cells, and no other', () => {
    try {
      setGameTheme('ice-and-fire', 'board', 'japanese');
      const mirrors = count(render(), /transform="matrix\(-1 0 0 1 /g);
      expect(mirrors).toBeGreaterThan(40);
      expect(mirrors).toBeLessThan(104);
      setGameTheme('ice-and-fire', 'board', 'bauhaus');
      expect(count(render(), /matrix\(-1/g)).toBe(0);
    } finally {
      forgetChosenThemes();
    }
  });

  it('draws mine spots that hold no mine, with a title', () => {
    const frame = sample();
    frame.buildings.push({ owner: 0, kind: 'mine', x: 3, y: 3 });
    const html = render({ frame });
    expect(html).toContain('Mine spot at 8,8');
    expect(html).not.toContain('Mine spot at 3,3');
  });

  it('keeps a small mark on an owned mine spot without a mine, and none on a neutral one', () => {
    const frame = sample();
    expect(count(render({ frame }), /class="spot"/g)).toBe(0);
    frame.owner[3]![3] = 0;
    frame.active[3]![3] = true;
    const html = render({ frame });
    expect(count(html, /class="spot"/g)).toBe(1);
    expect(html).toContain('Mine spot at 3,3');
    frame.buildings.push({ owner: 0, kind: 'mine', x: 3, y: 3 });
    expect(count(render({ frame }), /class="spot"/g)).toBe(0);
  });

  it('draws an HQ, a mine and a tower as the chosen theme’s pieces, by owner', () => {
    const frame = sample();
    frame.buildings.push(
      { owner: 0, kind: 'mine', x: 3, y: 3 },
      { owner: 1, kind: 'tower', x: 10, y: 10 },
    );
    const html = render({ frame });
    expect(html).toContain('data-pieces="lava"');
    expect(html).toContain('Red HQ at 0,0');
    expect(html).toContain('Blue HQ at 11,11');
    expect(html).toContain('Red mine at 3,3');
    expect(html).toContain('Blue tower at 10,10');
    for (const name of ['hq-fire', 'hq-ice', 'mine-fire', 'tower-ice']) {
      expect(count(html, new RegExp(`<image href="${pieceUrl('lava', name)}"`, 'g')), name).toBe(1);
    }
    expect(count(html, /class="building /g)).toBe(4);
  });

  it('draws a unit as the piece of its owner and level', () => {
    const html = render();
    expect(count(html, /class="unit /g)).toBe(2);
    expect(html).toContain('Red unit level 1 at 1,0');
    expect(html).toContain('Blue unit level 3 at 11,10');
    expect(html).toContain(`<image href="${pieceUrl('lava', 'unit-fire-1')}"`);
    expect(html).toContain(`<image href="${pieceUrl('lava', 'unit-ice-3')}"`);
  });

  it('draws the chosen themes, and the default for a stored id it does not know', () => {
    try {
      setGameTheme('ice-and-fire', 'pieces', 'storybook');
      setGameTheme('ice-and-fire', 'board', 'japanese');
      const html = render();
      expect(html).toContain('data-pieces="storybook"');
      expect(html).toContain('data-board="japanese"');
      expect(html).toContain(pieceUrl('storybook', 'unit-ice-3'));
      expect(html).not.toContain(pieceUrl('lava', 'unit-ice-3'));
      expect(tiles(html, 'japanese', 'ground-void')).toBe(1);
      // An id outside the lists is refused, never used to build a URL.
      setGameTheme('ice-and-fire', 'pieces', '../../x');
      expect(render()).toContain('data-pieces="storybook"');
    } finally {
      forgetChosenThemes();
    }
    expect(render()).toContain('data-pieces="lava"');
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

/** The badges of a markup: [side class, shape, text], in drawing order. */
const badges = (html: string): [string, string, string][] =>
  [
    ...html.matchAll(
      /<g class="badge ([rb])"><(rect|circle)[^>]*><\/\2><text[^>]*>([^<]*)<\/text>/g,
    ),
  ].map((m) => [m[1]!, m[2]!, m[3]!]);
/** The overlays of a markup, which no theme changes. */
const overlays = (html: string): string[] =>
  [
    ...html.matchAll(
      /<g class="(?:hatches|badges)">.*?<\/g>(?=<g class="(?:badges|last)"|<\/svg>)/g,
    ),
  ].map((m) => m[0]);

describe('the marks that do not depend on the theme', () => {
  it('gives a unit a badge with its level, a rounded square for Red and a circle for Blue', () => {
    const levels = badges(render()).filter(([, , text]) => /^\d$/.test(text));
    expect(levels).toEqual([
      ['r', 'rect', '1'],
      ['b', 'circle', '3'],
    ]);
    expect(render()).toMatch(/<rect class="plate"[^>]* rx="/);
  });

  it('gives a building a badge with the letter of its side, in the same shapes', () => {
    const frame = sample();
    frame.buildings.push(
      { owner: 0, kind: 'mine', x: 3, y: 3 },
      { owner: 1, kind: 'tower', x: 10, y: 10 },
    );
    const letters = badges(render({ frame })).filter(([, , text]) => /^[RB]$/.test(text));
    expect(letters).toEqual([
      ['r', 'rect', 'R'],
      ['b', 'circle', 'B'],
      ['r', 'rect', 'R'],
      ['b', 'circle', 'B'],
    ]);
  });

  it('puts a unit’s badge in the bottom-right corner of its cell and a building’s top-right', () => {
    const html = render();
    // The Red unit at 1,0 and the Red HQ at 0,0, with a badge of 0.36 centred 0.21 from the edges.
    expect(html).toContain('<rect class="plate" x="1.61" y="0.61"');
    expect(html).toContain('<rect class="plate" x="0.61" y="0.03"');
  });

  it('hatches the owned cells cut off from their HQ, and no active or neutral one', () => {
    const html = render();
    expect(count(html, /class="hatch /g)).toBe(1);
    expect(html).toContain('class="hatch r"');
    const frame = sample();
    frame.active[2]![0] = true;
    expect(count(render({ frame }), /class="hatch /g)).toBe(0);
    frame.owner[5]![5] = 1;
    expect(count(render({ frame }), /class="hatch b"/g)).toBe(1);
    expect(count(render({ frame: emptyFrame() }), /class="(hatch|badge) /g)).toBe(0);
  });

  it('draws the same hatch and badges whatever the themes are', () => {
    const plain = overlays(render());
    expect(plain).toHaveLength(2);
    try {
      for (const [pieces, board] of [
        ['bauhaus', 'japanese'],
        ['storybook', 'deco'],
      ] as const) {
        setGameTheme('ice-and-fire', 'pieces', pieces);
        setGameTheme('ice-and-fire', 'board', board);
        const html = render();
        expect(html).toContain(`data-pieces="${pieces}"`);
        expect(overlays(html)).toEqual(plain);
      }
    } finally {
      forgetChosenThemes();
    }
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
