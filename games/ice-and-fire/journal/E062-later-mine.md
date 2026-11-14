---
id: E062
date: 2026-11-14
pull_request: "#NN"
parent: ice-and-fire-v026
hypothesis: In v026's CodinGame games its first mine comes early (built by turn 8 in 54% of its games as red, 94% by turn 10; the top 10 40% and 60%, the leader zasmu 14% and 62%), in the turns when the middle of the map is won, where the top 10 hold about twice its cells by turn 12; starting E058's early mine bonus at turn 10 or 11 rather than at once keeps that gold for units during the race for the middle, and beats v026.
change: The plan bot's evaluation counts E058's early bonus for a safe mine only from game turn 10 or 11 (until 15, as before), for both sides or only for red's mines.
release: none
sprt: pending
elo: none
pairs: none
full_time: none
decision: pending
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

Pending.
