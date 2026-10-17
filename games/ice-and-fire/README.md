# A Code of Ice and Fire

CodinGame's [A Code of Ice and Fire](https://www.codingame.com/multiplayer/bot-programming/a-code-of-ice-and-fire) is the second game of this framework, named by the owner on 2026-10-17 (phase 7 of the [main roadmap](../../docs/ROADMAP.md)). Two players on a 12×12 map build armies, buildings and territory to capture the opposing headquarters. It began as a 10-day contest, now over; bots can still be submitted to it as a multiplayer game (source's README).

**Status:** stage 5, referee and engine ([ROADMAP.md](ROADMAP.md)): [RULES.md](RULES.md), [MODEL.md](MODEL.md), a readable referee with its map generator, and the `wait` bot. No fast engine or arena yet.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [MODEL.md](MODEL.md) | The game as a search problem: numbers, a formal model, what follows for bots |
| [ROADMAP.md](ROADMAP.md) | The game's stages in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md)) |
| [referee/](referee/) | `ice-and-fire-referee`: readable reference implementation of the rules, with our own map generator ([ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md)), for the arena |
| [bots/wait/](bots/wait/) | `ice-and-fire-bot-wait`: reads every turn's input and answers `WAIT` |
| [ci.sh](ci.sh) | The game's CI steps, run after the bots are bundled ([ADR 0029](../../docs/adr/0029-ci-scoped-by-area-and-game.md)); no arena games until stage 6 |

## Sources

