import { useEffect, useRef, useState, type ChangeEvent } from 'react';
import {
  ApiError,
  deleteHistory,
  getHistoryRecordText,
  listHistory,
  postHistoryText,
  type HistoryEntry,
} from './api';
import { arenaRecordText, listArena } from './arena';
import { downloadText, errorMessage } from './browser';
import {
  ARENA_NOTE,
  arenaEmptyText,
  droppedText,
  emptyText,
  faultTag,
  filtersActive,
  historyQuery,
  importFiles,
  NO_FILTERS,
  parseRecordText,
  releaseOptions,
  resultOptionsFor,
  resultSymbol,
  SOURCE_OPTIONS,
  sourceTag,
  summaryText,
  type HistoryFilters,
  type ImportRefusal,
  type ResultFilter,
  type SourceFilter,
} from './history';
import { seatNamesFor } from './games/registry';
import type { GamesState } from './hooks';
import { rememberFile } from './loadedFiles';
import { formatDate } from './analysis';
import { routeHash, type HistorySet } from './route';
import { Toast, useToast } from './Toast';

interface Listing {
  entries: HistoryEntry[] | null;
  error: string | null;
  /** Rows of the published index that are left out (the arena games). */
  dropped: number;
}

/**
 * The saved games, or the arena games, for a query; asked again when `refresh` changes. An answer
 * that comes after the query or the set has changed is ignored.
 */
