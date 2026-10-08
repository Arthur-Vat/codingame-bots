# 0013. Evaluation time limits: 20 ms plus 5 ms of tolerance, full time for strong bots

- Status: accepted; decision 3 put in place by [0014](0014-full-time-confirmation.md); the "no fault" part of decision 1 superseded by [0015](0015-tolerate-rare-timeouts.md)
- Date: 2026-10-07
- Supersedes: decision 7 of [ADR 0012](0012-evaluation.md)

## Context

ADR 0012 set evaluations (smoke test, SPRT, league) to a fifth of CodinGame's time limits, provisionally, until a real bot could be measured. The MCTS bot of Phase 4 searches for 90% of its scaled limit minus 2 ms: 16 ms of a 20 ms limit.

Measured on GitHub's runners (4 cores, AMD EPYC 7763), MCTS against `greedy` with 4-ply openings:

| Setting | Games | Timeouts | Slowest MCTS answer |
| --- | --- | --- | --- |
| Scale 0.2, 4 games at once | 1,000 | 4 | over 20 ms |
| Scale 0.2, 3 games at once | 1,000 | 0 | 18.3 ms of 20 |
| Scale 0.2 + 5 ms tolerance, 4 games at once | 1,000 | 0 | 21.6 ms of 25 |
| Scale 0.2 + 10 ms tolerance, 4 games at once | 1,000 | 0 | 20.9 ms of 30 |
| Scale 0.5, 4 games at once | 1,000 | 0 | 46.0 ms of 50 |
| Scale 1.0, 4 games at once | 400 | 0 | 91.2 ms of 100 |

The bot's own clock stayed at 16 to 17 ms in the games that timed out: the extra time is the busy machine waking processes up late, and it does not shrink with the time scale. At 0.4% of games, a 200-game smoke test with no timeout allowed would fail a sound bot about half the time.

Whether results at 20 ms hold at 100 ms was checked once: the exploration constant 0.5 beat 1.0 by 164 Elo at scale 0.2 and by 162 Elo at full time.

## Decision

1. **Evaluations keep a time scale of 0.2 and add 5 ms to every limit** (`TIME_TOLERANCE_MS` in `evaluation.env`, `--time-tolerance-ms` in the arena). Bots are not told about the tolerance and keep budgeting for the scaled limit, so it only absorbs the machine's delays. The smoke test still allows no fault.
2. **The CodinGame compatibility checks keep CodinGame's exact limits:** in CI, every bundled bot plays the random bot at full time with no tolerance.
3. **Full-time calibration when the bot gets strong.** Once accepted improvements fall below 20 Elo, or a release reaches Gold league on CodinGame, whichever comes first, Claude runs the latest releases at full time (scale 1, no tolerance) to measure their real Elo and to check that improvements measured at 20 ms hold at 100 ms. If they do not, a new ADR changes the evaluation time limits. The League workflow can be started by hand with any time scale and tolerance for this.

## Consequences

- Tests stay as fast as planned in ADR 0012: about half an hour for an average SPRT.
- A bot that overshoots its scaled budget by less than 5 ms is not caught by evaluations, but the full-time CI check catches overshoots that matter on CodinGame.
- The calibration of decision 3 costs a few hours of CI time once, when it is worth it.
- If a timeout still fails a test, re-running the SPRT job on GitHub tests again instead of reusing the result. Every test posts its result on the pull request, so retries stay visible.
- On slower or busier machines than GitHub's, such as a 2-core laptop running other work, 5 ms may not be enough: local runs can raise `TIME_TOLERANCE_MS`.
