/* What playing needs, as pure functions of a session: which `human_moves` entry a clicked action
   is, what a takeback keeps, who resigns, and what a rematch asks for. They hold no rules of the
   game: the server decides what is legal. */

import type { SeatInfo, SeatRequest, Session, SessionRequest } from './api';
import type { Mode } from './setup';

/** Structural equality of two JSON values. */
export function deepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== 'object' || typeof b !== 'object' || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  const left = a as Record<string, unknown>;
  const right = b as Record<string, unknown>;
  const keys = Object.keys(left);
  if (keys.length !== Object.keys(right).length) return false;
  return keys.every((key) => Object.hasOwn(right, key) && deepEqual(left[key], right[key]));
}

/** The index in `humanMoves` of the entry whose `action` equals `action`, or -1. */
export function findMoveIndex(humanMoves: readonly unknown[], action: unknown): number {
  return humanMoves.findIndex(
    (move) =>
      typeof move === 'object' &&
      move !== null &&
      'action' in move &&
      deepEqual((move as { action: unknown }).action, action),
  );
}

/** The seat that gave each turn's first answer (null for a turn without answers). */
export function turnSeats(turns: Session['turns']): (number | null)[] {
  return turns.map((turn) => turn[0]?.seat ?? null);
}

/** How a session was set up: two humans, one human against a release, or two releases. */
export function sessionMode(session: Session): Mode {
  const bots = session.seats.filter((seat) => seat.kind === 'bot').length;
  return bots === 0 ? 'friend' : bots === 1 ? 'computer' : 'bots';
}

/** The seat of the only human of a game against the computer, else null. */
export function humanSeatVsComputer(session: Session): number | null {
  if (sessionMode(session) !== 'computer') return null;
  const seat = session.seats.findIndex((candidate) => candidate.kind === 'human');
  return seat === -1 ? null : seat;
}

/**
 * The number of turns a takeback keeps, or null when there is nothing to take back or the
 * game has no takeback: a friend game loses its last turn; a game against the computer goes back
 * to before the human's last move, which removes the bot's reply too.
 */
export function takebackKeep(session: Session): number | null {
  const mode = sessionMode(session);
  if (mode === 'bots') return null;
  // The server refuses a takeback against a bot that plays in real time.
  if (session.seats.some((seat) => seat.kind === 'bot' && seat.mode === 'realtime')) return null;
  let keep: number;
  if (mode === 'friend') {
    keep = session.turns.length - 1;
  } else {
    const human = humanSeatVsComputer(session);
    let last = -1;
    session.turns.forEach((turn, index) => {
      if (turn.some((answer) => answer.seat === human)) last = index;
    });
    if (last === -1) return null;
    keep = last;
  }
  return keep >= session.opening_turns ? keep : null;
}

/** What the controls row may do now. */
export interface Controls {
  /** The `turns` to send to `/takeback`, or null when it is disabled. */
  takebackTo: number | null;
  /** The seat that would resign, or null when resigning is disabled. */
  resignSeat: number | null;
}

/** Takeback and resign are for a game in progress with no request in flight. */
export function controlsFor(session: Session | null, pending: boolean): Controls {
  if (session === null || pending) return { takebackTo: null, resignSeat: null };
  const mode = sessionMode(session);
  const takebackTo = session.status === 'waiting_human' ? takebackKeep(session) : null;
  let resignSeat: number | null = null;
  if (mode === 'friend' && session.status === 'waiting_human') {
    resignSeat = session.to_act[0] ?? null;
  } else if (
    mode === 'computer' &&
    (session.status === 'waiting_human' || session.status === 'bot_thinking')
  ) {
    resignSeat = humanSeatVsComputer(session);
  }
  return { takebackTo, resignSeat };
}

/**
 * The seat whose clock runs: the human to move, while the server waits for them. A request of
 * ours in flight (a move on its way) does not stop it: the time is the player's until the server
 * has the move. Null otherwise (a bot's turn, rewinding, compiling, measuring, a finished game).
 */
export function runningSeat(session: Session | null): number | null {
  if (session === null || session.status !== 'waiting_human') return null;
  const seat = session.to_act[0];
  return seat !== undefined && session.seats[seat]?.kind === 'human' ? seat : null;
}

