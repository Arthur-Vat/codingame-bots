# 0059. The CodinGame collection follows the newest release

- Status: accepted (owner, 2026-11-13)
- Date: 2026-11-13
- Completes: [0058](0058-codingame-collection-on-github-actions.md), decision 1
- Scope: framework

## Context

The collection of ADR 0058 plays, in each run, the day's batch of the newest release on `main` at the run's start. A batch can run for more than five hours, and GitHub starts only a few of the scheduled runs a day, so a release merged during the day waited for hours, while the release before it went on using the 600 test games of 24 hours. On 2026-11-13 the batch of `ice-and-fire-v018`, planned by the first run after its merge, played no game: the day's earlier batch of `ice-and-fire-v014` had already used the quota down to the owner's reserve. The owner asked that the daily games always be played with the latest release, and chose to stop a batch early and to start the collection when a release is published.

## Decision

1. **A batch of the collection stops before its next game when `main` holds a newer release of the game.** `scripts/codingame.sh batch --until-newer-release` reads the newest release on `origin/main` before a game, at most every five minutes, and stops cleanly, its games left planned, when it is newer than the one played.
2. **The Release workflow starts the collection when it publishes a release of the collected game.** It dispatches `codingame-collect.yml` on `main` with `play`; the collection keeps ADR 0058's triggers (`schedule` and `workflow_dispatch`) and its rules for the token, and the Release workflow gains the permission `actions: write` for this alone. A run already going stops by decision 1, and the dispatched run, which waits for it, plays the new release.

## Consequences

- A new release starts collecting within minutes of its merge, as far as the day's quota allows, and an older release stops using it.
- The games of the day can belong to two releases, in two batches; reports read them by release (ADR 0057, decision 3).
- The Release workflow can now start workflows; its other steps are unchanged. A change to it gets the deep review tier, as the collection's workflow does.
