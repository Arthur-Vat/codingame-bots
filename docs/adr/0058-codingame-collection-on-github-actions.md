# 0058. The daily collection of CodinGame games runs on GitHub Actions

- Status: accepted (owner, 2026-11-07)
- Date: 2026-11-07
- Supersedes: [0057](0057-daily-collection-of-codingame-games.md), decision 1; [0047](0047-codingame-api.md), decision 2, for the collection's token
- Scope: framework

## Context

[ADR 0057](0057-daily-collection-of-codingame-games.md) has a dedicated Claude session, woken once a day by a scheduled task, collect CodinGame games against the top players: it fetches our new ranking games and plays up to 500 test games a day. Its token follows [ADR 0047](0047-codingame-api.md) decision 2: it lives only in the session's sandbox, so the owner approves a new one from a link most days.

On 2026-11-07 the owner asked to retrieve the games with GitHub instead, so that no workflow runs in Claude's sessions, and chose, in a form, a GitHub workflow with the token stored as a secret, a token that lasts one year, a run every 3 hours, and the session's scheduled task disabled at once.

Facts the decision rests on (2026-11-07):

- A CodinGame token is made with the OAuth device flow; the user picks its lifetime on the approval page (two years by default) and can revoke it at any time on the website; it can be asked for with fewer scopes, such as `challenge:read challenge:play`, which cannot submit (API reference and Agent Skill, read 2026-11-06).
- GitHub stores repository and environment secrets encrypted and masks them in logs. Workflows that run for a pull request from a branch of the same repository receive the repository's secrets; an environment whose deployment branches are limited to `main` gives its secrets only to jobs running on `main`. Claude's sessions push branches to this repository and cannot change its settings or secrets (skill `github-api`).
- A job on a GitHub-hosted runner stops after 6 hours. CodinGame allows 200 test games per 6 hours, so 500 games take more than 12 hours: they need several runs a day, each resuming the same batch. Scheduled workflows can start late, and stop after 60 days without activity on the default branch.

## Decision

1. **A scheduled GitHub Actions workflow collects the games, every 3 hours and on manual dispatch.** Each run fetches our new ranking games, then plays the day's batch of ADR 0057 decision 2 (the current release against the top 10, 50 games each on the 50 fixed maps, leaving 100 test games of the 600 allowed in 24 hours), as far as the limits allow before the job's time runs out, and pushes what it stored to the branch `claude/codingame-games`. Runs never overlap; a run with nothing left to play ends at once. No Claude session runs the collection.
2. **Its token is the owner's, made for it alone, and stored by him.** He creates it with the device flow on his own computer, with the scopes `challenge:read challenge:play` and a lifetime of one year, and saves it as the secret `CODINGAME_TOKEN` of a GitHub environment `codingame` whose deployment branches are limited to `main`. Only the collection workflow uses that environment, never a pull request's workflow; the token is never printed, logged or committed. Sessions keep ADR 0047's practice for their own API use.
3. **The token's end is planned.** `HANDOFF.md` keeps its expiry date and a reminder a month before; when CodinGame refuses the token, the run fails visibly and plays nothing, and the owner makes a new one.
4. **The owner pauses or stops the collection by disabling the workflow** in GitHub's Actions page, or by asking in the conversation; the games collected stay.

## Consequences

- The owner no longer approves a link each day; the collection runs without any Claude session, and its runs and failures are visible in GitHub's Actions page.
- A token that can play test games for a year sits in the repository's settings; restricting its environment to `main`, and its scopes to reading and playing, limit what a leak could do, and the owner can revoke it on CodinGame at any time.
- Changing the collection workflow on `main` changes what the token is used for, so such a pull request gets the deep review tier, like any workflow.
- The dedicated session of ADR 0057 and its scheduled task are retired: the task is disabled, and the session archived after the workflow's first successful day.
- The skill `collect-games` changes from a session's procedure to the workflow's documentation and checks; `scripts/codingame.sh` gains what the workflow needs (reading the token from the environment, bounding a run by the job's time).
