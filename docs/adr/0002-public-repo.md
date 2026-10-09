# 0002. Public repository on a personal GitHub account

- Status: accepted
- Date: 2026-10-06
- Scope: framework

## Context

Rating bots takes thousands of games per experiment. GitHub Actions is free and unmetered on standard GitHub-hosted runners for public repositories, while private repositories only get a monthly quota of minutes.

## Decision

The repository is public, under the owner's personal GitHub account (`Arthur-Vat/codingame-bots`).

## Consequences

- All compute runs on free standard runners; larger runners are never used, since they are billed even on public repositories.
- Anyone can read and copy the bots. This is accepted.
- GitHub's terms limit Actions to building, testing and publishing the repository's own project. Matches between this repository's bots are testing it; the arena must not become a general compute service.
- Switching to private later is possible but erases stars and watchers, detaches forks, and ends free compute. Anything once public should be assumed copied.