/**
 * The seat whose timeout the front end must report now, or null. A clock ran out (`expired`),
 * the game is on and the server is not busy rewinding or starting, no request of ours is in
 * flight, and a failed report has not just been made (`backingOff`). The caller reports again
 * whenever this holds, so a timeout is never lost to a failed or busy moment.
 */
export function timeoutDue(
  session: Session | null,
  expired: number | null,
  pending: boolean,
  backingOff: boolean,
): number | null {
  if (session === null || expired === null || pending || backingOff) return null;
  return session.status === 'waiting_human' || session.status === 'bot_thinking' ? expired : null;
}

/** Whether a key press goes to a field that takes text or a choice, not to the board. */
export function isTextEntry(target: {
  tagName: string;
  type?: string;
  isContentEditable?: boolean;
}): boolean {
  if (target.isContentEditable) return true;
  const tag = target.tagName.toUpperCase();
  if (tag === 'SELECT' || tag === 'TEXTAREA') return true;
  if (tag !== 'INPUT') return false;
  // Checkboxes, switches and buttons are not text entry: the arrows still look back.
  return !['checkbox', 'radio', 'button', 'submit', 'reset', 'file', 'image', 'color'].includes(
    (target.type ?? 'text').toLowerCase(),
  );
}

/** Whether a click on the board may play: the latest frame, a human to move, nothing in flight. */
export function canPlay(
  session: Session | null,
  pending: boolean,
  onLatestFrame: boolean,
): boolean {
  return (
    onLatestFrame &&
    !pending &&
    runningSeat(session) !== null &&
    session !== null &&
    session.human_moves.length > 0
  );
}

function seatRequest(seat: SeatInfo): SeatRequest {
  return seat.kind === 'human'
    ? { kind: 'human', name: seat.name }
    : { kind: 'bot', release: seat.release, think_ms: seat.think_ms, mode: seat.mode };
}

/** The request of a rematch: the same settings, and against the computer the sides swapped. */
export function rematchRequest(session: Session, seed: number): SessionRequest {
  const [first, second] = session.seats.map(seatRequest) as [SeatRequest, SeatRequest];
  const swap = sessionMode(session) === 'computer';
  return {
    game: session.game,
    seed,
    opening_plies: session.opening_plies,
    seats: swap ? [second, first] : [first, second],
  };
}

/** `uttt-2026-10-10-123456.json`: the game, the UTC day, the seed. */
export function exportFileName(game: string, seed: number, now: Date): string {
  return `${game}-${now.toISOString().slice(0, 10)}-${seed}.json`;
}

/** The frame that shows the position after turn `turn` (0-based). */
export function frameAfterTurn(turn: number, openingTurns: number): number {
  return Math.max(0, turn + 1 - openingTurns);
}

/** The turn (0-based) a frame ends with, or -1 for the position before the first listed turn. */
export function turnOfFrame(frame: number, openingTurns: number): number {
  return frame <= 0 ? -1 : openingTurns + frame - 1;
}

/**
 * Where a key moves the shown frame. `shown` is the frame on the board, `latest` the last one.
 * Gives the frame to pin, `null` to follow the latest frame, or `undefined` for a key that does
 * nothing here. Looking back is Lichess-style: ← and → step, Home and End jump.
 */
export function cursorAfterKey(
  key: string,
  shown: number,
  latest: number,
): number | null | undefined {
  const pin = (frame: number) => (frame >= latest ? null : Math.max(0, frame));
  switch (key) {
    case 'ArrowLeft':
      return pin(shown - 1);
    case 'ArrowRight':
      return pin(shown + 1);
    case 'Home':
      return pin(0);
    case 'End':
      return null;
    default:
      return undefined;
  }
}

/** The time a seat has used: the sum of the `ms` of its answers. */
export function timeUsed(game: Pick<Session, 'turns'>, seat: number): number {
  let total = 0;
  for (const turn of game.turns) {
    for (const answer of turn) {
      if (answer.seat === seat) total += answer.ms;
    }
  }
  return total;
}
