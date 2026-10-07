# codingame-bots

A Rust framework to build, test, rate and improve bots for two-player [CodinGame](https://www.codingame.com) bot programming games, aiming for Legend league in each game over time. The first game is [Ultimate Tic-Tac-Toe](games/uttt/).

Every bot change is judged by CI: unit tests, a check that the bot compiles on CodinGame, and later statistical matches against the previous champion. Every released bot is a single readable file you paste into the CodinGame editor.

**Status:** Phase 1 (framework core). See the [roadmap](docs/ROADMAP.md).

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
