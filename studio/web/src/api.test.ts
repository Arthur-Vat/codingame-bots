import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import {
  ApiError,
  getAnalysis,
  listGames,
  startAnalysis,
  stopAnalysis,
  type AnalysisState,
} from './api';
import { setEngineKind } from './engine/detect';

interface Sent {
  url: string;
  method: string | undefined;
  body: string | undefined;
}

/** Stubs `fetch` with one answer and returns what was sent. */
function answering(status: number, body: unknown): Sent[] {
  const sent: Sent[] = [];
  vi.stubGlobal('fetch', (url: string, init?: RequestInit) => {
    sent.push({ url, method: init?.method, body: init?.body as string | undefined });
    return Promise.resolve(new Response(JSON.stringify(body), { status }));
  });
  return sent;
}

beforeEach(() => {
  setEngineKind('server');
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe('the games', () => {
  it('say whether each has an analysis program', async () => {
    answering(200, [{ id: 'uttt', name: 'U', releases: ['uttt-v001'], analysis: true }]);
    expect((await listGames())[0]?.analysis).toBe(true);
  });
});

describe('the analysis client', () => {
  it('starts an analysis with the request and the game, and gives the id', async () => {
    const sent = answering(200, { id: 'ab12' });
    const id = await startAnalysis({
      game: 'uttt',
      moves: ['4 4'],
      iterations: 1000,
      report_every: 1000,
    });
    expect(id).toBe('ab12');
    expect(sent).toHaveLength(1);
    expect(sent[0]?.url).toBe('/api/analysis');
    expect(sent[0]?.method).toBe('POST');
    expect(JSON.parse(sent[0]?.body ?? '')).toEqual({
      game: 'uttt',
      moves: ['4 4'],
      iterations: 1000,
      report_every: 1000,
    });
  });

  it('reads the state of an analysis, with the id encoded', async () => {
    const state: AnalysisState = { status: 'running', report: null, error: null };
    const sent = answering(200, state);
    expect(await getAnalysis('a b/c')).toEqual(state);
    expect(sent[0]).toMatchObject({ url: '/api/analysis/a%20b%2Fc', method: 'GET' });
  });

  it('gives a report as the server sent it, a move without a visit included', async () => {
    const state = {
      status: 'done',
      report: {
        iterations: 1000,
        nodes: 2000,
        to_move: 1,
        proven: null,
        wdl: [0.45, 0.1, 0.45],
        moves: [
          { move: '4 3', visits: 900, wdl: [0.4, 0.2, 0.4], proven: null, line: ['4 3', '3 1'] },
          { move: '0 0', visits: 0, wdl: null, proven: null, line: ['0 0'] },
        ],
        done: true,
        stop: 'iterations',
      },
      error: null,
    };
    answering(200, state);
    const got = await getAnalysis('ab12');
    expect(got.report?.moves[1]?.wdl).toBeNull();
    expect(got.report?.stop).toBe('iterations');
  });

  it('stops an analysis with a DELETE', async () => {
    const sent = answering(200, { deleted: true });
    await stopAnalysis('ab12');
    expect(sent[0]).toMatchObject({ url: '/api/analysis/ab12', method: 'DELETE' });
  });

  it('fails with the error of the answer', async () => {
    answering(404, { error: 'unknown analysis' });
    const failed = await getAnalysis('zzz').catch((error: unknown) => error);
    expect(failed).toBeInstanceOf(ApiError);
    expect(failed).toMatchObject({ message: 'unknown analysis', unreachable: false });
    answering(400, { error: 'unknown game "chess"' });
    await expect(startAnalysis({ game: 'chess', moves: [] })).rejects.toThrow(
      'unknown game "chess"',
    );
    answering(404, { error: 'unknown analysis' });
    await expect(stopAnalysis('zzz')).rejects.toThrow('unknown analysis');
  });
});
