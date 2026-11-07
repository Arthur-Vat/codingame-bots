# Signals

Picked by code from the numbers in this folder; no interpretation. Each line names its chart and the data file it comes from (`README.md` lists the files and their columns).

The AUC of a metric at a turn is the share of pairs (one value of the first group, one of the second) where the first group's value is the better one (higher, or lower where less is better), a tie counting half: 0.5 is no difference. Two groups "separate" from the first turn after which the AUC stays at 0.70 or more (for the top players against us, 0.70 or more or 0.30 or less) to the last turn with at least 5 games on each side. Medians are read for each side right after its own ply of the turn.

## Our wins against our losses

The four metrics with the highest mean AUC at the compared turns, read at the last of them.

- **cells**: our wins and our losses separate from turn 9; at turn 15 the median is 49.5 for our wins and 28 for our losses (AUC 0.91, n = 124 and 134). Chart: `wins-vs-losses-cells.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **voronoi**: our wins and our losses separate from turn 9; at turn 15 the median is 76 for our wins and 37 for our losses (AUC 0.90, n = 124 and 134). Chart: `wins-vs-losses-voronoi.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **material**: our wins and our losses separate from turn 10; at turn 15 the median is 482.5 for our wins and 311.5 for our losses (AUC 0.87, n = 124 and 134). Chart: `wins-vs-losses-material.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **hq_gap**: our wins and our losses separate from turn 10; at turn 15 the median is 52.5 for our wins and 96.5 for our losses (less is better; AUC 0.86, n = 124 and 134). Chart: `wins-vs-losses-hq-gap.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.

## The top players against us

The four metrics where the two groups differ most at the compared turns, read at the turn they separate.

- **voronoi**: us and top players separate from turn 5; at turn 5 the median is 52 for us and 57 for top players (AUC 0.24, n = 106 and 106). Chart: `top-vs-us-voronoi.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **cells**: us and top players separate from turn 1; at turn 1 the median is 3 for us and 4 for top players (AUC 0.06, n = 106 and 106). Chart: `top-vs-us-cells.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **material**: us and top players separate from turn 1; at turn 1 the median is 36 for us and 44 for top players (AUC 0.06, n = 106 and 106). Chart: `top-vs-us-material.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **income**: us and top players do not stay apart to the end; at turn 15 the median is 24 for us and 34.5 for top players (AUC 0.31, n = 95 and 94). Chart: `top-vs-us-income.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.

## Each top player against us

For each player, among the metrics that tell our wins from our losses best, the one where it and we differ most at the compared turns, read at the last of them.

