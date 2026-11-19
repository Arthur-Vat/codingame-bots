# The early game in territory measures, and a plan to rework ours (2026-11-19)

The owner asked, on 2026-11-19, whether zasmu's early game can be modelled with the number of active cells, the centre and the chokepoints, and the Voronoi share alone; then for those measures on many bots, a heuristic built from them, a closer look at the centre and the chokepoints roughly equidistant from both HQs, and a plan for a large rework of our early game that reaches zasmu's level, judged on early games first.

## Data and method

- `ice-and-fire-review codingame-early` (this note's command) replays the games of the data branch `claude/codingame-games` (at 9a7fa3a plus v029's ranking games: 7,026 ranking and test games) and writes, for each side at the start of each of its plies up to game turn 15: its active cells; its owned cells within one step of even between the HQs (`centre`) and the steps from its land to such a cell; its share of the map's choke cells (cells that at least 40% of the shortest HQ-to-HQ paths go through), all of them and those of the middle band (at most 4 steps from even between the HQs); its Voronoi share (cells nearer its land than the enemy's, over the cells nearer one side); and whether it won.
- Groups: zasmu; the players ranked 2 to 7 on the 2026-11-19 snapshot other than us (Pduhard-, karliso, _iaz, reCurse, _iahelka); the other players; our releases. Figures are by seat (red is seat 0, which starts with one cell, blue two). Every game is a game against us, so the other players' figures are against our bot; our red rows are mostly test games against the top players.
- "Predicts the winner" is the AUC of the difference between the two sides at the same turn (0.5 is chance), and for combinations a logistic regression scored by 5-fold cross-validation. These are correlations: stronger players both take more land and win more.
- The scripts that computed the tables were scratch Python over the command's CSV (not committed).

## What predicts the winner

| Game turn | Active cells | Centre cells | Steps to the centre | All chokes | Voronoi share | The five together |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 5 | 0.558 | 0.500 | 0.494 | 0.515 | 0.508 | 0.574 |
| 8 | 0.675 | 0.587 | 0.535 | 0.520 | 0.593 | 0.693 |
| 10 | 0.785 | 0.666 | 0.630 | 0.594 | 0.685 | 0.788 |
| 12 | 0.852 | 0.734 | 0.662 | 0.664 | 0.792 | 0.853 |

The number of active cells carries nearly all of it: added to it, the centre, the chokes and the Voronoi share raise the cross-validated AUC by at most 0.02 at turns 7 to 12.

## The groups at game turn 10 (red / blue)

| Group | Active cells | Centre cells | Voronoi share | Middle chokes held |
| --- | --- | --- | --- | --- |
| zasmu | 30.5 / 33.9 | 2.3 / 3.1 | 0.53 / 0.58 | 0.64 / 0.69 |
| players ranked 2 to 7 | 29.9 / 32.7 | 1.8 / 2.8 | 0.51 / 0.57 | 0.50 / 0.59 |
| other players | 26.9 / 30.9 | 1.4 / 2.4 | 0.48 / 0.54 | 0.38 / 0.51 |
| v028 | 28.0 / 29.6 | 0.8 / 1.7 | 0.43 / 0.45 | 0.34 / 0.41 |
| v029 | 28.0 / 30.6 | 0.9 / 2.0 | 0.42 / 0.48 | 0.35 / 0.45 |

(Middle chokes: games on maps with 2 or more of them, 4,249 of the 7,026; 40% of the maps have none at the 40% threshold.)

A heuristic fitted at turn 10 on the five measures (per unit: active cells +0.194, all-chokes share +1.94, Voronoi share +0.26, centre and steps to the centre about 0), read as the chance to beat an average other player of the same seat: zasmu 67% / 64%, ranked 2 to 7 65% / 58%, v029 57% / 50%, v028 57% / 45%, v014 41% / 47%.

## The centre and the chokepoints

