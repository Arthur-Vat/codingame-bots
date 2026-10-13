/* What the engine panel, the win/draw/loss bar, the heat map and the evaluation chart show, as pure
   functions of the analysis program's reports (studio/README.md, "The analysis program"). Reports
   come from our program, but every field is still read with a guard: a missing `wdl` or `moves`
   shows nothing rather than breaking the page. */

import type { AnalysisMove, AnalysisReport, Chances } from './api';
import type { AnalysisModel } from './analysis';
import type { MoveCell } from './games/registry';

/** The program takes at most this many moves in a request. */
export const MAX_REQUEST_MOVES = 200;

/**
 * The moves of a request for the position on frame `frame`: frame `i` shows the turns up to
 * `openingTurns + i`, so those turns, openings included, each as the acting answer's lines joined
 * by "\n". Null when the position cannot be asked: a frame outside the game, a turn that has not
 * exactly one answer (the protocol is for games where one seat acts at a time), or more moves than
 * the program takes.
 */
export function requestMoves(
  model: Pick<AnalysisModel, 'turns' | 'openingTurns'>,
  frame: number,
): string[] | null {
  if (!Number.isInteger(frame) || frame < 0) return null;
  const count = model.openingTurns + frame;
  if (count > model.turns.length || count > MAX_REQUEST_MOVES) return null;
  const moves: string[] = [];
  for (const turn of model.turns.slice(0, count)) {
    const answer = turn.length === 1 ? turn[0] : undefined;
    if (answer === undefined) return null;
    moves.push(answer.lines.join('\n'));
  }
  return moves;
}

/** The moves of every frame's request, 0 to `last`; null when any of them cannot be asked. */
export function chartRequests(
  model: Pick<AnalysisModel, 'turns' | 'openingTurns'>,
  last: number,
): string[][] | null {
  const all: string[][] = [];
  for (let frame = 0; frame <= last; frame++) {
    const moves = requestMoves(model, frame);
    if (moves === null) return null;
    all.push(moves);
  }
  return all;
}

/** The mark of a seat. */
export function sideName(seat: number): 'X' | 'O' {
  return seat === 0 ? 'X' : 'O';
}

/** `wdl` when it is three finite numbers, else null. */
export function validWdl(value: unknown): Chances | null {
  if (!Array.isArray(value) || value.length !== 3) return null;
  const [x, d, o] = value as unknown[];
  return [x, d, o].every((n) => typeof n === 'number' && Number.isFinite(n))
    ? [x as number, d as number, o as number]
    : null;
}

/** A chance from 0 to 1 as a whole percentage. */
export function percent(chance: number): number {
  if (!Number.isFinite(chance)) return 0;
  return Math.round(Math.min(1, Math.max(0, chance)) * 100);
}

/** The side ahead and its win chance, for the chip of a line. */
export interface Chip {
  side: 'X' | 'O';
  percent: number;
  /** `X 58%`. */
  text: string;
}

/** The chip of a move's chances: the side with the better win chance (X when equal). */
export function chipOf(wdl: Chances): Chip {
  const side = wdl[2] > wdl[0] ? 'O' : 'X';
  const chance = percent(side === 'X' ? wdl[0] : wdl[2]);
  return { side, percent: chance, text: `${side} ${chance}%` };
}

/** `X 58% · draw 2% · O 40%`. */
export function wdlText(wdl: Chances): string {
  return `X ${percent(wdl[0])}% · draw ${percent(wdl[1])}% · O ${percent(wdl[2])}%`;
}

