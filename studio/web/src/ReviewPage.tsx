import { useState, type ReactNode } from 'react';
import { getHistoryRecord, getRecord, postHistory, postHistoryText, saveSession } from './api';
import { arenaRecordText } from './arena';
import { BoardSlot } from './BoardSlot';
import { download, downloadText, errorMessage } from './browser';
import { emptyFrameFor, renderers, resultDetailFor } from './games/registry';
import { Row, useKeepInView } from './GameTable';
import { SeatMark } from './Marks';
import { recallFile } from './loadedFiles';
import { frameAfterTurn, timeUsed, turnOfFrame } from './play';
import { PlaybackBar } from './PlaybackBar';
import { exportName, formatDate, lastFrame, type ReviewModel, type ReviewPlayer } from './review';
import { routeHash, type ReviewKind } from './route';
import { formatSpent } from './clock';
import { moveRows, resultText, seatMark } from './sessionView';
import { Toast, useToast } from './Toast';
import { Redirect, StatusPanel } from './Panels';
import { usePlayback } from './usePlayback';
import { useReview } from './useReview';

/** One player's strip: mark, name, how it played, and the time its answers took. */
function ReviewStrip({ model, seat }: { model: ReviewModel; seat: number }) {
  const player: ReviewPlayer | undefined = model.players[seat];
  const spent = timeUsed(model, seat);
  return (
    <div className="player" data-testid={`player-${seat}`}>
      <div className="who">
        <SeatMark seat={seat} className="chip" />
        <b>{player?.name ?? `Player ${seat + 1}`}</b>
        <small>{seatMark(seat)}</small>
      </div>
      {player?.kind === 'bot' && (
        <span className="clock spent" title="Time the bot used">
          {formatSpent(spent)}
        </span>
      )}
    </div>
  );
}

function playerLine(player: ReviewPlayer): string {
  return player.settings === null ? player.name : `${player.name}, ${player.settings}`;
}

function InfoCard({ model, resultLine }: { model: ReviewModel; resultLine: string | null }) {
  return (
    <section className="panel info">
      <h3>Review</h3>
      <dl>
        <Row term="Source" value={model.source} />
        {model.unixTime !== null && <Row term="Date" value={formatDate(model.unixTime)} />}
        <Row term="Seed" value={String(model.seed)} />
        <Row term="X" value={playerLine(model.players[0])} />
        <Row term="O" value={playerLine(model.players[1])} />
        {resultLine !== null && <Row term="Result" value={resultLine} />}
      </dl>
    </section>
  );
}

function ReviewMoves({
  model,
  shownTurn,
  atLatest,
  headline,
  detail,
  onPick,
}: {
  model: ReviewModel;
  shownTurn: number;
  atLatest: boolean;
  headline: string | null;
  detail: string;
  onPick: (turn: number) => void;
}) {
  const list = useKeepInView(shownTurn, model.turns.length, model.result !== null, atLatest);
  const cell = (move: { turn: number; text: string }) => (
    <button
      type="button"
      className={move.turn === shownTurn ? 'cur' : undefined}
      aria-current={move.turn === shownTurn ? 'true' : undefined}
      onClick={() => onPick(move.turn)}
    >
      {move.text}
    </button>
  );
  return (
    <div className="moves" aria-label="Moves" ref={list}>
      {moveRows(model.turns).map((row) => (
        <MoveRow key={row.number} number={row.number}>
          {cell(row.moves[0])}
          {row.moves[1] === null ? <span /> : cell(row.moves[1])}
        </MoveRow>
      ))}
      {model.error !== null && (
        <div className="failed" role="alert">
          {model.error}
        </div>
      )}
      {headline !== null && (
        <div className="result" data-testid="result">
          <b>{headline}</b>
          {detail !== '' && <span>{detail}</span>}
        </div>
      )}
    </div>
  );
}

function MoveRow({ number, children }: { number: number; children: ReactNode }) {
  return (
    <>
      <span className="n">{number}</span>
      {children}
    </>
  );
}

/** The record to save or export, whatever the game came from but an arena game (kept as text). */
async function recordOf(model: ReviewModel): Promise<unknown> {
  if (model.kind === 'session') return getRecord(model.id);
  if (model.kind === 'saved') return getHistoryRecord(model.id);
  if (model.kind === 'arena') throw new Error('An arena game is kept as text.');
  const file = recallFile(model.id);
  if (file === null) throw new Error('This file is no longer loaded.');
  return file.record;
}

