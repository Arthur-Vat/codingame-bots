---
name: run-experiment
description: Runs one bot strength experiment end to end - hypothesis, journal entry, local screening, candidate release, SPRT, and the merge or the dropped entry. Use when the owner has asked for an experiment on a game's bot.
---

# Run a bot strength experiment

The steps and rules are in `docs/WORKFLOW.md` ("An experiment, step by step") and decision records 0012 to 0015, 0020 and 0042. This is the checklist.

## Before starting

- The owner has asked for this experiment, or approved it from a proposal: experiments are heavy (implementation, CI hours, the owner's usage). Strength work on a game is paused when the roadmap says so.
- One experiment per pull request, one hypothesis per experiment.
- Number: the next `ENNN` in `games/<game>/journal/README.md`. Branch: `claude/<game>-eNNN-<short-name>`.

## Steps

1. **Journal entry first:** `games/<game>/journal/ENNN-<short-name>.md` from `template.md`, with the hypothesis and the change, and its row in the journal's index.
2. **Change the bot** (an `implementer` agent can do it from a spec, skill `land-work`). Run `CLAUDE.md`'s checks; the bundle must stay within 100,000 characters (ADR 0037).
3. **Screen on GitHub** before spending an SPRT (ADR 0049, decision 2; `docs/WORKFLOW.md`): push each variant of the bot as a commit (or a branch) of the experiment's branch, then dispatch `screen.yml`: one calibrated runner per variant and shard, at CodinGame's speed and real limit (ADR 0048; ADR 0049 decision 8 for Ultimate Tic-Tac-Toe). Variants are `NAME=REF` (a commit SHA, a branch or a tag), 1 to 8; `baseline` is a release name (empty: the newest); `pairs` is 400 by default (a multiple of `shards`); `seed` is empty for the game's own.
   ```sh
   R=Arthur-Vat/codingame-bots
   gh api -X POST repos/$R/actions/workflows/screen.yml/dispatches -f ref=claude/<game>-eNNN-<short-name> \
     -f 'inputs[game]=<game>' -f 'inputs[bot]=<bot>' -f 'inputs[variants]=a=<sha> b=<sha>' \
     -f 'inputs[pairs]=800' -f 'inputs[shards]=2'
   ```
   Find the run with `gh api "repos/$R/actions/workflows/screen.yml/runs?per_page=3"`, then its jobs (`gh api repos/$R/actions/runs/<run>/jobs --jq '.jobs[] | {name, conclusion, id}'`), and read the results from the annotations of the job `Summary` (`gh api repos/$R/check-runs/<job id>/annotations`: one notice per variant with its Elo, interval, pairs and faults, a warning per shard without a result) or from that job's log; sessions cannot download the artifacts. The workflow's header lists the inputs. For a quick look, screen locally instead: `scripts/screen.sh play <game> <bot> <name> <ref> games/<game>/releases/<release>.rs <pairs> 0 1 <seed>` then `scripts/screen.sh summary <game> target/screen`, or the game's arena directly (`target/release/<game>-arena --help`). Say in the entry what was played. If a variant is clearly weaker, drop it; if all are, stop here (step 7) without a release.
4. **Freeze the candidate:** `scripts/new-release.sh <game> <bot>`, which writes the minified release and its readable copy (ADR 0045). In the same commit, point the game README's `**Current release:**` line and the root README's games table to the new release: `scripts/check-docs.sh` requires them.
5. **Open the pull request** (`feat(<game>): ENNN, <what changed>`), review at the bot tier (skill `review-pr`). The SPRT workflow plays the smoke test and the SPRT: at CodinGame's speed and real limit in every game (ADR 0048, ADR 0049 decision 8); a game with several SPRT runners (`SPRT_RUNNERS`, today both games with 8) plays it in blocks over calibrated runners (`sprt-parallel.yml`), in a fraction of that time, and the pull request adds exactly one release. If it accepts, the league of the release runs on the same pull request (800 pairs per pairing: about 3 to 3.5 hours for Ultimate Tic-Tac-Toe at scale 1) and posts its result lines in a comment. A game with a nightly league (`LEAGUE_NIGHTLY=true`, today both games) has no league job on the pull request: the release merges on its SPRT and the nightly league rates it overnight on the branch `league-results`.
6. **Record the result:** copy the SPRT comment's summary into the entry (`sprt`, `elo`, `pairs`, `full_time`, `decision`). For a game without a nightly league, copy the league comment's lines verbatim and in order at the end of `games/<game>/league.jsonl`, never through `jq` (the seeds are 64-bit), and regenerate `LEADERBOARD.md` with the command in the comment, on Linux, in the same pull request: the check `League results` fails until both match. A game with a nightly league skips this copy: its results reach `main` in the weekly pull request. Commits that change only Markdown and the results file reuse the SPRT's and the league's results.
7. **Accepted:** ask the owner to approve the merge (skill `land-work`, step 4; ADR 0023), then skill `deliver-release`. **Rejected or inconclusive:** remove the release file, its readable copy and the bot change, revert the README lines, set `release: none` and `decision: dropped`, explain in the entry what was learned, and ask the owner to approve merging the entry alone (skill `land-work`, step 4).

## Keep

Code worth keeping from a dropped experiment stays reachable: a closed pull request keeps its commits (ADR 0021). Name it in the entry.
