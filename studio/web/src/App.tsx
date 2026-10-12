import { Fragment, useState } from 'react';
import { requestIsLive, type AfterRequest } from './afterRequest';
import { AnalysisTab } from './AnalysisPage';
import { GamePage } from './GamePage';
import { memoryOf, NO_MEMORY, remember, tabHashes, type GameMemory } from './gameTabs';
import { HistoryPage } from './HistoryPage';
import { HomePage } from './HomePage';
import { goToHash, useGames, useRoute } from './hooks';
import { routeHash } from './route';
import { StatusPanel } from './Panels';
import type { ClockSettings } from './setup';
import { TopBar } from './TopBar';

/** The clock chosen for a session. A later change runs it. */
interface SessionClock {
  sessionId: string;
  clock: ClockSettings | null;
}

export default function App() {
  const route = useRoute();
  const gamesState = useGames();
  const [sessionClock, setSessionClock] = useState<SessionClock | null>(null);
  // What each game's tabs showed last, for the tabs' links and for the Play tab, which goes on
  // with its session while another tab is shown. Kept in memory only: a reload forgets it. A state
  // adjusted while rendering, so the links already lead there in this render.
  const [remembered, setRemembered] = useState<GameMemory>(NO_MEMORY);
  const memory = remember(remembered, route);
  if (memory !== remembered) setRemembered(memory);

  // Rematch or New game asked on the analysis board, taken by the Play tab. One at a time, and
  // dropped as soon as the page is anywhere it no longer makes sense (also adjusted while
  // rendering).
  const [request, setRequest] = useState<AfterRequest | null>(null);
  if (request !== null && !requestIsLive(request, route)) setRequest(null);

  const tabs = route.page === 'game' ? tabHashes(memory, route.gameId) : null;
  const games = gamesState.games;
  // The Play tab shows its route's session, or the one it last had when another tab is shown.
  const playSession =
    route.page !== 'game'
      ? null
      : route.tab === 'play'
        ? route.sessionId
        : memoryOf(memory, route.gameId).session;

  // Rematch and New game on the analysis board, for a game the Play tab still holds.
  const afterActions = (gameId: string, board: { kind: string; id: string }) =>
    board.kind === 'session' && memoryOf(memory, gameId).session === board.id
      ? {
          onRematch: () => ask(gameId, board.id, 'rematch'),
          onNewGame: () => ask(gameId, board.id, 'new-game'),
        }
      : null;
  const ask = (gameId: string, sessionId: string, action: AfterRequest['action']) => {
    setRequest({ gameId, sessionId, action });
    goToHash(routeHash({ page: 'game', gameId, tab: 'play', sessionId }));
  };

  return (
    <>
      <TopBar route={route} games={games} tabs={tabs} />
      {route.page === 'home' && <HomePage state={gamesState} />}
      {route.page === 'game' && (
        // One page for the game. The Play tab stays mounted while the analysis board or the
        // History tab is shown, so a game goes on (polling, human clocks, a timeout is still reported).
        <Fragment key={route.gameId}>
          <GamePage
            gameId={route.gameId}
            sessionId={playSession}
            active={route.tab === 'play'}
            gamesState={gamesState}
            clock={
              sessionClock !== null && sessionClock.sessionId === playSession
                ? sessionClock.clock
                : null
            }
            onStarted={(sessionId, clock) => setSessionClock({ sessionId, clock })}
            request={request}
            onRequestDone={() => setRequest(null)}
          />
          {route.tab !== 'play' && games !== null && !games.some((g) => g.id === route.gameId) ? (
            <StatusPanel text={`The server has no game called ${route.gameId}.`} alert />
          ) : route.tab === 'analysis' ? (
            <AnalysisTab
              key={`${route.board.kind}/${route.board.id}`}
              gameId={route.gameId}
              board={route.board}
              back={
                route.board.kind === 'session'
                  ? {
                      hash: routeHash({
                        page: 'game',
                        gameId: route.gameId,
                        tab: 'play',
                        sessionId: route.board.id,
                      }),
                      label: 'Back to the game',
                    }
                  : {
                      // Back to the list the game came from: the arena games for an arena game.
                      hash: routeHash({
                        page: 'game',
                        gameId: route.gameId,
                        tab: 'history',
                        set: route.board.kind === 'arena' ? 'arena' : 'mine',
                      }),
                      label: 'Back to history',
                    }
              }
              actions={afterActions(route.gameId, route.board)}
            />
          ) : route.tab === 'history' ? (
            // Each set starts with its own filters and its own list.
            <HistoryPage
              key={`${route.gameId}/${route.set}`}
              gameId={route.gameId}
              set={route.set}
              gamesState={gamesState}
            />
          ) : null}
        </Fragment>
      )}
    </>
  );
}
