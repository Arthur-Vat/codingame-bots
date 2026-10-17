import { useCallback, useEffect, useEffectEvent, useRef, useState } from 'react';
import {
  createSession,
  deleteSession,
  getRecordText,
  postEnd,
  postMove,
  postTakeback,
  saveSession,
  type GameSummary,
  type Session,
  type SessionRequest,
} from './api';
import type { AfterRequest } from './afterRequest';
import { BoardOverlay } from './BoardOverlay';
import { botsPlaying, unplayedView } from './botGame';
import { PlayingOverlay } from './PlayingOverlay';
import { downloadText, errorMessage, typesText } from './browser';
import { BoardSlot } from './BoardSlot';
import { clockSeed } from './setup';
import { emptyFrameFor, ownEntry, renderers, resultDetailFor } from './games/registry';
import { Controls, EndBlock, MoveList, PlayerStrip, Row } from './GameTable';
import type { GamesState } from './hooks';
import { ServerNotice } from './HomePage';
import {
  canPlay,
  hasGameInProgress,
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
import { PlaybackBar } from './PlaybackBar';
import { routeHash } from './route';
import {
  bottomSeat,
  moveBeingPlayed,
  overlayFor,
  seatMark,
  sessionTitle,
  statusText,
} from './sessionView';
import {
  buildSessionRequest,
  clockLabel,
  clockOf,
  thinkLabel,
  DEFAULT_MODE,
  type ClockSettings,
  type Mode,
} from './setup';
import { Toast, useToast } from './Toast';
import { useClocks } from './useClocks';
import { usePlayback } from './usePlayback';
import { useSession } from './useSession';

const GAME_BLURBS: Record<string, string> = {
  uttt: 'Nine small boards. Your move sends the opponent to the small board matching the cell you played. Win three small boards in a row, or the most small boards when no move is left.',
  'ice-and-fire':
    "Train and move units on a 12 by 12 map to take territory and the opponent's headquarters. Cells cut off from your headquarters are lost, and so are the units on them. Bots only: watch two bots play.",
};

interface Props {
  gameId: string;
  sessionId: string | null;
  /** Whether the Play tab is the one shown. The page stays mounted while another tab is shown, so a game goes on (its clocks, its bots), but it takes no key and opens no dialog. */
  active: boolean;
  gamesState: GamesState;
  /** The clock chosen for this session, kept by the app until a later change runs it. */
  clock: ClockSettings | null;
  onStarted: (sessionId: string, clock: ClockSettings | null) => void;
  /** Rematch or New game asked on the analysis board of the session; run once, then cleared. */
  request: AfterRequest | null;
  onRequestDone: () => void;
}

function InfoCard({
  game,
  gameId,
  session,
  move,
  clock,
  problem,
}: {
  game: GameSummary | null;
  gameId: string;
  session: Session | null;
  /** The number of the move being played, from the real session (a game of two bots hides its moves). */
  move: number | null;
  clock: ClockSettings | null;
  problem: string | null;
}) {
  if (session === null) {
    const blurb = problem ?? ownEntry(GAME_BLURBS, gameId);
    return (
      <section className="panel info">
        <h3>{game?.name ?? gameId}</h3>
        {blurb !== undefined && <p className="hint">{blurb}</p>}
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
        {session.status === 'over' && <Row term="Source" value="Played here" />}
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
        {bots === 2 && move !== null && <Row term="Move" value={String(move)} />}
        {clock && <Row term="Clock" value={clockLabel(clock)} />}
        <Row term="Seed" value={String(session.seed)} />
      </dl>
    </section>
  );
}

/** How long to wait before reporting a timeout again after a failure. */
const TIMEOUT_RETRY_MS = 1000;

export function GamePage({
  gameId,
  sessionId,
  active,
  gamesState,
  clock,
  onStarted,
  request,
  onRequestDone,
}: Props) {
  const { session, error, apply } = useSession(sessionId);
  // The setup menu: null while it is closed, else the mode it opened with.
  const [dialogMode, setDialogMode] = useState<Mode | null>(null);
  // The mode last chosen in the menu, which the menu opens with next.
  const [lastMode, setLastMode] = useState<Mode>(DEFAULT_MODE);
  // The menu has opened by itself since the Play tab was last shown without a game.
  const [autoOpened, setAutoOpened] = useState(false);
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
  // The session whose finished game was saved from this page (it reads "In your history").
  const [savedSession, setSavedSession] = useState<string | null>(null);
  const [pinned, setPinned] = useState<{ sessionId: string | null; frame: number } | null>(null);
  const toast = useToast();

  const game = gamesState.games?.find((candidate) => candidate.id === gameId) ?? null;
  const gameMissing = gamesState.games !== null && game === null;
  const closeDialog = useCallback(() => setDialogMode(null), []);
  const openMenu = (mode: Mode) => {
    setLastMode(mode);
    setDialogMode(mode);
  };
  // A page that is not shown opens no menu.
  if (!active && dialogMode !== null) setDialogMode(null);
  // The Play tab shown with no game opens the menu by itself, once: not again after it was closed,
  // but again the next time the tab is shown without a game. A session in the address never opens
  // it, and it waits for the game list to know the game. (Adjusted while rendering, so it runs
  // once also in strict mode.)
  if (!active || sessionId !== null) {
    if (autoOpened) setAutoOpened(false);
  } else if (game !== null && !autoOpened) {
    setAutoOpened(true);
    setDialogMode(lastMode);
  }

  // While two bots play, nothing of their game is shown (a spinner is, over the empty board); the
  // actions below keep using the real session.
  const shownSession = session === null ? null : unplayedView(session);
  const playing = session !== null && botsPlaying(session);

  const clocks = useClocks(shownSession, clock);
  const running = clock === null ? null : runningSeat(shownSession);

  const frames = shownSession?.frames ?? [];
  const latest = frames.length - 1;
  const pinnedFrame = pinned !== null && pinned.sessionId === sessionId ? pinned.frame : null;
  const looked = pinnedFrame === null ? latest : Math.min(pinnedFrame, latest);
  // A game that is over stays here with its playback: it starts on the frame that was looked at,
  // and a game between two bots on its first frame, paused.
  const over = shownSession?.status === 'over';
  const startFrame = over && sessionMode(shownSession) === 'bots' ? 0 : looked;
  const playback = usePlayback(latest, startFrame, over ? shownSession.id : null, {
    active,
    keys: active && dialogMode === null,
  });
  const shown = over ? playback.cursor : looked;
  const atLatest = shown === latest;
  const shownTurn = shownSession === null ? -1 : turnOfFrame(shown, shownSession.opening_turns);

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
      window.location.hash = routeHash({ page: 'game', gameId, tab: 'play', sessionId: id });
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

  const playable = canPlay(shownSession, pending, atLatest);
  const onAction = (action: unknown) => {
    if (session === null || !playable) return;
    const index = findMoveIndex(session.human_moves, action);
    const seat = session.to_act[0];
    if (index === -1 || seat === undefined) return;
    void run((current) => postMove(current.id, seat, index));
  };

  const controls = controlsFor(shownSession, pending);
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
      // Only a game that is over is saved whole: a game saved while it went on is not "in" the history.
      if (session.status === 'over') setSavedSession(session.id);
      toast.show(duplicate ? 'Already in your history' : 'Saved to your history');
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const exportFile = async () => {
    if (session === null) return;
    try {
      downloadText(
        exportFileName(session.game, session.seed, new Date()),
        await getRecordText(session.id),
      );
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
    if (!active || over || dialogMode !== null || latest < 0 || typesText(event.target)) return;
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
    if (shownSession === null) return;
    const frame = frameAfterTurn(turn, shownSession.opening_turns);
    if (over) {
      playback.goto(frame);
      return;
    }
    setPinned(frame >= latest ? null : { sessionId, frame });
  };

  const bottom = bottomSeat(shownSession);
  const problem = gameMissing
    ? `The server has no game called ${gameId}.`
    : error && session === null
      ? error.message
      : null;

  const Renderer = renderers[gameId];
  const frame = frames[shown];
  // With no frame yet (no game, or a session still compiling) the game's empty board is drawn.
  const shownFrame = frame ?? emptyFrameFor(gameId);
  const finalFrame = frames[latest];
  const detail =
    shownSession?.result?.end.kind === 'finished' && finalFrame !== undefined
      ? resultDetailFor(gameId, finalFrame)
      : null;
  const mode = shownSession === null ? null : sessionMode(shownSession);
  const gameInProgress = hasGameInProgress(session);

  // Rematch or New game asked on the analysis board of this session, once the Play tab is shown
  // with the finished game. The request is cleared as it is taken; a request taken already (the
  // effect runs twice in strict mode) is not run again.
  const requested =
    request !== null &&
    active &&
    over &&
    request.gameId === gameId &&
    request.sessionId === sessionId &&
    session?.id === sessionId
      ? request
      : null;
  const handled = useRef<AfterRequest | null>(null);
  const runRequest = useEffectEvent((taken: AfterRequest) => {
    onRequestDone();
    if (taken.action === 'rematch') {
      void rematch();
    } else if (mode !== null) {
      openMenu(mode);
    }
  });
  useEffect(() => {
    if (requested === null || handled.current === requested) return;
    handled.current = requested;
    runRequest(requested);
  }, [requested]);

  return (
    <>
      {gamesState.error?.unreachable && (
        <div className="wrap" hidden={!active} style={{ paddingBlock: '24px 0' }}>
          <ServerNotice />
        </div>
      )}
      <main className="wrap game" hidden={!active} data-testid="play-tab">
        <aside className="side side-left">
          <InfoCard
            game={game}
            gameId={gameId}
            session={shownSession}
            move={session === null ? null : moveBeingPlayed(session)}
            clock={clock}
            problem={gameMissing ? problem : null}
          />
          <section className="panel new-game" aria-label="New game">
            <button
              className="btn primary"
              type="button"
              disabled={game === null}
              onClick={() => openMenu(lastMode)}
            >
              New game
            </button>
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
            <p className="hint">
              {over
                ? 'Keys: ← → step, Home/End jump, Space plays.'
                : 'Keys: ← → step, Home/End jump.'}
            </p>
          </section>
        </aside>

        <section className="board-col">
          <div className="board-row">
            <BoardSlot
              showCoordinates={coordinates}
              label={`${game?.name ?? gameId} board`}
              bare={Renderer !== undefined}
              overlay={
                shownSession === null ? null : (
                  <>
                    <BoardOverlay overlay={overlayFor(shownSession)} />
                    {playing && <PlayingOverlay />}
                  </>
                )
              }
            >
              {Renderer !== undefined && shownFrame !== undefined ? (
                <Renderer
                  frame={shownFrame}
                  interactive={playable && frame !== undefined}
                  onAction={onAction}
                  showCoordinates={coordinates}
                />
              ) : undefined}
            </BoardSlot>
            {/* The bar's place, right of the board, kept so that the board is where the analysis board has it. */}
            <div className="evalbar off" aria-hidden="true" />
          </div>
          {over ? (
            <PlaybackBar
              cursor={playback.cursor}
              last={latest}
              playing={playback.playing}
              speed={playback.speed}
              onGoto={playback.goto}
              onToggle={playback.toggle}
              onSpeed={playback.setSpeed}
            />
          ) : (
            <div className="playback" data-testid="playback-slot" hidden />
          )}
        </section>

        <aside className="side side-right">
          <section className="panel table">
            <PlayerStrip
              session={shownSession}
              seat={1 - bottom}
              clocks={clocks}
              running={running}
            />
            <MoveList
              session={shownSession}
              problem={problem}
              lost={session !== null && error !== null}
              shownTurn={shownTurn}
              atLatest={atLatest}
              detail={detail}
              active={active}
              onPick={pickTurn}
            />
            {session !== null && (
              <Controls
                sessionId={session.id}
                showPlayControls={mode !== 'bots' && !over}
                takebackTo={controls.takebackTo}
                resignSeat={controls.resignSeat}
                canSave={!playing && session.turns.length > 0}
                saved={over && savedSession === session.id}
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
                onNewGame={() => openMenu(mode)}
                analysisHash={routeHash({
                  page: 'game',
                  gameId,
                  tab: 'analysis',
                  board: { kind: 'session', id: shownSession.id },
                })}
              />
            )}
            <PlayerStrip session={shownSession} seat={bottom} clocks={clocks} running={running} />
          </section>
        </aside>
      </main>
      <Toast text={active ? toast.text : null} />
      {active && dialogMode !== null && game !== null && (
        <SetupDialog
          initialMode={dialogMode}
          onModeChange={setLastMode}
          releases={game.releases}
          humanPlay={game.human_play}
          onStart={start}
          onClose={closeDialog}
          endsRunningGame={gameInProgress}
        />
      )}
    </>
  );
}
