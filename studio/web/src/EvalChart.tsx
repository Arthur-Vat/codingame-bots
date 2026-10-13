import type { PointerEvent } from 'react';
import {
  CHART_HEIGHT,
  CHART_PAD_X,
  CHART_WIDTH,
  chartPaths,
  chartX,
  chartY,
  computedCount,
  nearestFrame,
} from './analysisView';
import type { ChartValues } from './chartAnalysis';

interface Props {
  values: ChartValues;
  /** The frame on the board. */
  cursor: number;
  onGoto: (frame: number) => void;
}

/**
 * X's win chance after each move, as one SVG with no text: the curve and its fill, the 50% line
 * and fainter 0 and 100% lines, the part not computed yet shaded, and a line and a dot at the shown
 * frame. Clicking or dragging on it goes to the nearest frame. The plot is drawn in a fixed box
 * and stretched to the room it has (strokes keep their width on screen).
 */
export function EvalChart({ values, cursor, onGoto }: Props) {
  const last = values.length - 1;
  const done = computedCount(values);
  const { line, area } = chartPaths(values, last);
  const at = Math.min(Math.max(cursor, 0), Math.max(last, 0));
  const here = values[at] ?? null;
  const edge = done === 0 ? CHART_PAD_X : chartX(done - 1, last);
  const pending = done < values.length;

  const jump = (event: PointerEvent<SVGSVGElement>) => {
    const box = event.currentTarget.getBoundingClientRect();
    if (box.width <= 0) return;
    onGoto(nearestFrame((event.clientX - box.left) / box.width, last));
  };

  return (
    <div className="panel chart" data-testid="eval-chart">
      <svg
        viewBox={`0 0 ${CHART_WIDTH} ${CHART_HEIGHT}`}
        preserveAspectRatio="none"
        role="img"
        aria-label="Evaluation chart: X's win chance by move"
        onPointerDown={(event) => {
          event.currentTarget.setPointerCapture(event.pointerId);
          jump(event);
        }}
        onPointerMove={(event) => {
          if ((event.buttons & 1) === 1) jump(event);
        }}
      >
        <g className="grid">
          {[0, 1].map((chance) => (
            <line
              key={chance}
              className="faint"
              x1={CHART_PAD_X}
              x2={CHART_WIDTH - CHART_PAD_X}
              y1={chartY(chance)}
              y2={chartY(chance)}
            />
          ))}
          <line
            className="half"
            x1={CHART_PAD_X}
            x2={CHART_WIDTH - CHART_PAD_X}
            y1={chartY(0.5)}
            y2={chartY(0.5)}
          />
        </g>
        {pending && (
          <rect
            className="pending"
            data-testid="chart-pending"
            x={edge}
            y={chartY(1)}
            width={chartX(last, last) - edge}
            height={chartY(0) - chartY(1)}
          />
        )}
        {area !== '' && <path className="area" d={area} />}
        {line !== '' && <path className="curve" d={line} />}
        <line
          className="cursor"
          data-testid="chart-cursor"
          x1={chartX(at, last)}
          x2={chartX(at, last)}
          y1={chartY(1)}
          y2={chartY(0)}
        />
        {here !== null && (
          <path
            className="dot"
            data-testid="chart-dot"
            d={`M ${chartX(at, last)} ${chartY(here)} h 0`}
          />
        )}
      </svg>
    </div>
  );
}
