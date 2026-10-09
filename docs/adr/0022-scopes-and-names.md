# 0022. Scopes for decision records and pull requests

- Status: accepted (owner, 2026-10-09)
- Date: 2026-10-09
- Scope: framework

## Context

On 2026-10-09 the owner asked that decisions be documented for either a single game or the whole framework, and that pull requests say whether they concern one game or not.

The decision records so far share one numbered sequence in `docs/adr/`. Most concern the framework; four concern Ultimate Tic-Tac-Toe only (its training and its value network), and nothing marks them. Pull requests and commits follow Conventional Commits with an optional scope, used loosely: `uttt`, `adr`, `train` or none.

Links to the records are spread through the docs, the journal and pull requests, so the records should not move or be renumbered.

## Decision

1. **Every decision record names its scope** on a `Scope:` line under its status and date: `framework`, or a game's identifier such as `uttt` when it binds that game only. The records keep one sequence and stay in `docs/adr/`; the index groups them by scope. Adding this line to existing records is bookkeeping, like updating their status, not a rewrite.
2. **Every pull request title and commit subject has a scope:** `type(scope): summary`, with the types of Conventional Commits (`feat`, `fix`, `docs`, `test`, `ci`, `refactor`, `perf`, `chore`, `build`, `revert`) and one or more scopes separated by commas, without spaces:
   - a game's identifier, the name of its folder under `games/` (today `uttt`), for anything inside that folder or done for that game;
   - otherwise a framework area: `core`, `search`, `arena`, `bundler` (the crates `cg-core`, `cg-search`, `cg-arena`, `cg-bundler`), `workflows` (`.github/`), `scripts` (`scripts/`), `adr` (decision records), `docs` (other documentation), `agents` (Claude's project agents and skills in `.claude/`), `repo` (workspace configuration, `CLAUDE.md`, licenses, dependency updates).
   The title has at most 100 characters and no final period. A change to a game's training workflow, for instance, is `ci(workflows,uttt): ...`.
3. **A workflow labels pull requests by the paths they change** (`game:<id>`, `framework`, `ci`, `adr`, `agents`, `release`, `experiment`, `training`, `docs-only`) and checks the title. Dependabot's titles are generated, so they are not checked.

## Consequences

- Whether a decision or a change concerns one game or the framework shows in its title, its labels and the index.
- Older commits keep their subjects; the check applies to new pull requests.
- A new game adds its identifier to the allowed scopes by existing as a folder under `games/`.
