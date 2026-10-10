import { useEffect, useState, type ChangeEvent } from 'react';
import {
  ApiError,
  deleteHistory,
  getHistoryRecord,
  listHistory,
  postHistory,
  type HistoryEntry,
} from './api';
import { download, errorMessage } from './browser';
import {
  emptyText,
  faultTag,
  filtersActive,
  historyQuery,
  importFiles,
  NO_FILTERS,
  parseRecordText,
  releaseOptions,
  RESULT_OPTIONS,
  resultSymbol,
  SOURCE_OPTIONS,
  sourceTag,
  summaryText,
  type HistoryFilters,
  type ImportRefusal,
  type ResultFilter,
  type SourceFilter,
} from './history';
import type { GamesState } from './hooks';
import { rememberFile } from './loadedFiles';
import { formatDate } from './review';
import { routeHash } from './route';
import { Toast, useToast } from './Toast';

interface Listing {
  entries: HistoryEntry[] | null;
  error: string | null;
}

/** The saved games for a query; asked again when `refresh` changes. */
function useHistoryList(query: string, refresh: number): Listing | null {
  const [listing, setListing] = useState<Listing | null>(null);
  useEffect(() => {
    let live = true;
    listHistory(query).then(
      (entries) => {
        if (live) setListing({ entries, error: null });
      },
      (error: unknown) => {
        if (live) {
          const text = error instanceof ApiError ? error.message : errorMessage(error);
          setListing({ entries: null, error: text });
        }
      },
    );
    return () => {
      live = false;
    };
  }, [query, refresh]);
  // The list of the last query stays on screen until the new one arrives.
  return listing;
}

/** Reads the text of a file, or null when the browser cannot. */
async function readText(file: File): Promise<string | null> {
  try {
    return await file.text();
  } catch {
    return null;
  }
}

function Row({
  entry,
  confirming,
  onAsk,
  onCancel,
  onDelete,
  onExport,
}: {
  entry: HistoryEntry;
  confirming: boolean;
  onAsk: () => void;
  onCancel: () => void;
  onDelete: () => void;
  onExport: () => void;
}) {
  const fault = faultTag(entry);
  const symbol = resultSymbol(entry);
  return (
    <tr data-testid="history-row">
      <td className="num">{formatDate(entry.unix_time)}</td>
      <td>{entry.players[0]}</td>
      <td>{entry.players[1]}</td>
      <td>
        <span className={`res ${entry.winner === null ? 'd' : 'x'}`}>{symbol}</span>
        {fault !== null && <span className="tag keep">{fault}</span>}
      </td>
      <td className="num">{entry.turns}</td>
      <td className="src">
        <span className="tag">{sourceTag(entry.source)}</span>
      </td>
      <td>
        <div className="row-actions">
          <a className="ctrl" href={routeHash({ page: 'review', kind: 'saved', id: entry.id })}>
            Review
          </a>
          <button className="ctrl" type="button" onClick={onExport}>
            Export
          </button>
          {confirming ? (
            <>
              <button className="ctrl danger" type="button" onClick={onDelete}>
                Delete for good
              </button>
              <button className="ctrl" type="button" onClick={onCancel}>
                Cancel
              </button>
            </>
          ) : (
            <button className="ctrl" type="button" onClick={onAsk}>
              Delete
            </button>
          )}
        </div>
      </td>
    </tr>
  );
}

