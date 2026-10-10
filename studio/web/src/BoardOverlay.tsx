import type { Overlay } from './sessionView';

/** What covers the board while the server rewinds, compiles or measures, or two bots play. */
export function BoardOverlay({ overlay }: { overlay: Overlay | null }) {
  if (overlay === null) return null;
  if (overlay.kind === 'pill') {
    return (
      <div className="board-pill" role="status" data-testid="board-overlay">
        {overlay.text}
      </div>
    );
  }
  return (
    <div className="rewind" role="status" data-testid="board-overlay">
      <strong>{overlay.title}</strong>
      {overlay.text !== null && <p>{overlay.text}</p>}
      <div
        className={`bar${overlay.fraction === null ? ' indeterminate' : ''}`}
        role="progressbar"
        aria-label="Progress"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={overlay.fraction === null ? undefined : Math.round(overlay.fraction * 100)}
      >
        <i
          style={overlay.fraction === null ? undefined : { width: `${overlay.fraction * 100}%` }}
        />
      </div>
    </div>
  );
}
