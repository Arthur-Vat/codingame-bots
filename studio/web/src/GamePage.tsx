import { useCallback, useEffect, useEffectEvent, useRef, useState } from 'react';
import {
  createSession,
  deleteSession,
  getRecord,
  postEnd,
  postMove,
  postTakeback,
  saveSession,
  type GameSummary,
  type Session,
  type SessionRequest,
} from './api';
import { BoardOverlay } from './BoardOverlay';
import { download, errorMessage, typesText } from './browser';
import { BoardSlot } from './BoardSlot';
import { clockSeed } from './setup';
import { renderers, resultDetails } from './games/registry';
import { Controls, EndBlock, MoveList, PlayerStrip, Row } from './GameTable';
import type { GamesState } from './hooks';
import { ServerNotice } from './HomePage';
import {
  canPlay,
  controlsFor,
  cursorAfterKey,
  exportFileName,
  findMoveIndex,
  frameAfterTurn,
  rematchRequest,
  runningSeat,
  timeoutDue,
  sessionMode,
  turnOfFrame,
} from './play';
import { SetupDialog, type SetupResult } from './SetupDialog';
import { routeHash } from './route';
import { bottomSeat, overlayFor, seatMark, sessionTitle, statusText } from './sessionView';
import {
  buildSessionRequest,
  clockLabel,
  clockOf,
  thinkLabel,
  MODE_TITLES,
  type ClockSettings,
  type Mode,
} from './setup';
import { Toast, useToast } from './Toast';
import { useClocks } from './useClocks';
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

/** How long to wait before reporting a timeout again after a failure. */
const TIMEOUT_RETRY_MS = 1000;