function ReviewScreen({ model }: { model: ReviewModel }) {
  const toast = useToast();
  const [coordinates, setCoordinates] = useState(false);
  const [savedHere, setSavedHere] = useState(false);
  const last = lastFrame(model);
  // A review opened here starts at the first frame.
  const playback = usePlayback(last, 0, `${model.kind}/${model.id}`, { active: true, keys: true });
  const cursor = playback.cursor;
  const goto = playback.goto;
  const toggle = playback.toggle;

  const Renderer = renderers[model.game];
  const frame = model.frames[cursor] ?? emptyFrameFor(model.game);
  const finalFrame = model.frames[last];
  const detail =
    model.result?.end.kind === 'finished' && finalFrame !== undefined
      ? resultDetailFor(model.game, finalFrame)
      : null;
  const text = model.result === null ? null : resultText(model.result, model.players, detail);
  const endDetail =
    text === null ? '' : model.hiddenTurns > 0 ? `${text.detail}, not shown` : text.detail;
  const resultLine = text === null ? null : `${text.headline}${endDetail ? `, ${endDetail}` : ''}`;

  const shownTurn = turnOfFrame(cursor, model.openingTurns);
  const saved = model.saved || savedHere;

  const save = async () => {
    try {
      // An arena game is saved as the text it was published in.
      const { duplicate } =
        model.kind === 'session'
          ? await saveSession(model.id)
          : model.kind === 'arena'
            ? await postHistoryText(await arenaRecordText(model.id))
            : await postHistory(await recordOf(model));
      setSavedHere(true);
      toast.show(duplicate ? 'Already in your history' : 'Saved to your history');
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const exportFile = async () => {
    try {
      if (model.kind === 'arena') {
        downloadText(exportName(model, new Date()), await arenaRecordText(model.id));
      } else {
        download(exportName(model, new Date()), await recordOf(model));
      }
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };

  return (
    <>
      <main className="wrap game">
        <aside className="side side-left">
          <InfoCard model={model} resultLine={resultLine} />
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
            <p className="hint">Keys: ← → step, Home/End jump, Space plays.</p>
          </section>
        </aside>

        <section className="board-col">
          <div className="board-row">
            <div className="evalbar off" title="Win / draw / loss" aria-hidden="true" />
            <BoardSlot
              showCoordinates={coordinates}
              label={`${model.game} board`}
              bare={Renderer !== undefined}
            >
              {Renderer !== undefined && frame !== undefined ? (
                <Renderer frame={frame} interactive={false} showCoordinates={coordinates} />
              ) : undefined}
            </BoardSlot>
          </div>
          <PlaybackBar
            cursor={cursor}
            last={last}
            playing={playback.playing}
            speed={playback.speed}
            onGoto={goto}
            onToggle={toggle}
            onSpeed={playback.setSpeed}
          />
        </section>

        <aside className="side side-right">
          <section className="panel table">
            <ReviewStrip model={model} seat={1} />
            <ReviewMoves
              model={model}
              shownTurn={shownTurn}
              atLatest={cursor >= last}
              headline={text?.headline ?? null}
              detail={endDetail}
              onPick={(turn) => goto(frameAfterTurn(turn, model.openingTurns))}
            />
            <div className="controls">
              <button
                className="ctrl"
                type="button"
                disabled={saved || model.turns.length === 0}
                onClick={() => void save()}
              >
                {saved ? 'In your history' : 'Save to history'}
              </button>
              <button
                className="ctrl"
                type="button"
                disabled={model.turns.length === 0}
                onClick={() => void exportFile()}
              >
                Export file
              </button>
            </div>
            <ReviewStrip model={model} seat={0} />
          </section>
          <div className="analysis-slot" data-testid="analysis-slot" />
        </aside>
      </main>
      <Toast text={toast.text} />
    </>
  );
}

/** The game a review route names, loaded; its page when the game is loaded. */
function ReviewLoader({ gameId, kind, id }: { gameId: string; kind: ReviewKind; id: string }) {
  const { model, error } = useReview(kind, id);
  if (model === null) {
    return error === null ? (
      <StatusPanel text="Loading the game…" />
    ) : (
      <StatusPanel text={error} alert />
    );
  }
  // A hand-typed address with another game's id: the review belongs to its own game's page.
  if (model.game !== gameId) {
    return (
      <Redirect
        hash={routeHash({
          page: 'game',
          gameId: model.game,
          tab: 'review',
          review: { kind, id },
        })}
      />
    );
  }
  // A new game starts from its first frame, with its own playback.
  return <ReviewScreen key={`${model.kind}/${model.id}`} model={model} />;
}

/** The Review tab of a game: the game open for review, or a way to open one. */
export function ReviewTab({
  gameId,
  review,
}: {
  gameId: string;
  review: { kind: ReviewKind; id: string } | null;
}) {
  if (review === null) {
    return (
      <main className="wrap">
        <section className="panel empty-page" data-testid="review-empty">
          <p>No game open for review.</p>
          <p>
            <a href={routeHash({ page: 'game', gameId, tab: 'history', set: 'mine' })}>
              Open one from the history
            </a>
          </p>
        </section>
      </main>
    );
  }
  // A session is reviewed in place, in the Play tab.
  if (review.kind === 'session') {
    return (
      <Redirect hash={routeHash({ page: 'game', gameId, tab: 'play', sessionId: review.id })} />
    );
  }
  return <ReviewLoader gameId={gameId} kind={review.kind} id={review.id} />;
}

/**
 * An address of the time before the tabs, `#/review/<kind>/<id>`: found once the game is loaded,
 * then replaced with the address of its own game's page.
 */
export function OldReviewRedirect({ kind, id }: { kind: ReviewKind; id: string }) {
  const { model, error } = useReview(kind, id);
  if (model === null) {
    return error === null ? (
      <StatusPanel text="Loading the game…" />
    ) : (
      <StatusPanel text={error} alert />
    );
  }
  const hash =
    kind === 'session'
      ? routeHash({ page: 'game', gameId: model.game, tab: 'play', sessionId: id })
      : routeHash({ page: 'game', gameId: model.game, tab: 'review', review: { kind, id } });
  return <Redirect hash={hash} text="Loading the game…" />;
}
