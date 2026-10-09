# 0004. One repository, one Cargo workspace

- Status: accepted
- Date: 2026-10-06
- Scope: framework

## Context

The framework (arena, bundler, shared search) and each game could live in separate repositories. Before a second game exists, it is unclear which parts are truly generic.

## Decision

One repository holding one Cargo workspace: generic crates under `crates/`, each game under `games/<game>/`.

## Consequences

- A change that touches the framework and a game lands in one pull request, tested together.
- Every Claude session sees the whole system from one clone.
- No versioning of shared crates between repositories.
- Splitting the framework into its own repository stays possible if the second or third game shows it would help; that would be a new ADR.
