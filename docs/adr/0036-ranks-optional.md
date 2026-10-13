# 0036. CodinGame ranks are reported only when the owner wishes

- Status: accepted (owner, 2026-10-13)
- Date: 2026-10-13
- Supersedes: [0009](0009-manual-submission.md) (the rank reported after each submission)
- Scope: framework

## Context

[ADR 0009](0009-manual-submission.md) has the owner paste each released bot into CodinGame and then report its rank, which Claude records in the release's journal entry. On 2026-10-13 the owner said: "Do not expect a rank when publishing a release, I don't want to do it anymore. I may do it from time to time though."

## Decision

1. **The owner still pastes each release himself, but reporting its rank is optional.** Claude does not ask for a rank, wait for one, or treat a release as unfinished without one.
2. **A rank the owner does report is recorded as before:** in the release's journal entry, and in the game's README or the roadmap when it changes the league or a milestone (skill `deliver-release`).

## Consequences

- Delivering a release ends when the owner has the paste-ready file.
- The real leaderboard is no longer a regular check of the local ratings; the SPRT and the league remain the judges of strength, and an occasional rank still checks them.
- A journal entry's `cg_rank` stays empty unless a rank is reported.
