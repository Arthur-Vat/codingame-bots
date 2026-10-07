# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)).

**Status:** Phase 0. Only a hello-world bot exists. `RULES.md`, the engine and the referee come in Phase 2.

## Protocol

Each turn the bot reads:

1. `opponentRow opponentCol`: the opponent's last move, or `-1 -1` when the bot moves first;
2. `validActionCount`;
3. `validActionCount` lines of `row col`.

It answers with one line, `row col`. Rows and columns are numbered 0 to 8 across the whole 9×9 board.

## Rules and limits known so far

These come from public sources and are checked against real CodinGame games in Phase 2, before `RULES.md` is written.

| Topic | What we know | Source |
| --- | --- | --- |
| Time limits | 1,000 ms on the first turn, 100 ms on later turns | [CodinGame staff on the forum](https://forum.codingame.com/t/timeouts-how-do-they-work/24504/4) |
| Where to play | A move in cell (r, c) of a small board sends the opponent to the small board at position (r mod 3, c mod 3). If that board is won or full, the opponent may play in any empty cell. | [Forum](https://forum.codingame.com/t/ultimate-tic-tac-toe-not-getting-back-all-valid-actions/23080) |
| Winning | Three small boards in a line wins. If the game ends without that, the player who won more small boards wins. Equal counts are presumably a draw (to verify). | [Puzzle discussion](https://forum.codingame.com/t/ultimate-tic-tac-toe-puzzle-discussion/22616?page=2) |
| Code size | 100 kB, strict | Same thread |
| Lowest league | Believed to be plain 3×3 tic-tac-toe with the same protocol (to verify) | Memory, unverified |

The tiebreak differs from common Ultimate Tic-Tac-Toe rules, so it gets dedicated tests.

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed | Proves the paste-to-CodinGame path (Phase 0 gate) |

## Putting a bot on CodinGame

1. Open the bot's single file. Until Phase 1 adds the bundler, that is `bots/<name>/src/main.rs`.
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