- zasmu, **cells**: us and zasmu separate from turn 1; at turn 15 the median is 35.5 for us and 50.5 for zasmu (AUC 0.11, n = 10 and 10). Chart: `player-zasmu-cells.svg`. Data: `separation.csv` (set `player-zasmu`), `turns-zasmu.csv`.
- Pduhard-, **voronoi**: us and Pduhard- separate from turn 5; at turn 15 the median is 35 for us and 75 for Pduhard- (AUC 0.02, n = 9 and 9). Chart: `player-pduhard--voronoi.svg`. Data: `separation.csv` (set `player-pduhard-`), `turns-pduhard-.csv`.
- karliso, **material**: us and karliso separate from turn 1; at turn 15 the median is 355 for us and 575 for karliso (AUC 0.09, n = 8 and 8). Chart: `player-karliso-material.svg`. Data: `separation.csv` (set `player-karliso`), `turns-karliso.csv`.
- _iaz, **cells**: us and _iaz separate from turn 18; at turn 15 the median is 29 for us and 57 for _iaz (AUC 0.31, n = 9 and 9). Chart: `player--iaz-cells.svg`. Data: `separation.csv` (set `player--iaz`), `turns--iaz.csv`.
- reCurse, **voronoi**: us and reCurse separate from turn 21; at turn 15 the median is 47 for us and 67.5 for reCurse (AUC 0.21, n = 8 and 8). Chart: `player-recurse-voronoi.svg`. Data: `separation.csv` (set `player-recurse`), `turns-recurse.csv`.
- _iahelka, **voronoi**: us and _iahelka separate from turn 6; at turn 15 the median is 18 for us and 84 for _iahelka (AUC 0.02, n = 9 and 9). Chart: `player--iahelka-voronoi.svg`. Data: `separation.csv` (set `player--iahelka`), `turns--iahelka.csv`.
- Waffle3z, **material**: us and Waffle3z separate from turn 16; at turn 15 the median is 336.5 for us and 480.5 for Waffle3z (AUC 0.37, n = 10 and 10). Chart: `player-waffle3z-material.svg`. Data: `separation.csv` (set `player-waffle3z`), `turns-waffle3z.csv`.
- Zylo, **cells**: us and Zylo separate from turn 1; at turn 15 the median is 35 for us and 57.5 for Zylo (AUC 0.14, n = 10 and 10). Chart: `player-zylo-cells.svg`. Data: `separation.csv` (set `player-zylo`), `turns-zylo.csv`.
- NinjaDoggy, **voronoi**: us and NinjaDoggy separate from turn 5; at turn 15 the median is 43 for us and 86 for NinjaDoggy (AUC 0.04, n = 11 and 11). Chart: `player-ninjadoggy-voronoi.svg`. Data: `separation.csv` (set `player-ninjadoggy`), `turns-ninjadoggy.csv`.
- aCat, **voronoi**: us and aCat do not stay apart to the end; at turn 15 the median is 48 for us and 59.5 for aCat (AUC 0.32, n = 11 and 10). Chart: `player-acat-voronoi.svg`. Data: `separation.csv` (set `player-acat`), `turns-acat.csv`.

## Events by turn

Turns where the share of games in which a side does something differs by at least 20 points between the groups, with 5 games or more on each side.

The top players against us:

- **mine**: the rate of us is 20 points higher or more at turns 6 to 7; the rate of top players is 20 points higher or more at turn 11; widest at turn 6: 47% of us against 0% of top players (47 points). Chart: `top-vs-us-event-mine.svg`. Data: `turns-all-top-players.csv`.
- **tower**: the rate of top players is 20 points higher or more at turn 8; widest at turn 8: 8% of us against 50% of top players (43 points). Chart: `top-vs-us-event-tower.svg`. Data: `turns-all-top-players.csv`.
- **train level 1**: the rate of us is 20 points higher or more at turns 8 to 9, 18; the rate of top players is 20 points higher or more at turns 5 to 6; widest at turn 5: 10% of us against 89% of top players (78 points). Chart: `top-vs-us-event-train-level-1.svg`. Data: `turns-all-top-players.csv`.
- **train level 2**: the rate of top players is 20 points higher or more at turn 9; widest at turn 9: 11% of us against 31% of top players (20 points). Chart: `top-vs-us-event-train-level-2.svg`. Data: `turns-all-top-players.csv`.

Our wins against our losses:

- **large cut**: the rate of our wins is 20 points higher or more at turn 12; widest at turn 12: 23% of our wins against 1% of our losses (21 points). Chart: `wins-vs-losses-event-large-cut.svg`. Data: `turns-our-wins.csv`, `turns-our-losses.csv`.
- **train level 1**: the rate of our wins is 20 points higher or more at turns 5, 11 to 12; widest at turn 5: 53% of our wins against 27% of our losses (26 points). Chart: `wins-vs-losses-event-train-level-1.svg`. Data: `turns-our-wins.csv`, `turns-our-losses.csv`.

## Economy or fighting

Two explanations of why a side does better, in numbers and no more. Fighting: the exchange, the training cost of the enemy units a side removed less that of its own it lost. Economy: what a side gains for each unit of resource it spends, where it puts its spending, and how much of its gross income the upkeep eats. Medians at the last compared turn the set reaches; `README.md` defines each series.

