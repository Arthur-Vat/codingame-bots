---
id: E063
date: 2026-11-14
pull_request: "#NN"
parent: ice-and-fire-v026
hypothesis: The search scores a plan by the worst of the opponent replies it models, and in v026's CodinGame games it models about twice the damage the real reply does (the real reply is worse than every modeled one in about 1 ply in 10); at an equal evaluation our losses hold more gold and less land than our wins, as a search that over-defends would; scoring a plan by a blend of its worst reply and the opponent passing makes the search pay less for threats that do not come, and beats v026.
change: The plan bot's search scores a plan by 3, 2 or 1 quarters of its worst modeled reply and the rest of the evaluation if the opponent passes (4 quarters, the worst alone, in v026).
release: none
sprt: pending
elo: none
pairs: none
full_time: none
decision: pending
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form after the overconfidence study,
[research/codingame/2026-11-14-overconfidence.md](../research/codingame/2026-11-14-overconfidence.md):
"Both in parallel"), run beside E064.

## Why

The study of v023 to v026's CodinGame games (9,542 of our plies, judged by
v026's search): on the 8,264 plies whose values are material, the modeled
damage (the worst modeled reply against the opponent passing) is −68 on
average and the damage the real reply did −33; the ratio runs from 1.5 to
3 at every level of danger, and the real reply is the worst case in about
1 ply in 10. At an equal evaluation (−50 to +50, game turns 8 to 17), the
plies of games we lost had more gold (+26 against +15 in our wins) and less
land (−9.0 cells against −5.7) than those of games we won.

## The change

`bots/plan/src/search.rs`: `WORST_QUARTERS`, the quarters of a plan's score
its worst modeled reply makes; the rest is the evaluation if the opponent
passes (`after_reply` with no command), computed once a plan. At 4 (v026,
which the base commit keeps) the score is the worst reply alone and the
pass is not computed. The search's bound on a plan's score is carried to
the worst reply so that it still stops early.

## Screening

Pending.
