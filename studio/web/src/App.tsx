import { useState } from 'react';
import { GamePage } from './GamePage';
import { HistoryPage } from './HistoryPage';
import { HomePage } from './HomePage';
import { TopBar } from './TopBar';
import { useGames, useRoute } from './hooks';
import type { ClockSettings } from './setup';

/** The clock chosen for a session. A later change runs it. */
interface SessionClock {
  sessionId: string;
  clock: ClockSettings | null;
}

export default function App() {
  const route = useRoute();
  const gamesState = useGames();
  const [sessionClock, setSessionClock] = useState<SessionClock | null>(null);

  return (
    <>
      <TopBar route={route} games={gamesState.games} />
      {route.page === 'home' && <HomePage state={gamesState} />}
      {route.page === 'history' && <HistoryPage />}
      {route.page === 'game' && (
        <GamePage
          key={route.gameId}
          gameId={route.gameId}
          sessionId={route.sessionId}
          gamesState={gamesState}
          clock={
            sessionClock !== null && sessionClock.sessionId === route.sessionId
              ? sessionClock.clock
              : null
          }
          onStarted={(sessionId, clock) => setSessionClock({ sessionId, clock })}
        />
      )}
    </>
  );
}
