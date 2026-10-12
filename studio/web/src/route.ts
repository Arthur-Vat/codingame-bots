/**
 * What a review shows: a session of the server, a saved game, a file loaded in the browser, or an
 * arena game published with the site (ADR 0031).
 */
export type ReviewKind = 'session' | 'saved' | 'file' | 'arena';

const REVIEW_KINDS: readonly string[] = ['session', 'saved', 'file', 'arena'];

/** Whose games the history lists: the visitor's own, or the arena games published with the site. */
export type HistorySet = 'mine' | 'arena';

/** Where the app is, read from the URL hash. */
export type Route =
  | { page: 'home' }
  | { page: 'history'; set: HistorySet }
  | { page: 'review'; kind: ReviewKind; id: string }
  | { page: 'game'; gameId: string; sessionId: string | null };

/**
 * Reads a hash such as `#/game/uttt/ab12` into a route. Anything unknown is the home page.
 * Segments are percent-decoded; an empty session segment means no session.
 */
export function parseRoute(hash: string): Route {
  const path = hash.replace(/^#/, '').split('?')[0] ?? '';
  const segments = path
    .split('/')
    .filter((segment) => segment !== '')
    .map(decodeSegment);
  const [first, gameId, sessionId] = segments;
  if (first === 'history' && segments.length === 1) {
    return { page: 'history', set: 'mine' };
  }
  if (first === 'history' && segments.length === 2 && gameId === 'arena') {
    return { page: 'history', set: 'arena' };
  }
  if (first === 'review' && segments.length === 3 && gameId && sessionId) {
    // The second segment is the kind and the third the id.
    return REVIEW_KINDS.includes(gameId)
      ? { page: 'review', kind: gameId as ReviewKind, id: sessionId }
      : { page: 'home' };
  }
  if (first === 'game' && gameId && segments.length <= 3) {
    return { page: 'game', gameId, sessionId: sessionId ?? null };
  }
  return { page: 'home' };
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
    case 'history':
      return route.set === 'arena' ? '#/history/arena' : '#/history';
    case 'review':
      return `#/review/${route.kind}/${encodeURIComponent(route.id)}`;
    case 'game': {
      const game = `#/game/${encodeURIComponent(route.gameId)}`;
      return route.sessionId ? `${game}/${encodeURIComponent(route.sessionId)}` : game;
    }
  }
}
