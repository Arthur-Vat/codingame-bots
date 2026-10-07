# Architecture decision records

Each file records one decision: the context, what was decided and what follows from it. A decision is never edited away: to change it, add a new record that supersedes the old one and set the old one's status to "superseded by NNNN".

Use [template.md](template.md) for new records. Number them in order.

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
| [0011](0011-framework-structure.md) | Framework structure: text-protocol referees, reference rules, textual bundler | Accepted |
| [0012](0012-evaluation.md) | Evaluation: SPRT on game pairs, seeded openings, frozen releases | Accepted; decision 7 superseded by 0013 |
| [0013](0013-evaluation-time-limits.md) | Evaluation time limits: 20 ms plus 5 ms of tolerance, full time for strong bots | Accepted |
