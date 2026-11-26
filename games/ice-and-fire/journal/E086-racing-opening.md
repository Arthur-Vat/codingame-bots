---
id: E086
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: An opening whose plans meet an opponent playing at its turns and are ranked first by the fronts' centres they hold strongly (the enemy cannot take them at its next turn) wins the race to the areas of interest that v032 loses to zasmu in turns 1 to 6, without losing the fight on arrival, and beats ice-and-fire-v032.
change: The opening's simulation lets the opponent play the opening's own rules at its turns, and a new ranking of its plans (`Order::Race`) puts first the fronts' centres held strongly; each setting at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

In v032's 195 CodinGame games against zasmu (E085's entry, "The games
against zasmu"), v032, red in all of them, falls behind in the opening:
zasmu reaches the middle front's centre at game turn 7, a turn before
v032, having gained a step between turns 5 and 6, and leads by 3 cells and
11 in the Voronoi share at turn 7; v032's search scores its first turn
(turn 7) below 0 in 57% of the lost games. The opening plans turns 1 to 6
with an opponent that passes, and ranks its plans by our land, the centre
only as a tie-break. The owner (2026-11-26): build an opening that races
the opponent to the areas of interest, strong in the centre (no fight lost
right on arrival); show games first, and compare the lost fights in the
middle at game turns 6 to 10 with v032's.

## The change

- `opening/src/solo.rs`: `OPPONENT_PLAYS`, set by the bot: in the
  opening's simulation the opponent plays the opening's own rules (`econ`,
  v003's weights, the payback rule) at its turns rather than pass.
  `Order::Race { decay, middle, land_first }`: plans ranked by the fronts'
  centres ([`front_centres`]) held strongly (2 when the side holds one
  actively and the enemy cannot take it at its next turn, 1 when it can, -2
  when the enemy holds it; the middle front's `middle` times), then the
  active cells (or the land first), then as `Order::Central`.
- `bots/plan/src/main.rs`: `OPENING_RACE` (the middle's weight; 0 is
  v032's `Order::Central`), `OPENING_LAND_FIRST`, `OPENING_OPPONENT`.

## Games and screening

Pending.
