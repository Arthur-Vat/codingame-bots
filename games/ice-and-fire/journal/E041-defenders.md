---
id: E041
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games against the top 10, 31% of the gold we spent on trainings was dead after the opponent's next ply (18% for the top 10), most often after one capture at a joint cut a chain off, and E038 and E039 showed that putting that danger into the evaluation counts it twice; v018's generator offers a training on our own cell at level 1 only and a tower only where it lowers the cells the enemy can take, so offering level-2 defenders and towers at the joints the enemy can take gives the search the defensive turns it lacks and beats v018.
change: The plan bot's generator also offers, on our empty active cells that the enemy can take at its next turn and whose loss cuts off cells, a training at level 2, and towers on or next to such joints when they cut off enough cells.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Defenders in the generator"), after
E038 and E039 lost; run beside E040 (replies to long plans searched
harder) and a study of our replies to the top 10's long chains. E038's
entry gives the study of chains and trades this comes from.

## Why

A level-1 enemy training cannot take a cell held by our level-1 unit,
and a level-2 one cannot take a cell held by a level-2 unit; a tower
guards its cell and its neighbours. v018's generator trains at the lowest
level that may enter a cell, so on our own empty cells at level 1 only,
and offers a tower on a border cell only where it lowers the cells the
enemy can take or raises its chain to our HQ (E026), whatever lies behind
the cell. In one of the study's games (918418336, turn 13) v018 answers
a position where v014 pushed a chain and lost 14 cells by building two
towers; the top 10 train level-2 units about twice as often as we do.

## The change

`bots/plan/src/actions.rs`: with `DEFENDER_LEVEL`, a training at that
level on each of our empty active cells that the enemy can take at its
next turn (E037's reach) and whose loss cuts off at least
`DEFENDER_MIN_CUT` cells (`ice_and_fire_opening::solo::severed`); with
`TOWER_MIN_CUT`, a tower on each of our free cells on or next to such a
cell that cuts off at least that many, beside E026's. Both `None` in
v018, which the base commit keeps; the search scores them as any other
action.

## Screening

Pending.
