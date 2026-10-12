/* What the history page needs, as pure functions: the filters and their query, how a saved game
   is written in the table, and loading files into the history. */

import type { GameSummary, HistoryEntry } from './api';

/** The result filter's values are the server's own, or '' for any. */
export type ResultFilter = '' | 'x' | 'o' | 'draw' | 'unfinished' | 'fault';
export type SourceFilter = '' | 'studio' | 'arena';

/** The options of the result select, with the text shown. */
export const RESULT_OPTIONS: readonly { value: ResultFilter; label: string }[] = [
  { value: '', label: 'Any result' },
  { value: 'x', label: 'X wins' },
  { value: 'o', label: 'O wins' },
  { value: 'draw', label: 'Draw' },
  { value: 'unfinished', label: 'Unfinished' },
  { value: 'fault', label: 'Faults' },
];

export const SOURCE_OPTIONS: readonly { value: SourceFilter; label: string }[] = [
  { value: '', label: 'All games' },
  { value: 'studio', label: 'Played in the studio' },
  { value: 'arena', label: 'Arena samples' },
];

export interface HistoryFilters {
  /** A release name, or '' for all. */
  release: string;
  result: ResultFilter;
  source: SourceFilter;
  /** `YYYY-MM-DD`, or ''. */
  from: string;
  to: string;
}

export const NO_FILTERS: HistoryFilters = {
  release: '',
  result: '',
  source: '',
  from: '',
  to: '',
};

/** The filters as query parameters, the game's aside. */
function filterParams(filters: HistoryFilters): URLSearchParams {
  const params = new URLSearchParams();
  if (filters.release !== '') params.set('release', filters.release);
  if (filters.result !== '') params.set('result', filters.result);
  if (filters.source !== '') params.set('source', filters.source);
  if (filters.from !== '') params.set('from', filters.from);
  if (filters.to !== '') params.set('to', filters.to);
  return params;
}

/**
 * The query of `GET /api/history` (and of `POST /api/filter`) for the filters of one game's
 * history: always `game=<id>` first, then the filters that are set. Starts with `?`.
 */
export function historyQuery(filters: HistoryFilters, game: string): string {
  const params = new URLSearchParams({ game });
  for (const [key, value] of filterParams(filters)) params.set(key, value);
  return `?${params.toString()}`;
}

/** Whether any filter is set (the game is not a filter the visitor chose). */
export function filtersActive(filters: HistoryFilters): boolean {
  return filterParams(filters).size > 0;
}

/** The release names of one game, each once, in the order the server gave them. */
export function releaseOptions(games: readonly GameSummary[] | null, gameId: string): string[] {
  const names: string[] = [];
  for (const game of games ?? []) {
    if (game.id !== gameId) continue;
    for (const release of game.releases) {
      if (!names.includes(release)) names.push(release);
    }
  }
  return names;
}

/** The result as the table writes it: 1–0, 0–1, ½–½, or — for a game that was not finished. */
export function resultSymbol(entry: Pick<HistoryEntry, 'winner' | 'end'>): string {
  if (entry.end === 'aborted') return '—';
  if (entry.winner === 0) return '1–0';
  if (entry.winner === 1) return '0–1';
  return '½–½';
}

/** The tag of a game that ended by a fault of a bot, or null. */
export function faultTag(entry: Pick<HistoryEntry, 'end'>): string | null {
  return entry.end === 'timeout' || entry.end === 'crash' || entry.end === 'invalid'
    ? entry.end
    : null;
}

/** The source as a short tag: `studio`, `arena`, or the text itself. */
export function sourceTag(source: string): string {
  return source.startsWith('arena') ? 'arena' : source;
}

/** What the table says when it has no row. */
export function emptyText(filtered: boolean): string {
  return filtered
    ? 'No saved game matches these filters.'
    : 'No saved games yet. Save one from the analysis board, or load game files.';
}

/** What the table says in the arena games when it has no row. */
export function arenaEmptyText(filtered: boolean): string {
  return filtered ? emptyText(true) : 'No arena games published yet.';
}

/** The note under the switch of the arena games. */
export const ARENA_NOTE =
  'Games played by the releases on GitHub (SPRT and league samples). Read-only: save one to add it to your games.';

/** What is said of the rows of the published index that were left out, or null. */
export function droppedText(dropped: number): string | null {
  if (dropped <= 0) return null;
  return `${dropped} ${dropped === 1 ? 'entry' : 'entries'} of the published index ${dropped === 1 ? 'is' : 'are'} not valid and not listed.`;
}

/** A file to load: its name and its text, or null when it could not be read. */
export interface ImportInput {
  name: string;
  text: string | null;
}

export interface ImportRefusal {
  name: string;
  reason: string;
}

export interface ImportSummary {
  imported: number;
  duplicates: number;
  refused: ImportRefusal[];
}

/**
 * The JSON of a file's text, which must be an object; the server checks that it is a game record.
 * `reason` says why a file is refused here.
 */
export function parseRecordText(text: string | null): { record: object } | { reason: string } {
  if (text === null) return { reason: 'the file cannot be read' };
  let record: unknown;
  try {
    record = JSON.parse(text);
  } catch {
    return { reason: 'not a JSON file' };
  }
  if (typeof record !== 'object' || record === null || Array.isArray(record)) {
    return { reason: 'not a game record (the JSON is not an object)' };
  }
  return { record };
}

/**
 * Loads files into the history, one after the other. A file that is not JSON, or not an object,
 * is refused here; the rest goes to `save`, and a failure of `save` is the file's refusal, with
 * the server's reason.
 */
export async function importFiles(
  inputs: readonly ImportInput[],
  save: (record: unknown) => Promise<{ id: string; duplicate: boolean }>,
): Promise<ImportSummary> {
  const summary: ImportSummary = { imported: 0, duplicates: 0, refused: [] };
  for (const { name, text } of inputs) {
    const parsed = parseRecordText(text);
    if ('reason' in parsed) {
      summary.refused.push({ name, reason: parsed.reason });
      continue;
    }
    const { record } = parsed;
    try {
      const { duplicate } = await save(record);
      if (duplicate) summary.duplicates += 1;
      else summary.imported += 1;
    } catch (failure) {
      summary.refused.push({
        name,
        reason: failure instanceof Error ? failure.message : String(failure),
      });
    }
  }
  return summary;
}

/** `Imported 1, 1 already there, 1 refused`. */
export function summaryText(summary: ImportSummary): string {
  return `Imported ${summary.imported}, ${summary.duplicates} already there, ${summary.refused.length} refused`;
}
