import type { Answer, EndReason, SeatInfo, Session, SessionResult } from './api';

/** The mark of a seat: seat 0 moves first and is X. */
export function seatMark(seat: number): 'X' | 'O' {
  return seat === 0 ? 'X' : 'O';
}

/** One line of text for what the session is doing. */
export function statusText(session: Session): string {
  const { done, total } = session.progress ?? { done: 0, total: null };
  switch (session.status) {
    case 'compiling':
      return 'Compiling the bot…';
    case 'measuring':
      return 'Measuring the bot’s speed…';
    case 'rewinding':
      return total === null ? 'Replaying the bot…' : `Replaying the bot: ${done} of ${total}`;
    case 'bot_thinking': {
      const seat = session.to_act[0];
      return seat === undefined
        ? 'A bot is thinking…'
        : `${session.seats[seat]?.name ?? 'A bot'} is thinking…`;
    }
    case 'waiting_human': {
      const seat = session.to_act[0];
      return seat === undefined
        ? 'Waiting for a move'
        : `${session.seats[seat]?.name ?? 'A player'} to move (${seatMark(seat)})`;
    }
    case 'over':
      return 'Game over';
    case 'failed':
      return session.error ? `Failed: ${session.error}` : 'Failed';
  }
}

/** The title of the info card: who plays whom. */
export function sessionTitle(session: Session): string {
  const bots = session.seats.filter((seat) => seat.kind === 'bot').length;
  return bots === 0 ? 'Friend game' : bots === 1 ? 'Against the computer' : 'Bot against bot';
}

/** The text of a turn in the move list: its answers' lines. */
export function turnText(turn: readonly Answer[]): string {
  return turn.map((answer) => answer.lines.join(' ')).join(' / ');
}

/** One move of the list: the index of its turn and its text. */
export interface MoveCell {
  turn: number;
  text: string;
}

export interface MoveRow {
  number: number;
  /** The first and the second turn of the pair; the second is null in an odd list. */
  moves: [MoveCell, MoveCell | null];
}

/** The turns as numbered pairs, as the mockup's move list shows them. */
export function moveRows(turns: readonly (readonly Answer[])[]): MoveRow[] {
  const rows: MoveRow[] = [];
  for (let i = 0; i < turns.length; i += 2) {
    const second = turns[i + 1];
    rows.push({
      number: i / 2 + 1,
      moves: [
        { turn: i, text: turnText(turns[i] ?? []) },
        second ? { turn: i + 1, text: turnText(second) } : null,
      ],
    });
  }
  return rows;
}

/**
 * Why the game ended. `detail` is the game's own text for a game that ran to its end (the
 * renderer writes it, because it knows the game); the rest comes from the end kind.
 */
function endText(
  end: EndReason,
  seats: readonly { kind: 'human' | 'bot' }[],
  detail: string | null,
): string {
  const who = (seat: number) => (seats[seat]?.kind === 'bot' ? 'bot' : 'human');
  switch (end.kind) {
    case 'finished':
      return detail ?? '';
    case 'timeout':
      return who(end.seat) === 'bot' ? 'Timeout' : 'Out of time';
    case 'crash':
      return 'Crash';
    case 'invalid':
      return 'Invalid answer';
    case 'resigned':
      return `${seatMark(end.seat)} resigned`;
    case 'aborted':
      return end.reason;
  }
}

/** The headline ("1–0 · X wins") and the reason under the move list when the game is over. */
export function resultText(
  result: SessionResult,
  seats: readonly { kind: 'human' | 'bot' }[],
  detail: string | null,
): { headline: string; detail: string } {
  const reason = endText(result.end, seats, detail);
  if (result.winner === 0) return { headline: '1–0 · X wins', detail: reason };
  if (result.winner === 1) return { headline: '0–1 · O wins', detail: reason };
  return { headline: '½–½ · Draw', detail: reason };
}

/** The seat shown at the bottom: the first human's, else seat 0. */
export function bottomSeat(session: Session | null): number {
  const human = session?.seats.findIndex((seat) => seat.kind === 'human') ?? -1;
  return human === -1 ? 0 : human;
}

/** The number of the move being played (1 for the first), or null once the game is over. */
export function moveBeingPlayed(session: Session): number | null {
  return session.status === 'over' || session.status === 'failed' ? null : session.turns.length + 1;
}

/** A notice over the board while the session rewinds, compiles or measures. */
export interface Overlay {
  title: string;
  text: string | null;
  fraction: number | null;
}

function releaseNames(session: Session, only?: (seat: SeatInfo) => boolean): string {
  const names: string[] = [];
  for (const seat of session.seats) {
    if (seat.kind === 'bot' && (only?.(seat) ?? true) && !names.includes(seat.release)) {
      names.push(seat.release);
    }
  }
  return names.join(' and ');
}

/** The overlay for a session's status, or null when the board is free. */
export function overlayFor(session: Session): Overlay | null {
  switch (session.status) {
    case 'rewinding': {
      const names = releaseNames(session);
      const total = session.progress?.total ?? null;
      if (total === null || total === 0) {
        return {
          title: `Rewinding ${names}`,
          text: `Restarting ${names} from the first move.`,
          fraction: total === 0 ? 0 : null,
        };
      }
      const done = Math.min(session.progress?.done ?? 0, total);
      return {
        title: `Rewinding ${names}`,
        text: `Replaying ${total} move${total === 1 ? '' : 's'} with seed ${session.seed}.`,
        fraction: done / total,
      };
    }
    case 'compiling':
      return {
        title: `Compiling ${releaseNames(session)}…`,
        text: null,
        fraction: null,
      };
    case 'measuring': {
      const names = releaseNames(session, (seat) => seat.kind === 'bot' && seat.mode === 'fixed');
      return {
        title: `Measuring ${names || releaseNames(session)}'s speed on this computer…`,
        text: 'Only the first game against a release needs this.',
        fraction: null,
      };
    }
    default:
      return null;
  }
}
