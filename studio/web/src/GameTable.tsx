import { useEffect, useRef, useState, type ReactNode, type RefObject } from 'react';
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
  active = true,
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
  }, [shownTurn, turnCount, over, atLatest, active]);
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
  /** Whether the list is on screen: a list brought back keeps the shown move in view. */
  active?: boolean;
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
  active = true,
  onPick,
}: MoveListProps) {
  const turnCount = session?.turns.length ?? 0;
  const over = session?.result != null;
  const list = useKeepInView(shownTurn, turnCount, over, atLatest, active);

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
        <div className="empty">Press New game to start a game.</div>
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

/** How long the confirmation of a resignation waits before it takes itself back. */
export const RESIGN_CONFIRM_MS = 4000;

/**
 * The resign button, in two steps: the first click asks "Confirm resign", the second resigns. The
 * question goes away after RESIGN_CONFIRM_MS, on Escape, and when the button is not usable. The
 * caller gives it a new `key` for each session, so a new game starts with the first step.
 */
function ResignButton({ disabled, onResign }: { disabled: boolean; onResign: () => void }) {
  const [asking, setAsking] = useState(false);
  // A button that cannot be used forgets the question.
  if (disabled && asking) setAsking(false);
  const sure = asking && !disabled;
  useEffect(() => {
    if (!sure) return;
    const timer = setTimeout(() => setAsking(false), RESIGN_CONFIRM_MS);
    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setAsking(false);
    };
    window.addEventListener('keydown', onKey);
    return () => {
      clearTimeout(timer);
      window.removeEventListener('keydown', onKey);
    };
  }, [sure]);
  return (
    <button
      className={`ctrl${sure ? ' danger' : ''}`}
      type="button"
      disabled={disabled}
      onClick={() => {
        if (!sure) {
          setAsking(true);
          return;
        }
        setAsking(false);
        onResign();
      }}
    >
      {sure ? 'Confirm resign' : '⚑ Resign'}
    </button>
  );
}

interface ControlsProps {
  /** The session the buttons belong to: a resignation asked in another one is forgotten. */
  sessionId: string;
  /** Takeback, resign: shown while a game with a human is on. */
  showPlayControls: boolean;
  takebackTo: number | null;
  resignSeat: number | null;
  canSave: boolean;
  /** The finished game is in the history already. */
  saved?: boolean;
  onTakeback: () => void;
  onResign: () => void;
  onSave: () => void;
  onExport: () => void;
}

/** The row of buttons under the move list. */
export function Controls({
  sessionId,
  showPlayControls,
  takebackTo,
  resignSeat,
  canSave,
  saved = false,
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
          <ResignButton key={sessionId} disabled={resignSeat === null} onResign={onResign} />
        </>
      )}
      <button className="ctrl" type="button" disabled={!canSave || saved} onClick={onSave}>
        {saved ? 'In your history' : 'Save'}
      </button>
      <button className="ctrl" type="button" disabled={!canSave} onClick={onExport}>
        Export file
      </button>
    </div>
  );
}

/**
 * What to do after a game: play again with the same settings, choose new ones, and, when
 * `analysisHash` is given, open the game on the analysis board.
 */
export function EndBlock({
  onRematch,
  onNewGame,
  analysisHash,
  disabled,
  testId = 'end-block',
}: {
  onRematch: () => void;
  onNewGame: () => void;
  analysisHash?: string;
  disabled: boolean;
  testId?: string;
}) {
  return (
    <div className="after" data-testid={testId}>
      <button className="btn" type="button" disabled={disabled} onClick={onRematch}>
        Rematch
      </button>
      <button className="btn" type="button" disabled={disabled} onClick={onNewGame}>
        New game
      </button>
      {analysisHash !== undefined && (
        <a className="btn wide" href={analysisHash}>
          Analysis board
        </a>
      )}
    </div>
  );
}
