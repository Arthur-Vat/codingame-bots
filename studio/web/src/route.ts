/** Where the app is, read from the URL hash. */
export type Route =
  | { page: 'home' }
  | { page: 'history' }
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
    return { page: 'history' };
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
      return '#/history';
    case 'game': {
      const game = `#/game/${encodeURIComponent(route.gameId)}`;
      return route.sessionId ? `${game}/${encodeURIComponent(route.sessionId)}` : game;
    }
  }
}
