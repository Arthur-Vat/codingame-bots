# 0001. License: MIT OR Apache-2.0

- Status: accepted
- Date: 2026-10-06
- Scope: framework

## Context

The repository is public (ADR 0002). Without a license, others may read the code but not legally reuse it. The engines and the arena could be useful to other CodinGame players and Rust users.

## Decision

All code in this repository is dual-licensed under MIT OR Apache-2.0: users may follow either license. This is the Rust ecosystem convention. Apache-2.0 adds an explicit patent grant; MIT stays compatible with GPL v2 projects.

## Consequences

- `LICENSE-MIT` and `LICENSE-APACHE` sit at the repository root, and every crate sets `license = "MIT OR Apache-2.0"` through the workspace.
- Code copied from elsewhere keeps its own license and is listed in a `THIRD_PARTY.md` file when the first such copy happens.
- `cargo-deny` in CI only allows dependencies under licenses compatible with this choice (`deny.toml`).
- CodinGame contest rules may publish submitted code under GPL v3; that is compatible with this choice.
- The license of versions already published cannot be withdrawn.
