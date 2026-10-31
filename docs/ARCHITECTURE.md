# Architecture

A reusable Rust framework that takes a two-player CodinGame game from rules to a Legend-level bot. Each game plugs into the arena through one `Referee` trait; every bot ships as a single file pasted into CodinGame. Claude writes the code, CI judges the results, and the owner makes the decisions.

This document describes the target design. [ROADMAP.md](ROADMAP.md) says which parts exist today.

## Principles

- **Framework over one-shot.** Every game reuses the same arena, tests, rating and release machinery. A new game adds its rules, an engine and bots.
- **Claude proposes, CI judges.** No bot counts as stronger until a sequential probability ratio test (SPRT) says so.
- **The owner decides.** Claude merges only what the owner approved ([ADR 0006](adr/0006-human-approves-merges.md), [ADR 0023](adr/0023-chat-approved-merges.md)); important choices become ADRs.
- **Free and reproducible.** Public repository, standard GitHub-hosted runners, pinned compiler, seeded games.
- **Paste-ready output.** Every bot version ships as one minified `.rs` file, and a readable copy with comments is kept beside it ([ADR 0045](adr/0045-minified-releases.md)).

## Layout

Items marked "Phase N" do not exist yet.

```text
codingame-bots/
├─ Cargo.toml               workspace: edition 2021, rust-version 1.90
├─ README.md                overview, the games, where things are
├─ CLAUDE.md                rules for every Claude session
├─ LICENSE-MIT, LICENSE-APACHE
├─ deny.toml                dependency license and source policy
├─ docs/                    this file, ROADMAP, WORKFLOW, CODINGAME, NEW_GAME (the new-game playbook), adr/, research/ (methods shared by games), templates/ (a game's roadmap, model and research)
├─ scripts/
│  ├─ bundle-bots.sh        every bot -> target/cg/<game>-<bot>.rs
│  ├─ cg-check.sh           size and standalone build of paste-ready files
│  ├─ new-release.sh        a bot -> games/<game>/releases/<game>-vNNN.rs (minified) and games/<game>/readable/<game>-vNNN.rs (with comments)
│  ├─ check-releases.sh     releases and readable copies frozen; true to their sources
│  ├─ check-determinism.sh  releases replay identically with a fixed seed and iterations; iteration rates
│  ├─ check-minified.sh     a minified release sends the same answers as its readable copy (ADR 0045)
│  ├─ build-studio-wasm.sh  the studio's rules program, the games' analysis programs, every release and every baseline bot -> WebAssembly (ADR 0030, 0032, 0040)
│  ├─ sprt.sh               smoke test and SPRT of a candidate file
│  ├─ league.sh             the league of each new release: opponents chosen by the arena, new result lines printed (ADR 0038)
│  ├─ league-replay.sh      the stored pairings of a league played again in shards, then merged in order (ADR 0048)
│  ├─ calibrate.sh          the time scale and tolerance of CodinGame's speed on this machine, from the calibration bot's workload (ADR 0048)
│  ├─ league-check.sh       the merge gate: a release has its pairings in league.jsonl and LEADERBOARD.md matches them (ADR 0038)
│  ├─ prune-branches.sh     deletes finished claude/ branches (ADR 0021)
│  ├─ arena-archive.sh      keeps each game's arena games in one artifact: plan, gather, prune (ADR 0031)
│  ├─ check-docs.sh         indexes, status lines and links match the repository; agents declare model and tools; workflows and scripts are listed
│  ├─ pr-hygiene.sh         pull request titles (ADR 0022) and labels from changed paths
│  ├─ ci-scopes.sh          which CI checks and SPRT games run for the paths a change touches, and the verdict of the aggregate jobs `CI passed` and `SPRT passed` (ADR 0029)
│  ├─ annotate.sh           runs a workflow step and reports its failure as an annotation
│  ├─ agent-guard.sh        hook of Claude's sessions: only the project's agents and the read-only built-in ones may start
│  ├─ session-setup.sh      hook of Claude's cloud sessions: installs shellcheck, as in CI
│  ├─ tests/                tests of the scripts
│  ├─ lib/evaluation.sh     settings and helpers of sprt.sh, league.sh, league-replay.sh, calibrate.sh, league-check.sh and new-release.sh; the calibration (ADR 0048)
│  └─ lib/records.sh        the comparison of game records of check-determinism.sh and check-minified.sh
├─ crates/
│  ├─ cg-core/              bot side: CodinGame input reading, seeded RNG, time scale
│  ├─ cg-arena/             Referee trait, match runner, tournaments, summary, SPRT, ratings, CLI
│  ├─ cg-bundler/           bot + its workspace crates -> one minified paste-ready .rs file (needs rustc 1.90.0) or a readable one
│  ├─ cg-pack/              bot side: decodes data packed into characters outside ASCII
│  ├─ cg-packer/            tool side: packs data for cg-pack to decode; cg-packer binary writes it as a Rust module
│  ├─ cg-review/            tool side: game files for reviewers (boards, metrics, phases, decisive moments) and a batch's summary
│  └─ cg-search/            bot side: game trait, MCTS, time budgets
├─ games/
│  └─ uttt/
│     ├─ README.md          layout, bots, how to play matches
│     ├─ RULES.md           the rules in our own words, with sources
│     ├─ MODEL.md           new games (ADR 0035): the problem modelled, the framework gaps; also ROADMAP.md, research/, RETROSPECTIVE.md
│     ├─ evaluation.env     SPRT, league and arena settings
│     ├─ ci.sh              the CI steps only this game needs: arena games, evaluation controls, engine speed (ADR 0029)
│     ├─ referee/           readable reference rules, implements Referee
│     ├─ arena/             uttt-arena binary
│     ├─ engine/            fast rules for bots, checked against the referee
│     ├─ analysis/          uttt-analysis: the analysis program, a library and a command (JSON request in, JSON lines of reports out), natively and as WebAssembly (ADR 0032)
│     ├─ policy/            uttt-policy: E011's class policy and E015's pattern policy and their searches, for the trainer only (bots no longer use them)
│     ├─ value/             uttt-value: E012's value network and its searches, for the trainer only (bots never use it)
│     ├─ trainer/           uttt-trainer binary: self-play data, training of the playout policy, value network and move models
│     ├─ bots/              first-valid/, random/, greedy/, wood/, rules-check/, mcts/
│     ├─ releases/          frozen paste-ready (minified) file per version
│     ├─ readable/          frozen readable copy, with comments, of each release made since ADR 0045
│     ├─ journal/           one entry per experiment, with an index
│     ├─ training/          reports of the training runs used, E011's class weights
│     └─ studio/            Phase 6: the game's adapter for the studio
├─ studio/                  Phase 6: local app to play and analyse games (ADR 0025)
│  ├─ core/                 the studio's logic that needs the rules, shared by the server and the browser (ADR 0030)
│  ├─ game/                 the interface a game implements for the studio
│  ├─ rules/                the rules as a WebAssembly program (JSON requests in, JSON out) for the browser
│  ├─ server/               Rust server on 127.0.0.1: bots, games, history; `studio-archive` builds an archive of arena games (ADR 0031)
│  └─ web/                  front end: TypeScript, React, Vite; a board renderer per game
├─ .claude/                 agents/ and skills/ of Claude's sessions; worktrees/ ignored
└─ .github/
   ├─ workflows/            ci, sprt, league, release, train, prune-branches, archive, pr-hygiene, pages
   ├─ dependabot.yml        monthly updates of actions, crates and npm
   └─ pull_request_template.md
```

