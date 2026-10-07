# CodinGame platform facts

Facts the code depends on, and where each one comes from. Re-check them when CodinGame announces a language update. Game-specific facts live in each game's README (for example [games/uttt/README.md](../games/uttt/README.md)).

## Compilers

| Language | Version | Source |
| --- | --- | --- |
| Rust | rustc 1.90.0. The 2021 edition compiles; the 2024 edition was not tested. | Probe in the CodinGame IDE, 2026-10-06 |
| C++ (reference only) | GCC 11.2.0 | Same probe |

No crates are available: a bot is one file using the standard library only.

### Re-running the probe

1. Open any bot programming game on CodinGame and enter the IDE.
2. Select **Bash**, paste the script below, click **Play my code**, and read the error console:

   ```bash
   rustc --version >&2 || /usr/local/bin/rustc --version >&2
   g++ --version | head -1 >&2
   ```

3. Select **Rust**, paste the program below and play again. It prints `edition 2021 OK: 7` only if the 2021 edition is used (`try_into` is not in the 2018 prelude):

   ```rust
   fn main() {
       let x: u8 = 300u16.try_into().unwrap_or(7);
       eprintln!("edition 2021 OK: {}", x);
   }
   ```

If the Rust version changes, update `CG_RUST` in `.github/workflows/ci.yml` and `rust-version` in `Cargo.toml` together, and record it in a new ADR that supersedes [ADR 0010](adr/0010-codingame-rust-toolchain.md).

## Compilation mode

In the past, CodinGame compiled Rust in debug mode in the IDE and in release mode only for arena submissions ([forum](https://forum.codingame.com/t/rust-release-mode-compilation/33552?page=2)). A later announcement planned release mode everywhere ([forum](https://forum.codingame.com/t/languages-update/1574/223)). Until checked, expect a bot to be slower under **Play my code** than in ranked games. Do not tune time management from IDE runs.

## Submission limits

- **Code size:** players report a strict 100 kB limit ([forum](https://forum.codingame.com/t/ultimate-tic-tac-toe-puzzle-discussion/22616?page=2)). `scripts/cg-check.sh` enforces 100,000 bytes.
- **One file:** the bundler (Phase 1) merges a bot and its workspace crates into one file.

## Timing

CodinGame measures a turn from when it starts sending the turn's input until it reads the bot's output. Extra time before the first input exists in practice but is not guaranteed ([CodinGame staff on the forum](https://forum.codingame.com/t/timeouts-how-do-they-work/24504/4)). Most multiplayer games give 1 s on the first turn and 50 ms afterwards, but older games differ, so each game's README records its own limits.
