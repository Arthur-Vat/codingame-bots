# Architecture

A reusable Rust framework that takes a two-player CodinGame game from rules to a Legend-level bot. Each game plugs into the arena through one `Referee` trait; every bot ships as a single file pasted into CodinGame. Claude writes the code, CI judges the results, and the owner makes the decisions.

This document describes the target design. [ROADMAP.md](ROADMAP.md) says which parts exist today.

## Principles

- **Framework over one-shot.** Every game reuses the same arena, tests, rating and release machinery. A new game adds its rules, an engine and bots.
- **Claude proposes, CI judges.** No bot counts as stronger until a sequential probability ratio test (SPRT) says so.
- **The owner decides.** Claude merges only what the owner approved ([ADR 0006](adr/0006-human-approves-merges.md), [ADR 0023](adr/0023-chat-approved-merges.md)); important choices become ADRs.
- **Free and reproducible.** Public repository, standard GitHub-hosted runners, pinned compiler, seeded games.
- **Paste-ready output.** Every bot version ships as one readable `.rs` file.

## Layout

Items marked "Phase N" do not exist yet.

```text
codingame-bots/
├─ Cargo.toml               workspace: edition 2021, rust-version 1.90
├─ README.md                overview, the games, where things are
├─ CLAUDE.md                rules for every Claude session
├─ LICENSE-MIT, LICENSE-APACHE
├─ deny.toml                dependency license and source policy
├─ docs/                    this file, ROADMAP, WORKFLOW, CODINGAME, adr/
├─ scripts/
│  ├─ bundle-bots.sh        every bot -> target/cg/<game>-<bot>.rs
│  ├─ cg-check.sh           size and standalone build of paste-ready files
│  ├─ new-release.sh        a bot -> games/<game>/releases/<game>-vNNN.rs
│  ├─ check-releases.sh     releases frozen and true to their sources
│  ├─ sprt.sh               smoke test and SPRT of a candidate file
│  ├─ league.sh             Elo ratings of every release
│  ├─ prune-branches.sh     deletes finished claude/ branches (ADR 0021)
│  ├─ check-docs.sh         indexes, status lines and links match the repository; agents declare model and tools
│  ├─ pr-hygiene.sh         pull request titles (ADR 0022) and labels from changed paths
│  ├─ agent-guard.sh        hook of Claude's sessions: only the project's agents and the read-only built-in ones may start
│  ├─ tests/                tests of the scripts
│  └─ lib/evaluation.sh     settings and helpers of sprt.sh, league.sh and new-release.sh
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
│     ├─ trainer/           uttt-trainer binary: self-play data, training of the playout policy, value network and move models
│     ├─ bots/              first-valid/, random/, greedy/, wood/, rules-check/, mcts/
│     ├─ releases/          frozen paste-ready file per version
│     ├─ journal/           one entry per experiment, with an index
│     └─ training/          reports of the training runs used, E011's class weights
├─ .claude/                 agents/ and skills/ of Claude's sessions; worktrees/ ignored
└─ .github/
   ├─ workflows/            ci, sprt, league, release, train, prune-branches, pr-hygiene
   ├─ dependabot.yml        monthly updates of actions and crates
   └─ pull_request_template.md
```

## Components

