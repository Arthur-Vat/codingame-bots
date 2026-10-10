/* Pure helpers for Ultimate Tic-Tac-Toe's board: what to draw in a cell or a small board.
   They hold no rules; the frame comes from the server's adapter (games/uttt/studio). */

export type Seat = 0 | 1;
export type Mark = Seat | null;
export type SmallState = Seat | 'draw' | null;
export type Cell = [number, number];

/** The JSON frame of games/uttt/studio. Seat 0 plays X and moves first; seat 1 plays O. */
export interface UtttFrame {
  cells: Mark[][];
  small: SmallState[][];
  last: Cell | null;
  playable: Cell[];
  to_move: Seat | null;
  points: [number, number];
  result: null | { winner: Seat | null };
}

/** Share of the search (0..1) of each legal move, by the key "row,col". */
export type HeatMap = Map<string, number>;

/** A heat map reduced to the legal cells, with its largest share. */
export interface Heat {
  shares: Map<string, number>;
  max: number;
}

export type CellStatus = 'last' | 'play' | null;

export interface CellState {
  mark: Mark;
  /** `last` for the last move, `play` for a cell playable next. */
  status: CellStatus;
  playable: boolean;
  /** The share of this cell, only on playable cells. */
  heat: number | null;
  /** The opacity of the heat colour, 0 without heat. */
  heatAlpha: number;
  /** The percentage written in the cell, when its share is at least HEAT_LABEL_MIN. */
  heatLabel: string | null;
  /** The text a screen reader gets. */
  label: string;
}

export interface SmallBoardState {
  won: Seat | null;
  drawn: boolean;
}

/** Shares under this are drawn with colour only, without a number. */
export const HEAT_LABEL_MIN = 0.04;

export const cellKey = (row: number, col: number): string => `${row},${col}`;

export const markName = (mark: Seat): string => (mark === 0 ? 'X' : 'O');

/** Reads `value` as a frame, or gives null when it does not have a frame's shape. */
export function asUtttFrame(value: unknown): UtttFrame | null {
  if (typeof value !== 'object' || value === null) return null;
  const v = value as Partial<UtttFrame>;
  const grid = (rows: unknown, n: number): boolean =>
    Array.isArray(rows) &&
    rows.length === n &&
    rows.every((r) => Array.isArray(r) && r.length === n);
  if (!grid(v.cells, 9) || !grid(v.small, 3) || !Array.isArray(v.playable)) return null;
  return v as UtttFrame;
}

export function isPlayable(frame: UtttFrame, row: number, col: number): boolean {
  return frame.playable.some(([r, c]) => r === row && c === col);
}

/** The heat of the legal cells only; null when there is nothing to draw. */
export function prepareHeat(frame: UtttFrame, heat: HeatMap | null | undefined): Heat | null {
  if (!heat) return null;
  const shares = new Map<string, number>();
  let max = 0;
  for (const [row, col] of frame.playable) {
    const share = heat.get(cellKey(row, col));
    if (share === undefined || !Number.isFinite(share) || share < 0) continue;
    shares.set(cellKey(row, col), share);
    max = Math.max(max, share);
  }
  return shares.size === 0 ? null : { shares, max };
}

export function cellState(
  frame: UtttFrame,
  row: number,
  col: number,
  heat?: Heat | null,
): CellState {
  const mark = frame.cells[row]?.[col] ?? null;
  const playable = isPlayable(frame, row, col);
  const isLast = frame.last !== null && frame.last[0] === row && frame.last[1] === col;
  const status: CellStatus = isLast ? 'last' : playable ? 'play' : null;
  const share = playable ? (heat?.shares.get(cellKey(row, col)) ?? null) : null;
  const max = heat?.max ?? 0;
  const heatAlpha = share === null ? 0 : 0.12 + 0.7 * (max > 0 ? share / max : 0);
  const heatLabel =
    share !== null && share >= HEAT_LABEL_MIN ? String(Math.round(share * 100)) : null;
  let label = `row ${row} column ${col}`;
  if (mark !== null) label += ` ${markName(mark)}`;
  else if (playable) label += ' playable';
  return { mark, status, playable, heat: share, heatAlpha, heatLabel, label };
}

export function smallBoardState(
  frame: UtttFrame,
  boardRow: number,
  boardCol: number,
): SmallBoardState {
  const state = frame.small[boardRow]?.[boardCol] ?? null;
  return { won: state === 0 || state === 1 ? state : null, drawn: state === 'draw' };
}

/** True when every cell is playable (the first move): a display cue only, so none stands out. */
export function allCellsPlayable(frame: UtttFrame): boolean {
  return frame.playable.length === 81;
}

/** How a finished game is described: the small boards each seat won, when the frame has them. */
export function resultDetail(frame: UtttFrame): string | null {
  const points: unknown = frame.points;
  if (
    !Array.isArray(points) ||
    points.length !== 2 ||
    !points.every((p) => typeof p === 'number' && Number.isFinite(p))
  ) {
    return null;
  }
  return `Small boards won: ${points[0]}–${points[1]}`;
}
