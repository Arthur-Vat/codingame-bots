# The opening against the top 10: where the units and the land go (S0)

Stage S0 of the goal-directed opening (O1), asked by the owner on 2026-11-14 ("Ok, start with S0") after the review of O1 in the conversation. S0 changes no bot. It asks two questions before a new opening planner is built:

1. Where do we lose land to the top 10 in the first 12 turns: in how many cells we take, in the order we take them, or in their direction?
2. Do our units get in each other's way, or waste turns in other ways, and do the top 10's units do better?

A third question is whether the opening's current search would do better with more time (the headroom).

**In short:** our units do not get in each other's way (0% to 3% of unit turns, as for the top 10); up to turn 7 our opening matches the top 10's in cells and in unit use; more search time changes nothing. The gap is the race for the middle of the map from turn 7 to turn 12: alone, our opening reaches the middle as the top 10 do, but against them it arrives second, and after contact more of our units stand one step behind the land. The leader, zasmu, wins that race earlier still: 2.4 middle cells at turn 8 as red, against the top 10's 1.3 and our 0.6.

Written by Claude from the outputs of the commands below.

## Data and commands

- **Games:** every game on the branch `claude/codingame-games` at commit 063a143: 4,668 games, 4,666 read (2 left out: our engine disagrees with CodinGame on a refused command, as known). The comparisons below use v024 to v026 (2026-11-14) and the top 10 of the leaderboard snapshot of 2026-11-14T16:12Z.
- **Study:** `ice-and-fire-review codingame-opening --data DIR --me Tarleya [--release ice-and-fire-v026]` (README, "Review"). With `--release`, the top 10's rows come from the same games as ours, so both sides played each other. Most of the top 10's games in the whole data are against v014 (2,854 test games as red), a weaker opponent than v026; the tables below use the same-game rows.
- **Headroom:** `cargo run --release -p ice-and-fire-opening --example headroom -- --maps 100 --ms 608,2400,9600 --jobs 3`: v026's opening alone on source maps, the opponent passing.

Terms. A side's **home** is the cells at least 3 steps nearer its HQ than the enemy HQ (steps over non-void cells, from the map alone); the **middle** is the cells within 2 steps of even (about 25 a map); the rest is **far**. A **unit turn** is a turn of a unit that may move. A unit turn takes a neutral cell, takes an enemy cell, or takes nothing. A unit that takes nothing is *beaten* when it had a neutral neighbour but our other commands of the turn took them all, *free* when a neutral neighbour was left, *free by the enemy* when every such neighbour touches an enemy cell, and *enclosed* when it had no neutral neighbour (*near* when one is 2 steps away).

## 1. Turns 1 to 7: no gap, no wasted units

Up to turn 7, our opening and the top 10's do the same thing:

- Active cells at turn 6: 12.7 for v026 as red, 12.8 for the top 10; 14.9 and 14.7 as blue.
- 91% to 100% of unit turns take a neutral cell at every turn from 2 to 7, for us and for them.
- **Units in each other's way: 0% to 1% of unit turns** (*beaten*), for us and for them. Units do not collide in the opening.
- Trainings up to turn 4 take 4.65 cells in their next 5 turns (top 10: 4.76 to 4.81).

The coupling between our units, which a path planner would have to manage, costs almost nothing here: the current rules (the unit with fewest options moves first, each to a different cell, a penalty for crowding) already handle it.

## 2. Turns 8 to 12: the gap opens, in the middle

Same games, our side and the top 10's at the start of their turn (means; plies in brackets):