- us and top players, at turn 15 (n = 106 and 106): exchange us -20, top players +30; cells per gold us 0.191, top players 0.214; kills per gold on units us 0.357, top players 0.5; upkeep burden us 27.273%, top players 28.849%. Chart: `economy-top-vs-us-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-top-vs-us`).
- our wins and our losses, at turn 15 (n = 175 and 147): exchange our wins +30, our losses -30; cells per gold our wins 0.223, our losses 0.191; kills per gold on units our wins 0.455, our losses 0.333; upkeep burden our wins 29.935%, our losses 24.569%. Chart: `economy-wins-vs-losses-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-wins-vs-losses`).
- us and zasmu, at turn 15 (n = 10 and 10): exchange us -5, zasmu +15; cells per gold us 0.205, zasmu 0.176; kills per gold on units us 0.345, zasmu 0.361; upkeep burden us 28.802%, zasmu 20.81%. Chart: `economy-player-zasmu-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-zasmu`).
- us and Pduhard-, at turn 15 (n = 10 and 10): exchange us -10, Pduhard- +20; cells per gold us 0.184, Pduhard- 0.27; kills per gold on units us 0.5, Pduhard- 0.5; upkeep burden us 19.048%, Pduhard- 56.098%. Chart: `economy-player-pduhard--exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-pduhard-`).
- us and karliso, at turn 15 (n = 10 and 10): exchange us -35, karliso +35; cells per gold us 0.186, karliso 0.211; kills per gold on units us 0.234, karliso 0.5; upkeep burden us 46.307%, karliso 17.544%. Chart: `economy-player-karliso-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-karliso`).
- us and _iaz, at turn 15 (n = 10 and 10): exchange us -20, _iaz +30; cells per gold us 0.177, _iaz 0.218; kills per gold on units us 0.467, _iaz 0.579; upkeep burden us 27.273%, _iaz 36.842%. Chart: `economy-player--iaz-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player--iaz`).
- us and reCurse, at turn 15 (n = 10 and 10): exchange us -25, reCurse +30; cells per gold us 0.219, reCurse 0.184; kills per gold on units us 0.317, reCurse 0.577; upkeep burden us 33.974%, reCurse 23.34%. Chart: `economy-player-recurse-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-recurse`).
- us and _iahelka, at turn 15 (n = 10 and 10): exchange us -70, _iahelka +70; cells per gold us 0.173, _iahelka 0.234; kills per gold on units us 0.312, _iahelka 0.588; upkeep burden us 19.298%, _iahelka 59.722%. Chart: `economy-player--iahelka-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player--iahelka`).
- us and Waffle3z, at turn 15 (n = 11 and 11): exchange us +5, Waffle3z +10; cells per gold us 0.18, Waffle3z 0.19; kills per gold on units us 0.506, Waffle3z 0.439; upkeep burden us 50.125%, Waffle3z 20.139%. Chart: `economy-player-waffle3z-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-waffle3z`).
- us and Zylo, at turn 15 (n = 10 and 10): exchange us -35, Zylo +55; cells per gold us 0.217, Zylo 0.21; kills per gold on units us 0.237, Zylo 0.5; upkeep burden us 17.996%, Zylo 29.385%. Chart: `economy-player-zylo-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-zylo`).
- us and NinjaDoggy, at turn 15 (n = 12 and 12): exchange us -40, NinjaDoggy +60; cells per gold us 0.178, NinjaDoggy 0.208; kills per gold on units us 0.5, NinjaDoggy 0.773; upkeep burden us 34.286%, NinjaDoggy 34.483%. Chart: `economy-player-ninjadoggy-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-ninjadoggy`).
- us and aCat, at turn 15 (n = 13 and 13): exchange us -10, aCat +15; cells per gold us 0.191, aCat 0.245; kills per gold on units us 0.435, aCat 0.531; upkeep burden us 23.611%, aCat 36.046%. Chart: `economy-player-acat-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-acat`).

