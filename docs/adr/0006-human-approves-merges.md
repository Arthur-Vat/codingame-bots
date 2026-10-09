# 0006. A human approves every merge

- Status: accepted
- Date: 2026-10-06
- Scope: framework

## Context

Claude writes the code and can run experiments without supervision. The owner wants to stay in the loop on every change, with little time to spend.

## Decision

Claude never merges on its own. A pull request is merged only when its required checks are green and the owner has said to merge it.

## Consequences

- Claude works on `claude/`-prefixed branches and opens pull requests.
- A ruleset on `main` blocks direct pushes and requires the CI checks. It does not require a GitHub review: approval happens in the Claude conversation, and GitHub does not let a person approve their own pull request.
- Each pull request description must make the decision quick: what changed, why, and the check results.
