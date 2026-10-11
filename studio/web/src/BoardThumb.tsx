/** A fixed picture of a game in progress for a card: no game logic, only a drawing. */
const CELLS = [
  '...X.O...',
  '.O.....X.',
  'X.O.X....',
  '..X.O.O..',
  '.X.OXO.X.',
  '..O.X.X..',
  '....O.X.O',
  '.X.....O.',
  'O...X.X..',
];

const GAP = 1.5;
const SMALL = (90 - GAP * 4) / 3;
const CELL = SMALL / 3;

export function BoardThumb() {
  const shapes = [];
  for (let board = 0; board < 9; board++) {
    const x0 = GAP + (board % 3) * (SMALL + GAP);
    const y0 = GAP + Math.floor(board / 3) * (SMALL + GAP);
    shapes.push(
      <rect key={`b${board}`} x={x0} y={y0} width={SMALL} height={SMALL} fill="var(--sq)" />,
    );
    for (let cell = 0; cell < 9; cell++) {
      const mark = CELLS[board]?.[cell];
      const cx = x0 + (cell % 3) * CELL + CELL / 2;
      const cy = y0 + Math.floor(cell / 3) * CELL + CELL / 2;
      if (mark === 'X') {
        const d = CELL * 0.26;
        shapes.push(
          <path
            key={`c${board}-${cell}`}
            d={`M${cx - d} ${cy - d}L${cx + d} ${cy + d}M${cx + d} ${cy - d}L${cx - d} ${cy + d}`}
            stroke="var(--x)"
            strokeWidth={CELL * 0.17}
            strokeLinecap="round"
          />,
        );
      } else if (mark === 'O') {
        shapes.push(
          <circle
            key={`c${board}-${cell}`}
            cx={cx}
            cy={cy}
            r={CELL * 0.28}
            fill="none"
            stroke="var(--o)"
            strokeWidth={CELL * 0.15}
          />,
        );
      }
    }
  }
  return (
    <svg viewBox="0 0 90 90" role="img" aria-label="A game in progress">
      <rect width="90" height="90" fill="#44515a" />
      {shapes}
    </svg>
  );
}
