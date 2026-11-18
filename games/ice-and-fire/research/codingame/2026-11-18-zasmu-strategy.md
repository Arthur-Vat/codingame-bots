# How zasmu wins, checked against the games (2026-11-18)

The owner asked, on 2026-11-18, for a check of what he saw in the top player's games: zasmu (1st, 35.81 points on the 2026-11-18 leaderboard, 5.3 ahead of the 2nd) rushes the centre and the chokepoints, holds the front while building its economy (captures the land inside its lines, takes good trades, builds mines well, keeps few level 2 and 3 units), then banks gold and wins with one cut. He also asked for the economics of each unit level, and for a long-term plan built as a curriculum. This note gives the numbers, a verdict on each observation, and the plan.

## Data and method

- The games of the data branch `claude/codingame-games` (df48af6): 6,246 games replayed with our engine. Only our own games exist, so **every zasmu game is a game against us**: 540 games (zasmu won 502 of them, 93%), 502 as blue (467 of them test games, where CodinGame seats us first) and 38 as red. The same agent played all of them (agent 6803523, 2026-11-07 to 11-18).
- Three groups, always compared within one seat: **Z** = zasmu; **T** = the other players ranked 2 to 6 on 2026-11-18 (karliso, Pduhard-, _iaz, _iahelka, reCurse), 2,711 games against us (T won 72% as blue); **U** = our bot. For the U figures the opponent is in Z or T, and from the middle-game tables on, only v026 games are used (916 as red, 16 as blue). Z and T played the same opponent (us), so the difference between them is zasmu's own play; both are biased by what our bot does.
- A scratch exporter in the review tool (not committed) wrote each ply's board, units, buildings and commands; Python scripts in the session's scratchpad measured the rest. "Turn" means game turn; each figure is a mean per game at the start of the side's ply unless stated, over the games still running at that turn (so late-turn figures describe the longer games).
- Limits: zasmu's play against other bots is not in the data; the "chokepoint" measure is crude (cells on the shortest HQ-to-HQ paths that carry at least 40% of them); enemy cells cut off cannot be attributed to single commands, because the engine recomputes active cells at its own times inside a turn (the unit economics below therefore count captures and kills, not cuts).

## Verdicts

| Observation | Verdict | Evidence |
| --- | --- | --- |
| Rushes to the centre | **Partly.** All top players do; zasmu only a little more than T | Centre cells (equal distance ±1 to both HQs) held at turn 10, blue: Z 3.1, T 2.8, us (red) 0.6. Share of the Voronoi land at turn 10: Z 0.58, T 0.57, us 0.40 |
| Rushes to the chokepoints | **Not supported** by our measure | Share of choke cells held at turn 5, blue: Z 0.16, T 0.26; at turn 7: 0.31 against 0.39. Z catches up at turn 15 |
| Trains from the unit closest to the centre | **Supported** (blue) | In turns 1 to 10, the training is on the trainable cell nearest the enemy HQ: Z 74%, T 63%, us 49% (as red: Z 70%, T 71%) |
| Maximises TRAIN + MOVE early | **True of all top players, not specific to zasmu** | Until turn 7 Z and T play the same counts. Our gap starts at turn 8: moves per turn at turns 10 to 12, Z 5.3 to 5.5, T 4.6, us 3.0 |
| Captures the land behind the front | **Not specific** | Neutral cells at least 2 steps nearer us than the enemy, turn 20: Z 6.0, T 5.0, us 1.8 (ours is compact, so less is left to take) |
| No need for a formation hard to cut early | **Supported** | One captured cell would cut off this many cells of the land: turn 8, Z 15, T 19, us 3 to 7. But the frontier cells are not those joints: the worst cut from one exposed frontier cell is 1 cell for every group. Z thickens its land over time (15 at turn 8, 10 at turn 15) |
| Adapts its opening to the opponent | **From turn 5, by reacting** | Zasmu is deterministic: on the same map, when our moves were identical, its moves were identical in 66 of 68 pairs. When our play differed at turns 1 to 4, zasmu kept the same moves for a median 3 turns (it reacted the same turn in only 17% of pairs). When our play differed at turns 5 to 7, it reacted the same turn in 78% of pairs. So its first 4 turns do not depend on us, and from turn 5 it reacts at once. Whether it picks among precomputed openings or searches cannot be told apart |
| Holds the front | **Yes, with towers rather than units** | See "Towers" below: the largest difference we found |
| Takes only good trades | **Yes** | Share of its plies (turn > 10) that kill units: Z 29 to 31%, T 43 to 46%, us 44%. Unit value it then loses in the reply: Z 10, T 16 to 18, us 22. Unit value lost per opponent reply at turns 11 to 20: Z 7 to 8, T 12, us 14 |
| Builds mines at the right time and places | **Yes**: earlier than T, more of them, safe | See "Mines" |
| Avoids level 2 and 3 units | **Yes** | Upkeep share of gross income, turn 20 / 30: Z 0.18 / 0.19, T 0.38 / 0.44, us 0.30 / 0.37. Level 3 units on the board at turn 20: Z 0.1, T 0.5, us 0.4 |
| Banks gold, then one lethal cut | **Strongly confirmed** | See "The strike" |