export function HistoryPage({ gamesState }: { gamesState: GamesState }) {
  const [filters, setFilters] = useState<HistoryFilters>(NO_FILTERS);
  const [refresh, setRefresh] = useState(0);
  const [confirming, setConfirming] = useState<string | null>(null);
  const [refused, setRefused] = useState<ImportRefusal[]>([]);
  const [busy, setBusy] = useState(false);
  const toast = useToast();

  const query = historyQuery(filters);
  const listing = useHistoryList(query, refresh);
  const releases = releaseOptions(gamesState.games);
  const change = (patch: Partial<HistoryFilters>) => {
    setFilters((previous) => ({ ...previous, ...patch }));
    setConfirming(null);
  };

  const exportGame = async (id: string) => {
    try {
      download(`${id}.json`, await getHistoryRecord(id));
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const deleteGame = async (id: string) => {
    try {
      await deleteHistory(id);
      setConfirming(null);
      setRefresh((n) => n + 1);
      toast.show('Deleted');
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };

  const loadFiles = async (event: ChangeEvent<HTMLInputElement>) => {
    const input = event.target;
    const files = Array.from(input.files ?? []);
    input.value = '';
    if (files.length === 0 || busy) return;
    setBusy(true);
    try {
      const inputs = await Promise.all(
        files.map(async (file) => ({ name: file.name, text: await readText(file) })),
      );
      const summary = await importFiles(inputs, postHistory);
      setRefused(summary.refused);
      toast.show(summaryText(summary));
      setRefresh((n) => n + 1);
    } finally {
      setBusy(false);
    }
  };

  /** Opens a file for review without adding it to the history. */
  const reviewFile = async (event: ChangeEvent<HTMLInputElement>) => {
    const input = event.target;
    const file = input.files?.[0];
    input.value = '';
    if (file === undefined) return;
    const parsed = parseRecordText(await readText(file));
    if ('reason' in parsed) {
      setRefused([{ name: file.name, reason: parsed.reason }]);
      return;
    }
    const { record } = parsed;
    setRefused([]);
    const id = rememberFile({ name: file.name, record });
    window.location.hash = routeHash({ page: 'review', kind: 'file', id });
  };

  const entries = listing?.entries ?? null;
  return (
    <main className="wrap">
      <div className="hist-head">
        <h1>Game history</h1>
        <div className="hist-actions">
          <label className="btn filebtn" aria-disabled={busy}>
            <input
              type="file"
              multiple
              accept=".json,application/json"
              className="visually-hidden"
              disabled={busy}
              onChange={(event) => void loadFiles(event)}
            />
            <span>Load game files</span>
          </label>
          <label className="btn filebtn">
            <input
              type="file"
              accept=".json,application/json"
              className="visually-hidden"
              onChange={(event) => void reviewFile(event)}
            />
            <span>Review a file</span>
          </label>
        </div>
      </div>
      <p className="hint hist-hint">
        Loading adds games to your history. Workflow artifacts come zipped: unzip them first.
      </p>

      <section className="panel filters" aria-label="Filters">
        <div className="field">
          <label htmlFor="f-release">Release</label>
          <select
            id="f-release"
            value={filters.release}
            onChange={(event) => change({ release: event.target.value })}
          >
            <option value="">All releases</option>
            {releases.map((name) => (
              <option key={name} value={name}>
                {name}
              </option>
            ))}
          </select>
        </div>
        <div className="field">
          <label htmlFor="f-result">Result</label>
          <select
            id="f-result"
            value={filters.result}
            onChange={(event) => change({ result: event.target.value as ResultFilter })}
          >
            {RESULT_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        <div className="field">
          <label htmlFor="f-source">Source</label>
          <select
            id="f-source"
            value={filters.source}
            onChange={(event) => change({ source: event.target.value as SourceFilter })}
          >
            {SOURCE_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        <div className="field">
          <label htmlFor="f-from">From</label>
          <input
            id="f-from"
            type="date"
            value={filters.from}
            onChange={(event) => change({ from: event.target.value })}
          />
        </div>
        <div className="field">
          <label htmlFor="f-to">To</label>
          <input
            id="f-to"
            type="date"
            value={filters.to}
            onChange={(event) => change({ to: event.target.value })}
          />
        </div>
      </section>

      {refused.length > 0 && (
        <section className="panel refused" role="alert" aria-label="Refused files">
          <b>Not loaded</b>
          <ul>
            {refused.map((refusal, index) => (
              <li key={`${refusal.name}-${index}`}>
                <code>{refusal.name}</code>: {refusal.reason}
              </li>
            ))}
          </ul>
        </section>
      )}

      <section className="panel">
        {listing?.error != null && (
          <div className="empty-page" role="alert">
            {listing.error}
          </div>
        )}
        {entries !== null && entries.length === 0 && (
          <div className="empty-page" data-testid="history-empty">
            {emptyText(filtersActive(filters))}
          </div>
        )}
        {entries !== null && entries.length > 0 && (
          <div className="tbl-wrap">
            <table>
              <thead>
                <tr>
                  <th>Date</th>
                  <th>X</th>
                  <th>O</th>
                  <th>Result</th>
                  <th>Moves</th>
                  <th>Source</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                {entries.map((entry) => (
                  <Row
                    key={entry.id}
                    entry={entry}
                    confirming={confirming === entry.id}
                    onAsk={() => setConfirming(entry.id)}
                    onCancel={() => setConfirming(null)}
                    onDelete={() => void deleteGame(entry.id)}
                    onExport={() => void exportGame(entry.id)}
                  />
                ))}
              </tbody>
            </table>
          </div>
        )}
        {listing === null && <div className="empty-page">Loading…</div>}
      </section>
      <Toast text={toast.text} />
    </main>
  );
}
