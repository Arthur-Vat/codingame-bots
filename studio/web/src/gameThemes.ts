/* A game's chosen themes, kept in localStorage under `studio.theme.<gameId>.pieces` and
   `studio.theme.<gameId>.board`. A stored id is only ever looked up in the game's list of
   options: anything unknown (or a storage that fails) gives the default. */

import { useSyncExternalStore } from 'react';
import type { SettingsStorage } from './analysisSettings';
import { themesFor, type ThemeChoice } from './games/themeChoices';

export type ThemeKind = 'pieces' | 'board';

const THEME_PREFIX = 'studio.theme.';

export const themeKey = (gameId: string, kind: ThemeKind): string =>
  `${THEME_PREFIX}${gameId}.${kind}`;

function browserStorage(): SettingsStorage | null {
  try {
    return localStorage;
  } catch {
    return null;
  }
}

function choiceOf(gameId: string, kind: ThemeKind): ThemeChoice | undefined {
  return themesFor(gameId)?.[kind];
}

/** The id of `value` among the choice's options, else the choice's default. */
function validId(choice: ThemeChoice, value: string | null): string {
  return choice.options.find((o) => o.id === value)?.id ?? choice.default;
}

function stored(key: string, storage: SettingsStorage | null): string | null {
  try {
    return storage?.getItem(key) ?? null;
  } catch {
    return null;
  }
}

/** The theme chosen for a game, or its default; '' for a game that offers no such choice. */
export function readTheme(
  gameId: string,
  kind: ThemeKind,
  storage: SettingsStorage | null = browserStorage(),
): string {
  const choice = choiceOf(gameId, kind);
  return choice === undefined ? '' : validId(choice, stored(themeKey(gameId, kind), storage));
}

/** Remembers a choice; an id the game does not offer is refused (false). */
export function writeTheme(
  gameId: string,
  kind: ThemeKind,
  id: string,
  storage: SettingsStorage | null = browserStorage(),
): boolean {
  const choice = choiceOf(gameId, kind);
  if (choice === undefined || !choice.options.some((o) => o.id === id)) return false;
  try {
    storage?.setItem(themeKey(gameId, kind), id);
  } catch {
    // Not remembered; the choice holds until the page is left.
  }
  return true;
}

/* The choice made on this page is kept in memory too, so that a storage that refuses writes
   still changes the boards. Listeners are the boards on the page: all hear of each change. */
const chosen = new Map<string, string>();
const listeners = new Set<() => void>();

/** Hears of every change of a theme; exported for the tests. */
export function subscribe(listener: () => void): () => void {
  listeners.add(listener);
  // Another tab changed a setting: what is stored counts again.
  const onStorage = (event: StorageEvent) => {
    // A setting of another kind (or another page's key) leaves the choices alone.
    if (event.key !== null && !event.key.startsWith(THEME_PREFIX)) return;
    chosen.clear();
    listener();
  };
  window.addEventListener('storage', onStorage);
  return () => {
    listeners.delete(listener);
    window.removeEventListener('storage', onStorage);
  };
}

function current(gameId: string, kind: ThemeKind): string {
  const choice = choiceOf(gameId, kind);
  if (choice === undefined) return '';
  const own = chosen.get(themeKey(gameId, kind));
  return own === undefined ? readTheme(gameId, kind) : validId(choice, own);
}

/** Chooses a theme for a game: every board on the page follows at once. */
export function setGameTheme(gameId: string, kind: ThemeKind, id: string): void {
  if (!writeTheme(gameId, kind, id)) return;
  chosen.set(themeKey(gameId, kind), id);
  for (const listener of [...listeners]) listener();
}

export interface GameTheme {
  pieces: string;
  board: string;
}

/** The themes chosen for a game ('' for a kind it does not offer). */
export function useGameTheme(gameId: string): GameTheme {
  const pieces = useSyncExternalStore(
    subscribe,
    () => current(gameId, 'pieces'),
    () => current(gameId, 'pieces'),
  );
  const board = useSyncExternalStore(
    subscribe,
    () => current(gameId, 'board'),
    () => current(gameId, 'board'),
  );
  return { pieces, board };
}

/** For tests: forgets the choices made on this page. */
export function forgetChosenThemes(): void {
  chosen.clear();
}
