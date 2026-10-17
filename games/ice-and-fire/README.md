# A Code of Ice and Fire

CodinGame's [A Code of Ice and Fire](https://www.codingame.com/multiplayer/bot-programming/a-code-of-ice-and-fire) is the second game of this framework, named by the owner on 2026-10-17 (phase 7 of the [main roadmap](../../docs/ROADMAP.md)). Two players on a 12×12 map build armies, buildings and territory to capture the opposing headquarters. It began as a 10-day contest, now over; bots can still be submitted to it as a multiplayer game (source's README).

**Status:** stage 5, referee and engine ([ROADMAP.md](ROADMAP.md)): [RULES.md](RULES.md), [MODEL.md](MODEL.md), a readable referee with its map generator, a fast engine checked against it, the `wait` bot, a bot per league and the `rules-check` bot. No arena yet.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [MODEL.md](MODEL.md) | The game as a search problem: numbers, a formal model, what follows for bots |
| [ROADMAP.md](ROADMAP.md) | The game's stages in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md)) |
| [referee/](referee/) | `ice-and-fire-referee`: readable reference implementation of the rules, with our own map generator ([ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md)), for the arena |
| [engine/](engine/) | `ice-and-fire-engine`: the fast, standard-library engine for bots (bitboards, random turns, CodinGame's text protocol), with parity tests against the referee and the speed example |
| [policy/](policy/) | `ice-and-fire-policy`: the greedy turn policy and the input/output loop that the per-league bots share; its tests play every league's bot against the referee |
| [bots/](bots/) | The bots, listed in "Bots" below |
| [ci.sh](ci.sh) | The game's CI steps, run after the bots are bundled ([ADR 0029](../../docs/adr/0029-ci-scoped-by-area-and-game.md)); no arena games until stage 6 |

## Bots

Each league has its own bot, since the input never names the league; each is a small `main` that fixes its league and runs the policy of [policy/](policy/). Bundles are `target/cg/ice-and-fire-<bot>.rs`.

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [wait](bots/wait/) | Reads every turn's input and answers `WAIT` | An always-valid opponent for the arena and the CI |
| [wood3](bots/wood3/) | The greedy policy with the Wood 3 rules (level 1 only, level 1 beats level 1) | Promotion out of Wood 3 |
| [wood2](bots/wood2/) | The same policy with levels 1 to 3 | Promotion out of Wood 2 |
| [wood1](bots/wood1/) | The same policy, building mines on free mine spots | Promotion out of Wood 1 |
| [bronze](bots/bronze/) | The same policy with all the rules: a tower next to its HQ when enemy units come within 3 steps. Also serves above Bronze until real bots exist | Playing Bronze, where the `rules-check` bot runs |
| [rules-check](bots/rules-check/) | Plays random turns and checks every turn's input against RULES.md and the engine; logs the game's map | Checking our rules against CodinGame's, in Bronze or above ([below](#checking-the-rules-on-codingame)) |

The policy plans a turn on a scratch copy of the engine's state and keeps only the commands the rules accept. In order: a lethal move or chain of trainings to the enemy HQ if the gold pays for it (found by a cheapest-path search through the cells next to the territory); moves, the unit nearest the enemy HQ first, each to the neighbouring cell that gains the most (new cells, enemy cells, kills, cuts of enemy territory, mine spots) and gets nearer; a tower by the HQ under threat, else mines; trainings on the best cells next to the territory, level 1 unless a higher level is the only way to enter (to kill a unit or pass a tower), with income after upkeep kept at least 0 and a small gold reserve. It does no search and takes a few ms per turn (tested under 20 ms). It is a means of promotion, not a strong bot; none of the bosses of the leagues has been played yet.

Measured on 2026-10-17 over 200 maps in each seat (400 games per row, Rust 1.90.0 and current, release build), each bot in its own league against the reference referee (`cargo test -p ice-and-fire-policy --release`):

| League | Against `WAIT` | Against random turns (`State::random_turn`) |
| --- | --- | --- |
| Wood 3 | 400/400 | 400/400 |
| Wood 2 | 400/400 | 400/400 |
| Wood 1 | 400/400 | 400/400 |
| Bronze | 400/400 | 400/400 |

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

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. The `rules-check` bot checks that, and logs the maps CodinGame draws so that they can be set beside our generator's ([ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md), decision 3). To run it, once, in Bronze league or above (the bot assumes the Bronze rules):

1. Paste `target/cg/ice-and-fire-rules-check.rs` (`scripts/bundle-bots.sh ice-and-fire`; CI also publishes it in the `paste-ready-bots-ice-and-fire` artifact) into the IDE and click **Play my code** a few times, against different opponents.
2. Check that the MAP lines across the games show both `red` and `blue`, and say so if they all show the same seat: the rules differ between the seats (red plays first, blue starts with one more cell), and the first turn as each is checked.
3. For each game, copy from the console: the line `rules-check: MAP ...` (once, on the first turn), the last `rules-check: turn N, X checks, M differences` line (with `(first at turn K)` after a difference), every line starting with `rules-check: DIFFERENCE`, and the result on the end screen.

Each turn the bot answers a random turn and checks:

- the input alone: each player's active cells against their connection to its HQ, no unit of either player on its own inactive cell, both incomes against the income formula (the bot's own income after a wipe-out of its units is allowed to be the one from before it), mines only on mine spots and towers never, and, on the first turn, the start of the game;
- the input against what the engine predicted after the bot's own commands last turn: gold (plus the income shown, or 0 after a wipe-out), the cells, units and buildings of the bot, the ids of the opponent's units, and that each command the engine carried out shows up unless the opponent captured the cell.

