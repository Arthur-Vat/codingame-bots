---
id: E051
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: In the first 10 turns of the collected CodinGame games our units stand next to another of ours far more often than the top 10's (73% against 37% at turn 4 as red, 74% against 39% at turn 8 as blue), and by turn 10 a fifth of them have no free cell next to them (the top 10 a tenth); the opening's rules value a free cell by its free neighbours, its nearness to the enemy and our own cells next to it, never by our units already there, so lowering a cell's value for each of our units next to it spreads them, each finds a new cell every turn, and beats v022.
change: The opening's rules (`ice_and_fire_opening::econ`) value a cell 2, 4, 8 or 16 less for each of our units next to it, where this turn's moves and trainings so far put them.
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

A scratch scan of the first 10 turns of the collected CodinGame games
(v020 to v022 against the top 10, and the top 10 in the same games), by
seat ("units next to a unit": our units with another of ours next to
them; "Voronoi": the cells nearer our territory than the enemy's; "mine
built so far": the share of games):

| Seat | Turn | Who | Plies | Cells after (median) | Units | Moves taking a new cell | Units next to a unit | Units with no free neighbour | Gold held | Spent | Mine built so far | Voronoi |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| red | 2 | v020-22 | 323 | 5 | 1.9 | 100% | 97% | 0% | 2 | 0.6 | 0% | 52 |
| red | 2 | top 10 | 76 | 5 | 2.0 | 100% | 79% | 0% | 2 | 0.0 | 0% | 52 |
| red | 4 | v020-22 | 323 | 10 | 2.0 | 90% | 73% | 10% | 10 | 8.9 | 0% | 54 |
| red | 4 | top 10 | 76 | 10 | 2.0 | 100% | 37% | 0% | 10 | 9.6 | 0% | 57 |
| red | 5 | v020-22 | 323 | 13 | 2.9 | 99% | 75% | 1% | 7 | 1.1 | 0% | 52 |
| red | 5 | top 10 | 76 | 13 | 3.0 | 96% | 69% | 2% | 7 | 0.6 | 4% | 54 |
| red | 6 | v020-22 | 323 | 16 | 3.0 | 96% | 40% | 4% | 17 | 5.1 | 0% | 51 |
| red | 6 | top 10 | 76 | 17 | 3.0 | 93% | 34% | 5% | 17 | 11.3 | 30% | 53 |
| red | 7 | v020-22 | 321 | 20 | 3.5 | 92% | 58% | 8% | 20 | 8.0 | 0% | 49 |
| red | 7 | top 10 | 76 | 22 | 3.7 | 95% | 53% | 5% | 20 | 16.4 | 38% | 56 |
| red | 8 | v020-22 | 320 | 25 | 4.3 | 88% | 57% | 11% | 32 | 15.7 | 6% | 50 |
| red | 8 | top 10 | 76 | 26 | 4.9 | 93% | 57% | 4% | 17 | 15.5 | 38% | 56 |
| red | 10 | v020-22 | 320 | 31 | 5.4 | 73% | 45% | 20% | 35 | 20.9 | 24% | 49 |
| red | 10 | top 10 | 76 | 32 | 5.0 | 83% | 44% | 11% | 31 | 21.8 | 54% | 57 |
| blue | 2 | v020-22 | 138 | 6 | 2.0 | 100% | 100% | 0% | 4 | 0.0 | 0% | 55 |
| blue | 2 | top 10 | 266 | 6 | 2.0 | 100% | 81% | 0% | 4 | 0.0 | 0% | 53 |
| blue | 4 | v020-22 | 138 | 11 | 2.0 | 99% | 33% | 1% | 14 | 9.6 | 0% | 55 |
| blue | 4 | top 10 | 266 | 11 | 2.0 | 95% | 27% | 3% | 14 | 9.9 | 0% | 54 |
| blue | 5 | v020-22 | 138 | 15 | 3.0 | 96% | 82% | 4% | 12 | 9.5 | 0% | 57 |
| blue | 5 | top 10 | 266 | 15 | 3.0 | 91% | 68% | 7% | 12 | 10.1 | 0% | 57 |
| blue | 6 | v020-22 | 138 | 19 | 3.9 | 95% | 81% | 5% | 13 | 2.8 | 0% | 54 |
| blue | 6 | top 10 | 266 | 20 | 4.0 | 94% | 62% | 4% | 13 | 9.8 | 0% | 59 |
| blue | 7 | v020-22 | 138 | 24 | 4.2 | 95% | 73% | 3% | 28 | 11.2 | 0% | 52 |
| blue | 7 | top 10 | 264 | 25 | 5.0 | 97% | 54% | 3% | 18 | 11.9 | 21% | 60 |
| blue | 8 | v020-22 | 138 | 29 | 4.9 | 88% | 74% | 11% | 30 | 18.8 | 7% | 55 |
| blue | 8 | top 10 | 263 | 30 | 5.2 | 93% | 39% | 4% | 26 | 17.9 | 30% | 59 |
| blue | 10 | v020-22 | 138 | 33 | 5.4 | 70% | 55% | 21% | 34 | 22.3 | 27% | 54 |
| blue | 10 | top 10 | 263 | 37 | 5.5 | 77% | 41% | 11% | 36 | 24.5 | 54% | 63 |

Through turn 5 both sides take the same land. From turn 6 the top 10 spend
more (a mine in 30% to 38% of their games as red by turn 7, we none before
turn 8), crowd their units less (37% against 73% at turn 4 as red, 39%
against 74% at turn 8 as blue), leave fewer units with no free cell next
to them by turn 10 (11% against 20% to 21%), and hold more of the land
between the HQs (Voronoi 56 to 63 against 49 to 55 by turns 8 to 10).

## The change

`opening/src/econ.rs`: `Params::crowd`, how much less a cell is worth for
each of our units next to it (0 in `Params::V003`, so other users of the
crate are unchanged), with the units tracked as the turn's commands move
and train them; `bots/plan/src/main.rs`: `OPENING_CROWD`, set on every
weight set of the opening (0: v022, which the base commit keeps).

## Screening

Pending.
