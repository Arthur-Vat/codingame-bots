# 0061. The daily collection plays zasmu only

- Status: accepted (owner, 2026-11-28)
- Date: 2026-11-28
- Supersedes: [0057](0057-daily-collection-of-codingame-games.md), decision 2
- Scope: framework

## Context

ADR 0057's decision 2 had the daily collection play up to 500 test games a day, the current release against the day's top 10, 50 games each on 50 fixed maps. On 2026-11-27 the owner asked for a model of the opponent's reply built from how zasmu plays, the first of A Code of Ice and Fire's leaderboard, and on 2026-11-28 for one by unit type (each unit's type first, then for each type a set rule or a set of likely actions, by the phase of the game and other measures), learnt from zasmu's games only, since it is the strongest player. By then the data branch `claude/codingame-games` held 1,038 games against zasmu, all under one agent id, and zasmu had won 213 of the 232 ranking and test games against `ice-and-fire-v032` there.

A test game always seats our bot first (the tested code is agent 0 in CodinGame's API reference; [GAME_ANALYSIS.md](../GAME_ANALYSIS.md), "In test games our code is player 0"): zasmu plays the first seat only in our ranking games where it had it, or in their replays, which CodinGame plays with the same players in the same seats (`replay_game_id`). Among the test games on the data branch, a game played again on the same map with the same release repeated an earlier one move for move in 47 of 542 cases (both bots' searches depend on time), so a map played again still gives new games.

Asked in forms on 2026-11-28, the owner chose: shift the collection to zasmu, and use the games already collected first; all 500 games against zasmu; a fixed file of 250 maps; the new ranking games where zasmu had the first seat replayed each day within the 500; until he switches it back.

## Decision

1. **The daily batch plays the current release against zasmu only, 500 games a day,** in place of the top 10 at 50 games each. The quota floor (100 of the 600 test games of 24 hours left to the owner and other sessions), the job's hours and the stop at a newer release (ADR 0059) stay as they were.
2. **Its games use 250 fixed maps,** `games/ice-and-fire/codingame-seeds-250.txt`: the 50 maps of ADR 0057 first, in their order, then 200 maps of our ranking games drawn at random once, so that days and releases still compare.
3. **Each run first replays, with the current release, our ranking games against zasmu where it had the first seat,** won or lost, that ended on 2026-11-01 or later (the first full day in Legend, where we still play: CodinGame replays only games of the current league) and were not yet replayed with that release, at most 20 a run and within the same floor (`scripts/codingame.sh replay --seat 1 --since --keep`).
4. **It stays so until the owner switches it back;** going back to ADR 0057's top 10 at 50 games each on the 50 maps is a change of the workflow and of this record's status line.

## Consequences

- About 500 games a day against zasmu, about 7,500 of its positions, nearly all with our bot in the first seat, and a few a day with zasmu first.
- The other top players are no longer played in test games: their play is seen only in our ranking games, and reports on them rest on the games collected until 2026-11-28. The daily leaderboard snapshot still shows when their agents change.
- The workflow names the opponent and the replays' first day in its environment, and the replays' log is not in the job summary. The replays get the job's remaining time (`replay --max-hours`), as the batch does.
- ADR 0058's decision 1 describes the batch as ADR 0057 had it; this record replaces that description.
