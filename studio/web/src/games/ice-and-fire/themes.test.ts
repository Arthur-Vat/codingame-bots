import { describe, expect, it } from 'vitest';
import {
  BOARD_THEMES,
  PIECE_THEMES,
  boardThemeOf,
  groundName,
  groundUrl,
  mirrored,
  pieceThemeOf,
  pieceUrl,
} from './themes';

const PIECE_NAMES = [
  ...['fire', 'ice'].flatMap((side) => [1, 2, 3].map((level) => `unit-${side}-${level}`)),
  ...['hq', 'mine', 'tower'].flatMap((kind) => [`${kind}-fire`, `${kind}-ice`]),
];

const GROUND_NAMES = [
  'ground-fire',
  'ground-fire-cut-off',
  'ground-ice',
  'ground-ice-cut-off',
  'ground-neutral',
  'ground-mine-spot',
  'ground-void',
];

describe('the theme lists', () => {
  it('have the owner’s themes in order, the defaults first', () => {
    expect(PIECE_THEMES.map((t) => [t.id, t.label])).toEqual([
      ['lava', 'Lava and glacier'],
      ['storybook', 'Storybook'],
      ['ink', 'Ink and watercolour'],
      ['japanese', 'Japanese print'],
      ['bauhaus', 'Bauhaus'],
      ['scandinavian', 'Scandinavian minimal'],
      ['deco', 'Art deco'],
    ]);
    expect(BOARD_THEMES.map((t) => [t.id, t.label, t.mirror])).toEqual([
      ['scandinavian', 'Scandinavian', false],
      ['storybook', 'Storybook', true],
      ['lava', 'Lava and glacier', true],
      ['watercolour', 'Watercolour paper', true],
      ['japanese', 'Japanese', true],
      ['bauhaus', 'Bauhaus', false],
      ['deco', 'Art deco', false],
    ]);
  });

  it('give the default for an id they do not hold', () => {
    for (const id of ['', 'nope', 'constructor', '__proto__', '../x']) {
      expect(pieceThemeOf(id).id).toBe('lava');
      expect(boardThemeOf(id).id).toBe('scandinavian');
    }
    expect(pieceThemeOf('deco').id).toBe('deco');
    expect(boardThemeOf('japanese').id).toBe('japanese');
  });
});

describe('the drawings', () => {
  it('exist for every piece theme and every piece', () => {
    for (const theme of PIECE_THEMES) {
      for (const name of PIECE_NAMES) {
        expect(pieceUrl(theme.id, name), `${theme.id}/${name}`).toMatch(/\.svg/);
      }
    }
  });

  it('exist for every board theme and every ground', () => {
    for (const theme of BOARD_THEMES) {
      for (const name of GROUND_NAMES) {
        expect(groundUrl(theme.id, name), `${theme.id}/${name}`).toMatch(/\.svg/);
      }
    }
  });

  it('are all different files, and none for what is not a drawing', () => {
    const urls = [
      ...PIECE_THEMES.flatMap((t) => PIECE_NAMES.map((n) => pieceUrl(t.id, n))),
      ...BOARD_THEMES.flatMap((t) => GROUND_NAMES.map((n) => groundUrl(t.id, n))),
    ];
    expect(new Set(urls).size).toBe(urls.length);
    expect(pieceUrl('lava', 'unit-fire-4')).toBe('');
    expect(pieceUrl('nope', 'hq-fire')).toBe('');
    expect(groundUrl('lava', 'unit-fire-1')).toBe('');
    expect(groundUrl('../x', 'ground-void')).toBe('');
  });
});

describe('groundName', () => {
  const cell = (owner: 0 | 1 | null, active: boolean, mineSpot = false, isVoid = false) =>
    groundName({ void: isVoid, owner, active, mineSpot });

  it('is void for a void cell, whatever else it says', () => {
    expect(cell(null, false, false, true)).toBe('ground-void');
    expect(cell(0, true, true, true)).toBe('ground-void');
  });

  it('is neutral, or a mine spot, for a cell nobody owns', () => {
    expect(cell(null, false)).toBe('ground-neutral');
    expect(cell(null, false, true)).toBe('ground-mine-spot');
  });

  it('is fire for Red and ice for Blue, cut off when not connected to the HQ', () => {
    expect(cell(0, true)).toBe('ground-fire');
    expect(cell(0, false)).toBe('ground-fire-cut-off');
    expect(cell(1, true)).toBe('ground-ice');
    expect(cell(1, false)).toBe('ground-ice-cut-off');
    // An owned mine spot keeps its owner's ground (the board adds a mark).
    expect(cell(1, true, true)).toBe('ground-ice');
  });
});

describe('mirrored', () => {
  it('is fixed, and true on about half the cells without a pattern of rows or columns', () => {
    let count = 0;
    const rows = new Set<string>();
    for (let y = 0; y < 12; y++) {
      let row = '';
      for (let x = 0; x < 12; x++) {
        expect(mirrored(x, y)).toBe(mirrored(x, y));
        if (mirrored(x, y)) count++;
        row += mirrored(x, y) ? '1' : '0';
      }
      rows.add(row);
    }
    expect(count).toBeGreaterThan(54);
    expect(count).toBeLessThan(90);
    expect(rows.size).toBeGreaterThan(8);
  });
});
