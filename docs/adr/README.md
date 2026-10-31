# Architecture decision records

Each file records one decision: the context, what was decided and what follows from it. A decision is never edited away: to change it, add a new record that supersedes the old one and set the old one's status to "superseded by NNNN".

Use [template.md](template.md) for new records. Number them in order, in one sequence for the whole repository. Each record names its scope on its `Scope:` line: `framework`, or the game it binds ([ADR 0022](0022-scopes-and-names.md)). The index groups them by scope: framework records under "Framework", a game's records under a heading that ends with the game's identifier in code, such as "Ultimate Tic-Tac-Toe (`uttt`)". `scripts/check-docs.sh` checks both in CI.

## Framework

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-license.md) | License: MIT OR Apache-2.0 | Accepted |
| [0002](0002-public-repo.md) | Public repository on a personal GitHub account | Accepted |
| [0003](0003-rust.md) | Rust for engines, bots and tools | Accepted |
| [0004](0004-monorepo.md) | One repository, one Cargo workspace | Accepted |
| [0005](0005-first-game-uttt.md) | Ultimate Tic-Tac-Toe as the first game | Accepted |
| [0006](0006-human-approves-merges.md) | A human approves every merge | Accepted; completed by 0023 |
| [0007](0007-english.md) | English for all artifacts | Accepted |
| [0008](0008-claude-hub.md) | A Claude conversation is the hub | Accepted |
| [0009](0009-manual-submission.md) | Manual submission to CodinGame | Superseded by 0047; the rank reported after each submission superseded earlier by 0036 |
| [0010](0010-codingame-rust-toolchain.md) | Target CodinGame's Rust: 1.90.0, edition 2021 | Accepted |
| [0011](0011-framework-structure.md) | Framework structure: text-protocol referees, reference rules, textual bundler | Accepted; the bundle's comments superseded by 0016 |
| [0012](0012-evaluation.md) | Evaluation: SPRT on game pairs, seeded openings, frozen releases | Accepted; decision 7 superseded by 0013, part of decision 5 by 0015; decisions 10 and 12 completed by 0029; the league of decision 12 superseded by 0038; decision 9 completed by 0045 |
| [0013](0013-evaluation-time-limits.md) | Evaluation time limits: 20 ms plus 5 ms of tolerance, full time for strong bots | Accepted; decision 3 put in place by 0014, part of decision 1 superseded by 0015, and for games in Legend by 0020 and 0038; decisions 1 and 3 superseded by 0042 |
| [0014](0014-full-time-confirmation.md) | Confirm accepted candidates at CodinGame's time limits | Accepted; part of decision 3 superseded by 0015; superseded for games in Legend by 0020, and for all games by 0042 |
| [0015](0015-tolerate-rare-timeouts.md) | Tolerate rare timeouts: up to 1% of games, counted as losses | Accepted |
| [0016](0016-self-play-training.md) | Train from self-play on GitHub Actions, with a pure-Rust tool | Accepted; decision 2 completed by 0021, decision 4 by 0045 |
| [0020](0020-full-time-sprt-in-legend.md) | In Legend, the SPRT plays at CodinGame's time limits | Accepted; the league's part of decision 5 superseded by 0038; decisions 2, 3 and 5 by 0042 |
| [0021](0021-prune-finished-branches.md) | Delete finished branches, keeping their content | Accepted |
| [0022](0022-scopes-and-names.md) | Scopes for decision records and pull requests | Accepted; decision 2 completed by 0025 |
| [0023](0023-chat-approved-merges.md) | The owner approves merges in the conversation; Claude carries them out | Accepted |
| [0024](0024-handoff-branch.md) | A handoff file on a standing branch carries state between sessions | Accepted; the usage line of decision 3 superseded by 0034 |
| [0025](0025-studio.md) | A local studio to play and review games in the browser | Accepted; decision 1 superseded by 0030; decisions 2, 3, 4 and 5 completed by 0030, decision 6 by 0032, decision 4 also by 0040 |
| [0026](0026-node-for-the-studio.md) | Node and npm build the studio's front end | Accepted; decision 3 completed by 0030 |
| [0027](0027-game-records.md) | Game records: one format, a sample of each run, kept on the owner's computer | Accepted; decision 3 completed by 0030, decisions 2 and 3 by 0031 |
| [0028](0028-job-logs-from-sessions.md) | Job logs readable from Claude's sessions | Accepted |
| [0029](0029-ci-scoped-by-area-and-game.md) | CI scoped by area and game | Accepted; the league's part of decision 6 superseded by 0038 |
| [0030](0030-studio-on-github-pages.md) | The studio also runs in the browser, published on GitHub Pages | Accepted; decision 6 completed by 0031, decision 2 by 0032, decision 4 by 0040 |
| [0031](0031-arena-games-archive.md) | Arena games: one archive per game on GitHub, shown read-only in the studio | Accepted; the league sample's source in decision 2 superseded by 0038; decision 2's league sample at publication superseded by 0049 |
| [0032](0032-analysis-engine.md) | Analysis: one program per game, run as WebAssembly in the page or natively by the local server | Accepted |
| [0033](0033-no-daily-scheduled-session.md) | No daily scheduled session | Accepted; the usage line of decision 3 superseded by 0034 |
| [0034](0034-no-usage-monitoring.md) | No usage monitoring | Accepted |
| [0035](0035-new-game-lifecycle.md) | A lifecycle for each new game | Accepted |
| [0036](0036-ranks-optional.md) | CodinGame ranks are reported only when the owner wishes | Accepted; decision 1 superseded by 0047 |
| [0037](0037-size-limit-in-characters.md) | Count a bot's size in characters, and pack its data with Rust tools | Accepted |
| [0038](0038-incremental-league.md) | An incremental league at full time, with a leaderboard kept in the repository | Accepted; the 100 pairs of decision 2 and the cost estimate in its consequences superseded by 0042; decision 8 superseded for A Code of Ice and Fire's replay by 0048; decision 1's played-once rule, decision 6's league on the release PR, decision 7 and decision 9's archive source superseded by 0049; decision 4's anchor superseded by 0050 |
| [0040](0040-baseline-bots-in-the-studio.md) | The studio also plays each game's baseline bots | Accepted |
| [0042](0042-official-time-limits-and-800-pairs.md) | Every strength evaluation at CodinGame's time limits, and 800 pairs per league pairing | Accepted; decision 1, and the order of A Code of Ice and Fire's replay in decision 6, superseded for that game by 0048; decision 4, and decision 1 for Ultimate Tic-Tac-Toe once calibrated, superseded by 0049 |
| [0043](0043-game-reviews-by-haiku.md) | Game reviews written by Haiku agents from game files | Accepted; decision 5 completed by 0044 |
| [0044](0044-metrics-reports.md) | Metrics reports: large batches measured by code, interpreted by Haiku | Accepted |
| [0045](0045-minified-releases.md) | Minified releases, each with a readable copy | Accepted; decision 2's three levels and decision 3 superseded by 0046 |
| [0046](0046-one-minifier.md) | One minifier, with names outside ASCII and aliases | Accepted |
| [0047](0047-codingame-api.md) | CodinGame's public API for test games, submissions and ranks | Accepted |
| [0049](0049-parallel-evaluations-and-nightly-league.md) | Evaluations spread over runners, and a nightly league for every game | Accepted; decision 4's shared budget and decision 4.4's ±15 Elo superseded by 0050 |
| [0050](0050-league-anchor-target-and-budget.md) | Every league rated from the game's first release, a nightly target of ±10 Elo, and a nightly budget per game | Accepted |

## Ultimate Tic-Tac-Toe (`uttt`)

| # | Decision | Status |
| --- | --- | --- |
| [0017](0017-value-network.md) | Stage 2: a value network instead of playouts | Accepted; decision 5 superseded by 0018 |
| [0018](0018-value-network-gate.md) | Judge the value network by games at equal iterations | Accepted; decision 1 superseded by 0019 |
| [0019](0019-value-network-gate-at-time.md) | Judge the value network at the bot's time budget | Accepted |

## A Code of Ice and Fire (`ice-and-fire`)

| # | Decision | Status |
| --- | --- | --- |
| [0039](0039-ice-and-fire-maps-and-studio.md) | A Code of Ice and Fire: our own map generator, and replay before human play in the studio | Accepted; decisions 1 and 3 superseded by 0041 |
| [0041](0041-ice-and-fire-source-maps-from-a-spec.md) | A Code of Ice and Fire: the source's maps, rebuilt from a written spec | Accepted |
| [0048](0048-ice-and-fire-codingame-speed.md) | A Code of Ice and Fire: evaluations at CodinGame's speed and real limit, calibrated on each machine | Accepted |