## Towers

Zasmu's main difference from every other top player.

| | Z blue | Z red | T blue | us (v026, red) |
| --- | --- | --- | --- | --- |
| Towers built per game | 16.6 | 19.4 | 4.9 | 5.9 |
| Towers on the board, turn 15 / 20 / 25 / 30 (games still running) | 6 / 15 / 24 / 28 | 5 / 12 / 22 / 29 | 3 / 4 / 5 / 5 | 3 / 5 / 6 / 6 |
| Frontier cells protected by a tower (on or next to one), turn 15 / 20 / 25 | 73% / 83% / 84% | 65% / 78% / 88% | 49% / 50% / 52% | 50% / 53% / 56% |

Its towers stand 8 to 11 steps from its HQ and 1 to 4 steps from our land: a wall along the front, built from turn 13 to 19 (quartiles). A tower costs 15 gold once and no upkeep; behind the wall, level 1 units hold the land (they live 4 to 5 of its turns, against 2 to 3 for T and us) and its upkeep stays at 17 to 19% of gross income for the whole game. Only a level 3 unit can enter a protected cell, and our level 3 units cost 20 gold a turn.

## Mines

| | Z blue | Z red | T blue | us (v026, red) |
| --- | --- | --- | --- | --- |
| Mines built per game | 3.8 | 3.3 | 2.6 | 2.7 |
| First mine, game turn (quartiles) | 8 / 9 / 10 | 9 / 10 / 12 | 9 / 12 / none | 8 / 8 / 9 |
| Mines on the board, turn 12 / 20 / 30 | 1.8 / 3.4 / 3.3 | 1.2 / 2.5 / 2.9 | 0.9 / 1.8 / 1.5 | 1.6 / 2.3 / 2.7 |
| Owned free mine spots never mined | 33% | 35% | 54% | 42% |
| Mines lost by the end of the game | 6% | 7% | 14% | 21% |
| Gross income, turn 12 / 20 | 48 / 65 | 42 / 59 | 42 / 50 | 37 / 47 |

- **When:** from turn 7 to 12, as soon as a spot is free and gold allows (gold at the build: median 42; income 35). Chance of building a mine on a ply with a free spot, turns 10 to 14, gold 30 to 59: 44% when a spot is safe (3 or more steps from enemy land), 24% when all are exposed; T does the opposite (28% and 40%). From turn 15, with 60 gold or more, zasmu rarely builds a mine (4 to 20%): it is banking.
- **Where:** a median 6 steps from its HQ and 6 from our land, always the safest free spot (98%), and later behind its tower wall, so it keeps 94% of its mines.

## The strike

