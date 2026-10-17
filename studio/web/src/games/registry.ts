import { createElement, type ComponentType } from 'react';
import IceAndFireBoard from './ice-and-fire/Board';
import {
  asIceAndFireFrame,
  emptyFrame as iceAndFireEmptyFrame,
  resultDetail as iceAndFireResultDetail,
} from './ice-and-fire/view';
import { DEFAULT_SEAT_NAMES, type SeatNames } from '../sessionView';
import UtttBoard from './uttt/Board';
import {
  asUtttFrame,
  cellKey as utttCellKey,
  emptyFrame as utttEmptyFrame,
  resultDetail as utttResultDetail,
} from './uttt/view';

/* A game's renderer lives in `games/<game>/` and is listed here by the game's id (its folder
   name). The shell looks a game up with `renderers[id]`; a new game plugs in by adding one entry. */

/**
 * A map keyed by a game's id. A game id comes from the address or from a loaded file, so it may be
 * a name every object inherits (`constructor`, `__proto__`): the map has no prototype, and only
 * its own entries are found.
 */
function byId<T>(entries: Record<string, T>): Record<string, T> {
  return Object.assign(Object.create(null) as Record<string, T>, entries);
}

/** What the shell gives any renderer. `frame` is the adapter's JSON, which only the game reads. */
export interface RendererProps {
  frame: unknown;
  /** When true, the cells a player may play are clickable. */
  interactive: boolean;
  /**
   * A player chose an action. It is the game's own JSON, the `action` of one entry of the
   * session's `human_moves`; the shell matches it and sends the index.
   */
  onAction?: (action: unknown) => void;
  showCoordinates: boolean;
  /** Key "row,col" to share 0..1 of the search, drawn on legal cells only. */
  heat?: Map<string, number> | null;
  /** A move to outline, for the one hovered in a list of top moves. */
  highlight?: [number, number] | null;
}

function Uttt({ frame, onAction, ...rest }: RendererProps) {
  const checked = asUtttFrame(frame);
  return checked
    ? createElement(UtttBoard, {
        frame: checked,
        onCell: onAction ? (row: number, col: number) => onAction({ row, col }) : undefined,
        ...rest,
      })
    : null;
}

/** A game without human play: the board shows the position and takes no input. */
function IceAndFire({ frame, ...rest }: RendererProps) {
  const checked = asIceAndFireFrame(frame);
  return checked ? createElement(IceAndFireBoard, { frame: checked, ...rest }) : null;
}

export const renderers: Record<string, ComponentType<RendererProps>> = byId({
  uttt: Uttt,
  'ice-and-fire': IceAndFire,
});

/**
 * A game's own words for how a finished game ended (the renderer knows the game; the shell does
 * not compute rules), from the last frame. Optional; null when there is nothing to add.
 */
export const resultDetails: Record<string, (frame: unknown) => string | null> = byId({
  uttt: (frame: unknown) => {
    const checked = asUtttFrame(frame);
    return checked ? utttResultDetail(checked) : null;
  },
  'ice-and-fire': (frame: unknown) => {
    const checked = asIceAndFireFrame(frame);
    return checked ? iceAndFireResultDetail(checked) : null;
  },
});

/**
 * A game's board with nothing played and nothing to play, drawn where there is no frame yet (no
 * game started, or a session still compiling). Optional: a game without one keeps the shell's
 * placeholder.
 */
export const emptyFrames: Record<string, () => unknown> = byId({
  uttt: utttEmptyFrame,
  'ice-and-fire': iceAndFireEmptyFrame,
});

/**
 * The names a game gives its two seats (seat 0 moves first), where the shell writes a seat as a
 * letter or a name; a game without an entry has X and O.
 */
export const seatNames: Record<string, SeatNames> = byId({
  'ice-and-fire': ['Red', 'Blue'],
});

/** Where a move is on the board: the renderer's cell key (for its heat map) and the cell to outline. */
export interface MoveCell {
  key: string;
  at: [number, number];
}

/**
 * A game's reading of a move's answer text (as the analysis program writes it) into the cell it
 * plays, for the heat map and the outline; null for a text that is no move of the game. The shell
 * does not know a move's shape.
 */
export const moveCells: Record<string, (text: string) => MoveCell | null> = byId({
  uttt: (text: string) => {
    const match = /^([0-8]) ([0-8])$/.exec(text.trim());
    if (match === null) return null;
    const row = Number(match[1]);
    const col = Number(match[2]);
    return { key: utttCellKey(row, col), at: [row, col] };
  },
});

/**
 * A game's writing of an action of its renderer (what `onAction` gives) as the move's answer text,
 * the inverse of `moveCells`; null for something that is no action of the game. The shell does not
 * know an action's shape.
 */
export const actionTexts: Record<string, (action: unknown) => string | null> = byId({
  uttt: (action: unknown) => {
    if (typeof action !== 'object' || action === null) return null;
    const { row, col } = action as { row?: unknown; col?: unknown };
    const inside = (n: unknown): n is number =>
      Number.isInteger(n) && (n as number) >= 0 && (n as number) <= 8;
    return inside(row) && inside(col) ? `${row} ${col}` : null;
  },
});

/**
 * The entry of a map keyed by a game's id, or undefined. A game id comes from the address or from
 * a loaded file, so it may be a name every object inherits (`constructor`, `__proto__`): only an
 * entry the map owns counts.
 */
export function ownEntry<T>(map: Readonly<Record<string, T>>, gameId: string): T | undefined {
  return Object.hasOwn(map, gameId) ? map[gameId] : undefined;
}

/** A game's words for how a finished game ended, from the last frame, if it has any. */
export function resultDetailFor(gameId: string, frame: unknown): string | null {
  return ownEntry(resultDetails, gameId)?.(frame) ?? null;
}

/** The cell a move of a game plays, if the game can tell. */
export function moveCellFor(gameId: string, text: string): MoveCell | null {
  return ownEntry(moveCells, gameId)?.(text) ?? null;
}

/** The answer text of an action of a game's renderer, if the game can tell. */
export function actionTextFor(gameId: string, action: unknown): string | null {
  return ownEntry(actionTexts, gameId)?.(action) ?? null;
}

/** Whether a game can write its renderer's actions as moves (it can have side lines). */
export function hasActionTexts(gameId: string): boolean {
  return ownEntry(actionTexts, gameId) !== undefined;
}

/** The names of a game's two seats: its own, else X and O. */
export function seatNamesFor(gameId: string): SeatNames {
  return ownEntry(seatNames, gameId) ?? DEFAULT_SEAT_NAMES;
}

/** Whether a game names its seats itself (its marks are then coloured squares, not X and O). */
export function hasSeatNames(gameId: string): boolean {
  return ownEntry(seatNames, gameId) !== undefined;
}

/** A game's empty board, if it has one. */
export function emptyFrameFor(gameId: string): unknown {
  return ownEntry(emptyFrames, gameId)?.();
}
