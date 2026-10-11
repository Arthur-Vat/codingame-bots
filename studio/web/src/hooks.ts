import { useEffect, useState } from 'react';
import { ApiError, listGames, type GameSummary } from './api';
import { startPoller } from './poller';
import { parseRoute, type Route } from './route';

/** The current route, following the URL hash. */
export function useRoute(): Route {
  const [hash, setHash] = useState(() => window.location.hash);
  useEffect(() => {
    const onChange = () => setHash(window.location.hash);
    window.addEventListener('hashchange', onChange);
    return () => window.removeEventListener('hashchange', onChange);
  }, []);
  return parseRoute(hash);
}

export interface GamesState {
  /** Null while loading or after an error. */
  games: GameSummary[] | null;
  error: ApiError | null;
}

/** How long to wait before asking again while the server does not answer. */
export const GAMES_RETRY_MS = 3000;

/** The games the server offers, loaded once; asked again every 3 s while it is unreachable. */
export function useGames(): GamesState {
  const [state, setState] = useState<GamesState>({ games: null, error: null });
  useEffect(
    () =>
      startPoller<GameSummary[]>({
        fetch: listGames,
        delayAfter: () => null,
        delayAfterError: (error) =>
          error instanceof ApiError && !error.unreachable ? null : GAMES_RETRY_MS,
        onValue: (games) => setState({ games, error: null }),
        onError: (error) =>
          setState({
            games: null,
            error: error instanceof ApiError ? error : new ApiError(String(error), true),
          }),
      }),
    [],
  );
  return state;
}
