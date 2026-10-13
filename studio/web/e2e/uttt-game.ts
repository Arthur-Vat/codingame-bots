// A real game of Ultimate Tic-Tac-Toe as a record file, for the tests that run the analysis program
// for real (the browser engine): every move is legal, and O wins on three small boards at the end.

export const GAME_MOVES = [
  '1 0', '5 2', '7 6', '4 2', '3 7', '2 4', '6 4', '0 4', '1 5', '5 7', '8 3', '7 0', '3 1', '0 3',
  '2 0', '8 1', '6 5', '1 7', '5 3', '8 0', '6 0', '0 0', '2 2', '7 8', '4 6', '3 2', '0 8', '2 7',
  '7 4', '3 5', '0 7', '1 4', '5 4', '8 2', '6 8', '2 8', '6 7', '0 2', '1 8', '3 8', '0 6', '0 1',
]; // prettier-ignore

const player = (name: string) => ({
  name,
  kind: 'bot',
  bot_seed: 1,
  command: name,
  time_scale: 1,
  fixed_iters: null,
});

/**
 * The record text of the game's first `turns` moves: the whole game ends as the rules say; a
 * shorter one ends by the resignation of the seat to move (so the position on its last frame is
 * not over for the rules).
 */
export function gameFile(turns = GAME_MOVES.length): Buffer {
  const whole = turns >= GAME_MOVES.length;
  const loser = turns % 2;
  const record = {
    format: 1,
    game: 'uttt',
    seed: 7,
    opening_plies: 0,
    unix_time: 1_760_000_000,
    source: 'arena match',
    players: [player('uttt-v010'), player('uttt-v009')],
    turns: GAME_MOVES.slice(0, turns).map((move, index) => [
      { seat: index % 2, lines: [move], ms: 5 },
    ]),
    end: whole ? { kind: 'finished' } : { kind: 'resigned', seat: loser },
    winner: whole ? 1 : 1 - loser,
  };
  return Buffer.from(JSON.stringify(record));
}
