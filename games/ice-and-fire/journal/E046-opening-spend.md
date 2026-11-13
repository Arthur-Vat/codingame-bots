---
id: E046
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v021
hypothesis: As red, v020 and v021 bank their gold at turns 6 and 7 (a median 0 to 10 spent, 22 to 30 held at turn 7) where the top 10 of CodinGame, as red in our ranking games, train (82% and 74% of their turns) and sometimes build a mine (18% and 23%), and they end turn 10 with as many cells but 1 or 2 more income; the opening's planner weighs a gold as a third of a cell, and builds no mine since E034, so weighing gold half as much, or allowing mines on spots far from the enemy, spends the opening's gold as the top 10 do and beats v021.
change: The opening's planner weighs gold half as much (every other weight of its objective doubled), or uses a mine spot only when the enemy's territory is at least 5 or 8 steps away (none since E034).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 ("Go on with #1 in parallel"), the
first of the proposals made after the study of gold spent per turn,
after a check of red's openings in one seat.

## Why

A scratch scan of the collected games (the gold, spending, cells and
income of each side's first ten turns), red's turns only (test games put
us in red; the top 10 and the rest of Legend appear as red in our ranking
games when we play blue):

| Turn | v020 and v021 as red | Top 10 as red |
| --- | --- | --- |
| 1 to 5 | as everyone | as everyone |
| 6 | 17 gold held, 0 to 5 spent; trains 45% to 50% of turns, no mine | 17 held, 10 spent; trains 82%, a mine 18% |
| 7 | 22 to 30 held, 0 to 10 spent; trains 48% to 65%, no mine | 20 held, 20 spent; trains 74%, a mine 23% |
| 8 | 20 to 22 cells | 21 cells |
| 10 | 27 to 28 cells, income 22 to 23 | 28 cells, income 24 |

(v020: 90 test games, v021: 20; the top 10 as red: 82 ranking games.) E034
removed the opening's mines (+31) and found gold weighed 0 worse (−24.8
in screening).

## The change

`bots/plan/src/main.rs`: `OBJECTIVE`, the planner's weights (cells 3,
income 1, gold 1, cut 2, joints 2, tips 2, reach 6, voronoi 1), and
`OPENING_SAFE_SPOT` (`u8::MAX`: no mine). The variants change those
constants only; the base commit is v021.

## Screening

Pending.