function useHistoryList(set: HistorySet, query: string, refresh: number): Listing | null {
  const [listing, setListing] = useState<Listing | null>(null);
  useEffect(() => {
    let live = true;
    const asked =
      set === 'arena'
        ? listArena(query).then(({ entries, dropped }) => ({ entries, dropped }))
        : listHistory(query).then((entries) => ({ entries, dropped: 0 }));
    asked.then(
      ({ entries, dropped }) => {
        if (live) setListing({ entries, error: null, dropped });
      },
      (error: unknown) => {
        if (live) {
          const text = error instanceof ApiError ? error.message : errorMessage(error);
          setListing({ entries: null, error: text, dropped: 0 });
        }
      },
    );
    return () => {
      live = false;
    };
  }, [set, query, refresh]);
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
  gameId,
  set,
  entry,
  confirming,
  deleting,
  onAsk,
  onCancel,
  onDelete,
  onExport,
}: {
  gameId: string;
  set: HistorySet;
  entry: HistoryEntry;
  confirming: boolean;
  deleting: boolean;
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
          <a
            className="ctrl"
            href={routeHash({
              page: 'game',
              gameId,
              tab: 'analysis',
              board: { kind: set === 'arena' ? 'arena' : 'saved', id: entry.id },
            })}
          >
            Analysis board
          </a>
          <button className="ctrl" type="button" onClick={onExport}>
            Export
          </button>
          {set === 'arena' ? null : confirming ? (
            <>
              <button className="ctrl danger" type="button" disabled={deleting} onClick={onDelete}>
                Delete for good
              </button>
              <button className="ctrl" type="button" disabled={deleting} onClick={onCancel}>
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

/** The switch between the visitor's own games and the arena games published with the site. */
function SetSwitch({ gameId, set }: { gameId: string; set: HistorySet }) {
  const link = (target: HistorySet, text: string) => (
    <a
      href={routeHash({ page: 'game', gameId, tab: 'history', set: target })}
      aria-current={set === target ? 'page' : undefined}
    >
      {text}
    </a>
  );
  return (
    <nav className="hist-sets" aria-label="Games shown">
      {link('mine', 'My games')}
      {link('arena', 'Arena games')}
    </nav>
  );
}

/** The History tab of a game: its saved games, or the arena games, with their filters. */
export function HistoryPage({
  gameId,
  set,
  gamesState,
}: {
  gameId: string;
  set: HistorySet;
  gamesState: GamesState;
}) {
  const [filters, setFilters] = useState<HistoryFilters>(NO_FILTERS);
  const [refresh, setRefresh] = useState(0);
  const [confirming, setConfirming] = useState<string | null>(null);
  const [refused, setRefused] = useState<ImportRefusal[]>([]);
  const [busy, setBusy] = useState(false);
  const deleting = useRef(false);
  const [deletingNow, setDeletingNow] = useState(false);
  const toast = useToast();

  const query = historyQuery(filters, gameId);
  const listing = useHistoryList(set, query, refresh);
  const arena = set === 'arena';
  const releases = releaseOptions(gamesState.games, gameId);
  const names = seatNamesFor(gameId);
  const change = (patch: Partial<HistoryFilters>) => {
    setFilters((previous) => ({ ...previous, ...patch }));
    setConfirming(null);
    setRefused([]);
  };

  const exportGame = async (id: string) => {
    try {
      downloadText(`${id}.json`, await (arena ? arenaRecordText(id) : getHistoryRecordText(id)));
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const deleteGame = async (id: string) => {
    // One request at a time: a second click while the first is in flight does nothing.
    if (deleting.current) return;
    deleting.current = true;
    setDeletingNow(true);
    try {
      await deleteHistory(id);
      setConfirming(null);
      setRefresh((n) => n + 1);
      toast.show('Deleted');
    } catch (failure) {
      toast.show(errorMessage(failure));
    } finally {
      deleting.current = false;
      setDeletingNow(false);
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
      const summary = await importFiles(inputs, postHistoryText);
      setRefused(summary.refused);
      toast.show(summaryText(summary));
      setRefresh((n) => n + 1);
    } finally {
      setBusy(false);
    }
  };

  /** Opens a file on the analysis board without adding it to the history. */
  const openFile = async (event: ChangeEvent<HTMLInputElement>) => {
    const input = event.target;
    const file = input.files?.[0];
    input.value = '';
    if (file === undefined) return;
    const parsed = parseRecordText(await readText(file));
    if ('reason' in parsed) {
      setRefused([{ name: file.name, reason: parsed.reason }]);
      return;
    }
    const { record, text } = parsed;
    setRefused([]);
    const id = rememberFile({ name: file.name, text });
    // The record names its game; what it holds is untrusted, so the name counts only when it is
    // the id of a game the app has. Otherwise the file opens on the current game, and the view
    // request says what is wrong.
    const named = (record as { game?: unknown }).game;
    const known =
      typeof named === 'string' && (gamesState.games ?? []).some((game) => game.id === named);
    window.location.hash = routeHash({
      page: 'game',
      gameId: known ? named : gameId,
      tab: 'analysis',
      board: { kind: 'file', id },
    });
  };

  const entries = listing?.entries ?? null;
  return (
    <main className="wrap">
      <div className="hist-head">
        <h1>Game history</h1>
        {!arena && (
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
                onChange={(event) => void openFile(event)}
              />
              <span>Open a file</span>
            </label>
          </div>
        )}
      </div>
      <SetSwitch gameId={gameId} set={set} />
      {arena ? (
        <p className="hint hist-hint">{ARENA_NOTE}</p>
      ) : (
        <p className="hint hist-hint">Workflow artifacts come zipped: unzip them first.</p>
      )}

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
            {resultOptionsFor(names).map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        {!arena && (
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
        )}
        <div className="field">
          <label htmlFor="f-from">From (UTC)</label>
          <input
            id="f-from"
            type="date"
            value={filters.from}
            onChange={(event) => change({ from: event.target.value })}
          />
        </div>
        <div className="field">
          <label htmlFor="f-to">To (UTC)</label>
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
            {arena ? arenaEmptyText(filtersActive(filters)) : emptyText(filtersActive(filters))}
          </div>
        )}
        {entries !== null && entries.length > 0 && (
          <div className="tbl-wrap">
            <table>
              <thead>
                <tr>
                  <th>Date</th>
                  <th>{names[0]}</th>
                  <th>{names[1]}</th>
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
                    gameId={gameId}
                    set={set}
                    entry={entry}
                    confirming={confirming === entry.id}
                    deleting={deletingNow}
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
        {arena && droppedText(listing?.dropped ?? 0) !== null && (
          <p className="hint">{droppedText(listing?.dropped ?? 0)}</p>
        )}
      </section>
      <Toast text={toast.text} />
    </main>
  );
}
