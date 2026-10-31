---
id: E031
date: 2026-10-31
pull_request: "#NN"
parent: ice-and-fire-v014
hypothesis: v014 stops a third of its middle-game turns at its cap of 32 climbs and answers by 40 ms of a turn that CodinGame, on a machine about 1.7 times slower than ours, cuts at about 63 ms by the bot's clock; climbing until the clock ends the search and answering by a hard 48 ms beats v014 (candidate `ice-and-fire-v015`).
change: The answer leaves by 48 ms by the bot's clock (40 before); climbs and reply searches may start until about 44 ms (36), the check of the plan to play only until about 45 ms, its opponent climb stopping at about 47 ms; the cap of 32 climbs becomes 1,000, so the clock ends the search.
release: ice-and-fire-v015
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-30 and 2026-10-31 ("Can you make sure the
full budget is consistently used?", then "use the full budget (like 48ms for
the full turn ...) ... Make sure 48ms is a hard limit"). After the first
screening he chose to wait for the timing probe ("Wait for the probe"), then
to confirm at CodinGame's speed ("Confirm at CG speed (Recommended)").

## Why

In 40 v014 self-play games at CodinGame's limits (seed 4401), the first turn
used 609 of its 1,000 ms, the opening turns after it 0.0 ms in the median
(82% under 5 ms), and the middle-game turns 36.6 ms in the median, but 44% of
them ended before 35 ms, 78% of those at the cap of 32 climbs (the forced
climbs of E029 use climbs fast).

The timing probe (#227) in three self-play games in the CodinGame IDE, run
by the owner on 2026-10-31: answers held to 62, 64 and 64 ms by the bot's
clock were accepted, and the games were cut at 63, 65 and 64 ms, so
CodinGame's limit by a bot's clock is about 63 to 65 ms there, not 50. Its
workload ran 10.6 to 11.6 units a millisecond (dips to 5.0 and 8.2), against
18.9 on the session's machine: CodinGame is about 1.6 to 1.8 times slower,
so 40 ms there buys what about 24 ms buys in our screening.

## The change

- `bots/plan/src/main.rs`: `BUDGET_MS` 48 (40).
- `bots/plan/src/search.rs`: `MAX_CLIMBS` 1,000 (32); `CLIMB_SHARE` 0.79
  (0.75): the climbs at about 44 ms of the 48; the check of the plan to play
  (E027) starts only before `CHECK_SHARE` 0.84 (about 45 ms), since its cuts
  cannot stop halfway, and its opponent climb stops at `CHECK_CLIMB_SHARE`
  0.95 (about 47 ms).
- Two tests follow the new limits: the budget of a turn (48 ms) and a search
  with room enough climbing past 32.

Turns that overran the limit locally were the machine pausing the process,
not the code: a 66 ms turn against a 24 ms limit had done 999 tries, the
median of normal turns being 1,195 (2 such turns in 2,318).

## Screening

Against `ice-and-fire-v014`, 400 seat-swapped pairs each, two games at a
time on a 4-core machine (busier than usual: v014 itself had 4 to 11
timeouts in 800 games).

At CodinGame's limits on our machine (seeds 31031 and 31032):

| Variant | Elo against v014 | Timeouts (v014's) | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| cap 64, 40 ms | +2.6 [−12.9, +18.1] | 6 (7) | 37.0, 39.1, 44.8 ms |
| no cap, 40 ms | +14.8 [−1.9, +31.6] | 3 (11) | 37.0, 39.1, 50.8 ms |
| no cap, 44 ms | −8.7 [−25.7, +8.3] | 5 (4) | 41.3, 43.2, 45.3 ms |
| no cap, 48 ms | +6.9 [−10.9, +24.8] | 8 (10) | 44.8, 47.1, 49.4 ms |

At CodinGame's speed (both bots' time scaled to 0.6, `--time-scale 0.6`,
10 ms of tolerance the bots are not told of, against this machine's pauses):

| Variant | Seed | Elo against v014 | Timeouts (v014's) |
| --- | --- | --- | ---: |
| no cap, 48 ms | 31033 | +16.5 [−2.3, +35.4] | 6 (8) |
| no cap, 48 ms | 31035 | +18.3 [−1.9, +38.5] | 8 (7) |
| **both, 800 pairs** | | **+17.4 [+3.6, +31.2]** | 14 (15) |

More time is worth more on CodinGame's slower machine: at our speed v014
searches enough already, at CodinGame's it does not. The candidate is the
48 ms version, `ice-and-fire-v015`; the SPRT on CI judges it at our speed,
where the gain is smaller.
