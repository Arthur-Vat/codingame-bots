---
id: E041
date: 2026-11-13
pull_request: "#303"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games against the top 10, 31% of the gold we spent on trainings was dead after the opponent's next ply (18% for the top 10), most often after one capture at a joint cut a chain off, and E038 and E039 showed that putting that danger into the evaluation counts it twice; v018's generator offers a training on our own cell at level 1 only and a tower only where it lowers the cells the enemy can take, so offering level-2 defenders and towers at the joints the enemy can take gives the search the defensive turns it lacks and beats v018.
change: None released. Screening only: the plan bot's generator also offering, on our empty active cells that the enemy can take at its next turn and whose loss cuts off cells, a training at level 2, and towers on or next to such joints when they cut off enough cells.
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

The code is commit `163a87a` of this branch's history (at v018's
settings), each variant one commit before it; this pull request merges
`main` and takes its code back, keeping the entry.

## Screening

Against `ice-and-fire-v018` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "Defenders" are level-2
trainings on the exposed joints that cut off at least the given number
of cells, or on every exposed empty cell (`DEFENDER_MIN_CUT` 0);
"towers" those offered at joints that cut off at least the given number.

First round, run 43485504659, seed 41041, 800 pairs each over two runners:

| Variant | Commit | Defenders | Towers | Elo against v018 | Score |
| --- | --- | --- | --- | --- | ---: |
| d2j | `5202fa9` | from 1 | none | −5.2 [−13.5, +3.0] | 49.2% |
| d2j4 | `843eefc` | from 4 | none | +1.7 [−6.2, +9.7] | 50.2% |
| d2a | `4535b13` | every exposed cell | none | −7.4 [−16.5, +1.7] | 48.9% |
| t4 | `cc465fc` | none | from 4 | +7.4 [−1.7, +16.5] | 51.1% |
| t8 | `d4364e8` | none | from 8 | −0.4 [−8.6, +7.8] | 49.9% |
| d2jt4 | `211dc61` | from 1 | from 4 | +5.2 [−3.9, +14.3] | 50.7% |

The two best again, run 43488317537, seed 41042, 1,600 pairs each over
four runners: t4 +2.4 [−4.3, +9.1] (50.3%), d2jt4 −0.0 [−6.8, +6.8]
(50.0%).

## What was learned

Offering defenders and towers at the joints the enemy can take changed
little: the search had the choices and did not gain by them, so the
leak in efficiency against the top 10 is not a lack of defensive
actions. The study of our replies to the top 10's long chains points instead at
their standing threat on our HQ: after 28% of their long chains they
could pay for a chain to our HQ at their next turn (we after 4% of ours),
and the cuts that would have punished them mostly left our HQ open; E042
tries that next. The owner's
choice before the confirmation ("Confirm the 3 leaders (Recommended)",
three leaders across E040 and E041) was to send a leader that held to the SPRT and drop the rest; none held.
