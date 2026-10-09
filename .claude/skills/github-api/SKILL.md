---
name: github-api
description: Recipes that work from Claude's sessions for this repository's GitHub tasks (pull requests, merges, comments, checks, workflow runs, fetching branches), and what the session cannot do. Use before any GitHub operation.
---

# GitHub from Claude's sessions

The session reaches GitHub through a proxy. GitHub's GraphQL API is refused, so `gh pr create`, `gh pr merge`, `gh pr view` and similar commands fail: use `gh api` (REST). `R=Arthur-Vat/codingame-bots` below.

## Branches

- The clone may fetch only some branches. Fetch any branch with an explicit refspec: `git fetch origin +refs/heads/B:refs/remotes/origin/B`.
- Push with `git push -u origin <claude/...>`. Never push to `main`, never force-push.
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

1. The head is the approved one: `gh api repos/$R/pulls/<n> --jq .head.sha`.
2. The required checks are green on it, and a release has an accepted SPRT.
3. `gh api -X PUT repos/$R/pulls/<n>/merge -f merge_method=merge -f sha=<head sha>`.
4. `gh api repos/$R/issues/<n>/comments -f body="Merged by Claude on the owner's approval in the Claude chat, <date> <time> (Paris): \"<the owner's words>\". Head <sha>, unchanged since the approval; required checks green. Merge commit (ADR 0023)."`

## Comments and issues

- Comment: `gh api repos/$R/issues/<n>/comments -F body=@<file>`.
- Open an issue: `gh api repos/$R/issues -f title=... -F body=@<file> -f 'labels[]=decision'`.

## Workflow runs

- Start one: `gh api -X POST repos/$R/actions/workflows/<file>.yml/dispatches -f ref=main -f 'inputs[<name>]=<value>'`. If the proxy refuses it, give the owner the workflow and inputs to start it from GitHub's Actions tab.
- List recent runs: `gh api "repos/$R/actions/workflows/<file>.yml/runs?per_page=5" --jq '.workflow_runs[] | {id, status, conclusion, head_branch}'`.

## Not available

Rulesets and repository settings cannot be changed from the session: write the exact setting for the owner instead.
