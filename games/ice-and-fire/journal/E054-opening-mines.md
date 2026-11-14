---
id: E054
date: 2026-11-14
pull_request: "#323"
parent: ice-and-fire-v022
hypothesis: The top 10 build a mine by turn 7 in 30% to 38% of their games as red, while v022's opening builds none (E034) and E046 found mines in the whole opening losing; letting the opening build a mine only from game turn 5, 6 or 7, on a spot 3 or 5 steps from the enemy, buys income when the land is taken and beats v022.
change: None released. Screening only: the opening allowed to build a mine from game turn 5, 6 or 7, on a spot more than 3 or 5 steps from the enemy's territory (none in v022).
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

The study of the first 10 turns is in E051's entry, run in parallel (its own pull request).

## The change

`opening/src/econ.rs`: `Params::mine_from`, the first game turn on which
the opening builds a mine (0, any, in `Params::V003`);
`bots/plan/src/main.rs`: `OPENING_MINE_FROM` with `OPENING_SAFE_SPOT`
(`u8::MAX`, no mine: v022, which the base commit keeps). E046 screened
spots 5 and 8 from the first turn (−25 and −28).

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). Run 43575412826, seed 54054,
800 pairs each over two runners:

| Variant | Commit | Spot more than | From game turn | Elo against v022 | Score |
| --- | --- | ---: | ---: | --- | ---: |
| s3m7 | `cec772b` | 3 steps | 7 | +2.2 [−7.7, +12.1] | 50.3% |
| s3m6 | `24610f2` | 3 steps | 6 | **−23.1 [−37.9, −8.2]** | 46.7% |
| s3m5 | `862dd66` | 3 steps | 5 | **−27.4 [−42.2, −12.8]** | 46.1% |
| s5m6 | `07adf15` | 5 steps | 6 | **−27.4 [−42.6, −12.4]** | 46.1% |

Mines from turns 5 or 6 lost, from turn 7 changed nothing; none was
confirmed.

## What was learned

As in E034 and E046, a mine in our opening costs land the opening cannot
make up: the top 10's mines at turns 6 and 7 go with an expansion that
takes more of the contested land with fewer units crowded together, which
ours does not yet do. Spreading the units (E051) gained in the same round;
mines may be worth trying again on top of it. The knob was removed: the
pull request holds the journal entry alone.
