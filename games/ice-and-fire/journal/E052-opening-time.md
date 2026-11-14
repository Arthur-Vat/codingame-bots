---
id: E052
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: The opening's plan is made on the first turn in about 610 ms of the 800 ms we use of CodinGame's 1,000, over 7 of our turns, and each later turn has 40 ms of CodinGame's 50; a longer first search (900 or 950 ms), a longer horizon (8 or 9 turns) or more time a turn (45 ms) finds better openings and better turns, and beats v022.
change: The plan bot's first-turn budget is 900 or 950 ms rather than 800, its opening plans 8 or 9 of our turns rather than 7, or each turn has 45 ms rather than 40.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14: "Alright first work on improving the early
game (efficient planning until turn 7), by trying to have a similar
expansion as the one of the top bots. You should improve the evaluation
function and also maybe improve the search, you can try to go closer to
the time limit." He noted that our units are sometimes too close to each
other in the first turns, that on maps with corridors we should be first
at the choke points (then maybe lock them with a tower), and asked to
spend the gold and build mines at the best moment. One of four
experiments run in parallel on `ice-and-fire-v022` (E051 to E054).

## Why

The study of the first 10 turns is in [E051](E051-spread-units.md).

## The change

`bots/plan/src/main.rs`: `FIRST_BUDGET_MS` (800 in v022), `OPENING_TURNS`
(7) and `BUDGET_MS` (40). The variants change those constants only; the
base commit is v022. The timeouts of each variant are measured (the
screening counts them).

## Screening

Pending.
