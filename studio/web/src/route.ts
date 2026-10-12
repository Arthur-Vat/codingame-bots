/**
 * What a review shows: a saved game, a file loaded in the browser, or an arena game published with
 * the site (ADR 0031). A game played here is reviewed in place, in the Play tab.
 */
export type ReviewKind = 'saved' | 'file' | 'arena';

const REVIEW_KINDS: readonly string[] = ['saved', 'file', 'arena'];

/** Whose games the history lists: the visitor's own, or the arena games published with the site. */
export type HistorySet = 'mine' | 'arena';

/** The three tabs of a game's page. */
export type GameTab = 'play' | 'review' | 'history';

/** Where the app is, read from the URL hash. */
export type Route =
  | { page: 'home' }
  | { page: 'game'; gameId: string; tab: 'play'; sessionId: string | null }
  | { page: 'game'; gameId: string; tab: 'review'; review: { kind: ReviewKind; id: string } | null }
  | { page: 'game'; gameId: string; tab: 'history'; set: HistorySet };

/**
 * Reads a hash such as `#/game/uttt/ab12` into a route. Anything unknown is the home page.
 * Segments are percent-decoded; an empty segment is skipped.
 */
export function parseRoute(hash: string): Route {
  const path = hash.replace(/^#/, '').split('?')[0] ?? '';
  const segments = path
    .split('/')
    .filter((segment) => segment !== '')
    .map(decodeSegment);
  const [first, second] = segments;
  if (first === 'game' && second) {
    return parseGame(second, segments.slice(2));
  }
  return HOME;
}

const HOME: Route = { page: 'home' };

function isReviewKind(value: string): value is ReviewKind {
  return REVIEW_KINDS.includes(value);
}

/** The route of `#/game/<gameId>/<rest...>`. */
function parseGame(gameId: string, rest: string[]): Route {
  const [tab, kind, id] = rest;
  if (rest.length === 0) return { page: 'game', gameId, tab: 'play', sessionId: null };
  if (tab === 'review') {
    if (rest.length === 1) return { page: 'game', gameId, tab: 'review', review: null };
    if (rest.length === 3 && kind && id) {
      return isReviewKind(kind)
        ? { page: 'game', gameId, tab: 'review', review: { kind, id } }
        : HOME;
    }
    return HOME;
  }
  if (tab === 'history') {
    if (rest.length === 1) return { page: 'game', gameId, tab: 'history', set: 'mine' };
    if (rest.length === 2 && kind === 'arena') {
      return { page: 'game', gameId, tab: 'history', set: 'arena' };
    }
    return HOME;
  }
  if (rest.length === 1 && tab) return { page: 'game', gameId, tab: 'play', sessionId: tab };
  return HOME;
}

function decodeSegment(segment: string): string {
  try {
    return decodeURIComponent(segment);
  } catch {
    return segment;
  }
}

/** The hash of a route, the inverse of [`parseRoute`]. */
export function routeHash(route: Route): string {
  switch (route.page) {
    case 'home':
      return '#/';
    case 'game': {
      const game = `#/game/${encodeURIComponent(route.gameId)}`;
      switch (route.tab) {
        case 'play':
          return route.sessionId ? `${game}/${encodeURIComponent(route.sessionId)}` : game;
        case 'review':
          return route.review === null
            ? `${game}/review`
            : `${game}/review/${route.review.kind}/${encodeURIComponent(route.review.id)}`;
        case 'history':
          return route.set === 'arena' ? `${game}/history/arena` : `${game}/history`;
      }
    }
  }
}
