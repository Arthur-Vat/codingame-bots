# CodinGame players and our weaknesses: ice-and-fire-v014 against the top 10

What it covers: 322 games of `ice-and-fire-v014` (our nickname is Tarleya), all on the branch `claude/codingame-games` (commit ac331ba):

- 222 ranking games that its submission played on 2026-10-31 against Legend players;
- 100 test games against the top 10 of the leaderboard on 2026-11-06, 10 against each on the 10 fixed maps of `games/ice-and-fire/codingame-seeds.txt`: zasmu (#1) in a first pilot, then ranks 2 to 10 (batch `2026-11-06-ice-and-fire-v014-top2-10`). In test games our bot is always player 0 (red, first to play).

Ranks come from the leaderboard snapshot of 2026-11-06, 16:51 UTC. All 322 games were converted, replayed by our engine and found to agree with CodinGame, command by command.

Made by (about 5 seconds on 4 cores), from a worktree of the data branch:

```sh
ice-and-fire-review codingame-report --data <claude/codingame-games>/ice-and-fire --me Tarleya > report.md
```

Reading it: "ours" is Tarleya; a metric "at turn t" is read for each side right after its own ply of turn t (phase means: at the starts of its own plies); events and openings count the commands carried out (refused commands dropped); `[n = k]` marks a sample of fewer than 5 games.

## Interpretation

Written by Claude from the tables below; numbers are medians unless said otherwise.

**The top 10 are a level above us.** Against them v014 won 20 of 106 games (100 test games and 6 ranking games). Against ranks 11 to 30 it won 61 of 106, against ranks 31 to 60 22 of 35, and below that 39 of 40. Only aCat (#10, 5 won of 13) and Waffle3z (#7, 4 of 11) are within reach; we won 0 of 10 against karliso and 1 of 10 against zasmu, Pduhard-, _iahelka and Zylo.

**We do not lose the opening; we lose turns 8 to 15.** At turn 5 both sides hold about the same cells and income. By turn 10 seven of the top 10 earn 2.5 to 9 gold a turn more than we do (e.g. karliso 36 against our 27, Waffle3z 36 against 27); by turn 15 they hold 41 to 71 cells against our 15 to 39, and our income has stopped growing (17 to 28) while theirs keeps rising for seven of them (zasmu 49.5, karliso 59, Waffle3z 45.5). In our 147 losses, our income stays below the opponent's from turn 13 to the end in 95% of them, our towers from turn 11 in 76%.

**They spend on growth and strength; we keep gold.** Per game, the top bots train 2.7 to 4.9 level-2 units against our 1.3 to 2.8, and 1.1 to 2.6 level-3 units against our 0.3 to 1.8. Seven of them build 1.5 to 5.2 mines a game against our 1.1 to 2.0 (Pduhard-, NinjaDoggy and aCat build none). zasmu and Zylo build 11.2 and 8.4 towers a game against our 3.1 and 4.3. Against the strongest, we often sit on unspent gold at turn 15 (45 against zasmu's 7, 63 against Zylo's 23.5, 31 against _iahelka's 11) while they have turned theirs into income, units and towers.

**How they beat us: a large cut around turn 12 to 14.** The first moment favouring them in their wins is a large cut of our territory for NinjaDoggy (8 of 10 wins), Pduhard- (5 of 9), _iaz (4 of 8), karliso (4 of 10) and reCurse (3 of 7), around turns 12 to 14; a swing in holdings around turns 11 to 13 for _iahelka, Waffle3z and Zylo; zasmu more often threatens our HQ around turn 17. Games against them last 16 to 20.5 turns.

**What this suggests for our bot** (to be tested, one experiment each): spend in the middle game rather than bank (our banking keeps gold the top bots invest); build mines earlier and more often when spots are safe; train level-2 units to hold and push the front; and guard the cells whose loss would cut us off before turn 12, when the deciding cuts come (the decisive-moments study looks at our exposure before them). The judge comparison will show whether our evaluation underrates these moves or our search does not find them.

**Limits.** 10 games per top bot on 10 maps, always as player 0 in the test games; 6 ranking games against them otherwise. Differences of a few units are within the noise of 10 games; the gaps above (income, cells, mines, level-2 units) are large and consistent across the ten bots.

## Sources

Events and openings count the commands carried out by the rules, not the ones refused (a refused command has no effect).

| kind | batch | games |
|---|---:|---:|
| ranking | - | 222 |
| test | - | 10 |
| test | 2026-11-06-ice-and-fire-v014-top2-10 | 90 |

322 games used.
Ranks from the leaderboard snapshot 2026-11-06T165130Z.json.
No game left out.

## Opponents

Results are ours (won-lost, draws last when any). Seat columns: our seat 0 and our seat 1.

| player | rank | games | won-lost | seat 0 / 1 | turns |
|---|---:|---:|---:|---:|---:|
| zasmu | 1 | 10 | 1-9 | 1-9 / 0-0 | 19.5 |
| Pduhard- | 2 | 10 | 1-9 | 1-9 / 0-0 | 19 |
| karliso | 3 | 10 | 0-10 | 0-10 / 0-0 | 16 |
| _iaz | 4 | 10 | 2-8 | 2-8 / 0-0 | 17 |
| reCurse | 5 | 10 | 3-7 | 3-7 / 0-0 | 20 |
| _iahelka | 6 | 10 | 1-9 | 1-9 / 0-0 | 17 |
| Waffle3z | 7 | 11 | 4-7 | 4-6 / 0-1 | 16 |
| Zylo | 8 | 10 | 1-9 | 1-9 / 0-0 | 20.5 |
| NinjaDoggy | 9 | 12 | 2-10 | 2-8 / 0-2 | 17.5 |
| aCat | 10 | 13 | 5-8 | 5-5 / 0-3 | 17 |
| putibuzu | 11 | 5 | 0-5 | 0-4 / 0-1 | 21 |
| N3l | 12 | 7 | 2-5 | 1-1 / 1-4 | 22 |
| eulerscheZahl | 13 | 7 | 3-4 | 1-0 / 2-4 | 24 |
| ValGrowth | 14 | 10 | 4-6 | 1-4 / 3-2 | 18.5 |
| AlesRamsa | 15 | 4 [n = 4] | 2-2 | 1-1 / 1-1 | 25 |
| bonebone | 16 | 6 | 4-2 | 4-1 / 0-1 | 21 |
| Azkellas | 17 | 4 [n = 4] | 1-3 | 0-2 / 1-1 | 16 |
| ArturBieniek | 18 | 8 | 6-2 | 3-2 / 3-0 | 20 |
| siman | 20 | 9 | 7-2 | 3-1 / 4-1 | 20 |
| _Royale | 21 | 5 | 4-1 | 1-1 / 3-0 | 17 |
| kovi | 22 | 5 | 2-3 | 0-3 / 2-0 | 17 |
| wala | 23 | 6 | 2-4 | 0-3 / 2-1 | 23 |
| FrancoisB | 24 | 7 | 4-3 | 3-2 / 1-1 | 20 |
| Gabbek | 25 | 8 | 7-1 | 3-1 / 4-0 | 16.5 |
| Neumann | 26 | 7 | 6-1 | 4-0 / 2-1 | 15 |
| KnightMoves | 27 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 25 |
| aropan | 28 | 3 [n = 3] | 3-0 | 2-0 / 1-0 | 17 |
| kozuechan-AI | 29 | 2 [n = 2] | 1-1 | 0-1 / 1-0 | 24.5 |
| MSmits | 30 | 2 [n = 2] | 2-0 | 1-0 / 1-0 | 15 |
| BorisZ | 31 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 14 |
| Fangel | 32 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 32 |
| IloIlo | 33 | 2 [n = 2] | 2-0 | 1-0 / 1-0 | 14 |
| Magus | 34 | 3 [n = 3] | 1-2 | 1-1 / 0-1 | 27 |
| Nerchio | 35 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 22 |
| Drobor | 36 | 4 [n = 4] | 2-2 | 0-2 / 2-0 | 17.5 |
| dbdr | 37 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 0 |
| tanakat01 | 38 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 16 |
| OccitanieFreelancer | 39 | 2 [n = 2] | 0-2 | 0-2 / 0-0 | 17 |
| Marchete | 40 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 15 |
| Petras | 41 | 1 [n = 1] | 0-1 | 0-1 / 0-0 | 37 |
| PED | 43 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 17 |
| TylerDurden | 44 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 14 |
| ThomasNicoullaud | 45 | 2 [n = 2] | 1-1 | 1-0 / 0-1 | 15 |
| k4ng0u | 47 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 15 |
| Khao | 50 | 1 [n = 1] | 0-1 | 0-1 / 0-0 | 20 |
| DaFish | 51 | 1 [n = 1] | 0-1 | 0-1 / 0-0 | 25 |
| cedricdd | 52 | 2 [n = 2] | 1-1 | 1-0 / 0-1 | 23.5 |
| mikmak | 53 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 17 |
| wlesavo | 54 | 1 [n = 1] | 0-1 | 0-0 / 0-1 | 38 |
| kiwijam | 56 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 19 |
| DuckTypo | 59 | 3 [n = 3] | 2-1 | 0-0 / 2-1 | 15 |
| CodingGigachad | 60 | 2 [n = 2] | 2-0 | 1-0 / 1-0 | 17.5 |
| tutubalin | 61 | 2 [n = 2] | 2-0 | 0-0 / 2-0 | 17.5 |
| daaskare | 62 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 18 |
| egaetan | 63 | 1 [n = 1] | 0-1 | 0-0 / 0-1 | 30 |
| Samsonov | 65 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 19 |
| yurp | 66 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 15 |
| chris.steele | 69 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 19 |
| xurei | 72 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 20 |
| Cramer | 76 | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 17 |
| CrystalRushLearning | 78 | 2 [n = 2] | 2-0 | 1-0 / 1-0 | 10.5 |
| YALOKGAR | 85 | 15 | 15-0 | 7-0 / 8-0 | 14 |
| HKit | 90 | 8 | 8-0 | 1-0 / 7-0 | 14.5 |
| Fenz | 93 | 4 [n = 4] | 4-0 | 3-0 / 1-0 | 9.5 |
| TwT | 94 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 12 |
| dolmen1234 | 98 | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 14 |
| -Rutger- | - | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 14 |
| Cize | - | 2 [n = 2] | 2-0 | 1-0 / 1-0 | 14 |
| Daenerys Bossgaryen | - | 18 | 16-2 | 8-1 / 8-1 | 18.5 |
| DirtyHarry | - | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 15 |
| Eris_C | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 11 |
| Fantomat75 | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 15 |
| Gonny | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 11 |
| ItsNotABug | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 25 |
| IvanBot | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 17 |
| Michael_Howard | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 12 |
| PonyPonyCodeCode | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 12 |
| Seti | - | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 13 |
| Unknown | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 16 |
| YurkovAS | - | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 16 |
| cg123 | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 12 |
| r3versi | - | 1 [n = 1] | 1-0 | 1-0 / 0-0 | 13 |
| vinzz | - | 1 [n = 1] | 1-0 | 0-0 / 1-0 | 0 |

How the games ended, over all opponents: the rules ended the game: 310; blue timed out: 6; red timed out: 6.

Opponents with a game that did not end with "the rules ended the game":

| player | endings |
|---|---:|
| NinjaDoggy | the rules ended the game: 11; blue timed out: 1 |
| ValGrowth | the rules ended the game: 9; blue timed out: 1 |
| kovi | the rules ended the game: 4; red timed out: 1 |
| FrancoisB | the rules ended the game: 6; red timed out: 1 |
| Gabbek | the rules ended the game: 7; blue timed out: 1 |
| dbdr | red timed out: 1 |
| tutubalin | red timed out: 1; the rules ended the game: 1 |
| CrystalRushLearning | blue timed out: 1; the rules ended the game: 1 |
| HKit | the rules ended the game: 7; red timed out: 1 |
| Fenz | blue timed out: 2; the rules ended the game: 2 |
| vinzz | red timed out: 1 |

## Profiles

### zasmu (rank 1, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 9-1 |
| both | 10 | 9-1 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 | t20 |
|---|---:|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 | 5 / 5 |
| gold | 2 / 7 | 5.5 / 15 | 7 / 45 | 141 / 68 |
| income | 11 / 10 | 33 / 24 | 49.5 / 28 | 58 / 22 |
| cells | 15 / 13 | 41 / 27 | 50.5 / 35.5 | 59 / 37 |
| army | 40 / 30 | 85 / 50 | 90 / 85 | 80 / 130 |
| mines | 0 / 0 | 1 / 1 | 3 / 1 | 3 / 1 |
| towers | 0 / 0 | 1 / 1 | 5 / 2 | 12 / 3 |
| exposed30 | 0 / 0 | 2 / 2.5 | 2 / 2.5 | 1 / 4 |
| hq_gap | 128 / 144 | 102 / 102 | 93.5 / 96 | 17 / 102 |
| material | 135 / 119 | 359 / 265.5 | 544.5 / 373 | 707 / 441 |
| voronoi | 57 / 53.5 | 63 / 42 | 65 / 43 | 70 / 45 |

Left out: t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 12.9 / 11.6 | 34.8 / 38.5 | 82.9 / 74 |
| income | 6.7 / 5.8 | 28 / 23.1 | 46.2 / 22.6 |
| cells | 9.2 / 7.7 | 34.5 / 25.3 | 47.2 / 34.2 |
| army | 25.2 / 21.8 | 69.8 / 52.3 | 83 / 92.6 |
| mines | 0 / 0.1 | 0.8 / 1.1 | 2.7 / 1.1 |
| towers | 0 / 0 | 1.4 / 0.7 | 6.9 / 2.2 |
| exposed30 | 0.1 / 0 | 2.8 / 3.7 | 2 / 3.3 |
| hq_gap | 154.4 / 168.6 | 79.9 / 93.4 | 44.3 / 76.5 |
| material | 95.1 / 81.7 | 340.1 / 267.7 | 562.7 / 393.1 |
| voronoi | 51.8 / 47.7 | 59.3 / 40.4 | 58.8 / 42.9 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| mine | 100% (10/10) | 80% (8/10) | 9.5 | 3.2 / 1.3 |
| tower | 100% (10/10) | 100% (10/10) | 8 | 11.2 / 3.1 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 16.8 / 15.7 |
| train level 2 | 100% (10/10) | 80% (8/10) | 9 | 4.1 / 1.6 |
| train level 3 | 50% (5/10) | 60% (6/10) | 13 | 1.1 / 0.8 |
| large cut | 20% (2/10) | 40% (4/10) | 14.5 [n = 2] | 0.3 / 0.4 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 100% (9/9) | 0% (0/1) [n = 1] | 9 | 20 |
| HQ threat | for us | 0% (0/9) | 100% (1/1) [n = 1] | 1 | 19 [n = 1] |
| large cut | for them | 22% (2/9) | 0% (0/1) [n = 1] | 3 | 15 [n = 3] |
| large cut | for us | 33% (3/9) | 100% (1/1) [n = 1] | 4 | 12 [n = 4] |
| lead change | for them | 11% (1/9) | 0% (0/1) [n = 1] | 1 | 18 [n = 1] |
| lead change | for us | 11% (1/9) | 100% (1/1) [n = 1] | 2 | 14.5 [n = 2] |
| swing | for them | 44% (4/9) | 0% (0/1) [n = 1] | 4 | 14 [n = 4] |
| swing | for us | 33% (3/9) | 100% (1/1) [n = 1] | 5 | 18 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(0,2) T1(0,3) / M(0,4) M(1,2) / M(0,5) M(1,1)` | 3 [n = 3] | 100% (3/3) [n = 3] |
| `T1(1,1) T1(1,2) / M(1,0) M(2,2) / M(2,0) M(3,2)` | 2 [n = 2] | 50% (1/2) [n = 2] |
| `T1(0,2) T1(0,3) / M(0,4) M(1,2) / M(1,1) M(1,4)` | 1 [n = 1] | 100% (1/1) [n = 1] |

**How its wins came** (9 wins):

Endings: the rules ended the game: 9.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| HQ threat | 44% (4/9) | 17 [n = 4] |
| swing | 33% (3/9) | 15 [n = 3] |
| large cut | 22% (2/9) | 14.5 [n = 2] |

### Pduhard- (rank 2, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 9-1 |
| both | 10 | 9-1 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 9 / 9 |
| gold | 2 / 7 | 37 / 21.5 | 28 / 5 |
| income | 11 / 10 | 29 / 27.5 | 18 / 20 |
| cells | 15 / 13 | 38 / 24 | 50 / 24 |
| army | 40 / 30 | 80 / 50 | 120 / 60 |
| mines | 0 / 0 | 0 / 1.5 | 0 / 1 |
| towers | 0 / 0 | 1 / 1 | 1 / 3 |
| exposed30 | 0 / 0 | 3.5 / 2 | 4 / 4 |
| hq_gap | 128 / 143.5 | 52.5 / 106.5 | 40 / 121 |
| material | 135 / 119 | 354.5 / 270.5 | 443 / 254 |
| voronoi | 58 / 51 | 70 / 42 | 75 / 35 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 9 | 10 / 10 |
| gold | 12.8 / 12.3 | 38.4 / 39.6 | 62.4 / 32.5 |
| income | 6.5 / 6.6 | 22.4 / 24.5 | 23 / 20.3 |
| cells | 9 / 8.3 | 30.3 / 22.6 | 44 / 23.7 |
| army | 24.9 / 22.4 | 62.3 / 43 | 96.6 / 60.5 |
| mines | 0 / 0.2 | 0 / 1.7 | 0 / 1.5 |
| towers | 0 / 0 | 0.9 / 0.6 | 1.4 / 1.7 |
| exposed30 | 0.1 / 0.2 | 2.6 / 3.9 | 5.3 / 5.2 |
| hq_gap | 153.6 / 165.5 | 70.6 / 91.8 | 45 / 105.1 |
| material | 93.4 / 87.6 | 298.4 / 252.3 | 417.5 / 265.5 |
| voronoi | 52.2 / 46.8 | 61.1 / 38 | 68.8 / 29.6 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 15.1 / 17.5 |
| train level 2 | 100% (10/10) | 70% (7/10) | 10.5 | 2.7 / 1.4 |
| tower | 90% (9/10) | 100% (10/10) | 7 | 1.9 / 3.7 |
| train level 3 | 90% (9/10) | 40% (4/10) | 13 | 2.3 / 0.6 |
| large cut | 70% (7/10) | 20% (2/10) | 13 | 1 / 0.4 |
| mine | 0% (0/10) | 100% (10/10) | - | 0 / 1.8 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 78% (7/9) | 0% (0/1) [n = 1] | 7 | 19 |
| HQ threat | for us | 0% (0/9) | 100% (1/1) [n = 1] | 1 | 21 [n = 1] |
| debt | for them | 11% (1/9) | 0% (0/1) [n = 1] | 1 | 19 [n = 1] |
| debt | for us | 0% (0/9) | 0% (0/1) [n = 1] | 0 | - |
| large cut | for them | 78% (7/9) | 0% (0/1) [n = 1] | 10 | 14.5 |
| large cut | for us | 11% (1/9) | 100% (1/1) [n = 1] | 4 | 15 [n = 4] |
| lead change | for them | 11% (1/9) | 0% (0/1) [n = 1] | 1 | 17 [n = 1] |
| lead change | for us | 11% (1/9) | 100% (1/1) [n = 1] | 2 | 14.5 [n = 2] |
| swing | for them | 100% (9/9) | 0% (0/1) [n = 1] | 20 | 15 |
| swing | for us | 22% (2/9) | 100% (1/1) [n = 1] | 9 | 17 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,1) T1(1,2) / M(2,2) M(2,1) / M(2,3) M(3,1)` | 4 [n = 4] | 75% (3/4) [n = 4] |
| `T1(1,1) T1(1,2) / M(2,2) M(1,0) / M(2,0) M(3,2)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,1) T1(1,2) / M(1,3) M(2,1) / M(2,2) M(2,3)` | 1 [n = 1] | 100% (1/1) [n = 1] |

**How its wins came** (9 wins):

Endings: the rules ended the game: 9.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 56% (5/9) | 13 |
| swing | 44% (4/9) | 11.5 [n = 4] |

### karliso (rank 3, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 10-0 |
| both | 10 | 10-0 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 8 / 8 |
| gold | 2 / 7 | 7.5 / 26.5 | 42.5 / 29.5 |
| income | 11 / 10 | 36 / 27 | 59 / 21 |
| cells | 15 / 13 | 40.5 / 29.5 | 54 / 33.5 |
| army | 40 / 30 | 70 / 65 | 95 / 85 |
| mines | 0 / 0 | 0 / 1 | 4 / 1 |
| towers | 0 / 0 | 2.5 / 1 | 5.5 / 2 |
| exposed30 | 0 / 0 | 1.5 / 3 | 2.5 / 3.5 |
| hq_gap | 128 / 143.5 | 97 / 91.5 | 61 / 92 |
| material | 135 / 119 | 377 / 305.5 | 575 / 355 |
| voronoi | 58 / 52 | 64.5 / 42 | 71.5 / 43 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 12.8 / 11.5 | 38.3 / 44.8 | 96.7 / 55.1 |
| income | 6.5 / 6 | 29.6 / 24.5 | 50.5 / 22.7 |
| cells | 9 / 7.7 | 33.5 / 26 | 51.4 / 30 |
| army | 24.9 / 21.5 | 60.7 / 60.8 | 94.4 / 79.1 |
| mines | 0 / 0.1 | 0.6 / 1.3 | 4 / 1.2 |
| towers | 0 / 0 | 2 / 0.6 | 4.7 / 2.4 |
| exposed30 | 0.1 / 0.1 | 3.1 / 3.4 | 4.5 / 3 |
| hq_gap | 153.5 / 168.5 | 76 / 87.2 | 31.3 / 87.7 |
| material | 93.4 / 81.8 | 341.8 / 289.6 | 595.2 / 346.6 |
| voronoi | 52 / 47.4 | 59.6 / 40.8 | 63.9 / 33.8 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| mine | 100% (10/10) | 90% (9/10) | 11 | 5.2 / 1.4 |
| tower | 100% (10/10) | 100% (10/10) | 7 | 6.8 / 3.7 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 17.1 / 16.3 |
| train level 2 | 100% (10/10) | 60% (6/10) | 13.5 | 2.9 / 1.7 |
| train level 3 | 90% (9/10) | 80% (8/10) | 16 | 2.6 / 1.7 |
| large cut | 60% (6/10) | 30% (3/10) | 15.5 | 1.2 / 0.4 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 90% (9/10) | - (0) | 9 | 16 |
| HQ threat | for us | 0% (0/10) | - (0) | 0 | - |
| debt | for them | 30% (3/10) | - (0) | 3 | 15 [n = 3] |
| debt | for us | 0% (0/10) | - (0) | 0 | - |
| large cut | for them | 60% (6/10) | - (0) | 12 | 16 |
| large cut | for us | 30% (3/10) | - (0) | 4 | 13.5 [n = 4] |
| lead change | for them | 10% (1/10) | - (0) | 2 | 28.5 [n = 2] |
| lead change | for us | 20% (2/10) | - (0) | 3 | 14 [n = 3] |
| swing | for them | 90% (9/10) | - (0) | 20 | 16 |
| swing | for us | 30% (3/10) | - (0) | 7 | 19 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,1) T1(1,2) / M(2,2) M(1,0) / M(2,0) M(3,2)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(0,2) T1(0,3) / M(1,2) M(1,3) / M(2,2) M(1,4)` | 1 [n = 1] | 100% (1/1) [n = 1] |
| `T1(0,2) T1(1,2) / M(0,3) M(2,2) / M(0,4) M(3,2)` | 1 [n = 1] | 100% (1/1) [n = 1] |

**How its wins came** (10 wins):

Endings: the rules ended the game: 10.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 40% (4/10) | 13.5 [n = 4] |
| swing | 40% (4/10) | 14 [n = 4] |
| HQ threat | 10% (1/10) | 19 [n = 1] |
| debt | 10% (1/10) | 15 [n = 1] |

### _iaz (rank 4, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 8-2 |
| both | 10 | 8-2 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 9 / 9 |
| gold | 2 / 7 | 5 / 22.5 | 5 / 4 |
| income | 11 / 10 | 30 / 24.5 | 26 / 24 |
| cells | 15 / 13 | 42 / 27.5 | 57 / 29 |
| army | 40 / 30 | 90 / 50 | 120 / 90 |
| mines | 0 / 0 | 0 / 1 | 0 / 1 |
| towers | 0 / 0 | 1.5 / 1 | 3 / 2 |
| exposed30 | 0 / 0 | 3 / 3 | 3 / 3 |
| hq_gap | 133.5 / 148.5 | 91.5 / 102.5 | 52 / 86 |
| material | 135 / 119 | 379 / 272 | 503 / 272 |
| voronoi | 57.5 / 51.5 | 72.5 / 44 | 80 / 37 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 12.6 / 11.9 | 33.5 / 42.8 | 39.9 / 50 |
| income | 6.6 / 6.1 | 23.1 / 24.6 | 28.5 / 20.8 |
| cells | 9.2 / 8 | 31 / 24.1 | 46.1 / 25.5 |
| army | 25.7 / 22.2 | 66.8 / 41.3 | 104.2 / 56.8 |
| mines | 0 / 0.1 | 0 / 1.2 | 0.9 / 1.2 |
| towers | 0 / 0 | 1.2 / 0.6 | 3 / 1.5 |
| exposed30 | 0.1 / 0 | 4 / 3.4 | 3.4 / 4.8 |
| hq_gap | 155.3 / 167.2 | 75.7 / 91 | 63.7 / 95.8 |
| material | 94.9 / 84.4 | 305.2 / 260.5 | 444.2 / 286.3 |
| voronoi | 52.2 / 47 | 60.8 / 38.4 | 65.9 / 33.4 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| tower | 100% (10/10) | 100% (10/10) | 8 | 4.9 / 3.4 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 15.7 / 20.3 |
| train level 2 | 90% (9/10) | 60% (6/10) | 9 | 4.2 / 1.5 |
| train level 3 | 80% (8/10) | 60% (6/10) | 13 | 2.3 / 1.7 |
| large cut | 70% (7/10) | 40% (4/10) | 13 | 2.2 / 0.5 |
| mine | 30% (3/10) | 100% (10/10) | 14 [n = 3] | 1.5 / 1.6 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 88% (7/8) | 0% (0/2) [n = 2] | 7 | 16 |
| HQ threat | for us | 0% (0/8) | 50% (1/2) [n = 2] | 1 | 22 [n = 1] |
| debt | for them | 13% (1/8) | 0% (0/2) [n = 2] | 1 | 15 [n = 1] |
| debt | for us | 0% (0/8) | 0% (0/2) [n = 2] | 0 | - |
| large cut | for them | 88% (7/8) | 0% (0/2) [n = 2] | 22 | 15 |
| large cut | for us | 25% (2/8) | 100% (2/2) [n = 2] | 5 | 17 |
| lead change | for them | 13% (1/8) | 0% (0/2) [n = 2] | 1 | 17 [n = 1] |
| lead change | for us | 13% (1/8) | 50% (1/2) [n = 2] | 2 | 14 [n = 2] |
| swing | for them | 100% (8/8) | 0% (0/2) [n = 2] | 33 | 15 |
| swing | for us | 38% (3/8) | 100% (2/2) [n = 2] | 14 | 15.5 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(0,2) T1(0,3) / M(1,3) M(1,2) / M(1,4) M(2,2)` | 1 [n = 1] | 100% (1/1) [n = 1] |
| `T1(0,2) T1(1,2) / M(1,1) M(0,3) / M(1,3) M(2,1)` | 1 [n = 1] | 100% (1/1) [n = 1] |
| `T1(0,2) T1(1,2) / M(2,2) M(0,3) / M(3,2) M(0,4)` | 1 [n = 1] | 100% (1/1) [n = 1] |

**How its wins came** (8 wins):

Endings: the rules ended the game: 8.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 50% (4/8) | 12 [n = 4] |
| swing | 38% (3/8) | 11 [n = 3] |
| debt | 13% (1/8) | 15 [n = 1] |

### reCurse (rank 5, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 7-3 |
| both | 10 | 7-3 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 | t20 |
|---|---:|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 8 / 8 | 5 / 5 |
| gold | 2 / 7 | 7.5 / 15.5 | 27 / 38.5 | 2 / 15 |
| income | 11 / 10 | 33 / 24.5 | 41 / 22.5 | 31 / 30 |
| cells | 15 / 13 | 40 / 29 | 47 / 37 | 51 / 48 |
| army | 40 / 30 | 80 / 75 | 100 / 95 | 100 / 120 |
| mines | 0 / 0 | 0.5 / 1 | 2 / 1 | 2 / 1 |
| towers | 0 / 0 | 1.5 / 1 | 3.5 / 2.5 | 4 / 3 |
| exposed30 | 0 / 0 | 3 / 4 | 3.5 / 3.5 | 3 / 4 |
| hq_gap | 128 / 144 | 98 / 86.5 | 85.5 / 69.5 | 130 / 36 |
| material | 135 / 119 | 366 / 291.5 | 479.5 / 415 | 529 / 539 |
| voronoi | 57 / 50.5 | 63.5 / 44 | 67.5 / 47 | 59 / 54 |

Left out: t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 12.5 / 11.1 | 37.7 / 42 | 73.2 / 55.9 |
| income | 6.2 / 5.7 | 27.1 / 24.4 | 38.8 / 22.3 |
| cells | 8.6 / 7.4 | 32.1 / 27.2 | 43.6 / 32.9 |
| army | 24.1 / 21.1 | 65.7 / 61.7 | 86.5 / 97.6 |
| mines | 0 / 0.1 | 0.6 / 1 | 1.9 / 1 |
| towers | 0 / 0 | 1.3 / 0.8 | 3.5 / 2 |
| exposed30 | 0 / 0 | 4.7 / 3.7 | 4.6 / 4.1 |
| hq_gap | 155.3 / 169.6 | 76.4 / 84.3 | 55.9 / 74.2 |
| material | 89.5 / 78.7 | 326 / 293.9 | 485.2 / 372.6 |
| voronoi | 52.4 / 47.1 | 58.6 / 40.8 | 58.5 / 38.5 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| mine | 100% (10/10) | 90% (9/10) | 10 | 3.9 / 2 |
| tower | 100% (10/10) | 100% (10/10) | 8 | 4.8 / 2.6 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 17.9 / 19.7 |
| train level 2 | 90% (9/10) | 90% (9/10) | 10 | 2.9 / 2.8 |
| train level 3 | 60% (6/10) | 50% (5/10) | 17 | 1.4 / 0.9 |
| large cut | 30% (3/10) | 20% (2/10) | 14 [n = 3] | 0.3 / 0.4 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 100% (7/7) | 0% (0/3) [n = 3] | 7 | 15 |
| HQ threat | for us | 0% (0/7) | 67% (2/3) [n = 3] | 2 | 23 [n = 2] |
| debt | for them | 0% (0/7) | 0% (0/3) [n = 3] | 0 | - |
| debt | for us | 0% (0/7) | 33% (1/3) [n = 3] | 1 | 21 [n = 1] |
| large cut | for them | 43% (3/7) | 0% (0/3) [n = 3] | 3 | 14 [n = 3] |
| large cut | for us | 0% (0/7) | 67% (2/3) [n = 3] | 4 | 17 [n = 4] |
| lead change | for them | 14% (1/7) | 0% (0/3) [n = 3] | 3 | 21 [n = 3] |
| lead change | for us | 14% (1/7) | 67% (2/3) [n = 3] | 5 | 16 |
| swing | for them | 86% (6/7) | 0% (0/3) [n = 3] | 8 | 15 |
| swing | for us | 29% (2/7) | 67% (2/3) [n = 3] | 12 | 19.5 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,1) T1(2,1) / M(2,2) M(1,2) / M(3,2) M(1,3)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,1) T1(2,1) / M(3,1) M(1,2) / M(4,1) M(1,3)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,1) T1(1,2) / M(1,3) M(2,1) / M(1,4) M(2,2)` | 1 [n = 1] | 0% (0/1) [n = 1] |

**How its wins came** (7 wins):

Endings: the rules ended the game: 7.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 43% (3/7) | 14 [n = 3] |
| swing | 29% (2/7) | 13.5 [n = 2] |
| HQ threat | 14% (1/7) | 14 [n = 1] |
| lead change | 14% (1/7) | 16 [n = 1] |

### _iahelka (rank 6, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 9-1 |
| both | 10 | 9-1 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 9 / 9 |
| gold | 2 / 7 | 11.5 / 19 | 11 / 31 |
| income | 11 / 10 | 31 / 28.5 | 27 / 17 |
| cells | 15 / 13 | 42.5 / 31 | 71 / 15 |
| army | 40 / 30 | 85 / 50 | 170 / 50 |
| mines | 0 / 0 | 0.5 / 1 | 1 / 1 |
| towers | 0 / 0 | 1.5 / 1 | 3 / 2 |
| exposed30 | 0 / 0 | 3 / 2.5 | 1 / 2 |
| hq_gap | 148 / 153 | 88 / 89.5 | 60 / 137 |
| material | 135 / 119 | 372.5 / 301.5 | 559 / 261 |
| voronoi | 53.5 / 54 | 63.5 / 47.5 | 84 / 18 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 13.3 / 12.8 | 43.8 / 42.5 | 48.9 / 55.1 |
| income | 7.2 / 7.1 | 27 / 26.1 | 32.5 / 21.3 |
| cells | 9.9 / 8.7 | 34 / 26 | 55.6 / 20.8 |
| army | 27 / 23.4 | 68.3 / 49.4 | 127.2 / 33.2 |
| mines | 0 / 0.2 | 0.6 / 1.3 | 1.6 / 1.3 |
| towers | 0 / 0 | 0.8 / 0.6 | 2.3 / 1.4 |
| exposed30 | 0 / 0 | 3 / 2.6 | 3.4 / 3.7 |
| hq_gap | 161.5 / 164.2 | 80.1 / 82.8 | 43.4 / 107.4 |
| material | 101.7 / 92.3 | 337.6 / 279.9 | 518.9 / 249.2 |
| voronoi | 49.6 / 50 | 56.4 / 43.7 | 74.2 / 24 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| tower | 100% (10/10) | 100% (10/10) | 9.5 | 3.2 / 3.8 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 14.4 / 16.7 |
| train level 3 | 100% (10/10) | 40% (4/10) | 13 | 2.2 / 1.2 |
| large cut | 80% (8/10) | 20% (2/10) | 13 | 1.8 / 0.2 |
| train level 2 | 80% (8/10) | 80% (8/10) | 11 | 3 / 1.3 |
| mine | 70% (7/10) | 100% (10/10) | 8 | 2.6 / 1.5 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 56% (5/9) | 0% (0/1) [n = 1] | 5 | 17 |
| HQ threat | for us | 0% (0/9) | 100% (1/1) [n = 1] | 1 | 21 [n = 1] |
| large cut | for them | 89% (8/9) | 0% (0/1) [n = 1] | 18 | 13 |
| large cut | for us | 11% (1/9) | 100% (1/1) [n = 1] | 2 | 14 [n = 2] |
| lead change | for them | 22% (2/9) | 0% (0/1) [n = 1] | 2 | 16.5 [n = 2] |
| lead change | for us | 22% (2/9) | 100% (1/1) [n = 1] | 3 | 12 [n = 3] |
| swing | for them | 100% (9/9) | 0% (0/1) [n = 1] | 31 | 15 |
| swing | for us | 33% (3/9) | 100% (1/1) [n = 1] | 4 | 16 [n = 4] |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,0) T1(0,2) / M(1,2) M(2,0) / M(2,2) M(2,1)` | 3 [n = 3] | 100% (3/3) [n = 3] |
| `T1(1,0) T1(1,1) / M(1,2) M(2,0) / M(2,2) M(2,1)` | 3 [n = 3] | 100% (3/3) [n = 3] |
| `T1(1,0) T1(2,0) / M(3,0) M(1,1) / M(3,1) M(1,2)` | 2 [n = 2] | 50% (1/2) [n = 2] |

