/* The saved games of the browser engine: the IndexedDB database `studio`, object store `games`,
   keyed by id. A value is `{id, record_text, row}`: the record exactly as text (it is never parsed
   to be written again) and the line that lists it in the history. A small promise wrapper over
   IndexedDB, no package. */

import type { HistoryRow } from './rules';

export interface StoredGame {
  id: string;
  record_text: string;
  row: HistoryRow;
}

export interface GameStore {
  get: (id: string) => Promise<StoredGame | null>;
  put: (game: StoredGame) => Promise<void>;
  /** Whether a game was there. */
  delete: (id: string) => Promise<boolean>;
  all: () => Promise<StoredGame[]>;
}

export const DATABASE = 'studio';
export const STORE = 'games';

/** The message of the routes that need the store when the browser has none. */
export const STORE_UNAVAILABLE =
  'Saved games are unavailable: this browser does not let this page store data (IndexedDB)';

function settled<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error('IndexedDB failed'));
  });
}

function finished(transaction: IDBTransaction): Promise<void> {
  return new Promise((resolve, reject) => {
    transaction.oncomplete = () => resolve();
    transaction.onerror = () => reject(transaction.error ?? new Error('IndexedDB failed'));
    transaction.onabort = () => reject(transaction.error ?? new Error('IndexedDB was aborted'));
  });
}

/** Opens the database. Rejects where IndexedDB is missing or blocked. */
export function openGameStore(
  factory: IDBFactory | undefined = globalThis.indexedDB,
): Promise<GameStore> {
  if (factory === undefined) return Promise.reject(new Error(STORE_UNAVAILABLE));
  return new Promise<IDBDatabase>((resolve, reject) => {
    let request: IDBOpenDBRequest;
    try {
      request = factory.open(DATABASE, 1);
    } catch (error) {
      reject(error instanceof Error ? error : new Error(STORE_UNAVAILABLE));
      return;
    }
    request.onupgradeneeded = () => {
      request.result.createObjectStore(STORE, { keyPath: 'id' });
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error(STORE_UNAVAILABLE));
    request.onblocked = () => reject(new Error(STORE_UNAVAILABLE));
  }).then((database) => {
    // Another tab upgrading the database asks this one to let go.
    database.onversionchange = () => database.close();
    const objects = (mode: IDBTransactionMode) => {
      const transaction = database.transaction(STORE, mode);
      return { transaction, store: transaction.objectStore(STORE) };
    };
    return {
      async get(id) {
        const { store } = objects('readonly');
        const found = (await settled(store.get(id))) as StoredGame | undefined;
        return found ?? null;
      },
      async put(game) {
        const { transaction, store } = objects('readwrite');
        store.put(game);
        await finished(transaction);
      },
      async delete(id) {
        const { transaction, store } = objects('readwrite');
        const there = (await settled(store.count(id))) > 0;
        store.delete(id);
        await finished(transaction);
        return there;
      },
      async all() {
        const { store } = objects('readonly');
        return (await settled(store.getAll())) as StoredGame[];
      },
    };
  });
}

/** A store in memory, for tests. */
export function memoryGameStore(): GameStore {
  const games = new Map<string, StoredGame>();
  return {
    get: async (id) => games.get(id) ?? null,
    put: async (game) => void games.set(game.id, game),
    delete: async (id) => games.delete(id),
    all: async () => [...games.values()],
  };
}