## Components

- **`Referee` trait (`cg-arena`):** a game's rules as the arena sees them, in CodinGame's text protocol: which seats act this turn, the input to send each one, the time limits, and whether their answers are valid. Several seats may act in one turn, so simultaneous-move games fit too ([ADR 0011](adr/0011-framework-structure.md)).
- **Referee (per game):** a readable implementation of `RULES.md`, written to be obviously correct rather than fast. It is the reference that faster engines are checked against.
- **Engine (per game):** a fast implementation of the same rules for search inside bots, std-only so it can be bundled. Parity tests compare it with the referee on many random games, and a `rules-check` bot compares it with CodinGame's own valid actions during real games.
- **Arena:** each game has a tiny binary (`uttt-arena`) that hands its referee to the shared command line of `cg-arena`, with four commands: `match` (two bots), `sprt` (does a candidate beat a baseline?), `league` (Elo ratings of several bots) and `leaderboard` (writes a game's `LEADERBOARD.md` from the league's results file, or with `--check` tells whether the committed one matches it, without playing or running any bot). With `--results FILE`, `league` is incremental ([ADR 0038](adr/0038-incremental-league.md)): it reads the pairings stored in a JSON-lines results file, plays only a new release's pairings that are not stored (the previous release, fixed opponents and the bots nearest in rating), appends each one to `--new-results` as a line of the same format, rates every bot over stored and new pairings, and can replay one stored pairing as a canary (`--canary A,B`) to show whether the runners' speed has changed. Three more commands serve the nightly league ([ADR 0049](adr/0049-parallel-evaluations-and-nightly-league.md)): `plan` chooses a night's pairings from the results file (bots that lack games first, then the top, then the other bots) without playing, `play` plays the lines of a plan, or of one shard of it, and `merge` adds the new result lines to the results file, reporting drift when a pairing is played again, which the file now accepts. Bots run as separate processes and receive their input on stdin, exactly as on CodinGame; a bot that exceeds its time limit, exits, or answers invalidly loses the game. Games run in parallel, in seat-swapped pairs, and each one is written as a JSON line: seed, bots by seat, winner, end reason, turns and answer times. `--records DIR` (optionally with `--records-sample N`, about N games per pair of bots, each matchup of a league, plus every game lost by a fault) also writes full game records, a JSON file per game with every answer ([ADR 0027](adr/0027-game-records.md)). For the studio, `cg_arena::live` drives a game one turn at a time (a human can fill a seat) and restarts a bot by replaying a game into it.
- **Trainer (per game):** a tool, not part of any bot, that plays self-play games with the bot's search and fits what a bot can learn from them offline. For Ultimate Tic-Tac-Toe, `uttt-trainer` records the searched positions and fits the playout policy, the value network and the move models, in the Train workflow on GitHub's runners ([ADR 0016](adr/0016-self-play-training.md), [ADR 0017](adr/0017-value-network.md)). Its output is Rust source or text; it reaches a bot only through an experiment and its SPRT. Being a tool, it may use the crates.io dependencies `deny.toml` allows.
- **Bot:** an ordinary binary reading stdin and writing stdout. The arena gives each bot a reproducible seed in `CG_SEED`; on CodinGame it is absent and bots seed from the clock. When the arena scales the time limits, it passes the factor in `CG_TIME_SCALE` so that bots scale their budget too; `CG_FIXED_ITERS` replaces the time budget by a fixed iteration count for deterministic tests.
- **`cg-core`:** what every bot needs, std-only: a line-based reader for the referee's input, a seeded xoshiro256++ generator, and the arena's time scale.
- **`cg-pack` and `cg-packer`:** data a bot carries (weights, tables) packed into characters outside ASCII, 127 bytes to 64 characters, about 2.6 times as much as base64 under CodinGame's limit in characters ([research note](research/utf16-packing.md), [ADR 0037](adr/0037-size-limit-in-characters.md)). `cg-pack`, std-only, holds the alphabet and the decoder a bot calls once at start; `cg-packer` is the encoder, for tools such as trainers, and takes the alphabet from `cg-pack`, so the two cannot disagree; its command writes a file's bytes as a Rust module with one string constant. A bundler test proves a bot carrying packed data, over 100,000 bytes and under 100,000 characters, passes `scripts/cg-check.sh`. They are two crates because the bundler copies a whole crate into a bot, which then carries the decoder only.
- **`cg-review`:** reviews of recorded games, the same for every game. A game's review binary (`ice-and-fire-review`) replays a record with its engine into a trace: the board as text and both sides' metrics at every ply, the phases and the decisive moments it finds. `cg-review` writes from it the game file a reviewer reads (Markdown) and the metrics (CSV), finds the moments the metrics alone show (large drops, lead changes and swings), and summarizes a batch: which metrics go with the winner at three points of the game, how often the side a kind of moment favoured won, and whether the plays reviewers judged good or bad moved the lead as they said.
- **`cg-search`:** game-independent search, std-only: a `Game` trait for two-player games where players take turns, Monte Carlo tree search (UCT) over it, and time budgets derived from a turn's CodinGame limit. Each game's engine implements the trait.
- **Bundler:** flattens a bot and the workspace crates it uses into one file, checked and formatted by rustfmt, then minifies it to save room ([ADR 0016](adr/0016-self-play-training.md), [ADR 0045](adr/0045-minified-releases.md), [ADR 0046](adr/0046-one-minifier.md)), in three stages, each checked, none selectable. First it compiles the bundle with rustc 1.90.0 (CodinGame's version, found with rustup or named by `CG_RUSTC`, never replaced by another) and removes the items that rustc reports as never used, whole and with their attributes and doc comments, then compiles again until nothing more can go; a removal that rustc rejects is undone, and enum variants, struct fields, trait impls and `main` are never removed. Then it gives every name the bundle defines the shortest name of the same case not otherwise used, counted in UTF-16 units (the letters, then CJK ideographs from U+4E00, the most frequent names first, each name everywhere at once), keeping the names that std or the compiler may read (std's members after `.`, `::`, `fn` or `type`, std paths, imports and preludes, macros called with `!`, attributes, names in format strings; a std path's middle segments only where they are; the std names are listed in `crates/cg-bundler/src/rename/std-names.txt`, written from rustc 1.90.0's `rust-src` by the example `std-names`); in the same stage, methods that use `self` often start with `let s=self;`, modules declare aliases for common types where they save room (`type U=usize;`, `type V<T>=Vec<T>;`), and numbers lose their digit separators; the result is compiled, and each change that rustc rejects is undone alone. Last it lexes the code and writes it again with only the whitespace Rust needs between tokens and no comments, in lines of at most about 300 characters, after checking that the result has the same tokens as what it was given; string and character literals are copied byte for byte in every stage. The minified bundle is compiled once more. `--keep-comments` gives the readable bundle instead, with `#[allow(dead_code, unused_imports)]` on the library modules, which a release keeps beside its minified file and which tools that only compile a bot, like the studio, use; it needs no rustc 1.90.0. The bundler works on text and relies on conventions listed in its crate documentation: modules in files declared with `mod name;`, tests in separate files, other crates referred to by name, no crates.io dependencies on the bot side.
- **Studio (Phase 6):** an app in the browser, in the style of Lichess, served by a local server ([ADR 0025](adr/0025-studio.md)). With the local server, its Rust server is the only judge of the rules: a game is its seed and each turn's answers, any position is rebuilt by replaying the game's referee, and a human's legal moves come from the referee's turn input. Bots run as processes, as in the arena. Its computer seats are a game's releases and its baseline bots (`games/<game>/bots/`, bundled from the current code, [ADR 0040](adr/0040-baseline-bots-in-the-studio.md)), so a game can be watched before its first release. Against a human, a bot plays with a fixed seed and iterations, so a takeback restarts it and replays the game. Analysis comes from a program built from the current code with the latest release's settings, `games/<game>/analysis` ([ADR 0032](adr/0032-analysis-engine.md)): the browser engine runs it as WebAssembly in a Web Worker (`wasm/<game>/analysis.wasm`), the local server links it and runs it natively on a thread, both behind the routes `POST /api/analysis`, `GET /api/analysis/<id>` and `DELETE /api/analysis/<id>`, and the protocol is written in `studio/README.md`. Each game adds an adapter crate and a board renderer. Game records are versioned JSON, kept on the owner's computer ([ADR 0027](adr/0027-game-records.md)); the front end is built with Node ([ADR 0026](adr/0026-node-for-the-studio.md)). The same front end also runs as static files (GitHub Pages, `npm run preview`) with a browser engine that answers the server's API inside the page: the rules come from a WebAssembly program built from the same Rust code, each release and baseline bot runs as WebAssembly in a Web Worker, and saved games live in IndexedDB; the front end picks the engine at start-up, and the local server stays ([ADR 0030](adr/0030-studio-on-github-pages.md), roadmap phase 6, step F). The history also lists the "Arena games" read-only, beside the visitor's own: the games the releases played on GitHub, which the Pages build publishes as `arena-games/index.json` and `records/<id>.json` from the archives of [ADR 0031](adr/0031-arena-games-archive.md). The browser engine reads them from its own site, the local server's front end from the published site (it needs a connection); both filter them with `POST /api/filter`, show them through `POST /api/view`, and a copy saved to the history is an ordinary game of the visitor's.
- **Versions:** `releases/` keeps the exact file pasted for each version, and `readable/` the same bot with its comments and names for each version made since [ADR 0045](adr/0045-minified-releases.md); CI checks that the two play the same games. The arena compiles old versions from the files of `releases/` with `rustc` alone, so history never rebuilds differently.

