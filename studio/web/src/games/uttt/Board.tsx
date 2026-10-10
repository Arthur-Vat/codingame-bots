import type { CSSProperties } from 'react';
import './board.css';
import {
  cellState,
  cellKey,
  isFreeChoice,
  prepareHeat,
  smallBoardState,
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
        className={`uttt-board${isFreeChoice(frame) ? ' free' : ''}`}
        role="grid"
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
                role="row"
                aria-label={`small board row ${boardRow} column ${boardCol}`}
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
                        role="gridcell"
                        aria-label={s.label}
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
                  {small.won === 0 && <XMark className="big-x" />}
                  {small.won === 1 && <OMark className="big-o" />}
                </div>
              </div>
            );
          }),
        )}
      </div>
    </div>
  );
}
