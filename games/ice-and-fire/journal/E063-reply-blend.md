---
id: E063
date: 2026-11-14
pull_request: "#338"
parent: ice-and-fire-v026
hypothesis: The search scores a plan by the worst of the opponent replies it models, and in v026's CodinGame games it models about twice the damage the real reply does (the real reply is worse than every modeled one in about 1 ply in 10); at an equal evaluation our losses hold more gold and less land than our wins, as a search that over-defends would; scoring a plan by a blend of its worst reply and the opponent passing makes the search pay less for threats that do not come, and beats v026.
change: None released. Screening only: the plan bot's search scoring a plan by 3, 2 or 1 quarters of its worst modeled reply and the rest of the evaluation if the opponent passes (4 quarters, the worst alone, in v026).
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
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

Against `ice-and-fire-v026` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no fault). "Worst" is `WORST_QUARTERS`. Run 43631102635, seed 63063,
800 pairs each over two runners:

| Variant | Commit | Worst reply | Pass | Elo against v026 |
| --- | --- | ---: | ---: | --- |
| w3 | `f7eec96` | 3/4 | 1/4 | −5.2 [−18.6, +8.2] |
| w2 | `a52a39b` | 1/2 | 1/2 | **−41.9 [−56.3, −27.6]** |
| w1 | `cd05b74` | 1/4 | 3/4 | **−133.9 [−150.5, −118.0]** |

The less the worst reply counts, the more the bot loses, steeply.

## What was learned

- **Planning against the worst modeled reply is right**, although the
  opponent's real reply is that bad in only about 1 ply in 10 and does
  about half the modeled damage on average. Giving a quarter of the score
  to the opponent passing is already no better, half loses 42 Elo, three
  quarters 134. The worst-case score is what keeps the plans safe against
  the replies a strong opponent does find: this blend, at least, does
  not win back the land we lose.
- The overconfidence study's other reading stands: what the search misses
  is rare large cuts it cannot construct (E064 tries to model them).

The code stays reachable in this pull request's commits (the base
`8b1b9de` and the variants above); the branch's head holds the journal
entry alone. Dropped by the owner on 2026-11-14 ("Drop E063").