Because the referee and the engine are written by the same hands, they could agree on a misread rule. Two checks guard against that: `RULES.md` cites the game's official source for every rule, and the `rules-check` bot compares the engine's valid actions with those of real CodinGame games (done for Ultimate Tic-Tac-Toe in Phase 2).

## Evaluation pipeline

Seven tiers protect every change; only the SPRT decides whether a bot is stronger. The settings are in [ADR 0012](adr/0012-evaluation.md) and in each game's `evaluation.env`.

| Tier | Checks | Runs on | Passes when |
| --- | --- | --- | --- |
| 1. Static | Format, lints, unit tests, property tests of engine invariants | Pushes to `main` and pull requests that change their area ([ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)) | All green |
| 2. Parity | The fast engine against the reference referee on 10,000 random games | Pushes to `main` and pull requests that change their area ([ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)) (part of the tests) | Identical valid actions and results |
| 3. CodinGame compatibility | Bundle, size, standalone compile with Rust 1.90.0, 1,000 games between bundled bots, every bundled bot against random | Pushes to `main` and pull requests that change their area ([ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)) | Compiles, stays within 100,000 characters ([ADR 0037](adr/0037-size-limit-in-characters.md)), no faults except timeouts in up to 1% of games for bots that search ([ADR 0015](adr/0015-tolerate-rare-timeouts.md)) |
| 4. Smoke | Candidate release against the random bot, 100 pairs with openings, at the SPRT's time limits | Pull requests adding a release, before the SPRT | No crash or invalid answer, timeouts in at most 1% of games, at least 99% of the points |
| 5. Speed | Simulations per second on fixed positions | Every bot change (planned in phase 5) | No drop over 5% against the parent version (proposed) |
| 6. SPRT | Candidate release against the previous release, at CodinGame's exact limits ([ADR 0042](adr/0042-official-time-limits-and-800-pairs.md)); for A Code of Ice and Fire, at CodinGame's speed and real limit, calibrated on the runner ([ADR 0048](adr/0048-ice-and-fire-codingame-speed.md)) | Pull requests adding a release | The test accepts the candidate as stronger, with no crash or invalid answer and timeouts in at most 1% of games |
| 6b. Full-time confirmation | Not played since [ADR 0042](adr/0042-official-time-limits-and-800-pairs.md), as the SPRT plays at full time (`CONFIRM_PAIRS=0`): the accepted candidate against the previous release at CodinGame's exact limits ([ADR 0014](adr/0014-full-time-confirmation.md)) | After the SPRT accepts | Not clearly weaker, no crash or invalid answer, timeouts in at most 1% of games; the Elo measured is recorded |
| 7. League | Each new release against its previous release, the fixed opponents and the bots nearest in rating, at the SPRT's time limits ([ADR 0038](adr/0038-incremental-league.md)) | On the release's pull request, after its SPRT accepts; the bootstrap and replays on demand | New result lines in a comment, copied into `games/<game>/league.jsonl`; `LEADERBOARD.md` generated from the file; the release's faults and the results' presence are checked |

