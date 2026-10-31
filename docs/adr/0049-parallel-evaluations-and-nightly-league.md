# 0049. Evaluations spread over runners, and a nightly league for every game

- Status: accepted (owner, 2026-10-31); decision 4's shared budget and decision 4.4's ±15 Elo superseded by [0050](0050-league-anchor-target-and-budget.md)
- Date: 2026-10-31
- Supersedes: [0038](0038-incremental-league.md), decision 1's rule that a pairing is played once, decision 6's league on the release's pull request, decision 7 (the canary) and decision 9's source of the archive's league sample; [0042](0042-official-time-limits-and-800-pairs.md), decision 4 (the league job's time limit), and decision 1 for Ultimate Tic-Tac-Toe once its calibration is merged (decision 8 below); [0031](0031-arena-games-archive.md), decision 2's league sample added when a release is published
- Scope: framework

## Context

A release's pull request waits for its SPRT, then for its league. Recent A Code of Ice and Fire releases took 7 to 40 minutes of SPRT and 83 to 95 minutes of league ([sprt.yml](../../.github/workflows/sprt.yml) runs of 2026-10-30 and 2026-10-31). An Ultimate Tic-Tac-Toe SPRT can take about 4 hours. Screening variants runs in the session, one variant after another, on 4 CPUs.

A free GitHub runner has 4 vCPUs on 2 physical cores, and the arena already plays 4 games at once on it. [ADR 0048](0048-ice-and-fire-codingame-speed.md) measured the timing bot's workload at 22.8 units a millisecond with one game and 13.4 per game with four at a time, 53.6 in all. The bots use their whole time budget, so a machine's games are bound by its compute. With each job calibrated to CodinGame's speed, more games at once only slow each game, and the machine's total stays the same. One runner is already at its limit. The same holds for the session's machine. More speed needs more runners or fewer games.

The calibration also makes runners comparable. Each job gives the bots CodinGame's compute, whatever processor it runs on, so results from several runners can be added together. That holds for A Code of Ice and Fire. Ultimate Tic-Tac-Toe still plays at fixed limits ([ADR 0042](0042-official-time-limits-and-800-pairs.md)), so its runners differ in speed.

On 2026-10-31 the owner asked to speed up the SPRT and the quick benchmarks, locally and in CI, and for a league that runs on GitHub every day, for every game, so that unplayed bots get their games and the other ratings grow more precise. He chose in the conversation, in forms:

1. The SPRT is split over runners in rounds, "but make sure we have similar conditions as codingame if it is not yet the case"; Ultimate Tic-Tac-Toe is calibrated first ("Calibrate UTTT first (Recommended)").
2. Screening is a GitHub workflow, calibrated like the SPRT, with one runner per variant, dispatched by the session.
3. Release pull requests wait for the SPRT only; the league runs nightly ("SPRT only; league nightly (Recommended)").
4. The nightly results go to a branch that is never merged, and reach `main` in a pull request about weekly ("Results branch, PR weekly (Recommended)").
5. Nightly budget: "10 runners, up to 3 h (Recommended)", for all games together, at about 03:00 Paris.
6. After new bots, precision goes to neighbours near the top, "but make sure lower elo bots do play from time to time": 20% of each night is for them.
7. Pool: releases and baselines. A night stops early once every bot is rated and the top 10's intervals are within ±15 Elo.
8. Ultimate Tic-Tac-Toe's league history is replayed once its calibration is merged ("Replay it (Recommended)").

## Decision

