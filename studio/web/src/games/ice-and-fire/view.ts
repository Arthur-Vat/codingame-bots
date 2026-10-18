/* Pure helpers for A Code of Ice and Fire's board: the frame's shape check and the words and
   geometry the renderer draws. They hold no rules; the frame comes from the adapter
   (games/ice-and-fire/studio). Seat 0 is red (HQ at 0,0, plays first), seat 1 is blue. */

export type Seat = 0 | 1;
export type Kind = 'hq' | 'mine' | 'tower';
export type Point = [number, number];
export type Level = 1 | 2 | 3;

/** The map is 12 x 12 cells, x along a row, y down the rows. */
export const SIZE = 12;
/** A game lasts this many game turns. */
export const TURNS = 100;

export interface Building {
  owner: Seat;
  kind: Kind;
  x: number;
  y: number;
}

export interface Unit {
  id: number;
  owner: Seat;
  level: Level;
  x: number;
  y: number;
}

export interface Last {
  seat: Seat;
  answer: string;
  moved: { id: number; from: Point; to: Point }[];
  trained: { id: number; level: Level; x: number; y: number }[];
  built: { kind: 'mine' | 'tower'; x: number; y: number }[];
  removed: { id: number; owner: Seat; level: Level; x: number; y: number }[];
}

/** The JSON frame of games/ice-and-fire/studio. Grids are 12 rows (y) of 12 entries (x). */
export interface IceAndFireFrame {
  void: boolean[][];
  owner: (Seat | null)[][];
  active: boolean[][];
  mine_spots: Point[];
  buildings: Building[];
  units: Unit[];
  gold: [number, number];
  income: [number, number];
  military: [number, number];
  turn: number;
  to_move: Seat | null;
  last: Last | null;
  result: null | { winner: Seat | null; end: 'hq' | 'turns' };
}

/* The shape check. Frames come from records users load, so nothing is taken on trust. */

/** No list in a frame is longer than the map has cells. */
const MAX_ITEMS = SIZE * SIZE;

const isObject = (v: unknown): v is Record<string, unknown> =>
  typeof v === 'object' && v !== null && !Array.isArray(v);
const isInt = (v: unknown): v is number => typeof v === 'number' && Number.isInteger(v);
const isCoord = (v: unknown): v is number => isInt(v) && v >= 0 && v < SIZE;
const isSeat = (v: unknown): v is Seat => v === 0 || v === 1;
const isLevel = (v: unknown): v is Level => v === 1 || v === 2 || v === 3;
const isId = (v: unknown): v is number => isInt(v) && v >= 0;
const isBool = (v: unknown): v is boolean => typeof v === 'boolean';
const isOwner = (v: unknown): boolean => v === null || isSeat(v);

function allOf(v: unknown, check: (item: unknown) => boolean): boolean {
  return Array.isArray(v) && v.length <= MAX_ITEMS && v.every(check);
}

function isGrid(v: unknown, check: (cell: unknown) => boolean): boolean {
  return (
    Array.isArray(v) &&
    v.length === SIZE &&
    v.every((row) => Array.isArray(row) && row.length === SIZE && row.every(check))
  );
}

function isPoint(v: unknown): boolean {
  return Array.isArray(v) && v.length === 2 && isCoord(v[0]) && isCoord(v[1]);
}

/** A pair of whole numbers: a value for each seat. */
function isPair(v: unknown): boolean {
  return Array.isArray(v) && v.length === 2 && isInt(v[0]) && isInt(v[1]);
}

function isLast(v: unknown): boolean {
  if (!isObject(v) || !isSeat(v['seat']) || typeof v['answer'] !== 'string') return false;
  return (
    allOf(
      v['moved'],
      (m) => isObject(m) && isId(m['id']) && isPoint(m['from']) && isPoint(m['to']),
    ) &&
    allOf(
      v['trained'],
      (t) =>
        isObject(t) && isId(t['id']) && isLevel(t['level']) && isCoord(t['x']) && isCoord(t['y']),
    ) &&
    allOf(
      v['built'],
      (b) =>
        isObject(b) &&
        (b['kind'] === 'mine' || b['kind'] === 'tower') &&
        isCoord(b['x']) &&
        isCoord(b['y']),
    ) &&
    allOf(
      v['removed'],
      (r) =>
        isObject(r) &&
        isId(r['id']) &&
        isSeat(r['owner']) &&
        isLevel(r['level']) &&
        isCoord(r['x']) &&
        isCoord(r['y']),
    )
  );
}

function isResult(v: unknown): boolean {
  return isObject(v) && isOwner(v['winner']) && (v['end'] === 'hq' || v['end'] === 'turns');
}

