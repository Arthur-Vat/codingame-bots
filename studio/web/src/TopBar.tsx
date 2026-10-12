import type { GameSummary } from './api';
import { engineKind } from './engine/detect';
import { routeHash, type Route } from './route';

/** The mark: nine small squares, the centre one in the accent colour (the favicon is the same). */
function LogoMark() {
  const squares = [0, 1, 2].flatMap((row) => [0, 1, 2].map((col) => [row, col] as const));
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true" focusable="false">
      {squares.map(([row, col]) => (
        <rect
          key={`${row}${col}`}
          className={row === 1 && col === 1 ? 'mark-core' : 'mark-rest'}
          x={col === 0 ? 0 : col === 1 ? 7.5 : 15}
          y={row === 0 ? 0 : row === 1 ? 7.5 : 15}
          width="5"
          height="5"
          rx="1.2"
        />
      ))}
    </svg>
  );
}

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
        <LogoMark />
        studio
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
