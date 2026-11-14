---
id: E054
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: The top 10 build a mine by turn 7 in 30% to 38% of their games as red, while v022's opening builds none (E034) and E046 found mines in the whole opening losing; letting the opening build a mine only from game turn 5, 6 or 7, on a spot 3 or 5 steps from the enemy, buys income when the land is taken and beats v022.
change: The opening may build a mine from game turn 5, 6 or 7, on a spot more than 3 or 5 steps from the enemy's territory (none in v022).
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

`opening/src/econ.rs`: `Params::mine_from`, the first game turn on which
the opening builds a mine (0, any, in `Params::V003`);
`bots/plan/src/main.rs`: `OPENING_MINE_FROM` with `OPENING_SAFE_SPOT`
(`u8::MAX`, no mine: v022, which the base commit keeps). E046 screened
spots 5 and 8 from the first turn (−25 and −28).

## Screening

Pending.
