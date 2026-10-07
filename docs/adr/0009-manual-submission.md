# 0009. Manual submission to CodinGame

- Status: accepted
- Date: 2026-10-06

## Context

CodinGame has no official API to submit bots or download replays. Community tools use unofficial endpoints that can break or conflict with the site's terms.

## Decision

The owner submits each released bot version by pasting its file into the CodinGame editor, then reports the resulting rank.

## Consequences

- Every release must include one readable, formatted file that compiles on CodinGame as is (ADR 0010).
- The real leaderboard stays an independent check of the local ratings.
- Before entering a prize contest, its rules must be read: contest rules can restrict outside help and may publish submissions under GPL v3.
