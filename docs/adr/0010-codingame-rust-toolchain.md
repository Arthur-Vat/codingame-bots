# 0010. Target CodinGame's Rust: 1.90.0, edition 2021

- Status: accepted
- Date: 2026-10-07
- Scope: framework

## Context

On 2026-10-06 a probe in the CodinGame IDE showed rustc 1.90.0, and a file using 2021-edition behaviour compiled. Whether the 2024 edition is accepted was not tested. CodinGame offers no crates.

The Claude workspace used for this project cannot download Rust toolchains (`static.rust-lang.org` is blocked by its network policy). A `rust-toolchain.toml` pinning 1.90.0 would therefore make every local `cargo` command fail there.

## Decision

- Every crate uses the 2021 edition.
- `rust-version = "1.90"` is set for the whole workspace, so clippy rejects standard library APIs newer than CodinGame's compiler.
- CI builds, lints and tests with exactly Rust 1.90.0 (`CG_RUST` in `.github/workflows/ci.yml`), and compiles every paste-ready file on its own with that compiler (`scripts/cg-check.sh`).
- Local work may use any newer stable Rust; CI is the judge.
- No `rust-toolchain.toml`.
- Crates that end up inside a bot use the standard library only.

## Consequences

- Language features or APIs newer than 1.90 are caught by clippy locally or by CI.
- `CG_RUST` and `rust-version` must change together, after re-running the probe described in `docs/CODINGAME.md`.
- Tools that never run on CodinGame (arena, bundler) may use crates.io dependencies, subject to `deny.toml`.
