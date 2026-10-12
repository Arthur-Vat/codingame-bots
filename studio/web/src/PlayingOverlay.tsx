/** Over the board while two bots play: a spinning circle, so the viewer knows the game goes on. */
export function PlayingOverlay() {
  return (
    <div className="playing" role="status" aria-label="The bots are playing" data-testid="playing">
      <div className="playing-chip">
        <span className="spinner" aria-hidden="true" />
        <small>Playing…</small>
      </div>
    </div>
  );
}
