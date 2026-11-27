# Cell values, step 0: do they predict fights and the early race? (2026-11-27)

The owner asked, on 2026-11-27, for ways to value each cell to guide the search (flows from the HQ, a kernel on free cells), toward a plan of holding a large territory that is cheap to defend, with towers and few units late in the game. The proposal combines them in one value per cell, in gold: its income over the turns left, times its safety, where safety comes from the cell's **price**, the cheapest chain of trainings by which the enemy takes the cell or cuts it off from our HQ, against the enemy's gold. Step 0, measured here with no bot change, asks whether those numbers predict what happens in real games. The owner's emphasis: fights first, then the early game (where to send units, how fast to rush), and a caution that games may be short only because the bots are not good enough yet.

## Data and method

The 2,350 test games of `ice-and-fire-v032` against the top 10 of CodinGame's Legend (data branch `claude/codingame-games`, converted by `ice-and-fire-review codingame`: all 2,359 v032 logs agree with our engine; the 9 ranking games are left out). We are red in every test game, so red's numbers carry our play. The tool is `games/ice-and-fire/arena/examples/cells.rs`:

```sh
cargo run --release -p ice-and-fire-arena --example cells -- OUT.json RECORDS/*.json
```

For each ply, with A the player to act and D the other, and for each active cell c of D:

