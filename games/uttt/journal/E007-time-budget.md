---
id: E007
date: 2026-10-08
pull_request: "#17"
parent: uttt-v006
hypothesis: Searching for 90 ms of the 100 allowed instead of 82 adds strength worth far more than the rare games it may lose on time; E006 measured about 85 Elo per doubling of search, so 1.1× more time should be worth about +11 Elo.
change: The bot no longer subtracts an 8 ms reserve from 90% of each turn's limit: 90 ms of search instead of 82 at CodinGame's limits, 900 ms instead of 892 on the first turn. ADR 0015 now tolerates timeouts in up to 1% of games.
release: uttt-v007
sprt: accepted
elo: +17.2 [+4.6, +29.8] at 20 ms
pairs: 973
full_time: +14.6 [-1.8, +31.1] over 500 pairs, 1 timeout in 1,000 games, 99.9% of answers within 92.1 ms
decision: promoted
cg_rank: pending
---

The owner's question was whether a timeout in 1 or 2 games out of 1,000
matters, since what counts is the expected score. Near a 50% score, a
fraction r of games lost on time costs about 350 × r Elo: 0.7 Elo for 2
games in 1,000. ADR 0015 records the decision and the 1% cap.

Calibration before choosing the budget, locally at CodinGame's limits
(2-core sandbox, two games at a time, 4-ply openings, 50 pairs of each
budget against v006):

| Budget | Median answer | 99.9% of answers within | Timeouts in 100 games | Elo against v006 |
| --- | --- | --- | --- | --- |
| 82 ms (v006) | 82.1 ms | 86 to 88 ms | 1 in its 300 games | |
| 86 ms | 86.1 ms | 90.5 ms | 2 | -24.4 [-82.0, +31.9] |
| 90 ms (this release) | 90.1 ms | 94.1 ms | 0 | +59.6 [+13.3, +108.2] |
| 94 ms | 94.1 ms | 97.4 ms | 4 | -20.9 [-76.2, +33.4] |

The bot keeps its budget: 99.9% of answers come within about 4 ms of
it. The timeouts come from rare pauses of 10 to 15 ms on this busy
machine, whatever the budget, which is why v006 timed out once and 86 ms
more often than 90 ms. 94 ms leaves 6 ms for such pauses and lost 4% of
its games on time here. The Elo column is too noisy at 50 pairs to rank
the budgets; 90 ms is the largest budget that kept a 10 ms margin.

At 20 ms per move with the SPRT's 5 ms of tolerance (200 pairs, same
machine), the candidate scored +17.4 Elo [-11.3, +46.3] against v006.
Both bots lost games on time there, 6 and 7 of 400: this sandbox is
noisier than GitHub's runners, where v006 lost none of 721 games in
E006's SPRT. The 1.6 ms of extra search at that scale did not add
timeouts.

Result, from the SPRT comment on #17: smoke test 200 wins in 200 games;
SPRT accepted after 973 pairs (737 wins, 568 draws, 641 losses), LLR
2.96; confirmation at CodinGame's limits 330 wins, 382 draws, 288
losses.

The gain is about what the rate of E006 predicted (+11): +17.2 at 20 ms
and +14.6 at full time. The full-time interval reaches just below 0
(-1.8), so the confirmation alone would not prove a gain; it shows the
candidate is not clearly weaker at full time, which is what ADR 0014
asks of it.

Timeouts on GitHub's runners: none in 1,946 games at 20 ms, where the
candidate searched 18.0 ms against a limit of 25 (99.9% of answers
within 20.0 ms), and 1 in 1,000 games at full time (0.1%, a tenth of
ADR 0015's cap). At full time the median answer took 90.1 ms and 99.9%
came within 92.1 ms; the slowest that arrived took 93.2 ms. The local
sandbox, with 6 timeouts in 400 games at 20 ms, was far noisier than
the runners.

Once pasted, the owner checks v007's CodinGame games for timeouts: the
runners' rate says little about CodinGame's machines (ADR 0015).
