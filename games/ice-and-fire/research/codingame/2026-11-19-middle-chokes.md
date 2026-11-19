# How zasmu takes the middle (2026-11-19)

Phase 1 of the early-game rework ([2026-11-19-early-game.md](2026-11-19-early-game.md)), started on the owner's word ("Go on with phase 1", 2026-11-19) as the short study that feeds phase 2: which units reach the middle chokes, from which turn and from where; how the trainings follow; the gold kept; how the seat and the map change it. It ends with what the planner of phase 2 must be able to do, each with a measure.

## Data and method

- `ice-and-fire-review codingame-middle` (this note's command) replays the CodinGame games of the data branch `claude/codingame-games` at 5788694 and measures each side up to the end of its ply of game turn 12, on maps with at least 2 middle chokes (the choke cells of the middle band, [2026-11-19-early-game.md](2026-11-19-early-game.md)); 2,370 of the 5,934 games of the groups are on maps with fewer and are left out. Groups: zasmu, the players ranked 2 to 7, and our releases v014, v026 and v029.
- A middle choke is **taken by a move** when, after the side's ply, a unit it had before the ply stands on it; **by a training** when the unit standing on it is new. For a move, the unit's travel is counted from the cell it was trained on: the steps between, and the turns it took beyond them (its detour). A training's **gap** is the trained cell's steps to the nearest middle choke the side does not hold, less the fewest such steps over the cells it could train on (its active cells and their neighbours, not the enemy's, empty); gap 0 is the cell nearest the middle. A move goes **toward** the middle when it shortens the unit's steps to such a choke. The candidates are those at the start of the ply, so a training placed after the ply's moves or after another training can count as gap 0 when it is not quite: gap 0 is a lenient measure.
- **Where the land comes from** (asked by the owner on 2026-11-19), on every map: each cell a side gains during its own ply is counted by its owner just before the ply: **neutral**; **the enemy's** (a cell the side never had); or **retaken** (a cell of the side's that the enemy had taken). Cells it loses during the enemy's plies are counted too. A lost cell never turns neutral in this game: it passes to the enemy.
- **Every CodinGame game is against us,** so groups face different opponents: zasmu and the top players face our releases, while our releases face the top players (as red, mostly in test games). For a fair comparison, `early-openings` now also measures the bot's own games from the opening suite's starts ([openings/README.md](../../openings/README.md)), where it meets the very releases the player met on the same map and seat. The tables below give both.

## Zasmu, and v029 from zasmu's starts

v029 played from zasmu's starts on maps with middle chokes (28 red, 125 blue), against the release zasmu met there, to the end of game turn 12 (`early-openings`, time scale 0.487). Zasmu's figures are its own CodinGame games against our releases on such maps (32 red, 343 blue), so the opponents are the same releases but the samples are not the same games. Red / blue:

| Measure (up to the end of game turn 12) | zasmu | v029 |
| --- | --- | --- |
| First middle choke held (mean game turn) | 6.8 / 5.6 | 7.3 / 6.4 |
| Holds one by turn 7 | 94% / 93% | 64% / 85% |
| Share held at the start of turn 12 | 0.68 / 0.72 | 0.42 / 0.59 |
| Taken by a move, a game | 2.28 / 2.38 | 1.68 / 2.06 |
| Taken by a training, a game | 1.78 / 2.10 | 1.04 / 1.43 |
| Trainings at gap 0 | 75% / 68% | 64% / 64% |
| Moves toward the middle | 57% / 58% | 66% / 61% |

From all of zasmu's starts in the suite (both kinds of maps), at the start of turn 12: active cells 36.6 / 41.1 against 33.8 / 38.5, gold 39 / 48 against 52 / 60.

## The groups in their own CodinGame games

| Group | Seat | Games | First middle choke | By turn 7 | Held at 12 | Taken by a move / a training | Lost to the enemy | Moving captures without a detour (steps from the training cell) | Trainings at gap 0 | Gold at turn 8 / 10 |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | --- | ---: | --- |
| zasmu | red | 32 | 6.8 | 94% | 0.68 | 2.28 / 1.78 | 0.94 | 88% (2.0) | 75% | 20 / 33 |
| zasmu | blue | 343 | 5.6 | 93% | 0.72 | 2.38 / 2.10 | 0.69 | 91% (2.1) | 68% | 24 / 36 |
| Pduhard- | blue | 345 | 5.8 | 94% | 0.74 | 2.69 / 1.74 | 0.70 | 90% (2.1) | 68% | 24 / 47 |
| karliso | blue | 340 | 5.5 | 98% | 0.60 | 2.27 / 1.58 | 0.88 | 98% (2.2) | 61% | 25 / 40 |
| _iaz | blue | 342 | 5.9 | 93% | 0.62 | 2.23 / 1.87 | 1.04 | 90% (1.9) | 67% | 25 / 35 |
| reCurse | blue | 349 | 5.9 | 93% | 0.50 | 2.33 / 1.39 | 1.19 | 87% (3.0) | 59% | 24 / 41 |
| _iahelka | blue | 343 | 7.0 | 73% | 0.53 | 2.31 / 1.27 | 0.76 | 84% (2.3) | 55% | 29 / 42 |
| v029 | red | 130 | 7.5 | 62% | 0.37 | 1.42 / 0.96 | 0.68 | 91% (3.1) | 60% | 22 / 35 |
| v029 | blue | 53 | 6.4 | 85% | 0.51 | 1.89 / 1.55 | 0.60 | 87% (2.2) | 58% | 24 / 35 |
| v026 | red | 970 | 7.5 | 54% | 0.39 | 1.64 / 1.31 | 0.87 | 91% (2.8) | 65% | 33 / 35 |

The players ranked 2 to 7 as red (35 to 43 games each) take their first middle choke at turns 6.5 to 7.6 and hold 0.34 to 0.68 of them at turn 12. The full tables, with the moves and every release, are in the command's output.

By the map: on maps with 4 or more middle chokes zasmu takes its first one about a turn earlier than on maps with 2 or 3 (blue 5.3 against 6.5, red 6.7 against 7.3), and so does everyone; v029 too (blue 6.0 against 8.0).

## What it shows

- **Zasmu is not better at walking; it is earlier, trains nearer the chokes and onto them.** Its units that take a choke by moving were trained about 2 steps from it and reach it with no detour 9 times in 10; v029's do the same (no detour 87-91%). v029 even heads toward the middle more often (61-65% of its moves against 57-58%: zasmu's other moves spread its land). The gap is elsewhere:
  - **Time:** zasmu holds a middle choke by turn 7 in 93-94% of games in both seats; v029 in 64% as red and 85% as blue, from zasmu's starts.
  - **Where the walkers start:** as red, v029's units that take a choke by moving were trained 3.1 steps from it in its CodinGame games (v026 2.8), zasmu's 2.0: a step more to walk, which fits its later arrival (requirement 3). As blue both start about 2 steps away.
  - **Trainings onto the chokes:** zasmu takes 1.8 to 2.1 middle chokes a game by training a unit on them, v029 1.0 to 1.4. Its trainings land on the cell nearest the middle (gap 0) more often as red (75% against 64%).
  - **Number:** zasmu takes 4.1 (red) to 4.5 (blue) middle chokes a game by turn 12 against v029's 2.7 to 3.5, and holds 0.68 / 0.72 of them at turn 12 against 0.42 / 0.59. It loses about as many a game as v029 does in its own games (0.6 to 0.9), but v029 holds fewer and meets other opponents there, so this does not show that v029 keeps its chokes as well; taking is the larger gap.
