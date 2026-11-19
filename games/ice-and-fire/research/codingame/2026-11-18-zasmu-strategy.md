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

## Long-term plan: a curriculum of capabilities

*Rewritten on 2026-11-19 at the owner's request ("Rewrite the plan", form). The first version was a list of levels by game phase, each reached by new evaluation terms screened against our last release. That is the method of E003 to E067: hill-climbing in parameter space, judged by our own previous bot, which can only find what beats ourselves. The owner asked for a curriculum precisely to avoid such a local optimum. Its levels 0 and 1 were done before the rewrite: level 0 is the command `codingame-style` (#347), and level 1 is E068 (`ice-and-fire-v028`, the tower wall as an evaluation term, +41.9 over v027, #349).*

### Principles

- **A stage adds a capability, not a weight.** It is something the bot could not do before: plan over several turns, save toward a goal, defend against a threat that is still two turns away. A stage builds on the ones before it, and each later stage uses the earlier ones' machinery.
- **Each capability has its own test, judged outside self-play.** A stage ends when its capability passes its tests and holds up against strong opponents, not when an SPRT against our last release accepts. That SPRT stays, as a check that nothing regressed.
- **Weights come last.** Within a stage the parameters get sensible values only. They are tuned once its capability works, so that tuning does not lock in the structure before it is complete.
- **Measure the behaviour, not only the result.** Each candidate's games are read with `codingame-style` (and the capability tests), to check that the behaviour aimed at appears. Example: does v028 actually build a wall?

### Stage A: an outside judge

Before new capabilities, a way to see them and to judge them without self-play.

1. **Capability tests.** Positions taken from the collected games, each with one decision that a capability decides. Examples:
   - our chain to the enemy HQ is affordable in 2 turns if we save, and not if we spend;
   - the opponent is saving and its chain to our HQ will be affordable in 2 turns: defend now;
   - 2 towers hold a front that units cannot.

   Each test checks a property of the answer, not one exact move. Examples: gold kept at or above a level; towers that raise the opponent's chain cost by a given amount; the HQ taken within k turns in a scripted continuation. A bot's score is its pass rate per capability, reported by a command of the review tool. zasmu's own decisions in those positions are the reference, where it played them.
2. **A sparring bot in zasmu's style.** A baseline bot built by hand from the study's numbers: hold the front with towers, mines on safe spots, save gold toward the cheapest chain to the enemy HQ, strike when it is affordable. It joins the screenings and the league, so that self-play includes the style that beats us. Its first version is simple; it improves as stages B to E give it better parts.
3. **Test batches against the top players.** At each stage's end, a batch of CodinGame test games against zasmu and the top 5 on the fixed maps (`scripts/codingame.sh batch --opponents`), on the owner's approval. It is read with `codingame-style` and compared with the release before.

Possibly an ADR: the judge of a stage changes from the SPRT alone to the SPRT plus the capability tests and the batch.

### Stage B: a strategic layer; first goal, the strike

A plan over several turns above the existing search. Each turn it picks a goal and constraints, and the search plays the turn within them: a gold reserve, towers it must build, cells it must take or keep.

The first goal is the strike: save toward our cheapest chain to the enemy HQ when gold plus k incomes reach its cost (k ≤ 3), keep that reserve, then play the chain (the existing lethal finder). It comes first because its success is exact and measurable:
- the capability tests for saving and striking;
- in the games, the share of wins by one cut of the whole enemy land, from 7 or more cells away (zasmu: 100%, from 9).

### Stage C: defend against a strike

The same machinery from the other side. Follow the opponent's gold plus k incomes against its chain to our HQ, and when a strike comes within reach, set a goal that raises that chain's cost (towers on its path, units that block it) before the turn it becomes affordable. Measured by:
- the capability tests for defence;
- in the zasmu batch, the losses to one cut of our whole land (now most of them).

### Stage D: hold the front

The tower wall as a goal of the strategic layer, rather than a weight. The frontier cells to guard, chosen for the strike (B) and the defence (C) they allow, and units released from the front. E068's term becomes one of the tools. Measured by the frontier cells guarded (zasmu 75 to 85%), the upkeep share (18%) and the units lost per reply (7 to 8).

### Stage E: an economy behind the wall

Mines as a goal behind the held front: which spots, when, given the strike's reserve. Measured by mines on the board and the share lost (zasmu 3.4 at turn 20, 6% lost), and by gross income at turn 20.

### Stage F: an opening toward a target position

The opening picks a target position at turn 10 (land, centre and chokepoints, units placed to start the wall), chosen for the middle game that the strategic layer plays from it. It is no longer a rule set judged on its own. The study's opening measures are the check: the Voronoi share, the centre cells, and trainings at the most advanced cell.

### Stage G: choosing goals per opponent

Recognise the opponent's style from its first turns and its gold (a saver, a wall, an attacker) and weigh the goals accordingly. Measured by the win rate against each of the top 5 in the batch.

### Then tuning

Once a stage's capability passes its tests, its parameters are tuned (screenings, the SPRT), and the stage's last candidate is checked against the top players.
