import { useCallback, useState } from 'react';
import { createSession, deleteSession, type GameSummary, type Session } from './api';
import { BoardSlot } from './BoardSlot';
import type { GamesState } from './hooks';
import { ServerNotice } from './HomePage';
import { SeatMark } from './Marks';
import { SetupDialog, type SetupResult } from './SetupDialog';
import { routeHash } from './route';
import {
  bottomSeat,
  moveRows,
  resultText,
  seatMark,
  sessionTitle,
  statusText,
} from './sessionView';
import {
  buildSessionRequest,
  clockLabel,
  clockOf,
  thinkLabel,
  MODE_TITLES,
  type ClockSettings,
  type Mode,
} from './setup';
import { useSession } from './useSession';

const LOBBY: { mode: Mode; hint: string }[] = [
  { mode: 'friend', hint: 'Two people, one screen' },
  { mode: 'computer', hint: 'Any release, any think time' },
  { mode: 'bots', hint: 'Plays a full game in a few seconds' },
];

const GAME_BLURBS: Record<string, string> = {
  uttt: 'Nine small boards. Your move sends the opponent to the small board matching the cell you played. Win three small boards in a row, or the most small boards when no move is left.',
};
const DEFAULT_BLURB = 'Play a friend, play a release, or watch two releases play each other.';

interface Props {
  gameId: string;
  sessionId: string | null;
  gamesState: GamesState;
  /** The clock chosen for this session, kept by the app until a later change runs it. */
  clock: ClockSettings | null;
  onStarted: (sessionId: string, clock: ClockSettings | null) => void;
}

function InfoCard({
  game,
  gameId,
  session,
  clock,
  problem,
}: {
  game: GameSummary | null;
  gameId: string;
  session: Session | null;
  clock: ClockSettings | null;
  problem: string | null;
}) {
  if (session === null) {
    return (
      <section className="panel info">
        <h3>{game?.name ?? gameId}</h3>
        <p className="hint">{problem ?? GAME_BLURBS[gameId] ?? DEFAULT_BLURB}</p>
      </section>
    );
  }
  const bot = session.seats.find((seat) => seat.kind === 'bot');
  const bots = session.seats.filter((seat) => seat.kind === 'bot').length;
  return (
    <section className="panel info">
      <h3>{sessionTitle(session)}</h3>
      <dl>
        <dt>Status</dt>
        <dd>{statusText(session)}</dd>
        {bots === 2 &&
          session.seats.map((seat, index) => (
            <Row
              key={index}
              term={seatMark(index)}
              value={`${seat.name}${seat.kind === 'bot' ? `, ${thinkLabel(seat.think_ms)}` : ''}`}
            />
          ))}
        {bots === 1 && bot?.kind === 'bot' && (
          <Row term="Bot" value={`${bot.release}, ${thinkLabel(bot.think_ms)}`} />
        )}
        {clock && <Row term="Clock" value={clockLabel(clock)} />}
        <Row term="Seed" value={String(session.seed)} />
      </dl>
    </section>
  );
}

function Row({ term, value }: { term: string; value: string }) {
  return (
    <>
      <dt>{term}</dt>
      <dd>{value}</dd>
    </>
  );
}

function PlayerStrip({ session, seat }: { session: Session | null; seat: number }) {
  const info = session?.seats[seat];
  const name = info?.name ?? `Player ${seat + 1}`;
  const sub = info ? (info.kind === 'bot' ? thinkLabel(info.think_ms) : seatMark(seat)) : '';
  return (
    <div className="player" data-testid={`player-${seat}`}>
      <div className="who">
        <SeatMark seat={seat} className="chip" />
        <b>{name}</b>
        <small>{sub}</small>
      </div>
      <span className="clock" title="Clocks come later">
        –
      </span>
    </div>
  );
}

function MoveList({
  session,
  problem,
  lost,
}: {
  session: Session | null;
  problem: string | null;
  lost: boolean;
}) {
  if (problem !== null && session === null) {
    return (
      <div className="moves">
        <div className="empty">{problem}</div>
      </div>
    );
  }
  if (session === null) {
    return (
      <div className="moves">
        <div className="empty">Choose a mode on the left to start a game.</div>
      </div>
    );
  }
  const rows = moveRows(session.turns);
  return (
    <div className="moves" aria-label="Moves">
      {rows.length === 0 && <div className="empty">{statusText(session)}</div>}
      {rows.map((row) => (
        <MoveRow key={row.number} number={row.number} moves={row.moves} />
      ))}
      {session.result && (
        <div className="result">
          <b>{resultText(session.result).headline}</b>
          <span>{resultText(session.result).detail}</span>
        </div>
      )}
      {lost && (
        <div className="lost" role="status">
          Connection lost — retrying
        </div>
      )}
    </div>
  );
}

