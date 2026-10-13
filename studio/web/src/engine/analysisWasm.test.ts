import { describe, expect, it } from 'vitest';
import { analysisModule } from './testSupport';
import { runOnce } from './wasi';

interface ReportLine {
  report: {
    iterations: number;
    to_move: number | null;
    wdl: number[];
    moves: unknown[];
    done: boolean;
    stop: string | null;
  };
}

/** The lines the analysis program of Ultimate Tic-Tac-Toe answers to a request, parsed. */
async function analyse(
  request: unknown,
): Promise<{ code: number; lines: ReportLine[]; text: string }> {
  const module = await analysisModule('uttt');
  const { code, stdout } = runOnce(module, JSON.stringify(request));
  const lines = stdout
    .split('\n')
    .filter((line) => line !== '')
    .map((line) => JSON.parse(line) as ReportLine);
  return { code, lines, text: stdout };
}

describe("Ultimate Tic-Tac-Toe's analysis program, as WebAssembly", () => {
  it('streams reports and ends with a done report', async () => {
    const { code, lines, text } = await analyse({
      moves: ['4 4'],
      iterations: 3000,
      report_every: 1000,
    });
    expect(code).toBe(0);
    expect(text.endsWith('\n')).toBe(true);
    expect(lines).toHaveLength(3);
    const reports = lines.map((line) => line.report);
    expect(reports.map((report) => report.iterations)).toEqual([1000, 2000, 3000]);
    expect(reports.map((report) => report.done)).toEqual([false, false, true]);
    const final = reports[2];
    if (final === undefined) throw new Error('no report');
    expect(final.stop).toBe('iterations');
    expect(final.to_move).toBe(1);
    expect(final.moves).toHaveLength(8);
    expect(final.wdl.reduce((total, share) => total + share, 0)).toBeCloseTo(1, 9);
  });

  it('gives the same lines for the same request', async () => {
    const request = { moves: ['4 4', '4 3'], iterations: 2000, report_every: 1000, seed: '5' };
    expect((await analyse(request)).text).toBe((await analyse(request)).text);
  });

  it('answers a move that is not legal with one error line', async () => {
    const { code, lines } = await analyse({ moves: ['4 4', '9 9'] });
    expect(code).toBe(0);
    expect(lines).toEqual([{ error: 'move 2 ("9 9") is not legal' }]);
  });

  it('answers a request that is not JSON with one error line', async () => {
    const module = await analysisModule('uttt');
    const { code, stdout } = runOnce(module, 'not json');
    expect(code).toBe(0);
    expect(stdout.trimEnd().split('\n')).toHaveLength(1);
    expect(JSON.parse(stdout)).toHaveProperty('error');
  });
});
