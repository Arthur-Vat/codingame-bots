import { useEffect, useMemo, useState } from 'react';
import { ApiError, getHistoryView, postView, type RecordView } from './api';
import { recallFile } from './loadedFiles';
import { reviewOfRecord, reviewOfSession, type ReviewModel } from './review';
import type { ReviewKind } from './route';
import { useSession } from './useSession';

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

/** Fetches the view of a saved game, or of a file kept in the page, once. */
async function loadView(kind: 'saved' | 'file', id: string): Promise<ReviewModel> {
  if (kind === 'saved') {
    return reviewOfRecord(await getHistoryView(id), { kind: 'saved', id });
  }
  const file = recallFile(id);
  if (file === null) {
    throw new Error('This file is no longer loaded. Load it again from the history page.');
  }
  const view: RecordView = await postView(file.record);
  return reviewOfRecord(view, { kind: 'file', id, name: file.name });
}

/**
 * The game of a review route as one model. A session is polled (it may still be going on); a
 * saved game and a file are loaded once.
 */
export function useReview(kind: ReviewKind, id: string): ReviewState {
  const { session, error: sessionError } = useSession(kind === 'session' ? id : null);
  const [loaded, setLoaded] = useState<Loaded | null>(null);
  const key = `${kind}/${id}`;

  useEffect(() => {
    if (kind === 'session') return;
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

  const sessionModel = useMemo(
    () => (session === null ? null : reviewOfSession(session)),
    [session],
  );

  if (kind === 'session') {
    return {
      model: sessionModel,
      error: sessionModel === null ? (sessionError?.message ?? null) : null,
    };
  }
  if (loaded === null || loaded.key !== key) return { model: null, error: null };
  return { model: loaded.model, error: loaded.error };
}