- **Zasmu cannot be told from the other top players by active cells, the centre or the Voronoi share** up to turn 10; its lead in active cells shows from turn 12 (37.7 against 34.7 as red). Its 94% of wins as blue against 71% for the players ranked 2 to 7 comes mostly later (the wall and the strike of the zasmu study).
- **It can by the middle chokes.** As blue it holds 37% of them at turn 7 (ranked 2 to 7: 28%), 69% at turn 10 (59%); it holds more of them than its opponent at turn 10 in 313 of 372 games. All groups reach their first middle choke at turn 7 (blue) or 8 (red), so the difference is how many and how fast.
- **We lose that contest.** v029 holds fewer middle chokes than its opponent at turn 10 in 74 of its 153 games (on such maps), and wins 42% of those, 81% of the 36 where it holds more (v026: 38% of 496 and 76% of 294).
- **The middle chokes block the other side.** On maps with middle chokes, from turn 8 to 13 a side grows by about 3.9 cells less (±0.5) when its opponent holds all of them at turn 8 rather than none, the active cells of both sides and its own Voronoi share held equal; its own Voronoi share at turn 8 predicts its growth most (+11 cells per unit of share, ±1.8).
- **All chokes, without the band, mislead:** most choke cells crowd around the HQs, where every shortest path starts or ends, so a compact side holds them early (our releases hold half of them by turn 8 to 10, earlier than the top players) and the share predicts less growth, not more. This is why the zasmu study (2026-11-18) found the chokepoint rush "not supported".
- **The centre cells** (within one step of even) add nothing to the active cells as a predictor.

So the three measures model the early game well in general (with the active cells first), and zasmu's own early trait is the race for the contested chokes; our gap is about 2 active cells, 0.09 of Voronoi share and a quarter of the middle chokes at turn 10.

## The plan: rework the early game

The owner chose (forms, 2026-11-19): the early score and duels cut at turn 12 as the judge; a two-sided early search first, then a learned value; a generous compute budget (training runs and long batches without asking, within CI's limits); this plan recorded and started in a new session. It replaces stage F of the curriculum ([2026-11-18-zasmu-strategy.md](2026-11-18-zasmu-strategy.md), "an opening toward a target position") and does not stop at a quick win.

**Target:** at game turn 10, zasmu's levels in both seats against several opponents: about 31 / 34 active cells, a Voronoi share of 0.53 / 0.58 and 64% / 69% of the middle chokes.

### Phase 0: an early-game judge

1. **An early score:** a win probability fitted on the CodinGame positions of game turns 10 to 12 (both sides' active cells, Voronoi share, middle chokes, units and their levels, gold and income), calibrated, and checked on held-out games.
2. **Duels cut at game turn 12:** an arena mode that ends a game at turn 12 and scores it with the early score; screenings and an SPRT-like test on it, many times faster than full games.
3. **Validation before use:** in self-play between our releases, the early score's difference at turn 12 must predict the full game's winner (its AUC reported against the full-game result); if it does not, the judge is not used and the plan stops here for the owner's decision.
4. **Opening suites:** the top players' positions of turns 5 to 10, to compare what our planner reaches from the same maps and starts.

### Phase 1: how zasmu takes the middle

Which units reach the middle chokes, from which turn, along which paths; how its trainings follow (the cell nearest the enemy, the gold it keeps); how the map's middle (few or many chokes) and the seat change it. The result is a list of what the planner must be able to do, each with a measure.

### Phase 2: a two-sided early planner

A search dedicated to game turns 1 to about 12, replacing the opening (its 27 weight sets and its solo plan) and the first turns of the middle-game search:

- on turn 1 (about 1 s), a target position: the middle chokes and the Voronoi frontier to reach, per map and seat;
- each turn, units matched to target cells by distance (moves) and trainings where a target lacks a unit, as plans searched against an opponent that expands too (a greedy expander first, then a light version of the planner itself), not one that passes;
- plans scored by the early score at turn 12.

Judged by duels cut at turn 12 against v029 and other bots in both seats, then the turn-10 measures against the target.

### Phase 3: a learned early value

The early score replaced by a value learned from positions: first a fit on hand-made features (territory, middle chokes, units, gold, distances, the frontier) from the CodinGame games, then a small network trained on the planner's self-play (the UTTT trainer and its packing of weights into the bundle are the starting point). Standard library only, within the 100,000-character limit.

### Phase 4: the handover to the middle game

Where the early planner hands over to the middle-game search (game turn, gold kept, units placed), then a release through the SPRT against the release of the day, and a probe on CodinGame (ADR 0060) to read its early measures against the real top players.

### Acceptance and risks

Each step must pass, in order: duels cut at turn 12 against v029 (both seats); the turn-10 measures at the target against several opponents; the SPRT on full games; on CodinGame, its ranking games' early measures beside zasmu's. Risks: a planner tuned to the early score may reach positions that score well at turn 12 and lose later (hence the validation of phase 0 and a full-game SPRT at the end of each phase); our data holds only games against us; a two-sided search must fit in 45 ms a turn after the first.