- **Its gold goes into it:** at turns 8 and 10 v029 now keeps about what zasmu keeps (22-24 and 35 against 20-24 and 33-36 on CodinGame), but by turn 12, from the same starts, it keeps 52 / 60 against 39 / 48: zasmu's turns 10 to 12 buy the chokes and the land around them.
- **Red is where we lose most.** From zasmu's red starts, v029 misses a middle choke by turn 7 a third of the time (64% against 94%) and holds 0.42 of them at turn 12 against 0.68. In the turn of its first choke it lags zasmu less as red (0.5 turn) than as blue (0.8), so red's gap is how often it is late, not by how much on average.
- **The other top players** are close to zasmu in time (blue, first choke at turns 5.5 to 5.9 except _iahelka's 7.0) and in the number taken; Pduhard- holds as many at turn 12 (0.74). Zasmu's share at turn 12 is the highest with Pduhard-'s; the race for the middle is what the top of Legend does, and zasmu does it most surely.

## Where the land comes from, and zasmu's Voronoi lead

Cumulative cells by the end of game turn 10 and 12, all maps: zasmu in its CodinGame games against our releases (53 red, 572 blue), v029 from zasmu's starts in the suite against the same releases (44 red, 210 blue starts). "From the enemy" is the enemy's cells and the retaken ones together; retaken cells are few (0.03 to 0.09 a turn for every group, last table).

| | Seat | By turn 10: neutral / from the enemy / lost | By turn 12: neutral / from the enemy / lost |
| --- | --- | --- | --- |
| zasmu | red | 34.1 / 1.9 / 1.6 | 40.2 / 4.7 / 4.2 |
| v029, zasmu's starts | red | 31.0 / 0.8 / 1.4 | 36.1 / 2.5 / 3.1 |
| zasmu | blue | 35.5 / 2.8 / 1.0 | 41.6 / 5.1 / 2.8 |
| v029, zasmu's starts | blue | 34.0 / 1.2 / 1.0 | 40.0 / 3.7 / 2.3 |

Zasmu's surplus over v029, as cells gained (red / blue):

| | From neutral cells | From our cells | Lost (more for zasmu) | Net |
| --- | ---: | ---: | ---: | ---: |
| By turn 10 | +3.1 / +1.5 | +1.1 / +1.6 | 0.2 / 0.0 | +4.0 / +3.1 |
| By turn 12 | +4.1 / +1.6 | +2.2 / +1.4 | 1.1 / 0.5 | +5.2 / +2.5 |

Turn by turn, mean cells (neutral / from the enemy / lost):

| | Seat | Turn 7 | Turn 8 | Turn 9 | Turn 10 | Turn 11 | Turn 12 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| zasmu | red | 5.4 / 0.0 / 0.1 | 4.8 / 0.3 / 0.2 | 4.3 / 0.6 / 0.5 | 3.9 / 1.0 / 0.7 | 3.3 / 1.4 / 1.3 | 2.7 / 1.4 / 1.4 |
| v029, zasmu's starts | red | 4.4 / 0.0 / 0.0 | 4.7 / 0.2 / 0.4 | 3.5 / 0.2 / 0.7 | 2.8 / 0.4 / 0.4 | 2.5 / 0.8 / 0.7 | 2.5 / 0.9 / 1.0 |
| zasmu | blue | 5.1 / 0.0 / 0.0 | 4.8 / 0.5 / 0.2 | 4.1 / 0.9 / 0.3 | 3.8 / 1.3 / 0.5 | 3.4 / 1.1 / 0.7 | 2.7 / 1.3 / 1.1 |
| v029, zasmu's starts | blue | 5.0 / 0.0 / 0.0 | 4.5 / 0.1 / 0.3 | 3.7 / 0.4 / 0.3 | 3.4 / 0.7 / 0.4 | 3.1 / 1.1 / 0.6 | 2.9 / 1.4 / 0.7 |

Every group in its own CodinGame games and v029 from zasmu's starts, means a turn over turns 1 to 12, and by turn 12 (`codingame-middle`, `early-openings`):

| Group | Seat | Games | Gained a turn: neutral / enemy's / retaken | Lost a turn | By turn 12: neutral / from the enemy / lost | Turn 12: neutral / from the enemy |
| --- | --- | ---: | --- | ---: | --- | --- |
| zasmu | red | 53 | 3.35 / 0.32 / 0.07 | 0.35 | 40.2 / 4.7 / 4.2 | 2.7 / 1.4 |
| zasmu | blue | 572 | 3.47 / 0.35 / 0.08 | 0.24 | 41.6 / 5.1 / 2.8 | 2.7 / 1.3 |
| ranked 2 to 7, pooled | red | 317 | 3.16 / 0.32 / 0.09 | 0.43 | 38.0 / 4.9 / 5.1 | 2.4 / 1.7 |
| ranked 2 to 7, pooled | blue | 2,845 | 3.37 / 0.37 / 0.09 | 0.33 | 40.4 / 5.5 / 4.0 | 2.4 / 1.8 |
| v026 | red | 1,628 | 2.86 / 0.29 / 0.05 | 0.40 | 34.3 / 4.1 / 4.8 | 2.0 / 1.5 |
| v026 | blue | 89 | 2.92 / 0.37 / 0.14 | 0.44 | 35.0 / 6.1 / 5.3 | 1.6 / 1.3 |
| v029 | red | 217 | 2.96 / 0.27 / 0.03 | 0.32 | 35.5 / 3.7 / 3.9 | 2.3 / 1.5 |
| v029 | blue | 81 | 3.06 / 0.36 / 0.06 | 0.31 | 36.7 / 5.1 / 3.7 | 1.9 / 1.6 |
| v029, zasmu's starts | red | 44 | 3.01 / 0.17 / 0.03 | 0.26 | 36.1 / 2.5 / 3.1 | 2.5 / 0.9 |
| v029, zasmu's starts | blue | 210 | 3.33 / 0.26 / 0.05 | 0.19 | 40.0 / 3.7 / 2.3 | 2.9 / 1.4 |

For every group (zasmu, the players ranked 2 to 7 pooled, v026, v029, in their CodinGame games, and v029 from the suite), the mean gains from the enemy never exceed the mean neutral gains by turn 12: neutral land is still most of what every side takes, though the two lines close at turns 11 and 12 (zasmu 2.7 against 1.3 to 1.4 a turn; the players ranked 2 to 7 2.4 against 1.7 to 1.8). Taking cells from the enemy starts at turn 8 for everyone. The trainings on cells that are not neutral are nearly all on the enemy's (2.1 to 3.1 a game by turn 12, on maps with middle chokes); zasmu and the other top players train on their own cells 0.07 to 0.10 times a game, our releases never (E045).

**So zasmu's Voronoi lead over v029 by turns 10 to 12 (0.52 / 0.57 against 0.44 / 0.53 at turn 10, 0.53 / 0.59 against 0.45 / 0.55 at turn 12, red / blue) comes mostly from neutral land as red and from both about evenly as blue.** As red, about three quarters of its extra gains by turn 10 are neutral (3.1 of 4.2), about two thirds by turn 12 (4.1 of 6.3); it reaches more neutral land at turns 7 to 10 (5.4, 4.8, 4.3, 3.9 a turn against 4.4, 4.7, 3.5, 2.8). As blue, half of its extra gains by turn 10 are our cells (1.6 of 3.1): it takes more of them from turn 8 on (0.5 at turn 8 and 0.9 at turn 9, against 0.1 and 0.4), about a turn ahead. Its losses are not lower than v029's; it trades more, not less. The Voronoi share counts the cells nearer a side's land than the enemy's, so a cell taken from us both adds to zasmu's land and removes one from ours; cell for cell, those blue captures move the share more than neutral gains do.

For the planner: requirement 1 (the race to the middle, from more neutral land at turns 7 to 10) is red's gap; as blue, it must also take our cells from turn 8 (requirement 7 below).

## What the planner of phase 2 must do

Each with its measure, red / blue: for the bot `early-openings` on suite v1 (zasmu's starts) unless the row says otherwise, for the players `codingame-middle`; v029 now and the target (zasmu's own):

| # | The planner must | Measure | v029 | Target |
| --- | --- | --- | --- | --- |
| 1 | Reach a middle choke early, in both seats | Holds one by turn 7, on maps with middle chokes | 64% / 85% | 94% / 93% |
| 2 | Train units onto the middle chokes, not only walk to them | Middle chokes taken by a training a game | 1.04 / 1.43 | 1.78 / 2.10 |
| 3 | Train on the frontier cell nearest the middle, so that walkers start about 2 steps from their choke | Trainings at gap 0; steps from the training cell of the units that take a choke by moving (CodinGame games) | 64% / 64%; 3.1 / 2.2 | 75% / 68%; 2.0 / 2.1 |
| 4 | Take most of the middle chokes by turn 12 | Share held at the start of turn 12 | 0.42 / 0.59 | 0.68 / 0.72 |
| 5 | Spend its gold on that by turn 12, without losing land elsewhere | Gold, and active cells, at the start of turn 12 (all of zasmu's starts) | 52 / 60 gold, 33.8 / 38.5 cells | 39 / 48 gold, 36.6 / 41.1 cells |
| 6 | Aim at the nearest middle choke from turn 1, more urgently on maps with many | First middle choke on maps with 4 or more (CodinGame games: the suite's report does not split by map; `codingame-middle` does) | 7.2 / 6.0 | 6.7 / 5.3 |
| 7 | Take the enemy's cells from turn 8, not only neutral ones | Cells gained from the enemy by turn 10 (all maps) | 0.8 / 1.2 | 1.9 / 2.8 |

What it need not change: its units already walk straight to their target and toward the middle. Whether it keeps its chokes as well as zasmu is not settled here (above); phase 2's two-sided search plans against an opponent that takes them back, which is where keeping shows.

## Limits

- Zasmu's red rows rest on 32 games (and 28 starts for v029 from them); the red targets are the least sure.
- The players' figures are from their games against our releases; the bot's from its games against the same releases from the same starts, but its opponents answer the bot, not the player, so they need not play the same moves.
- On maps with fewer than 2 middle chokes (40% of the games) this study says nothing; there the plan's other targets (active cells, Voronoi share) stand alone.
- The bot's figures vary from run to run (red: 1.29 to 1.68 moving captures a game in five runs of the 28 starts); the tables give the last run, made with this note's code.

## Reproduce

```sh
cargo run --release -p ice-and-fire-review -- codingame-middle --data DATA --me Tarleya --players zasmu,Pduhard-,karliso,_iaz,reCurse,_iahelka --releases ice-and-fire-v014,ice-and-fire-v026,ice-and-fire-v029
cargo run --release -p ice-and-fire-review -- early-openings --suite games/ice-and-fire/openings/suite-v1.tsv --bot v029=BINARY --opponent ice-and-fire-v014=BINARY ... --time-scale 0.487
```