1. **An SPRT plays in blocks over 8 runners.** Every runner plays blocks of seat-swapped pairs with its own seeds, calibrated on that runner at its own parallelism as [ADR 0048](0048-ice-and-fire-codingame-speed.md), decision 2, says. The test adds up the finished blocks of every runner and checks the SPRT's bounds as blocks finish. It stops at the first check past a bound, or at `SPRT_MAX_PAIRS`. The SPRT's bounds, error rates and cap do not change ([ADR 0042](0042-official-time-limits-and-800-pairs.md), decision 5). Wald's bounds still hold when checks come only between blocks, and in practice the error rates fall. The block size and the way runners learn that the test has stopped are set in the implementation's pull request, with a simulation of the arena's SPRT that measures the error rates and the pairs played.
2. **Screening runs on GitHub, one runner per variant.** The session dispatches a workflow with a branch, a baseline, the variants, a number of pairs and a seed. Each variant plays its pairs on its own runner, calibrated as the SPRT is. The results come back in the run's summary and annotations, which the session reads. A screening that is too long for one runner may be split over several, since it plays a fixed number of pairs. Screening in the session stays possible, calibrated the same way (`scripts/calibrate.sh`), for quick looks.
3. **A release's pull request no longer plays the league.** It merges on its SPRT, the required checks and the owner's approval. The League results check no longer asks a release for its pairings; it still checks that a pull request which changes a league's results keeps them and its leaderboard consistent. The SPRT's own games still never count in the league ([ADR 0038](0038-incremental-league.md), decision 6): a test that stops as soon as it is confident overstates the winner's margin.
4. **A league runs every night for every game, at about 03:00 Paris, on at most 10 runners for at most 3 hours, all games together.** Each night, for each game:
   1. Every bot of the pool without pairings first plays the opponents of [ADR 0038](0038-incremental-league.md), decision 2, at `LEAGUE_PAIRS` pairs each: its previous release first, then the fixed opponents and the nearest bots in rating.
   2. 80% of what remains goes to pairs of bots near the top of the ratings whose order is least certain, weighted toward the top. Each night plays the pairings that would shrink the top intervals most.
   3. 20% goes to pairings among the other bots, weighted toward those played least recently.
   4. The 80% share stops early once every bot is rated and every top-10 bot's 95% interval is within ±15 Elo. The 20% share still plays.
   Every job is calibrated as decision 1 says. The pool is every release and the game's baselines, as `LEAGUE_BOTS` lists.
5. **A pairing may be played again.** Each run adds lines to the results file, with fresh seeds, and ratings are computed over every line, as [ADR 0038](0038-incremental-league.md), decision 4, already does. A pairing played again replaces the canary: a new result outside the stored pairing's interval is reported as a sign of drift, and both stay in the file.
6. **The nightly results go to the branch `league-results`, which is never merged, and reach `main` in a pull request about once a week.** The branch holds only each game's results file and leaderboard. The pull request comes from a `claude/` branch made from `main` that copies those files from `league-results`. It also comes when a release or a decision needs fresh ratings, and the owner approves it like any other ([ADR 0023](0023-chat-approved-merges.md)).
7. **The night that first plays a release adds that league's sample of games to the archive** ([ADR 0031](0031-arena-games-archive.md)). The release itself adds its SPRT sample when it is published, as before.
8. **Ultimate Tic-Tac-Toe is calibrated before it plays split over runners.** It follows [ADR 0048](0048-ice-and-fire-codingame-speed.md), decisions 1 to 3:
   - a timing bot of its own measures CodinGame's speed and real limit in test games through the API ([ADR 0047](0047-codingame-api.md));
   - the figures go into its `evaluation.env` in a pull request that cites the games;
   - its league history is then replayed at the new settings on the replay workflow, the old results replaced, on the owner's go-ahead for that run.
   Until then, Ultimate Tic-Tac-Toe keeps its SPRT on one runner and its league on the release's pull request, as before; decisions 1, 3 and 4 apply to it once its calibration is merged.

## Consequences

- A release's pull request waits only for its SPRT. That saves about 90 minutes for A Code of Ice and Fire, and a long SPRT takes a fraction of its time today. Its rating arrives within a day, on the results branch.
- `LEADERBOARD.md` on `main` can lag the results branch by up to a week. The branch is the place to read current ratings.
- One SPRT takes 8 runners and the nightly league up to 10. With the CI jobs, more than the 20 concurrent jobs a free account gets can queue at times. The nightly hour keeps that rare.
- Runner start-up (checkout, cached build, calibration) costs a minute or two. Short SPRTs gain little; long ones gain most.
- Checking between blocks lets a test run past the point where it was decided, and a few more close tests end at the cap undecided. The block size trades that against how often runners stop to report.
- The arena needs per-shard seeds, a command that adds up shards and checks the SPRT's bounds, and a planner for the night's pairings. The leaderboard must accept several lines for one pairing. The workflows need a staged SPRT, a scheduled league with its results branch and weekly pull request, and a screening workflow. The docs follow (`docs/WORKFLOW.md`, `docs/ARCHITECTURE.md`, the games' READMEs, the skills `run-experiment` and `deliver-release`).
- A scheduled workflow stops after 60 days without activity in a public repository. The weekly pull request's merges keep the repository active.
- The nightly workflow needs `contents: write` to push to `league-results`.
- Ultimate Tic-Tac-Toe's calibration needs a probe on CodinGame, which a session can now run itself.
