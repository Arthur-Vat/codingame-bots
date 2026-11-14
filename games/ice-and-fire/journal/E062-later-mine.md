---
id: E062
date: 2026-11-14
pull_request: "#NN"
parent: ice-and-fire-v026
hypothesis: In v026's CodinGame games its first mine comes early (built by turn 8 in 54% of its games as red, 94% by turn 10; the top 10 40% and 60%, the leader zasmu 14% and 62%), in the turns when the middle of the map is won, where the top 10 hold about twice its cells by turn 12; starting E058's early mine bonus at turn 10 or 11 rather than at once keeps that gold for units during the race for the middle, and beats v026.
change: None released. Screening only: the plan bot's evaluation counting E058's early bonus for a safe mine only from game turn 10 or 11 (until 15, as before), for both sides or only for red's mines.
release: none
sprt: not run: gain too small for an SPRT (owner's choice)
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form after the opening study S0,
[research/codingame/2026-11-14-opening-study.md](../research/codingame/2026-11-14-opening-study.md):
"Both in parallel", then "Later start, 3 to 4 variants"), run beside E061.

## Why

The study S0 of v024 to v026's CodinGame games: up to turn 7 our opening
matches the top 10's (cells, unit turns), but from turn 7 to 12 the top 10
take the middle of the map (within 2 steps of even between the HQs): at
turn 12 they hold 8.1 middle cells as red and 10.4 as blue, v026 4.3 and
6.8, with the same land on their own side. In those turns v026 builds its
first mine earlier than they do:

| Seat | Who | Mine built by turn 8 | By turn 10 | By turn 12 |
| --- | --- | ---: | ---: | ---: |
| red | v026 | 54% | 94% | 97% |
| red | top 10 | 40% | 60% | 73% |
| red | zasmu | 14% | 62% | 83% |
| blue | v026 | 37% | 74% | 88% |
| blue | top 10 | 31% | 54% | 66% |
| blue | zasmu | 34% | 73% | 91% |

(Top 10 and zasmu from all their collected games, most against v014.)
E058's screening found that ending the bonus at turn 12 rather than 15 did
less; this experiment moves its start instead.

## The change

`bots/plan/src/eval.rs`: `EARLY_MINE_FROM`, the first game turn on which
a safe mine of each side (red's, then blue's) counts E058's early bonus
(`[0, 0]`: v026, which the base commit keeps). Before it, a safe mine
counts E033's bonus alone (one per 4 active cells).

## Screening

Against `ice-and-fire-v026` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; one timeout of r10 in 1,600 games, no other fault). "From" is
`EARLY_MINE_FROM`, red's then blue's. Run 43622840090, seed 62062, 800
pairs each over two runners:

| Variant | Commit | From | Elo against v026 |
| --- | --- | --- | --- |
| r10 | `9689619` | 10, 0 | +9.1 [−3.0, +21.3] |
| f11 | `d6ee50f` | 11, 11 | +9.1 [−4.0, +22.3] |
| f9 | `f953b37` | 9, 9 | +2.6 [−6.1, +11.3] |
| f10 | `8df4870` | 10, 10 | −9.6 [−23.4, +4.2] |

No variant gained significantly, and f10 below its neighbours f9 and f11
suggests noise rather than a trend.

The two leaders again on a new seed, run 43624891394, seed 62063, 1,600
pairs each over four runners (no fault):

| Variant | Commit | From | Elo against v026 |
| --- | --- | --- | --- |
| r10 | `9689619` | 10, 0 | +8.5 [−0.1, +17.0] |
| f11 | `d6ee50f` | 11, 11 | −1.3 [−10.9, +8.3] |

r10 held its lead (+9.1 then +8.5, about +8.7 over 2,400 pairs); f11 did
not. A third run with r10's neighbours (red only), run 43627207056, seed
62064, 1,600 pairs each over four runners (one timeout of v026, no other
fault):

| Variant | Commit | From | Elo against v026 |
| --- | --- | --- | --- |
| r10 | `9689619` | 10, 0 | +4.3 [−4.3, +12.9] |
| r11 | `b554afe` | 11, 0 | +2.6 [−6.5, +11.7] |
| r9 | `86d091d` | 9, 0 | −1.1 [−7.4, +5.2] |

Over its three runs (4,000 pairs) r10 scores about +7 (the pairs-weighted
mean of +9.1, +8.5 and +4.3); its neighbours r9 and r11 do not confirm a
trend. Starting the bonus later for red's mines only fits the study: as red,
v026 built its first mine by turn 8 in 54% of its games, the leader in
14%; as blue the two are close (37% and 34%).

## Decision

Dropped by the owner on 2026-11-14 ("Drop E062"): r10's gain, about +7
over 4,000 pairs in three screenings, falling from +9 to +4, is too small
to spend an SPRT on, and its neighbours (r9, r11) show no trend.

## What was learned

- **The first mine's turn is not where the middle is lost.** v026 builds
  its first mine earlier than the top 10 and the leader, but delaying
  E058's bonus by a few turns changes the result little either way: from
  turn 10 for red only is a little ahead, from turn 10 for both seats a
  little behind, from turn 9 or 11 level.
- **Noise at 800 pairs is about ±12 Elo**: f10 at −9.6 between f9 at
  +2.6 and f11 at +9.1 in the first round, f11 from +9.1 to −1.3 when
  confirmed. Leaders of a first round of 800 pairs should be confirmed
  before any conclusion, as here.

The code stays reachable in this pull request's commits (the base
`ab0fd44` and the variants above); the branch's head holds the journal
entry alone.

