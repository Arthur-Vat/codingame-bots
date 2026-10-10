import type { ReactNode } from 'react';

const INDEXES = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/**
 * The square a game's board is drawn in. The coordinates always take their room (hidden or
 * not), and the square is as wide as its column allows up to the row's limit, so no toggle
 * changes its size. A game's board renderer goes in as `children`.
 */
export function BoardSlot({
  showCoordinates,
  label,
  children,
}: {
  showCoordinates: boolean;
  label: string;
  children?: ReactNode;
}) {
  return (
    <div className={`board-wrap${showCoordinates ? ' coords' : ''}`} data-testid="board-wrap">
      <div className="coords-top" aria-hidden="true">
        {INDEXES.map((i) => (
          <span key={i}>{i}</span>
        ))}
      </div>
      <div className="coords-left" aria-hidden="true">
        {INDEXES.map((i) => (
          <span key={i}>{i}</span>
        ))}
      </div>
      <div className="board-slot" role="group" aria-label={label} data-testid="board-slot">
        {children ?? 'The board appears here.'}
      </div>
    </div>
  );
}
