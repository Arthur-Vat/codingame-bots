---
id: E006
date: 2026-10-22
pull_request: "#187 (archive of the code, closed)"
parent: ice-and-fire-v004
hypothesis: Games are decided by cuts through necks of a territory that grow under the plan search (research/game-study.md), so a plan search that guards its necks beats v004.
change: None released. Local screening only: v004 whose plan search takes a neck term from its evaluation, or adds bypass actions and a fixed term for the joints of the start of the turn.
release: none
sprt: not run: clearly weaker in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-22 after the game study: "Neck-aware plan
search (Recommended)" (form), then, after the first two variants lost,
"Try neck-safe growth".

A **joint** of a side's territory is an active cell whose capture cuts
other cells off from its HQ (an articulation point, found by Tarjan's
depth-first search from the HQ). Each variant valued what a joint cuts
off as the shared evaluation values cells (7 a cell, 12 more with a mine,
10 more with a tower, and the training cost of a unit), and counted a
joint as threatened when the enemy could take it by a chain of trainings
within its gold plus two turns of income.

## Variants

Against `ice-and-fire-v004` at CodinGame's limits, seed 131, 200 pairs
each, two games at a time:

| Variant | Elo | Faults |
| --- | --- | --- |
| **Neck term, both sides:** half the value cut off by the worst threatened joint, taken from our score; the enemy's added to it | −79.5 [−116.2, −44.6] | 1 timeout |
| **Neck term, our necks only** | −43.7 [−77.3, −10.8] | 1 timeout |
| **Neck-safe growth:** the joints and their values fixed at the start of the turn, so growing beyond a joint costs nothing and covering or widening it gains; plus bypass actions, two level 1 trainings that join the cells beyond one of our two most valuable joints to the HQ's side by a second path | −36.6 [−67.5, −6.3] | none |

## What the games showed

- **Both neck terms made the bot passive.** Crediting the bot with the
  enemy's necks within its own banked gold made it hoard: as red, 44 to 56
  gold in hand against v004's 9 to 20, and fewer cells. Counting only its
  own necks still made growth cost more than it earned (a training beyond
  a threatened joint lost about as much as it gained), so it grew less, and
  its exposure to a cut of 30 or 50 gold did not fall.
- **Neck-safe growth ended the hoarding** (it held less gold than v004 at
  every turn) **but not the losses**: fewer cells by turn 17 (33.6 against
  36.4), a Voronoi share of 48.2 against 53.5, the same exposure to a cut
  of 30 gold (3.3 cells against 3.6), and the first large cut of the game
  (30% of the territory and 6 cells or more) suffered in 171 of 400 games
  against v004's 140. Its turns spent covering joints were turns not spent
  growing, and the cuts came elsewhere.
- Three ways of guarding necks all lost: the plan search, which already
  answers the cuts the enemy can pay now, gains nothing from guarding
  against cuts a turn or two away at the price of growth. This led to
  [E007](E007-reserve.md), which tried keeping gold to answer a cut
  instead of preventing it.

The code (`ice_and_fire_search::eval::Necks`, `actions::bypasses`, and
their tests) is in #187, commits 631528e, 516c800 and fa61089.
