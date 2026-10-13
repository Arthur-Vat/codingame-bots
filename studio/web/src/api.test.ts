import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import {
  ApiError,
  getAnalysis,
  getHistoryRecordText,
  getHistoryView,
  getRecordText,
  getSession,
  listGames,
  postHistoryText,
  postViewText,
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

/** Stubs `fetch` with one answer given as text, exactly as the server wrote it. */
function answeringText(status: number, text: string): Sent[] {
  const sent: Sent[] = [];
  vi.stubGlobal('fetch', (url: string, init?: RequestInit) => {
    sent.push({ url, method: init?.method, body: init?.body as string | undefined });
    return Promise.resolve(new Response(text, { status }));
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

const BIG = '18446744073709551557';
const BOT = '9007199254740993';
const recordText = `{"format":1,"game":"uttt","seed":${BIG},"players":[{"name":"a","bot_seed":${BOT}},{"name":"b","bot_seed":null}],"turns":[]}`;

describe('records with 64-bit seeds', () => {
  it('gives the record of a session and of a saved game as the exact text', async () => {
    const sent = answeringText(200, recordText);
    expect(await getRecordText('ab12')).toBe(recordText);
    expect(sent[0]).toMatchObject({ url: '/api/sessions/ab12/record', method: 'GET' });
    answeringText(200, recordText);
    expect(await getHistoryRecordText('uttt-1-aa')).toBe(recordText);
    const pretty = `{\r\n  "seed": ${BIG}\r\n}\r\n`;
    answeringText(200, pretty);
    expect(await getRecordText('ab12')).toBe(pretty);
  });

  it('fails as before when the record cannot be had', async () => {
    answering(404, { error: 'unknown session' });
    const failed = await getRecordText('zzz').catch((error: unknown) => error);
    expect(failed).toBeInstanceOf(ApiError);
    expect(failed).toMatchObject({ message: 'unknown session', unreachable: false });
    answering(404, { error: 'unknown game' });
    await expect(getHistoryRecordText('zzz')).rejects.toThrow('unknown game');
    answeringText(500, '');
    await expect(getRecordText('ab12')).rejects.toMatchObject({
      message: 'The server sent something that is not JSON (HTTP 500).',
      unreachable: true,
    });
    answering(500, {});
    await expect(getRecordText('ab12')).rejects.toThrow('HTTP 500');
  });

  it('sends the text of a record unchanged to be viewed or saved', async () => {
    const saved = answeringText(200, '{"id":"x","duplicate":false}');
    await postHistoryText(recordText);
    expect(saved[0]).toMatchObject({ url: '/api/history', method: 'POST', body: recordText });
    const viewed = answeringText(200, '{"record":{"seed":1,"players":[]},"frames":[]}');
    await postViewText(recordText);
    expect(viewed[0]).toMatchObject({ url: '/api/view', method: 'POST', body: recordText });
  });

  it('shows the exact seeds of a view and of a session', async () => {
    answeringText(
      200,
      `{"record":{"seed":${BIG},"players":[{"bot_seed":${BOT}},{"bot_seed":null}],"unix_time":1760000000},"frames":[]}`,
    );
    const view = await postViewText(recordText);
    expect(view.record.seed).toBe(BIG);
    expect(view.record.players[0].bot_seed).toBe(BOT);
    expect(view.record.players[1].bot_seed).toBeNull();
    expect(view.record.unix_time).toBe(1760000000);
    answeringText(200, `{"record":{"seed":${BIG}}}`);
    expect((await getHistoryView('uttt-1-aa')).record.seed).toBe(BIG);
    answeringText(200, `{"id":"s","seed":${BIG}}`);
    expect((await getSession('s')).seed).toBe(BIG);
    answeringText(200, '{"id":"s","seed":1760000000000}');
    expect((await getSession('s')).seed).toBe(1760000000000);
  });
});
