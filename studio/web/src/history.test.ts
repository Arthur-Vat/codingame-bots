import { describe, expect, it } from 'vitest';
import { ApiError, type HistoryEntry } from './api';
import {
  emptyText,
  faultTag,
  filtersActive,
  historyQuery,
  importFiles,
  NO_FILTERS,
  parseRecordText,
  releaseOptions,
  resultSymbol,
  sourceTag,
  summaryText,
} from './history';

const entry = (patch: Partial<HistoryEntry> = {}): HistoryEntry => ({
  id: 'uttt-1-abc',
  game: 'uttt',
  unix_time: 1_760_000_000,
  players: ['uttt-v010', 'uttt-v009'],
  winner: 0,
  end: 'finished',
  turns: 41,
  source: 'arena match',
  ...patch,
});

describe('historyQuery', () => {
  it('is empty without filters', () => {
    expect(historyQuery(NO_FILTERS)).toBe('');
    expect(filtersActive(NO_FILTERS)).toBe(false);
  });

  it('maps each filter to the server parameter', () => {
    expect(historyQuery({ ...NO_FILTERS, release: 'uttt-v009' })).toBe('?release=uttt-v009');
    expect(historyQuery({ ...NO_FILTERS, result: 'x' })).toBe('?result=x');
    expect(historyQuery({ ...NO_FILTERS, result: 'o' })).toBe('?result=o');
    expect(historyQuery({ ...NO_FILTERS, result: 'draw' })).toBe('?result=draw');
    expect(historyQuery({ ...NO_FILTERS, result: 'unfinished' })).toBe('?result=unfinished');
    expect(historyQuery({ ...NO_FILTERS, result: 'fault' })).toBe('?result=fault');
    expect(historyQuery({ ...NO_FILTERS, source: 'studio' })).toBe('?source=studio');
    expect(historyQuery({ ...NO_FILTERS, source: 'arena' })).toBe('?source=arena');
    expect(historyQuery({ ...NO_FILTERS, from: '2026-10-01' })).toBe('?from=2026-10-01');
    expect(historyQuery({ ...NO_FILTERS, to: '2026-10-10' })).toBe('?to=2026-10-10');
  });

  it('joins several filters and encodes the values', () => {
    const query = historyQuery({
      release: 'my bot&co',
      result: 'fault',
      source: 'arena',
      from: '2026-10-01',
      to: '2026-10-10',
    });
    const params = new URLSearchParams(query);
    expect(query.startsWith('?')).toBe(true);
    expect(params.get('release')).toBe('my bot&co');
    expect(params.get('result')).toBe('fault');
    expect(params.get('source')).toBe('arena');
    expect(params.get('from')).toBe('2026-10-01');
    expect(params.get('to')).toBe('2026-10-10');
    expect(query).not.toContain('&co');
    expect(filtersActive({ ...NO_FILTERS, from: '2026-10-01' })).toBe(true);
  });
});

describe('table cells', () => {
  it('writes the result', () => {
    expect(resultSymbol(entry({ winner: 0 }))).toBe('1–0');
    expect(resultSymbol(entry({ winner: 1 }))).toBe('0–1');
    expect(resultSymbol(entry({ winner: null }))).toBe('½–½');
    expect(resultSymbol(entry({ winner: null, end: 'aborted' }))).toBe('—');
    expect(resultSymbol(entry({ winner: 1, end: 'timeout' }))).toBe('0–1');
  });

  it('tags the faults only', () => {
    expect(faultTag(entry({ end: 'timeout' }))).toBe('timeout');
    expect(faultTag(entry({ end: 'crash' }))).toBe('crash');
    expect(faultTag(entry({ end: 'invalid' }))).toBe('invalid');
    expect(faultTag(entry({ end: 'finished' }))).toBeNull();
    expect(faultTag(entry({ end: 'resigned' }))).toBeNull();
    expect(faultTag(entry({ end: 'aborted' }))).toBeNull();
  });

  it('shortens the source', () => {
    expect(sourceTag('studio')).toBe('studio');
    expect(sourceTag('arena match')).toBe('arena');
    expect(sourceTag('arena sprt')).toBe('arena');
    expect(sourceTag('something else')).toBe('something else');
  });

  it('says why the table is empty', () => {
    expect(emptyText(true)).toBe('No saved game matches these filters.');
    expect(emptyText(false)).toContain('No saved games yet');
  });
});

