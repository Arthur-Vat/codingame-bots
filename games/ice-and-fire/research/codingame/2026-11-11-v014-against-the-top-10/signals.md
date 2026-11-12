# Signals

Picked by code from the numbers in this folder; no interpretation. Each line names its chart and the data file it comes from (`README.md` lists the files and their columns).

The AUC of a metric at a turn is the share of pairs (one value of the first group, one of the second) where the first group's value is the better one (higher, or lower where less is better), a tie counting half: 0.5 is no difference. Two groups "separate" from the first turn after which the AUC stays at 0.70 or more (for the top players against us, 0.70 or more or 0.30 or less) to the last turn with at least 5 games on each side. Medians are read for each side right after its own ply of the turn.

## Our wins against our losses

The four metrics with the highest mean AUC at the compared turns, read at the last of them.

- **cells**: our wins and our losses separate from turn 10; at turn 15 the median is 48 for our wins and 29 for our losses (AUC 0.89, n = 456 and 1399). Chart: `wins-vs-losses-cells.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **voronoi**: our wins and our losses separate from turn 10; at turn 15 the median is 67 for our wins and 35 for our losses (AUC 0.88, n = 456 and 1399). Chart: `wins-vs-losses-voronoi.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **material**: our wins and our losses separate from turn 10; at turn 15 the median is 474 for our wins and 322 for our losses (AUC 0.87, n = 456 and 1399). Chart: `wins-vs-losses-material.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.
- **hq_gap**: our wins and our losses separate from turn 10; at turn 15 the median is 61 for our wins and 103 for our losses (less is better; AUC 0.85, n = 456 and 1399). Chart: `wins-vs-losses-hq-gap.svg`. Data: `separation.csv` (set `wins-vs-losses`), `turns-our-wins.csv`, `turns-our-losses.csv`.

## The top players against us

The four metrics where the two groups differ most at the compared turns, read at the turn they separate.

- **cells**: us and top players separate from turn 1; at turn 1 the median is 3 for us and 4 for top players (AUC 0.00, n = 1785 and 1777). Chart: `top-vs-us-cells.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **material**: us and top players separate from turn 1; at turn 1 the median is 36 for us and 44 for top players (AUC 0.00, n = 1785 and 1777). Chart: `top-vs-us-material.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **voronoi**: us and top players separate from turn 6; at turn 6 the median is 50 for us and 60 for top players (AUC 0.22, n = 1777 and 1777). Chart: `top-vs-us-voronoi.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.
- **income**: us and top players separate from turn 10; at turn 10 the median is 27 for us and 32 for top players (AUC 0.29, n = 1763 and 1763). Chart: `top-vs-us-income.svg`. Data: `separation.csv` (set `top-vs-us`), `turns-all-top-players.csv`.

## Each top player against us

For each player, among the metrics that tell our wins from our losses best, the one where it and we differ most at the compared turns, read at the last of them.

- zasmu, **cells**: us and zasmu separate from turn 1; at turn 15 the median is 32 for us and 51 for zasmu (AUC 0.06, n = 187 and 187). Chart: `player-zasmu-cells.svg`. Data: `separation.csv` (set `player-zasmu`), `turns-zasmu.csv`.
- Pduhard-, **cells**: us and Pduhard- separate from turn 1; at turn 15 the median is 26 for us and 50 for Pduhard- (AUC 0.08, n = 160 and 160). Chart: `player-pduhard--cells.svg`. Data: `separation.csv` (set `player-pduhard-`), `turns-pduhard-.csv`.
- karliso, **material**: us and karliso separate from turn 1; at turn 15 the median is 348 for us and 568 for karliso (AUC 0.13, n = 163 and 163). Chart: `player-karliso-material.svg`. Data: `separation.csv` (set `player-karliso`), `turns-karliso.csv`.
- _iaz, **cells**: us and _iaz do not stay apart to the end; at turn 15 the median is 27 for us and 56 for _iaz (AUC 0.20, n = 171 and 171). Chart: `player--iaz-cells.svg`. Data: `separation.csv` (set `player--iaz`), `turns--iaz.csv`.
- reCurse, **material**: us and reCurse do not stay apart to the end; at turn 15 the median is 370 for us and 522 for reCurse (AUC 0.27, n = 171 and 171). Chart: `player-recurse-material.svg`. Data: `separation.csv` (set `player-recurse`), `turns-recurse.csv`.
- _iahelka, **cells**: us and _iahelka separate from turn 1; at turn 15 the median is 28 for us and 56 for _iahelka (AUC 0.06, n = 151 and 151). Chart: `player--iahelka-cells.svg`. Data: `separation.csv` (set `player--iahelka`), `turns--iahelka.csv`.
- Waffle3z, **material**: us and Waffle3z separate from turn 1; at turn 15 the median is 345 for us and 504 for Waffle3z (AUC 0.23, n = 171 and 169). Chart: `player-waffle3z-material.svg`. Data: `separation.csv` (set `player-waffle3z`), `turns-waffle3z.csv`.
- Zylo, **cells**: us and Zylo separate from turn 1; at turn 15 the median is 30 for us and 56 for Zylo (AUC 0.14, n = 191 and 191). Chart: `player-zylo-cells.svg`. Data: `separation.csv` (set `player-zylo`), `turns-zylo.csv`.
- N3l, **cells**: us and N3l separate from turn 1; at turn 15 the median is 29 for us and 55 for N3l (AUC 0.27, n = 27 and 25). Chart: `player-n3l-cells.svg`. Data: `separation.csv` (set `player-n3l`), `turns-n3l.csv`.
- NinjaDoggy, **voronoi**: us and NinjaDoggy separate from turn 5; at turn 15 the median is 35 for us and 70 for NinjaDoggy (AUC 0.19, n = 159 and 159). Chart: `player-ninjadoggy-voronoi.svg`. Data: `separation.csv` (set `player-ninjadoggy`), `turns-ninjadoggy.csv`.