- **Openings and pairs:** Ultimate Tic-Tac-Toe has no random map, so the referee imposes a few random moves at the start of each pair, drawn from the pair's seed, and the pair plays them twice with seats swapped.
- **SPRT:** scored on game pairs, which handles draws; bounds of 0 and 10 Elo; 5% error rates; no verdict before 30 pairs; capped at 8,000 pairs for Ultimate Tic-Tac-Toe, which fits GitHub's 6-hour job limit at full time. Bounds tighten as the bot matures.
- **Sanity checks of the pipeline itself:** on every pull request and push to `main`, CI runs an SPRT of a bot against a copy of itself (A/A test, must not be accepted), of a weaker bot against a stronger one (must be rejected) and the reverse (must be accepted), with fixed seeds.
- **Ratings:** Bradley-Terry maximum likelihood with 95% intervals, the random bot anchored at 0.
- **Timing noise:** matches run one game per CPU (GitHub's runners have 4 CPUs on 2 physical cores). Every strength evaluation of Ultimate Tic-Tac-Toe (smoke test, SPRT, league, local screening) plays at CodinGame's exact limits, time scale 1 and no tolerance ([ADR 0042](adr/0042-official-time-limits-and-800-pairs.md), which supersedes the scale of 0.2 plus 5 ms of [ADR 0013](adr/0013-evaluation-time-limits.md) and [ADR 0020](adr/0020-full-time-sprt-in-legend.md)). A Code of Ice and Fire's play at CodinGame's speed and real limit, calibrated on each machine ([ADR 0048](adr/0048-ice-and-fire-codingame-speed.md)): with `TIME_SCALE=auto` in its `evaluation.env`, the scripts first run the timing bot's workload as many times at once as the games they play, then set the scale to CodinGame's speed over the machine's and the tolerance to CodinGame's 13 ms beyond its 50 at that scale, and record both and the speed with each result. A league pairing plays 800 pairs.

## Journal and versioning

Every experiment gets a journal entry, including failures; only experiments that pass the SPRT become versions.

- **Candidate:** a pull request that adds `games/<game>/releases/<game>-vNNN.rs` and its readable copy `games/<game>/readable/<game>-vNNN.rs`, made by `scripts/new-release.sh`. CI checks that they are the minified bundle and the bundle with comments of its bot, numbered right after the last release, that they play the same games, and that released files never change.
- **Entry:** one file per experiment in `games/<game>/journal/`, written in the experiment's pull request: the hypothesis and the change before the test, the SPRT result (posted as a comment on the pull request) after it.
- **Accepted:** the pull request is merged; a workflow tags the version (`uttt-v001`, `uttt-v002`, ...) and publishes a GitHub release with the paste-ready file and the readable copy.
- **Rejected or inconclusive:** the bot change and the release file are removed from the pull request and the entry is merged alone, so failed ideas stay on record.
- **CodinGame rank:** a version Claude submits through CodinGame's API on the owner's approval has its rank read and recorded in its entry when the ranking games end ([ADR 0047](adr/0047-codingame-api.md)); a rank the owner reports after pasting a version himself is recorded the same way ([ADR 0036](adr/0036-ranks-optional.md)).

The entry format is in each game's `journal/template.md`.

## CI/CD

GitHub Actions on free standard runners. `main` only accepts changes whose checks are green.

| Workflow | Trigger | Does |
| --- | --- | --- |
| `ci.yml` | Pushes to `main`, every pull request, by hand | Runs per area and per game: `scripts/ci-scopes.sh` maps the changed paths to jobs, and pushes to `main` and runs by hand run everything ([ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)). Jobs: `Changes`; format and lint of the workspace; the framework's tests (also when a game changes, since the bundler's tests bundle its bots); for each game, its tests (parity included) and its bots (bundles every bot, checks and plays the bundles, `games/<game>/ci.sh` with the evaluation sanity checks and the engine speed report, new releases checked, replayed identically with a fixed seed and iterations, and played against their readable copies); dependency policy; the studio's tests (every game's adapter included) and the build of the rules and the releases to WebAssembly; the studio's front end (package licenses, format, lint, types, the build of the rules and the releases to WebAssembly that the browser engine's tests run, unit tests, build, license notices, browser tests; [ADR 0026](adr/0026-node-for-the-studio.md)); the studio's tests, a release build and a bot-against-bot game through the server on Windows ([ADR 0025](adr/0025-studio.md)); docs indexes, status lines and links, shellcheck, actionlint and the scripts' tests, on every pull request; and `CI passed`, which gathers them all |
| `sprt.yml` | Every pull request | Per game, in a job named after it ([ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)): tiers 4 and 6 for each release file the pull request adds to that game, verdict as a check and a comment; a result is kept for the framework's code and that game's code only (Markdown and `league.jsonl` left out). After the SPRT accepts, a second job per game plays the league of the new release and posts the new result lines ([ADR 0038](adr/0038-incremental-league.md)), and the job `League results` (`scripts/league-check.sh`) fails until the lines are in `games/<game>/league.jsonl` and `LEADERBOARD.md` matches them. The aggregate check `SPRT passed` also gathers both and passes at once when there is no new release and no league file changed. The SPRT and the league play at CodinGame's full time, the league job within 300 minutes ([ADR 0042](adr/0042-official-time-limits-and-800-pairs.md)) |
| `league.yml` | By hand only | The bootstrap and replays of the league ([ADR 0038](adr/0038-incremental-league.md), decision 8): plays the bots listed, in order, in one job until a time budget, and prints each one's new result lines in its log and summary; read-only, it changes nothing in the repository. The league of a new release runs in `sprt.yml` |
| `league-replay.yml` | By hand only | The whole league of a game played again at its current settings ([ADR 0048](adr/0048-ice-and-fire-codingame-speed.md), decision 5): the stored pairings split over up to 20 parallel jobs (15 by default), each calibrated on its runner and replaying its pairings with their stored seeds (`scripts/league-replay.sh`), then merged in the stored order with the leaderboard they give, printed in the log and summary; read-only, it changes nothing in the repository |
| `release.yml` | Push to `main` adding a release | Tag and GitHub release with the paste-ready file, and the release's readable copy as `<name>-readable.rs` when it has one ([ADR 0045](adr/0045-minified-releases.md)) |
| `train.yml` | By hand | Self-play in up to 20 parallel jobs, then the fit of the playout policy, the value network or the move models; results and report go to a branch `claude/train/<run id>` ([ADR 0016](adr/0016-self-play-training.md)) |
| `prune-branches.yml` | Weekly (Monday), and by hand | Deletes finished `claude/` branches once their content is safe; a run by hand only lists them unless asked to delete ([ADR 0021](adr/0021-prune-finished-branches.md)) |
| `archive.yml` | A push to `main` that adds a release, weekly (Wednesday), and by hand | Keeps one artifact `arena-games-<game>` per game ([ADR 0031](adr/0031-arena-games-archive.md), `scripts/arena-archive.sh`): the records of the archive, and the latest league sample and the latest SPRT sample still kept from the release's pull request branch ([ADR 0038](adr/0038-incremental-league.md)), each checked by `studio-archive`, uploaded for 90 days with the older copies deleted; the weekly run uploads the archive again when it expires in less than 30 days, and the first archive starts from the release's samples. Then asks `pages.yml` to rebuild. A run by hand uploads a dry-run copy only, deleting nothing, unless asked otherwise |
| `pages.yml` | Push to `main` changing the studio, a game's releases, analysis program, engine, bots, referee or studio adapter, `cg-arena`, `cg-core`, the lockfile or the manifest, or this workflow; by hand | Builds the studio's rules, the games' analysis programs, every release and every baseline bot to WebAssembly, gathers the arena games from the archive artifacts (`scripts/arena-archive.sh site`, which needs the job's `actions: read` permission and only warns when there is no archive) into `arena-games/` of the site, builds the front end and the license notices (`THIRD-PARTY-NOTICES.txt`), then deploys `studio/web/dist` to GitHub Pages ([ADR 0030](adr/0030-studio-on-github-pages.md)); the repository's Pages source must be GitHub Actions. Pull requests never deploy |
| `pr-hygiene.yml` | Every pull request: opened, edited, new commits | Labels it by the paths it changes and checks its title ([ADR 0022](adr/0022-scopes-and-names.md)); Dependabot's are skipped, forks get the title check only |

- **Node:** the `Studio: front end` job uses the Node line written in `studio/web/package.json` (`engines`), and installs with `npm ci` ([ADR 0026](adr/0026-node-for-the-studio.md)).
- **Compiler:** every job that builds Rust uses 1.90.0, CodinGame's version ([ADR 0010](adr/0010-codingame-rust-toolchain.md)).
- **Branch protection:** a ruleset on `main` requires `CI passed` (the aggregate of `ci.yml`, [ADR 0029](adr/0029-ci-scoped-by-area-and-game.md)), `SPRT passed` (the aggregate of `sprt.yml`) and `Pull request title and labels`, so a rejected release cannot be merged. Each aggregate always runs and fails when a job it gathers failed, was cancelled, or was skipped although its area changed, so every job of `ci.yml`, the studio's included, and every game's SPRT is blocking through them; jobs can change without touching the ruleset. It blocks direct pushes, without requiring a GitHub review: the owner approves merges in the conversation ([ADR 0006](adr/0006-human-approves-merges.md), [ADR 0023](adr/0023-chat-approved-merges.md)).
- **Compute:** standard runners only, which are free on public repositories. The SPRT runs in one job on all of the runner's cores, which keeps it sequential; training spreads self-play over up to 20 jobs.
