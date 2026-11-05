# 0054. Local checks follow CI's scopes

- Status: accepted (owner, 2026-11-05)
- Date: 2026-11-05
- Completes: [0029](0029-ci-scoped-by-area-and-game.md), decision 3
- Scope: framework

## Context

Since [ADR 0029](0029-ci-scoped-by-area-and-game.md), CI runs only the checks of the areas a pull request changes. The local checks did not follow: `CLAUDE.md` asked for the same five commands before every pull request, whatever it changed, so a scripts-only or game-only change still tested and bundled the whole workspace. On 2026-11-05, for a change to two scripts and a workflow, `cargo test --workspace` alone took about 6 minutes and every bot was bundled, although no Rust had changed.

The owner asked on 2026-11-05 whether local tests should target only the impacted parts of the repository. Offered four improvements, he chose "Scoped check script"; not chosen that day: pinning Rust 1.90.0 locally, cargo-nextest or profiling the slowest test binaries, and a target folder of their own for reviewer agents.

## Decision

1. **`scripts/check.sh` runs the local checks that CI would run for a branch's changes.** It takes the files changed since the merge base with `origin/main` (committed or not, untracked included, both paths of a move, as CI lists them), asks `scripts/ci-scopes.sh` for their areas, as CI does, and runs each area's local checks: the docs check, shellcheck and the script tests always; formatting and clippy on the workspace when Rust is in scope; `cargo test --workspace` and every bot's bundle for the framework, a workflow, the workspace's configuration or an unknown path; for a game, the tests of the shared crates, of the game's packages and of the studio, one `cargo test` per CI job (one run of several jobs' packages would build each with the features of all), and its bots' bundles; for the studio's Rust crates, their tests and the games' adapters'; for the studio or the framework, the Ultimate Tic-Tac-Toe adapter's tests built alone (without its feature `analysis`) and the WebAssembly build (`scripts/build-studio-wasm.sh --strict`, cached, when the target is installed). The dependency policy runs when `cargo-deny` is installed, actionlint when a workflow changed and Docker runs. Left to CI: the games' `ci.sh` (arena games), the release checks and the front end's checks.
2. **`CLAUDE.md` asks for `scripts/check.sh` before saying work is done**, and its summary lines in the pull request. `scripts/check.sh --all` runs every check, as the former list did.
3. **No new mapping of paths to areas.** `scripts/ci-scopes.sh` stays the one place that decides (ADR 0029, decision 3); this script only turns its areas into local commands.
4. Unchanged: playing the bundled bots with the game's arena for bot or engine changes, the speed report for engine changes, and the experiment steps (`CLAUDE.md`).

## Consequences

- A docs or scripts change checks in seconds or minutes instead of a full test run; a game's change no longer tests or bundles the other game.
- A mistake in the scheme now hides a failure locally as well as on the pull request; pushes to `main` still run everything (ADR 0029, decision 2).
- The script uses the default toolchain, as `CLAUDE.md`'s commands did; CI uses 1.90.0. `RUSTUP_TOOLCHAIN=1.90.0 scripts/check.sh` matches it.
- A new CI job for an area needs its local counterpart in `scripts/check.sh` when it can run locally.
