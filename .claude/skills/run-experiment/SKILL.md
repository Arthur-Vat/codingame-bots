---
name: run-experiment
description: Runs one bot strength experiment end to end - hypothesis, journal entry, local screening, candidate release, SPRT, and the merge or the dropped entry. Use when the owner has asked for an experiment on a game's bot.
---

# Run a bot strength experiment

The steps and rules are in `docs/WORKFLOW.md` ("An experiment, step by step") and decision records 0012 to 0015 and 0020. This is the checklist.

## Before starting

- The owner has asked for this experiment, or approved it from a proposal: experiments are heavy (implementation, CI hours, the owner's usage). Strength work on a game is paused when the roadmap says so.
- One experiment per pull request, one hypothesis per experiment.
- Number: the next `ENNN` in `games/<game>/journal/README.md`. Branch: `claude/<game>-eNNN-<short-name>`.

## Steps

1. **Journal entry first:** `games/<game>/journal/ENNN-<short-name>.md` from `template.md`, with the hypothesis and the change, and its row in the journal's index.
2. **Change the bot** (an `implementer` agent can do it from a spec, skill `land-work`). Run `CLAUDE.md`'s checks; the bundle must stay under 100,000 bytes.
3. **Screen locally** before spending an SPRT: play the candidate against the current release with the game's arena (`target/release/<game>-arena --help`). Twenty-millisecond moves are allowed for screening (ADR 0020); say in the entry what was played. If it is clearly weaker, stop here: drop it (step 7) without a release.
4. **Freeze the candidate:** `scripts/new-release.sh <game> <bot>`. In the same commit, point the game README's `**Current release:**` line and the root README's games table to the new release: `scripts/check-docs.sh` requires them.
5. **Open the pull request** (`feat(<game>): ENNN, <what changed>`), review at the bot tier (skill `review-pr`). The SPRT workflow plays the smoke test and the SPRT: at CodinGame's exact limits for a game in Legend, up to about 4 hours.
6. **Record the result:** copy the SPRT comment's summary into the entry (`sprt`, `elo`, `pairs`, `full_time`, `decision`). Markdown-only commits reuse the SPRT's result.
7. **Accepted:** ask the owner to approve the merge with `merge #<n>` (ADR 0023), then skill `deliver-release`. **Rejected or inconclusive:** remove the release file and the bot change, revert the README lines, set `release: none` and `decision: dropped`, explain in the entry what was learned, and ask the owner to approve merging the entry alone (`merge #<n>`).

## Keep

Code worth keeping from a dropped experiment stays reachable: a closed pull request keeps its commits (ADR 0021). Name it in the entry.
