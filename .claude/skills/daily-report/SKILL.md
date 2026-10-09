---
name: daily-report
description: Writes the daily report for the owner - what was done in the last day, what waits for him, what could be improved, hot fixes and evolutions to propose. Use in the daily scheduled session, or when the owner asks for a report.
---

# Daily report

The owner's brief (2026-10-09): sum up the day's work, identify areas of improvement, propose hot fixes, and, when relevant, propose evolutions for the next steps. The run is quick and read-only.

## Rules

- Read only: no commits, pushes, pull requests, merges, issues or workflow runs. Hot fixes and evolutions are proposals; the owner approves them in the conversation and a later session carries them out.
- Quick: a few commands and no subagents. Stop reading once the report can be written.
- Facts only from the repository and GitHub. Say "unknown" rather than guess.

## Gather (since the last report, about 24 hours)

`R=Arthur-Vat/codingame-bots`; fetch `main` first (skill `github-api`).

1. Merged: `git log --merges --since="24 hours ago" --format='%s' origin/main` and the merged pull requests' titles.
2. Open pull requests, their checks and how long they have waited: `gh api "repos/$R/pulls?state=open"`.
3. Workflow runs of the day and their conclusions: `gh api "repos/$R/actions/runs?created=>=<date>&per_page=50" --jq '.workflow_runs[] | {name, conclusion, head_branch}'`; for a failure, its annotations.
4. SPRT verdicts and training runs finished, from pull request comments and `claude/train/` branches.
5. Decisions waiting for the owner: those asked in open pull requests' bodies, and the roadmap's unticked items that are his (settings, choices). The scheduled session cannot see the conversation, so it lists what it can find and says so.
6. Health: `scripts/check-docs.sh` on `main`; branches the Prune branches workflow would delete (`REPO=$R scripts/prune-branches.sh`, a dry run by default).
7. The roadmap's current step: what is ticked, what is next.

## Look for improvements

Failures and their causes, checks that are slow or flaky, steps that needed retries, work that cost much usage for its result, recurring review findings, docs drift, rules or skills that did not fit what happened.

## Write

At most about 300 words, in this order, leaving out empty sections:

1. **Headline:** one sentence.
2. **Done:** merged pull requests and results (SPRT, training), one line each.
3. **Waiting for you:** pull requests to approve, decisions to take, each with a link.
4. **Problems:** what failed or drifted, with the cause when known.
5. **Hot fixes proposed:** each with its size (minutes), risk and what it fixes.
6. **Evolutions proposed:** additions to the next steps, each with why now.

If nothing happened, the report is one line saying so and what is waiting.

## Deliver

The report is the session's final message, headline first. In the daily scheduled session, the scheduled task sends it to the owner's phone as a push notification: send no other one. When the owner asks for a report in the conversation, the reply is the report.