**How its wins came** (9 wins):

Endings: the rules ended the game: 9.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| swing | 56% (5/9) | 12 |
| large cut | 44% (4/9) | 11.5 [n = 4] |

### Waffle3z (rank 7, 11 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 1 [n = 1] | 1-0 |
| seat 1 | 10 | 6-4 |
| both | 11 | 7-4 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 11 / 11 | 11 / 11 | 10 / 10 |
| gold | 2 / 7 | 10 / 25 | 11 / 5.5 |
| income | 11 / 10 | 36 / 27 | 45.5 / 25 |
| cells | 15 / 13 | 36 / 28 | 49.5 / 34 |
| army | 40 / 30 | 70 / 60 | 80 / 85 |
| mines | 0 / 0 | 2 / 2 | 2 / 1.5 |
| towers | 0 / 0 | 2 / 1 | 3 / 1.5 |
| exposed30 | 0 / 0 | 2 / 2 | 2.5 / 3 |
| hq_gap | 138 / 153 | 89 / 91 | 69 / 89 |
| material | 135 / 119 | 347 / 293 | 480.5 / 336.5 |
| voronoi | 57 / 52 | 66 / 42 | 62 / 40 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 11 / 11 | 11 / 11 | 11 / 11 |
| gold | 13.2 / 12 | 37.7 / 48 | 75.3 / 54.4 |
| income | 6.9 / 6.4 | 29.6 / 25.3 | 37.1 / 24.3 |
| cells | 9.3 / 8 | 31 / 25.5 | 37.5 / 31.8 |
| army | 25.2 / 22.1 | 60.4 / 49.5 | 69.7 / 69.3 |
| mines | 0 / 0.1 | 1.3 / 1.4 | 2.1 / 1.3 |
| towers | 0 / 0 | 1.1 / 0.6 | 2.2 / 2.2 |
| exposed30 | 0.1 / 0 | 3.6 / 2.9 | 3.2 / 4 |
| hq_gap | 157.7 / 167.8 | 78 / 85.1 | 51.3 / 76 |
| material | 96.3 / 85.3 | 321.9 / 281.1 | 427.7 / 345.4 |
| voronoi | 50.4 / 48.2 | 57 / 41.1 | 51.5 / 45.7 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| mine | 100% (11/11) | 82% (9/11) | 7 | 3.1 / 1.6 |
| tower | 100% (11/11) | 91% (10/11) | 8 | 3.9 / 2.7 |
| train level 1 | 100% (11/11) | 100% (11/11) | 1 | 15.1 / 15.6 |
| train level 2 | 100% (11/11) | 91% (10/11) | 10 | 3.2 / 1.7 |
| train level 3 | 55% (6/11) | 55% (6/11) | 16.5 | 1.8 / 1.4 |
| large cut | 36% (4/11) | 36% (4/11) | 13 [n = 4] | 0.5 / 1.4 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 86% (6/7) | 0% (0/4) [n = 4] | 6 | 16 |
| HQ threat | for us | 0% (0/7) | 75% (3/4) [n = 4] | 3 | 16 [n = 3] |
| debt | for them | 14% (1/7) | 0% (0/4) [n = 4] | 1 | 14 [n = 1] |
| debt | for us | 0% (0/7) | 0% (0/4) [n = 4] | 0 | - |
| large cut | for them | 43% (3/7) | 25% (1/4) [n = 4] | 6 | 13.5 |
| large cut | for us | 0% (0/7) | 100% (4/4) [n = 4] | 15 | 14 |
| lead change | for them | 0% (0/7) | 0% (0/4) [n = 4] | 0 | - |
| lead change | for us | 0% (0/7) | 75% (3/4) [n = 4] | 3 | 16 [n = 3] |
| swing | for them | 100% (7/7) | 0% (0/4) [n = 4] | 17 | 14 |
| swing | for us | 0% (0/7) | 100% (4/4) [n = 4] | 10 | 16 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(0,2) T1(1,0) / M(1,2) M(2,0) / M(2,1) M(2,2)` | 5 | 40% (2/5) |
| `T1(0,1) T1(0,2) / M(1,2) M(1,1) / M(2,1) M(2,2)` | 1 [n = 1] | 100% (1/1) [n = 1] |
| `T1(0,2) T1(1,2) / M(0,3) M(2,2) / M(1,3) M(3,2)` | 1 [n = 1] | 0% (0/1) [n = 1] |

**How its wins came** (7 wins):

Endings: the rules ended the game: 7.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| swing | 71% (5/7) | 11 |
| large cut | 29% (2/7) | 13.5 [n = 2] |

### Zylo (rank 8, 10 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 0 | 0-0 |
| seat 1 | 10 | 9-1 |
| both | 10 | 9-1 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 | t20 |
|---|---:|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 | 6 / 6 |
| gold | 2 / 7 | 4.5 / 28.5 | 23.5 / 63 | 26 / 24.5 |
| income | 11 / 10 | 32 / 25.5 | 44.5 / 26.5 | 31 / 24.5 |
| cells | 15 / 13 | 37 / 27.5 | 57.5 / 35 | 71 / 31 |
| army | 40 / 30 | 75 / 55 | 145 / 60 | 215 / 60 |
| mines | 0 / 0 | 1 / 1 | 2.5 / 1 | 3.5 / 1 |
| towers | 0 / 0 | 2 / 1 | 5 / 2.5 | 7 / 3.5 |
| exposed30 | 0 / 0 | 2 / 2 | 3 / 2.5 | 2 / 3 |
| hq_gap | 128.5 / 143.5 | 96 / 100.5 | 73 / 81 | 69.5 / 124.5 |
| material | 135 / 119 | 355.5 / 274 | 609 / 403.5 | 689.5 / 333.5 |
| voronoi | 57.5 / 51.5 | 66.5 / 47.5 | 67 / 43 | 81 / 31.5 |

Left out: t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 10 / 10 | 10 / 10 | 10 / 10 |
| gold | 12.6 / 12 | 39.7 / 55.8 | 85.2 / 71.6 |
| income | 6.6 / 6.1 | 30.3 / 25.2 | 40.8 / 23.4 |
| cells | 9.2 / 8 | 34.7 / 25.7 | 54.9 / 30.9 |
| army | 25.7 / 22.3 | 69.5 / 47.6 | 140.2 / 69.1 |
| mines | 0 / 0.1 | 1 / 1.1 | 2.6 / 1 |
| towers | 0 / 0 | 2.3 / 1 | 6 / 2.9 |
| exposed30 | 0.1 / 0 | 3 / 2.6 | 2.7 / 2.8 |
| hq_gap | 153.9 / 165.9 | 79.8 / 85.2 | 43.8 / 100.9 |
| material | 94.8 / 84.4 | 362.1 / 291.7 | 627.5 / 363.2 |
| voronoi | 51.7 / 47.6 | 58.6 / 40.6 | 63.7 / 36.5 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| mine | 100% (10/10) | 90% (9/10) | 8 | 3.4 / 1.1 |
| tower | 100% (10/10) | 100% (10/10) | 8 | 8.4 / 4.3 |
| train level 1 | 100% (10/10) | 100% (10/10) | 1 | 23.4 / 19.8 |
| train level 2 | 100% (10/10) | 90% (9/10) | 9.5 | 4.9 / 1.6 |
| train level 3 | 90% (9/10) | 40% (4/10) | 18 | 2.6 / 1.8 |
| large cut | 60% (6/10) | 20% (2/10) | 15.5 | 1 / 0.5 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 78% (7/9) | 0% (0/1) [n = 1] | 7 | 19 |
| HQ threat | for us | 0% (0/9) | 100% (1/1) [n = 1] | 1 | 21 [n = 1] |
| debt | for them | 22% (2/9) | 0% (0/1) [n = 1] | 2 | 27 [n = 2] |
| debt | for us | 0% (0/9) | 0% (0/1) [n = 1] | 0 | - |
| large cut | for them | 67% (6/9) | 0% (0/1) [n = 1] | 10 | 17 |
| large cut | for us | 11% (1/9) | 100% (1/1) [n = 1] | 5 | 19 |
| lead change | for them | 0% (0/9) | 0% (0/1) [n = 1] | 0 | - |
| lead change | for us | 0% (0/9) | 100% (1/1) [n = 1] | 1 | 16 [n = 1] |
| swing | for them | 78% (7/9) | 0% (0/1) [n = 1] | 26 | 19 |
| swing | for us | 44% (4/9) | 100% (1/1) [n = 1] | 8 | 19.5 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,1) T1(2,1) / M(3,1) M(1,2) / M(4,1) M(2,2)` | 5 | 100% (5/5) |
| `T1(1,1) T1(2,1) / M(2,2) M(1,2) / M(3,2) M(0,2)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,1) T1(2,1) / M(3,1) M(1,2) / M(3,2) M(2,2)` | 2 [n = 2] | 100% (2/2) [n = 2] |

