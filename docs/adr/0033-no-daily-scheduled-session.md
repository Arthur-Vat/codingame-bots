# 0033. No daily scheduled session

- Status: accepted (owner, 2026-10-13); the usage line of decision 3 superseded by [0034](0034-no-usage-monitoring.md)
- Date: 2026-10-13
- Scope: framework

## Context

Phase 5 set up one quick scheduled session a day, at 06:45 Paris time, whose final message was a daily report sent to the owner's phone: the last day's work, what waited for the owner, areas of improvement, and hot fixes and evolutions to propose (roadmap, phase 5, step C). It changed nothing and never merged ([ADR 0023](0023-chat-approved-merges.md)). Since 2026-10-09 every interactive session has read and updated `HANDOFF.md` ([ADR 0024](0024-handoff-branch.md)), which holds what waits for the owner, the open questions, the follow-ups and each piece of work's usage. On 2026-10-13 the owner asked to stop the daily report altogether.

## Decision

1. **No scheduled session runs for this repository.** The owner deletes the scheduled task of the daily report, and the skill `daily-report` is removed.
2. **The owner learns what waits for him in the conversation and from `HANDOFF.md`.** A decision asked while he is away is written in the session's final message, sent as one push notification, and listed under "Waiting for the owner" in `HANDOFF.md` (skill `ask-decision`).
3. **What went wrong or cost too much in a piece of work is said to the owner at its end,** with its usage line, instead of being kept for a report.

## Consequences

- The daily report's usage stops, and nothing summarises the day unless the owner asks.
- What ADR 0023 and ADR 0024 say about scheduled sessions still holds should one come back: it would read `HANDOFF.md` only, and never merge.
- [ADR 0031](0031-arena-games-archive.md) expected the daily report to warn when an arena-games archive is close to expiring; that warning was never added and is dropped. If an archive is lost, it starts again from the next release, as that record accepts.
- Bringing a scheduled session back is a new decision.
