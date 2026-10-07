# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)). The rules, protocol and time limits are in [RULES.md](RULES.md).

**Status:** Phase 4. The reference referee, the fast engine (checked against it), the arena with its evaluation tools, five utility bots and the first real bot, MCTS, exist. No release yet.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [referee/](referee/) | Readable reference implementation of the rules, driven by the arena |
| [engine/](engine/) | Fast implementation of the rules for bots: bitboards, no allocation, random playouts |
| [arena/](arena/) | `uttt-arena`: plays bots against each other through the referee |
| [bots/](bots/) | One crate per bot |
| [evaluation.env](evaluation.env) | Settings of the SPRT, the league and the arena for evaluations ([ADR 0012](../../docs/adr/0012-evaluation.md)) |
| `releases/` | Frozen paste-ready file of each version, from the first release on |
| [journal/](journal/) | One entry per experiment |

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed. CodinGame shuffles that list, so this plays randomly there. | Proved the paste-to-CodinGame path (Phase 0, 2026-10-07) |
| [random](bots/random/) | Plays a uniformly random valid action, seeded | Baseline for every rating; first bot built from shared crates |
| [greedy](bots/greedy/) | Looks one move ahead: wins the game or a small board when it can, avoids handing the opponent either, or a free choice; random among equals | Fixed baseline about 500 Elo above random: the first release's opponent, and the arena's controls in CI |
| [mcts](bots/mcts/) | Monte Carlo tree search (UCT, random playouts that take a game-winning move when there is one) for 82 ms of 100, keeping its tree from turn to turn and proving wins and losses; plays a forced or game-winning action at once | The real bot: released as `uttt-v001`, then improved by experiments (see the journal) |
| [wood](bots/wood/) | Perfect 3×3 tic-tac-toe; among moves that never lose, the one that wins most against random play | Promotion out of Wood league, whose boss plays randomly. Expected score against random play: 99.7% moving first, 95.8% moving second. On the 9×9 board it plays randomly. |
| [rules-check](bots/rules-check/) | Plays randomly; every turn, compares CodinGame's valid actions with the engine's | Checking the engine against real CodinGame games, from Bronze league |

## Playing matches locally

```sh
cargo build --release -p uttt-arena -p uttt-bot-random -p uttt-bot-greedy
target/release/uttt-arena match \
  --bot greedy=target/release/uttt-bot-greedy \
  --bot random=target/release/uttt-bot-random \
  --pairs 100 --opening-plies 4 --out target/uttt-results.jsonl
```

Each pair plays the same seed twice with seats swapped; `--opening-plies 4` starts the pair with 4 random moves imposed by the referee. Two more commands use the same options: `sprt` plays a candidate against a baseline until the test decides, and `league` rates several bots. See `uttt-arena <command> --help`.

## Evaluating a bot

With the settings of [evaluation.env](evaluation.env):

- `scripts/sprt.sh uttt CANDIDATE.rs [BASELINE.rs]`: smoke test against random, then SPRT against the baseline (by default the newest release older than the candidate, or `greedy`). Settings can be overridden from the environment, for example `TIME_SCALE=1 TIME_TOLERANCE_MS=0` for CodinGame's full limits.
- `scripts/league.sh uttt`: ratings of every release with `random` and `greedy`.
- `scripts/new-release.sh uttt BOT`: freezes a bot as the next release. The whole experiment flow is in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Engine speed

`cargo run --release -p uttt-engine --example speed` plays random games from the start position for a few seconds, then decisive playouts (a move that wins the game when there is one, as the search plays them), then runs 100 ms MCTS searches from there. CI runs it on every push and shows the table in the job summary of the "CodinGame compatibility" job.

Baseline, measured on 2026-10-07 on one thread of an Intel Xeon at 2.8 GHz (this shared machine varies by about 10% between runs), after E005 made playouts take a winning move when there is one:

| Engine benchmark | Result |
| --- | --- |
| Random playouts from the start | about 925,000 per second (915,819 to 935,661 over three runs) |
| Moves per playout | 58.9 |
| Moves per second | about 54,000,000 |
| Decisive playouts from the start | about 976,000 per second (946,885 to 991,444), 54.2 moves each |
| MCTS iterations from the start, 100 ms searches | about 620,000 per second (609,767 to 633,933) |

Random playouts did not change in E005; they measured about 985,000 per second just before it, so the machine ran about 6% slower for this baseline. Decisive playouts are shorter, which makes up for the check they add.

After E004: about 985,000 random playouts and 578,000 MCTS iterations per second. Before E004: 441,681 playouts and about 350,000 MCTS iterations per second.

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. To check that, once in Bronze league or above:

1. Paste the bundled `rules-check` bot into the IDE and click **Play my code** a few times.
2. In the console, every turn prints a line ending with `N turns checked, M with differences`. Any difference is printed on a line starting with `rules-check: DIFFERENCE`, with the actions only one side listed.
3. When the bot's own move ends the game, it prints the result the engine expects, to compare with CodinGame's.

## Putting a bot on CodinGame

Each release has a GitHub release with its paste-ready file attached; the same file is in `releases/`. For any other bot:

1. Bundle it into one file: `scripts/bundle-bots.sh` writes `target/cg/uttt-<bot>.rs` for every bot. CI also publishes these files as the `paste-ready-bots` artifact of each run.
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
