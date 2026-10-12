/**
 * What the analysis board shows: a saved game, a file loaded in the browser, an arena game
 * published with the site (ADR 0031), or a game played in this page (by its session id).
 */
export type AnalysisKind = 'saved' | 'file' | 'arena' | 'session';

const ANALYSIS_KINDS: readonly string[] = ['saved', 'file', 'arena', 'session'];

/** Whose games the history lists: the visitor's own, or the arena games published with the site. */
export type HistorySet = 'mine' | 'arena';

/** The views of a game's page: the two tabs, and the analysis board, which is not a tab. */
export type GameTab = 'play' | 'history' | 'analysis';

/** The tabs the top bar shows. */
export type ShownTab = Exclude<GameTab, 'analysis'>;

/** Where the app is, read from the URL hash. */
export type Route =
  | { page: 'home' }
  | { page: 'game'; gameId: string; tab: 'play'; sessionId: string | null }
  | { page: 'game'; gameId: string; tab: 'history'; set: HistorySet }
  | { page: 'game'; gameId: string; tab: 'analysis'; board: { kind: AnalysisKind; id: string } };

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

function isAnalysisKind(value: string): value is AnalysisKind {
  return ANALYSIS_KINDS.includes(value);
}

/** The route of `#/game/<gameId>/<rest...>`. */
function parseGame(gameId: string, rest: string[]): Route {
  const [tab, kind, id] = rest;
  if (rest.length === 0) return { page: 'game', gameId, tab: 'play', sessionId: null };
  if (tab === 'analysis') {
    return rest.length === 3 && kind && id && isAnalysisKind(kind)
      ? { page: 'game', gameId, tab: 'analysis', board: { kind, id } }
      : HOME;
  }
  // The Review tab of the earlier versions is gone, with its addresses.
  if (tab === 'review') return HOME;
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
        case 'history':
          return route.set === 'arena' ? `${game}/history/arena` : `${game}/history`;
        case 'analysis':
          return `${game}/analysis/${route.board.kind}/${encodeURIComponent(route.board.id)}`;
      }
    }
  }
}
