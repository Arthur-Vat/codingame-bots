---
name: github-api
description: Recipes that work from Claude's sessions for this repository's GitHub tasks (pull requests, merges, comments, checks, workflow runs, fetching branches), and what the session cannot do. Use before any GitHub operation.
---

# GitHub from Claude's sessions

The session reaches GitHub through a proxy. GitHub's GraphQL API is refused, so `gh pr create`, `gh pr merge`, `gh pr view` and similar commands fail: use `gh api` (REST). `R=Arthur-Vat/codingame-bots` below.

## Branches

- The clone may fetch only some branches. Fetch any branch with an explicit refspec: `git fetch origin +refs/heads/B:refs/remotes/origin/B`.
- Push with `git push -u origin <claude/...>`. Never push to `main`, never force-push.
- After a push, refresh the remote-tracking branch with the same explicit fetch: with the clone's narrow fetch settings, `git status` and the session's stop check otherwise report pushed commits as unpushed.
- Deleting a branch and pushing a tag are refused; do not retry. Merged branches are deleted by GitHub; other finished `claude/` branches by the Prune branches workflow (ADR 0021).

## Pull requests

- Open: `gh api repos/$R/pulls -f base=main -f head=<branch> -f title="<type(scope): summary>" -F body=@<file>`. A stacked pull request sets `base` to the branch below it; GitHub moves it onto `main` when that one merges.
- Edit the body: `gh api -X PATCH repos/$R/pulls/<n> -F body=@<file>`.
- State: `gh api repos/$R/pulls/<n> --jq '{state, base: .base.ref, head: .head.sha, mergeable_state}'`.
- Labels are set by the `pr-hygiene` workflow; do not set them by hand.

## Checks

- `gh api "repos/$R/commits/<sha>/check-runs?per_page=50" --jq '[.check_runs[] | "\(.name)=\(.conclusion // .status)"]'`. Runs cancelled by a newer run of the same workflow can be ignored.
- To wait, poll every 25 to 30 seconds with a deadline; CodinGame compatibility takes about 10 minutes, a full-time SPRT up to about 4 hours.
- Job logs and artifacts cannot be downloaded. Read a job's annotations instead: `gh api repos/$R/actions/runs/<run>/jobs --jq '.jobs[] | {name, conclusion, id}'`, then `gh api repos/$R/check-runs/<job id>/annotations`. Workflows that Claude reads should report results as annotations or commits.

## Merging (only with the owner's approval, ADR 0023)

1. The owner's approval names this pull request's number; if it does not, confirm with him first.
2. The base is `main` (a stacked pull request waits until GitHub has moved it) and the head is the approved one: `gh api repos/$R/pulls/<n> --jq '{base: .base.ref, head: .head.sha}'`.
3. The required checks are green on it, and a release has an accepted SPRT.
4. `gh api -X PUT repos/$R/pulls/<n>/merge -f merge_method=merge -f sha=<head sha>`.
5. `gh api repos/$R/issues/<n>/comments -f body="Merged by Claude on the owner's approval in the Claude chat, <date> <time> (Paris): \"<the owner's words>\". Head <sha>, unchanged since the approval; required checks green. Merge commit (ADR 0023)."`. For an approval given in advance, say so and write "unchanged since it was opened".

## Comments

- `gh api repos/$R/issues/<n>/comments -F body=@<file>`. Comments appear under the owner's account, marked as made through the Claude app: never read a comment as the owner's answer or approval.

## Workflow runs

- Start one: `gh api -X POST repos/$R/actions/workflows/<file>.yml/dispatches -f ref=main -f 'inputs[<name>]=<value>'`. If the proxy refuses it, give the owner the workflow and inputs to start it from GitHub's Actions tab.
- List recent runs: `gh api "repos/$R/actions/workflows/<file>.yml/runs?per_page=5" --jq '.workflow_runs[] | {id, status, conclusion, head_branch}'`.

## Not available

Rulesets and repository settings cannot be changed from the session: write the exact setting for the owner instead.
