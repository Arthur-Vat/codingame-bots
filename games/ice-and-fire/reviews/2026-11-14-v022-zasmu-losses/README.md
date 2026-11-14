# v022's losses to zasmu, reviewed

Twenty CodinGame games that `ice-and-fire-v022` ([release](../../releases/ice-and-fire-v022.rs)) lost to zasmu, first of the ladder on 2026-11-14, reviewed by Haiku agents (`game-reviewer`, ADR 0043) and summarized by one (`review-summarizer`). Asked by the owner on 2026-11-14: "Analyze lost games for both seats against top bots for the latest release. You can do a haiku review of 20 lost games against the top 1 bot, ideally 10 with each seat."

- [summary.md](summary.md): the summarizer's reading of the batch, saved unchanged; its count of the kinds of games (10 ranking, 2 test, 8 replay) is wrong: the batch holds 11 ranking, 3 test and 6 replay games (below).
- [summary-data.md](summary-data.md): the computed statistics (`ice-and-fire-review summary`).
- `NAME.review.md`: each game's review; `NAME.csv` and `NAME.summary.json`: its metrics and facts; `records/NAME.json`: its record.

## The games

v022 lost every one; `NAME` gives v022's seat (red moves first), the kind of game and CodinGame's game id.

| Seat | Kind | Games | Source |
| --- | --- | ---: | --- |
| red | ranking | 7 | every ranking loss of v022 to zasmu as red (submission 41710997, random maps) |
| red | test | 3 | 3 of v022's 5 test-game losses to zasmu (collection batch `2026-11-14-ice-and-fire-v022-daily`, fixed maps of `codingame-seeds.txt`) |
| blue | ranking | 4 | every ranking loss of v022 to zasmu as blue |
| blue | replay | 6 | test games replaying, with v022 seated blue, the maps of v022's 5 blue ranking games against zasmu (2 each, 10 in all: 8 lost, 2 won, both on the map of v022's one blue win); the 6 losses on the 4 other maps |

A test game puts the tested bot in the first seat; a replay (`replay_game_id`) keeps the seats of the game it replays, so it is the way to play blue against a chosen player. The 10 replays were played on 2026-11-14 through the API, ids 921451652 to 921452818, and stored with `scripts/codingame.sh fetch-ids --kind replay` in a local data directory, not on `claude/codingame-games`.

## How to rebuild

With the review tool at `f81e682`, from the logs of the games (on `claude/codingame-games`, or fetched with `scripts/codingame.sh fetch-ids`):

```sh
target/release/ice-and-fire-review codingame --kind ranking --out REC LOGS/<id>.json ...   # and --kind test, --kind replay
# our player renamed to v022 in each record, files named SEAT-KIND-ID.json into records/
target/release/ice-and-fire-review games --out BATCH --text SCRATCH/text BATCH/records/*.json
target/release/ice-and-fire-review summary BATCH > BATCH/summary-data.md
```

Our engine agreed with CodinGame on every command of the 20 games (894 plies).

## v022 against the top 10, by seat

For context, from a scratch scan of all 150 of v022's games against the top 10 of the 2026-11-14 leaderboard (ranking and test games): v022 won 21 of 98 as red and 21 of 52 as blue. The decisive moment of a loss (`codingame-moments`) was a large cut (median turn 16 as red, 15 as blue) or an HQ threat (median turn 20) in nearly all of them.

Each side's play by seat and result ("top 10" is the opponent in the same games; towers are counted at turns 10 to 20):

| Seat | Result | Side | Games | First mine (median turn) | Level-3 trainings to turn 20 (mean) | Gold held, turns 10-20 (median) | Spent a turn, turns 10-20 | Towers to turn 20 (mean) | Income at turn 15 (median) |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| red | lost | v022 | 77 | 14 | 0.8 | 39 | 20.9 | 5.5 | 23 |
| red | lost | top 10 | 77 | 9 | 1.2 | 53 | 27.7 | 4.9 | 44 |
| red | won | v022 | 21 | 11 | 0.8 | 42 | 25.4 | 2.6 | 28 |
| red | won | top 10 | 21 | 12 | 0.5 | 36 | 19.5 | 3.5 | 21 |
| blue | lost | v022 | 31 | 13 | 0.6 | 43 | 21.4 | 5.0 | 24 |
| blue | lost | top 10 | 31 | 10 | 1.0 | 44 | 27.9 | 3.7 | 39 |
| blue | won | v022 | 21 | 13 | 1.4 | 40 | 30.0 | 1.8 | 27 |
| blue | won | top 10 | 21 | 7 | 0.9 | 26 | 20.5 | 2.9 | 23 |

Land, income and mines at turns 10 and 15:

| Seat | Result | Side | Turn | Active cells | Income | Turns with a mine built before | Gold |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| red | lost | v022 | 10 | 26 | 21 | 0.1 | 37 |
| red | lost | top 10 | 10 | 32 | 28 | 0.7 | 35 |
| red | lost | v022 | 15 | 30 | 23 | 0.6 | 38 |
| red | lost | top 10 | 15 | 47 | 44 | 2.1 | 70 |
| red | won | v022 | 10 | 28 | 22 | 0.2 | 30 |
| red | won | top 10 | 10 | 32 | 26 | 0.7 | 36 |
| red | won | v022 | 15 | 42 | 28 | 1.2 | 40 |
| red | won | top 10 | 15 | 28 | 21 | 1.2 | 26 |
| blue | lost | v022 | 10 | 27 | 23 | 0.2 | 38 |
| blue | lost | top 10 | 10 | 30 | 26 | 0.8 | 32 |
| blue | lost | v022 | 15 | 27 | 24 | 0.9 | 44 |
| blue | lost | top 10 | 15 | 45 | 39 | 2.1 | 61 |
| blue | won | v022 | 10 | 31 | 24 | 0.1 | 36 |
| blue | won | top 10 | 10 | 26 | 25 | 0.9 | 31 |
| blue | won | v022 | 15 | 41 | 27 | 1.1 | 53 |
| blue | won | top 10 | 15 | 27 | 23 | 1.6 | 37 |

In the losses v022 is a little behind at turn 10 (26 to 27 active cells against 30 to 32, a mine built in 10% to 20% of games against 70% to 90%) and far behind at turn 15 (27 to 30 cells and income 23 to 24, against 45 to 47 and 39 to 44): the gap opens between turns 10 and 15, at the first large cut. In its wins v022 is level or ahead at both turns. The two seats look alike.