| Seat | Who | Turn | Active cells | Home held | Middle held | Middle held by the enemy |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| red | v026 (96) | 8 | 20.4 | 19.9 | 0.6 | 3.0 |
| red | top 10 (22) | 8 | 21.6 | 20.0 | 1.6 | 2.3 |
| red | v026 | 10 | 27.4 | 24.5 | 3.2 | 7.7 |
| red | top 10 | 10 | 29.2 | 24.3 | 5.2 | 5.7 |
| red | v026 | 12 | 31.3 | 26.9 | 4.3 | 11.1 |
| red | top 10 | 12 | 35.1 | 27.3 | 8.1 | 7.6 |
| blue | v026 (27) | 8 | 23.1 | 21.3 | 2.2 | 3.4 |
| blue | top 10 (86) | 8 | 24.1 | 21.4 | 2.9 | 2.1 |
| blue | v026 | 10 | 29.6 | 24.1 | 5.3 | 7.2 |
| blue | top 10 | 10 | 32.0 | 24.9 | 7.4 | 4.4 |
| blue | v026 | 12 | 31.2 | 25.7 | 6.8 | 10.0 |
| blue | top 10 | 12 | 37.2 | 27.0 | 10.4 | 5.5 |

v024 and v025 against the top 10 give the same picture (middle held at turn 12: 3.8 to 6.3 for us, 6.3 to 10.6 for them).

- **The home is level**: we hold as much of our own side as the top 10 do at every turn.
- **The middle is not.** By turn 8, the last turn of our planned opening, the top 10 already hold more middle cells than we do (red 1.6 against 0.6, blue 2.9 against 2.2). By turn 12 they hold 1.5 to 1.9 times as many (8.1 against 4.3 as red, 10.4 against 6.8 as blue), and the enemy holds more of the middle against us than it does against them.
- So the gap is in **direction and timing**, not in count: we fill our home as well as they do, but they reach the middle first and keep winning it after contact.

## 3. Where our unit turns go after contact

Shares of unit turns, v026's games (the top 10's rows from the same games: 22 as red, 86 as blue):

| Seat | Who | Turn | Neutral | Enemy | Beaten | Free | Free by the enemy | Enclosed near | Enclosed far |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| red | v026 | 8 | 78% | 0% | 2% | 3% | 3% | 10% | 4% |
| red | top 10 | 8 | 81% | 10% | 1% | 4% | 0% | 3% | 1% |
| red | v026 | 9 | 58% | 5% | 3% | 8% | 2% | 19% | 6% |
| red | top 10 | 9 | 70% | 3% | 2% | 3% | 1% | 15% | 6% |
| red | v026 | 10 | 41% | 3% | 3% | 10% | 2% | 28% | 13% |
| red | top 10 | 10 | 56% | 10% | 3% | 7% | 2% | 14% | 9% |
| blue | v026 | 8 | 72% | 0% | 1% | 6% | 2% | 17% | 2% |
| blue | top 10 | 8 | 82% | 1% | 1% | 5% | 0% | 7% | 3% |
| blue | v026 | 9 | 52% | 9% | 2% | 9% | 0% | 20% | 8% |
| blue | top 10 | 9 | 70% | 2% | 3% | 5% | 1% | 14% | 5% |
| blue | v026 | 10 | 39% | 13% | 1% | 8% | 1% | 21% | 16% |
| blue | top 10 | 10 | 55% | 8% | 2% | 5% | 1% | 18% | 11% |

- By turn 10 v026's units spend 6.4 unit turns a game taking nothing (20% to 21% of their turns), the top 10's 4.0 as red (13%) and 5.2 as blue (15%) in the same games. v024 and v025: 5.4 to 7.5 (18% to 23%).
- The difference is in two kinds of idle turns, both after turn 7. **Free:** at turns 9 and 10, 8% to 10% of our unit turns stand next to neutral land and do not take it, against 3% to 7% for the top 10 (4% to 5% over the whole data). In most of them the land is not next to the enemy (*free by the enemy* is 0% to 3%). The leader does it as often as we do (section 6), so this may be a choice, such as holding a front, rather than a waste. **Enclosed near:** at turns 9 and 10, 19% to 28% against 14% to 18%: units one step from land they do not reach.
- *Beaten* stays at 1% to 3% for both: even after contact, our units seldom lose their cell to one another.

## 4. Trainings