## Events by turn

Turns where the share of games in which a side does something differs by at least 20 points between the groups, with 5 games or more on each side.

The top players against us:

- **mine**: the rate of us is 20 points higher or more at turns 6 to 7; the rate of top players is 20 points higher or more at turns 9 to 11; widest at turn 6: 56% of us against 0% of top players (56 points). Chart: `top-vs-us-event-mine.svg`. Data: `turns-all-top-players.csv`.
- **tower**: the rate of top players is 20 points higher or more at turns 7 to 8; widest at turn 8: 15% of us against 48% of top players (33 points). Chart: `top-vs-us-event-tower.svg`. Data: `turns-all-top-players.csv`.
- **train level 1**: the rate of us is 20 points higher or more at turn 8; the rate of top players is 20 points higher or more at turns 5 to 6; widest at turn 5: 7% of us against 98% of top players (92 points). Chart: `top-vs-us-event-train-level-1.svg`. Data: `turns-all-top-players.csv`.

Our wins against our losses:

- No event differs by 20 points at a turn with 5 games on each side.

## Economy or fighting

Two explanations of why a side does better, in numbers and no more. Fighting: the exchange, the training cost of the enemy units a side removed less that of its own it lost. Economy: what a side gains for each unit of resource it spends, where it puts its spending, and how much of its gross income the upkeep eats. Medians at the last compared turn the set reaches; `README.md` defines each series.

- us and top players, at turn 15 (n = 1785 and 1785): exchange us -20, top players +30; cells per gold us 0.194, top players 0.212; kills per gold on units us 0.353, top players 0.526; upkeep burden us 25.806%, top players 26.027%. Chart: `economy-top-vs-us-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-top-vs-us`).
- our wins and our losses, at turn 15 (n = 559 and 1617): exchange our wins +20, our losses -20; cells per gold our wins 0.216, our losses 0.189; kills per gold on units our wins 0.408, our losses 0.333; upkeep burden our wins 31.148%, our losses 25%. Chart: `economy-wins-vs-losses-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-wins-vs-losses`).
- us and zasmu, at turn 15 (n = 196 and 196): exchange us 0, zasmu +10; cells per gold us 0.205, zasmu 0.175; kills per gold on units us 0.333, zasmu 0.368; upkeep burden us 27.586%, zasmu 17.188%. Chart: `economy-player-zasmu-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-zasmu`).
- us and Pduhard-, at turn 15 (n = 196 and 196): exchange us -10, Pduhard- +20; cells per gold us 0.184, Pduhard- 0.255; kills per gold on units us 0.5, Pduhard- 0.5; upkeep burden us 29.79%, Pduhard- 58.537%. Chart: `economy-player-pduhard--exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-pduhard-`).
- us and karliso, at turn 15 (n = 194 and 194): exchange us -30, karliso +40; cells per gold us 0.188, karliso 0.194; kills per gold on units us 0.364, karliso 0.611; upkeep burden us 25%, karliso 18.75%. Chart: `economy-player-karliso-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-karliso`).
- us and _iaz, at turn 15 (n = 195 and 195): exchange us -30, _iaz +40; cells per gold us 0.19, _iaz 0.238; kills per gold on units us 0.312, _iaz 0.522; upkeep burden us 22.917%, _iaz 38.636%. Chart: `economy-player--iaz-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player--iaz`).
- us and reCurse, at turn 15 (n = 195 and 195): exchange us -10, reCurse +30; cells per gold us 0.203, reCurse 0.208; kills per gold on units us 0.316, reCurse 0.588; upkeep burden us 34.328%, reCurse 20.339%. Chart: `economy-player-recurse-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-recurse`).
- us and _iahelka, at turn 15 (n = 195 and 195): exchange us -40, _iahelka +60; cells per gold us 0.182, _iahelka 0.231; kills per gold on units us 0.364, _iahelka 0.607; upkeep burden us 21.053%, _iahelka 47.143%. Chart: `economy-player--iahelka-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player--iahelka`).
- us and Waffle3z, at turn 15 (n = 196 and 196): exchange us +10, Waffle3z +10; cells per gold us 0.186, Waffle3z 0.193; kills per gold on units us 0.471, Waffle3z 0.462; upkeep burden us 33.333%, Waffle3z 22.222%. Chart: `economy-player-waffle3z-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-waffle3z`).
- us and Zylo, at turn 15 (n = 195 and 195): exchange us -30, Zylo +50; cells per gold us 0.2, Zylo 0.213; kills per gold on units us 0.294, Zylo 0.524; upkeep burden us 22.222%, Zylo 28.846%. Chart: `economy-player-zylo-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-zylo`).
- us and N3l, at turn 15 (n = 27 and 27): exchange us -30, N3l +50; cells per gold us 0.194, N3l 0.216; kills per gold on units us 0.353, N3l 0.52; upkeep burden us 23.077%, N3l 31.707%. Chart: `economy-player-n3l-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-n3l`).
- us and NinjaDoggy, at turn 15 (n = 196 and 196): exchange us -30, NinjaDoggy +40; cells per gold us 0.189, NinjaDoggy 0.215; kills per gold on units us 0.3, NinjaDoggy 0.636; upkeep burden us 25%, NinjaDoggy 30%. Chart: `economy-player-ninjadoggy-exchange.svg` and its neighbours. Data: `economy_curves.csv` (set `economy-player-ninjadoggy`).

