// The server's analysis routes, mocked: what the front end asks, and what it is told. A request
// with 50,000 iterations is the chart's and is answered done at once; any other is the live
// analysis and goes on running, its report growing at each read, until it is stopped.

import type { Page, Route } from '@playwright/test';

export interface AnalysisBody {
  game: string;
  moves: string[];
  iterations: number;
  report_every: number;
  depth?: number;
  seed: string;
}

export interface MockedAnalysis {
  /** Every request, in order. */
  posts: AnalysisBody[];
  /** The ids stopped with DELETE, in order. */
  deletes: string[];
  /** Reads of each analysis. */
  reads: Map<string, number>;
  /** The ids that exist: started and not stopped. */
  live: () => string[];
  /** The text of an error the next read answers with, for a live analysis. */
  failWith: (message: string | null) => void;
}

interface Options {
  /** A position with this many moves is a finished game (the program answers "over"). */
  overAt?: number;
  /** X's win chance of a position with `n` moves, for the chart. */
  chance?: (moves: number) => number;
}

/** The lines the mocked program reports for a position: the move to play, then two more. */
export const MOCK_LINES = [
  { move: '4 4', wdl: [0.58, 0.02, 0.4], visits: 9012, line: ['4 4', '4 3', '3 1', '1 5'] },
  { move: '2 2', wdl: [0.55, 0.05, 0.4], visits: 5000, line: ['2 2', '1 1', '0 0'] },
  { move: '0 0', wdl: [0.4, 0.09, 0.51], visits: 3000, line: ['0 0', '1 1'] },
];

function report(body: AnalysisBody, reads: number, options: Options) {
  const n = body.moves.length;
  if (options.overAt !== undefined && n === options.overAt) {
    return {
      iterations: 0,
      nodes: 0,
      to_move: null,
      proven: null,
      wdl: [0, 0, 1],
      moves: [],
      done: true,
      stop: 'over',
    };
  }
  const x = options.chance?.(n) ?? 0.5;
  if (body.iterations === 50_000) {
    return {
      iterations: 50_000,
      nodes: 1000,
      to_move: n % 2,
      proven: null,
      wdl: [x, 0, 1 - x],
      moves: [],
      done: true,
      stop: 'iterations',
    };
  }
  return {
    iterations: 20_000 * reads,
    nodes: 1000 * reads,
    to_move: n % 2,
    proven: null,
    wdl: [0.58, 0.02, 0.4],
    moves: MOCK_LINES.map((line) => ({ ...line, proven: null })),
    done: false,
    stop: null,
  };
}

export async function mockAnalysis(page: Page, options: Options = {}): Promise<MockedAnalysis> {
  const bodies = new Map<string, AnalysisBody>();
  let failure: string | null = null;
  let next = 1;
  const mock: MockedAnalysis = {
    posts: [],
    deletes: [],
    reads: new Map(),
    live: () => [...bodies.keys()].filter((id) => !mock.deletes.includes(id)),
    failWith: (message) => {
      failure = message;
    },
  };
  await page.route(/\/api\/analysis(\/|$)/, async (route: Route) => {
    const request = route.request();
    const id = decodeURIComponent(new URL(request.url()).pathname.split('/')[3] ?? '');
    if (request.method() === 'POST') {
      const body = request.postDataJSON() as AnalysisBody;
      mock.posts.push(body);
      const created = `m${next++}`;
      bodies.set(created, body);
      await route.fulfill({ json: { id: created } });
    } else if (request.method() === 'DELETE') {
      mock.deletes.push(id);
      await route.fulfill({ json: { deleted: true } });
    } else {
      const body = bodies.get(id);
      if (body === undefined || mock.deletes.includes(id)) {
        await route.fulfill({ status: 404, json: { error: 'unknown analysis' } });
        return;
      }
      const reads = (mock.reads.get(id) ?? 0) + 1;
      mock.reads.set(id, reads);
      if (failure !== null && body.iterations !== 50_000) {
        await route.fulfill({ json: { status: 'error', report: null, error: failure } });
        return;
      }
      const shown = report(body, reads, options);
      await route.fulfill({
        json: { status: shown.done ? 'done' : 'running', report: shown, error: null },
      });
    }
  });
  return mock;
}
