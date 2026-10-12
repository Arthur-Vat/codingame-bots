import { useEffect, useState } from 'react';
import { ApiError, getHistoryView, postView, postViewText, type RecordView } from './api';
import { arenaRecordText } from './arena';
import { recallFile } from './loadedFiles';
import { reviewOfRecord, type ReviewModel } from './review';
import type { ReviewKind } from './route';

export interface ReviewState {
  /** Null while loading, or when it failed. */
  model: ReviewModel | null;
  /** Why the game cannot be shown. */
  error: string | null;
}

interface Loaded {
  key: string;
  model: ReviewModel | null;
  error: string | null;
}

function failure(error: unknown): string {
  return error instanceof ApiError ? error.message : String(error);
}

/**
 * Fetches the view of a saved game, of an arena game or of a file kept in the page, once. An arena
 * game's record is fetched from the published site and checked by the rules (`POST /api/view`),
 * sent as the text it was published in.
 */
async function loadView(kind: 'saved' | 'file' | 'arena', id: string): Promise<ReviewModel> {
  if (kind === 'saved') {
    return reviewOfRecord(await getHistoryView(id), { kind: 'saved', id });
  }
  if (kind === 'arena') {
    return reviewOfRecord(await postViewText(await arenaRecordText(id)), { kind: 'arena', id });
  }
  const file = recallFile(id);
  if (file === null) {
    throw new Error('This file is no longer loaded. Load it again from the history page.');
  }
  const view: RecordView = await postView(file.record);
  return reviewOfRecord(view, { kind: 'file', id, name: file.name });
}

/**
 * The game of a review route as one model: a saved game, an arena game or a file, loaded once.
 */
export function useReview(kind: ReviewKind, id: string): ReviewState {
  const [loaded, setLoaded] = useState<Loaded | null>(null);
  const key = `${kind}/${id}`;

  useEffect(() => {
    let live = true;
    loadView(kind, id).then(
      (model) => {
        if (live) setLoaded({ key, model, error: null });
      },
      (error: unknown) => {
        if (live) setLoaded({ key, model: null, error: failure(error) });
      },
    );
    return () => {
      live = false;
    };
  }, [kind, id, key]);

  if (loaded === null || loaded.key !== key) return { model: null, error: null };
  return { model: loaded.model, error: loaded.error };
}
