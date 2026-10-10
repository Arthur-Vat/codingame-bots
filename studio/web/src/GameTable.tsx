import { useEffect, useRef, type ReactNode, type RefObject } from 'react';
import type { Session } from './api';
import { clockClass, formatClock, formatSpent, type ClockState } from './clock';
import { SeatMark } from './Marks';
import { timeUsed } from './play';
import { moveRows, resultText, seatMark, statusText } from './sessionView';
import { thinkLabel } from './setup';

/** One player's strip: name, mark, and the clock (humans) or the time used (bots). */
export function PlayerStrip({
  session,
  seat,
  clocks,
  running,
}: {
  session: Session | null;
  seat: number;
  clocks: ClockState | null;
  /** The human seat whose clock runs. */
  running: number | null;
}) {
  const info = session?.seats[seat];
  const name = info?.name ?? `Player ${seat + 1}`;
  const sub = info ? (info.kind === 'bot' ? thinkLabel(info.think_ms) : seatMark(seat)) : '';
  const left = clocks?.remaining[seat] ?? null;
  let clock;
  if (session !== null && info?.kind === 'bot') {
    const thinking = session.status === 'bot_thinking' && session.to_act.includes(seat);
    clock = (
      <span
        className={`clock spent${thinking ? ' run' : ''}`}
        title="Time the bot has used"
        data-testid={`clock-${seat}`}
      >
        {formatSpent(timeUsed(session, seat))}
      </span>
    );
  } else if (left !== null) {
    clock = (
      <span
        className={`clock ${clockClass(left, running === seat)}`.trim()}
        data-testid={`clock-${seat}`}
      >
        {formatClock(left)}
      </span>
    );
  } else {
    clock = (
      <span className="clock" data-testid={`clock-${seat}`}>
        –
      </span>
    );
  }
  return (
    <div className="player" data-testid={`player-${seat}`}>
      <div className="who">
        <SeatMark seat={seat} className="chip" />
        <b>{name}</b>
        <small>{sub}</small>
      </div>
      {clock}
    </div>
  );
}

/**
 * Keeps the shown move of a move list in view; at the latest position of a finished game, shows
 * the end block under the list.
 */
export function useKeepInView(
  shownTurn: number,
  turnCount: number,
  over: boolean,
  atLatest: boolean,
): RefObject<HTMLDivElement | null> {
  const list = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const element = list.current;
    if (element === null) return;
    if (over && atLatest) {
      element.scrollTop = element.scrollHeight;
      return;
    }
    const current = element.querySelector<HTMLElement>('button.cur');
    if (current === null) return;
    if (current.offsetTop < element.scrollTop) {
      element.scrollTop = current.offsetTop;
    } else if (
      current.offsetTop + current.offsetHeight >
      element.scrollTop + element.clientHeight
    ) {
      element.scrollTop = current.offsetTop + current.offsetHeight - element.clientHeight;
    }
  }, [shownTurn, turnCount, over, atLatest]);
  return list;
}

/** One line of an info card: a term and its value. */
export function Row({ term, value }: { term: string; value: string }) {
  return (
    <>
      <dt>{term}</dt>
      <dd>{value}</dd>
    </>
  );
}

interface MoveListProps {
  session: Session | null;
  problem: string | null;
  lost: boolean;
  /** The turn the board shows the position after, or -1. */
  shownTurn: number;
  atLatest: boolean;
  /** The game's words for how a finished game ended, if it has any. */
  detail: string | null;
  onPick: (turn: number) => void;
}

/** The numbered moves, each one a button that shows its position, and the end block. */
export function MoveList({
  session,
  problem,
  lost,
  shownTurn,
  atLatest,
  detail,
  onPick,
}: MoveListProps) {
  const turnCount = session?.turns.length ?? 0;
  const over = session?.result != null;
  const list = useKeepInView(shownTurn, turnCount, over, atLatest);

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
  const result = session.result ? resultText(session.result, session.seats, detail) : null;
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
      {rows.length === 0 && session.status !== 'failed' && (
        <div className="empty">{statusText(session)}</div>
      )}
      {rows.map((row) => (
        <MoveRowView key={row.number} number={row.number}>
          {cell(row.moves[0])}
          {row.moves[1] === null ? <span /> : cell(row.moves[1])}
        </MoveRowView>
      ))}
      {session.status === 'failed' && (
        <div className="failed" role="alert">
          {session.error ?? 'The game failed.'}
        </div>
      )}
      {result && (
        <div className="result" data-testid="result">
          <b>{result.headline}</b>
          {result.detail !== '' && <span>{result.detail}</span>}
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

function MoveRowView({ number, children }: { number: number; children: ReactNode }) {
  return (
    <>
      <span className="n">{number}</span>
      {children}
    </>
  );
}

interface ControlsProps {
  /** Takeback, resign: shown while a game with a human is on. */
  showPlayControls: boolean;
  takebackTo: number | null;
  resignSeat: number | null;
  canSave: boolean;
  onTakeback: () => void;
  onResign: () => void;
  onSave: () => void;
  onExport: () => void;
}

/** The row of buttons under the move list. */
export function Controls({
  showPlayControls,
  takebackTo,
  resignSeat,
  canSave,
  onTakeback,
  onResign,
  onSave,
  onExport,
}: ControlsProps) {
  return (
    <div className="controls">
      {showPlayControls && (
        <>
          <button
            className="ctrl"
            type="button"
            disabled={takebackTo === null}
            onClick={onTakeback}
          >
            ↶ Takeback
          </button>
          <button className="ctrl" type="button" disabled={resignSeat === null} onClick={onResign}>
            ⚑ Resign
          </button>
        </>
      )}
      <button className="ctrl" type="button" disabled={!canSave} onClick={onSave}>
        Save
      </button>
      <button className="ctrl" type="button" disabled={!canSave} onClick={onExport}>
        Export file
      </button>
    </div>
  );
}

/** What to do after a game: play again with the same settings, or choose new ones. */
export function EndBlock({
  onRematch,
  onNewGame,
  onReview,
  disabled,
}: {
  onRematch: () => void;
  onNewGame: () => void;
  onReview: () => void;
  disabled: boolean;
}) {
  return (
    <div className="after" data-testid="end-block">
      <button className="btn" type="button" disabled={disabled} onClick={onRematch}>
        Rematch
      </button>
      <button className="btn" type="button" disabled={disabled} onClick={onNewGame}>
        New game
      </button>
      <button className="btn primary wide" type="button" onClick={onReview}>
        Review
      </button>
    </div>
  );
}