- **price with cuts**: the least of `Reach::new(state, A, true).cost` over c and over every cell of D whose loss cuts c off from D's HQ (A's movable units step first, as in E037);
- **direct price**: the same without cuts (what the evaluation's `threatened` counts, E023, E037);
- **steps**: walking steps from A's active land (a baseline);
- **margin**: price minus A's gold at the start of its ply (its income is paid by then);
- **lost**: c is no longer an active cell of D after A's ply (captured, or cut off).

For the early game, at red's starts of game turns 2 to 10, each neutral cell's **contest margin** m = (steps from blue's active land) − (steps from red's), and who holds it after game turn 12.

## How the games end

| | |
| --- | --- |
| Ended by an HQ capture | 2,282 of 2,350 (97%); the other 68 stopped early on CodinGame (no turn 100) |
| Game turns | median 20, quartiles 17 to 23, 90% by 27; none reach turn 100 |
| Against zasmu | 9% won, median 21 game turns |

The owner's caution stands for our own play, but these are games against the ten strongest players, zasmu included, and they end as early. At this level, a late game held by towers does not happen yet: the territory has to be made safe by game turns 12 to 20, and safe means first against the chain to the HQ and against cuts.

## Fights: the price with cuts predicts the cells lost

AUC: the chance that a cell lost at a ply ranks as more exposed than a cell kept (0.5 is chance, 1 perfect).

| Game turns | Attacker | Cells | Lost | Price with cuts | Direct price | Steps |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 7 to 12 | they | 444,214 | 3.0% | **0.935** | 0.922 | 0.842 |
| 7 to 12 | we | 399,658 | 4.2% | **0.934** | 0.915 | 0.849 |
| 13 to 20 | they | 649,927 | 7.3% | **0.888** | 0.874 | 0.759 |
| 13 to 20 | we | 553,423 | 13.4% | **0.854** | 0.822 | 0.743 |
| 21 to 40 | they | 189,342 | 10.0% | **0.875** | 0.868 | 0.746 |
| 21 to 40 | we | 180,789 | 14.1% | **0.868** | 0.845 | 0.750 |

No cell is lost before game turn 7. The cuts add 1 to 3 points of AUC everywhere, most where we attack (we cut more), and both prices are far ahead of distance.

**Cut off rather than taken.** Of the cells lost, 49% at game turns 7 to 12 and 62 to 65% from game turn 13 were cut off from their HQ, not captured. The evaluation's `threatened` term counts only the cells the enemy's chains enter; the shape term counts joints whatever the enemy can afford. Neither counts the cells behind an affordable cut.

**Calibration.** The share of cells lost at the ply, by margin, from game turn 7:

| Margin (gold) | Attacker: they | Attacker: we |
| --- | ---: | ---: |
| −100 or less | 34.8% | 44.4% |
| −90 to −50 | 26.9% | 40.1% |
| −40 to −10 | 17.2% | 27.9% |
| 0 to 30 (just out of reach) | 1.5% | 3.2% |
| 40 to 90 | 0.10% | 0.14% |
| 100 or more | 0.01% | 0.04% |

A cell the attacker cannot pay for is almost never lost, and one 40 gold beyond its gold is safe. A cell it can pay for is lost a fifth to two fifths of the time at that ply, the more often the cheaper it is: the attacker plays one chain or cut, not all of them. E023's count of 0.75 for a threatened cell (a 25% loss) is about right on average; the margin tells the cells lost a third of the time from those lost a sixth of the time. The safety of the proposal would be about 0.6 to 0.85 below a margin of 0, about 0.97 from 0 to 30 and 1 beyond.

## The early game: the contest margin

The share of neutral cells that red (we) and blue hold after game turn 12, by the contest margin m at a given game turn (positive: nearer red's land):

| m | turn 4: red / blue | turn 6 | turn 8 | turn 10 |
| ---: | ---: | ---: | ---: | ---: |
| −2 | 27% / 37% | 32% / 40% | 13% / 47% | 5% / 33% |
| −1 | 39% / 35% | 34% / 28% | 26% / 36% | 17% / 28% |
| 0 | 39% / 27% | 52% / 20% | 46% / 24% | 34% / 17% |
| +1 | 55% / 19% | 49% / 15% | 57% / 12% | 42% / 9% |
| +2 | 50% / 17% | 64% / 9% | 58% / 7% | 46% / 3% |

The rest stays neutral (a third to a half by game turn 10, often pockets inside a side's land). Up to game turn 6 the margin decides little: a cell 2 steps nearer blue still ends red's 27 to 32% of the time, as the race is still open. From game turn 8 it sets the owner on blue's side (m ≤ −2: blue's 3 to 15 times as often), less on ours, since many cells near our land stay neutral pockets. The cells at m between −1 and +1 are where a unit sent or a training changes the owner; those are the ones a kernel on the margin would weigh most.

What wins: the difference after game turn 12, against the winner (AUC over the games):

| Difference after game turn 12 | Ahead: our score | Behind: our score | AUC |
| --- | ---: | ---: | ---: |
| Active cells | 0.88 (1,240 games) | 0.38 (1,025) | **0.840** |
| Cells held among the contested of game turn 4 (\|m\| ≤ 1) | 0.75 (1,596) | 0.37 (525) | 0.734 |
| Income | 0.79 (1,203) | 0.50 (1,067) | 0.713 |

All the land at game turn 12 tells the winner better than the contested cells alone or income. Against zasmu the contested cells are level (5.0 to 4.9 a game); against the others we hold more (for example 7.2 to 3.8 against reCurse). So zasmu's edge is not the contested band of game turn 4, as the frontline study found for the land (2026-11-20).

## What follows

1. **The price with cuts is a good fight value**, better than what the evaluation counts. The smallest experiment: the `threatened` cells of E023 and E037 become those whose price with cuts is within the enemy's gold, the cells behind an affordable cut included, at the same count of `THREAT_QUARTERS`. A second one weighs them by their margin, as in the table above. Each is one experiment against v032.
2. **The fortress plan does not apply yet at this level**: no game reaches turn 30 in a tenth of the games. Its parts that do apply now are the defence by price (towers where they raise the price of the cheapest chain and of the worst cut the most) during game turns 12 to 20.
3. **The early game**: the contest margin sets the owner from game turn 8 only, and the land at game turn 12 predicts the winner best. Allocating units by the margin (to the cells at |m| ≤ 1 from game turns 4 to 8) is a candidate for the opening's goals, to compare with E073's band and E080's turn-1 plan; it needs a measure of where our units went against those cells, which this study does not make.

The data are not committed (they rebuild from the data branch in a second).
