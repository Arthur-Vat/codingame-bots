# The early game in territory measures, and a plan to rework ours (2026-11-19)

The owner asked, on 2026-11-19, whether zasmu's early game can be modelled with the number of active cells, the centre and the chokepoints, and the Voronoi share alone; then for those measures on many bots, a heuristic built from them, a closer look at the centre and the chokepoints roughly equidistant from both HQs, and a plan for a large rework of our early game that reaches zasmu's level, judged on early games first.

## Data and method

- `ice-and-fire-review codingame-early --kinds ranking,test --turns 15` (this note's command) replays the games of the data branch `claude/codingame-games` (at 9a7fa3a plus v029's ranking games: 7,026 ranking and test games) and writes, for each side at the start of each of its plies up to game turn 15: its active cells; its owned cells within one step of even between the HQs (`centre`) and the steps from its land to such a cell; its share of the map's choke cells (cells that at least 40% of the shortest HQ-to-HQ paths go through), all of them and those of the middle band (at most 4 steps from even between the HQs); its Voronoi share (cells nearer its land than the enemy's, over the cells nearer one side); and whether it won.
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

(Middle chokes: games on maps with 2 or more of them, 4,249 of the 7,026; the other games' maps have fewer than 2 at the 40% threshold, most of them none.)

A heuristic fitted at turn 10 on the five measures (per unit: active cells +0.194, all-chokes share +1.94, Voronoi share +0.26, centre and steps to the centre about 0; the all-chokes weight holds the active cells equal, so it rewards a compact land of the same size, not the chokepoints, see below), read as the chance to beat an average other player of the same seat: zasmu 67% / 64%, ranked 2 to 7 65% / 58%, v029 57% / 50%, v028 57% / 45%, v014 41% / 47%.

## The centre and the chokepoints

- **Zasmu cannot be told from the other top players by active cells, the centre or the Voronoi share** up to turn 10; its lead in active cells shows from turn 12 (37.7 against 34.7 as red). Its 94% of wins as blue against 71% for the players ranked 2 to 7 comes mostly later (the wall and the strike of the zasmu study).
- **It can by the middle chokes.** As blue it holds 37% of them at turn 7 (ranked 2 to 7: 28%), 69% at turn 10 (59%); it holds more of them than its opponent at turn 10 in 313 of 372 games. All groups reach their first middle choke at turn 7 (blue) or 8 (red), so the difference is how many and how fast.
- **We lose that contest.** v029 holds fewer middle chokes than its opponent at turn 10 in 74 of its 153 games (on such maps), and wins 42% of those, 81% of the 36 where it holds more (v026: 38% of 496 and 76% of 294).
- **The middle chokes block the other side.** On maps with middle chokes, from turn 8 to 13 a side grows by about 3.9 cells less (±0.5) when its opponent holds all of them at turn 8 rather than none, the active cells of both sides and its own Voronoi share held equal; its own Voronoi share at turn 8 predicts its growth most (+11 cells per unit of share, ±1.8).
- **All chokes, without the band, mislead:** most choke cells crowd around the HQs, where every shortest path starts or ends, so a compact side holds them early (our releases hold half of them by turn 8 to 10, earlier than the top players) and the share predicts less growth, not more. This is why the zasmu study (2026-11-18) found the chokepoint rush "not supported".
- **The centre cells** (within one step of even) add nothing to the active cells as a predictor.

So the three measures model the early game well in general (with the active cells first), and zasmu's own early trait is the race for the contested chokes; our gap to zasmu at turn 10 is 2.5 to 3.3 active cells, 0.10 to 0.13 of Voronoi share and a quarter of the middle chokes (v028 and v029, red and blue).

## The plan: rework the early game

The owner chose (forms, 2026-11-19): the early score and duels cut at turn 12 as the judge; a two-sided early search first, then a learned value; a generous compute budget (training runs and long batches without asking, within CI's limits); this plan recorded and started in a new session. It replaces stage F of the curriculum ([2026-11-18-zasmu-strategy.md](2026-11-18-zasmu-strategy.md), "an opening toward a target position") and does not stop at a quick win.

**Target:** at game turn 10, zasmu's levels in both seats against several opponents: about 31 / 34 active cells, a Voronoi share of 0.53 / 0.58 and 64% / 69% of the middle chokes.

### Phase 0: an early-game judge

**Status (2026-11-19):** steps 1 to 3 done, with the early score as a *zasmu-likeness* score at the owner's choice ([2026-11-19-zasmu-likeness.md](2026-11-19-zasmu-likeness.md)): it predicts the winner on CodinGame (AUC 0.81 in the games without zasmu, active cells alone 0.85) and in self-play full games (0.89, active cells 0.87), and duels cut at turn 12 (`ice-and-fire-arena-early`) rank five pairings of releases as full games do, in about a tenth of the time. Step 4: the opening suite [openings/suite-v1.tsv](../../openings/suite-v1.tsv) (3,668 games of zasmu and the players ranked 2 to 7, 809 starts) and `early-openings`; from zasmu's starts v029 holds about 2 active cells fewer at turn 10, a quarter to half fewer middle chokes, and scores −0.17 / −1.35 against zasmu's +0.65 / +0.83 (blue / red) ([openings/README.md](../../openings/README.md)).

1. **An early score:** a win probability fitted on the CodinGame positions of game turns 10 to 12 (both sides' active cells, Voronoi share, middle chokes, units and their levels, gold and income), calibrated, and checked on held-out games.
2. **Duels cut at game turn 12:** an arena mode that ends a game at turn 12 and scores it with the early score; screenings and an SPRT-like test on it, many times faster than full games.
3. **Validation before use:** in self-play between our releases, the early score's difference at turn 12 must predict the full game's winner (its AUC reported against the full-game result); if it does not, the judge is not used and the plan stops here for the owner's decision.
4. **Opening suites:** the top players' positions of turns 5 to 10, to compare what our planner reaches from the same maps and starts.

### Phase 1: how zasmu takes the middle

**Status (2026-11-19):** done ([2026-11-19-middle-chokes.md](2026-11-19-middle-chokes.md), command `codingame-middle`): zasmu does not walk better than v029 (both reach chokes with no detour, from about 2 steps); it is earlier (a middle choke by turn 7 in 93-94% of games, v029 64% / 85% from the same starts), trains onto the chokes twice as often, takes 4.1-4.5 a game by turn 12 (v029 2.7-3.5) and spends its gold on it by turn 12. Its Voronoi lead over v029 by turns 10 to 12 comes mostly from neutral land as red and about evenly from neutral land and our cells as blue. Seven planner requirements, each with a measure.

Which units reach the middle chokes, from which turn, along which paths; how its trainings follow (the cell nearest the enemy, the gold it keeps); how the map's middle (few or many chokes) and the seat change it. The result is a list of what the planner must be able to do, each with a measure.

### Phase 2: a two-sided early planner

**Status (2026-11-19):** step 1 tried and dropped ([E072](../../journal/E072-frontier-assignment.md)): the joint assignment lost as the opening (−21 to −121 in the cut duels against v029) and as steps of the solo beam (−42 to −93). At the owner's request the problem was then laid out mathematically and an exact solver of one side's early game written ([2026-11-19-optimal-early-plans.md](2026-11-19-optimal-early-plans.md)): on 78 of zasmu's starts (study run 44626073005), v029's opening holds the most land at turn 6 on four starts in five, but plans with the same land hold 1.9 to 7 times its middle chokes at turns 6 and 7, and 1.45 to 1.9 times its middle cells; the next step (a lexicographic choice in the opening, or the two-sided check) is the owner's.

A search dedicated to game turns 1 to about 12, replacing the opening (its 27 weight sets and its solo plan) and the first turns of the middle-game search:

- on turn 1 (about 1 s), a target position: the middle chokes and the Voronoi frontier to reach, per map and seat;
- each turn, units matched to target cells by distance (moves) and trainings where a target lacks a unit, as plans searched against an opponent that expands too (a greedy expander first, then a light version of the planner itself), not one that passes;
- plans scored by the early score at turn 12.

Judged by duels cut at turn 12 against v029 and other bots in both seats, then the turn-10 measures against the target.

**Steps, revised on 2026-11-19** after phase 1 ([2026-11-19-middle-chokes.md](2026-11-19-middle-chokes.md), its seven requirements) and the owner's points on the early-game claims of the zasmu study ([2026-11-18-zasmu-strategy.md](2026-11-18-zasmu-strategy.md), its verdict table); added at his choice (form: "Into #367, then merge"). Each step is an experiment of its own, judged as above, and built beside the current opening behind a switch so that the cut arena compares them:

1. **Joint frontier assignment.** The owner's points: units should work together, so their goals are not chosen one by one; and not all units should aim at the same cell: the units behind the front should cover a wide frontier. So the targets are the middle chokes plus cells spaced along the Voronoi frontier, one unit for each, and the assignment is solved jointly each turn (least total arrival turns), with the possible trainings at the frontier cells counted as units, so that a training from the front can take a choke while a unit already near covers another target (requirements 1 to 3: a middle choke by turn 7, trainings onto the chokes, trainings at the frontier cell nearest the middle). No penalty for a thin or easily cut shape before about turn 10: the zasmu study found that shape does no harm early (one captured cell would cut off 15 of zasmu's cells at turn 8, but its exposed frontier cells cut off 1), and our opening's compactness term is what keeps our land small.
2. **The gold spent by turn 12** on that land and those chokes, without losing land elsewhere (requirements 4 and 5).
3. **The two-sided search** above: plans against an opponent that expands, so that the spread is checked against the enemy's units, and enemy cells are taken from turn 8 (requirement 7).
4. **The handover** to the middle game (phase 4 below).
5. **Optional, later: an adaptive choice among openings.** Several candidate openings computed on turn 1 and one chosen around turns 4 to 7 from how the opponent expands. Deferred: zasmu's first 4 turns do not depend on its opponent and it reacts from turn 5 (the zasmu study), which a planner that plans again every turn against the opponent it sees already does. This step is taken only if the cut duels or the suite show the planner misreading some opponents.

Left out of this rework, as the zasmu study judged them: the count of TRAIN and MOVE actions (all top players play many; it is to be watched, not optimised), the land left behind the front and the land the opponent will not claim (the middle game's, or the Voronoi race's own).

### Phase 3: a learned early value

The early score replaced by a value learned from positions: first a fit on hand-made features (territory, middle chokes, units, gold, distances, the frontier) from the CodinGame games, then a small network trained on the planner's self-play (the UTTT trainer and its packing of weights into the bundle are the starting point). Standard library only, within the 100,000-character limit.

### Phase 4: the handover to the middle game

Where the early planner hands over to the middle-game search (game turn, gold kept, units placed), then a release through the SPRT against the release of the day, and a probe on CodinGame (ADR 0060) to read its early measures against the real top players.

### Acceptance and risks

Each step must pass, in order: duels cut at turn 12 against v029 (both seats); the turn-10 measures at the target against several opponents; the SPRT on full games; on CodinGame, its ranking games' early measures beside zasmu's. Risks: a planner tuned to the early score may reach positions that score well at turn 12 and lose later (hence the validation of phase 0 and a full-game SPRT at the end of each phase); our data holds only games against us; a two-sided search must fit in 45 ms a turn after the first.
