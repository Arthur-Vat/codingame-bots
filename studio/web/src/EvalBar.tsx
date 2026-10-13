import { barLabel, barShares, validWdl } from './analysisView';
import type { AnalysisReport } from './api';

/**
 * The win, draw and loss bar, right of the board and as tall as its squares: O on top (light),
 * draw in the middle, X at the bottom (dark). Without a report (the engine off, a game without
 * analysis, nothing found yet) the bar is hidden but keeps its place, so the board never moves.
 */
export function EvalBar({ report }: { report: AnalysisReport | null }) {
  const wdl = validWdl(report?.wdl);
  if (wdl === null) return <div className="evalbar off" aria-hidden="true" />;
  const shares = barShares(wdl);
  const label = barLabel(wdl);
  return (
    <div className="evalbar" role="img" aria-label={label} title={label} data-testid="evalbar">
      <div className="o" style={{ flexGrow: shares.o }} data-testid="evalbar-o" />
      <div className="d" style={{ flexGrow: shares.draw }} data-testid="evalbar-draw" />
      <div className="x" style={{ flexGrow: shares.x }} data-testid="evalbar-x" />
    </div>
  );
}
