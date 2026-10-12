import type { CSSProperties } from 'react';
import './board.css';
import {
  cellState,
  cellKey,
  allCellsPlayable,
  prepareHeat,
  smallBoardState,
  winningLines,
  type HeatMap,
  type UtttFrame,
} from './view';

export type { UtttFrame } from './view';

export interface BoardProps {
  frame: UtttFrame;
  /** When true, playable cells are clickable and show the hover dot. */
  interactive: boolean;
  onCell?: (row: number, col: number) => void;
  showCoordinates: boolean;
  /** Key "r,c" to share 0..1, drawn on legal cells only. */
  heat?: HeatMap | null;
  /** Amber inset outline, for the move hovered in a list of top moves. */
  highlight?: [number, number] | null;
}

const NINE = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const THREE = [0, 1, 2];

function XMark({ className }: { className: string }) {
  return (
    <svg viewBox="0 0 100 100" className={className} aria-hidden="true">
      <line x1="22" y1="22" x2="78" y2="78" />
      <line x1="78" y1="22" x2="22" y2="78" />
    </svg>
  );
}

function OMark({ className }: { className: string }) {
  return (
    <svg viewBox="0 0 100 100" className={className} aria-hidden="true">
      <circle className="edge" cx="50" cy="50" r="30" />
      <circle className="ring" cx="50" cy="50" r="30" />
    </svg>
  );
}

/* The large marks of a closed small board span about 70% of it, the same for X and O. */
function BigX() {
  return (
    <svg viewBox="0 0 100 100" className="big-x" aria-hidden="true">
      <line x1="15" y1="15" x2="85" y2="85" />
      <line x1="85" y1="15" x2="15" y2="85" />
    </svg>
  );
}

function BigO() {
  return (
    <svg viewBox="0 0 100 100" className="big-o" aria-hidden="true">
      <circle cx="50" cy="50" r="33" />
    </svg>
  );
}

/* A won game's line, in a 300 x 300 box over the board: the small boards' centres are at 50, 150
   and 250 (to within a pixel or two), and the line runs a little past the two outer ones. */
const REACH = 24;

function WinLines({ frame }: { frame: UtttFrame }) {
  const lines = winningLines(frame);
  if (lines.length === 0) return null;
  const at = (n: number) => 50 + 100 * n;
  return (
    <svg className="uttt-win" viewBox="0 0 300 300" aria-hidden="true" data-testid="win-lines">
      {lines.map(({ from, to }) => {
        const dr = Math.sign(to[0] - from[0]);
        const dc = Math.sign(to[1] - from[1]);
        const length = Math.hypot(dr, dc);
        const coords = {
          x1: at(from[1]) - (dc / length) * REACH,
          y1: at(from[0]) - (dr / length) * REACH,
          x2: at(to[1]) + (dc / length) * REACH,
          y2: at(to[0]) + (dr / length) * REACH,
        };
        return (
          <g key={`${from[0]}${from[1]}${to[0]}${to[1]}`}>
            <line className="edge" {...coords} />
            <line className="dash" {...coords} />
          </g>
        );
      })}
    </svg>
  );
}

export default function Board({
  frame,
  interactive,
  onCell,
  showCoordinates,
  heat,
  highlight,
}: BoardProps) {
  const prepared = prepareHeat(frame, heat);
  return (
    <div className={`uttt${showCoordinates ? '' : ' coords-off'}`}>
      <div className="uttt-coords uttt-coords-top" aria-hidden="true">
        {NINE.map((i) => (
          <span key={i}>{i}</span>
        ))}
      </div>
      <div className="uttt-coords uttt-coords-left" aria-hidden="true">
        {NINE.map((i) => (
          <span key={i}>{i}</span>
        ))}
      </div>
      <div
        className={`uttt-board${allCellsPlayable(frame) ? ' free' : ''}`}
        role="group"
        aria-label="Ultimate Tic-Tac-Toe board"
      >
        {THREE.flatMap((boardRow) =>
          THREE.map((boardCol) => {
            const small = smallBoardState(frame, boardRow, boardCol);
            const kind =
              small.won === 0 ? ' won-x' : small.won === 1 ? ' won-o' : small.drawn ? ' drawn' : '';
            return (
              <div
                key={`${boardRow}${boardCol}`}
                className={`uttt-sb${kind}`}
                role="group"
                aria-label={`small board ${boardRow} ${boardCol}`}
              >
                {THREE.flatMap((dr) =>
                  THREE.map((dc) => {
                    const row = boardRow * 3 + dr;
                    const col = boardCol * 3 + dc;
                    const s = cellState(frame, row, col, prepared);
                    const legal = interactive && s.playable;
                    const classes = ['uttt-cell'];
                    if (s.status) classes.push(s.status);
                    if (s.heat !== null) classes.push('heat');
                    if (legal) classes.push('legal');
                    if (highlight && highlight[0] === row && highlight[1] === col) {
                      classes.push('hl');
                    }
                    const style =
                      s.heat !== null
                        ? ({ '--h': s.heatAlpha.toFixed(2) } as CSSProperties)
                        : undefined;
                    return (
                      <div
                        key={cellKey(row, col)}
                        className={classes.join(' ')}
                        style={style}
                        {...(legal ? {} : { role: 'img', 'aria-label': s.label })}
                      >
                        {s.mark === 0 && <XMark className="mark mark-x" />}
                        {s.mark === 1 && <OMark className="mark mark-o" />}
                        {s.heatLabel !== null && <span className="pct">{s.heatLabel}</span>}
                        {legal && (
                          <button
                            type="button"
                            className="cell-btn"
                            aria-label={`play row ${row} column ${col}`}
                            onClick={() => onCell?.(row, col)}
                          />
                        )}
                      </div>
                    );
                  }),
                )}
                <div className="sb-over" aria-hidden="true">
                  {small.won === 0 && <BigX />}
                  {small.won === 1 && <BigO />}
                </div>
              </div>
            );
          }),
        )}
      </div>
      <WinLines frame={frame} />
    </div>
  );
}
