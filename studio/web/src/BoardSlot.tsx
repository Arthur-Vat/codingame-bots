import type { ReactNode } from 'react';

const INDEXES = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/**
 * The square a game's board is drawn in. The coordinates always take their room (hidden or
 * not), and the square is as wide as its column allows up to the row's limit, so no toggle
 * changes its size. A game's board renderer goes in as `children`.
 *
 * A renderer that draws its own coordinates sets `bare`: the slot then has no coordinates and no
 * padding of its own, and the renderer fills it. `overlay` covers the slot (a rewind notice, a
 * line over a game between two bots) without changing its size.
 */
export function BoardSlot({
  showCoordinates,
  label,
  bare = false,
  overlay,
  children,
}: {
  showCoordinates: boolean;
  label: string;
  bare?: boolean;
  overlay?: ReactNode;
  children?: ReactNode;
}) {
  const classes = ['board-wrap'];
  if (showCoordinates) classes.push('coords');
  if (bare) classes.push('bare');
  return (
    <div className={classes.join(' ')} data-testid="board-wrap">
      {!bare && (
        <>
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
        </>
      )}
      <div
        className={`board-slot${children === undefined || children === null ? ' empty' : ''}`}
        role="group"
        aria-label={label}
        data-testid="board-slot"
      >
        {children ?? 'The board appears here.'}
      </div>
      {overlay}
    </div>
  );
}
