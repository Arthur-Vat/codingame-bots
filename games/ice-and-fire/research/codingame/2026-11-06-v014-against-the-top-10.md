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

## Data

The numbers, charts and findings behind this report are in the folder [2026-11-06-v014-against-the-top-10/](2026-11-06-v014-against-the-top-10/):

- [README.md](2026-11-06-v014-against-the-top-10/README.md): every file and its columns;
- [signals.md](2026-11-06-v014-against-the-top-10/signals.md): the strongest findings, picked by code, each with its chart and data file;
- the tables as CSV: [sources](2026-11-06-v014-against-the-top-10/sources.csv), [opponents](2026-11-06-v014-against-the-top-10/opponents.csv), [profile metrics at turns](2026-11-06-v014-against-the-top-10/profile_metrics_turn.csv) and [by phase](2026-11-06-v014-against-the-top-10/profile_metrics_phase.csv), [events](2026-11-06-v014-against-the-top-10/profile_events.csv), [moments](2026-11-06-v014-against-the-top-10/profile_moments.csv), [openings](2026-11-06-v014-against-the-top-10/profile_openings.csv), [wins](2026-11-06-v014-against-the-top-10/profile_wins.csv), [our weaknesses](2026-11-06-v014-against-the-top-10/weaknesses_stays_below.csv), [the AUC by turn](2026-11-06-v014-against-the-top-10/separation.csv), [spending](2026-11-06-v014-against-the-top-10/spending.csv), [games](2026-11-06-v014-against-the-top-10/games.csv);
- every metric and event turn by turn: [all top players](2026-11-06-v014-against-the-top-10/turns-all-top-players.csv), [our wins](2026-11-06-v014-against-the-top-10/turns-our-wins.csv), [our losses](2026-11-06-v014-against-the-top-10/turns-our-losses.csv) and one file for each top player (`turns-NAME.csv`);
- 62 SVG charts (`top-vs-us-*`, `wins-vs-losses-*`, `player-NAME-*`), for example [income, our wins against our losses](2026-11-06-v014-against-the-top-10/wins-vs-losses-income.svg).

Made by (about 6 seconds on 4 cores), from a worktree of the data branch:

```sh
ice-and-fire-review codingame-report --data <claude/codingame-games>/ice-and-fire --me Tarleya --out games/ice-and-fire/research/codingame/2026-11-06-v014-against-the-top-10
```