| | Z blue | Z red | T blue | us (v026, red) |
| --- | --- | --- | --- | --- |
| Largest single cut, share of the enemy's land (median) | 100% | 100% | 73% | 27% |
| Gold before it (median) | 190 | 234 | 92 | 68 |
| Gold 3, 2, 1 own turns before it | 90, 115, 154 | 100, 129, 174 | 69, 82, 101 | 67, 79, 93 |
| Trainings and buildings on the 2 turns before it | 0.9, 0.3 | 0.6, 0.3 | 1.6, 1.2 | 1.8, 1.3 |
| Ply of the largest cut: trainings (L1, L2, L3), moves | 5.0, 1.2, 3.1; 0.3 | 3.6, 0.5, 4.7; 0.4 | 4.1, 0.8, 1.8; 2.8 | 3.8, 0.2, 2.0; 3.1 |
| Distance from its land to the enemy HQ before the winning ply | 9 | 9 | 5 | 4 |
| Opponent's income at that ply (median) | 23.5 | 13 | 11 | (16 games) |
| Game turn of its wins (median) | 21 | 26 | 18 | 22 |

Zasmu stops spending for about two turns (income ~53), reaches ~200 gold, and in one turn trains a chain of 8 to 10 units from 9 cells away to our HQ, with level 3 units through our towers. It does not wait for us to collapse: our income is still 23 when it strikes. The other top players win the usual way, by finishing an opponent already broken (income 11) from 4 or 5 cells away. Our bot already finds lethal chains ([`tactics/src/chain.rs`](../../tactics/src/chain.rs)) and its evaluation has a pressure term for the gold missing to the chain to the enemy HQ ([`tactics/src/eval.rs`](../../tactics/src/eval.rs), capped at 200); what it lacks is saving for that chain over several turns, and defending against an opponent who does.

## Unit economics by level

