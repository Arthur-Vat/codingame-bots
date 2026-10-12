import type { GameSummary } from './api';
import { engineKind } from './engine/detect';
import { routeHash, type Route } from './route';

/** The top bar: brand, navigation, and the current game as a crumb. */
export function TopBar({ route, games }: { route: Route; games: GameSummary[] | null }) {
  const gameName =
    route.page === 'game'
      ? (games?.find((game) => game.id === route.gameId)?.name ?? route.gameId)
      : null;
  const inBrowser = engineKind() === 'browser';
  // From an arena game, History leads back to the arena games.
  const historyLink = routeHash({
    page: 'history',
    set: route.page === 'review' && route.kind === 'arena' ? 'arena' : 'mine',
  });
  return (
    <header
      className="top"
      title={inBrowser ? 'Running in your browser' : 'Running on the local server'}
    >
      <a className="brand" href="#/" aria-label="studio, home">
        studio<span>.</span>
      </a>
      <nav className="nav" aria-label="Main">
        <a href="#/" aria-current={route.page === 'history' ? undefined : 'page'}>
          Games
        </a>
        <a href={historyLink} aria-current={route.page === 'history' ? 'page' : undefined}>
          History
        </a>
      </nav>
      {gameName !== null && <span className="crumb">{gameName}</span>}
      {inBrowser && <span className="engine">in your browser</span>}
    </header>
  );
}