Investment mix, the share of spending on each role by phase (`economy_mix.csv`):

- top players, early game: economy 5%, army 90%, defence 5% (12344 plies).
- top players, middle game: economy 14%, army 51%, defence 35% (10051 plies).
- top players, endgame: economy 7%, army 73%, defence 20% (11888 plies).
- us, same games, early game: economy 33%, army 67%, defence 0% (12834 plies).
- us, same games, middle game: economy 0%, army 74%, defence 26% (10300 plies).
- us, same games, endgame: economy 4%, army 83%, defence 14% (11449 plies).
- our wins, early game: economy 31%, army 69%, defence 0% (3884 plies).
- our wins, middle game: economy 1%, army 82%, defence 17% (3195 plies).
- our wins, endgame: economy 8%, army 86%, defence 6% (4051 plies).
- our losses, early game: economy 33%, army 67%, defence 0% (11636 plies).
- our losses, middle game: economy 0%, army 72%, defence 27% (9483 plies).
- our losses, endgame: economy 2%, army 81%, defence 17% (9769 plies).

Tempo (`economy_tempo.csv`):

- top players: first contact at turn 7 (1762 of 1785 games); first decisive moment in its favour at turn 15 (1454 games); from first contact to it 8 turns (1453 games).
- us, same games: first contact at turn 8 (1759 of 1785 games); first decisive moment in its favour at turn 15 (482 games); from first contact to it 7 turns (482 games).
- our wins: first contact at turn 8 (530 of 559 games); first decisive moment in its favour at turn 14 (510 games); from first contact to it 6 turns (508 games).
- our losses: first contact at turn 8 (1613 of 1617 games); first decisive moment in its favour at turn 15 (234 games); from first contact to it 7 turns (234 games).

The value of a lead, in our games: wins out of games from each size of lead (`economy_lead.csv`):

- material at turn 5: behind by more than 16: 45 of 430; behind by up to 16: 398 of 1597; ahead by up to 16: 87 of 118; ahead by more than 16: 16 of 18.
- material at turn 10: behind by more than 64: 65 of 954; behind by up to 64: 198 of 789; level: 8 of 15; ahead by up to 64: 168 of 289; ahead by more than 64: 92 of 101.
- material at turn 15: behind by more than 196: 5 of 729; behind by up to 196: 89 of 669; level: 1 of 1; ahead by up to 196: 152 of 246; ahead by more than 196: 184 of 185.
- material at turn 20: behind by more than 269: 4 of 309; behind by up to 269: 32 of 258; level: 0 of 1; ahead by up to 269: 105 of 156; ahead by more than 269: 104 of 104.
- material at turn 30: behind by more than 259: 1 of 37; behind by up to 259: 4 of 35; ahead by up to 259: 17 of 26; ahead by more than 259: 19 of 19.
- income at turn 5: behind by more than 1: 16 of 209; behind by up to 1: 334 of 1524; level: 78 of 188; ahead by up to 1: 96 of 154; ahead by more than 1: 22 of 88.
- income at turn 10: behind by more than 6: 57 of 730; behind by up to 6: 121 of 644; level: 35 of 106; ahead by up to 6: 185 of 401; ahead by more than 6: 133 of 267.
- income at turn 15: behind by more than 19: 30 of 720; behind by up to 19: 86 of 543; level: 4 of 14; ahead by up to 19: 155 of 366; ahead by more than 19: 156 of 187.
- income at turn 20: behind by more than 23: 10 of 295; behind by up to 23: 44 of 242; level: 6 of 16; ahead by up to 23: 103 of 183; ahead by more than 23: 82 of 92.
- income at turn 30: behind by more than 21: 1 of 28; behind by up to 21: 2 of 31; level: 1 of 1; ahead by up to 21: 14 of 29; ahead by more than 21: 23 of 28.

## Versions

A player who changes its code gets a new agent id (`versions.csv`, `profile_versions.csv`).

- No tracked top player changed its agent id between the 6 days of snapshots (2026-11-06 to 2026-11-11).

