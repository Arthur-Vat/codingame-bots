import { createElement, type ComponentType } from 'react';
import UtttBoard from './uttt/Board';
import { asUtttFrame, resultDetail as utttResultDetail } from './uttt/view';

/* A game's renderer lives in `games/<game>/` and is listed here by the game's id (its folder
   name). The shell looks a game up with `renderers[id]`; a new game plugs in by adding one entry. */

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

export const renderers: Record<string, ComponentType<RendererProps>> = {
  uttt: Uttt,
};

/**
 * A game's own words for how a finished game ended (the renderer knows the game; the shell does
 * not compute rules), from the last frame. Optional; null when there is nothing to add.
 */
export const resultDetails: Record<string, (frame: unknown) => string | null> = {
  uttt: (frame) => {
    const checked = asUtttFrame(frame);
    return checked ? utttResultDetail(checked) : null;
  },
};
