---
id: E091
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Repairing each plan the search finds against its own worst reply (dropping a command, or adding a counter or an action), then searching the repaired plan's replies afresh and repairing again, finds plans that hold against the replies they open, and beats ice-and-fire-v032.
change: After a climb's plan has had its replies searched, the search repairs it up to `REPAIR` times (`search/repair.rs`), each repaired plan getting its own replies; each setting at its v032 value is v032.
release: none
sprt: not run: level with v032 at 1,200 pairs
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The owner (2026-11-26), after E090: the search should place its towers,
and its turn, by the opponent's reply to it: our turn, the reply, our turn
changed to counter it, and so on. v032 already searches the replies to each
plan and keeps the most damaging in a memory against which every later plan
is scored, but it counters them only by climbing a new plan from the start
of the turn (and, for one cell, E044's blocker towers); it never changes the
plan the reply was found against.

E090's spending splits found plans the search scored higher, yet lost
games: they were judged against the replies of the first climb only. So
here every repaired plan has its replies searched afresh before it can be
repaired again or chosen (the final choice scores each plan by the worst of
the memory and its own replies, E027).

## The change

`bots/plan/src/search/repair.rs`, used by `search.rs` for our own search:

- `REPAIR` (0 in v032): after a climb's plan has had its replies searched,
  it is repaired against its worst reply, then the repaired plan's replies
  are searched and join the memory, up to `REPAIR` times, as long as a
  repair raises the plan's score and gives a plan not found before.
- A repair takes up to `REPAIR_STEPS` (4) steps, each the best of dropping
  one of the plan's commands, adding a counter to the worst reply (a tower
  on each free cell of ours the reply enters or next to one, a unit of
  level 1 or 2 on each free cell of ours it enters), and, with
  `REPAIR_ANY`, adding any action the climb is offered; a step must raise
  the plan's score against the memory.

## Before the screening

The `codingame-judge` run of each setting on 929 of v032's CodinGame plies
from game turn 7 (decided positions left out), 600 tries a search (the
replies searched are not counted as tries), and the tries a millisecond on
the bench bank:

| Setting | Above v032's play | Same as v032's play | Tries a ms |
|---|---|---|---|
| v032 | 9.6% | 80.1% | 21.5 |
| REPAIR 1 | 26.2% | 43.6% | 23.6 |
| REPAIR 2 | 29.0% | 39.9% | 25.5 |
| REPAIR 4 | 29.1% | 39.0% | 31.6 |
| REPAIR 2, counters only | 26.4% | 49.6% | 19.8 |

## Screening

Each variant against `ice-and-fire-v032`, 400 pairs, full games, no
faults on either side (run 45882241276):

| Variant | Settings | Elo |
|---|---|---|
| repair1 | `REPAIR` 1 | +7.8 [-13.9, +29.6] |
| repair2 | `REPAIR` 2 | +17.4 [-4.1, +39.0] |
| repair4 | `REPAIR` 4 | +11.3 [-9.7, +32.4] |
| repair2-counters | `REPAIR` 2, no `REPAIR_ANY` | -0.9 [-21.7, +19.9] |
| repair2-steps2 | `REPAIR` 2, `REPAIR_STEPS` 2 | +19.1 [-2.9, +41.3] |

Every variant that may add any action leans positive, the first of this
day's experiments to do so as a family; none has an interval above 0. The
counters alone are level with v032: the gain comes from changing the plan
freely against its worst reply, not from the targeted counters.

## Confirmation

The two best and two neighbours again, 1,200 pairs over three shards each
(the owner's choice; run 45884703683), no faults:

| Variant | Settings | Elo |
|---|---|---|
| repair2 | `REPAIR` 2 | -5.2 [-16.9, +6.4] |
| repair2-steps2 | `REPAIR` 2, `REPAIR_STEPS` 2 | +2.6 [-9.6, +14.9] |
| repair3-steps2 | `REPAIR` 3, `REPAIR_STEPS` 2 | -0.3 [-12.2, +11.6] |
| repair2-steps1 | `REPAIR` 2, `REPAIR_STEPS` 1 | -11.3 [-22.8, +0.2] |

The lean of the first screening does not hold: the repair is level with
v032, as E088's leaners were.

## Result

Dropped (the owner, 2026-11-26).