| Short name | What it is |
| --- | --- |
| Statement | The statement template in the game's source, `config/statement_en.html.tpl`. The owner chose it on 2026-10-17 over pasting CodinGame's page; the page may have changed since the source was written, so the `rules-check` bot of stage 5 compares it with CodinGame's real games |
| Source | The referee, [Azkellas/a-code-of-ice-and-fire](https://github.com/Azkellas/a-code-of-ice-and-fire) on its `develop` branch, commit `efe9e78`, in Java on CodinGame's game engine. File paths below are relative to `src/main/java/com/codingame/` |

**License of the source: GNU GPL version 3** (`LICENSE.md`). Under [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md), the source is read for facts only: nothing of it is translated or copied here, the statement template and its pictures included. Our rules, referee and engine are written from our own `RULES.md`.

## Facts

Each fact gives its source. [RULES.md](RULES.md) gives the full rules with their edge cases.

| Fact | Value | Source |
| --- | --- | --- |
| Players | 2 | Statement; `config/config.ini` (`max_players=2`); `antiyoy/Constants.java` (`PLAYER_COUNT`) |
| Turn format | One player after the other: red (player 0) plays a whole turn, then blue (player 1); a game turn is both | `game/Referee.java`, `computeCurrentPlayer` (lines 165-176) |
| A turn's answer | One line of commands separated by `;`: `MOVE id x y`, `TRAIN level x y`, `BUILD MINE x y` or `BUILD TOWER x y`, `WAIT`, and `MSG text` for the viewer. The commands are applied one by one in order; an invalid one is skipped | Statement, "Output" and "Order of actions"; `game/Referee.java`, `readInput` (lines 352-383) |
| Answers that lose | An unrecognised command, or no answer in time | Statement, "Lose Conditions"; `game/Referee.java` lines 368-381 |
| Hidden information | None: each turn gives both players' gold and income, the whole map, and every building and unit | Statement, "Game turn input"; `antiyoy/GameState.java`, `sendState` (lines 705-718) |
| Randomness | Only the map, generated from the game's seed; the mine spots are given once, at the start | `antiyoy/GameState.java` line 41 (`new Random(seed)`); `game/Referee.java`, `sendInitialInput` |
| Map | 12×12 cells, each void, neutral, or owned (active or not). The headquarters are at (0,0) for red and (11,11) for blue; the cells next to each headquarters, diagonals included, are never void; blue also owns (11,10) at the start | Statement, "The map" and "Expert Rules"; `antiyoy/GameState.java`, `createHQs` (lines 367-383) |
| Map symmetry | The generator makes the map symmetric | `antiyoy/GameState.java` comments at lines 227 and 299; the kind of symmetry is to be stated in stage 2 |
| Mine spots | Between 6 and 15% of the non-void cells, an even number; always one right of the red headquarters and one left of the blue headquarters. The code counts the non-void cells before the headquarters are placed, as the statement says, and places mine spots only from Wood 1 on | Statement, "Expert Rules"; `antiyoy/GameState.java` lines 316-333 |
| End of the game | A headquarters captured, a player losing by its answer, or 100 game turns. After 100 turns the higher military power wins: gold plus the training cost of the units alive | Statement, "Victory Conditions"; `antiyoy/Constants.java` (`MAX_TURNS = 100`); `game/Referee.java`, `discriminateEndGame` and `checkForHqCapture`; `antiyoy/GameState.java`, `getScores` |
| Time limits | 1,000 ms for the first turn, 50 ms for the others | Statement, "Constraints" |
| Input | At the start: the number of mine spots and their coordinates. Each turn: own gold and income, the opponent's, 12 rows of 12 characters, the buildings (owner, type, x, y) and the units (owner, id, level, x, y) | Statement, "Game Input" |
| Units | Levels 1, 2 and 3: training cost 10, 20 and 30, upkeep 1, 4 and 20 per turn | Statement, "Armies"; `antiyoy/Constants.java` |
| Buildings | Mine (15 gold, on mine spots only, +4 income) and tower (15 gold, protects its four neighbouring cells) | Statement, "Buildings"; `antiyoy/Constants.java` |

### Leagues

The rules grow with the leagues; Bronze and above play them all (statement; `game/Referee.java`, `init`, lines 66-86).

| League | Rules added |
| --- | --- |
| Wood 3 | Level 1 units only, no buildings |
| Wood 2 | Units of levels 2 and 3 |
| Wood 1 | Mines |
| Bronze | Towers: all the rules |

Leagues above Bronze, and how many players each holds, are not in the sources (open questions in [ROADMAP.md](ROADMAP.md)).

### Disagreements and points to settle

- The statement template declares five league levels; the referee knows four and treats every level from 4 up as Bronze (`game/Referee.java`, lines 66-86).
- Equal military power after 100 turns: the referee gives both players their score (`discriminateEndGame`); whether CodinGame then reports a draw is a platform fact to check in stage 5, not guessed here.
- Red always plays first, and blue's extra cell at (11,10) offsets that (statement, "Expert Rules"). Whether CodinGame plays each pairing from both seats is a platform fact to check.
- The source is the `develop` branch at commit `efe9e78` (2020-11-25); CodinGame's referee may differ. The `rules-check` bot of stage 5 settles it, and CodinGame's behaviour wins.

## Notes on the owner's criteria

The owner named these criteria on 2026-10-13. They inform; they do not grade the game he chose.

- **Interest and research value.** Unlike Ultimate Tic-Tac-Toe, a turn is a sequence of commands (moves, training, buildings) with an economy behind it, so a turn has far more possible answers than one move, and a search over single moves no longer fits as is. Territory connectivity (cells cut from the headquarters go inactive and their units die) gives tactical shots that a search can find. Stage 3 (model) gives the numbers; stage 8 (research) reads the contest's write-ups.
- **Source and license.** The referee is public but under the GPL: facts only, so the referee and engine are our own. Bots never need the map generator, since CodinGame sends the map. Our arena needs maps too, and how it makes them (its own generator from the stated facts, or another way) is a question for stage 4.
- **Framework gaps already visible** (for stage 4): several commands per turn; an end of game decided on a score after a turn limit, not only a win; an asymmetric start (red first, blue's extra cell); a random map each game.
- **How it shows in the studio.** A 12×12 grid with territory, units and buildings draws well. Human play needs a way to build a turn of several commands before sending it.
- **Depth of the leaderboard.** Not in the sources; the owner can tell, or it stays unknown.
