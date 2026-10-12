import { Fragment, useState } from 'react';
import { GamePage } from './GamePage';
import { memoryOf, NO_MEMORY, remember, tabHashes, type GameMemory } from './gameTabs';
import { HistoryPage } from './HistoryPage';
import { HomePage } from './HomePage';
import { useGames, useRoute } from './hooks';
import { StatusPanel } from './Panels';
import { ReviewTab } from './ReviewPage';
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

  const tabs = route.page === 'game' ? tabHashes(memory, route.gameId) : null;
  const games = gamesState.games;
  // The Play tab shows its route's session, or the one it last had when another tab is shown.
  const playSession =
    route.page !== 'game'
      ? null
      : route.tab === 'play'
        ? route.sessionId
        : memoryOf(memory, route.gameId).session;

  return (
    <>
      <TopBar route={route} games={games} tabs={tabs} />
      {route.page === 'home' && <HomePage state={gamesState} />}
      {route.page === 'game' && (
        // One page for the game. The Play tab stays mounted while the Review or the History tab
        // is shown, so a game goes on (polling, human clocks, a timeout is still reported).
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
          />
          {route.tab !== 'play' && games !== null && !games.some((g) => g.id === route.gameId) ? (
            <StatusPanel text={`The server has no game called ${route.gameId}.`} alert />
          ) : route.tab === 'review' ? (
            <ReviewTab
              key={route.review === null ? '' : `${route.review.kind}/${route.review.id}`}
              gameId={route.gameId}
              review={route.review}
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
