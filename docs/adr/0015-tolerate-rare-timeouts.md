# 0015. Tolerate rare timeouts: up to 1% of games, counted as losses

- Status: accepted
- Date: 2026-10-08
- Supersedes: the "no timeout" part of decision 5 of [ADR 0012](0012-evaluation.md) and of decision 1 of [ADR 0013](0013-evaluation-time-limits.md) (smoke test), and of decision 3 of [ADR 0014](0014-full-time-confirmation.md) (confirmation)

## Context

Since ADR 0012, a single timeout of the candidate fails an evaluation: smoke test, SPRT and full-time confirmation alike. The rule was meant to catch broken bots, but it also forces a wide margin. `uttt-v006` searches for 82 ms of the 100 allowed, and its slowest answers in 1,000 full-time games reach 87 to 90 ms.

The owner asked whether a timeout in 1 or 2 games out of 1,000 matters, since what counts is the expected score. The experiments so far say it does not:

- **More time buys strength.** E006 measured +30.7 Elo at full time for 1.29× more iterations, about 85 Elo per doubling. Searching for 90 ms instead of 82 would be worth about +11 Elo, and 94 ms about +17.
- **A timeout loses one game.** Near a 50% score, one Elo is worth about 0.14% of the score, so losing a fraction r of games on time costs about 350 × r Elo: 0.7 Elo for 2 games in 1,000, and 3.5 Elo for 1%.

What the evaluation cannot measure is the timeout rate on CodinGame's machines, which may be higher than on GitHub's runners. The owner saw no timeout with `uttt-v004` (82 ms budget). How CodinGame measures the 100 ms, and whether it allows any slack, is not known ([docs/CODINGAME.md](../CODINGAME.md)).

## Decision

The owner agreed to tolerate rare timeouts and left the cap open; the value below is Claude's recommendation.

1. **A timeout still loses its game, but no longer fails an evaluation on its own.** In each stage (smoke test, SPRT, full-time confirmation), the candidate may lose up to `MAX_TIMEOUT_RATE` of its games on time, rounded down. Crashes and invalid answers still fail at once. The arena's `--max-timeout-rate` sets this tolerance for `--expect-no-faults` and `--expect-no-faults-from`.
2. **`MAX_TIMEOUT_RATE` is 1%** (`evaluation.env`): 2 timeouts in the 200-game smoke test, 10 in the 1,000-game confirmation. At 1%, timeouts cost about 3.5 Elo on the runners. Even if CodinGame's machines time out three times as often, the cost is about 10 Elo, still below the +10 to +17 Elo that more search time can buy. A higher cap could let a bot that does well on the runners lose its gain on CodinGame. A lower one, 0.5% as first proposed, would leave less room to use the time.
3. **CI's check of every bundled bot against random at full time uses the same rate.** CI's other arena checks, between bots that never look at the clock, still allow no timeout.
4. **The arena measures margins.** Every summary gives the median, 99th and 99.9th percentiles of each bot's answers after the first, so that timing decisions rest on the distribution, not only the slowest answer.

## Consequences

- An experiment can trade margin for search time, and the full-time confirmation measures the trade: its Elo includes the games lost on time.
- A broken bot that times out often still fails, from 1% of its games.
- The SPRT at 20 ms counts timeouts as losses too; there, the 5 ms of tolerance absorbs most of the runners' delays (ADR 0013).
- Only CodinGame's games show the real timeout rate, and only the owner can see them. After pasting a release that searches longer, the owner checks its games for timeouts; if they come more often than 1 in 100 games, the next release searches less.
