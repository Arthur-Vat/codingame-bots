# 0051. A weekly scheduled task opens the league results pull request

- Status: accepted (owner, 2026-10-31)
- Date: 2026-10-31
- Supersedes: [0033](0033-no-daily-scheduled-session.md), decision 1, for this one task
- Scope: framework

## Context

[ADR 0049](0049-parallel-evaluations-and-nightly-league.md), decision 6, sends the nightly league's results to the branch `league-results` and brings them to `main` in a pull request about once a week. The owner chose 06:00 UTC on Mondays on 2026-10-31, then Thursdays ("I want the weekly schedule on Thursday actually").

A workflow can prepare that pull request's branch, but it cannot usefully open the pull request. GitHub starts no workflow run for an event caused by a workflow's own token, except `workflow_dispatch` and `repository_dispatch` ([GitHub's documentation](https://docs.github.com/en/actions/security-for-github-actions/security-guides/automatic-token-authentication#using-the-github_token-in-a-workflow)). A pull request opened that way would get no CI, so its required checks would never pass. [ADR 0033](0033-no-daily-scheduled-session.md), decision 1, says that no scheduled session runs for this repository.

Three ways were put to the owner on 2026-10-31: a weekly scheduled Claude task that only opens the pull request; a personal access token stored as a repository secret, so that the workflow opens it itself; or a session opening it the next time the owner works. He chose the first: "Alright with A".

## Decision

1. **A weekly scheduled Claude task opens the league results pull request, Thursdays at 06:00 UTC.** The League weekly workflow first prepares the branch `claude/league-results-YYYY-MM-DD` from `main`, with each game's results file from `league-results` and its leaderboard regenerated. The task opens a pull request from that branch, when it exists and has none yet.
2. **The task does nothing else.** It does not merge, review, push, or change other pull requests. As [ADR 0023](0023-chat-approved-merges.md) says, a scheduled session never merges: the pull request waits for the owner's approval in a conversation. It reads `HANDOFF.md` only if it needs context ([ADR 0033](0033-no-daily-scheduled-session.md), consequences).

## Consequences

- The ratings on `main` follow the nightly league weekly without the owner or a session having to remember it.
- The task costs a few minutes of Claude usage a week. Any other scheduled session is still a new decision.
- If the task fails or is turned off, a session can open the pull request from the prepared branch by hand.
