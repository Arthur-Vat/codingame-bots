# 0055. Pin CodinGame's Rust in the repository

- Status: accepted (owner, 2026-11-06)
- Date: 2026-11-06
- Supersedes: [0010](0010-codingame-rust-toolchain.md), its decisions that local work may use any newer stable Rust and that there is no `rust-toolchain.toml`
- Scope: framework

## Context

[ADR 0010](0010-codingame-rust-toolchain.md) left local work on any newer stable Rust and ruled out a `rust-toolchain.toml`, because Claude's sessions could not download toolchains then. They can since 2026-10-16.

Sessions start on a newer stable Rust, while CI and CodinGame use 1.90.0. A newer clippy brings new lints: on 2026-11-06, Rust 1.97's `unnecessary_sort_by` failed `scripts/check.sh` on two lines of `ice-and-fire-opening` that CI accepts, which happened often in new sessions. A newer compiler also runs the same code at another speed ([E026](../../games/uttt/journal/E026-fused-children.md)), and the bundler's minifier needs rustc 1.90.0 ([ADR 0046](0046-one-minifier.md)), so sessions installed it by hand and had to remember `cargo +1.90.0` or `RUSTUP_TOOLCHAIN=1.90.0`.

The owner chose to pin the version in the repository.

## Decision

1. **A `rust-toolchain.toml` at the root pins Rust 1.90.0 with rustfmt and clippy, in rustup's minimal profile.** Every `cargo` and `rustc` command in the repository uses it, and rustup installs whatever is missing on the first one.
2. **It lists no extra targets.** rustup installs a listed target in every job that runs `cargo`, and only a few jobs build to WebAssembly (the studio's and the Pages deployment's): those install `wasm32-wasip1` themselves, and a session adds it with `rustup target add wasm32-wasip1` when it needs the WebAssembly build.
3. **Its version changes together with every other place that names CodinGame's Rust**: `rust-version` in `Cargo.toml`, `CG_RUST` in each workflow, the bundler's `VERSION` (`crates/cg-bundler/src/prune/rustc.rs`) and the std names list made from that version's `rust-src`, after re-running the probe described in `docs/CODINGAME.md`.

## Consequences

- Local checks, bundles and speed measurements use CodinGame's compiler without extra flags; `cargo +1.90.0` still works and means the same.
- A CI job whose setup installs 1.90.0 without rustfmt or clippy now also downloads them when it first runs `cargo` (about a second in a session).
- A newer Rust is used only when asked for (`cargo +stable`).
- Running `scripts/check.sh` with `RUSTUP_TOOLCHAIN=1.90.0`, as [ADR 0054](0054-local-checks-follow-ci-scopes.md)'s consequences advise, is no longer needed.
- The version is named in several places, which a test should keep equal.
