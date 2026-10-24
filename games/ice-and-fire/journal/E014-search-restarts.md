---
id: E014
date: 2026-10-24
pull_request: "#204"
parent: ice-and-fire-v005
hypothesis: v005's middle-game search stops at about 10 ms in most turns because its climbs find nothing new, not for lack of time; a search that climbs again from other first actions until its time is up, with cuts of up to 5 trainings, beats v005.
change: When a climb teaches nothing new (a plan found before, or replies the memory already holds), the next climb may not start with the first action of any plan found so far, until the soft limit or 32 climbs (8 before); cuts, ours and the opponent's replies, are chains of up to 5 trainings (4 before). Time limits, memory and the opening are v005's.
release: ice-and-fire-v006
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-24: "I would like to keep improve the a code
of ice and fire bot ... 1. Make sure the time usage is optimized (except in
the early game for now) 2. Optimize the search speed". He chose the first
("Option 1: use the time (Recommended)").

## Why the time went unused

In the 1,000-game batch of v005 against v004 (seed 234), v005's answers
from its turn 9 on took 12 ms on average of CodinGame's 50 ms: half under
10 ms, and 17% at the soft limit, 24 ms, after which no new climb starts
(60% of the bot's 40 ms budget; the rest is for choosing among the plans
found). The others stopped because the search had nothing left to do: a
climb found a plan found before, or the opponent's replies to its plan were
in the memory already, so a climb against the same memory would find the
same plan. A faster search would help only the turns at the limit.

## The variants

Every variant keeps v005's opening (with its deadline, 30.4 ms after the
first turn), its lethal check and its evaluation.

| Variant | Change |
| --- | --- |
| soft30 | no new climb after 30 ms (24 in v005), answer by 40 ms |
| t35 | no new climb after 35 ms, answer by 45 ms |
| restart | when a climb teaches nothing new, climb again with the first actions of the plans found banned at the root, up to 32 climbs |
| mem | a memory of 16 replies (8), the opponent's 6 best cuts (3) after each climb, up to 16 climbs |
| rm | restart and mem |
| rm30 | restart, mem and soft30 |
| restart-cut5, restart-cut6 | restart, with cuts (ours and the replies') of up to 5 or 6 trainings (4) |
| rm30-cut5 | rm30 with cuts of up to 5 trainings |

The cut variants were added on the owner's word ("Yes you can do it") after
he asked whether the search looks for longer cuts: it does not, a climb
step must gain on its own, so a cut longer than 4 trainings was never found
or feared.

## Screening

Against `ice-and-fire-v005` at CodinGame's limits, 400 seat-swapped pairs
each, seed 14014, two games at a time.

| Variant | Elo against v005 | Score as red | Score as blue | Variant's timeouts |
| --- | --- | ---: | ---: | ---: |
| restart | **+34.9 [+13.9, +56.1]** | 44.0% | 66.0% | 2 |
| rm | **+21.7 [+2.2, +41.4]** | 42.5% | 63.8% | 0 |
| rm30 | **+49.8 [+29.2, +70.8]** | 47.2% | 67.0% | 2 |
| mem | −1.7 [−21.0, +17.5] | 39.0% | 60.5% | 0 |
| soft30 | −3.5 [−18.3, +11.4] | 38.5% | 60.5% | 1 |
| t35 | +13.0 [−3.2, +29.3] | 43.2% | 60.5% | 0 |
| restart-cut5 | **+52.5 [+32.3, +73.1]** | 45.8% | 69.2% | 2 |
| restart-cut6 | **+41.9 [+20.0, +64.1]** | 43.5% | 68.5% | 4 |
| rm30-cut5 | **+44.5 [+24.0, +65.4]** | 44.5% | 68.2% | 5 |

Only the variants with restarts gain. More time (soft30, t35) or a larger
memory (mem) alone does not: the search stops before using either.

## Confirmation

The two best, on 600 new pairs each, seed 14015:

| Variant | Elo against v005 | Score as red | Score as blue | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | ---: | ---: | --- |
| restart-cut5 | **+62.6 [+46.6, +78.9]** | 46.5% | 71.3% | 1 | 23.4, 32.2, 34.7 ms |
| rm30 | **+54.9 [+38.4, +71.6]** | 45.2% | 70.5% | 0 | 30.4, 36.5, 39.3 ms |

The owner chose restart-cut5 as the candidate ("restart + cut5
(Recommended)"): the higher in both stages, the simpler change (v005's time
limits and memory), and the one with more room under 50 ms.

## SPRT

Pending.