export function GamePage({ gameId, sessionId, gamesState, clock, onStarted }: Props) {
  const { session, error, apply } = useSession(sessionId);
  const [dialogMode, setDialogMode] = useState<Mode | null>(null);
  const [coordinates, setCoordinates] = useState(false);
  // A request of ours is in flight: input and clocks wait for its answer.
  const [pending, setPending] = useState(false);
  const inFlight = useRef(false);
  // A new game is being created (Rematch, or a dialog's Start): one at a time.
  const launching = useRef(false);
  const [starting, setStarting] = useState(false);
  // After a failed timeout report, the next try waits a moment.
  const [backingOff, setBackingOff] = useState(false);
  // The frame the user looks at while looking back; null follows the latest frame.
  const [pinned, setPinned] = useState<{ sessionId: string | null; frame: number } | null>(null);
  const toast = useToast();

  const game = gamesState.games?.find((candidate) => candidate.id === gameId) ?? null;
  const gameMissing = gamesState.games !== null && game === null;
  const closeDialog = useCallback(() => setDialogMode(null), []);

  const clocks = useClocks(session, clock);
  const running = clock === null ? null : runningSeat(session);

  const frames = session?.frames ?? [];
  const latest = frames.length - 1;
  const pinnedFrame = pinned !== null && pinned.sessionId === sessionId ? pinned.frame : null;
  const shown = pinnedFrame === null ? latest : Math.min(pinnedFrame, latest);
  const atLatest = shown === latest;
  const shownTurn = session === null ? -1 : turnOfFrame(shown, session.opening_turns);

  const launch = async (request: SessionRequest, newClock: ClockSettings | null) => {
    if (launching.current) return;
    launching.current = true;
    setStarting(true);
    try {
      const id = await createSession(request);
      if (sessionId !== null) {
        // The old game is forgotten; if the server cannot do it, it drops the session itself.
        deleteSession(sessionId).catch(() => undefined);
      }
      onStarted(id, newClock);
      setDialogMode(null);
      window.location.hash = routeHash({ page: 'game', gameId, sessionId: id });
    } finally {
      launching.current = false;
      setStarting(false);
    }
  };

  const start = ({ form, seed }: SetupResult) =>
    launch(buildSessionRequest(gameId, form, seed, Math.random), clockOf(form));

  /** Runs a request that answers with the session, one at a time. `onFailure` runs before the request counts as settled. */
  const run = async (job: (current: Session) => Promise<Session>, onFailure?: () => void) => {
    if (session === null || inFlight.current) return;
    inFlight.current = true;
    setPending(true);
    try {
      apply(await job(session));
    } catch (failure) {
      toast.show(errorMessage(failure));
      onFailure?.();
    } finally {
      inFlight.current = false;
      setPending(false);
    }
  };

  const playable = canPlay(session, pending, atLatest);
  const onAction = (action: unknown) => {
    if (session === null || !playable) return;
    const index = findMoveIndex(session.human_moves, action);
    const seat = session.to_act[0];
    if (index === -1 || seat === undefined) return;
    void run((current) => postMove(current.id, seat, index));
  };

  const controls = controlsFor(session, pending);
  const takeback = () => {
    const keep = controls.takebackTo;
    if (keep === null) return;
    setPinned(null);
    void run((current) => postTakeback(current.id, keep));
  };
  const resign = () => {
    const seat = controls.resignSeat;
    if (seat === null) return;
    void run((current) => postEnd(current.id, seat, 'resign'));
  };

  const save = async () => {
    if (session === null) return;
    try {
      const { duplicate } = await saveSession(session.id);
      toast.show(duplicate ? 'Already in your history' : 'Saved to your history');
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const exportFile = async () => {
    if (session === null) return;
    try {
      download(exportFileName(session.game, session.seed, new Date()), await getRecord(session.id));
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const rematch = async () => {
    if (session === null) return;
    try {
      await launch(rematchRequest(session, clockSeed(Date.now())), clock);
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };

  // A human clock that reaches zero ends the game (it expires once, see clock.ts). The report is
  // made as soon as nothing of ours is in flight and the server is not busy, and again after a
  // failure, so a timeout is never lost.
  const timeoutSeat = timeoutDue(session, clocks?.expired ?? null, pending, backingOff);
  const reportTimeout = useEffectEvent((seat: number) => {
    void run(
      (current) => postEnd(current.id, seat, 'timeout'),
      () => {
        setBackingOff(true);
        setTimeout(() => setBackingOff(false), TIMEOUT_RETRY_MS);
      },
    );
  });
  useEffect(() => {
    if (timeoutSeat !== null) reportTimeout(timeoutSeat);
  }, [timeoutSeat]);

  // ← → Home End look back and forward, as on Lichess.
  const onKey = useEffectEvent((event: KeyboardEvent) => {
    if (dialogMode !== null || latest < 0 || typesText(event.target)) return;
    if (event.altKey || event.ctrlKey || event.metaKey) return;
    const next = cursorAfterKey(event.key, shown, latest);
    if (next === undefined) return;
    event.preventDefault();
    setPinned(next === null ? null : { sessionId, frame: next });
  });
  useEffect(() => {
    const handler = (event: KeyboardEvent) => onKey(event);
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  const pickTurn = (turn: number) => {
    if (session === null) return;
    const frame = frameAfterTurn(turn, session.opening_turns);
    setPinned(frame >= latest ? null : { sessionId, frame });
  };

  const bottom = bottomSeat(session);
  const problem = gameMissing
    ? `The server has no game called ${gameId}.`
    : error && session === null
      ? error.message
      : null;

  const Renderer = renderers[gameId];
  const frame = frames[shown];
  const finalFrame = frames[latest];
  const detail =
    session?.result?.end.kind === 'finished' && finalFrame !== undefined
      ? (resultDetails[gameId]?.(finalFrame) ?? null)
      : null;
  const mode = session === null ? null : sessionMode(session);
  const over = session?.status === 'over';

  const openReview = useCallback(() => {
    if (sessionId !== null) {
      window.location.hash = routeHash({ page: 'review', kind: 'session', id: sessionId });
    }
  }, [sessionId]);
  // A game between two bots opens its review when it ends, if it was seen going on.
  const seenGoing = useRef<string | null>(null);
  const status = session?.status;
  useEffect(() => {
    if (sessionId === null || status === undefined) return;
    if (status !== 'over') {
      seenGoing.current = sessionId;
    } else if (seenGoing.current === sessionId && mode === 'bots') {
      seenGoing.current = null;
      openReview();
    }
  }, [sessionId, status, mode, openReview]);

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
            {LOBBY.map(({ mode: lobbyMode, hint }) => (
              <button
                key={lobbyMode}
                type="button"
                disabled={game === null}
                onClick={() => setDialogMode(lobbyMode)}
              >
                <strong>{MODE_TITLES[lobbyMode]}</strong>
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
            <p className="hint">Keys: ← → step, Home/End jump.</p>
          </section>
        </aside>

        <section className="board-col">
          <div className="board-row">
            <div className="evalbar off" title="Win / draw / loss" aria-hidden="true" />
            <BoardSlot
              showCoordinates={coordinates}
              label={`${game?.name ?? gameId} board`}
              bare={Renderer !== undefined}
              overlay={session === null ? null : <BoardOverlay overlay={overlayFor(session)} />}
            >
              {Renderer !== undefined && frame !== undefined ? (
                <Renderer
                  frame={frame}
                  interactive={playable}
                  onAction={onAction}
                  showCoordinates={coordinates}
                />
              ) : undefined}
            </BoardSlot>
          </div>
          <div className="playback" data-testid="playback-slot" hidden />
        </section>

        <aside className="side side-right">
          <section className="panel table">
            <PlayerStrip session={session} seat={1 - bottom} clocks={clocks} running={running} />
            <MoveList
              session={session}
              problem={problem}
              lost={session !== null && error !== null}
              shownTurn={shownTurn}
              atLatest={atLatest}
              detail={detail}
              onPick={pickTurn}
            />
            {session !== null && (
              <Controls
                showPlayControls={mode !== 'bots' && !over}
                takebackTo={controls.takebackTo}
                resignSeat={controls.resignSeat}
                canSave={session.turns.length > 0}
                onTakeback={takeback}
                onResign={resign}
                onSave={() => void save()}
                onExport={() => void exportFile()}
              />
            )}
            {over && mode !== null && (
              <EndBlock
                disabled={pending || starting}
                onRematch={() => void rematch()}
                onNewGame={() => setDialogMode(mode)}
                onReview={openReview}
              />
            )}
            <PlayerStrip session={session} seat={bottom} clocks={clocks} running={running} />
          </section>
        </aside>
      </main>
      <Toast text={toast.text} />
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
