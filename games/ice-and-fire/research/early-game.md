# A Code of Ice and Fire: the early game

Research for experiment E003, asked by the owner on 2026-10-22 after a CodinGame replay of `ice-and-fire-v002` (as blue: 3 gold and an income of −9 at turn 7, a thin line of units, then the army wiped out by the debt rule): "think about strategies for the early game ... The goal is to maximize the expansion to generate the most income, while also being safe when the middle game begins. You should consider several approaches from a theoretical point of view, then benchmark them. Once best candidates are selected you can use them as part of a full bot, and then you can pick the best."

The early game here is the **opening**: from the first turn until the two territories meet. Stage 8's research ([README.md](README.md)) covers the rest of the game; this note keeps to the opening and to the hand-over to the middle game.

## Sources

| Source | What was taken from it |
| --- | --- |
| [MSmits, 17th](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/21) | Beam search of the first 12 turns in the first turn's 1,000 ms (width 5,000), stopping once half the map is taken; it maximised cells owned and nearness to the enemy HQ, which "sort of has the effect of the bot trying to get as far out as possible", and he believed it reached the highest income possible; the territory was sometimes "messy, hard to defend"; replanning each turn with a Voronoi term would have answered the opponent |
| [NinjaDoggy, 3rd](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/45) | The opening lasts while the nearest units of the two players are at least 3 cells apart; a beam search moves the units, trains as many as it can, then simulates 3 more turns of moves; it ranks states by the difference of Voronoi territory, then cells owned, then the units' distance to the enemy HQ |
| [Magus, about 15th](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/3) | A Monte Carlo search over 10 actions (a move or a training) that maximises cells owned, until the enemy's territory can be reached |
| [reCurse, 1st](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/30) | No opening of its own: the plan search with terms that "favor early spread and territory grab" (capturable cells per unit) and early aggression (distance to the front line); no level 2 or 3 outside chains, since upkeep cripples "the most precious game resource: gold income"; the weight of gold and income doubled after some turn |
| [k4ng0u, 25th](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/41) | The gold boss's rule (each move and training to the cell nearest the enemy HQ, with tie-breaks that spread the units) gave "fast expansion to the center while spreading territory", worth 20 to 30 ranks; towers only where a border cell is in danger, the rest spent on expansion ("game changer"); mines nearest the HQ with leftover gold when the income is below the opponent's |
| [eulerscheZahl, 4th](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/5) | Moves as an assignment of units to cells (Hungarian method); early trainings as near the enemy base as possible, then expansion of the own territory once the opponent is close; mines with leftover gold, far from the opponent, of doubtful use |
| [teccles, 2nd](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/25) | His units moved greedily (enemy cells, then neutral cells, then cells that do not block allies, near the enemy HQ, not next to friends), and "filling territory near-optimally shouldn't be too hard": the opening was the part he would have done better |
| [Nerchio, WinterThrone 2020](https://forum.codingame.com/t/debriefing-winterthrone2020-code-of-ice-and-fire/186956) | A fixed first turn ("build-order"); units with only one cell left to take move first; reaching the centre fast; banking from about turn 8, when the armies meet in the middle; groups of units and connected territory, which are hard to cut |
| [a1k0n, Google AI Challenge 2010 (Tron)](https://www.a1k0n.net/2010/03/04/google-ai-postmortem.html) and [Janzert's Tron bots](https://janzert.com/tron/) | Voronoi territory, each cell to the player who reaches it first, as the evaluation of a race for space |
| [Artificial Generals Intelligence, 2025](https://arxiv.org/abs/2507.06825) | In generals.io, territory is the economy (every cell grows the army), the same compounding as here; not used further |

No source gives numbers for an opening's economy: they are worked out below.

## The economy of the opening

From [RULES.md](../RULES.md): income is +1 per active cell, +4 per mine on an active cell, minus 1 per level 1 unit (4 per level 2, 20 per level 3). A unit costs 10 gold, a mine or tower 15. Each player starts with 20 gold; red owns 1 cell, blue 2.

Before the armies meet only level 1 units and mines matter. With C active cells, U level 1 units and M mines, the income is I = C + 4M − U, and:

| Action | Gold | Change of income |
| --- | ---: | ---: |
| A unit moves onto a neutral cell | 0 | +1 for the rest of the game |
| A unit moves onto an own cell | 0 | 0 |
| Train a unit on a neutral cell | −10 | 0 (its cell +1, its upkeep −1) |
| Build a mine on an own empty mine spot | −15 | +4 |
| Build a tower | −15 | 0 |

So **trainings do not raise income; moves do.** A unit pays back through the cells it walks onto afterwards. Measured in gold collected by a horizon H turns later:

- a **mine** built now returns 4H − 15: it pays back after 4 turns;
- a **unit** trained now that takes a new cell on each of its next k turns returns about kH − k(k+1)/2 − 10; when it stops at the horizon (k = H − 1) that is H(H − 1)/2 − 10, which pays back after 5 turns. A unit that finds no free cell returns −10, and −1 a turn after that.

Hence three facts:

1. **Every unit should take a new cell every turn** while free cells are near: it is free income. The units must not block each other, and those inside the territory must walk to where free cells are.
2. **Trainings pay only while free cells remain for them.** Each side's half of the map (cells nearer its HQ than the other) holds about 49 cells, out of 106 (2,000 source maps, `--maps`). A unit trained 4 turns or fewer before its side is full never pays back.
3. **Mines pay back faster than units** (4 turns against 5) and need no free cells, but they take no territory, and territory the player does not take, the enemy takes. The mine spot next to each HQ ((1, 0) for red) is the safest mine of the game; the next spot on a side is about 5 steps from the HQ (4.8 spots per side on average).

An idealized model (one free cell per unit per turn until the side's 49 cells are taken, no enemy, mines on the spot by the HQ from turn 2 and on a second spot from turn 6) gives, at the start of each turn:

| Policy | Turn 9: gold, cells, income | Turn 15: gold, cells, income |
| --- | --- | --- |
| Train whenever gold allows | 4, 37, 28 | 187, 49, 37 |
| The same, two mines first | 1, 30, 30 | 223, 49, 46 |
| The same, mines with what is left | 4, 37, 28 | 189, 49, 45 |
| Each gold use only when it pays back by turn 15, units before mines | 8, 32, 34 | 281, 49, 51 |

Training whenever gold allows fills the side by turn 10, then leaves 12 units that cost 12 a turn and earn nothing. Stopping the trainings when they no longer pay back, and building mines, gives about 50% more gold by turn 15 in this model. The model overstates every policy: units walk to free cells and the enemy contests the middle, so the real figures are lower and the trade between cells now and income later is measured below.

What v002 does (100 pairs against v001 at CodinGame's limits, the benchmark below): its evaluation pays +10 for each step that brings the chain to the enemy HQ closer, against +4 for a training and +7 for a move onto a free cell, and −3 for a mine. So it never builds a mine (it trains on the mine spot next to its HQ on turn 1), and sends a line of units along the edge of the map toward the enemy HQ: at contact, about turn 8.8, it has 23.5 cells and an income of 17.4; v001, which builds mines, has 18.1 cells and an income of 24.4. At game turn 12 of those games v002 has 30.7 cells, an income of 23.4 and 29.4 gold, where the idealized model reaches 49 cells, 37 and 76.

## Safety when the middle game begins

Games between strong bots end by a chain of trainings to the HQ after both sides bank gold (top games lasted 17.5 turns on average, research README). So what the opening leaves matters in four ways:

- **Income at contact** decides who first affords the decisive chain (Magus's "time to win"): the opening's main product.
- **Shape.** A thin line is cut by one capture: the cells beyond stop paying and its units die. Cuts are worth their cells and unit value (the tactics' cut finder). A compact territory, with units side by side on the front, is costly to cut.
- **Units on the front** raise the enemy's chain cost: a level 1 unit on a cell makes it cost 20 instead of 10 to take.
- **Solvency.** At its start of turn a player whose gold plus income is below 0 loses all its units (RULES.md). Level 2 and 3 units (upkeep 4 and 20) are what makes the income negative, as in the owner's replay. A plan must end with gold + income ≥ 0, with a margin for a cut that removes income.

## Approaches

Each is an opening module of the `plan` bot, played while the territories are more than 3 cells apart (the hand-over below); the plan search plays from contact on, as in v002.

| Name | Idea | Expected | Cost |
| --- | --- | --- | --- |
| `plan` (v002) | The hill climb with its static evaluation | The baseline: cells along one line, no mines | None |
| `rush` | The gold boss's rule (k4ng0u): each unit to the neighbour nearest the enemy HQ, each training on the free cell next to the territory nearest the enemy HQ, as gold allows | Early contact, a big share of the middle, a thin territory; v002's shape without its waste | Low |
| `fill` | Free cells first: units, those with the fewest options first, each to a free neighbour that no other unit takes, preferring cells with more free cells around them and contested cells (nearer the enemy); units with no free neighbour walk to the nearest free cell; trainings on free cells next to the territory, contested first, while the side's free cells are more than k per unit | The most cells per turn, a compact territory; contact later | Medium |
| `econ` | `fill`, plus mines on the side's own spots, nearest the HQ first, when a mine's return to the horizon beats a unit's | The highest income at contact, fewer cells | Low once `fill` exists |
| `rollout` | A search over the turn (after MSmits' and NinjaDoggy's beams, in the cheaper form of a rollout): up to 30 variants of the `econ` turn (fewer purchases, mines or not, more or less weight on contested cells), each played on with `econ` for both sides until contact, the one whose end is worth most kept (gold plus income to the horizon, ours minus theirs); within the turn's time | No worse than `econ` if the value of the end is right, since `econ` is among the variants | Medium |

On top of the best opening, the safety rules:

- **Debt guard**, in every phase: the evaluation counts a player whose gold plus income is below 0 as having lost its units (v002's flaw).
- **Reserve at contact**: the last turns before contact keep gold for the first exchanges (a tower is 15, a level 2 unit 20).
- **Hand-over distance**: the opening ends at a gap of 3 (the plan search sees the first contact turn) or more.

## Benchmark

Measured in E003 ([journal entry](../journal/E003-opening.md), all the tables): each opening inside the `plan` bot, at CodinGame's limits, against v002 and in a round robin of 200 pairs per pairing, with the opening benchmark (`cargo run --release -p ice-and-fire-arena --example opening`, [README](../README.md#arena)) reading the games. Ratings of the round robin, v002 at 0:

| Opening | Elo | At contact: cells, income, gold, mines | The opponent's best cut after it: cells, unit value |
| --- | ---: | --- | --- |
| `econ` | +255.7 ± 21.6 | 20.3, 20.0, 30.5, 0.89 | 1.0, 9.5 |
| `econ` with a reserve of 15 | +245.8 | 20.5, 18.9, 32.1, 0.60 | 1.1, 9.5 |
| `rollout` | +230.1 | 19.8, 21.0, 29.6, 1.22 | 1.0, 9.6 |
| `fill` | +223.9 | 21.1, 16.0, 19.9, 0 | 1.0, 8.0 |
| `rush` | +143.2 | 20.4, 15.4, 20.4, 0 | 1.0, 7.7 |
| `plan` (v002) | 0 | 18.7, 13.8, 18.6, 0 | 1.2, 8.3 |

What the theory got right: every unit taking a free cell each turn is most of the gain (`fill` alone is worth +224); mines pay once a unit is valued only for the cells it takes before contact, and the one next to the HQ gets built in most games; a thin line toward the enemy HQ (v002, `rush`) is the weakest shape. What it did not foresee: the trainings' stopping rule never binds, since the territories meet around turn 8 with free cells still on each side; and a search for the most gold and income at contact (`rollout`) did a little worse than the rule it searched over, by giving up cells and units. The reserve, the debt check and the hand-over gap made no measured difference; the debt check stays, since it fixes the wipe-out of the owner's replay at no cost. The candidate is `econ`: `ice-and-fire-v003`.

## The middle-game score (E004)

E004 compares openings without playing whole games: the owner asked (2026-10-22) to "benchmark them only in the early game: consider we reached the middle game when one player can reach the opponent territory", with a score "based on heuristics that you should determine carefully", never fitted to games. The crate [`ice-and-fire-opening`](../opening/) holds the openings, the middle game and the score; its harness (`cargo run --release -p ice-and-fire-opening --example early`) plays openings against each other, in process, from both seats of many source maps, and scores both sides there.

- **The middle game** starts at the first start of a player's turn whose gold pays a chain of trainings to an enemy cell, its units' steps counted as free (`middle::reached`). With E003's openings it comes at game turn 7.5 to 8.
- **The score** (`score::score`) of a seat is its value less the enemy's, in gold at game turn 20, with L = max(1, 20 − turn): gold + L × income + the free cells it can still take (nearer its territory than the enemy's, a tie half; the k-th, at distance d, taken on turn max(d, k) and then earning L − that) − its exposure − the value of its units if it cannot pay its upkeep − 1,000 if the enemy can pay a chain to its HQ (its exposure then counts 0, so that the loss counts once). The **exposure** is the most the enemy could take after banking 1, 2 or 3 turns while the side only builds towers: the cut finder's worst cut for it (up to 4 + j trainings within the enemy's gold), after up to j towers placed greedily against that cut, costing the cells × (L − j), the units' value, 4 (L − j) per mine and 15 per tower built.

**The check against games.** The score must rank E003's openings in the order their games did (`econ` > `fill` > `rush` > v002). Each of them played v003 (`econ`) at CodinGame's limits, 200 pairs on seed 31, and the arena's opening benchmark scored both sides at the middle game of every game:

| Opening against v003 | Elo in the games | Score as first written | Score with one cell a turn | Its sign agrees with the winner |
| --- | ---: | ---: | ---: | ---: |
| `fill` | −15.6 [−46.5, +14.9] | +42.5 | −15.5 | 55.0% |
| `rush` | −135.0 [−171.9, −100.7] | −10.0 | −56.1 | 67.0% |
| v002 | −246.3 [−296.5, −203.6] | −45.9 | −81.8 | 78.7% |

As first written, with n units taking n cells a turn (the k-th on turn max(d, ceil(k / n))), the territory term put `fill`, which buys more units and no mine, 42 gold above `econ`, in games that `fill` lost. Of the terms, territory was the largest difference (+52 gold for `fill`) and tracked the result least (correlation +0.05 with the game's result, against +0.17 for gold, +0.18 for income and −0.17 for exposure). Units take free cells until the territories meet; from the middle game on they fight, and the plan search does not send them back to fill. With one cell a turn, whatever the units, the order matches the games, and the sign of the score agrees with the winner in 67% of these 1,200 games, games scored 0 aside (62% before). Looking 1 or 2 turns ahead in the exposure, instead of 3, changed the means by less than 7 gold and was kept at 3.

In the harness, over 1,000 maps from both seats (scores of the row against the column, ± the 95% interval near 2):

| Ours | `econ` | `fill` | `rush` |
| --- | ---: | ---: | ---: |
| `econ` | 0 | +32.9 | +70.2 |
| `fill` | −32.9 | 0 | +28.8 |
| `rush` | −70.2 | −28.8 | 0 |

The order is the same at horizons 15 and 25 (`econ` over `fill` +30.3 and +35.3, over `rush` +54.6 and +86.4).