Per unit trained, over its life (blue for Z and T, red for us; level 1 kills nothing since it cannot enter a unit's cell):

| | Level 1 | Level 2 | Level 3 |
| --- | --- | --- | --- |
| Cost; upkeep a turn | 10; 1 | 20; 4 | 30; 20 |
| Cells taken (neutral + enemy), Z / T / us | 3.6 / 3.9 / 3.2 | 1.9 / 2.2 / 1.9 | 1.3 / 1.8 / 1.7 |
| Unit value killed, Z / T / us | 0 | 11 / 12 / 12 | 5 / 8 / 10 |
| Upkeep paid over its life, Z / T / us | 6 / 5 / 5 | 14 / 15 / 13 | 7 / 24 / 22 |
| Cells per 10 gold (cost + upkeep), Z / T / us | 2.2 / 2.5 / 2.1 | 0.6 / 0.7 / 0.6 | 0.3 / 0.3 / 0.3 |
| Life in own turns (median; the upkeep above is a mean, so a few long-lived units carry it), Z / T / us | 4 / 3 / 2 | 0 / 2 / 2 | 0 / 0 / 0 |

- **Level 1** is the land unit: about 3.5 cells for 15 gold in all. A cell held earns 1 gold a turn, so a level 1 unit that takes 3 cells and lives 4 turns has paid itself back after about 4 to 5 turns of holding them.
- **Level 2** takes about 2 cells, most of them enemy cells, and kills about 11 gold of units per training; at 4 gold a turn it loses money unless the kill or the cut is worth it. Its value is in cuts, which these counts do not include.
- **Level 3** is a one-turn tool: it passes towers and kills anything, but at 20 gold a turn, keeping one alive for 5 turns costs 130 gold. Zasmu trains 3 to 5 per game, nearly all in the winning ply (upkeep paid: 7), and keeps ~0.1 on the board; T and we keep them (upkeep paid: 22 to 24).
- **Mine:** 15 gold for +4 a turn, paid back in 4 turns, but only while it stays; ours are lost three times as often as zasmu's. **Tower:** 15 gold, no upkeep; zasmu's wall replaces the units others keep at the front.

## What this means for our bot

1. **The early game is not where zasmu beats the other top players**: up to turn 10 Z and T are alike. Our opening gap (Voronoi 0.40 against 0.57, centre cells 0.6 against 3) is a gap to all top players, and the earlier lessons stand (making the opening buy more lost three times; changing who plays turn 7 gained, E067 in #344). The train-at-the-most-advanced-cell rule is the one opening trait where zasmu stands out.
2. **The middle game is where zasmu pulls away**: from turn 10 to 20 its cells (34 → 51) and gross income (37 → 65) keep growing while T levels off (50) and we stall (47). It does this with towers instead of units (upkeep 18%), mines on safe spots, and few fights.
3. **The late game is a plan over several turns**: bank for ~2 to 3 turns, then a ~190-gold training chain from 9 cells away. Our one-turn search cannot want this unless something values gold toward the chain, and it cannot defend against it unless it looks at the opponent's gold over more than one turn.

## Long-term plan: a curriculum

Each level builds on the ones before it and has a measurable target taken from zasmu's numbers above. A level is reached by one or more experiments (one per pull request, screened against the current release, then the SPRT, as in `docs/WORKFLOW.md`); not every level needs a release. Because self-play can hide what only strong opponents punish, each level's last candidate is also checked with a test batch against zasmu and the top 5 on the 10 fixed maps (the collection's batch, `scripts/codingame.sh batch --opponents`), on the owner's approval.

**Level 0 — Measure what zasmu does (no change to the bot).**
Make this note's measures review commands instead of scratch scripts: towers on the board, frontier cells protected, upkeep share, mines lost, gold banked before the largest cut, the distance of the winning chain, and, for both sides each turn, the cost of the cheapest chain to the enemy HQ against gold plus income over the next 1 to 3 turns. Add them to `codingame-report` so every release is compared with zasmu on them. Target: the tables above are produced by one command.

**Level 1 — Hold the front with towers, not units.**
Our towers (6 a game) only guard where the opponent can threaten now (E026). Experiments: value a tower by the frontier cells it protects; prefer a tower to keeping a level 2 or 3 unit at the front; let the evaluation count upkeep over several turns rather than the income of this turn alone. Targets: frontier protected ≥ 75% at turn 20, upkeep ≤ 25% of gross income, unit value lost per opponent reply at turns 11 to 20 near 8 (now 14).

**Level 2 — An economy behind the wall.**
With a held front, mines are safe. Experiments: the first mine around turn 9 on the safest free spot; a safe spot (behind towers, or 3 or more steps from enemy land) worth more than an exposed one; no mine on an exposed spot. Targets: 3 or more mines on the board at turn 20, under 10% of mines lost, gross income ≥ 60 at turn 20.

**Level 3 — Do not die to a strike.**
Before we bank, we must survive opponents who do (zasmu kills us from 9 cells away while our income is 23). Experiments: a threat term on the opponent's chain to our HQ against its gold plus 2 or 3 turns of income (now only the next turn), answered by towers on that chain's path; this also teaches the bot what a wall is worth. Targets: fewer losses to a single cut of 100% of our land in the zasmu batch; the opponent's margin over its chain to our HQ kept negative over 2 turns.

**Level 4 — Bank and strike.**
The capability that most separates zasmu. A turn-level plan: each turn, compute our chain cost C to the enemy HQ and the turns k until gold plus k incomes reach it; while k ≤ 3 and our front is held (levels 1 and 3), switch to banking: spend only on defence and on captures that pay back before the strike, and strike when gold reaches C (the existing lethal finder plays it). Experiments: the horizon k (2, 3, 4), the safety margin over C, how the opponent's expected towers raise C. Targets: largest cut 100% in most wins, from 7 or more cells away, with gold ≥ 150 before it.

**Level 5 — An opening that feeds this middle game.**
Only now revisit the opening, judged by the middle game that follows. Experiments: train at the trainable cell nearest the enemy HQ (zasmu 74%, us 49%); a Voronoi or centre target at turn 7 to 10 instead of more purchases; a few candidate openings per map, chosen by turn 5 from the opponent's first moves (zasmu's own play does not depend on ours before turn 5). Targets: Voronoi share ≥ 0.55 and 2 or more centre cells at turn 10.

**Level 6 — Adapt to the opponent.**
With all of the above in place: recognise bankers (opponent gold rising over 2 turns with few actions) and wall early; against opponents without walls, strike earlier with fewer level 3 units; against walls, plan level 3 cuts. Targets: win rate against each of the top 5 in the batch, per opponent.

Levels 1 to 3 can run in any order among themselves; level 4 needs 1 and 3 (banking with an open front loses, as the [overconfidence study](2026-11-14-overconfidence.md) showed: our big losses came from weak positions and an opponent banking 54 gold against our 7); level 5 is last on purpose, since four opening experiments were already dropped.
