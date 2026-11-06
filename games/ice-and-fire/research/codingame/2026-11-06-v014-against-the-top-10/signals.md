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