**How its wins came** (9 wins):

Endings: the rules ended the game: 9.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| swing | 67% (6/9) | 13 |
| HQ threat | 22% (2/9) | 18.5 [n = 2] |
| large cut | 11% (1/9) | 15 [n = 1] |

### NinjaDoggy (rank 9, 12 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 2 [n = 2] | 2-0 |
| seat 1 | 10 | 8-2 |
| both | 12 | 10-2 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 12 / 12 | 11 / 11 | 11 / 11 |
| gold | 2 / 7 | 22 / 23 | 15 / 11 |
| income | 10 / 10 | 26 / 26 | 34 / 21 |
| cells | 14 / 13 | 35 / 28 | 43 / 30 |
| army | 40 / 30 | 60 / 50 | 70 / 70 |
| mines | 0 / 0 | 0 / 1 | 0 / 1 |
| towers | 0 / 0 | 2 / 1 | 3 / 2 |
| exposed30 | 0 / 0 | 3 / 2 | 2 / 4 |
| hq_gap | 138 / 143.5 | 72 / 92 | 58 / 105 |
| material | 128 / 119 | 320 / 260 | 414 / 296 |
| voronoi | 57.5 / 52.5 | 69 / 41 | 86 / 43 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 12 / 12 | 11 / 11 | 11 / 11 |
| gold | 12.6 / 12.5 | 42.8 / 43.2 | 62.6 / 40.1 |
| income | 6.6 / 6.5 | 24.6 / 24.7 | 28.6 / 15.2 |
| cells | 9.2 / 8.3 | 31.3 / 25.4 | 45.9 / 19.5 |
| army | 25.7 / 22.6 | 54.3 / 46.5 | 77.9 / 46.8 |
| mines | 0 / 0.1 | 0 / 1.1 | 0 / 1 |
| towers | 0 / 0 | 1.9 / 0.8 | 5 / 1.5 |
| exposed30 | 0.1 / 0 | 3.8 / 3.6 | 3 / 3.9 |
| hq_gap | 157.3 / 165 | 70.3 / 86.5 | 38.8 / 113 |
| material | 94.9 / 88 | 314.9 / 273.9 | 459.6 / 225.7 |
| voronoi | 53 / 48 | 60.8 / 37.2 | 73.2 / 24.8 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| train level 1 | 100% (12/12) | 100% (12/12) | 1 | 14.4 / 15.5 |
| tower | 92% (11/12) | 92% (11/12) | 8 | 6 / 3.2 |
| train level 2 | 92% (11/12) | 92% (11/12) | 9 | 2.8 / 2.1 |
| large cut | 83% (10/12) | 8% (1/12) | 13 | 1.9 / 0.2 |
| train level 3 | 83% (10/12) | 50% (6/12) | 14.5 | 1.6 / 0.9 |
| mine | 0% (0/12) | 83% (10/12) | - | 0 / 1.2 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 80% (8/10) | 0% (0/2) [n = 2] | 8 | 17.5 |
| HQ threat | for us | 0% (0/10) | 50% (1/2) [n = 2] | 1 | 23 [n = 1] |
| debt | for them | 10% (1/10) | 0% (0/2) [n = 2] | 1 | 18 [n = 1] |
| debt | for us | 0% (0/10) | 0% (0/2) [n = 2] | 0 | - |
| large cut | for them | 90% (9/10) | 50% (1/2) [n = 2] | 23 | 14 |
| large cut | for us | 0% (0/10) | 50% (1/2) [n = 2] | 3 | 13 [n = 3] |
| lead change | for them | 30% (3/10) | 0% (0/2) [n = 2] | 3 | 12 [n = 3] |
| lead change | for us | 0% (0/10) | 50% (1/2) [n = 2] | 1 | 14 [n = 1] |
| swing | for them | 80% (8/10) | 0% (0/2) [n = 2] | 22 | 16.5 |
| swing | for us | 20% (2/10) | 50% (1/2) [n = 2] | 8 | 20 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(1,0) T1(0,2) / M(1,2) M(1,1) / M(2,2) M(2,1)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,0) T1(2,0) / M(2,1) M(1,1) / M(2,2) M(1,2)` | 2 [n = 2] | 50% (1/2) [n = 2] |
| `T1(0,2) T1(0,3) / M(0,4) M(1,2) / M(0,5) M(2,2)` | 1 [n = 1] | 100% (1/1) [n = 1] |

**How its wins came** (10 wins):

Endings: the rules ended the game: 10.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 80% (8/10) | 13 |
| HQ threat | 10% (1/10) | 15 [n = 1] |
| swing | 10% (1/10) | 12 [n = 1] |

### aCat (rank 10, 13 games)

**Results against us**, by their seat (won-lost):

| their seat | games | won-lost |
|---|---:|---:|
| seat 0 | 3 [n = 3] | 3-0 |
| seat 1 | 10 | 5-5 |
| both | 13 | 8-5 |

**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).

| metric | t5 | t10 | t15 |
|---|---:|---:|---:|
| games | 13 / 13 | 13 / 13 | 10 / 11 |
| gold | 7 / 7 | 11 / 7 | 48 / 15 |
| income | 10 / 10 | 25 / 24 | 26.5 / 28 |
| cells | 13 / 13 | 32 / 30 | 41 / 39 |
| army | 30 / 30 | 70 / 70 | 95 / 110 |
| mines | 0 / 0 | 0 / 1 | 0 / 1 |
| towers | 0 / 0 | 0 / 1 | 1 / 3 |
| exposed30 | 0 / 0 | 3 / 3 | 3 / 2 |
| hq_gap | 131 / 143 | 83 / 103 | 97 / 91 |
| material | 121 / 119 | 302 / 293 | 394.5 / 371 |
| voronoi | 59 / 52 | 65 / 51 | 59.5 / 48 |

Left out: t20, t30, which fewer than half of the games reach.

**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.

| metric | early game | middle game | endgame |
|---|---:|---:|---:|
| games | 13 / 13 | 13 / 13 | 13 / 13 |
| gold | 12.4 / 12 | 41.8 / 38.2 | 68.2 / 45.2 |
| income | 5.9 / 6.2 | 21 / 23.9 | 23.6 / 27.1 |
| cells | 8.3 / 8.1 | 28.7 / 27.2 | 38.4 / 34.4 |
| army | 23.4 / 22.4 | 59.8 / 60.1 | 91.6 / 86 |
| mines | 0 / 0.1 | 0 / 0.9 | 0 / 1.1 |
| towers | 0 / 0 | 0.6 / 0.7 | 1.4 / 1.8 |
| exposed30 | 0.1 / 0 | 3.8 / 3 | 6.2 / 3.3 |
| hq_gap | 157.3 / 164.8 | 78.9 / 85.5 | 58.2 / 74.4 |
| material | 86.8 / 85.1 | 285.1 / 286.3 | 398.3 / 367.8 |
| voronoi | 54.2 / 48.6 | 58.4 / 44.5 | 59.3 / 45.7 |

**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.

| event | theirs | ours | first turn | per game |
|---|---:|---:|---:|---:|
| train level 1 | 100% (13/13) | 100% (13/13) | 1 | 14 / 18.7 |
| tower | 92% (12/13) | 92% (12/13) | 10.5 | 1.8 / 2.8 |
| train level 2 | 92% (12/13) | 85% (11/13) | 9.5 | 3.9 / 2 |
| train level 3 | 54% (7/13) | 31% (4/13) | 14 | 1.2 / 0.3 |
| large cut | 38% (5/13) | 31% (4/13) | 13 | 0.8 / 0.5 |
| mine | 0% (0/13) | 77% (10/13) | - | 0 / 1.2 |

**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.

| kind | favours | in wins | in losses | all | turn |
|---|---:|---:|---:|---:|---:|
| HQ threat | for them | 88% (7/8) | 0% (0/5) | 7 | 17 |
| HQ threat | for us | 0% (0/8) | 60% (3/5) | 3 | 16 [n = 3] |
| debt | for them | 13% (1/8) | 0% (0/5) | 1 | 21 [n = 1] |
| debt | for us | 0% (0/8) | 0% (0/5) | 0 | - |
| large cut | for them | 63% (5/8) | 0% (0/5) | 10 | 13.5 |
| large cut | for us | 13% (1/8) | 60% (3/5) | 7 | 14 |
| lead change | for them | 50% (4/8) | 0% (0/5) | 6 | 14.5 |
| lead change | for us | 50% (4/8) | 40% (2/5) | 6 | 13 |
| swing | for them | 75% (6/8) | 0% (0/5) | 16 | 15 |
| swing | for us | 0% (0/8) | 100% (5/5) | 15 | 14 |

**Openings**: their most common first 3 own plies.

| opening | games | win rate |
|---|---:|---:|
| `T1(0,2) T1(1,2) / M(2,2) M(0,3) / M(3,2) M(0,4)` | 2 [n = 2] | 50% (1/2) [n = 2] |
| `T1(0,2) T1(1,2) / M(2,2) M(1,2) / M(3,2) M(1,1)` | 2 [n = 2] | 100% (2/2) [n = 2] |
| `T1(1,0) T1(2,0) / M(2,1) M(1,1) / M(2,2) M(0,1)` | 2 [n = 2] | 100% (2/2) [n = 2] |

**How its wins came** (8 wins):

Endings: the rules ended the game: 8.

First moment favouring it in each win:

| kind | wins | median turn |
|---|---:|---:|
| large cut | 38% (3/8) | 12 [n = 3] |
| lead change | 25% (2/8) | 12.5 [n = 2] |
| swing | 25% (2/8) | 9.5 [n = 2] |
| HQ threat | 13% (1/8) | 15 [n = 1] |

## Our weaknesses

Our games only: 147 losses, 86 of them against the focus players.

Turns are compared with each side read right after its own ply of that turn (for its last ply, at its start); a turn at which either side has no ply is skipped.

Moments against us in our losses, by phase (early / middle / end), counted:

| kind | against | early | middle | end |
|---|---:|---:|---:|---:|
| HQ threat | others | 0 | 0 | 43 |
| HQ threat | focus | 0 | 0 | 72 |
| debt | others | 0 | 0 | 7 |
| debt | focus | 0 | 0 | 10 |
| large cut | others | 0 | 16 | 94 |
| large cut | focus | 0 | 25 | 90 |
| lead change | others | 0 | 6 | 12 |
| lead change | focus | 0 | 7 | 12 |
| swing | others | 0 | 20 | 182 |
| swing | focus | 0 | 22 | 175 |

Metrics of ours that stay below the opponent's from some turn to the end of our losses (for metrics where less is better, such as a gap or an exposure, read it the other way):

| metric | losses where below | median from turn |
|---|---:|---:|
| gold | 93% (136/147) | 16 |
| income | 95% (140/147) | 13 |
| cells | 95% (139/147) | 1 |
| army | 73% (107/147) | 12 |
| mines | 45% (66/147) | 12 |
| towers | 76% (112/147) | 11 |
| exposed30 | 37% (54/147) | 17 |
| hq_gap | 1% (2/147) | 13.5 [n = 2] |
| material | 99% (145/147) | 1 |
| voronoi | 94% (138/147) | 5 |

The 5 losses where the lead (`material`) fell behind for good earliest:

| game | opponent | from turn |
|---|---:|---:|
| 914253192 | FrancoisB | 1 |
| 914253194 | OccitanieFreelancer | 1 |
| 914267808 | DaFish | 1 |
| 914279047 | Drobor | 1 |
| 914294072 | Drobor | 1 |

## Games

Game ids by opponent, with our result (W, L, D).

- **zasmu** (10): 917607785 L, 917607947 L, 917608139 L, 917608288 L, 917608414 L, 917608618 W, 917608826 L, 917608907 L, 917608968 L, 917609212 L
- **Pduhard-** (10): 917642158 L, 917643731 L, 917645478 W, 917646585 L, 917649890 L, 917651183 L, 917652387 L, 917655500 L, 917657747 L, 917670774 L
- **karliso** (10): 917642388 L, 917643894 L, 917645592 L, 917646713 L, 917650014 L, 917651349 L, 917652529 L, 917655589 L, 917657802 L, 917671077 L
- **_iaz** (10): 917642566 W, 917644176 L, 917645736 L, 917646902 W, 917650068 L, 917651462 L, 917652705 L, 917655799 L, 917658016 L, 917671627 L
- **reCurse** (10): 917642733 W, 917644354 L, 917645840 L, 917648864 W, 917650244 L, 917651701 L, 917652945 L, 917656078 L, 917658214 L, 917671800 W
- **_iahelka** (10): 917642852 L, 917644489 L, 917645982 W, 917649050 L, 917650392 L, 917651783 L, 917653004 L, 917656251 L, 917658387 L, 917672319 L
- **Waffle3z** (11): 914253190 L, 917643076 L, 917644824 W, 917646172 L, 917649342 W, 917650589 W, 917651823 L, 917653219 W, 917656402 L, 917658585 L, 917672905 L
- **Zylo** (10): 917643294 L, 917644936 L, 917646278 L, 917649399 L, 917650763 L, 917651911 L, 917654833 L, 917656731 L, 917659129 L, 917673333 W
- **NinjaDoggy** (12): 914359832 L, 914372859 L, 917643422 L, 917645099 W, 917646371 L, 917649496 L, 917650846 L, 917652057 L, 917655096 L, 917656871 L, 917659289 L, 917673534 W
- **aCat** (13): 914336316 L, 914353947 L, 914372150 L, 917643529 W, 917645236 W, 917646438 W, 917649718 L, 917651136 W, 917652300 L, 917655380 L, 917657194 W, 917670330 L, 917673917 L
- **putibuzu** (5): 914344341 L, 914361838 L, 914363816 L, 914367188 L, 914371199 L
- **N3l** (7): 914319980 W, 914330521 W, 914331621 L, 914337751 L, 914338811 L, 914346609 L, 914385169 L
- **eulerscheZahl** (7): 914335161 L, 914336816 W, 914340917 L, 914352254 W, 914363383 L, 914364818 L, 914368693 W
- **ValGrowth** (10): 914253191 L, 914311503 W, 914329124 L, 914331025 L, 914333129 L, 914348641 W, 914349608 W, 914362403 W, 914371697 L, 914378504 L
- **AlesRamsa** (4): 914326354 L, 914355150 L, 914373444 W, 914374368 W
- **bonebone** (6): 914317925 W, 914326132 W, 914337225 L, 914369011 W, 914370041 L, 914381770 W
- **Azkellas** (4): 914317611 L, 914322816 L, 914329684 W, 914362778 L
- **ArturBieniek** (8): 914306542 W, 914316350 W, 914317249 W, 914332824 W, 914355837 W, 914357527 L, 914360608 W, 914374916 L
- **siman** (9): 914322276 L, 914323795 W, 914334586 W, 914345448 W, 914351520 W, 914360116 W, 914364515 W, 914377464 L, 914385600 W
- **_Royale** (5): 914305857 W, 914321173 W, 914345807 W, 914351112 L, 914382264 W
- **kovi** (5): 914310557 L, 914319080 L, 914350809 W, 914359559 L, 914384830 W
- **wala** (6): 914300659 W, 914309255 W, 914339364 L, 914342654 L, 914380319 L, 914380730 L
- **FrancoisB** (7): 914253192 L, 914308563 W, 914311792 L, 914332147 W, 914358259 L, 914370422 W, 914376397 W
- **Gabbek** (8): 914307579 L, 914308944 W, 914333422 W, 914347791 W, 914348275 W, 914353647 W, 914357893 W, 914386214 W
- **Neumann** (7): 914293343 W, 914312922 W, 914313582 W, 914328126 L, 914334191 W, 914340582 W, 914343665 W
- **KnightMoves** (1): 914289658 W
- **aropan** (3): 914299529 W, 914304948 W, 914309907 W
- **kozuechan-AI** (2): 914301831 W, 914302677 L
- **MSmits** (2): 914298033 W, 914301138 W
- **BorisZ** (1): 914289219 W
- **Fangel** (1): 914303945 W
- **IloIlo** (2): 914289952 W, 914295422 W
- **Magus** (3): 914286769 L, 914294489 W, 914299065 L
- **Nerchio** (1): 914290877 W
- **Drobor** (4): 914253193 W, 914279047 L, 914294072 L, 914295662 W
- **dbdr** (1): 914285010 W
- **tanakat01** (1): 914290357 W
- **OccitanieFreelancer** (2): 914253194 L, 914285271 L
- **Marchete** (1): 914288627 W
- **Petras** (1): 914280761 L
- **PED** (1): 914284318 W
- **TylerDurden** (1): 914285987 W
- **ThomasNicoullaud** (2): 914283076 L, 914284614 W
- **k4ng0u** (1): 914276247 W
- **Khao** (1): 914268661 L
- **DaFish** (1): 914267808 L
- **cedricdd** (2): 914253195 W, 914271861 L
- **mikmak** (1): 914282527 W
- **wlesavo** (1): 914268947 L
- **kiwijam** (1): 914279863 W
- **DuckTypo** (3): 914259657 W, 914272549 L, 914275803 W
- **CodingGigachad** (2): 914258404 W, 914270986 W
- **tutubalin** (2): 914273354 W, 914274457 W
- **daaskare** (1): 914272846 W
- **egaetan** (1): 914253196 L
- **Samsonov** (1): 914269832 W
- **yurp** (1): 914274136 W
- **chris.steele** (1): 914256924 W
- **xurei** (1): 914253197 W
- **Cramer** (1): 914255738 W
- **CrystalRushLearning** (2): 914253198 W, 914254769 W
- **YALOKGAR** (15): 914188729 W, 914189555 W, 914194025 W, 914199573 W, 914201943 W, 914203406 W, 914212978 W, 914218236 W, 914219979 W, 914229272 W, 914229804 W, 914233470 W, 914239084 W, 914245917 W, 914247409 W
- **HKit** (8): 914214068 W, 914217131 W, 914217821 W, 914221449 W, 914224100 W, 914227049 W, 914245586 W, 914252438 W
- **Fenz** (4): 914192790 W, 914226601 W, 914230306 W, 914248521 W
- **TwT** (1): 914185946 W
- **dolmen1234** (1): 914185130 W
- **-Rutger-** (1): 914178797 W
- **Cize** (2): 914178791 W, 914184594 W
- **Daenerys Bossgaryen** (18): 914187268 L, 914195163 W, 914195396 W, 914197457 W, 914198067 W, 914200904 W, 914204236 W, 914206113 W, 914210580 W, 914215884 W, 914216834 W, 914219725 W, 914224416 W, 914230693 L, 914235900 W, 914240811 W, 914242179 W, 914244643 W
- **DirtyHarry** (1): 914178799 W
- **Eris_C** (1): 914178795 W
- **Fantomat75** (1): 914183215 W
- **Gonny** (1): 914178793 W
- **ItsNotABug** (1): 914180848 W
- **IvanBot** (1): 914178792 W
- **Michael_Howard** (1): 914178794 W
- **PonyPonyCodeCode** (1): 914187991 W
- **Seti** (1): 914178796 W
- **Unknown** (1): 914261027 W
- **YurkovAS** (1): 914181465 W
- **cg123** (1): 914178798 W
- **r3versi** (1): 914181954 W
- **vinzz** (1): 914178800 W