describe('releaseOptions', () => {
  it('lists the releases of all games once', () => {
    expect(
      releaseOptions([
        { id: 'uttt', name: 'U', releases: ['uttt-v010', 'uttt-v009'] },
        { id: 'other', name: 'O', releases: ['uttt-v009', 'other-v001'] },
      ]),
    ).toEqual(['uttt-v010', 'uttt-v009', 'other-v001']);
    expect(releaseOptions(null)).toEqual([]);
  });
});

describe('parseRecordText', () => {
  it('accepts a JSON object', () => {
    expect(parseRecordText('{"format":1}')).toEqual({ record: { format: 1 } });
  });

  it('refuses what is not a JSON object', () => {
    expect(parseRecordText(null)).toEqual({ reason: 'the file cannot be read' });
    expect(parseRecordText('PK\u0003\u0004 zipped')).toEqual({ reason: 'not a JSON file' });
    expect(parseRecordText('')).toEqual({ reason: 'not a JSON file' });
    for (const text of ['[1]', '3', 'null', '"x"']) {
      expect(parseRecordText(text)).toEqual({
        reason: 'not a game record (the JSON is not an object)',
      });
    }
  });
});

describe('importFiles', () => {
  it('counts what was imported, what was there already, and what was refused', async () => {
    const saved: unknown[] = [];
    const summary = await importFiles(
      [
        { name: 'new.json', text: '{"seed":1}' },
        { name: 'again.json', text: '{"seed":2}' },
        { name: 'bad.json', text: '{"seed":3}' },
        { name: 'zip.json', text: 'not json' },
      ],
      async (record) => {
        saved.push(record);
        const seed = (record as { seed: number }).seed;
        if (seed === 3)
          throw new ApiError('the record does not replay: turn 5 is not legal', false);
        return { id: `id-${seed}`, duplicate: seed === 2 };
      },
    );
    expect(summary).toEqual({
      imported: 1,
      duplicates: 1,
      refused: [
        { name: 'bad.json', reason: 'the record does not replay: turn 5 is not legal' },
        { name: 'zip.json', reason: 'not a JSON file' },
      ],
    });
    // A file that is not JSON never reaches the server.
    expect(saved).toEqual([{ seed: 1 }, { seed: 2 }, { seed: 3 }]);
    expect(summaryText(summary)).toBe('Imported 1, 1 already there, 2 refused');
  });

  it('says it for no file at all, and for every file refused', async () => {
    expect(summaryText(await importFiles([], async () => ({ id: 'x', duplicate: false })))).toBe(
      'Imported 0, 0 already there, 0 refused',
    );
    const summary = await importFiles([{ name: 'a', text: '{' }], async () => {
      throw new Error('unreachable');
    });
    expect(summaryText(summary)).toBe('Imported 0, 0 already there, 1 refused');
  });

  it('reads the reason of a failure that is not an Error', async () => {
    const summary = await importFiles([{ name: 'a', text: '{}' }], () => Promise.reject('no'));
    expect(summary.refused).toEqual([{ name: 'a', reason: 'no' }]);
  });

  it('saves the files one after the other', async () => {
    const order: string[] = [];
    await importFiles(
      [
        { name: 'a', text: '{"n":1}' },
        { name: 'b', text: '{"n":2}' },
      ],
      async (record) => {
        const n = (record as { n: number }).n;
        order.push(`start ${n}`);
        await Promise.resolve();
        order.push(`end ${n}`);
        return { id: String(n), duplicate: false };
      },
    );
    expect(order).toEqual(['start 1', 'end 1', 'start 2', 'end 2']);
  });
});
