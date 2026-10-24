# Metrics report

1000 games, 1000 decided; 50 metrics a side, followed at the start of each side's own ply, turns 1 to 40. Bots: v004, v005.

| Bot | Games | Wins | Wins as red | Wins as blue |
| --- | ---: | ---: | ---: | ---: |
| v004 | 1000 | 433 | 164 | 269 |
| v005 | 1000 | 567 | 231 | 336 |

Median length: 18 turns.

Games still running at turn: 5: 1000; 8: 999; 10: 999; 12: 991; 15: 797; 20: 436;

## When each metric tells the winner

The share of decided games still running in which the winner's value was higher than the loser's (a tie counting half), at each turn; below 50% a lower value goes with winning. "Tells from" is the first turn from turn 2 with at least 30 games at which the share is at least 70% or at most 30%.

Red and blue are each measured at the start of their own turn, and blue starts with one more cell: in the first turns a metric can tell the winner only because the winner sat in the better seat, above all in a slice where one seat wins most games. Read early turns with the seats in mind.

| Metric | Tells from | Turn 5 | Turn 8 | Turn 10 | Turn 12 | Turn 15 | Turn 20 | Meaning |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| income | 10 (74%) | 60% | 55% | 74% | 83% | 87% | 85% | Gold earned each turn: active cells, plus 4 a mine on an active cell, less upkeep. |
| cells | 10 (71%) | 60% | 62% | 71% | 82% | 87% | 85% | Active cells: owned and joined to the HQ. |
| material | 10 (73%) | 61% | 62% | 73% | 83% | 90% | 90% | Gold + 3 x income + 4 x cells + army + 10 x towers: a side's holdings as the bots' shared evaluation counts them. The lead is its difference. |
| gross_income | 10 (74%) | 60% | 59% | 74% | 83% | 88% | 87% | Income before upkeep: active cells plus 4 a mine on an active cell. |
| cells_change | 10 (71%) | 51% | 58% | 71% | 77% | 81% | 77% | The change of this side's active cells since the start of its previous turn. |
| material_change | 10 (71%) | 59% | 58% | 71% | 79% | 83% | 81% | The change of this side's material since the start of its previous turn. |
| gold | 11 (71%) | 57% | 46% | 63% | 76% | 82% | 81% | Gold in hand at this point. |
| units | 11 (74%) | 52% | 59% | 69% | 77% | 84% | 85% | Units on active cells. |
| army | 11 (73%) | 52% | 59% | 68% | 76% | 83% | 83% | Training cost of those units: 10, 20 or 30 by level. |
| voronoi | 11 (73%) | 45% | 51% | 67% | 77% | 84% | 82% | Non-void cells nearer this side's active cells than the enemy's (a tie counts half, rounded down): the land it is likely to get. |
| hq_gap | 11 (25%) | 39% | 47% | 33% | 21% | 14% | 12% | Gold this side still lacks to pay its cheapest chain of trainings to the enemy HQ: 0 when it can pay now, 999 when no chain reaches it. |
| level1 | 11 (74%) | 52% | 59% | 67% | 77% | 85% | 87% | Level 1 units on active cells. |
| reach | 11 (27%) | 44% | 39% | 33% | 24% | 17% | 17% | Steps from the nearest active cell to the enemy HQ. |
| far_cells | 11 (70%) | 50% | 53% | 65% | 75% | 82% | 85% | Active cells nearer the enemy HQ than this side's own (Manhattan distance). |
| spread | 11 (72%) | 45% | 61% | 66% | 74% | 81% | 84% | The mean Manhattan distance of the active cells from this side's HQ, times 10. |
| moves | 11 (73%) | 50% | 59% | 63% | 76% | 79% | 75% | Moves this side made in its last turn. |
| captured | 11 (71%) | 50% | 51% | 64% | 73% | 75% | 66% | Enemy cells this side took in its last turn. |
| upkeep | 12 (74%) | 52% | 59% | 65% | 74% | 80% | 78% | Upkeep of the units on active cells: 1, 4 and 20 for levels 1, 2 and 3. |
| cut_off | 12 (29%) | 50% | 51% | 35% | 29% | 25% | 33% | Owned cells cut off from the HQ: they earn nothing, and units on them die. |
| can_cut | 12 (72%) | 50% | 53% | 62% | 72% | 76% | 74% | Enemy active cells this side could take or cut off with the gold it has, in the same way. |
| free_spots | 12 (70%) | 48% | 55% | 61% | 70% | 79% | 81% | Mine spots on active cells without a mine: mines this side could still build at home. |
| frontier | 12 (72%) | 50% | 54% | 60% | 72% | 78% | 73% | Active cells next to an enemy cell. |
| frontier_units | 12 (74%) | 50% | 54% | 64% | 74% | 79% | 73% | Units on frontier cells. |
| inner_units | 12 (70%) | 52% | 53% | 59% | 70% | 82% | 85% | Units with no enemy cell within 2 steps: kept away from any fight. |
| lost | 12 (25%) | 50% | 52% | 37% | 25% | 22% | 34% | Cells of this side the enemy took in its last turn. |
| border | 13 (74%) | 47% | 46% | 58% | 69% | 77% | 77% | Active cells next to a cell this side does not own (void aside): the territory's perimeter. |
| room | 13 (72%) | 47% | 45% | 56% | 67% | 73% | 73% | Neutral cells next to this side's active cells: where it can still grow without a fight. |
| spent_units | 13 (70%) | 52% | 52% | 53% | 66% | 73% | 72% | Gold this side spent training units in its last turn. |
| trains | 13 (71%) | 52% | 51% | 58% | 66% | 74% | 71% | Units this side trained in its last turn. |
| level2 | 15 (70%) | 50% | 51% | 53% | 58% | 70% | 72% | Level 2 units on active cells. |
| guarded_joints | 15 (70%) | 52% | 49% | 58% | 62% | 70% | 66% | Joints that hold a unit of this side. |
| contact | 15 (28%) | 50% | 53% | 45% | 36% | 28% | 32% | Enemy units next to this side's active cells. |
| max_cut | 38 (70%) | 44% | 45% | 47% | 50% | 59% | 60% | The most active cells that the loss of a single cell would cut off. |
| mines | – | 50% | 46% | 46% | 47% | 50% | 54% | Mines on active cells. |
| towers | – | 50% | 49% | 51% | 49% | 50% | 62% | Towers on active cells. |
| exposed30 | – | 50% | 54% | 42% | 46% | 53% | 60% | Active cells the enemy could take or cut off with 30 gold of trainings: the most among its 8 best cuts of up to 5 trainings. |
| exposed50 | – | 50% | 56% | 50% | 54% | 55% | 63% | The same with 50 gold. |
| level3 | – | 50% | 50% | 50% | 50% | 52% | 54% | Level 3 units on active cells. |
| joints | – | 46% | 42% | 49% | 55% | 64% | 62% | Active cells whose loss would cut other active cells off from the HQ. |
| total_cut | – | 46% | 43% | 47% | 52% | 61% | 62% | The sum over the joints of the cells each would cut off: how much of the territory hangs on single cells. |
| line_ends | – | 46% | 44% | 54% | 59% | 65% | 61% | Active cells other than the HQ with at most one own active neighbour: the tips of thin lines. |
| frontier_joints | – | 50% | 50% | 56% | 60% | 62% | 56% | Joints next to an enemy cell: the links the enemy can reach first. |
| empty_joints | – | 45% | 43% | 49% | 54% | 62% | 60% | Joints that hold no unit, building or HQ of this side. |
| units_lost | – | 50% | 52% | 43% | 41% | 40% | 45% | Units of this side lost since the end of its last turn: taken by the enemy, cut off, or lost to debt. |
| hq_guard | – | 49% | 49% | 49% | 48% | 49% | 51% | Units of this side within 2 steps of its HQ. |
| frontier_towers | – | 50% | 50% | 52% | 48% | 46% | 53% | Towers on frontier cells. |
| tower_cover | – | 50% | 49% | 51% | 49% | 51% | 64% | Active cells a tower of this side protects (the tower's cell and its neighbours). |
| spent_buildings | – | 50% | 54% | 50% | 47% | 48% | 53% | Gold this side spent building mines and towers in its last turn. |
| kept_share | – | 57% | 46% | 45% | 45% | 42% | 41% | The share of its gold this side kept unspent in its last turn, in percent. |
| builds | – | 50% | 54% | 50% | 47% | 48% | 53% | Mines and towers this side built in its last turn. |

## The metrics that tell the winner best

At each turn, the ten metrics whose share is farthest from 50%, with the winner's and the loser's means.

### Turn 8 (999 games)

| Metric | Winner higher | Winner's mean | Loser's mean |
| --- | ---: | ---: | ---: |
| cells | 62% | 21.6 | 20.3 |
| material | 62% | 221 | 212 |
| reach | 39% | 11.8 | 12.3 |
| spread | 61% | 53.2 | 51.0 |
| upkeep | 59% | 4.1 | 3.8 |
| units | 59% | 4.1 | 3.7 |
| army | 59% | 40.7 | 37.4 |
| gross_income | 59% | 25.1 | 24.2 |
| level1 | 59% | 4.0 | 3.7 |
| moves | 59% | 3.9 | 3.6 |

### Turn 12 (989 games)

| Metric | Winner higher | Winner's mean | Loser's mean |
| --- | ---: | ---: | ---: |
| income | 83% | 30.2 | 20.7 |
| material | 83% | 352 | 224 |
| gross_income | 83% | 39.1 | 25.6 |
| cells | 82% | 35.2 | 21.6 |
| hq_gap | 21% | 61.3 | 102 |
| material_change | 79% | 37.0 | -11.8 |
| units | 77% | 6.5 | 3.3 |
| voronoi | 77% | 62.2 | 35.2 |
| level1 | 77% | 5.7 | 2.7 |
| cells_change | 77% | 3.6 | -1.5 |

### Turn 16 (681 games)

| Metric | Winner higher | Winner's mean | Loser's mean |
| --- | ---: | ---: | ---: |
| material | 90% | 480 | 218 |
| hq_gap | 12% | 45.1 | 129 |
| gross_income | 88% | 50.7 | 23.3 |
| income | 86% | 34.0 | 16.7 |
| cells | 86% | 46.7 | 19.4 |
| level1 | 85% | 8.4 | 2.4 |
| gold | 83% | 60.3 | 37.2 |
| units | 83% | 10.0 | 3.2 |
| voronoi | 83% | 69.8 | 26.3 |
| reach | 17% | 7.5 | 14.0 |

### Turn 20 (405 games)

| Metric | Winner higher | Winner's mean | Loser's mean |
| --- | ---: | ---: | ---: |
| material | 90% | 575 | 242 |
| hq_gap | 12% | 43.7 | 145 |
| gross_income | 87% | 58.9 | 24.7 |
| level1 | 87% | 10.9 | 2.8 |
| income | 85% | 35.0 | 15.4 |
| cells | 85% | 53.8 | 20.7 |
| units | 85% | 13.2 | 3.9 |
| far_cells | 85% | 14.6 | 1.0 |
| inner_units | 85% | 7.7 | 1.6 |
| spread | 84% | 81.0 | 42.7 |

## v004 against v005

Each bot's mean over its games (both seats) at each turn, as `v004` / `v005`, and the metrics on which they differ most at turn 10 (by the difference over the larger mean).

| Metric | Turn 5 | Turn 8 | Turn 10 | Turn 12 | Turn 15 | Turn 20 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| total_cut | 19.3 / 6.5 | 55.3 / 11.2 | 60.5 / 7.7 | 64.1 / 9.7 | 61.7 / 13.1 | 51.1 / 15.1 |
| max_cut | 8.1 / 4.6 | 17.7 / 7.9 | 23.0 / 3.3 | 24.2 / 3.7 | 24.3 / 4.7 | 21.4 / 5.7 |
| hq_guard | 0.1 / 0.1 | 0.0 / 0.1 | 0.1 / 0.4 | 0.1 / 0.2 | 0.3 / 0.2 | 0.7 / 0.5 |
| empty_joints | 4.4 / 1.3 | 7.9 / 1.1 | 5.4 / 1.6 | 5.2 / 1.8 | 4.1 / 1.7 | 2.9 / 1.5 |
| frontier_towers | 0.0 / 0.0 | 0.0 / 0.0 | 0.1 / 0.2 | 0.1 / 0.3 | 0.2 / 0.4 | 0.3 / 0.5 |
| joints | 5.4 / 2.2 | 8.7 / 1.7 | 7.0 / 2.4 | 6.7 / 2.8 | 6.1 / 3.0 | 4.9 / 2.8 |
| far_cells | 0.0 / 0.0 | 0.1 / 0.1 | 1.7 / 0.8 | 3.7 / 2.7 | 7.0 / 5.5 | 8.9 / 7.8 |
| spent_buildings | 0.0 / 0.0 | 4.8 / 9.5 | 1.7 / 3.3 | 1.9 / 2.0 | 3.1 / 2.7 | 3.1 / 3.4 |
| builds | 0.0 / 0.0 | 0.3 / 0.6 | 0.1 / 0.2 | 0.1 / 0.1 | 0.2 / 0.2 | 0.2 / 0.2 |
| tower_cover | 0.0 / 0.0 | 0.1 / 0.0 | 0.8 / 1.4 | 1.7 / 2.5 | 3.7 / 4.5 | 7.4 / 7.5 |
| towers | 0.0 / 0.0 | 0.0 / 0.0 | 0.2 / 0.4 | 0.4 / 0.7 | 1.0 / 1.2 | 1.9 / 1.9 |
| room | 9.8 / 7.5 | 16.6 / 11.4 | 14.9 / 8.9 | 12.6 / 7.7 | 10.2 / 6.0 | 6.1 / 3.7 |
| level2 | 0.0 / 0.0 | 0.0 / 0.0 | 0.5 / 0.3 | 0.7 / 0.6 | 1.0 / 0.9 | 1.6 / 1.5 |
| border | 8.5 / 6.6 | 15.8 / 11.5 | 18.7 / 11.9 | 18.1 / 12.5 | 17.2 / 12.2 | 13.2 / 9.7 |
| guarded_joints | 1.0 / 0.9 | 0.2 / 0.3 | 0.9 / 0.6 | 0.8 / 0.8 | 1.1 / 0.9 | 1.2 / 0.9 |
| line_ends | 1.6 / 1.0 | 2.1 / 0.7 | 1.8 / 1.3 | 1.9 / 1.5 | 1.9 / 1.6 | 1.8 / 1.5 |
| captured | 0.0 / 0.0 | 0.0 / 0.0 | 1.5 / 1.2 | 2.1 / 2.4 | 2.2 / 2.7 | 1.7 / 2.3 |
| cells_change | 2.9 / 2.8 | 3.4 / 3.8 | 2.2 / 1.8 | 0.4 / 1.7 | 0.8 / 1.8 | 1.3 / 1.5 |
| frontier_units | 0.0 / 0.0 | 0.4 / 0.5 | 2.0 / 1.6 | 2.3 / 2.3 | 2.6 / 2.8 | 2.6 / 2.7 |
| can_cut | 0.0 / 0.0 | 1.1 / 0.5 | 2.7 / 3.3 | 3.6 / 4.5 | 3.7 / 4.7 | 4.3 / 5.4 |
| mines | 0.0 / 0.0 | 0.9 / 1.0 | 0.9 / 1.1 | 0.9 / 1.1 | 0.9 / 1.1 | 1.0 / 1.4 |
| contact | 0.0 / 0.0 | 0.4 / 0.7 | 1.9 / 2.2 | 2.2 / 2.3 | 2.1 / 2.4 | 2.2 / 2.3 |
| upkeep | 3.0 / 2.9 | 3.7 / 4.2 | 6.0 / 5.0 | 6.9 / 6.9 | 10.4 / 10.4 | 17.4 / 16.8 |
| voronoi | 53.3 / 49.2 | 53.0 / 48.0 | 52.8 / 44.6 | 50.6 / 46.8 | 50.5 / 48.2 | 48.5 / 47.2 |
| spread | 32.3 / 28.5 | 53.0 / 51.2 | 62.4 / 54.2 | 63.2 / 56.8 | 64.0 / 58.5 | 64.6 / 61.1 |
| army | 29.9 / 29.2 | 36.1 / 42.0 | 51.1 / 44.8 | 54.0 / 56.0 | 72.1 / 75.1 | 109 / 108 |
| lost | 0.0 / 0.0 | 0.1 / 0.3 | 1.6 / 1.8 | 2.5 / 2.2 | 2.6 / 2.1 | 2.0 / 1.7 |
| inner_units | 3.0 / 2.9 | 2.5 / 3.0 | 1.6 / 1.8 | 1.6 / 1.8 | 2.3 / 2.4 | 4.9 / 4.8 |
| material_change | 24.9 / 24.4 | 41.9 / 43.6 | 17.9 / 16.0 | 5.7 / 19.4 | 9.3 / 20.3 | 13.7 / 18.5 |
| units | 3.0 / 2.9 | 3.6 / 4.2 | 4.7 / 4.2 | 4.7 / 5.0 | 6.0 / 6.4 | 8.9 / 9.0 |
| reach | 15.6 / 16.0 | 12.1 / 11.9 | 10.4 / 11.6 | 10.5 / 11.1 | 10.4 / 10.8 | 10.5 / 10.7 |
| hq_gap | 148 / 151 | 90.0 / 98.2 | 82.4 / 91.0 | 79.7 / 83.5 | 82.5 / 84.3 | 93.3 / 89.0 |
| frontier_joints | 0.0 / 0.0 | 0.1 / 0.1 | 0.7 / 0.6 | 1.0 / 1.0 | 1.3 / 1.1 | 1.0 / 0.8 |
| cut_off | 0.0 / 0.0 | 0.1 / 0.1 | 2.1 / 1.9 | 3.9 / 2.7 | 3.9 / 2.7 | 2.8 / 2.1 |
| units_lost | 0.0 / 0.0 | 0.1 / 0.2 | 1.6 / 1.5 | 1.7 / 1.4 | 1.3 / 1.1 | 1.0 / 1.2 |
| trains | 1.0 / 0.9 | 0.2 / 0.3 | 1.9 / 1.7 | 1.6 / 1.8 | 1.7 / 1.8 | 1.7 / 1.8 |
| exposed30 | 0.0 / 0.0 | 1.7 / 2.0 | 5.0 / 5.4 | 5.6 / 5.4 | 4.4 / 3.9 | 3.6 / 3.1 |
| exposed50 | 0.0 / 0.0 | 4.6 / 5.2 | 10.5 / 11.3 | 11.1 / 10.9 | 8.7 / 8.2 | 7.1 / 6.4 |
| frontier | 0.0 / 0.0 | 0.5 / 0.7 | 3.5 / 3.8 | 5.2 / 5.4 | 6.1 / 6.5 | 5.8 / 6.0 |
| gold | 9.5 / 10.1 | 35.6 / 26.8 | 31.6 / 33.8 | 36.0 / 38.5 | 42.2 / 48.5 | 55.4 / 67.1 |
| level1 | 3.0 / 2.9 | 3.6 / 4.2 | 4.2 / 3.9 | 4.0 / 4.4 | 4.9 / 5.4 | 7.0 / 7.3 |
| moves | 2.0 / 2.0 | 3.4 / 4.1 | 3.2 / 3.0 | 2.6 / 2.7 | 2.2 / 2.5 | 1.8 / 2.1 |
| kept_share | 14.7 / 21.8 | 67.5 / 30.2 | 25.4 / 26.8 | 31.2 / 32.2 | 35.9 / 38.4 | 46.9 / 51.5 |
| spent_units | 9.9 / 9.2 | 2.4 / 3.3 | 22.8 / 21.7 | 20.2 / 21.1 | 20.8 / 21.7 | 21.1 / 21.8 |
| income | 7.4 / 7.4 | 20.2 / 21.1 | 23.6 / 24.7 | 24.5 / 26.5 | 24.6 / 26.9 | 24.5 / 27.5 |
| free_spots | 1.2 / 1.6 | 1.2 / 1.6 | 1.7 / 1.7 | 1.9 / 2.0 | 2.2 / 2.4 | 2.7 / 2.5 |
| cells | 10.4 / 10.3 | 20.4 / 21.5 | 26.1 / 25.4 | 27.9 / 29.0 | 31.6 / 32.9 | 37.9 / 38.8 |
| material | 103 / 102 | 214 / 218 | 260 / 258 | 279 / 296 | 324 / 348 | 408 / 432 |
| gross_income | 10.4 / 10.3 | 23.9 / 25.3 | 29.6 / 29.8 | 31.4 / 33.3 | 35.0 / 37.3 | 41.9 / 44.2 |
| level3 | 0.0 / 0.0 | 0.0 / 0.0 | 0.0 / 0.0 | 0.0 / 0.0 | 0.1 / 0.1 | 0.2 / 0.2 |

## How the batch splits

The same games cut along four dimensions: length (the shortest, regular and longest thirds), seats, which bot won, and whether the side ahead in the lead metric at turn 10 went on to win. Each slice is analysed as the whole batch is above.

| Slice | Games | v004 won | v005 won | Median length |
| --- | ---: | ---: | ---: | ---: |
| All games | 1000 | 433 | 567 | 18 |
| Short (7 to 16 turns) | 333 | 131 | 202 | 14 |
| Regular (16 to 21 turns) | 333 | 135 | 198 | 18 |
| Long (21 to 71 turns) | 334 | 167 | 167 | 26 |
| v004 red, v005 blue | 500 | 164 | 336 | 18 |
| v005 red, v004 blue | 500 | 269 | 231 | 19 |
| v004 won | 433 | 433 | 0 | 19 |
| v005 won | 567 | 0 | 567 | 18 |
| Leader at turn 10 won | 731 | 340 | 391 | 17 |
| Leader at turn 10 lost | 263 | 91 | 172 | 22 |

## When each metric tells the winner, by slice

From the start: the first turn at which the metric tells the winner in that slice (at least 30 games, a share of at least 70% or at most 30%), with the share then; – when it never does.

| Metric | All games | Short (7 to 16 turns) | Regular (16 to 21 turns) | Long (21 to 71 turns) | v004 red, v005 blue | v005 red, v004 blue | v004 won | v005 won | Leader at turn 10 won | Leader at turn 10 lost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| gold | 11 (71%) | 2 (72%) | 11 (71%) | 17 (72%) | 11 (72%) | 11 (71%) | 7 (90%) | 7 (12%) | 2 (71%) | 2 (26%) |
| income | 10 (74%) | 2 (75%) | 10 (77%) | 13 (74%) | 10 (71%) | 10 (76%) | 11 (78%) | 7 (25%) | 2 (74%) | 2 (23%) |
| upkeep | 12 (74%) | 6 (74%) | 12 (72%) | 17 (70%) | 12 (73%) | 12 (75%) | 9 (70%) | 8 (71%) | 6 (73%) | 6 (26%) |
| cells | 10 (71%) | 2 (75%) | 11 (72%) | 14 (73%) | 9 (71%) | 11 (73%) | 10 (78%) | 11 (72%) | 2 (74%) | 2 (23%) |
| cut_off | 12 (29%) | 10 (22%) | 13 (24%) | 28 (30%) | 12 (28%) | 12 (30%) | 13 (28%) | 12 (27%) | 10 (26%) | 14 (29%) |
| units | 11 (74%) | 6 (74%) | 11 (70%) | 14 (70%) | 11 (74%) | 11 (74%) | 10 (80%) | 8 (71%) | 6 (73%) | 6 (26%) |
| army | 11 (73%) | 6 (74%) | 12 (74%) | 15 (70%) | 11 (72%) | 11 (73%) | 9 (70%) | 8 (71%) | 6 (73%) | 6 (26%) |
| mines | – | 7 (29%) | – | – | 34 (71%) | – | – | 24 (70%) | – | 8 (73%) |
| towers | – | – | 20 (77%) | – | – | 25 (70%) | – | – | – | – |
| voronoi | 11 (73%) | 10 (83%) | 11 (71%) | 15 (73%) | 10 (71%) | 12 (73%) | 2 (84%) | 2 (15%) | 10 (83%) | 6 (28%) |
| exposed30 | – | – | – | – | – | – | – | – | – | – |
| exposed50 | – | 15 (70%) | 19 (72%) | – | – | – | – | – | 22 (70%) | – |
| can_cut | 12 (72%) | 10 (70%) | 12 (73%) | 20 (72%) | 12 (74%) | 12 (71%) | 14 (73%) | 12 (76%) | 11 (70%) | 15 (77%) |
| hq_gap | 11 (25%) | 2 (25%) | 11 (27%) | 14 (29%) | 10 (30%) | 11 (29%) | 8 (28%) | 12 (26%) | 2 (26%) | 2 (77%) |
| material | 10 (73%) | 2 (75%) | 10 (70%) | 13 (71%) | 9 (71%) | 10 (71%) | 10 (79%) | 11 (75%) | 2 (74%) | 2 (23%) |
| gross_income | 10 (74%) | 2 (75%) | 10 (72%) | 13 (73%) | 9 (70%) | 10 (74%) | 10 (76%) | 8 (73%) | 2 (74%) | 2 (23%) |
| free_spots | 12 (70%) | 10 (70%) | 13 (72%) | 16 (71%) | 12 (74%) | 13 (72%) | 2 (29%) | 7 (76%) | 10 (70%) | 2 (71%) |
| level1 | 11 (74%) | 6 (74%) | 11 (70%) | 13 (70%) | 11 (73%) | 11 (75%) | 10 (77%) | 8 (71%) | 6 (73%) | 6 (26%) |
| level2 | 15 (70%) | 13 (72%) | 14 (71%) | 21 (70%) | 18 (71%) | 14 (70%) | 14 (73%) | 18 (70%) | 14 (72%) | 31 (74%) |
| level3 | – | – | – | – | – | – | – | – | – | – |
| joints | – | 14 (77%) | 20 (71%) | – | 28 (72%) | – | 2 (84%) | 2 (14%) | 15 (70%) | 3 (30%) |
| max_cut | 38 (70%) | 15 (73%) | – | 38 (70%) | 29 (70%) | – | 2 (84%) | 2 (14%) | 21 (70%) | 34 (71%) |
| total_cut | – | 14 (72%) | 20 (70%) | – | 28 (70%) | – | 2 (84%) | 2 (14%) | 18 (72%) | 32 (70%) |
| line_ends | – | 13 (72%) | – | – | 28 (72%) | – | 5 (76%) | 2 (27%) | 28 (71%) | 31 (71%) |
| reach | 11 (27%) | 6 (25%) | 12 (26%) | 15 (29%) | 10 (29%) | 12 (27%) | 2 (27%) | 13 (26%) | 6 (26%) | 2 (73%) |
| frontier | 12 (72%) | 11 (76%) | 13 (75%) | 19 (72%) | 12 (72%) | 12 (72%) | 12 (72%) | 12 (72%) | 11 (70%) | 18 (72%) |
| border | 13 (74%) | 11 (80%) | 14 (76%) | 18 (70%) | 12 (74%) | 14 (70%) | 2 (73%) | 4 (17%) | 11 (76%) | 2 (27%) |
| room | 13 (72%) | 11 (76%) | 14 (74%) | 20 (70%) | 12 (72%) | 17 (71%) | 3 (80%) | 3 (19%) | 11 (73%) | 2 (28%) |
| far_cells | 11 (70%) | 10 (79%) | 13 (76%) | 15 (70%) | 11 (75%) | 12 (70%) | 9 (72%) | 13 (73%) | 10 (80%) | 9 (28%) |
| spread | 11 (72%) | 6 (75%) | 13 (75%) | 16 (70%) | 10 (72%) | 13 (72%) | 2 (73%) | 3 (13%) | 6 (74%) | 2 (27%) |
| frontier_joints | – | 12 (70%) | – | – | – | – | – | – | – | – |
| empty_joints | – | 14 (73%) | 20 (70%) | – | – | – | 2 (73%) | 4 (20%) | 15 (70%) | 2 (27%) |
| guarded_joints | 15 (70%) | 6 (72%) | 14 (70%) | – | 15 (71%) | 15 (70%) | 7 (27%) | 2 (26%) | 6 (70%) | 6 (29%) |
| frontier_units | 12 (74%) | 10 (78%) | 13 (77%) | 18 (71%) | 12 (74%) | 12 (74%) | 10 (76%) | 12 (70%) | 10 (76%) | 18 (73%) |
| inner_units | 12 (70%) | 6 (74%) | 13 (76%) | 14 (71%) | 12 (70%) | 12 (71%) | 13 (73%) | 7 (73%) | 6 (73%) | 6 (26%) |
| units_lost | – | 14 (27%) | – | – | – | – | – | – | – | – |
| contact | 15 (28%) | 12 (22%) | 15 (27%) | 28 (30%) | 13 (30%) | 15 (27%) | 13 (30%) | 32 (29%) | 13 (25%) | 32 (29%) |
| hq_guard | – | – | – | – | – | – | 2 (30%) | 28 (28%) | – | 2 (71%) |
| frontier_towers | – | – | – | – | – | – | – | – | – | – |
| tower_cover | – | – | 20 (77%) | – | – | 24 (72%) | – | 28 (70%) | 24 (70%) | – |
| spent_units | 13 (70%) | 6 (72%) | 14 (74%) | 20 (71%) | 13 (72%) | 14 (71%) | 7 (23%) | 7 (82%) | 6 (71%) | 6 (28%) |
| spent_buildings | – | 7 (30%) | – | – | – | – | – | – | – | – |
| kept_share | – | 2 (72%) | – | – | – | – | 8 (78%) | 8 (21%) | 2 (71%) | 2 (26%) |
| moves | 11 (73%) | 7 (75%) | 11 (70%) | 16 (71%) | 11 (71%) | 11 (75%) | 11 (78%) | 8 (70%) | 7 (73%) | 7 (26%) |
| trains | 13 (71%) | 6 (72%) | 14 (73%) | 20 (70%) | 13 (73%) | 13 (70%) | 7 (23%) | 7 (82%) | 6 (71%) | 6 (28%) |
| builds | – | 7 (30%) | – | – | – | – | – | – | – | – |
| captured | 11 (71%) | 10 (73%) | 11 (70%) | – | 11 (70%) | 11 (71%) | 9 (79%) | 12 (73%) | 10 (74%) | 13 (73%) |
| lost | 12 (25%) | 10 (28%) | 11 (30%) | – | 12 (25%) | 11 (30%) | 9 (25%) | 12 (25%) | 10 (28%) | 13 (28%) |
| cells_change | 10 (71%) | 6 (75%) | 11 (71%) | 18 (73%) | 11 (72%) | 10 (72%) | 9 (79%) | 7 (71%) | 6 (74%) | 6 (28%) |
| material_change | 10 (71%) | 2 (75%) | 11 (74%) | 13 (71%) | 10 (70%) | 10 (72%) | 9 (71%) | 11 (75%) | 2 (74%) | 2 (23%) |

## 5 turns before the end, by slice

Each game lined up on its last turn: the share of games in which the winner's value was higher, 5 turns before it (a tie counting half); – with fewer than 30 games.

| Metric | All games | Short (7 to 16 turns) | Regular (16 to 21 turns) | Long (21 to 71 turns) | v004 red, v005 blue | v005 red, v004 blue | v004 won | v005 won | Leader at turn 10 won | Leader at turn 10 lost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| gold | 75% | 53% | 83% | 89% | 70% | 80% | 78% | 72% | 72% | 83% |
| income | 83% | 68% | 90% | 92% | 82% | 84% | 80% | 85% | 82% | 86% |
| upkeep | 79% | 78% | 81% | 78% | 78% | 79% | 81% | 77% | 83% | 68% |
| cells | 87% | 84% | 89% | 88% | 89% | 86% | 88% | 86% | 92% | 74% |
| cut_off | 31% | 38% | 23% | 33% | 33% | 30% | 35% | 29% | 34% | 26% |
| units | 83% | 79% | 85% | 84% | 82% | 83% | 86% | 80% | 87% | 72% |
| army | 81% | 79% | 83% | 81% | 80% | 81% | 84% | 79% | 85% | 70% |
| mines | 48% | 38% | 52% | 53% | 45% | 51% | 39% | 54% | 38% | 74% |
| towers | 55% | 52% | 54% | 60% | 56% | 54% | 51% | 58% | 53% | 60% |
| voronoi | 78% | 71% | 81% | 81% | 80% | 76% | 90% | 68% | 83% | 64% |
| exposed30 | 54% | 44% | 52% | 66% | 53% | 55% | 56% | 52% | 54% | 53% |
| exposed50 | 57% | 49% | 58% | 64% | 58% | 57% | 59% | 56% | 59% | 52% |
| can_cut | 74% | 62% | 79% | 81% | 74% | 74% | 71% | 76% | 73% | 76% |
| hq_gap | 13% | 21% | 13% | 4% | 10% | 16% | 7% | 17% | 10% | 21% |
| material | 93% | 88% | 93% | 97% | 92% | 93% | 92% | 93% | 95% | 85% |
| gross_income | 86% | 77% | 92% | 88% | 85% | 87% | 83% | 88% | 88% | 80% |
| free_spots | 73% | 65% | 73% | 81% | 73% | 73% | 74% | 72% | 75% | 66% |
| level1 | 83% | 78% | 84% | 87% | 82% | 84% | 85% | 81% | 87% | 73% |
| level2 | 63% | 53% | 65% | 69% | 62% | 64% | 66% | 60% | 63% | 60% |
| level3 | 51% | 50% | 50% | 53% | 51% | 51% | 51% | 51% | 51% | 51% |
| joints | 54% | 42% | 57% | 62% | 45% | 62% | 87% | 28% | 56% | 46% |
| max_cut | 50% | 42% | 49% | 60% | 40% | 61% | 93% | 18% | 53% | 42% |
| total_cut | 52% | 41% | 53% | 62% | 42% | 62% | 93% | 20% | 55% | 44% |
| line_ends | 54% | 44% | 58% | 60% | 48% | 60% | 70% | 42% | 54% | 54% |
| reach | 20% | 21% | 21% | 18% | 16% | 24% | 11% | 27% | 14% | 37% |
| frontier | 71% | 61% | 77% | 74% | 73% | 69% | 70% | 71% | 71% | 71% |
| border | 65% | 52% | 70% | 75% | 63% | 67% | 92% | 45% | 69% | 54% |
| room | 64% | 51% | 69% | 71% | 63% | 65% | 91% | 43% | 68% | 53% |
| far_cells | 75% | 66% | 76% | 82% | 77% | 73% | 85% | 67% | 80% | 62% |
| spread | 79% | 80% | 76% | 81% | 86% | 72% | 90% | 71% | 87% | 59% |
| frontier_joints | 58% | 55% | 60% | 59% | 58% | 58% | 59% | 57% | 57% | 60% |
| empty_joints | 53% | 42% | 56% | 62% | 45% | 62% | 84% | 30% | 56% | 47% |
| guarded_joints | 62% | 58% | 64% | 64% | 64% | 60% | 65% | 60% | 64% | 57% |
| frontier_units | 73% | 67% | 79% | 73% | 73% | 74% | 77% | 70% | 75% | 68% |
| inner_units | 74% | 65% | 75% | 83% | 74% | 75% | 72% | 76% | 76% | 69% |
| units_lost | 46% | 45% | 44% | 49% | 45% | 47% | 48% | 44% | 46% | 45% |
| contact | 38% | 50% | 32% | 32% | 41% | 35% | 32% | 42% | 37% | 39% |
| hq_guard | 49% | 48% | 49% | 50% | 51% | 47% | 48% | 50% | 48% | 51% |
| frontier_towers | 51% | 51% | 49% | 53% | 51% | 51% | 46% | 55% | 49% | 56% |
| tower_cover | 55% | 52% | 53% | 61% | 56% | 55% | 50% | 59% | 53% | 62% |
| spent_units | 65% | 54% | 71% | 69% | 63% | 67% | 68% | 62% | 64% | 67% |
| spent_buildings | 51% | 50% | 50% | 52% | 52% | 49% | 48% | 52% | 51% | 49% |
| kept_share | 46% | 47% | 42% | 49% | 47% | 45% | 46% | 46% | 47% | 44% |
| moves | 75% | 75% | 76% | 75% | 76% | 75% | 75% | 76% | 81% | 61% |
| trains | 66% | 56% | 73% | 70% | 64% | 68% | 69% | 63% | 65% | 68% |
| builds | 51% | 50% | 50% | 52% | 52% | 49% | 48% | 52% | 51% | 49% |
| captured | 69% | 62% | 77% | 67% | 68% | 70% | 70% | 68% | 69% | 69% |
| lost | 31% | 39% | 22% | 33% | 33% | 30% | 32% | 31% | 33% | 28% |
| cells_change | 78% | 78% | 82% | 74% | 76% | 80% | 79% | 78% | 80% | 75% |
| material_change | 83% | 80% | 83% | 85% | 82% | 83% | 83% | 82% | 84% | 79% |

## 1 turn before the end, by slice

Each game lined up on its last turn: the share of games in which the winner's value was higher, 1 turn before it (a tie counting half); – with fewer than 30 games.

| Metric | All games | Short (7 to 16 turns) | Regular (16 to 21 turns) | Long (21 to 71 turns) | v004 red, v005 blue | v005 red, v004 blue | v004 won | v005 won | Leader at turn 10 won | Leader at turn 10 lost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| gold | 97% | 99% | 98% | 93% | 97% | 96% | 95% | 98% | 96% | 97% |
| income | 96% | 98% | 98% | 93% | 96% | 97% | 94% | 98% | 98% | 94% |
| upkeep | 91% | 96% | 95% | 83% | 90% | 93% | 91% | 92% | 93% | 87% |
| cells | 95% | 99% | 97% | 89% | 94% | 96% | 94% | 96% | 97% | 89% |
| cut_off | 20% | 7% | 18% | 35% | 19% | 21% | 23% | 18% | 20% | 19% |
| units | 95% | 98% | 96% | 91% | 94% | 96% | 95% | 95% | 97% | 90% |
| army | 94% | 97% | 96% | 88% | 92% | 95% | 94% | 94% | 96% | 89% |
| mines | 59% | 42% | 61% | 74% | 58% | 61% | 53% | 64% | 51% | 81% |
| towers | 58% | 49% | 59% | 65% | 59% | 57% | 55% | 60% | 57% | 59% |
| voronoi | 93% | 99% | 96% | 85% | 93% | 94% | 93% | 93% | 95% | 88% |
| exposed30 | 53% | 40% | 59% | 59% | 46% | 60% | 57% | 50% | 50% | 60% |
| exposed50 | 61% | 50% | 65% | 68% | 58% | 63% | 62% | 59% | 58% | 67% |
| can_cut | 90% | 92% | 90% | 88% | 90% | 90% | 87% | 92% | 89% | 93% |
| hq_gap | 1% | 1% | 1% | 0% | 1% | 0% | 1% | 0% | 0% | 2% |
| material | 99% | 99% | 98% | 99% | 98% | 100% | 99% | 99% | 100% | 97% |
| gross_income | 95% | 99% | 97% | 90% | 94% | 97% | 94% | 97% | 97% | 91% |
| free_spots | 87% | 93% | 92% | 77% | 87% | 88% | 87% | 87% | 89% | 83% |
| level1 | 96% | 98% | 96% | 94% | 96% | 97% | 97% | 96% | 98% | 92% |
| level2 | 78% | 73% | 85% | 78% | 76% | 81% | 78% | 78% | 78% | 80% |
| level3 | 54% | 52% | 55% | 56% | 54% | 54% | 56% | 53% | 54% | 56% |
| joints | 66% | 65% | 74% | 58% | 55% | 77% | 83% | 52% | 65% | 68% |
| max_cut | 60% | 56% | 67% | 58% | 45% | 75% | 84% | 42% | 59% | 64% |
| total_cut | 63% | 60% | 70% | 61% | 49% | 78% | 85% | 47% | 62% | 67% |
| line_ends | 67% | 71% | 72% | 56% | 61% | 72% | 72% | 62% | 65% | 70% |
| reach | 7% | 2% | 4% | 16% | 8% | 6% | 7% | 7% | 5% | 12% |
| frontier | 89% | 95% | 93% | 79% | 88% | 90% | 89% | 89% | 90% | 87% |
| border | 88% | 97% | 93% | 74% | 88% | 88% | 89% | 87% | 89% | 85% |
| room | 81% | 96% | 88% | 60% | 82% | 81% | 81% | 81% | 83% | 76% |
| far_cells | 94% | 98% | 97% | 87% | 94% | 94% | 94% | 94% | 96% | 88% |
| spread | 94% | 99% | 97% | 87% | 94% | 94% | 94% | 94% | 96% | 88% |
| frontier_joints | 64% | 72% | 67% | 53% | 59% | 69% | 68% | 61% | 63% | 67% |
| empty_joints | 63% | 61% | 71% | 58% | 54% | 72% | 79% | 51% | 62% | 65% |
| guarded_joints | 78% | 85% | 83% | 67% | 75% | 81% | 81% | 76% | 78% | 78% |
| frontier_units | 89% | 96% | 92% | 79% | 88% | 91% | 89% | 90% | 90% | 88% |
| inner_units | 92% | 90% | 95% | 91% | 92% | 92% | 92% | 92% | 93% | 88% |
| units_lost | 36% | 30% | 36% | 43% | 36% | 37% | 40% | 33% | 37% | 33% |
| contact | 21% | 14% | 21% | 29% | 21% | 22% | 22% | 21% | 20% | 26% |
| hq_guard | 52% | 48% | 52% | 58% | 52% | 52% | 58% | 48% | 53% | 52% |
| frontier_towers | 48% | 43% | 47% | 54% | 48% | 49% | 43% | 52% | 47% | 51% |
| tower_cover | 60% | 49% | 61% | 70% | 60% | 60% | 58% | 62% | 59% | 63% |
| spent_units | 83% | 91% | 88% | 71% | 81% | 85% | 81% | 85% | 83% | 83% |
| spent_buildings | 53% | 44% | 52% | 64% | 53% | 53% | 53% | 53% | 53% | 52% |
| kept_share | 35% | 34% | 35% | 37% | 38% | 32% | 35% | 35% | 34% | 39% |
| moves | 88% | 96% | 92% | 76% | 87% | 89% | 88% | 88% | 89% | 86% |
| trains | 84% | 93% | 88% | 71% | 83% | 85% | 82% | 85% | 84% | 83% |
| builds | 53% | 44% | 52% | 64% | 53% | 53% | 53% | 53% | 53% | 52% |
| captured | 80% | 95% | 82% | 64% | 80% | 81% | 75% | 84% | 80% | 80% |
| lost | 20% | 5% | 18% | 37% | 20% | 20% | 24% | 17% | 19% | 22% |
| cells_change | 87% | 97% | 93% | 71% | 86% | 88% | 86% | 88% | 88% | 85% |
| material_change | 95% | 97% | 97% | 92% | 95% | 96% | 94% | 96% | 96% | 94% |

## v004 red, v005 blue: the two bots at turn 10

The ten metrics on which the bots' means differ most at turn 10 in these games, as `v004` / `v005`, at turns 8, 10 and 12.

| Metric | Turn 8 | Turn 10 | Turn 12 |
| --- | ---: | ---: | ---: |
| max_cut | 16.0 / 2.5 | 20.4 / 3.3 | 19.9 / 4.1 |
| total_cut | 35.5 / 6.1 | 43.7 / 8.2 | 41.7 / 11.3 |
| frontier_towers | 0.0 / 0.0 | 0.0 / 0.2 | 0.1 / 0.2 |
| empty_joints | 6.3 / 1.0 | 4.3 / 1.6 | 3.6 / 2.0 |
| joints | 7.3 / 1.3 | 6.2 / 2.5 | 5.4 / 3.2 |
| towers | 0.0 / 0.0 | 0.2 / 0.5 | 0.4 / 0.7 |
| far_cells | 0.0 / 0.2 | 0.6 / 1.4 | 1.3 / 4.7 |
| tower_cover | 0.0 / 0.0 | 0.7 / 1.6 | 1.6 / 2.5 |
| material_change | 40.5 / 45.9 | 12.5 / 22.2 | -4.1 / 26.6 |
| spent_buildings | 5.5 / 11.5 | 1.6 / 2.8 | 2.0 / 1.8 |

## v005 red, v004 blue: the two bots at turn 10

The ten metrics on which the bots' means differ most at turn 10 in these games, as `v004` / `v005`, at turns 8, 10 and 12.

| Metric | Turn 8 | Turn 10 | Turn 12 |
| --- | ---: | ---: | ---: |
| far_cells | 0.3 / 0.0 | 2.9 / 0.2 | 6.1 / 0.8 |
| total_cut | 75.1 / 16.2 | 77.2 / 7.2 | 86.3 / 8.1 |
| max_cut | 19.5 / 13.4 | 25.5 / 3.4 | 28.4 / 3.4 |
| hq_guard | 0.1 / 0.0 | 0.1 / 0.7 | 0.1 / 0.3 |
| empty_joints | 9.5 / 1.2 | 6.6 / 1.7 | 6.8 / 1.6 |
| joints | 10.0 / 2.2 | 7.8 / 2.4 | 8.1 / 2.4 |
| mines | 0.4 / 1.1 | 0.4 / 1.3 | 0.4 / 1.3 |
| frontier_towers | 0.0 / 0.0 | 0.1 / 0.2 | 0.1 / 0.3 |
| room | 18.9 / 11.1 | 17.6 / 7.2 | 16.1 / 5.3 |
| material_change | 43.4 / 41.3 | 23.2 / 9.9 | 15.4 / 12.4 |
