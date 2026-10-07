# Ultimate Tic-Tac-Toe rules on CodinGame

These are the rules our referee implements, written in our own words. They come from the game's statement and from its source code, which the statement links to: [dreignier/game-ultimate-tictactoe](https://github.com/dreignier/game-ultimate-tictactoe) (CodinGame SDK, 2018). That repository has no license, so it is read for facts only; no code is copied from it.

**Checked on CodinGame:** on 2026-10-07, in Bronze league, the `rules-check` bot found CodinGame's valid actions identical to our engine's on every turn it checked. In one game, the engine predicted that the bot's move ended the game with a win, 4 small boards to 3; CodinGame ended the game on that move and declared the same winner. `rules-check` prints the expected result whenever its own move ends a game, so more games can be checked the same way.

## Board and players

- The board has 9×9 cells. Rows and columns are numbered 0 to 8.
- Cell (row, col) lies in the small board (row / 3, col / 3), at position (row mod 3, col mod 3) inside it.
- Two players alternate. The player in seat 0 moves first.

## Valid actions

- On the first move of the game, every cell is valid.
- After a move in cell (r, c), the next player is sent to the small board (r mod 3, c mod 3). If that small board is still open, the valid actions are its empty cells.
- If that small board is closed, the valid actions are the empty cells of every open small board.
- A small board is open while it is neither won nor full. The empty cells of a won small board can never be played.
- CodinGame shuffles the list of valid actions every turn.

## Small boards

A player who has three marks in a row, column or diagonal of a small board wins it. A full small board without such a line belongs to nobody. Each small board won is worth one point.

## End of the game

- A player who wins three small boards forming a row, column or diagonal of the main board wins at once.
- When no valid action is left, the player with more points (small boards won) wins. Equal points is a draw.
- A player whose answer is not two integers forming a valid action loses at once. So does a player who exceeds the time limit or stops responding.

## Protocol

Each turn, the player to move receives:

1. `opponentRow opponentCol`: the opponent's last action, or `-1 -1` on the very first turn of the game;
2. `validActionCount`;
3. `validActionCount` lines of `row col`.

It answers with one line: `row col`. CodinGame splits the answer on single spaces and reads the first two parts as integers; anything after them is ignored. Bots should print exactly `row col`.

## Time limits

1,000 ms for a player's first answer, then 100 ms per answer, as stated in the game's statement. CodinGame measures from when it starts sending the turn's input until it reads the answer.

The SDK source uses 10 s and 1 s instead; CodinGame staff have said that the SDK's timing code differs from production, so the statement's values are the ones that apply.

## Wood league

The lowest league is plain tic-tac-toe on a single 3×3 board: coordinates 0 to 2, the same protocol, at most 9 moves. Three in a line wins; a full board without a line is a draw. The Ultimate rules above start in the next league.

In the game's source, the boss of this league picks a uniformly random valid action every turn (`config/Boss.java`). Our referee implements only the Ultimate rules; the `wood` bot is meant for this league.
