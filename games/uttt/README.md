# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)). The rules, protocol and time limits are in [RULES.md](RULES.md).

**Status:** Phase 2. The reference referee, the fast engine (checked against it), an arena and four utility bots exist. The first real bot, MCTS, comes in Phase 4.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [referee/](referee/) | Readable reference implementation of the rules, driven by the arena |
| [engine/](engine/) | Fast implementation of the rules for bots: bitboards, no allocation, random playouts |
| [arena/](arena/) | `uttt-arena`: plays bots against each other through the referee |
| [bots/](bots/) | One crate per bot |

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed. CodinGame shuffles that list, so this plays randomly there. | Proved the paste-to-CodinGame path (Phase 0, 2026-10-07) |
| [random](bots/random/) | Plays a uniformly random valid action, seeded | Baseline for every rating; first bot built from shared crates |
| [wood](bots/wood/) | Perfect 3×3 tic-tac-toe; among moves that never lose, the one that wins most against random play | Promotion out of Wood league, whose boss plays randomly. Expected score against random play: 99.7% moving first, 95.8% moving second. On the 9×9 board it plays randomly. |
| [rules-check](bots/rules-check/) | Plays randomly; every turn, compares CodinGame's valid actions with the engine's | Checking the engine against real CodinGame games, from Bronze league |

## Playing matches locally

```sh
cargo build --release -p uttt-arena -p uttt-bot-random -p uttt-bot-first-valid
target/release/uttt-arena \
  --bot random=target/release/uttt-bot-random \
  --bot first-valid=target/release/uttt-bot-first-valid \
  --pairs 100 --out target/uttt-results.jsonl
```

Each pair plays the same seed twice with seats swapped. See `uttt-arena --help` for all options.

## Engine speed

`cargo run --release -p uttt-engine --example speed` plays random games from the start position for a few seconds. CI runs it on every push and shows the table in the job summary of the "CodinGame compatibility" job.

Baseline, measured on 2026-10-07 on one thread of an Intel Xeon at 2.8 GHz:

| Engine benchmark | Result |
| --- | --- |
| Random playouts from the start | 441,681 per second |
| Moves per playout | 58.9 |
| Moves per second | 26,020,079 |

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. To check that, once in Bronze league or above:

1. Paste the bundled `rules-check` bot into the IDE and click **Play my code** a few times.
2. In the console, every turn prints a line ending with `N turns checked, M with differences`. Any difference is printed on a line starting with `rules-check: DIFFERENCE`, with the actions only one side listed.
3. When the bot's own move ends the game, it prints the result the engine expects, to compare with CodinGame's.

## Putting a bot on CodinGame

1. Bundle it into one file: `scripts/bundle-bots.sh` writes `target/cg/uttt-<bot>.rs` for every bot. CI also publishes these files as the `paste-ready-bots` artifact of each run.
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
