# 0053. The weekly task starts the League weekly workflow itself

- Status: accepted (owner, 2026-11-05)
- Date: 2026-11-05
- Completes: [0051](0051-weekly-league-pull-request.md), decision 1
- Scope: framework

## Context

[ADR 0051](0051-weekly-league-pull-request.md), decision 1, has a weekly Claude task open the league results pull request on Thursdays at 06:00 UTC, from a branch that the League weekly workflow prepares first. The workflow was scheduled at 05:30 UTC.

GitHub started this repository's scheduled runs 5 to 8 hours late from 2026-11-01 to 2026-11-04 (the nightly league's 02:00 UTC schedule started between 08:36 and 09:40). On 2026-11-05, the first Thursday, the League weekly workflow had not started when the task woke at 06:06, so the task found no branch and opened nothing. A run started by hand (`workflow_dispatch`) starts at once: a session started one at 06:55 and it pushed the branch within minutes.

The owner was offered three ways on 2026-11-05: the task starts the workflow itself and waits for it; an earlier schedule, on Wednesday evening, which still depends on GitHub's delay and could run before Wednesday's night is published; or no change. He chose the first ("Task starts the workflow").

## Decision

1. **The League weekly workflow has no schedule; the weekly task starts it, waits for it, then opens the pull request.** On Thursdays at 06:00 UTC the task starts the workflow by hand unless a run of the same UTC date already exists, waits for that run to end, and then acts as ADR 0051 says: it opens the pull request from the branch when the branch exists and has none yet.

## Consequences

- The weekly pull request no longer depends on when GitHub starts scheduled runs.
- The task starts one workflow run a week, which ADR 0051, decision 2, did not list among what it may do; it still merges, reviews, pushes and comments nothing.
- If the task fails or is turned off, no branch is made either: a session starts the workflow by hand (`gh workflow run league-weekly.yml`, or the REST dispatch in the skill `github-api`) and opens the pull request.
- The task's stored prompt changes with this record.
