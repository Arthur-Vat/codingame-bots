import type { GameSummary } from './api';
import { engineKind } from './engine/detect';
import { currentTab } from './gameTabs';
import type { Route, ShownTab } from './route';

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

const TABS: { tab: ShownTab; label: string }[] = [
  { tab: 'play', label: 'Play' },
  { tab: 'history', label: 'History' },
];

/**
 * The top bar: brand, navigation, and on a game's page the game as a crumb with its two tabs.
 * `tabs` holds where each tab's link leads (what the tab showed last); the board loses no height
 * to them.
 */
export function TopBar({
  route,
  games,
  tabs,
}: {
  route: Route;
  games: GameSummary[] | null;
  tabs: Record<ShownTab, string> | null;
}) {
  const gameName =
    route.page === 'game'
      ? (games?.find((game) => game.id === route.gameId)?.name ?? route.gameId)
      : null;
  const inBrowser = engineKind() === 'browser';
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
        <a href="#/" aria-current={route.page === 'home' ? 'page' : undefined}>
          Games
        </a>
      </nav>
      {gameName !== null && <span className="crumb">{gameName}</span>}
      {route.page === 'game' && tabs !== null && (
        <nav className="nav tabs" aria-label="Game">
          {TABS.map(({ tab, label }) => (
            <a
              key={tab}
              href={tabs[tab]}
              aria-current={currentTab(route) === tab ? 'page' : undefined}
            >
              {label}
            </a>
          ))}
        </nav>
      )}
      {inBrowser && <span className="engine">in your browser</span>}
    </header>
  );
}
