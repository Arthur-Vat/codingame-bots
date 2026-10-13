import type { CSSProperties } from 'react';
import {
  chipOf,
  chipTitle,
  lineTokens,
  positionText,
  selectLines,
  statLine,
  type Chip,
  type ShownLine,
} from './analysisView';
import type { LineCount } from './analysisSettings';
import type { LiveState } from './liveAnalysis';

interface Props {
  /** `uttt-v010 engine`. */
  name: string;
  on: boolean;
  onToggle: (on: boolean) => void;
  live: LiveState;
  /** Why the evaluation chart stopped, if it did: said in the footer, and the lines go on. */
  chartProblem: string | null;
  lines: LineCount;
  onLines: (count: LineCount) => void;
  /** The number of turns played on the shown position: the ply the lines start at. */
  ply: number;
  /** The line the pointer is over, if any (its first move is outlined on the board). */
  onHover: (move: string | null) => void;
  /** A line was clicked: its moves. Null when the board cannot follow a line (the lines are then plain text). */
  onPick: ((line: string[]) => void) | null;
  /** Clicks on the lines are ignored for now (a side line is on its way). */
  pickDisabled: boolean;
  onKeep: (keep: boolean) => void;
}

/** What a line shows: its chip and its moves; a button when the board can follow the line. */
function LineBody({
  line,
  chip,
  index,
  ply,
  onPick,
  disabled,
}: {
  line: ShownLine;
  chip: Chip;
  index: number;
  ply: number;
  onPick: ((line: string[]) => void) | null;
  disabled: boolean;
}) {
  const content = (
    <>
      <span
        className={`engine-chip ${chip.side === 'X' ? 'x' : 'o'}`}
        title={chipTitle(line)}
        data-testid="engine-chip"
      >
        {chip.text}
      </span>
      <span className="engine-moves" data-testid={`engine-moves-${index}`}>
        {lineTokens(line.line, ply).map((token, at) =>
          token.kind === 'number' ? (
            <span key={at} className="n">
              {token.text}
            </span>
          ) : (
            <span key={at} className={token.first ? 'first' : undefined}>
              {token.text}{' '}
            </span>
          ),
        )}
      </span>
    </>
  );
  return onPick === null ? (
    <div className="engine-pick">{content}</div>
  ) : (
    <button
      type="button"
      className="engine-pick"
      disabled={disabled}
      onClick={() => onPick(line.line)}
    >
      {content}
    </button>
  );
}

/** The engine's panel: its switch, its stat line, Keep thinking, the best lines and the position's chances. */
export function EnginePanel({
  name,
  on,
  onToggle,
  live,
  chartProblem,
  lines,
  onLines,
  ply,
  onHover,
  onPick,
  pickDisabled,
  onKeep,
}: Props) {
  const { report } = live;
  const over = report?.stop === 'over';
  const shown = on && !over ? selectLines(report, lines) : [];
  const stat = on ? statLine({ elapsedMs: live.elapsedMs, report }) : '';
  const footer = on ? positionText(report) : null;
  // The program's own stop (its node limit, a proven position) leaves nothing to keep thinking about.
  const finished = live.status === 'done';
  const keeping = live.keep && live.status === 'running';
  // The program's message about this position replaces its lines; the chart's problem does not.
  const error = on ? live.error : null;

  return (
    <section className="panel engine" aria-label="Engine analysis" data-testid="engine-panel">
      <div className="engine-head">
        <label className="switch engine-switch">
          <input
            type="checkbox"
            aria-label={name}
            checked={on}
            onChange={(event) => onToggle(event.target.checked)}
          />
        </label>
        <span className="engine-name">
          <b>{name}</b>
          {stat !== '' && <small data-testid="engine-stat">{stat}</small>}
        </span>
        {on && (
          <button
            type="button"
            className={`engine-keep${keeping ? ' on' : ''}`}
            disabled={finished || over || live.status === 'idle' || live.status === 'error'}
            aria-pressed={keeping}
            onClick={() => onKeep(!keeping)}
          >
            {keeping ? 'Stop' : 'Keep thinking'}
          </button>
        )}
      </div>
      {on && error !== null && (
        <p className="engine-error" role="alert">
          {error}
        </p>
      )}
      {on && !over && error === null && (
        <ul
          className="engine-lines"
          style={{ '--n': lines } as CSSProperties}
          aria-label="Best lines"
        >
          {shown.map((line, index) => {
            const chip = chipOf(line.wdl);
            return (
              <li
                key={line.move}
                className="engine-line"
                data-testid="engine-line"
                onMouseEnter={() => onHover(line.move)}
                onMouseLeave={() => onHover(null)}
              >
                <LineBody
                  line={line}
                  chip={chip}
                  index={index}
                  ply={ply}
                  onPick={onPick}
                  disabled={pickDisabled}
                />
              </li>
            );
          })}
        </ul>
      )}
      {on && (footer !== null || !over) && (
        <div className="engine-foot">
          <span data-testid="engine-position">{footer}</span>
          {!over && (
            <button
              type="button"
              className="engine-more"
              onClick={() => onLines(lines === 1 ? 3 : 1)}
            >
              {lines === 1 ? 'Show 3 lines ▾' : 'Show 1 line ▴'}
            </button>
          )}
        </div>
      )}
      {on && chartProblem !== null && (
        <p className="engine-problem" data-testid="chart-problem">
          Chart stopped: {chartProblem}
        </p>
      )}
    </section>
  );
}
