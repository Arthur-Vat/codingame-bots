# Architecture

A reusable Rust framework that takes a two-player CodinGame game from rules to a Legend-level bot. Each game plugs into the arena through one `Referee` trait; every bot ships as a single file pasted into CodinGame. Claude writes the code, CI judges the results, and the owner makes the decisions.

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
├─ scripts/
│  ├─ bundle-bots.sh        every bot -> target/cg/<game>-<bot>.rs
│  ├─ cg-check.sh           size and standalone build of paste-ready files
│  ├─ new-release.sh        a bot -> games/<game>/releases/<game>-vNNN.rs
│  ├─ check-releases.sh     releases frozen and true to their sources
│  ├─ sprt.sh               smoke test and SPRT of a candidate file
│  └─ league.sh             Elo ratings of every release
├─ crates/
│  ├─ cg-core/              bot side: CodinGame input reading, seeded RNG, time scale
│  ├─ cg-arena/             Referee trait, match runner, tournaments, summary, SPRT, ratings, CLI
│  ├─ cg-bundler/           bot + its workspace crates -> one paste-ready .rs file
│  └─ cg-search/            bot side: game trait, MCTS, time budgets
├─ games/
│  └─ uttt/
│     ├─ README.md          layout, bots, how to play matches
│     ├─ RULES.md           the rules in our own words, with sources
│     ├─ evaluation.env     SPRT, league and arena settings
│     ├─ referee/           readable reference rules, implements Referee
│     ├─ arena/             uttt-arena binary
│     ├─ engine/            fast rules for bots, checked against the referee
│     ├─ bots/              first-valid/, random/, greedy/, wood/, rules-check/, mcts/
│     ├─ releases/          frozen paste-ready file per version, from the first release
│     └─ journal/           one entry per experiment
├─ .claude/                 Phase 5: skills, agents, settings
└─ .github/                 CI workflow, dependabot, PR template
```

## Components

- **`Referee` trait (`cg-arena`):** a game's rules as the arena sees them, in CodinGame's text protocol: which seats act this turn, the input to send each one, the time limits, and whether their answers are valid. Several seats may act in one turn, so simultaneous-move games fit too ([ADR 0011](adr/0011-framework-structure.md)).
- **Referee (per game):** a readable implementation of `RULES.md`, written to be obviously correct rather than fast. It is the reference that faster engines are checked against.
- **Engine (per game):** a fast implementation of the same rules for search inside bots, std-only so it can be bundled. Parity tests compare it with the referee on many random games, and a `rules-check` bot compares it with CodinGame's own valid actions during real games.
- **Arena:** each game has a tiny binary (`uttt-arena`) that hands its referee to the shared command line of `cg-arena`, with three commands: `match` (two bots), `sprt` (does a candidate beat a baseline?) and `league` (Elo ratings of several bots). Bots run as separate processes and receive their input on stdin, exactly as on CodinGame; a bot that exceeds its time limit, exits, or answers invalidly loses the game. Games run in parallel, in seat-swapped pairs, and each one is written as a JSON line: seed, bots by seat, winner, end reason, turns and answer times.
- **Bot:** an ordinary binary reading stdin and writing stdout. The arena gives each bot a reproducible seed in `CG_SEED`; on CodinGame it is absent and bots seed from the clock. When the arena scales the time limits, it passes the factor in `CG_TIME_SCALE` so that bots scale their budget too; `CG_FIXED_ITERS` replaces the time budget by a fixed iteration count for deterministic tests.
- **`cg-core`:** what every bot needs, std-only: a line-based reader for the referee's input, a seeded xoshiro256++ generator, and the arena's time scale.
- **`cg-search`:** game-independent search, std-only: a `Game` trait for two-player games where players take turns, Monte Carlo tree search (UCT) over it, and time budgets derived from a turn's CodinGame limit. Each game's engine implements the trait.
- **Bundler:** flattens a bot and the workspace crates it uses into one file formatted by rustfmt. It works on text so the result keeps its comments, and relies on conventions listed in its crate documentation: modules in files declared with `mod name;`, tests in separate files, other crates referred to by name, no crates.io dependencies on the bot side.
- **Versions:** `releases/` keeps the exact file pasted for each version. The arena compiles old versions from these files with `rustc` alone, so history never rebuilds differently.

Because the referee and the engine are written by the same hands, they could agree on a misread rule. Two checks guard against that: `RULES.md` cites the game's official source for every rule, and Phase 2 compares the referee with real CodinGame games.

## Evaluation pipeline

Seven tiers protect every change; only the SPRT decides whether a bot is stronger. The settings are in [ADR 0012](adr/0012-evaluation.md) and in each game's `evaluation.env`.

| Tier | Checks | Runs on | Passes when |
| --- | --- | --- | --- |
| 1. Static | Format, lints, unit tests, property tests of engine invariants | Every push | All green |
| 2. Parity | The fast engine against the reference referee on 10,000 random games | Every push (part of the tests) | Identical valid actions and results |
| 3. CodinGame compatibility | Bundle, size, standalone compile with Rust 1.90.0, 1,000 games between bundled bots, every bundled bot against random | Every push | Compiles, stays under 100 kB, no faults |
| 4. Smoke | Candidate release against the random bot, 100 pairs with openings | Pull requests adding a release, before the SPRT | No faults, at least 99% of the points |
| 5. Speed | Simulations per second on fixed positions | Every bot change (Phase 4) | No drop over 5% against the parent version (proposed) |
| 6. SPRT | Candidate release against the previous release, at 20 ms per move | Pull requests adding a release | The test accepts the candidate as stronger |
| 6b. Full-time confirmation | The accepted candidate against the previous release, 500 pairs at CodinGame's exact limits ([ADR 0014](adr/0014-full-time-confirmation.md)) | After the SPRT accepts | Not clearly weaker, no faults; the Elo measured is recorded |
| 7. League | Every release and the baseline bots | After each release, and on demand | Ratings published |

- **Openings and pairs:** Ultimate Tic-Tac-Toe has no random map, so the referee imposes a few random moves at the start of each pair, drawn from the pair's seed, and the pair plays them twice with seats swapped.
- **SPRT:** scored on game pairs, which handles draws; bounds of 0 and 10 Elo; 5% error rates; no verdict before 30 pairs; capped at 10,000 pairs (20,000 games). Bounds tighten as the bot matures.
- **Sanity checks of the pipeline itself:** on every push, CI runs an SPRT of a bot against a copy of itself (A/A test, must not be accepted), of a weaker bot against a stronger one (must be rejected) and the reverse (must be accepted), with fixed seeds.
- **Ratings:** Bradley-Terry maximum likelihood with 95% intervals, the random bot anchored at 0.
- **Timing noise:** matches run one game per CPU core, with time limits scaled down to keep tests affordable (0.2) plus a 5 ms tolerance, not told to bots, that absorbs the machine's delays ([ADR 0013](adr/0013-evaluation-time-limits.md)). Since the first release reached Legend, every accepted candidate is also confirmed at CodinGame's full limits.

## Journal and versioning

Every experiment gets a journal entry, including failures; only experiments that pass the SPRT become versions.

- **Candidate:** a pull request that adds `games/<game>/releases/<game>-vNNN.rs`, made by `scripts/new-release.sh`. CI checks that it is the bundle of its bot, numbered right after the last release, and that released files never change.
- **Entry:** one file per experiment in `games/<game>/journal/`, written in the experiment's pull request: the hypothesis and the change before the test, the SPRT result (posted as a comment on the pull request) after it.
- **Accepted:** the pull request is merged; a workflow tags the version (`uttt-v001`, `uttt-v002`, ...) and publishes a GitHub release with the paste-ready file.
- **Rejected or inconclusive:** the bot change and the release file are removed from the pull request and the entry is merged alone, so failed ideas stay on record.
- **CodinGame rank:** after pasting a version, the owner reports the rank and it is recorded in that version's entry.

The entry format is in each game's `journal/template.md`.

## CI/CD

GitHub Actions on free standard runners. `main` only accepts changes whose checks are green.

| Workflow | Trigger | Does |
| --- | --- | --- |
| `ci.yml` | Every push and pull request | Tiers 1 to 3: format, lint, tests (parity included); bundles every bot, checks and plays the bundles; checks new releases; evaluation sanity checks; engine speed; dependency policy |
| `sprt.yml` | Every pull request | Tiers 4 and 6 for each release file the pull request adds, verdict as a check and a comment; passes at once when there is none |
| `release.yml` | Push to `main` adding a release | Tag and GitHub release with the paste-ready file |
| `league.yml` | Push to `main` adding a release, and on demand | Tier 7, ratings in the run summary |

- **Compiler:** every job uses Rust 1.90.0, CodinGame's version ([ADR 0010](adr/0010-codingame-rust-toolchain.md)).
- **Branch protection:** a ruleset on `main` requires the CI checks and blocks direct pushes, without requiring a GitHub review ([ADR 0006](adr/0006-human-approves-merges.md)).
- **Compute:** standard runners only, which are free on public repositories. The SPRT runs in one job on all of the runner's cores, which keeps it sequential.
