/* The artwork of A Code of Ice and Fire's board: our own SVG drawings in `themes/`, in two
   independent choices, the pieces (units and buildings) and the board (ground tiles). Fire is
   seat 0 (Red), ice is seat 1 (Blue). Each file is a 64 x 64 drawing; pieces carry their own
   margins and shadow, so a piece is drawn at the full cell. */

import type { Seat } from './view';

export interface PieceTheme {
  id: string;
  label: string;
}

export interface BoardTheme {
  id: string;
  label: string;
  /** The tiles are mirrored left-right on about half the cells, so the texture does not repeat. */
  mirror: boolean;
}

export const PIECE_THEMES: readonly PieceTheme[] = [
  { id: 'lava', label: 'Lava and glacier' },
  { id: 'storybook', label: 'Storybook' },
  { id: 'ink', label: 'Ink and watercolour' },
  { id: 'japanese', label: 'Japanese print' },
  { id: 'bauhaus', label: 'Bauhaus' },
  { id: 'scandinavian', label: 'Scandinavian minimal' },
  { id: 'deco', label: 'Art deco' },
];

export const BOARD_THEMES: readonly BoardTheme[] = [
  { id: 'scandinavian', label: 'Scandinavian', mirror: false },
  { id: 'storybook', label: 'Storybook', mirror: true },
  { id: 'lava', label: 'Lava and glacier', mirror: true },
  { id: 'watercolour', label: 'Watercolour paper', mirror: true },
  { id: 'japanese', label: 'Japanese', mirror: true },
  { id: 'bauhaus', label: 'Bauhaus', mirror: false },
  { id: 'deco', label: 'Art deco', mirror: false },
];

export const DEFAULT_PIECES = 'lava';
export const DEFAULT_BOARD = 'scandinavian';

/** The theme of the list with this id, or the list's first (the default) for anything else. */
export function pieceThemeOf(id: string): PieceTheme {
  return PIECE_THEMES.find((t) => t.id === id) ?? PIECE_THEMES[0]!;
}

export function boardThemeOf(id: string): BoardTheme {
  return BOARD_THEMES.find((t) => t.id === id) ?? BOARD_THEMES[0]!;
}

/* The URLs of the drawings, as the bundler gives them, keyed by the path of the file. */
const ASSETS = import.meta.glob<string>('./themes/**/*.svg', {
  eager: true,
  query: '?url',
  import: 'default',
});

/** The URL of a piece drawing (`unit-fire-1`, `hq-ice`, ...), or '' when there is none. */
export function pieceUrl(theme: string, name: string): string {
  return ASSETS[`./themes/pieces/${theme}/${name}.svg`] ?? '';
}

/** The URL of a ground tile (`ground-neutral`, ...), or '' when there is none. */
export function groundUrl(theme: string, name: string): string {
  return ASSETS[`./themes/boards/${theme}/${name}.svg`] ?? '';
}

/** What decides a cell's ground. */
export interface GroundCell {
  void: boolean;
  owner: Seat | null;
  /** Connected to its owner's HQ. */
  active: boolean;
  /** A mine can be built here (whether or not one is). */
  mineSpot: boolean;
}

/** The name of the ground tile of a cell. */
export function groundName(cell: GroundCell): string {
  if (cell.void) return 'ground-void';
  if (cell.owner === null) return cell.mineSpot ? 'ground-mine-spot' : 'ground-neutral';
  const side = sideName(cell.owner);
  return cell.active ? `ground-${side}` : `ground-${side}-cut-off`;
}

/** How the files call a seat: fire is Red, ice is Blue. */
export const sideName = (seat: Seat): 'fire' | 'ice' => (seat === 0 ? 'fire' : 'ice');

/** Whether a cell's tile is mirrored: a fixed mix of x and y that is true on about half. */
export function mirrored(x: number, y: number): boolean {
  let h = Math.imul(x + 1, 0x9e3779b1) ^ Math.imul(y + 1, 0x85ebca6b);
  h ^= h >>> 15;
  h = Math.imul(h, 0x2c1b3c6d);
  h ^= h >>> 12;
  return (h & 1) === 1;
}
