# Summary of the batch

20 games, 20 decided; 21.5 turns on average.

| Player | Wins as red | Wins as blue | Draws |
| --- | ---: | ---: | ---: |
| v003 | 4 | 6 | 0 |
| v004 | 4 | 6 | 0 |

## Metrics that go with the winner

For each point of the game, in the decided games that reach it and where both sides have a value: how often the winner's value of the metric was higher than the loser's, lower or equal (the percentage leaves out the equal ones), and the mean of the winner's value less the loser's. Some metrics are better low (see their meanings in a game file).

### At the start of the middle game (20 games)

| Metric | Winner's higher | Lower | Equal | Mean difference |
| --- | ---: | ---: | ---: | ---: |
| gold | 44% (7) | 9 | 4 | -3.8 |
| income | 41% (7) | 10 | 3 | -0.3 |
| upkeep | 63% (12) | 7 | 1 | +0.6 |
| cells | 63% (12) | 7 | 1 | +1.4 |
| cut_off | – (0) | 0 | 20 | +0.0 |
| units | 63% (12) | 7 | 1 | +0.6 |
| army | 63% (12) | 7 | 1 | +5.5 |
| mines | 39% (7) | 11 | 2 | -0.3 |
| towers | 0% (0) | 1 | 19 | -0.1 |
| voronoi | 63% (12) | 7 | 1 | +4.0 |
| exposed30 | 33% (2) | 4 | 14 | -0.1 |
| exposed50 | 50% (6) | 6 | 8 | +0.2 |
| can_cut | 27% (3) | 8 | 9 | -0.2 |
| hq_gap | 50% (10) | 10 | 0 | +0.8 |
| material | 65% (13) | 7 | 0 | +5.8 |

### Five turns into the middle game (18 games)

| Metric | Winner's higher | Lower | Equal | Mean difference |
| --- | ---: | ---: | ---: | ---: |
| gold | 39% (7) | 11 | 0 | -1.4 |
| income | 76% (13) | 4 | 1 | +7.9 |
| upkeep | 78% (14) | 4 | 0 | +6.7 |
| cells | 72% (13) | 5 | 0 | +15.3 |
| cut_off | 29% (4) | 10 | 4 | -1.4 |
| units | 76% (13) | 4 | 1 | +4.1 |
| army | 78% (14) | 4 | 0 | +45.0 |
| mines | 38% (6) | 10 | 2 | -0.2 |
| towers | 44% (4) | 5 | 9 | +0.0 |
| voronoi | 78% (14) | 4 | 0 | +27.6 |
| exposed30 | 64% (9) | 5 | 4 | +1.7 |
| exposed50 | 56% (9) | 7 | 2 | +1.3 |
| can_cut | 47% (7) | 8 | 3 | -0.2 |
| hq_gap | 17% (3) | 15 | 0 | -30.8 |
| material | 78% (14) | 4 | 0 | +128.4 |

### At the start of the endgame (20 games)

| Metric | Winner's higher | Lower | Equal | Mean difference |
| --- | ---: | ---: | ---: | ---: |
| gold | 20% (4) | 16 | 0 | -8.7 |
| income | 79% (15) | 4 | 1 | +7.8 |
| upkeep | 80% (16) | 4 | 0 | +6.5 |
| cells | 80% (16) | 4 | 0 | +15.4 |
| cut_off | 17% (3) | 15 | 2 | -5.5 |
| units | 80% (16) | 4 | 0 | +4.7 |
| army | 80% (16) | 4 | 0 | +51.0 |
| mines | 42% (8) | 11 | 1 | -0.2 |
| towers | 40% (2) | 3 | 15 | -0.1 |
| voronoi | 80% (16) | 4 | 0 | +39.5 |
| exposed30 | 44% (8) | 10 | 2 | -0.5 |
| exposed50 | 28% (5) | 13 | 2 | -2.0 |
| can_cut | 21% (4) | 15 | 1 | -1.8 |
| hq_gap | 25% (5) | 15 | 0 | -31.3 |
| material | 80% (16) | 4 | 0 | +126.5 |

## Decisive moments

For each kind, in the decided games: how many games had one, how many there were, and how often the side the first one favoured won.

| Kind | Games | Moments | First one's side won |
| --- | ---: | ---: | ---: |
| HQ threat | 18 | 18 | 100% (18 of 18) |
| debt | 2 | 3 | 50% (1 of 2) |
| large cut | 19 | 49 | 84% (16 of 19) |
| lead change | 7 | 7 | 100% (7 of 7) |
| swing | 20 | 76 | 80% (16 of 20) |

## Reviewers' judgments against the metrics

20 of 20 games reviewed. For each play a reviewer judged, the change of the player's lead in `material` from the start of that turn to the start of its next one; a judgment agrees when a good play raised the lead and a bad one lowered it.

| Verdict | Phase | Plays | Mean change of the lead | Agrees |
| --- | --- | ---: | ---: | ---: |
| bad | all | 76 | -63.8 | 93% |
| bad | early | 8 | -13.2 | 62% |
| bad | end | 23 | -79.6 | 91% |
| bad | middle | 45 | -64.7 | 100% |
| good | all | 144 | +46.1 | 78% |
| good | early | 36 | +3.5 | 64% |
| good | end | 46 | +84.3 | 83% |
| good | middle | 62 | +42.5 | 84% |

12 judgments of a play in a side's last turn of its game were left out: no next turn shows its effect.
