import { useEffect, useState } from 'react';
import { ApiError, listGames, type GameSummary } from './api';
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

/** The games the server offers, loaded once. */
export function useGames(): GamesState {
  const [state, setState] = useState<GamesState>({ games: null, error: null });
  useEffect(() => {
    let stopped = false;
    listGames().then(
      (games) => {
        if (!stopped) setState({ games, error: null });
      },
      (error: unknown) => {
        if (!stopped) {
          setState({
            games: null,
            error: error instanceof ApiError ? error : new ApiError(String(error), true),
          });
        }
      },
    );
    return () => {
      stopped = true;
    };
  }, []);
  return state;
}
