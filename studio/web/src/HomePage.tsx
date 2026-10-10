import { BoardThumb } from './BoardThumb';
import type { GamesState } from './hooks';
import { HOME_HEADING, HOME_INTRO, SERVER_HINT, releaseCount } from './home';
import { defaultRelease } from './setup';

/** The notice shown in place of the content when the server does not answer. */
export function ServerNotice({ message }: { message?: string }) {
  return (
    <section className="panel notice" role="alert">
      <h2>The server is not running</h2>
      <p>{SERVER_HINT}</p>
      {message && <p className="hint">{message}</p>}
    </section>
  );
}

export function HomePage({ state }: { state: GamesState }) {
  const { games, error } = state;
  return (
    <main className="wrap">
      <div className="home-head">
        <h1>{HOME_HEADING}</h1>
        <p>{HOME_INTRO}</p>
      </div>
      {error ? (
        error.unreachable ? (
          <ServerNotice />
        ) : (
          <section className="panel notice" role="alert">
            <h2>The games could not be loaded</h2>
            <p>{error.message}</p>
          </section>
        )
      ) : games === null ? (
        <p className="hint">Loading the games…</p>
      ) : (
        <div className="cards">
          {games.map((game) => (
            <a key={game.id} className="card live" href={`#/game/${encodeURIComponent(game.id)}`}>
              <div className="thumb">
                <BoardThumb />
              </div>
              <h2>{game.name}</h2>
              <div className="meta">
                <span>{releaseCount(game.releases.length)}</span>
                {game.releases.length > 0 && <span>Latest: {defaultRelease(game.releases)}</span>}
              </div>
            </a>
          ))}
          <div className="card soon">
            <h2>Next game</h2>
            <p className="hint">Appears when the framework adds its second game.</p>
          </div>
          <div className="card soon">
            <h2>More games</h2>
            <p className="hint">
              Each game brings its referee, a board renderer and, later, its analysis.
            </p>
          </div>
        </div>
      )}
    </main>
  );
}