/** A count with thousands separators, the same in every browser: `812,000`. */
export function formatCount(count: number): string {
  return Math.round(count)
    .toString()
    .replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

/** The tooltip of a line's chip: `X 58% · draw 2% · O 40% · 9,012 visits`. */
export function chipTitle(move: Pick<AnalysisMove, 'visits'> & { wdl: Chances }): string {
  const visits = move.visits === 1 ? '1 visit' : `${formatCount(move.visits)} visits`;
  return `${wdlText(move.wdl)} · ${visits}`;
}

/** A line to show: a move the search has visited, with its chances and its moves. */
export interface ShownLine {
  /** The move the line starts with (the answer text). */
  move: string;
  line: string[];
  wdl: Chances;
  visits: number;
}

/**
 * The first `count` moves of a report that have visits and chances (the report orders them: the
 * move the search would play first, then by visits), with their lines. A move with no `line` is a
 * line of that move alone.
 */
export function selectLines(report: AnalysisReport | null, count: number): ShownLine[] {
  const shown: ShownLine[] = [];
  if (report === null || !Array.isArray(report.moves)) return shown;
  for (const entry of report.moves) {
    if (shown.length >= count) break;
    const wdl = validWdl(entry.wdl);
    if (wdl === null || !(entry.visits > 0) || typeof entry.move !== 'string') continue;
    const line = Array.isArray(entry.line) && entry.line.length > 0 ? entry.line : [entry.move];
    shown.push({ move: entry.move, line, wdl, visits: entry.visits });
  }
  return shown;
}

/** A piece of a line as written: a move number, or a move. */
export type LineToken =
  { kind: 'number'; text: string } | { kind: 'move'; text: string; first: boolean };

/**
 * A line as the list of its pieces. `startPly` is the number of turns played before its first
 * move; ply `i` (0-based over all turns, openings included) has the number `floor(i / 2) + 1`: X's
 * moves (even plies) are preceded by `9.`, and a line that starts with O's move begins `8…`.
 */
export function lineTokens(line: readonly string[], startPly: number): LineToken[] {
  const tokens: LineToken[] = [];
  line.forEach((text, index) => {
    const ply = startPly + index;
    const number = Math.floor(ply / 2) + 1;
    if (ply % 2 === 0) tokens.push({ kind: 'number', text: `${number}.` });
    else if (index === 0) tokens.push({ kind: 'number', text: `${number}…` });
    tokens.push({ kind: 'move', text, first: index === 0 });
  });
  return tokens;
}

/** Each legal move's share of the visits, by the renderer's cell key; null when nothing was visited. */
export function heatShares(
  report: AnalysisReport | null,
  cellOf: (text: string) => MoveCell | null,
): Map<string, number> | null {
  if (report === null || !Array.isArray(report.moves)) return null;
  let total = 0;
  for (const entry of report.moves) {
    if (entry.visits > 0) total += entry.visits;
  }
  if (!(total > 0)) return null;
  const shares = new Map<string, number>();
  for (const entry of report.moves) {
    if (!(entry.visits > 0)) continue;
    const cell = cellOf(entry.move);
    if (cell !== null) shares.set(cell.key, entry.visits / total);
  }
  return shares.size === 0 ? null : shares;
}

/** `2.0 s`: time spent on a position, in tenths of a second. */
export function formatThinking(ms: number): string {
  return `${(Math.max(0, ms) / 1000).toFixed(1)} s`;
}

/** What the header's stat line needs. */
export interface StatInput {
  elapsedMs: number;
  report: AnalysisReport | null;
}

/**
 * The stat line: the time thinking on this position, then the latest report's iterations; or who
 * the position is proven for, or that the program stopped at its memory limit.
 */
export function statLine({ elapsedMs, report }: StatInput): string {
  const time = formatThinking(elapsedMs);
  if (report === null) return time;
  if (report.stop === 'over') return '';
  if (report.proven === 0 || report.proven === 1) {
    return `${time} · proven: ${sideName(report.proven)} wins`;
  }
  if (report.stop === 'nodes') return `${time} · memory limit reached`;
  return `${time} · ${formatCount(report.iterations)} iterations`;
}

/** The footer's position text: who moves and the position's chances, or the result of a finished game. */
export function positionText(report: AnalysisReport | null): string | null {
  const wdl = report === null ? null : validWdl(report.wdl);
  if (report === null || wdl === null) return null;
  if (report.stop === 'over' || report.to_move === null) {
    return `Game over · ${wdl[0] > wdl[2] ? 'X wins' : wdl[2] > wdl[0] ? 'O wins' : 'draw'}`;
  }
  return `${sideName(report.to_move)} to move · ${wdlText(wdl)}`;
}

/** The bar's three parts from the position's chances, as shares of its height: O on top, draw, X below. */
export function barShares(wdl: Chances): { o: number; draw: number; x: number } {
  const clean = wdl.map((n) => Math.max(0, n));
  const sum = clean[0]! + clean[1]! + clean[2]!;
  if (!(sum > 0)) return { o: 0, draw: 1, x: 0 };
  return { o: clean[2]! / sum, draw: clean[1]! / sum, x: clean[0]! / sum };
}

/** What the bar says to a screen reader and in its tooltip. */
export function barLabel(wdl: Chances): string {
  return `Win, draw and loss: ${wdlText(wdl)}`;
}

/* The evaluation chart's geometry. The plot is drawn in a box of CHART_WIDTH by CHART_HEIGHT units,
   stretched to the room it has, so that nothing here depends on the page's pixels. */

export const CHART_WIDTH = 1000;
export const CHART_HEIGHT = 100;
/** Room kept beside the plot for the dot and the lines' ends, in units. */
export const CHART_PAD_X = 8;
export const CHART_PAD_Y = 6;

/** The x of frame `frame` among `last + 1` frames. */
export function chartX(frame: number, last: number): number {
  const span = CHART_WIDTH - 2 * CHART_PAD_X;
  return CHART_PAD_X + (last <= 0 ? span / 2 : (span * frame) / last);
}

/** The y of a chance from 0 (bottom) to 1 (top). */
export function chartY(chance: number): number {
  const clamped = Math.min(1, Math.max(0, chance));
  return CHART_PAD_Y + (CHART_HEIGHT - 2 * CHART_PAD_Y) * (1 - clamped);
}

/** The frame nearest to a point at `fraction` (0 to 1) of the chart's width. */
export function nearestFrame(fraction: number, last: number): number {
  if (last <= 0 || !Number.isFinite(fraction)) return 0;
  const unit = fraction * CHART_WIDTH;
  const span = CHART_WIDTH - 2 * CHART_PAD_X;
  const frame = Math.round(((unit - CHART_PAD_X) / span) * last);
  return Math.min(last, Math.max(0, frame));
}

/** How many frames, from the first, have a value: the chart is filled in order. */
export function computedCount(values: readonly (number | null)[]): number {
  let count = 0;
  while (count < values.length && values[count] !== null) count++;
  return count;
}

/** The curve of the computed values, and the area under it; empty paths for fewer than two values. */
export function chartPaths(
  values: readonly (number | null)[],
  last: number,
): { line: string; area: string } {
  const count = computedCount(values);
  if (count < 2) return { line: '', area: '' };
  const points = values
    .slice(0, count)
    .map((value, frame) => `${chartX(frame, last).toFixed(1)} ${chartY(value ?? 0).toFixed(1)}`);
  const line = `M ${points.join(' L ')}`;
  const bottom = chartY(0).toFixed(1);
  const area = `${line} L ${chartX(count - 1, last).toFixed(1)} ${bottom} L ${chartX(0, last).toFixed(1)} ${bottom} Z`;
  return { line, area };
}
