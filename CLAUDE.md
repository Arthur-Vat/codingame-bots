# CLAUDE.md

Rust framework for two-player CodinGame bots. Claude writes the code; CI judges; the owner (Arthur) decides and merges. Read `docs/ROADMAP.md` for the current phase and `docs/ARCHITECTURE.md` for the target design before starting non-trivial work.

## Commands

Run all of these before saying work is done, and paste their summary lines in the pull request:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/cg-check.sh games/*/bots/*/src/main.rs
```

## Hard rules

- Never merge, push to `main`, or force-push. Work on a `claude/` branch and open a pull request; the owner approves merges.
- Never weaken a test, the referee, a CI check or a threshold to make a change pass. If a check seems wrong, say so and stop.
- One topic per pull request. For bot strength work: one experiment per pull request.
- A decision that is costly to reverse goes to the owner first, then into a new ADR in `docs/adr/` (template in `docs/adr/template.md`). Never rewrite an accepted ADR; supersede it.
- If something needed is missing (access, a network domain, a secret, a fact about CodinGame), say exactly what and stop. Do not mock, guess or invent CodinGame rules.
- English everywhere: code, comments, docs, commits, pull requests.

## Rust and CodinGame constraints

- CodinGame compiles Rust 1.90.0 with the 2021 edition (`docs/CODINGAME.md`). The workspace sets `rust-version = "1.90"`; CI uses exactly 1.90.0. This workspace may have a newer Rust and cannot download 1.90.0, so heed clippy's `incompatible_msrv` warnings and avoid syntax newer than 1.90.
- Any code that ends up inside a bot uses the standard library only: no crates.io dependencies. Tools (arena, bundler) may use dependencies allowed by `deny.toml`.
- A paste-ready bot is one file under 100,000 bytes that compiles with `rustc --edition 2021` alone.
- No `unsafe` (workspace lint). Proposing an exception needs an ADR.
- Bots must flush stdout after every move and print debug output to stderr only.

## Conventions

- Commits follow Conventional Commits: `feat:`, `fix:`, `docs:`, `test:`, `ci:`, `refactor:`, `chore:`, with a scope when useful (`feat(uttt): ...`).
- New crates join the workspace in the root `Cargo.toml` and inherit `[workspace.package]` fields and `[lints] workspace = true`.
- Game-specific facts go in `games/<game>/README.md` with a source; platform facts in `docs/CODINGAME.md`.
- Keep `docs/ROADMAP.md` status in sync when a phase item is done.