Cells taken by a trained unit in its next 5 turns (games that lasted that long; the top 10 in v026's games):

| Seat | Who | Trained at turns 5 to 7 | None taken | Trained at turns 8 to 10 | None taken |
| --- | --- | ---: | ---: | ---: | ---: |
| red | v026 | 2.32 | 3% | 1.02 | 48% |
| red | top 10 | 2.55 | 2% | 1.37 | 49% |
| blue | v026 | 2.33 | 2% | 0.85 | 54% |
| blue | top 10 | 2.94 | 7% | 1.47 | 37% |

As blue, the top 10's trainings of turns 5 to 7 take 0.6 more cells each: they are placed where land is left for them. After contact about half of everyone's trainings take no cell (they guard or fight); ours take fewer cells, by 0.35 as red and 0.6 as blue.

## 5. Mines

v026 builds a mine by turn 8 in 54% of its games as red (top 10: 23%) and by turn 10 in 94% (55%); as blue 37% and 74% (31% and 52%). Since E058 our first mine comes earlier than the top 10's, at 15 gold, 1.5 units, in the turns when the middle is decided.

## 6. The leader, zasmu

zasmu, first on the snapshot, as a group of its own (`--top 1`). It played few games against v024 to v026 (3 to 9 a release and seat), so its rows come from all its games in the data (29 as red, 352 as blue), most of them against v014; the top 10's rows here come from the whole data too, for the same reason, and v026's from its own games. Columns *middle lost* depend on the opponent and are left out.

| Seat | Who | Turn | Active cells | Home held | Middle held | Mine built so far |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| red | zasmu | 8 | 22.1 | 19.8 | 2.4 | 14% |
| red | top 10 | 8 | 21.2 | 20.0 | 1.3 | 40% |
| red | v026 | 8 | 20.4 | 19.9 | 0.6 | 54% |
| red | zasmu | 12 | 35.9 | 27.2 | 9.0 | 83% |
| red | top 10 | 12 | 33.0 | 26.4 | 7.2 | 73% |
| red | v026 | 12 | 31.3 | 26.9 | 4.3 | 97% |
| blue | zasmu | 8 | 24.6 | 20.8 | 3.9 | 34% |
| blue | top 10 | 8 | 24.5 | 21.2 | 3.4 | 31% |
| blue | v026 | 8 | 23.1 | 21.3 | 2.2 | 37% |
| blue | zasmu | 12 | 41.2 | 27.9 | 11.3 | 91% |
| blue | top 10 | 12 | 38.5 | 26.7 | 10.6 | 66% |
| blue | v026 | 12 | 31.2 | 25.7 | 6.8 | 88% |

- **zasmu wins the middle earlier still.** At turn 8, the end of the opening, it holds 2.4 middle cells as red (top 10 1.3, v026 0.6) and 3.9 as blue (3.4, 2.2), with the same home as everyone. By turn 12 it holds 0.7 to 1.8 middle cells more than the top 10 and 4.5 to 4.7 more than v026 (2.7 to 2.9 active cells more than the top 10, 4.6 to 10 more than v026).
- **Its units are not more efficient than ours in the way section 3 measures.** Unit turns idle by turn 10: 3.8 a game as red (12%), 4.7 as blue (13%), a little below the top 10 and v026. But it leaves units next to quiet free land as often as v026 does (*free*: 11% to 12% at turns 9 to 11 as red, 8% as blue). Where it differs is *enclosed near*: 12% to 18% as red and 15% to 21% as blue at turns 9 to 11, against v026's 19% to 28%. Its units stay at the edge of the land, not one step behind it.
- **Its trainings take more land**: those of turns 5 to 7 take 2.48 cells each as red and 3.03 as blue (v026 2.32 and 2.33), those of turns 8 to 10 1.31 and 1.32 (v026 1.02 and 0.85).
- **Its first mine is late as red** (14% by turn 8, against v026's 54%) and in between as blue.

## 7. Headroom of the opening's search

v026's opening played alone on source maps 1 to 100 (the opponent passing), its first plan searched for the bot's own 608 ms or 4 and 16 times longer, measured at the start of turn 8 (means over 100 sides; the last column is the gain in the plans' objective over 608 ms, paired by map, with its 95% interval). *Rules* is the plans' final step alone (`solo::REST`), with no search.

| Seat | First plan | Cells | Income | Gold | Middle held | Objective | Over 608 ms |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| red | rules | 19.5 | 14.4 | 17.6 | 0.3 | 58.7 | |
| red | 608 ms | 22.1 | 16.5 | 19.4 | 1.7 | 102.3 | |
| red | 2,400 ms | 22.2 | 16.5 | 18.9 | 1.8 | 101.8 | −0.5 ± 0.6 |
| red | 9,600 ms | 22.2 | 16.5 | 18.8 | 1.8 | 99.8 | −2.5 ± 1.3 |
| blue | rules | 24.4 | 18.5 | 25.7 | 1.9 | 95.0 | |
| blue | 608 ms | 25.1 | 19.2 | 28.4 | 3.7 | 136.3 | |
| blue | 2,400 ms | 25.2 | 19.4 | 28.2 | 3.8 | 137.3 | +1.0 ± 1.0 |
| blue | 9,600 ms | 25.3 | 19.4 | 28.5 | 4.0 | 137.3 | +0.9 ± 1.3 |

- **The search over weight sets is saturated.** Sixteen times more time changes the position at turn 8 by at most a tenth of an active cell and 0.3 of a middle cell. (The objective can fall with more time because the plans are rated at the end of their 9 turns, not at turn 8.) Searching harder within the 27 weight sets and 8 ways of buying has nothing left to give.
- **The search itself is worth a lot**: over the rules alone, +2.6 cells and +1.4 middle cells as red, and about +40 on the objective in both seats.
- **Alone, our opening reaches the middle as the top 10 do in real games** (red 1.7 middle cells at turn 8, the top 10 1.6; blue 3.7 against 2.9). In real games against the top 10 it holds 0.6 and 2.2. The plan is made as if the opponent passed, and in real games the opponent takes part of that middle first (it holds 3.0 middle cells by our turn 8 as red, 3.4 as blue). An opening that knows when the enemy will arrive would choose other cells.

## What follows for O1

- **The coupling of units is not where we lose.** Units in each other's way cost 0% to 3% of unit turns, for us as for the top 10. A planner that makes units' paths jointly would fix a waste we hardly have.
- **The opening up to turn 7 matches the top 10's in cells and unit use, and its search is saturated.** The difference at turn 8 is where the land is: about one middle cell, which our plan aims at but the enemy takes first.
- **The gap is the race for the middle, from turn 7 to turn 12**, and the leader shows it most. It spans the end of the opening and the first turns of the middle game. In those turns more of our units stand one step behind the land (enclosed near), our trainings take less land, and our mines come earlier; the leader's units stay at the edge, its trainings take the most land, and its first mine as red is the latest.

So S1 as proposed (a path planner for the opening, scored by today's objective) aims at the part that already works. A goal-directed plan still fits, but its goal is the middle at turns 8 to 12, with the enemy's arrival times, not more cells in our home by turn 8. The proposals that follow from the data:

1. **S2 first, inside the current opening** (medium): count each middle cell by when we reach it against when the enemy can (a deadline: its arrival time with the enemy's opening mirrored, S3), so that the opening's plans head for the middle cells they can still win before turn 8. The pull toward the enemy HQ (`reach`) and the Voronoi weight did not do this (E034, E053); a deadline measures the race itself.
2. **The hand-over** (small to medium): after contact our units stand one step behind the land more often than the leader's. A study of those plies with the judge (`codingame-judge`) would say whether the middle game's evaluation or its climb keeps them there.
3. **Mines and the middle** (small): a screening of the first mine a few turns later than v026 (E058's bonus until turn 12 rather than 15), since it competes with units for the middle.
4. **S1 later, if at all**: the path planner pays only once the goal is known; it can then carry S2's deadlines through contact.
