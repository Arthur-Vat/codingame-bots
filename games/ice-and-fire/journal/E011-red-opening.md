---
id: E011
date: 2026-10-23
pull_request: "#NN"
parent: ice-and-fire-v005
hypothesis: The metrics report of 1,000 games showed v005 winning 46% as red against 67% as blue and pushing little toward the enemy as red, so an opening whose plans, as red only, pull harder toward the enemy HQ and penalize shape less beats v005.
change: None released. Local screening only: v005 whose turn-1 solo plan uses, as red, an objective with a stronger pull toward the enemy HQ (reach 10 or 14 instead of 6), halved shape penalties (cut, joints and line ends 1 instead of 2), or both; blue's objective unchanged.
release: none
sprt: not run: weaker in local confirmation
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-23 ("Experiment: red opening"), from the
interpretation of the first metrics report
([batch](../reviews/2026-10-23-v005-v004-metrics/summary.md), idea 3): v005
won 231 of its 500 games as red (46%) and 336 as blue (67%), and at turn 10
as red it had 0.2 cells nearer the enemy HQ against 2.9 for v004 as blue.
Both bots push less as red, though (v004 as red 0.6, v005 as blue 1.4), so
part of the gap is the seat's: blue starts with one more cell.

## The change

`Planner::Solo` took one objective for both seats (E010). The variant gave
it one objective a seat and changed red's only:

1. reach 10 (the pull toward the enemy HQ, 6 in v005);
2. reach 14;
3. cut, joints and line ends 1 (2 in v005);
4. reach 10 with the shape penalties halved.

## Screening

Against `ice-and-fire-v005` at CodinGame's limits, two games at a time. As
blue the variant plays exactly as v005, so its games as blue measure v005's
own red seat: v005 as red scored about 37% against itself on these maps.

| Red's objective | Seed | Pairs | Elo against v005 | Score as red | Score as blue |
| --- | ---: | ---: | --- | ---: | ---: |
| 1. Reach 10 | 5011 | 400 | −17.4 [−36.7, +1.8] | 32.5% | 62.5% |
| 2. Reach 14 | 5011 | 400 | −5.2 [−24.0, +13.6] | 33.0% | 65.5% |
| 3. Shape penalties halved | 5011 | 400 | −5.2 [−23.4, +13.0] | 36.2% | 62.2% |
| 4. Reach 10, penalties halved | 5011 | 400 | +16.5 [−3.6, +36.8] | 41.2% | 63.5% |
| 4, confirmation | 6011 | 600 | **−18.5 [−35.3, −1.9]** | 35.2% | 59.5% |

The fourth looked better as red on the screening maps (41% against about
37%), and lost on 600 new pairs (35% as red, where v005 as red scored about
40.5% against itself). The screening's gain was noise. Red's weakness is
not fixed by a harder push in the opening's plans; the seat itself, and what
happens after the opening, are left to later work.

The variants' code was not kept: an objective per seat in
`ice_and_fire_opening::planned::Planner::Solo` and a red objective in the
plan bot, built in a scratch worktree for the screening.