function MoveRow({ number, moves }: { number: number; moves: [string, string | null] }) {
  return (
    <>
      <span className="n">{number}</span>
      <span className="m">{moves[0]}</span>
      {moves[1] === null ? <span /> : <span className="m">{moves[1]}</span>}
    </>
  );
}

export function GamePage({ gameId, sessionId, gamesState, clock, onStarted }: Props) {
  const { session, error } = useSession(sessionId);
  const [dialogMode, setDialogMode] = useState<Mode | null>(null);
  const [coordinates, setCoordinates] = useState(false);

  const game = gamesState.games?.find((candidate) => candidate.id === gameId) ?? null;
  const gameMissing = gamesState.games !== null && game === null;
  const closeDialog = useCallback(() => setDialogMode(null), []);

  const start = async ({ form, seed }: SetupResult) => {
    const request = buildSessionRequest(gameId, form, seed, Math.random);
    const id = await createSession(request);
    if (sessionId !== null) {
      // The old game is forgotten; if the server cannot do it, it drops the session itself.
      deleteSession(sessionId).catch(() => undefined);
    }
    onStarted(id, clockOf(form));
    setDialogMode(null);
    window.location.hash = routeHash({ page: 'game', gameId, sessionId: id });
  };

  const bottom = bottomSeat(session);
  const problem = gameMissing
    ? `The server has no game called ${gameId}.`
    : error && session === null
      ? error.message
      : null;

  return (
    <>
      {gamesState.error?.unreachable && (
        <div className="wrap" style={{ paddingBlock: '24px 0' }}>
          <ServerNotice />
        </div>
      )}
      <main className="wrap game">
        <aside className="side side-left">
          <InfoCard
            game={game}
            gameId={gameId}
            session={session}
            clock={clock}
            problem={gameMissing ? problem : null}
          />
          <section className="panel lobby" aria-label="New game">
            <span className="label">New game</span>
            {LOBBY.map(({ mode, hint }) => (
              <button
                key={mode}
                type="button"
                disabled={game === null}
                onClick={() => setDialogMode(mode)}
              >
                <strong>{MODE_TITLES[mode]}</strong>
                <span>{hint}</span>
              </button>
            ))}
          </section>
          <section className="panel toggles" aria-label="Board settings">
            <span className="label">Board</span>
            <label className="switch">
              Show coordinates
              <input
                type="checkbox"
                checked={coordinates}
                onChange={(event) => setCoordinates(event.target.checked)}
              />
            </label>
            <label className="switch" title="Available with analysis">
              Heatmap of best moves
              <input type="checkbox" disabled />
            </label>
            <p className="hint">Keys: ← → step, Home/End jump, Space play or pause.</p>
          </section>
        </aside>

        <section className="board-col">
          <div className="board-row">
            <div className="evalbar off" title="Win / draw / loss" aria-hidden="true" />
            <BoardSlot showCoordinates={coordinates} label={`${game?.name ?? gameId} board`} />
          </div>
          <div className="playback" data-testid="playback-slot" hidden />
        </section>

        <aside className="side side-right">
          <section className="panel table">
            <PlayerStrip session={session} seat={1 - bottom} />
            <MoveList
              session={session}
              problem={problem}
              lost={session !== null && error !== null}
            />
            <div className="controls">
              {['↶ Takeback', '⚑ Resign', 'Save', 'Export file'].map((label) => (
                <button key={label} className="ctrl" type="button" disabled title="Coming later">
                  {label}
                </button>
              ))}
            </div>
            <PlayerStrip session={session} seat={bottom} />
          </section>
        </aside>
      </main>
      {dialogMode !== null && game !== null && (
        <SetupDialog
          key={dialogMode}
          mode={dialogMode}
          releases={game.releases}
          onStart={start}
          onClose={closeDialog}
        />
      )}
    </>
  );
}
