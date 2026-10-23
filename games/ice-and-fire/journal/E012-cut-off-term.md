---
id: E012
date: 2026-10-23
pull_request: "#200"
parent: ice-and-fire-v005
hypothesis: The metrics report of 1,000 games showed that the joints of a territory do not tell the winner while cells actually cut off do, so a middle-game shape term that counts only the cuts the enemy can reach, rather than the joints, beats v005.
change: None released. Local screening only: v005 whose evaluation measures a side's weakness as the largest cut through a joint next to an enemy cell, as the sum of such cuts, or as the largest cut without counting joints, instead of the joints plus the largest cut.
release: none
sprt: not run: weaker in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-23 ("Experiment: cut-off term"), from the
interpretation of the first metrics report
([batch](../reviews/2026-10-23-v005-v004-metrics/summary.md), idea 2): over
1,000 games of v005 against v004, `joints`, `max_cut` and `total_cut` stayed
near 50% through turn 10 and never told the winner before turn 38, while
`cut_off`, the cells actually cut off, told it from turn 12.

## The change

v005's evaluation takes from each side's score 2 times its weakness, the
number of its joints plus the most cells one capture cuts off
([E010](E010-solo-opening.md)). The variants changed only what the weakness
counts, keeping the weight 2:

1. the most cells cut off by one capture of a joint next to an enemy cell
   (the cuts the enemy can reach first);
2. the sum of the cells each such joint cuts off;
3. the most cells one capture cuts off, joints not counted.

## Screening

Against `ice-and-fire-v005` at CodinGame's limits, seed 5011, 400 pairs
each, two games at a time:

| Weakness | Elo against v005 | Score as red | Score as blue |
| --- | --- | ---: | ---: |
| 1. Largest reachable cut | −15.6 [−36.7, +5.3] | 35.2% | 60.2% |
| 2. Sum of reachable cuts | −10.4 [−31.2, +10.3] | 36.2% | 60.8% |
| 3. Largest cut, no joints | −22.6 [−42.8, −2.6] | 34.5% | 59.0% |

All three were weaker; the third clearly. Counting the joints matters even
though, across games, the number of joints does not tell the winner: a
metric that does not separate winners from losers can still be worth
optimizing, since both bots already keep it low in the games that decide
the outcome, and a report of correlations does not show what a change to
the evaluation will do. The screening is the judge.