It prints `rules-check: EXPECT win for <red|blue> (HQ captured)` when its own commands capture the opponent's HQ, and on turn 100 both military powers and the result it expects, to compare with the end screen (equal powers are printed as a draw, which RULES.md does not confirm). Any difference, a turn's input that does not parse included, is a `rules-check: DIFFERENCE` line with both sides, and the bot answers `WAIT` to input it cannot read. With `CG_RULES_CHECK_STRICT` set it exits at the first difference (CI only: the owner cannot set it on CodinGame).

The MAP line is `rules-check: MAP <red|blue> <144 characters, row by row> <mine spots as x,y separated by spaces>`, with `#` for a void cell and `.` for any other.

What the bot cannot check from its input: the opponent's commands, gold and upkeep (only the opponent's income is shown), the order of the opponent's actions, the time limits, and the rules of the leagues below Bronze. Ids of the bot's own new units are the bot's guess, since it cannot see the ids of units that died, so only that they come after the ids it has seen is checked. How CodinGame chooses the seats and the map seed is not in the input.

## Engine speed

`cargo +1.90.0 run --release -p ice-and-fire-engine --example speed -- [SECONDS]` plays Bronze games on 64 of the referee's maps with random turns (`State::random_turn`, then `play_turn`), then times `turn_input` and `read` on positions from move 40 of those games. CodinGame compiles with Rust 1.90.0, so measure with it (`rustup toolchain install 1.90.0 --profile minimal`). CI runs it from [`ci.sh`](ci.sh) and shows the table in the job summary; it is a report, not a check.

Measured on 2026-10-17 with Rust 1.90.0, one thread of a shared 4-core machine (Intel(R) Xeon(R) Processor @ 2.10GHz; it varies by about 10% between runs), 2 s per row:

| Engine benchmark | Result |
| --- | --- |
| Random turns generated and played | 139,582 per second |
| Random turns generated alone (from move 40 of a game) | 312,073 per second |
| Whole random games (Bronze) | 946 per second |
| Turns per random game | 147.6 |
| `turn_input` | 213,415 per second |
| `read` (a new map every time) | 7,902 per second |
| `read` (the map of the last read) | 206,329 per second |
| `State::new` (builds the distance table) | 8,750 per second |

A turn here is one player's turn, so a game of 147.6 turns is about 74 game turns. A random turn clones the state (about 1 KB and a reference count) to choose commands on a scratch copy. `read` rebuilds the 144 × 144 distance table, about 123 microseconds, only when the map differs from the last one it read on the thread; a bot reads the same map every turn.

## Notes on the owner's criteria

The owner named these criteria on 2026-10-13. They inform; they do not grade the game he chose.

- **Interest and research value.** Unlike Ultimate Tic-Tac-Toe, a turn is a sequence of commands (moves, training, buildings) with an economy behind it, so a turn has far more possible answers than one move, and a search over single moves no longer fits as is. Territory connectivity (cells cut from the headquarters go inactive and their units die) gives tactical shots that a search can find. Stage 3 (model) gives the numbers; stage 8 (research) reads the contest's write-ups.
- **Source and license.** The referee is public but under the GPL: facts only, so the referee and engine are our own. Bots never need the map generator, since CodinGame sends the map. Our arena needs maps too, and how it makes them (its own generator from the stated facts, or another way) is a question for stage 4.
- **Framework gaps already visible** (for stage 4): several commands per turn; an end of game decided on a score after a turn limit, not only a win; an asymmetric start (red first, blue's extra cell); a random map each game.
- **How it shows in the studio.** A 12×12 grid with territory, units and buildings draws well. Human play needs a way to build a turn of several commands before sending it.
- **Depth of the leaderboard.** Not in the sources; the owner can tell, or it stays unknown.
