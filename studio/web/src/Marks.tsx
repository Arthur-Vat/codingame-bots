/** The marks of the two seats, drawn on a seat-coloured square (seat 0 is X, seat 1 is O). */
export function SeatMark({ seat, className }: { seat: number; className?: string }) {
  const classes = `seat-${seat === 0 ? 0 : 1} ${className ?? ''}`.trim();
  return seat === 0 ? (
    <svg className={`mark-x ${classes}`} viewBox="0 0 100 100" aria-hidden="true">
      <line x1="24" y1="24" x2="76" y2="76" />
      <line x1="76" y1="24" x2="24" y2="76" />
    </svg>
  ) : (
    <svg className={`mark-o ${classes}`} viewBox="0 0 100 100" aria-hidden="true">
      <circle className="edge" cx="50" cy="50" r="28" />
      <circle className="ring" cx="50" cy="50" r="28" />
    </svg>
  );
}
