# Architecture

A reusable Rust framework that takes a two-player CodinGame game from rules to a Legend-level bot. Each game plugs into generic crates through one `Game` trait; every bot ships as a single file pasted into CodinGame. Claude writes the code, CI judges the results, and the owner makes the decisions.

This document describes the target design. [ROADMAP.md](ROADMAP.md) says which parts exist today.

## Principles

- **Framework over one-shot.** Every game reuses the same arena, tests, rating and release machinery. A new game adds its rules, an engine and bots.
- **Claude proposes, CI judges.** No bot counts as stronger until a sequential probability ratio test (SPRT) says so.
- **The owner decides.** Claude never merges ([ADR 0006](adr/0006-human-approves-merges.md)); important choices become ADRs.
- **Free and reproducible.** Public repository, standard GitHub-hosted runners, pinned compiler, seeded games.
- **Paste-ready output.** Every bot version ships as one readable `.rs` file.

## Layout

Items marked "Phase N" do not exist yet.

```text
codingame-bots/
├─ Cargo.toml               workspace: edition 2021, rust-version 1.90
├─ CLAUDE.md                rules for every Claude session
├─ deny.toml                dependency license and source policy
├─ docs/                    this file, ROADMAP, WORKFLOW, CODINGAME, adr/
├─ scripts/cg-check.sh      size and standalone-build check of paste-ready files
├─ crates/                  Phase 1
│  ├─ cg-core/              Game trait, CodinGame protocol I/O, time budget, RNG
│  ├─ cg-arena/             match runner, tournaments, SPRT, ratings
│  ├─ cg-bundler/           bot + its crates -> one paste-ready .rs file
│  └─ cg-search/            reusable search (MCTS first), once a second game needs it
├─ games/
│  └─ uttt/
│     ├─ README.md          protocol, known rules, bots
│     ├─ RULES.md           Phase 2: the rules in our own words
│     ├─ engine/            Phase 2: rules and fast state, used by referee and bots
│     ├─ referee/           Phase 2: CodinGame-protocol referee binary
│     ├─ bots/              first-valid/ today; random/, mcts/ ... later
│     ├─ releases/          Phase 3: frozen paste-ready file per version
│     ├─ openings/          Phase 3: opening positions for fair matches
│     └─ journal/           Phase 3: one entry per experiment
├─ .claude/                 Phase 5: skills, agents, settings
└─ .github/                 CI workflow, dependabot, PR template
```

## Components

- **`Game` trait (`cg-core`):** state, moves, legal-move generation, applying a move, outcome, player to move, and how a turn is written to and read from the CodinGame protocol. Each game implements it once.
- **Referee:** one binary per game, built on its engine. It talks to two bot processes exactly as CodinGame does and enforces the game's time limits.
- **Arena:** runs many referee matches in parallel, from openings, in both seat orders. It writes one JSON line per game: bots, opening, result, per-turn times, crashes and timeouts.
- **Bot:** an ordinary binary reading stdin and writing stdout. An optional `CG_FIXED_ITERS` environment variable replaces the time budget by a fixed iteration count for reproducible tests; CodinGame never sets it.
- **Bundler:** flattens a bot and the workspace crates it uses into one formatted file. Crates that end up in a bot use the standard library only.
- **Versions:** `releases/` keeps the exact file pasted for each version. The arena compiles old versions from these files, so history never rebuilds differently.

Because the referee and the bots share one engine, a rules bug would be invisible in local matches: both sides would agree on the wrong rule. The parity tests against an independent reference exist to catch exactly that.

## Evaluation pipeline

Seven tiers protect every change; only the SPRT decides whether a bot is stronger. Values marked "proposed" are confirmed by an ADR in Phase 3.

| Tier | Checks | Runs on | Passes when |
| --- | --- | --- | --- |
| 1. Static | Format, lints, unit tests, property tests of engine invariants | Every push | All green |
| 2. Parity | Our referee against an independent reference on random games | Engine or referee changes | Identical states and results |
| 3. CodinGame compatibility | Bundle, size, standalone compile with Rust 1.90.0 | Every bot change | Compiles and stays under 100 kB |
| 4. Smoke | Bot against a random bot, both seats, many openings | Every bot change | 0 crashes, 0 timeouts, 0 illegal moves, at least 99% wins (proposed) |
| 5. Speed | Simulations per second on fixed positions | Every bot change | No drop over 5% against the parent version (proposed) |
| 6. SPRT | Candidate against the current champion | Candidate pull requests | The test accepts the candidate as stronger |
| 7. League | Round-robin of all released versions | Weekly and after each release | Ratings published |

- **Openings and pairs:** Ultimate Tic-Tac-Toe has no random map, so each match starts from an opening (a few random moves) played twice with seats swapped.
- **SPRT (proposed):** scored on game pairs, which handles draws; bounds of 0 and 10 Elo; 5% error rates; capped at 20,000 games. Bounds tighten as the bot matures.
- **Sanity checks of the pipeline itself:** an A/A test (a bot against itself must not "win") and a deliberately weakened bot that must be rejected.
- **Ratings:** Elo with 95% confidence intervals over all released versions, the random bot anchored at 0.
- **Timing noise:** shared runners are noisy, so matches run one game per CPU core with a small tolerance on time limits. Fixed-iteration mode is only for deterministic tests; strength is always measured under real time limits.

## Journal and versioning

Every experiment gets a journal entry, including failures; only experiments that pass the SPRT become versions.

- **Versions:** git tags `uttt-v001`, `uttt-v002`, one per promoted champion.
- **Release:** each version gets a GitHub release with the paste-ready file attached; the same file is committed under `releases/`.
- **Entry:** one file per experiment in `games/<game>/journal/`. The pull request collects the hypothesis and the change, CI fills in the results, and the entry is committed when the pull request is merged or closed.
- **Failed experiments stay** so that ideas are not retried blindly.
- **CodinGame rank:** after pasting a version, the owner reports the rank and it is recorded in that version's entry.

Example entry (illustrative values):

```markdown
---
id: E007
date: 2026-10-20
parent: uttt-v003
hypothesis: Reusing the search tree between turns adds strength at 100 ms.
change: Keep the subtree of the played move instead of starting fresh.
sprt: accepted
elo: +38 ± 12
games: 3,412
speed: -1%
decision: promoted
version: uttt-v004
cg_rank: 112 (Gold)
---
Notes: gains come mostly from the first 20 moves.
```

## CI/CD

GitHub Actions on free standard runners. `main` only accepts changes whose checks are green.

| Workflow | Trigger | Does | Exists |
| --- | --- | --- | --- |
| `ci.yml` | Every push and pull request | Format, lint, tests, CodinGame compatibility, dependency policy; later smoke and speed | Yes |
| `parity.yml` | Pull requests touching an engine or referee | Tier 2 | Phase 2 |
| `sprt.yml` | Pull requests labelled `candidate` | Tier 6, verdict as a PR comment and a required check | Phase 3 |
| `release.yml` | Merge of a candidate pull request | Tag, paste-ready file, GitHub release, journal entry, dashboard | Phase 3 |
| `league.yml` | Weekly and after each release | Tier 7, ratings on GitHub Pages | Phase 3 |

- **Compiler:** every job uses Rust 1.90.0, CodinGame's version ([ADR 0010](adr/0010-codingame-rust-toolchain.md)).
- **Branch protection:** a ruleset on `main` requires the CI checks and blocks direct pushes, without requiring a GitHub review ([ADR 0006](adr/0006-human-approves-merges.md)).
- **Compute:** standard runners only, which are free on public repositories. The SPRT is split across parallel jobs.
