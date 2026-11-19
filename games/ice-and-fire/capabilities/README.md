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
| `wall` | zasmu was P and built a tower on or next to one of its frontier cells | O's gold at its turn reaches no more of P's active cells than after zasmu's own answer |

An answer that takes O's HQ passes every test. A position can test several capabilities (a `save` and a `wall`, say); each is a test of its own, with the same answer.

What the properties leave out:

- **They judge one ply.** A save test does not check that P strikes later, and a defend test does not check that O's chain stays out of reach after O's reply. "The HQ taken within k turns in a scripted continuation" (the plan's third kind of property) is not in v1.
- **Passing is not always right.** Hoarding gold passes every save test. A save position may call for spending first, for example to block O's advance. So a save rate is read beside the defend rate, and beside the rate of the players' own answers. The plan's later stages need tests of when *not* to save.
- **The wall test needs zasmu's answer.** So it only holds positions where zasmu moved, all in games against us. Its pass rule compares an effect, so an answer that kills the attackers rather than building a tower also passes.

## The suite

[`suite-v1.tsv`](suite-v1.tsv) has 600 tests: up to 100 per capability, at most 2 per game, drawn at random (seed 20261119) from the 6,550 games of the data branch `claude/codingame-games` at f137bcd that our engine replays as CodinGame did (4 others were left out). Candidates by capability: finish 5,001; save-1 15,132; save-2 32,108; defend-1 10,428; defend-2 27,474; wall 3,012. Each line is a test: capability, CodinGame game id, ply (from 0, in the replayed game), game turn, and the player who answered in the game. The suite is frozen, so scores can be compared across bots. A new suite is a new file (`suite-v2.tsv`), never an edit of this one.

Of the 600 tests, 243 were answered in the game by us (`Tarleya`), 130 by zasmu (100 of them the wall tests), and the rest by 19 other players.

## Running it

```sh
cargo run --release -p ice-and-fire-review -- codingame-capability-suite --data DATA --me Tarleya [--reference zasmu] [--per 100] [--per-game 2] [--seed 20261119] [--jobs 4] > suite.tsv
cargo run --release -p ice-and-fire-review -- codingame-capabilities --data DATA --me Tarleya --suite games/ice-and-fire/capabilities/suite-v1.tsv --bot NAME=BINARY [--bot ...] [--tries 600] [--csv FILE] [--jobs 4] > capabilities.md
```

`DATA` is the `ice-and-fire` folder of the data branch (for example a worktree: `git worktree add --detach /tmp/data origin/claude/codingame-games`, then `--data /tmp/data/ice-and-fire`). `codingame-capabilities` checks that every test of the suite still stands on the data. Each bot sits in the mover's seat and is sent the game's own inputs, from the initial input and its first turn up to the test's ply. Its answers before that ply are ignored, so its game turn and its memory are as in a real game. It answers with `CG_FIXED_ITERS` (600 tries by default, as `codingame-judge`; 0 for the bot's own clock), so a run can be repeated. A bot that does not answer fails the test, and the report lists it. One bot process serves all the tests of one game and seat. A run of the 600 tests takes about 5 minutes per plan-bot release on 4 cores. Two runs give the same verdicts (60 tests of v028 repeated, 0 differences).

The report has one row per capability: each bot's pass rate, then the rate of the answers the players sent in the games (all of them, zasmu's, ours, the others'). `--csv` writes each test's verdicts.

## Results

Suite v1, 2026-11-19, `--tries 600`, the releases compiled from `releases/` with `rustc --edition 2021 -O` (1.90.0). Each test's verdicts are in [`results-v1-2026-11-19.csv`](results-v1-2026-11-19.csv). "Played" is the answers the players sent in the games. In `Played by zasmu`, the save and defend columns rest on 2 to 9 tests, too few to read.

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
