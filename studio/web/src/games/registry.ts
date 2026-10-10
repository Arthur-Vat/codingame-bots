import { createElement, type ComponentType } from 'react';
import UtttBoard from './uttt/Board';
import { asUtttFrame } from './uttt/view';

/* A game's renderer lives in `games/<game>/` and is listed here by the game's id (its folder
   name). The shell looks a game up with `renderers[id]`; a new game plugs in by adding one entry. */

/** What the shell gives any renderer. `frame` is the adapter's JSON, which only the game reads. */
export interface RendererProps {
  frame: unknown;
  /** When true, the cells a player may play are clickable. */
  interactive: boolean;
  onCell?: (row: number, col: number) => void;
  showCoordinates: boolean;
  /** Key "row,col" to share 0..1 of the search, drawn on legal cells only. */
  heat?: Map<string, number> | null;
  /** A move to outline, for the one hovered in a list of top moves. */
  highlight?: [number, number] | null;
}

function Uttt({ frame, ...rest }: RendererProps) {
  const checked = asUtttFrame(frame);
  return checked ? createElement(UtttBoard, { frame: checked, ...rest }) : null;
}

export const renderers: Record<string, ComponentType<RendererProps>> = {
  uttt: Uttt,
};
