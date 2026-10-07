# 0012. Evaluation: SPRT on game pairs, seeded openings, frozen releases

- Status: accepted; decision 7 (time limits) superseded by [0013](0013-evaluation-time-limits.md)
- Date: 2026-10-07

## Context

Phase 3 decides how the framework says "this bot is stronger" and what happens to a bot that is. The answer must hold with free GitHub-hosted runners (4 cores, up to 6 hours per job), noisy timing, and an owner who decides in a few hours a week. The settings below were proposed in `docs/ARCHITECTURE.md` and are fixed here, with the evidence gathered while building them.

## Decision

### The test

1. **Sequential probability ratio test on seat-swapped pairs.** Each pair plays one seed twice, seats swapped. The candidate's points in a pair (0, ½, 1, 1½ or 2) form a pentanomial distribution; the log-likelihood ratio uses the normal approximation of the generalized SPRT, with logistic Elo, as chess engine testing does.
2. **H0: at most 0 Elo stronger; H1: at least 10 Elo stronger; both error rates 5%.** The test stops when the LLR leaves (−2.94, 2.94), or after 10,000 pairs (20,000 games) as inconclusive, which counts as a failure.
3. **No verdict before 30 pairs.** The normal approximation needs an estimate of the variance of pair results, which a handful of pairs cannot give: without this guard, one pair won twice accepts the candidate at once. Simulated tests accept an equal candidate about 30% of the time without the guard and about 5% with it; a unit test keeps it that way.
4. **Pairs are counted in order** (pairs 0..n once all are complete), so the verdict does not depend on which game finishes first.
5. **Before the SPRT, a smoke test:** 100 pairs against the random bot, no fault allowed (timeout, crash, invalid answer) and at least 99% of the points.

### The conditions

6. **Seeded random openings of 4 plies.** The referee imposes 4 random moves at the start of each pair, drawn from the pair's seed, by offering a single valid action on those turns. This replaces a folder of opening positions: no files to maintain, endless variety, reproducible from the seed. Some random openings favor one side, but each pair plays its opening from both seats, so the imbalance cancels, and fewer even games make pair results more informative: chess engine testing chooses unbalanced openings on purpose for that reason. Bots must pick from the valid actions they receive, as they must on CodinGame anyway. An opening never ends the game. Phase 4 checks the number of plies with the MCTS bot.
7. **Time limits scaled by 0.2, provisionally:** 200 ms for the first answer and 20 ms afterwards, instead of 1,000 and 100. Bots read the factor from `CG_TIME_SCALE` and scale their budget. Full limits would make an average test take hours. The owner accepted this value for now, to be revisited with the MCTS bot in Phase 4, using its timeouts and its strength at 0.2 against 1.0.
8. **Settings live in `games/<game>/evaluation.env`,** read by `scripts/sprt.sh` and `scripts/league.sh`, so a run on CI and a run on a laptop use the same ones.

### Versions

9. **A release is a frozen paste-ready file:** `games/<game>/releases/<game>-vNNN.rs`, made by `scripts/new-release.sh`. CI checks that a new release is the bundle of the bot named in its header, numbered right after the last one, one per pull request, named by a journal entry, and that released files never change. Old versions therefore build forever with `rustc` alone, whatever happens to the shared crates.
10. **A pull request that adds a release file is a candidate.** The SPRT workflow runs on every pull request, passes at once when no release is added, and otherwise tests the new release against the previous one (against the `greedy` bot before the first release). The verdict is a check and a comment on the pull request. The result is kept for the exact code and settings it was obtained with, so editing the journal afterwards does not run the test again.
11. **Journal: one entry per experiment, in the experiment's pull request.** The entry states the hypothesis and the change before the test and the results after it. An accepted candidate is merged whole. For a rejected or inconclusive one, the bot change and the release file are removed from the pull request and the entry is merged alone, so failed ideas stay on record.
12. **On merge, a GitHub release** tagged `<game>-vNNN` carries the paste-ready file. **A league** rates every release with the baseline bots (random anchored at 0) after each release and on demand; ratings appear in the workflow's summary.

## Consequences

- Expected cost of a test near the bounds, from simulations and from 40 A/A tests with the greedy bot: about 2,800 pairs (5,600 games). With 20 ms moves on 4 cores that is roughly half an hour; clear improvements end in a few hundred pairs.
- The A/A tests rejected the copy 39 times out of 40 and were inconclusive once; none accepted it. The greedy bot is accepted against random and random rejected against greedy after the minimum 30 pairs. CI repeats these checks on every push with fixed seeds.
- Bounds of 0 and 10 Elo cannot see gains of a few Elo. When improvements get that small, a new ADR can narrow the bounds and raise the cap, at a higher cost per test.
- Timing noise on shared runners can turn into timeouts at 20 ms. Measured here (2 cores, no search), answers of a bot that does not search took at most 9 ms even with twice as many games as cores. A bot using its whole budget leaves less room, which is why the time scale is provisional.
- Differences from `docs/ARCHITECTURE.md`: candidates are found by their release file rather than a label; openings are seeded rather than stored; the league runs after releases rather than weekly, and publishes to the run summary rather than GitHub Pages; the SPRT runs in one job rather than split across jobs.
