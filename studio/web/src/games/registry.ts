import { createElement, type ComponentType } from 'react';
import UtttBoard from './uttt/Board';
import {
  asUtttFrame,
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

export const renderers: Record<string, ComponentType<RendererProps>> = byId({
  uttt: Uttt,
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
});

/**
 * A game's board with nothing played and nothing to play, drawn where there is no frame yet (no
 * game started, or a session still compiling). Optional: a game without one keeps the shell's
 * placeholder.
 */
export const emptyFrames: Record<string, () => unknown> = byId({
  uttt: utttEmptyFrame,
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

/** A game's empty board, if it has one. */
export function emptyFrameFor(gameId: string): unknown {
  return ownEntry(emptyFrames, gameId)?.();
}
