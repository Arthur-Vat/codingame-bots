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

Variants (commits on the branch): race2 (`OPENING_RACE` 2, the opponent
playing, 5c1a975c1ba2), race1 (1, 6af082908bbe), race2-noopp (2, the
opponent passing, 04841120c54f), race2-land (2, the land first,
cdf18e780ddb), opp (v032's ranking, the opponent playing, 2a78a820bfd6).

Games on the owner's request, before any screening: each variant against
v032, and v032 against itself, on 20 maps (the six CodinGame maps of E080's
study and 14 of the repository's CodinGame seeds), both seats, at
CodinGame's limits, cut at game turn 12 (`one_map`, 24 plies); measured from
the bot's side (for v032 against itself, one of its sides), 40 games each:

| Bot | Middle cells lost, turns 6 to 10 | Plies losing 3+ middle cells | All cells lost, turns 6 to 10 | Steps to its middle centre, turns 5 / 6 / 7 | Holds it at turns 8 / 10 | Cells at turn 12, its / v032's |
| --- | --- | --- | --- | --- | --- | --- |
| v032 | 0.47 | 0.03 | 0.85 | 4 / 3 / 1 | 64% / 86% | 35.5 / 36 |
| race2 | 0.85 | 0.05 | 1.50 | 4 / 3 / 1 | 75% / 89% | 35 / 37 |
| race1 | 0.95 | 0.05 | 1.60 | 4 / 3 / 1 | 61% / 86% | 35 / 37.5 |
| race2-noopp | 0.60 | 0.00 | 1.05 | 4 / 3 / 1 | 72% / 81% | 36.5 / 37 |
| race2-land | 0.75 | 0.03 | 1.48 | 4 / 3 / 1 | 67% / 89% | 36 / 37.5 |
| opp | 0.90 | 0.03 | 1.50 | 4 / 3 / 1 | 58% / 83% | 36 / 36 |

The racing ranking does not reach the middle centre sooner (the steps are
v032's: the opening already goes to it as fast as its gold allows), and it
holds it more often at turn 8 (75% for race2 against 64%), but every
variant loses more middle cells at turns 6 to 10 than v032 (0.60 to 0.95
against 0.47 a game), the most those whose simulation lets the opponent
play: they are not stronger in the centre. Pictures of race2 beside v032
against itself, action by action: https://claude.ai/artifact/Esh6hC6nvscvnkxknci3xy.
