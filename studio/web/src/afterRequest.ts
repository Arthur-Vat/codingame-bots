/* A request the analysis board makes of the Play tab for a game just played: Rematch or New game.
   The Play tab stays mounted while the analysis board is shown, so it holds the session, the clock
   settings and the way to start a game; the board only asks. Pure functions; the app holds it. */

import type { Route } from './route';

export interface AfterRequest {
  gameId: string;
  /** The session the request is about: the Play tab acts only while it still shows it. */
  sessionId: string;
  action: 'rematch' | 'new-game';
}

/**
 * Whether a request is still wanted on `route`: while the analysis board of its session is shown
 * (it navigates to the Play tab next) and while the Play tab shows that session. Anywhere else,
 * or with another session in the Play tab, it is stale and is dropped.
 */
export function requestIsLive(request: AfterRequest, route: Route): boolean {
  if (route.page !== 'game' || route.gameId !== request.gameId) return false;
  if (route.tab === 'play') return route.sessionId === request.sessionId;
  if (route.tab === 'analysis') {
    return route.board.kind === 'session' && route.board.id === request.sessionId;
  }
  return false;
}
