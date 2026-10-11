import type { Answer, EndReason, Session, SessionResult } from './api';

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

export interface MoveRow {
  number: number;
  /** The first and the second turn of the pair; the second is null in an odd list. */
  moves: [string, string | null];
}

/** The turns as numbered pairs, as the mockup's move list shows them. */
export function moveRows(turns: readonly (readonly Answer[])[]): MoveRow[] {
  const rows: MoveRow[] = [];
  for (let i = 0; i < turns.length; i += 2) {
    const second = turns[i + 1];
    rows.push({
      number: i / 2 + 1,
      moves: [turnText(turns[i] ?? []), second ? turnText(second) : null],
    });
  }
  return rows;
}

function endText(end: EndReason): string {
  switch (end.kind) {
    case 'finished':
      return 'By the rules';
    case 'timeout':
      return 'Out of time';
    case 'crash':
      return 'The bot crashed';
    case 'invalid':
      return 'Invalid answer';
    case 'resigned':
      return 'Resigned';
    case 'aborted':
      return end.reason;
  }
}

/** The headline and the detail under the move list when the game is over. */
export function resultText(result: SessionResult): { headline: string; detail: string } {
  const detail = endText(result.end);
  if (result.winner === 0) return { headline: '1–0 · X wins', detail };
  if (result.winner === 1) return { headline: '0–1 · O wins', detail };
  return { headline: '½–½ · Draw', detail };
}

/** The seat shown at the bottom: the first human's, else seat 0. */
export function bottomSeat(session: Session | null): number {
  const human = session?.seats.findIndex((seat) => seat.kind === 'human') ?? -1;
  return human === -1 ? 0 : human;
}
