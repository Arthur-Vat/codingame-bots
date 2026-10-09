# CLAUDE.md

Rust framework for two-player CodinGame bots. Claude writes the code; CI judges; the owner (Arthur) decides and merges. Read `docs/ROADMAP.md` for the current phase and `docs/ARCHITECTURE.md` for the target design before starting non-trivial work.

## Commands

Run all of these before saying work is done, and paste their summary lines in the pull request:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/bundle-bots.sh && scripts/cg-check.sh target/cg/*.rs
```

For bot or engine changes, also play the bundled bots against each other with the game's arena (for example `target/release/uttt-arena --help`) and report the summary. For engine changes, also report `cargo run --release -p uttt-engine --example speed` against the baseline in `games/uttt/README.md`.

For a bot strength experiment, follow "An experiment, step by step" in `docs/WORKFLOW.md`: `scripts/new-release.sh` makes the candidate, `scripts/sprt.sh` runs the test locally, and the SPRT workflow judges it.

## Hard rules

- Never merge, push to `main`, or force-push. Work on a `claude/` branch and open a pull request; the owner approves merges.
- Never weaken a test, the referee, a CI check or a threshold to make a change pass. If a check seems wrong, say so and stop.
- One topic per pull request. For bot strength work: one experiment per pull request.
- Released files (`games/*/releases/`) never change; a better bot is a new release.
- A decision that is costly to reverse goes to the owner first, then into a new ADR in `docs/adr/` (template in `docs/adr/template.md`). Never rewrite an accepted ADR; supersede it.
- If something needed is missing (access, a network domain, a secret, a fact about CodinGame), say exactly what and stop. Do not mock, guess or invent CodinGame rules.
- English everywhere: code, comments, docs, commits, pull requests.

## Rust and CodinGame constraints

- CodinGame compiles Rust 1.90.0 with the 2021 edition (`docs/CODINGAME.md`). The workspace sets `rust-version = "1.90"`; CI uses exactly 1.90.0. This workspace may have a newer Rust and cannot download 1.90.0, so heed clippy's `incompatible_msrv` warnings and avoid syntax newer than 1.90.
- Any code that ends up inside a bot uses the standard library only: no crates.io dependencies. Tools (arena, bundler) may use dependencies allowed by `deny.toml`.
- A paste-ready bot is one file under 100,000 bytes that compiles with `rustc --edition 2021` alone.
- No `unsafe` (workspace lint). Proposing an exception needs an ADR.
- Crates that end up in a bot follow the bundler's conventions (`crates/cg-bundler/src/lib.rs`): modules in files declared with `mod name;` on their own line, tests in separate files (`#[cfg(test)]` then `mod tests;`), other workspace crates referred to by name (`cg_core::`), no `#[path]`.
- Bots must flush stdout after every move and print debug output to stderr only.

## Conventions

- Commit subjects and pull request titles follow Conventional Commits with a required scope, `type(scope): summary` ([ADR 0022](docs/adr/0022-scopes-and-names.md)): a game's folder name (`uttt`) for work on that game, else a framework area: `core`, `search`, `arena`, `bundler`, `workflows`, `scripts`, `adr`, `docs`, `agents`, `repo`. Several scopes are separated by commas (`ci(workflows,uttt): ...`). At most 100 characters, no final period.
- Every decision record has a `Scope:` line, `framework` or a game's folder name, and appears in its scope's group in `docs/adr/README.md`.
- New crates join the workspace in the root `Cargo.toml` and inherit `[workspace.package]` fields and `[lints] workspace = true`.
- Game rules go in `games/<game>/RULES.md`, in our own words, with a source for every rule; other game facts in `games/<game>/README.md`; platform facts in `docs/CODINGAME.md`.
- Keep `docs/ROADMAP.md` status in sync when a phase item is done.