/** Reads `value` as a frame, or gives null when it does not have a frame's exact shape. */
export function asIceAndFireFrame(value: unknown): IceAndFireFrame | null {
  if (!isObject(value)) return null;
  const v = value;
  const turn = v['turn'];
  const ok =
    isGrid(v['void'], isBool) &&
    isGrid(v['owner'], isOwner) &&
    isGrid(v['active'], isBool) &&
    allOf(v['mine_spots'], isPoint) &&
    allOf(
      v['buildings'],
      (b) =>
        isObject(b) &&
        isSeat(b['owner']) &&
        (b['kind'] === 'hq' || b['kind'] === 'mine' || b['kind'] === 'tower') &&
        isCoord(b['x']) &&
        isCoord(b['y']),
    ) &&
    allOf(
      v['units'],
      (u) =>
        isObject(u) &&
        isId(u['id']) &&
        isSeat(u['owner']) &&
        isLevel(u['level']) &&
        isCoord(u['x']) &&
        isCoord(u['y']),
    ) &&
    isPair(v['gold']) &&
    isPair(v['income']) &&
    isPair(v['military']) &&
    isInt(turn) &&
    turn >= 1 &&
    turn <= TURNS &&
    isOwner(v['to_move']) &&
    (v['last'] === null || isLast(v['last'])) &&
    (v['result'] === null || isResult(v['result']));
  return ok ? (v as unknown as IceAndFireFrame) : null;
}

/** The board before anything is played and nothing can be: bare cells, nothing on them. */
export function emptyFrame(): IceAndFireFrame {
  const grid = <T>(fill: T): T[][] => Array.from({ length: SIZE }, () => Array<T>(SIZE).fill(fill));
  return {
    void: grid(false),
    owner: grid<Seat | null>(null),
    active: grid(false),
    mine_spots: [],
    buildings: [],
    units: [],
    gold: [0, 0],
    income: [0, 0],
    military: [0, 0],
    turn: 1,
    to_move: null,
    last: null,
    result: null,
  };
}

export const seatName = (seat: Seat): string => (seat === 0 ? 'Red' : 'Blue');

const KIND_NAMES: Record<Kind, string> = { hq: 'HQ', mine: 'mine', tower: 'tower' };
export const kindName = (kind: Kind): string => KIND_NAMES[kind];

/** A number with its sign: +5, +0, -3. */
export const signed = (n: number): string => (n < 0 ? `-${-n}` : `+${n}`);

/** The turn counter: `turn 12 / 100`. */
export const turnText = (frame: IceAndFireFrame): string => `turn ${frame.turn} / ${TURNS}`;

/** Whose move it is, or that the game is over; empty when there is no game. */
export function toMoveText(frame: IceAndFireFrame): string {
  if (frame.to_move !== null) return `${seatName(frame.to_move)} to move`;
  return frame.result === null ? '' : 'Game over';
}

/** The last answer with the seat that gave it; empty at the start. */
export function answerText(frame: IceAndFireFrame): string {
  if (frame.last === null) return '';
  const text = frame.last.answer.trim();
  return `${seatName(frame.last.seat)}: ${text === '' ? '(empty answer)' : text}`;
}

/** The text a screen reader gets for the whole board. */
export function boardLabel(frame: IceAndFireFrame): string {
  return [
    'A Code of Ice and Fire board',
    turnText(frame),
    toMoveText(frame),
    `gold ${frame.gold[0]} red, ${frame.gold[1]} blue`,
    `military power ${frame.military[0]} red, ${frame.military[1]} blue`,
  ]
    .filter((part) => part !== '')
    .join(', ');
}

/** How a finished game ended, from the frame's result; null while it is not over. */
export function resultDetail(frame: IceAndFireFrame): string | null {
  const result: unknown = frame.result;
  if (!isObject(result)) return null;
  const { winner, end } = result;
  if (end === 'hq') {
    return isSeat(winner)
      ? `${seatName(winner)} captured ${seatName(winner === 0 ? 1 : 0)}'s HQ.`
      : null;
  }
  if (end === 'turns') {
    return isSeat(winner)
      ? `${seatName(winner)} had more military power after ${TURNS} turns.`
      : `Equal military power after ${TURNS} turns.`;
  }
  return null;
}

/** The centre of a cell, in the board's units (a cell is 1 x 1). */
export const centre = ([x, y]: Point): Point => [x + 0.5, y + 0.5];

/**
 * An arrow between two cells' centres: the shaft, kept off the unit's disc, and the head's
 * corners. Null when both ends are one cell.
 */
export function arrow(
  from: Point,
  to: Point,
): { x1: number; y1: number; x2: number; y2: number; head: string } | null {
  const [fx, fy] = centre(from);
  const [tx, ty] = centre(to);
  const length = Math.hypot(tx - fx, ty - fy);
  if (length === 0) return null;
  const ux = (tx - fx) / length;
  const uy = (ty - fy) / length;
  const size = 0.3;
  const tipX = tx - ux * 0.15;
  const tipY = ty - uy * 0.15;
  const baseX = tipX - ux * size;
  const baseY = tipY - uy * size;
  const half = size * 0.55;
  const head = [
    [tipX, tipY],
    [baseX - uy * half, baseY + ux * half],
    [baseX + uy * half, baseY - ux * half],
  ]
    .map(([px, py]) => `${px?.toFixed(3)},${py?.toFixed(3)}`)
    .join(' ');
  return { x1: fx + ux * 0.3, y1: fy + uy * 0.3, x2: baseX, y2: baseY, head };
}
