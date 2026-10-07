# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)). The rules, protocol and time limits are in [RULES.md](RULES.md).

**Status:** Phase 1. A referee implementing the rules, two baseline bots and an arena exist. A fast engine for search bots comes in Phase 2.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [referee/](referee/) | Readable reference implementation of the rules, driven by the arena |
| [arena/](arena/) | `uttt-arena`: plays bots against each other through the referee |
| [bots/](bots/) | One crate per bot |

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed. CodinGame shuffles that list, so this plays randomly there. | Proved the paste-to-CodinGame path (Phase 0, 2026-10-07) |
| [random](bots/random/) | Plays a uniformly random valid action, seeded | Baseline for every rating; first bot built from shared crates |

## Playing matches locally

```sh
cargo build --release -p uttt-arena -p uttt-bot-random -p uttt-bot-first-valid
target/release/uttt-arena \
  --bot random=target/release/uttt-bot-random \
  --bot first-valid=target/release/uttt-bot-first-valid \
  --pairs 100 --out target/uttt-results.jsonl
```

Each pair plays the same seed twice with seats swapped. See `uttt-arena --help` for all options.

## Putting a bot on CodinGame

1. Bundle it into one file: `scripts/bundle-bots.sh` writes `target/cg/uttt-<bot>.rs` for every bot. CI also publishes these files as the `paste-ready-bots` artifact of each run.
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