- **`Referee` trait (`cg-arena`):** a game's rules as the arena sees them, in CodinGame's text protocol: which seats act this turn, the input to send each one, the time limits, and whether their answers are valid. Several seats may act in one turn, so simultaneous-move games fit too ([ADR 0011](adr/0011-framework-structure.md)).
- **Referee (per game):** a readable implementation of `RULES.md`, written to be obviously correct rather than fast. It is the reference that faster engines are checked against.
- **Engine (per game):** a fast implementation of the same rules for search inside bots, std-only so it can be bundled. Parity tests compare it with the referee on many random games, and a `rules-check` bot compares it with CodinGame's own valid actions during real games.
- **Arena:** each game has a tiny binary (`uttt-arena`) that hands its referee to the shared command line of `cg-arena`, with three commands: `match` (two bots), `sprt` (does a candidate beat a baseline?) and `league` (Elo ratings of several bots). Bots run as separate processes and receive their input on stdin, exactly as on CodinGame; a bot that exceeds its time limit, exits, or answers invalidly loses the game. Games run in parallel, in seat-swapped pairs, and each one is written as a JSON line: seed, bots by seat, winner, end reason, turns and answer times.
- **Trainer (per game):** a tool, not part of any bot, that plays self-play games with the bot's search and fits what a bot can learn from them offline. For Ultimate Tic-Tac-Toe, `uttt-trainer` records the searched positions and fits the playout policy, the value network and the move models, in the Train workflow on GitHub's runners ([ADR 0016](adr/0016-self-play-training.md), [ADR 0017](adr/0017-value-network.md)). Its output is Rust source or text; it reaches a bot only through an experiment and its SPRT. Being a tool, it may use the crates.io dependencies `deny.toml` allows.
- **Bot:** an ordinary binary reading stdin and writing stdout. The arena gives each bot a reproducible seed in `CG_SEED`; on CodinGame it is absent and bots seed from the clock. When the arena scales the time limits, it passes the factor in `CG_TIME_SCALE` so that bots scale their budget too; `CG_FIXED_ITERS` replaces the time budget by a fixed iteration count for deterministic tests.
- **`cg-core`:** what every bot needs, std-only: a line-based reader for the referee's input, a seeded xoshiro256++ generator, and the arena's time scale.
- **`cg-search`:** game-independent search, std-only: a `Game` trait for two-player games where players take turns, Monte Carlo tree search (UCT) over it, and time budgets derived from a turn's CodinGame limit. Each game's engine implements the trait.
- **Bundler:** flattens a bot and the workspace crates it uses into one file, checked and formatted by rustfmt, then removes comments, indentation and blank lines to save room ([ADR 0016](adr/0016-self-play-training.md); `--keep-comments` gives a readable bundle). It works on text and relies on conventions listed in its crate documentation: modules in files declared with `mod name;`, tests in separate files, other crates referred to by name, no crates.io dependencies on the bot side.
- **Versions:** `releases/` keeps the exact file pasted for each version. The arena compiles old versions from these files with `rustc` alone, so history never rebuilds differently.

Because the referee and the engine are written by the same hands, they could agree on a misread rule. Two checks guard against that: `RULES.md` cites the game's official source for every rule, and the `rules-check` bot compares the engine's valid actions with those of real CodinGame games (done for Ultimate Tic-Tac-Toe in Phase 2).

## Evaluation pipeline

Seven tiers protect every change; only the SPRT decides whether a bot is stronger. The settings are in [ADR 0012](adr/0012-evaluation.md) and in each game's `evaluation.env`.

| Tier | Checks | Runs on | Passes when |
| --- | --- | --- | --- |
| 1. Static | Format, lints, unit tests, property tests of engine invariants | Every pull request and push to `main` | All green |
| 2. Parity | The fast engine against the reference referee on 10,000 random games | Every pull request and push to `main` (part of the tests) | Identical valid actions and results |
| 3. CodinGame compatibility | Bundle, size, standalone compile with Rust 1.90.0, 1,000 games between bundled bots, every bundled bot against random | Every pull request and push to `main` | Compiles, stays under 100 kB, no faults except timeouts in up to 1% of games for bots that search ([ADR 0015](adr/0015-tolerate-rare-timeouts.md)) |
| 4. Smoke | Candidate release against the random bot, 100 pairs with openings, at the SPRT's time limits | Pull requests adding a release, before the SPRT | No crash or invalid answer, timeouts in at most 1% of games, at least 99% of the points |
| 5. Speed | Simulations per second on fixed positions | Every bot change (planned in phase 5) | No drop over 5% against the parent version (proposed) |
| 6. SPRT | Candidate release against the previous release: at CodinGame's exact limits once the game's bot is in Legend ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)), else at 20 ms per move | Pull requests adding a release | The test accepts the candidate as stronger, with no crash or invalid answer and timeouts in at most 1% of games |
| 6b. Full-time confirmation | For games below Legend that ask for it: the accepted candidate against the previous release at CodinGame's exact limits ([ADR 0014](adr/0014-full-time-confirmation.md)) | After the SPRT accepts | Not clearly weaker, no crash or invalid answer, timeouts in at most 1% of games; the Elo measured is recorded |
| 7. League | Every release and the baseline bots | After each release or change of evaluation settings, on pull requests that change the league, and on demand | Ratings published |

