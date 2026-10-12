/* Where each tab of a game's page leads: the last thing seen in that tab, kept in memory (not in
   storage; a reload forgets it). Pure functions; the app holds the state. */

import { routeHash, type GameTab, type HistorySet, type ReviewKind, type Route } from './route';

/** What one game's tabs remember. */
export interface TabMemory {
  /** The session last seen in the Play tab. */
  session: string | null;
  /** The game last opened in the Review tab. */
  review: { kind: ReviewKind; id: string } | null;
  /** The set last shown in the History tab. */
  set: HistorySet;
}

/** The memory of every game, by its id. */
export type GameMemory = Readonly<Record<string, TabMemory>>;

export const NO_MEMORY: GameMemory = {};

const FRESH: TabMemory = { session: null, review: null, set: 'mine' };

/** What a game's tabs remember, `FRESH` for a game not seen yet. */
export function memoryOf(memory: GameMemory, gameId: string): TabMemory {
  return Object.hasOwn(memory, gameId) ? (memory[gameId] ?? FRESH) : FRESH;
}

/**
 * The memory after the app showed `route`. The same object when nothing changes, so that the app
 * can keep its state. The Play tab follows its route exactly (a new game replaces the session, a
 * page without one forgets it); the Review tab keeps the last game that was open; the History
 * tab keeps its set.
 */
export function remember(memory: GameMemory, route: Route): GameMemory {
  if (route.page !== 'game') return memory;
  const before = memoryOf(memory, route.gameId);
  let after: TabMemory = before;
  if (route.tab === 'play') {
    if (route.sessionId !== before.session) after = { ...before, session: route.sessionId };
  } else if (route.tab === 'review') {
    const review = route.review;
    if (
      review !== null &&
      (before.review?.kind !== review.kind || before.review.id !== review.id)
    ) {
      after = { ...before, review };
    }
  } else if (route.set !== before.set) {
    after = { ...before, set: route.set };
  }
  return after === before ? memory : { ...memory, [route.gameId]: after };
}

/** The hash each tab's link leads to. */
export function tabHashes(memory: GameMemory, gameId: string): Record<GameTab, string> {
  const seen = memoryOf(memory, gameId);
  return {
    play: routeHash({ page: 'game', gameId, tab: 'play', sessionId: seen.session }),
    review: routeHash({ page: 'game', gameId, tab: 'review', review: seen.review }),
    history: routeHash({ page: 'game', gameId, tab: 'history', set: seen.set }),
  };
}
