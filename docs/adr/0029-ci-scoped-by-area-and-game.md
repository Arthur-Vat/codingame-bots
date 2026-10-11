# 0029. CI scoped by area and game

- Status: accepted (owner, 2026-10-11)
- Date: 2026-10-11
- Completes: [0012](0012-evaluation.md) (decisions 10 and 12: the SPRT and the league run per game)
- Scope: framework

## Context

Until this record, every pull request ran every CI job, whatever it changed: a one-line edit to a journal entry built and tested the whole workspace, bundled and played every bot, and built the studio's front end and its Windows job. A second game (phase 7) would double that again. The roadmap's phase 5 step E asked that documentation-only pull requests skip the heavy steps while still reporting the required checks.

On 2026-10-11 the owner chose to scope CI fully, by area and by game, rather than only skipping work for documentation, and warned that the SPRT is scoped by game as well. The same day the owner answered two related questions in the conversation: the studio's checks become blocking, through the aggregate check below ("Yes", settling whether the studio's front end job should be required, a question left open on 2026-10-10), and the engine speed measurement stays a report, not a check ("Ok").

GitHub's required checks are matched by job name. A job skipped by a condition counts as passed, but one that never starts leaves a required check waiting forever, so a scheme that skips jobs needs required checks that always run.

## Decision

1. **A pull request runs the checks of the areas it changes, and of the areas that build on them.** The areas follow the scopes of [ADR 0022](0022-scopes-and-names.md):
   - *framework*: the shared crates and scripts (`crates/`, `scripts/`);
   - *a game*: its folder `games/<game>/`, Markdown aside;
   - *the studio*: its Rust crates (`studio/game/`, `studio/server/`) and its front end (`studio/web/`), apart;
   - *docs*: Markdown anywhere, and everything under `docs/` and `.claude/`, which only the docs and scripts check covers.

   A framework change runs every game's checks and the studio's. A game's change runs that game's checks, the framework's tests, since the bundler's tests bundle a real bot, and the studio's Rust checks, since the server holds the games' adapters and its tests play the game's releases. A change to the studio's Rust crates runs the studio's Rust checks, which test every game's studio adapter as well, since the adapters build on the studio's shared crate. A front end change runs the front end's checks only; a docs change, the docs check only. The dependency policy runs when any `Cargo.toml`, `Cargo.lock` or `deny.toml` changes. The docs and scripts check runs on every pull request.
2. **Pushes to `main`, runs started by hand, changes to the workflows (`.github/`) or the workspace's configuration (the root `Cargo.toml` and `Cargo.lock`, `deny.toml`, the toolchain and Cargo settings), and any path the scheme does not know run everything.** `main` is proven whole after every merge, so a mistake in the scheme shows up there at the latest, and a new kind of file is checked until the scheme learns it.
3. **One script maps paths to areas**, `scripts/ci-scopes.sh`, with a table test. The workflows read its output; nothing else decides what runs.
4. **A game's checks run in jobs of their own, named after the game** (`<game>: tests`, `<game>: bots`). Steps that only one game needs live in `games/<game>/ci.sh`; framework scripts take the game as an argument. A new game joins CI by its folder and its `ci.sh`.
5. **Branch protection requires aggregate checks, not individual jobs:** `CI passed` for `ci.yml`, `SPRT passed` for the SPRT workflow, and the pull request hygiene check. Each aggregate always runs, and fails when a job it gathers failed, was cancelled, or was skipped although its area changed. Jobs can then be added, renamed, split per game or skipped without touching the ruleset, and every job under an aggregate is blocking, the studio's included.
6. **The SPRT and the league run per game.** The SPRT tests each game that a pull request adds a release to, in a job named after the game, and keeps each result for the exact framework and game code it was obtained with, so a change to another game or to Markdown does not run it again. The league runs, on `main`, for each game whose releases or settings changed, or for every game when the league itself changed (its workflow, its script or the evaluation settings' shared reader).
7. **The engine speed measurement stays a report.** It runs with the game's bot checks and never fails them; a speed regression check is separate work (phase 5, step E).

## Consequences

- A documentation or journal pull request finishes in minutes; a game's change no longer waits on the other games, nor a front end change on Rust.
- A pull request can now merge with a check skipped that would have failed; pushes to `main` catch it after the merge rather than before. A mistake in `scripts/ci-scopes.sh` costs exactly that, which its table test guards against.
- The owner changes the ruleset once, when the aggregates arrive, and no more when jobs change. The list of required checks is kept in `docs/ARCHITECTURE.md`.
- A new game must provide `games/<game>/ci.sh`; the skill that adds a game (`new-game`, phase 7) must say so.
