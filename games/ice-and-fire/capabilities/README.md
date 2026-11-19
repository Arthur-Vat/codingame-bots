# Capability tests

The first piece of stage A ("an outside judge") of the curriculum in [research/codingame/2026-11-18-zasmu-strategy.md](../research/codingame/2026-11-18-zasmu-strategy.md). Each test is a position from the CodinGame games, plus a property of the answer that says whether the bot made the decision the position calls for. A bot's score is the share of tests it passes, per capability. The tests come from games, not from self-play, so they can show that a capability is missing even when an SPRT against our last release accepts.

The owner chose the capabilities and the wall rule on 2026-11-19 (form): finish, save, defend and wall, with a wall test passing when the answer leaves no more of the side's land exposed than zasmu's own answer.

## The capabilities

P is the side to act and O its opponent. A side's **chain** is its cheapest chain of trainings to the enemy HQ, with its units stepping first ([`tactics/src/chain.rs`](../tactics/src/chain.rs)). A side's gold and income are those shown at the start of the ply, so "in k turns" means gold plus k incomes as they stand.

| Capability | Position | Passes if, after the answer |
| --- | --- | --- |
| `finish` | P can pay its chain now | P took O's HQ |
| `save-1`, `save-2` | P cannot pay its chain now but can with k turns of income (the fewest that do, k ≤ 2), and O cannot pay its own chain at its next turn | P's gold plus k incomes still pays P's chain, recomputed on the new state (moves that make it cheaper count) |
| `defend-1`, `defend-2` | O can pay its chain at its k-th next turn (the first that does, k ≤ 2), and P cannot pay its own now | O's gold at its turn plus k − 1 incomes no longer pays O's chain, recomputed on the new state |
| `wall` | zasmu was P and built a tower on or next to one of its frontier cells, and P cannot pay its chain now | O's gold at its turn reaches no more of P's active cells than after zasmu's own answer |

An answer that takes O's HQ passes every test. A position can test several capabilities (a `save` and a `wall`, say); each is a test of its own, with the same answer.

What the properties leave out:

- **They judge one ply.** A save test does not check that P strikes later, and a defend test does not check that O's chain stays out of reach after O's reply. "The HQ taken within k turns in a scripted continuation" (the plan's third kind of property) is not in v1.
- **Passing is not always right.** Hoarding gold passes every save test. A save position may call for spending first, for example to block O's advance. So a save rate is read beside the defend rate, and beside the rate of the players' own answers. The plan's later stages need tests of when *not* to save.
- **The wall test needs zasmu's answer.** So it only holds positions where zasmu moved, all in games against us. Its pass rule compares an effect, so an answer that kills the attackers rather than building a tower also passes.

## The suite

[`suite-v1.tsv`](suite-v1.tsv) has 600 tests: up to 100 per capability, at most 2 per game, drawn at random (seed 20261119) from the 6,550 games of the data branch `claude/codingame-games` at f137bcd that our engine replays as CodinGame did (4 others were left out). Candidates by capability: finish 5,001; save-1 15,132; save-2 32,108; defend-1 10,428; defend-2 27,474; wall 3,012. Each line is a test: capability, CodinGame game id, ply (from 0, in the replayed game), game turn, and the player who answered in the game. The suite is frozen, so scores can be compared across bots. A new suite is a new file, never an edit of an existing one.

Of the 600 tests, 243 were answered in the game by us (`Tarleya`), 130 by zasmu (100 of them the wall tests), and the rest by 20 other players.

[`suite-v2-zasmu.tsv`](suite-v2-zasmu.tsv) (owner, 2026-11-19: suite v1 drew too few of zasmu's save and defend positions to read) holds only positions that zasmu answered, drawn from the same games with the same rules and seed (`--mover zasmu`): 600 tests from 367 games, 100 per capability. zasmu's candidates: finish 451; save-1 1,019; save-2 3,553; defend-1 105 (in 79 games: we seldom threaten it); defend-2 667; wall 3,012. On it, our bots answer the very positions zasmu answered, all from games against us, so they are compared with zasmu on equal terms.

## Running it

```sh
cargo run --release -p ice-and-fire-review -- codingame-capability-suite --data DATA --me Tarleya [--reference zasmu] [--per 100] [--per-game 2] [--seed 20261119] [--mover PLAYER] [--jobs 4] > suite.tsv
cargo run --release -p ice-and-fire-review -- codingame-capabilities --data DATA --me Tarleya --suite games/ice-and-fire/capabilities/suite-v1.tsv --bot NAME=BINARY [--bot ...] [--tries 600] [--csv FILE] [--jobs 4] > capabilities.md
```

To read a submitted release's own answers in its ranking games, draw every candidate of its games and score no bot: the column "Played by Tarleya" is then the release's pass rates, and the other columns its opponents'.

```sh
cargo run --release -p ice-and-fire-review -- codingame-capability-suite --data DATA --me Tarleya --kinds ranking --release ice-and-fire-v028 --per 1000000 --per-game 1000000 > v028.tsv
cargo run --release -p ice-and-fire-review -- codingame-capabilities --data DATA --me Tarleya --suite v028.tsv [--csv FILE] > v028.md
```

`DATA` is the `ice-and-fire` folder of the data branch (for example a worktree: `git worktree add --detach /tmp/data origin/claude/codingame-games`, then `--data /tmp/data/ice-and-fire`). `codingame-capabilities` checks that every test of the suite still stands on the data. Each bot sits in the mover's seat and is sent the game's own inputs, from the initial input and its first turn up to the test's ply. Its answers before that ply are ignored, so its game turn and its memory are as in a real game. It answers with `CG_FIXED_ITERS` (600 tries by default, as `codingame-judge`; 0 for the bot's own clock), so a run can be repeated. A bot that does not answer fails the test, and the report lists it. One bot process serves all the tests of one game and seat. A run of the 600 tests takes about 5 minutes per plan-bot release on 4 cores. Two runs give the same verdicts (60 tests of v028 repeated, 0 differences).

The report has one row per capability: each bot's pass rate, then the rate of the answers the players sent in the games (all of them, zasmu's, ours, the others'). `--csv` writes each test's verdicts.

## Results

Suite v1, 2026-11-19, `--tries 600`, the releases compiled from `releases/` with `rustc --edition 2021 -O` (1.90.0). Each test's verdicts are in [`results-v1-2026-11-19.csv`](results-v1-2026-11-19.csv). "Played" is the answers the players sent in the games. In `Played by zasmu`, the save and defend columns rest on 0 to 9 tests, too few to read: suite v2 below answers that.

| Capability | Tests | v028 | v027 | v026 | v001 | Played, all | Played by zasmu | Played by Tarleya | Played by others |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| finish | 100 | 100% | 99% | 99% | 100% | 100% (100/100) | 100% (12/12) | 100% (36/36) | 100% (52/52) |
| save-1 | 100 | 42% | 44% | 44% | 88% | 57% (57/100) | 78% (7/9) | 51% (26/51) | 60% (24/40) |
| save-2 | 100 | 65% | 65% | 65% | 42% | 60% (60/100) | 29% (2/7) | 58% (21/36) | 65% (37/57) |
| defend-1 | 100 | 66% | 67% | 67% | 68% | 58% (58/100) | - | 56% (31/55) | 60% (27/45) |
| defend-2 | 100 | 77% | 76% | 76% | 29% | 60% (60/100) | 50% (1/2) | 62% (40/65) | 58% (19/33) |
| wall | 100 | 63% | 57% | 51% | 25% | 100% (wall reference) | 100% (100/100) | - | - |

What they say:

- **Finish works.** Every bot takes the HQ when its chain is affordable; v026 and v027 miss one test each.
- **Our search spends the strike it could make next turn.** On save-1, v026 to v028 keep the strike affordable in 42 to 44% of the tests. v001 (`banker`) does in 88%: its one rule is to keep gold for a winning chain affordable next turn. This is the gap stage B is for. A save rate needs the defend rate beside it, since hoarding passes every save test. v001's 29% on defend-2 shows what that rule costs.
- **Defence is the same in v026 to v028**: 66 to 67% on defend-1 and 76 to 77% on defend-2. Some defend-1 positions may have no defence at all; the players' own answers pass 58%.
- **The wall test sees E068.** The wall rate rises from v026 (51%) to v027 (57%) and to v028 (63%). E068 is v028's tower wall term; v027 changed who plays turn 7. So the test measures the behaviour E068 aimed at, without self-play. It still sits well below zasmu's own answers, which pass by construction.

### Suite v2: zasmu's positions

Suite v2, 2026-11-19, the same settings and binaries as above. Each test's verdicts are in [`results-v2-zasmu-2026-11-19.csv`](results-v2-zasmu-2026-11-19.csv). Every position was answered by zasmu in a game against us (the data branch holds only our own games); the bots answer from zasmu's seat.

| Capability | Tests | v028 | v027 | v026 | v001 | zasmu |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| finish | 100 | 100% | 100% | 100% | 100% | 100% |
| save-1 | 100 | 51% | 50% | 50% | 88% | 71% |
| save-2 | 100 | 69% | 75% | 75% | 55% | 43% |
| defend-1 | 100 | 78% | 79% | 79% | 78% | 81% |
| defend-2 | 100 | 96% | 96% | 96% | 50% | 62% |
| wall | 100 | 51% | 46% | 44% | 19% | 100% (reference) |

Over every candidate position of the data, not only the suites' draws, the players' own answers pass these shares (positions in brackets; each player's own positions, so the columns are not the same positions). They come from a one-off run, not from a committed file; to reproduce them, draw every candidate (`codingame-capability-suite --per 1000000 --per-game 1000000`, wall tests left out) and score any bot on it: the report's "Played" columns are these rates.

| Capability | zasmu | us (`Tarleya`) | Other players |
| --- | ---: | ---: | ---: |
| finish | 100% (451) | 100% (1,851) | 99% (2,699) |
| save-1 | 76% (1,019) | 53% (7,349) | 70% (6,764) |
| save-2 | 48% (3,553) | 62% (11,981) | 65% (16,574) |
| defend-1 | 82% (105) | 48% (5,854) | 59% (4,469) |
| defend-2 | 69% (667) | 62% (15,958) | 51% (10,849) |

What suite v2 adds:

- **Saving one turn ahead is the gap, on zasmu's own positions too.** zasmu keeps a strike affordable next turn in 71% of these tests (76% over all its positions). v026 to v028 do in 50 to 51%.
- **zasmu does not save two turns ahead.** On save-2 it passes 43% (48% over all its positions), less than our releases (69 to 75%) and less than every other group. It spends until about two turns before the strike and then banks, as the study found ("The strike"). So save-2 is not a target in itself; stage B should aim at save-1 and at when to start saving.
- **Defence against a strike one turn away is level with zasmu**, on positions where it was threatened: 78 to 79% against 81%. Against a threat two turns away our releases defend in 96% of the tests, zasmu in 62%. zasmu leads in most of these games, so it may afford to ignore a distant threat; whether our 96% costs us elsewhere is not something a one-ply test can tell.
- **The wall rate rises the same way as on suite v1**: v026 44%, v027 46%, v028 51%.

### Submitted releases, in their ranking games

The owner, 2026-11-19: no test batches against the top players; the capability tests read the games each submitted release played on CodinGame instead. Data branch at e3b20fa (v028's 159 ranking games fetched by the collection that day; one replays differently and is left out), every candidate of each release's ranking games, no bot (the commands above). These are the releases' own answers in real games, so the positions differ between releases, and so do the opponents: a rate moves with the opponents a release met as well as with the bot. The wall test has no row: it needs zasmu's answer to the same position. Releases with fewer than 50 ranking games on the data branch (v016, v021, v022) are left out.

| Release | Ranking games | finish | save-1 | save-2 | defend-1 | defend-2 | Opponents: save-1 | Opponents: defend-1 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| v014 | 227 | 97% (127) | 44% (402) | 70% (418) | 45% (111) | 65% (282) | 72% (127) | 42% (213) |
| v018 | 108 | 96% (47) | 42% (181) | 60% (231) | 47% (81) | 60% (190) | 65% (101) | 46% (85) |
| v019 | 159 | 100% (64) | 64% (188) | 66% (226) | 44% (119) | 63% (284) | 76% (146) | 57% (145) |
| v020 | 126 | 100% (58) | 66% (134) | 69% (167) | 38% (77) | 59% (190) | 71% (100) | 46% (106) |
| v023 | 159 | 100% (72) | 63% (192) | 55% (312) | 46% (94) | 66% (306) | 69% (131) | 49% (139) |
| v024 | 83 | 100% (25) | 62% (106) | 65% (185) | 47% (66) | 64% (186) | 65% (86) | 68% (77) |
| v025 | 159 | 100% (64) | 64% (188) | 63% (355) | 47% (94) | 67% (357) | 64% (146) | 56% (146) |
| v026 | 53 | 100% (19) | 63% (54) | 62% (93) | 47% (49) | 70% (126) | 75% (55) | 52% (40) |
| v028 | 158 | 99% (74) | 55% (286) | 62% (418) | 52% (83) | 69% (287) | 66% (112) | 56% (168) |

Rates are of the release's own positions (in brackets); the opponents' columns are its opponents' answers in the same games.

Beside them, `codingame-style --kinds ranking --release ice-and-fire-v028` (and v025's for comparison), against the top 6 of the newest leaderboard snapshot in the same games:

| Measure (mean) | v028 red / blue | v025 red / blue | top 6 against v028, red / blue |
| --- | --- | --- | --- |
| Towers built a game | 8.8 / 7.4 | 5.6 / 5.2 | 6.5 / 5.8 |
| Towers on the board at turn 15 | 3.9 / 3.8 | 2.9 / 2.7 | 2.8 / 3.0 |
| Frontier cells protected by a tower at turn 15 | 0.67 / 0.60 | 0.45 / 0.47 | 0.54 / 0.45 |
| Voronoi share at turn 10 | 0.41 / 0.43 | 0.39 / 0.41 | 0.52 / 0.56 |
| Moves a ply at turns 10-12 | 2.9 / 2.9 | 3.0 / 2.2 | 4.6 / 4.8 |

What they say:

- **v028 builds the wall on CodinGame.** It builds more towers than v025 and than the top 6 (8.8 and 7.4 a game) and guards more of its frontier at turn 15 (0.67 and 0.60 of its frontier cells). E068 aimed at this; the suites saw it too (wall rate 51% against v026's 44% on suite v2).
- **v028 saves a strike one turn ahead less often than the releases before it**: 55% of its 286 save-1 positions, against 62 to 66% for v019 to v026 (and 72 to 76% for zasmu in the suites). The towers spend gold that a strike would need: a likely cost of the wall term, and the gap stage B is for. Its defence against a strike one turn away is its best (52%), within the noise of 83 positions.
- **The opening is unchanged**: Voronoi share at turn 10 and moves a ply at turns 10-12 stay well below the top 6, as before E068.

