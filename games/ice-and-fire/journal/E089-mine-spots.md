---
id: E089
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Valuing the mine spots a side holds safely, more early in the game, and a small preference for spots in the opening's moves and trainings, make the bot take and keep the spots that unlock income for the rest of the game, and beat ice-and-fire-v032.
change: The evaluation counts each mine spot a side holds on an active cell (safe from the enemy's next turn, by default), a weight falling to nothing at game turn 40; the opening's cell choice gets its existing `spot` weight; each setting at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner (2026-11-26): the bot seems to ignore mine spots that are not
built yet, though taking one unlocks income for the rest of the game.
v032's evaluation counts a built mine (its income, and E033/E058's bonus
when safe), not a spot: an empty spot is worth any other cell.

A measure first, over v032's 1,925 CodinGame games replayed to game turns
8, 12 and 16 (`arena/examples/spots.rs`):

| Game turn | Spots held, v032 | Spots held, opponent | Per 100 active cells, v032 | Per 100 active cells, opponent | Free spots next to the land, v032 | Same, opponent |
|---|---|---|---|---|---|---|
| 8 | 2.59 | 2.29 | 11.7 | 9.5 | 1.08 | 1.67 |
| 12 | 3.70 | 3.44 | 10.3 | 9.7 | 0.94 | 1.31 |
| 16 | 4.69 | 3.91 | 10.6 | 10.5 | 0.61 | 0.74 |

v032 already holds more spots than its opponents, and more per cell (the
maps have about 9.5 spots per 100 valid cells). Against zasmu, the gap is
land and building: zasmu holds 4.15 spots to 3.42 at game turn 12 with 39
active cells to 33, and has built 1.96 mines to 1.09 (3.30 to 1.59 at game
turn 16). The owner chose to screen the term all the same, with a very
small term in the opening.

## The change

- `bots/plan/src/eval.rs`: `SPOT_WEIGHT` (0 in v032) per mine spot on a
  side's active cells, built or not (so that building a mine keeps the
  value), times `(SPOT_UNTIL - turn) / SPOT_UNTIL` (`SPOT_UNTIL` 40);
  with `SPOT_SAFE`, only the spots the enemy cannot take at its next turn.
  The enemy's spots count against us.
- `bots/plan/src/main.rs`: `OPENING_SPOT` (0 in v032), the opening's
  existing `Params::spot`, how much more a mine spot is worth to its moves
  and trainings (its other cell weights: 3 per free neighbour, 2 to 6 for
  the shape).

## Screening

Pending.
