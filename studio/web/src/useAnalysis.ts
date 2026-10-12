import { useEffect, useState } from 'react';
import {
  ApiError,
  getHistoryView,
  getRecord,
  getSession,
  postView,
  postViewText,
  type RecordView,
} from './api';
import { arenaRecordText } from './arena';
import { recallFile } from './loadedFiles';
import { analysisOfRecord, type AnalysisModel } from './analysis';
import type { AnalysisKind } from './route';

export interface AnalysisState {
  /** Null while loading, or when it failed. */
  model: AnalysisModel | null;
  /** Why the game cannot be shown. */
  error: string | null;
  /** The game of a session is still being played: it is not shown. */
  playing: boolean;
}

interface Loaded {
  key: string;
  model: AnalysisModel | null;
  error: string | null;
  playing: boolean;
}

function failure(error: unknown): string {
  return error instanceof ApiError ? error.message : String(error);
}

/**
 * Fetches the view of a saved game, of an arena game, of a file kept in the page or of a game
 * played here, once; null for a game still being played. An arena game's record is fetched from the
 * published site and checked by the rules (`POST /api/view`), sent as the text it was published in.
 * A game played here is shown once it is over, never while it is played (a game of two bots must
 * not be seen move by move).
 */
async function loadView(kind: AnalysisKind, id: string): Promise<AnalysisModel | null> {
  if (kind === 'session') {
    const session = await getSession(id);
    if (session.status === 'failed') throw new Error(session.error ?? 'The game failed.');
    if (session.result === null) return null;
    return analysisOfRecord(await postView(await getRecord(id)), { kind: 'session', id });
  }
  if (kind === 'saved') {
    return analysisOfRecord(await getHistoryView(id), { kind: 'saved', id });
  }
  if (kind === 'arena') {
    return analysisOfRecord(await postViewText(await arenaRecordText(id)), { kind: 'arena', id });
  }
  const file = recallFile(id);
  if (file === null) {
    throw new Error('This file is no longer loaded. Load it again from the history page.');
  }
  const view: RecordView = await postView(file.record);
  return analysisOfRecord(view, { kind: 'file', id, name: file.name });
}

/**
 * The game of an analysis board route as one model: a saved game, an arena game, a file or a game
 * played here, loaded once.
 */
export function useAnalysis(kind: AnalysisKind, id: string): AnalysisState {
  const [loaded, setLoaded] = useState<Loaded | null>(null);
  const key = `${kind}/${id}`;

  useEffect(() => {
    let live = true;
    loadView(kind, id).then(
      (model) => {
        if (live) setLoaded({ key, model, error: null, playing: model === null });
      },
      (error: unknown) => {
        if (live) setLoaded({ key, model: null, error: failure(error), playing: false });
      },
    );
    return () => {
      live = false;
    };
  }, [kind, id, key]);

  if (loaded === null || loaded.key !== key) return { model: null, error: null, playing: false };
  return { model: loaded.model, error: loaded.error, playing: loaded.playing };
}
