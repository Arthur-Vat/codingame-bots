# How zasmu takes the middle (2026-11-19)

Phase 1 of the early-game rework ([2026-11-19-early-game.md](2026-11-19-early-game.md)), started on the owner's word ("Go on with phase 1", 2026-11-19) as the short study that feeds phase 2: which units reach the middle chokes, from which turn and from where; how the trainings follow; the gold kept; how the seat and the map change it. It ends with what the planner of phase 2 must be able to do, each with a measure.

## Data and method

- `ice-and-fire-review codingame-middle` (this note's command) replays the CodinGame games of the data branch `claude/codingame-games` at 5788694 and measures each side up to the end of its ply of game turn 12, on maps with at least 2 middle chokes (the choke cells of the middle band, [2026-11-19-early-game.md](2026-11-19-early-game.md)); 2,370 of the 5,934 games of the groups are on maps with fewer and are left out. Groups: zasmu, the players ranked 2 to 7, and our releases v014, v026 and v029.
- A middle choke is **taken by a move** when, after the side's ply, a unit it had before the ply stands on it; **by a training** when the unit standing on it is new. For a move, the unit's travel is counted from the cell it was trained on: the steps between, and the turns it took beyond them (its detour). A training's **gap** is the trained cell's steps to the nearest middle choke the side does not hold, less the fewest such steps over the cells it could train on (its active cells and their neighbours, not the enemy's, empty); gap 0 is the cell nearest the middle. A move goes **toward** the middle when it shortens the unit's steps to such a choke.
- **Every CodinGame game is against us,** so groups face different opponents: zasmu and the top players face our releases, while our releases face the top players (as red, mostly in test games). For a fair comparison, `early-openings` now also measures the bot's own games from the opening suite's starts ([openings/README.md](../../openings/README.md)), where it meets the very releases the player met on the same map and seat. The tables below give both.

## Zasmu against v029 from the same starts

v029 played from zasmu's starts on maps with middle chokes (28 red, 125 blue), against the release zasmu met there (`early-openings`, time scale 0.487); zasmu's figures are its own CodinGame games on such maps (32 red, 343 blue). Red / blue:

| Measure (up to the end of the ply of game turn 12) | zasmu | v029 |
| --- | --- | --- |
| First middle choke held (mean game turn) | 6.8 / 5.6 | 7.2 / 6.3 |
| Holds one by turn 7 | 94% / 93% | 68% / 85% |
| Share held at the start of turn 12 | 0.68 / 0.72 | 0.41 / 0.57 |
| Taken by a move, a game | 2.28 / 2.38 | 1.54 / 2.05 |
| Taken by a training, a game | 1.78 / 2.10 | 0.89 / 1.49 |
| Trainings at gap 0 | 75% / 68% | 64% / 65% |
| Moves toward the middle | 57% / 58% | 65% / 61% |

From all the suite's starts (zasmu's, both kinds of maps), at the start of turn 12: active cells 36.6 / 41.1 against 34.4 / 38.3, gold 39 / 48 against 52 / 58.

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

- **Zasmu is not better at walking; it is earlier and trains onto the chokes.** Its units that take a choke by moving were trained about 2 steps from it and reach it with no detour 9 times in 10; v029's do the same (no detour 87-91%). v029 even heads toward the middle more often (61-65% of its moves against 57-58%: zasmu's other moves spread its land). The gap is elsewhere:
  - **Time:** zasmu holds a middle choke by turn 7 in 93-94% of games in both seats; v029 in 68% as red and 85% as blue, from the same starts.
  - **Trainings onto the chokes:** zasmu takes 1.8 to 2.1 middle chokes a game by training a unit on them, v029 0.9 to 1.5. Its trainings land on the cell nearest the middle (gap 0) more often as red (75% against 64%).
  - **Number:** zasmu takes 4.1 (red) to 4.5 (blue) middle chokes a game by turn 12 against v029's 2.4 to 3.5, and holds 0.68 / 0.72 of them at turn 12 against 0.41 / 0.57. It loses about as many as v029 does in its own games (0.6 to 0.9 a game): the gap is in taking, not keeping.
- **Its gold goes into it:** at turns 8 and 10 v029 now keeps about what zasmu keeps (22-24 and 35 against 20-24 and 33-36 on CodinGame), but by turn 12, from the same starts, it keeps 52 / 58 against 39 / 48: zasmu's turns 10 to 12 buy the chokes and the land around them.
- **Red is where we lose most.** As red, from zasmu's red starts, v029 takes its first middle choke 0.4 turn later than zasmu (7.2 against 6.8), misses turn 7 a third of the time, and holds 0.41 of the chokes at turn 12 against 0.68.
- **The other top players** are close to zasmu in time (blue, first choke at turns 5.5 to 5.9 except _iahelka's 7.0) and in the number taken; Pduhard- holds as many at turn 12 (0.74). Zasmu's share at turn 12 is the highest with Pduhard-'s; the race for the middle is what the top of Legend does, and zasmu does it most surely.

## What the planner of phase 2 must do

Each with its measure (`early-openings` on suite v1 for the bot, `codingame-middle` for the players), red / blue, v029 now and the target (zasmu's own):

| # | The planner must | Measure | v029 | Target |
| --- | --- | --- | --- | --- |
| 1 | Reach a middle choke early, in both seats | Holds one by turn 7, on maps with middle chokes | 68% / 85% | 94% / 93% |
| 2 | Train units onto the middle chokes, not only walk to them | Middle chokes taken by a training a game | 0.89 / 1.49 | 1.78 / 2.10 |
| 3 | Train on the frontier cell nearest the middle | Trainings at gap 0 | 64% / 65% | 75% / 68% |
| 4 | Take most of the middle chokes by turn 12 | Share held at the start of turn 12 | 0.41 / 0.57 | 0.68 / 0.72 |
| 5 | Spend its gold on that by turn 12, without losing land elsewhere | Gold, and active cells, at the start of turn 12 (suite, zasmu's starts) | 52 / 58 gold, 34.4 / 38.3 cells | 39 / 48 gold, 36.6 / 41.1 cells |
| 6 | Aim at the nearest middle choke from turn 1, more urgently on maps with many | First middle choke on maps with 4 or more (CodinGame games) | 7.2 / 6.0 | 6.7 / 5.3 |

What it need not change: its units already walk straight to their target and toward the middle; and it loses about as many chokes as zasmu, so keeping them needs no new rule before phase 2's two-sided search.

## Limits

- Zasmu's red rows rest on 32 games (and 28 starts for v029 from them); the red targets are the least sure.
- The players' figures are from their games against our releases; the bot's from its games against the same releases from the same starts, but its opponents answer the bot, not the player, so they need not play the same moves.
- On maps with fewer than 2 middle chokes (40% of the games) this study says nothing; there the plan's other targets (active cells, Voronoi share) stand alone.
- The bot's figures vary from run to run (red: 1.29 to 1.54 moving captures a game in three runs of the 28 starts).

## Reproduce

```sh
cargo run --release -p ice-and-fire-review -- codingame-middle --data DATA --me Tarleya --players zasmu,Pduhard-,karliso,_iaz,reCurse,_iahelka --releases ice-and-fire-v014,ice-and-fire-v026,ice-and-fire-v029
cargo run --release -p ice-and-fire-review -- early-openings --suite games/ice-and-fire/openings/suite-v1.tsv --bot v029=BINARY --opponent ice-and-fire-v014=BINARY ... --time-scale 0.487
```
