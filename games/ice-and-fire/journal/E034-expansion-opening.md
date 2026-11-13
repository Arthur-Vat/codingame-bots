---
id: E034
date: 2026-11-13
pull_request: pending
parent: ice-and-fire-v015
hypothesis: v015's opening builds a mine early (at its turn 6 or 7 in most games, where the rest of Legend trains) and plans its turns by a score that weighs income and gold more than the land nearer us than the enemy; an opening that favours expansion, by fewer or no mines or by a larger weight for that land (`voronoi`), cells or a smaller one for income, reaches the middle game with more land and beats v015.
change: Screening only so far: the opening's mines (a mine spot used only farther from the enemy, or none) and the weights of the solo planner's score (cells, income, gold, voronoi), one variant each.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13, after E033 ("I think the evaluation
should favor more the expansion (voronoi and such metrics) than economy in
the early game", then "do the next experiment. Try many variants in
parallel"). It is also the third experiment proposed by the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
"The first mine").

## Why

The opening ([`opening/`](../opening/)) plays until the middle game, by
rules (E003) whose weights and purchases the solo planner (E010) chooses
turn by turn with a beam search over our next 7 turns, as if the enemy
passed. Its score of the position reached weighs cells 4, income 3, gold
1, closeness to the enemy HQ 6, and the land nearer our cells than the
enemy's (`voronoi`) only 1; its purchases buy a mine whenever it pays back
more than a training by the horizon. E033 left it unchanged: v015 builds a
mine at its own turn 6 in 29% of games and at turn 7 in 67% to 68%, as
v014 does. In the report the rest of Legend builds that early mine in
about 6% of games, and from turn 10 our wins and losses differ most in
cells and voronoi. The report could not tell whether the early mine hurts
(it goes with the map), and earlier changes to the opening were dropped
(E011, E013), so it is screened.

## The change

`bots/plan/src/main.rs`: `OPENING_SAFE_SPOT` sets the opening's
`safe_spot` (a mine spot is used only when the enemy's territory is more
than that many steps away) in every weight set: v003's 3, 5 for fewer and
later mines, 255 for none in the opening; and the planner's `OBJECTIVE`.
At 3 and with v015's weights the bot plays as v015. The middle game is
unchanged, E033's mines included.

## Screening

All against `ice-and-fire-v015` on GitHub (`screen.yml`, 800 pairs each
over two calibrated runners; no timeout, crash or invalid answer on
either side in any match). Each variant is a commit of this pull
request. "No mine" is a safe spot of 255, "safe 5" of 5; the weights are
the planner's score's (v015: cells 4, income 3, gold 1, voronoi 1, reach
6), only those named changed.

First round, runs 43425890486 (seed 34001) and 43425893737 (seed 34002):

| Variant | Commit | Elo against v015 | Score |
| --- | --- | --- | ---: |
| no mine | `03b96a3` | **+22.6 [+7.0, +38.3]** | 53.2% |
| safe 5 | `6ae691b` | +8.7 [−0.1, +17.5] | 51.2% |
| voronoi 3 | `1610fe7` | +1.3 [−13.1, +15.7] | 50.2% |
| voronoi 6 | `973808c` | **−17.4 [−33.0, −1.9]** | 47.5% |
| voronoi 10 | `553f874` | **−29.2 [−45.4, −13.0]** | 45.8% |
| no mine, voronoi 3 | `339a977` | −0.9 [−16.8, +15.1] | 49.9% |
| no mine, voronoi 6 | `fbe2ac1` | −2.2 [−17.7, +13.4] | 49.7% |
| safe 5, voronoi 3 | `6032904` | −6.5 [−20.9, +7.9] | 49.1% |
| income 2 | `35f5af6` | +10.4 [−2.8, +23.7] | 51.5% |
| income 1 | `8e67817` | **+21.7 [+7.2, +36.4]** | 53.1% |
| income 1, voronoi 3 | `184379e` | −1.3 [−16.8, +14.2] | 49.8% |
| cells 6 | `c3da14b` | −6.1 [−18.4, +6.2] | 49.1% |
| cells 6, voronoi 3 | `edba09b` | **−14.8 [−29.3, −0.3]** | 47.9% |
| gold 0 | `f5f5660` | +2.2 [−12.4, +16.8] | 50.3% |
| no mine, income 1 | `adaed51` | **+43.7 [+27.9, +59.6]** | 56.2% |
| no mine, cells 6, voronoi 3 | `a858da1` | −4.8 [−20.6, +11.0] | 49.3% |
