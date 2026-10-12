/**
 * What a review shows: a session of the server, a saved game, a file loaded in the browser, or an
 * arena game published with the site (ADR 0031).
 */
export type ReviewKind = 'session' | 'saved' | 'file' | 'arena';

const REVIEW_KINDS: readonly string[] = ['session', 'saved', 'file', 'arena'];

/** Whose games the history lists: the visitor's own, or the arena games published with the site. */
export type HistorySet = 'mine' | 'arena';

/** The three tabs of a game's page. */
export type GameTab = 'play' | 'review' | 'history';

/** Where the app is, read from the URL hash. */
export type Route =
  | { page: 'home' }
  | { page: 'game'; gameId: string; tab: 'play'; sessionId: string | null }
  | { page: 'game'; gameId: string; tab: 'review'; review: { kind: ReviewKind; id: string } | null }
  | { page: 'game'; gameId: string; tab: 'history'; set: HistorySet }
  // The pages that came before the tabs, kept working by a redirect.
  | { page: 'old-history'; set: HistorySet }
  | { page: 'old-review'; kind: ReviewKind; id: string };

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
  const [first, second, third] = segments;
  if (first === 'history' && segments.length === 1) {
    return { page: 'old-history', set: 'mine' };
  }
  if (first === 'history' && segments.length === 2 && second === 'arena') {
    return { page: 'old-history', set: 'arena' };
  }
  if (first === 'review' && segments.length === 3 && second && third) {
    // The second segment is the kind and the third the id.
    return isReviewKind(second) ? { page: 'old-review', kind: second, id: third } : HOME;
  }
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

/** The hash of a route, the inverse of [`parseRoute`] (the two old pages get their old hashes). */
export function routeHash(route: Route): string {
  switch (route.page) {
    case 'home':
      return '#/';
    case 'old-history':
      return route.set === 'arena' ? '#/history/arena' : '#/history';
    case 'old-review':
      return `#/review/${route.kind}/${encodeURIComponent(route.id)}`;
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
