// Fixtures shared by the browser tests of the review and history screens. The API is mocked with
// page.route: these are what the server would send.

import type { Page } from '@playwright/test';

export const GAMES = [
  { id: 'uttt', name: 'Ultimate Tic-Tac-Toe', releases: ['uttt-v010', 'uttt-v009', 'uttt-v001'] },
];

export async function mockGames(page: Page): Promise<void> {
  await page.route('**/api/games', (route) => route.fulfill({ json: GAMES }));
}

type Mark = 0 | 1 | null;

/** A uttt frame with the given marks `[row, col, seat]`. */
export function frame(
  marks: [number, number, 0 | 1][] = [],
  winner?: 0 | 1 | null,
  points: [number, number] = [0, 0],
) {
  const cells: Mark[][] = Array.from({ length: 9 }, () => Array<Mark>(9).fill(null));
  for (const [row, col, seat] of marks) cells[row]![col] = seat;
  return {
    cells,
    small: Array.from({ length: 3 }, () => Array<Mark>(3).fill(null)),
    last: marks.length === 0 ? null : [marks[marks.length - 1]![0], marks[marks.length - 1]![1]],
    playable: [],
    to_move: winner === undefined ? marks.length % 2 : null,
    points,
    result: winner === undefined ? null : { winner },
  };
}

/** Moves of a short game, X first. */
export const MOVES: [number, number][] = [
  [4, 4],
  [4, 5],
  [5, 3],
  [3, 7],
];

/** The frames of the short game: the empty board, then one more mark per move. */
export function gameFrames(count = MOVES.length) {
  const frames = [frame()];
  for (let i = 1; i <= count; i++) {
    const marks = MOVES.slice(0, i).map(([r, c], seat): [number, number, 0 | 1] => [
      r,
      c,
      (seat % 2) as 0 | 1,
    ]);
    const done = i === MOVES.length;
    frames.push(frame(marks, done ? 1 : undefined, done ? [1, 2] : [0, 0]));
  }
  return frames;
}

export const answer = (seat: number, line: string, ms = 7) => [{ seat, lines: [line], ms }];

export function gameTurns(count = MOVES.length) {
  return MOVES.slice(0, count).map(([r, c], i) => answer(i % 2, `${r} ${c}`));
}

export const botPlayer = (name: string, extra: Record<string, unknown> = {}) => ({
  name,
  kind: 'bot',
  bot_seed: 1,
  command: name,
  time_scale: 1,
  fixed_iters: null,
  ...extra,
});

/** A record as the server stores it. */
export function record(patch: Record<string, unknown> = {}) {
  return {
    format: 1,
    game: 'uttt',
    seed: 99,
    opening_plies: 0,
    unix_time: 1_760_000_000,
    source: 'arena match',
    players: [botPlayer('uttt-v010'), botPlayer('uttt-v009', { time_scale: 2 })],
    turns: gameTurns(),
    end: { kind: 'finished' },
    winner: 1,
    ...patch,
  };
}

/** What `GET /api/history/{id}/view` and `POST /api/view` answer. */
export function recordView(patch: Record<string, unknown> = {}, shown = MOVES.length) {
  return {
    record: record(patch),
    frames: gameFrames(shown),
    opening_turns: 0,
    shown_turns: shown,
  };
}