Investment mix, the share of spending on each role by phase (`economy_mix.csv`):

- top players, early game: economy 5%, army 92%, defence 4% (735 plies).
- top players, middle game: economy 13%, army 54%, defence 33% (596 plies).
- top players, endgame: economy 6%, army 78%, defence 16% (656 plies).
- us, same games, early game: economy 31%, army 69%, defence 0% (760 plies).
- us, same games, middle game: economy 1%, army 73%, defence 26% (607 plies).
- us, same games, endgame: economy 2%, army 84%, defence 14% (634 plies).
- our wins, early game: economy 24%, army 76%, defence 0% (1166 plies).
- our wins, middle game: economy 1%, army 83%, defence 16% (921 plies).
- our wins, endgame: economy 6%, army 88%, defence 6% (914 plies).
- our losses, early game: economy 29%, army 71%, defence 0% (1044 plies).
- our losses, middle game: economy 1%, army 70%, defence 29% (877 plies).
- our losses, endgame: economy 2%, army 80%, defence 18% (1058 plies).

Tempo (`economy_tempo.csv`):

- top players: first contact at turn 7 (105 of 106 games); first decisive moment in its favour at turn 15 (85 games); from first contact to it 7 turns (85 games).
- us, same games: first contact at turn 8 (105 of 106 games); first decisive moment in its favour at turn 14 (31 games); from first contact to it 7 turns (31 games).
- our wins: first contact at turn 8 (167 of 175 games); first decisive moment in its favour at turn 13 (163 games); from first contact to it 5 turns (161 games).
- our losses: first contact at turn 8 (147 of 147 games); first decisive moment in its favour at turn 15 (25 games); from first contact to it 7 turns (25 games).

The value of a lead, in our games: wins out of games from each size of lead (`economy_lead.csv`):

- material at turn 5: behind by more than 16: 14 of 40; behind by up to 16: 69 of 159; ahead by up to 16: 71 of 100; ahead by more than 16: 16 of 18.
- material at turn 10: behind by more than 57: 24 of 107; behind by up to 57: 40 of 85; ahead by up to 57: 56 of 73; ahead by more than 57: 48 of 50.
- material at turn 15: behind by more than 187: 0 of 66; behind by up to 187: 18 of 71; level: 1 of 1; ahead by up to 187: 39 of 52; ahead by more than 187: 55 of 57.
- material at turn 20: behind by more than 296: 0 of 32; behind by up to 296: 4 of 32; ahead by up to 296: 19 of 23; ahead by more than 296: 23 of 23.
- material at turn 30: behind by more than 291: 0 of 7; behind by up to 291: 1 of 6; ahead by up to 291: 2 of 2; ahead by more than 291: 1 of 1.
- income at turn 5: behind by more than 1: 8 of 19; behind by up to 1: 61 of 146; level: 13 of 27; ahead by up to 1: 70 of 101; ahead by more than 1: 18 of 24.
- income at turn 10: behind by more than 5: 17 of 65; behind by up to 5: 34 of 83; level: 16 of 31; ahead by up to 5: 43 of 66; ahead by more than 5: 58 of 70.
- income at turn 15: behind by more than 16: 7 of 64; behind by up to 16: 21 of 62; level: 1 of 2; ahead by up to 16: 35 of 61; ahead by more than 16: 49 of 58.
- income at turn 20: behind by more than 21: 0 of 26; behind by up to 21: 6 of 31; level: 2 of 4; ahead by up to 21: 14 of 25; ahead by more than 21: 24 of 24.
- income at turn 30: behind by more than 9: 0 of 2; behind by up to 9: 1 of 6; ahead by up to 9: 1 of 3; ahead by more than 9: 2 of 5.

## Versions

A player who changes its code gets a new agent id (`versions.csv`, `profile_versions.csv`).

- 1 day of leaderboard snapshots: no change from one day to the next can be seen.