- **Openings and pairs:** Ultimate Tic-Tac-Toe has no random map, so the referee imposes a few random moves at the start of each pair, drawn from the pair's seed, and the pair plays them twice with seats swapped.
- **SPRT:** scored on game pairs, which handles draws; bounds of 0 and 10 Elo; 5% error rates; no verdict before 30 pairs; capped at 8,000 pairs for Ultimate Tic-Tac-Toe, which fits GitHub's 6-hour job limit at full time. Bounds tighten as the bot matures.
- **Sanity checks of the pipeline itself:** on every pull request and push to `main`, CI runs an SPRT of a bot against a copy of itself (A/A test, must not be accepted), of a weaker bot against a stronger one (must be rejected) and the reverse (must be accepted), with fixed seeds.
- **Ratings:** Bradley-Terry maximum likelihood with 95% intervals, the random bot anchored at 0.
- **Timing noise:** matches run one game per CPU core. Below Legend, time limits are scaled down to keep tests affordable (0.2) plus a 5 ms tolerance, not told to bots, that absorbs the machine's delays ([ADR 0013](adr/0013-evaluation-time-limits.md)). Once a game's bot is in Legend, its smoke test and SPRT play at CodinGame's exact limits ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)); the league keeps the scaled limits.

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
| `ci.yml` | Pushes to `main`, every pull request, by hand | Tiers 1 to 3: format, lint, tests (parity included); bundles every bot, checks and plays the bundles; checks new releases; evaluation sanity checks; engine speed; dependency policy; docs indexes, status lines and links, shellcheck and actionlint |
| `sprt.yml` | Every pull request | Tiers 4 and 6 for each release file the pull request adds, verdict as a check and a comment; passes at once when there is none. For a game in Legend the SPRT plays at CodinGame's full time ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)) |
| `league.yml` | Push to `main` changing releases or evaluation settings, pull requests changing the league itself, by hand | Tier 7, ratings of all releases in the run summary |
| `release.yml` | Push to `main` adding a release | Tag and GitHub release with the paste-ready file |
| `train.yml` | By hand | Self-play in up to 20 parallel jobs, then the fit of the playout policy, the value network or the move models; results and report go to a branch `claude/train/<run id>` ([ADR 0016](adr/0016-self-play-training.md)) |
| `prune-branches.yml` | Weekly (Monday), and by hand | Deletes finished `claude/` branches once their content is safe; a run by hand only lists them unless asked to delete ([ADR 0021](adr/0021-prune-finished-branches.md)) |
| `pr-hygiene.yml` | Every pull request: opened, edited, new commits | Labels it by the paths it changes and checks its title ([ADR 0022](adr/0022-scopes-and-names.md)); Dependabot's are skipped, forks get the title check only |

- **Compiler:** every job that builds Rust uses 1.90.0, CodinGame's version ([ADR 0010](adr/0010-codingame-rust-toolchain.md)).
- **Branch protection:** a ruleset on `main` requires five checks: the four of `ci.yml` (format, lint and test; CodinGame compatibility; dependency licenses and sources; docs and scripts) and the SPRT's, so a rejected release cannot be merged. It blocks direct pushes, without requiring a GitHub review: the owner approves merges in the conversation ([ADR 0006](adr/0006-human-approves-merges.md), [ADR 0023](adr/0023-chat-approved-merges.md)).
- **Compute:** standard runners only, which are free on public repositories. The SPRT runs in one job on all of the runner's cores, which keeps it sequential; training spreads self-play over up to 20 jobs.
