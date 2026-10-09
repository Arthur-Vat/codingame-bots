# codingame-bots

A Rust framework to build, test, rate and improve bots for two-player [CodinGame](https://www.codingame.com) bot programming games, aiming for Legend league in each game over time. The first game is [Ultimate Tic-Tac-Toe](games/uttt/).

Every bot change is judged by CI: unit tests, a check that the bot compiles on CodinGame, and, for a new release, a statistical match (a sequential probability ratio test, SPRT) against the previous release. Every released bot is a single readable file you paste into the CodinGame editor.

**Status:** phases 0 to 4 are done; phase 5, autonomy and workflow, is in progress. See the [roadmap](docs/ROADMAP.md).

## Games

| Game | CodinGame league | Current release | Journal |
| --- | --- | --- | --- |
| [Ultimate Tic-Tac-Toe](games/uttt/) | Legend since 2026-10-07; 59th with `uttt-v008` (2026-10-08) | [`uttt-v010`](games/uttt/releases/uttt-v010.rs) | [journal](games/uttt/journal/) |

## Where things are

- [`crates/`](crates/): code shared by every game. `cg-core` reads CodinGame's input and gives bots a seeded random generator and the time scale. `cg-search` is a game trait with Monte Carlo tree search. `cg-arena` is the referee trait, the match runner, the SPRT, the ratings and the command line. `cg-bundler` turns a bot and the crates it uses into one paste-ready file.
- [`games/<game>/`](games/): everything for one game. Its rules (`RULES.md`), a readable referee, a fast engine, an arena binary, a trainer, the bots, the frozen releases, the journal of experiments, the reports of training runs, and the settings of its evaluations.
- [`docs/`](docs/): architecture, roadmap, workflow, CodinGame facts, and the decision records in `docs/adr/`.
- [`scripts/`](scripts/): bundling and checking bots, making and checking releases, the SPRT, the league, and pruning finished branches.
- [`.github/workflows/`](.github/workflows/):
  - `ci.yml`: format, lint, tests, CodinGame compatibility, dependency licenses.
  - `sprt.yml`: smoke test and SPRT of each new release.
  - `league.yml`: ratings of all releases.
  - `release.yml`: a GitHub release for each new release file.
  - `train.yml`: self-play and training, started by hand.
  - `prune-branches.yml`: weekly deletion of finished `claude/` branches.

## Documentation

- [Architecture](docs/ARCHITECTURE.md): components, evaluation pipeline, journal, CI
- [Roadmap](docs/ROADMAP.md): phases, gates, risks
- [Workflow](docs/WORKFLOW.md): how work is proposed, judged and approved
- [CodinGame platform facts](docs/CODINGAME.md): compiler version, limits, how to re-check them
- [Decision records](docs/adr/)

## Development

The workspace targets Rust 1.90.0 with the 2021 edition, the compiler CodinGame uses. Any newer stable Rust works locally; CI checks with exactly 1.90.0.

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/bundle-bots.sh                 # every bot -> target/cg/<game>-<bot>.rs
scripts/cg-check.sh target/cg/*.rs     # size and standalone compile of each bundle
```

To play bots against each other locally, see [games/uttt/README.md](games/uttt/README.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
