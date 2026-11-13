---
id: E042
date: 2026-11-13
pull_request: "#304"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games the top 10, after 28% of their long chains (4 or more trainings), could pay at their next turn for a chain of trainings to our HQ (we after 4% of ours), and the cuts that would have killed three quarters of those chains mostly left our HQ open (122 of 296 lost by force in v018's search); v018's pressure term stops counting once a side can pay for that chain, so counting a standing threat on the enemy HQ makes the search keep one, which ties the enemy to its defence and protects our chains, and beats v018.
change: The plan bot's evaluation counts a side that can pay, at its next turn, for its cheapest chain of trainings to the enemy HQ the gold it would have left after paying, up to 100 (flat bonuses and other shares screened).
release: ice-and-fire-v019
sprt: accepted
elo: +20.3 [+6.6, +34.0] at CodinGame's limits
pairs: 480
full_time: +20.3 [+6.6, +34.0] (the SPRT's own, ADR 0042), 1 timeout of v019 in 960 games (v018 none), slowest answer 520.0 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Yes, screen it"), after the study of
our replies to the top 10's long chains in v014's CodinGame games (E038's
entry gives the study of chains and trades it continues).

## Why

When a top-10 player trained 4 or more units in a ply, the best of 8
cuts available to us at our next ply would have killed three quarters of
those units in 42% of cases; v014 did in 25%, and v018's search would in
28%. On the 296 positions where such a cut existed and v018 would kill
under half, v018's judge found the cut losing by force in 122 (it leaves
our HQ to a chain the enemy can pay for), every plan losing in 60, and
the cut 126 below v018's own plan (median) in the other 114: the search
is right not to punish. After their long chains the top 10 could pay for
a chain to our HQ at their next turn 28% of the time; we after ours 4%.

The shared evaluation's pressure term counts how far a side is from
paying for its cheapest chain to the enemy HQ (at most 200), and nothing
once it can: being able to strike is worth no more than being exactly
short of it.

## The change

`tactics/src/eval.rs`: `margin`, a side's gold and income less its
cheapest chain to the enemy HQ, and `evaluate_with_margins`, the shared
evaluation with both margins as its pressure term found them (no work
more). `bots/plan/src/eval.rs`: `standing(margin)`, 0 for a side that
cannot pay, else `HQ_BONUS` plus `HQ_MARGIN_HALVES` halves of the margin
up to `HQ_MARGIN_CAP` (100); it counts for each side (both 0 in v018,
which the base commit keeps). A test checks the bonus and the margins.
The candidate sets `HQ_MARGIN_HALVES` to 2 (all of the gold left, up to
100) and no flat bonus, as the screened commit `140d284`; two older tests
that add up the evaluation's terms on the corridor, where red can pay for
blue's HQ, now count this term too. The candidate's minified bundle is
byte for byte the bundle of `140d284` (41,581 characters).

## Screening

All against `ice-and-fire-v018` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "Bonus" is `HQ_BONUS`,
"share" the share of the gold left counted (`HQ_MARGIN_HALVES` halves),
"cap" the most of it counted.

First round, run 43488592743, seed 42042, 800 pairs each over two runners:

| Variant | Commit | Bonus | Share, cap | Elo against v018 | Score |
| --- | --- | ---: | --- | --- | ---: |
| b20 | `e0483e3` | 20 | none | −0.4 [−9.8, +8.9] | 49.9% |
| b40 | `cab35b0` | 40 | none | +1.7 [−8.0, +11.5] | 50.2% |
| b80 | `f071d9b` | 80 | none | +7.8 [−1.9, +17.6] | 51.1% |
| m1 | `f82e977` | 0 | half, 100 | +7.4 [−1.7, +16.5] | 51.1% |
| m2 | `140d284` | 0 | all, 100 | **+10.9 [+1.2, +20.5]** | 51.6% |
| b20m1 | `8dcd7c7` | 20 | half, 100 | +5.6 [−3.7, +15.0] | 50.8% |
| b40m1 | `af3f1dc` | 40 | half, 100 | +3.0 [−5.7, +11.8] | 50.4% |

The three best again, run 43491001609, seed 42043, 1,600 pairs each over
four runners:

| Variant | Commit | Elo against v018 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| m2 | `140d284` | **+11.7 [+5.1, +18.4]** | 51.7% | about +11 |
| b80 | `f071d9b` | +4.1 [−2.6, +10.9] | 50.6% | about +5 |
| m1 | `f82e977` | **+6.3 [+0.4, +12.2]** | 50.9% | about +7 |

The leader's neighbours, run 43491005932, seed 42044, 800 pairs each:

| Variant | Commit | Bonus | Share, cap | Elo against v018 | Score |
| --- | --- | ---: | --- | --- | ---: |
| m4 | `7fd721d` | 0 | double, 100 | **+12.6 [+3.0, +22.2]** | 51.8% |
| m2c200 | `db510ff` | 0 | all, 200 | +7.8 [−0.7, +16.3] | 51.1% |
| b40m2 | `4ef171e` | 40 | all, 100 | **+10.0 [+0.8, +19.2]** | 51.4% |

Every form gained something, the more so as the bonus grows with the
gold left; the leader held on new pairs, and its neighbours scored
alike. The candidate is m2, the form with 2,400 pairs.

## SPRT

Accepted on the pull request (run 43494036748, at 69e9fc30), at CodinGame's
speed and real limit, in blocks of 10 pairs over 8 calibrated runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v019 vs ice-and-fire-v018: 960 games, 480 complete pairs
  ice-and-fire-v019: 508 wins, 0 draws, 452 losses (score 52.9%)
  Elo of ice-and-fire-v019 over ice-and-fire-v018: +20.3 [+6.6, +34.0] (95%, from pairs)
  pairs by ice-and-fire-v019's points (0, 1/2, 1, 3/2, 2): [33, 0, 386, 0, 61]
  ice-and-fire-v019 faults: 1 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v018 faults: 0 timeouts, 0 crashes, 0 invalid answers
  slowest answer: ice-and-fire-v019 520.0 ms, ice-and-fire-v018 522.5 ms
Checked 6 rounds of 8 blocks (48 blocks counted, 64 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.15 (bounds -2.94, 2.94) after 480 pairs
Verdict: ACCEPTED: ice-and-fire-v019 is stronger than ice-and-fire-v018
```

The SPRT's +20.3 is above the screening's +11 over 2,400 pairs; its
interval holds both.
