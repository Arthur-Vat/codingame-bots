---
id: E017
date: 2026-10-24
pull_request: "#208"
parent: ice-and-fire-v007
hypothesis: The weights of the plan bot's evaluation were set by hand; weights fitted to the outcomes of v007's own games, so that the evaluation predicts the winner better, make the search choose better plans and beat v007.
change: None released. Local screening only: v007 whose evaluation weighs its terms (gold, income, active cells, units, towers, mines, each side's pressure and debt, joints, the largest cut) with weights fitted by logistic regression on 26,889 middle-game positions of 1,000 v007 self-play games, or halfway between those and v007's.
release: none
sprt: not run: weaker in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-24 after E016 ("Drop E016, then tune eval
(Recommended)").

## The fit

1,000 games of `ice-and-fire-v007` against itself at CodinGame's limits
(500 seat-swapped pairs, seed 17017; no timeout, every game decided). A
review-tool example replayed each record and wrote, at the start of every
middle-game ply (from the first where the side to act can pay a chain of
trainings to an enemy cell), the evaluation's terms for both sides and
whether the side to act won: 26,889 positions. A logistic regression
(Newton's method, a small L2 penalty, a seat term) was fitted on four
fifths of the games and checked on the rest.

| Term (gold = 1) | v007 | Fitted, without mines | Fitted, with mines |
| --- | ---: | ---: | ---: |
| Income | 3 | 6.7 | 4.3 |
| Active cells | 4 | 6.2 | 7.9 |
| Units' training cost | 1 | 0.74 | 0.36 |
| Towers | 10 | 25 | 25 |
| Mines | 0 | — | 44 |
| Own / opponent's pressure | +1 / −1 | +1.0 / −0.85 | +1.1 / −0.9 |
| Own / opponent's debt | −2 / +2 | −0.8 / 0 | −1.0 / +0.35 |
| Joints | −2 | −6.8 | −7.2 |
| Largest cut | −2 | −1.6 | −1.3 |

On the held-out games the fit without mines predicted the winner barely
better than v007's weights (log loss 0.291 against 0.294, 86.5% against
86.4% of positions); with mines it fit the training games better and the
held-out ones worse (0.297), so mines overfit.

## Screening

Against `ice-and-fire-v007` at CodinGame's limits, 400 seat-swapped pairs
each, seed 17018, two games at a time. The threat to the HQ stayed as in
v007 (half a win).

| Variant | Elo against v007 | Score as red | Score as blue | Variant's timeouts | Median answer (v007's) |
| --- | --- | ---: | ---: | ---: | --- |
| fit (without mines) | **−78.6 [−102.7, −55.2]** | 32.0% | 45.8% | 0 | 5.3 ms (9.9) |
| half (halfway to v007's) | **−37.5 [−60.8, −14.5]** | 39.0% | 50.2% | 0 | 5.6 ms (9.4) |
| fitmines | **−73.2 [−96.6, −50.4]** | 30.8% | 48.5% | 2 | 5.2 ms (9.8) |

All three are clearly weaker, the more so the further from v007's weights.

## What was learned

- Weights that predict who wins a position are not weights that choose
  plans well. The fit learns what goes with being ahead (cells, towers,
  a solid shape), and gives little to what a plan must trade for it
  (units, 0.4 to 0.7 a gold of their cost; the opponent's debt, about 0),
  so the search spends less on the units and captures that win games.
- The fitted variants also answered in about half v007's time: their
  climbs found nothing better sooner, a flatter landscape for the search.
- v007's hand-set weights predict the winner about as well as the best fit
  of the same terms, so a better evaluation needs new terms, not new
  weights on these.
- Two of the plan bot's tests encode v007's evaluation (the debt counted
  twice for both sides, a cheap territory gain taken); both failed with
  the fitted weights, which is consistent with the games.

The owner chose next a search that looks one turn deeper ("Drop E017,
then 2-turn look (Recommended)").

The variants' code was not kept: it is in this pull request's first
commit (`games/ice-and-fire/review/examples/features.rs`, the fitting
script `fit_weights.py` beside it, and the weighted evaluation behind a
build-time switch in the plan bot's `eval.rs`), removed by the next.
