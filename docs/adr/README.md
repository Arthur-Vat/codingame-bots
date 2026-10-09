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
| [0006](0006-human-approves-merges.md) | A human approves every merge | Accepted |
| [0007](0007-english.md) | English for all artifacts | Accepted |
| [0008](0008-claude-hub.md) | A Claude conversation is the hub | Accepted |
| [0009](0009-manual-submission.md) | Manual submission to CodinGame | Accepted |
| [0010](0010-codingame-rust-toolchain.md) | Target CodinGame's Rust: 1.90.0, edition 2021 | Accepted |
| [0011](0011-framework-structure.md) | Framework structure: text-protocol referees, reference rules, textual bundler | Accepted; the bundle's comments superseded by 0016 |
| [0012](0012-evaluation.md) | Evaluation: SPRT on game pairs, seeded openings, frozen releases | Accepted; decision 7 superseded by 0013, part of decision 5 by 0015 |
| [0013](0013-evaluation-time-limits.md) | Evaluation time limits: 20 ms plus 5 ms of tolerance, full time for strong bots | Accepted; decision 3 put in place by 0014, part of decision 1 superseded by 0015, and for games in Legend by 0020 |
| [0014](0014-full-time-confirmation.md) | Confirm accepted candidates at CodinGame's time limits | Accepted; part of decision 3 superseded by 0015; superseded for games in Legend by 0020 |
| [0015](0015-tolerate-rare-timeouts.md) | Tolerate rare timeouts: up to 1% of games, counted as losses | Accepted |
| [0016](0016-self-play-training.md) | Train from self-play on GitHub Actions, with a pure-Rust tool | Accepted; decision 2 completed by 0021 |
| [0020](0020-full-time-sprt-in-legend.md) | In Legend, the SPRT plays at CodinGame's time limits | Accepted |
| [0021](0021-prune-finished-branches.md) | Delete finished branches, keeping their content | Accepted |
| [0022](0022-scopes-and-names.md) | Scopes for decision records and pull requests | Accepted |

## Ultimate Tic-Tac-Toe (`uttt`)

| # | Decision | Status |
| --- | --- | --- |
| [0017](0017-value-network.md) | Stage 2: a value network instead of playouts | Accepted; decision 5 superseded by 0018 |
| [0018](0018-value-network-gate.md) | Judge the value network by games at equal iterations | Accepted; decision 1 superseded by 0019 |
| [0019](0019-value-network-gate-at-time.md) | Judge the value network at the bot's time budget | Accepted |
