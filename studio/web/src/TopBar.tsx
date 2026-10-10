import type { GameSummary } from './api';
import type { Route } from './route';

/** The top bar: brand, navigation, and the current game as a crumb. */
export function TopBar({ route, games }: { route: Route; games: GameSummary[] | null }) {
  const gameName =
    route.page === 'game'
      ? (games?.find((game) => game.id === route.gameId)?.name ?? route.gameId)
      : null;
  return (
    <header className="top">
      <a className="brand" href="#/" aria-label="studio, home">
        studio<span>.</span>
      </a>
      <nav className="nav" aria-label="Main">
        <a href="#/" aria-current={route.page === 'history' ? undefined : 'page'}>
          Games
        </a>
        <a href="#/history" aria-current={route.page === 'history' ? 'page' : undefined}>
          History
        </a>
      </nav>
      {gameName !== null && <span className="crumb">{gameName}</span>}
    </header>
  );
}
