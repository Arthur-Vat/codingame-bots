# 0042. Every strength evaluation at CodinGame's time limits, and 800 pairs per league pairing

- Status: accepted (owner, 2026-10-21); decision 1 superseded for A Code of Ice and Fire by [0048](0048-ice-and-fire-codingame-speed.md)
- Date: 2026-10-21
- Supersedes: [0013](0013-evaluation-time-limits.md), decision 1 (the time scale of 0.2 and the 5 ms of tolerance) and the calibration of decision 3; [0014](0014-full-time-confirmation.md), the confirmation at full time; [0020](0020-full-time-sprt-in-legend.md), decisions 2, 3 and 5 (full time only for a game in Legend, and screening at 20 ms); [0038](0038-incremental-league.md), the league's 100 pairs in decision 2 and the cost estimate in its consequences
- Scope: framework

## Context

Evaluations ran at a fifth of CodinGame's time limits plus 5 ms of tolerance ([ADR 0013](0013-evaluation-time-limits.md)) until a game's bot reached Legend, and at the exact limits after that ([ADR 0020](0020-full-time-sprt-in-legend.md), [ADR 0038](0038-incremental-league.md)). Ultimate Tic-Tac-Toe's smoke test, SPRT and league play at the exact limits. A Code of Ice and Fire's played at a time scale of 0.2 plus 5 ms, which is 10 ms plus 5 ms of its 50 ms.

A replay of A Code of Ice and Fire's bootstrap at both limits gave identical results for the bots of 2026-10-19 (the baselines and the ladder): they answer in about 0.1 ms and never read the clock. The coming search bot, `plan`, uses its time, so the limit matters from now on.

On 2026-10-21 the owner decided:

1. "I would like to have the official time control in all games."
2. "I also want to make sure many more games are played to reduce uncertainty on elo", then, asked whether 800 pairs fit the job's limit: "What about 800 pairs? Is it within the job limit? If yes go on with this."
3. The league's history is replayed at the new settings.

The cost of a league pairing, measured: Ultimate Tic-Tac-Toe's `uttt-v018` league, 8 pairings of 100 pairs at full time, took 23 minutes ([League run 39029150086](https://github.com/Arthur-Vat/codingame-bots/actions/runs/39029150086)). At 800 pairs a pairing has 1,600 games and its Elo has a 95% interval of about plus or minus 17 near 50%, against plus or minus 50 at 100 pairs.

## Decision

1. **Every strength evaluation of every game plays at CodinGame's exact time limits** (time scale 1, tolerance 0): the smoke test, the SPRT, the league, and local screening through the game's `evaluation.env`. The settings `TIME_SCALE=1` and `TIME_TOLERANCE_MS=0` hold for all of them, and `SPRT_TIME_SCALE` and `SPRT_TIME_TOLERANCE_MS` stay empty. The tolerance and the scale remain options of the arena and overrides in the environment, for a small or busy machine where the exact limits cause timeouts that GitHub's runners do not; a result that decides a candidate or a rating is never played with them. A game's bot being in Legend no longer matters.
2. **The confirmation at full time is not played** (`CONFIRM_PAIRS=0`): the SPRT itself plays at those limits. The mechanism stays in `scripts/sprt.sh`.
3. **A league pairing plays 800 pairs** (`LEAGUE_PAIRS=800`), 1,600 games, in every game. A release still plays 8 opponents ([ADR 0038](0038-incremental-league.md), decision 2).
4. **The league job's time limit is 300 minutes** (`timeout-minutes` in `sprt.yml`; GitHub's limit is 360).
5. **The SPRT keeps its own rules**: bounds, error rates, and the cap on pairs (`SPRT_MAX_PAIRS`) do not change with this record. It stops as soon as it is confident.
6. **Existing league results are replayed at the new settings**: Ultimate Tic-Tac-Toe's history (the pairings of its bootstrap, played at 100 pairs), and A Code of Ice and Fire's bootstrap, at full time and 800 pairs, release after release as in [ADR 0038](0038-incremental-league.md), decision 8. Claude does it after this record is merged, with the owner's go-ahead for each League run as usual.

## Consequences

- Every decision and rating measures what CodinGame plays, for every game, and a bot that uses its time is judged on it. A change that helps only at short time controls can no longer be accepted.
- A pairing's Elo has an interval about three times narrower, at 8 times the games. Ultimate Tic-Tac-Toe's release league takes about 3 to 3.5 hours (8 pairings of 800 pairs take 8 times the 23 minutes of 100 pairs, about 184 minutes, and the canary adds one more pairing, about 23 minutes), within the 300 minutes. A Code of Ice and Fire's has at most 7 opponents (the previous release, the 5 nearest and `bronze`) and a canary when one is chosen, so at most 8 pairings: 12,800 games, 3,200 game durations on 4 cores. It fits the 300 minutes only while a game averages 5.6 seconds or less, about 140 answers of 40 ms, or about 90 when both bots also use 1,000 ms for their first answer. The Bronze bot's games are short (about 30 answers), but games between search bots can reach 200 answers, so the fit is checked, with `SPRT_MAX_PAIRS`, once the first search bot's game length is measured (the `plan` bot, E002). A release's pull request waits for the league, and a longer wait is the price of the narrower intervals.
- The SPRT of Ultimate Tic-Tac-Toe is unchanged (about 3.8 hours at the cap, [ADR 0020](0020-full-time-sprt-in-legend.md)). A Code of Ice and Fire's at full time plays 16,000 games at the cap of 8,000 pairs, 4,000 game durations on 4 cores: it fits the SPRT job's 360 minutes while a game averages 5.4 seconds, about 130 answers of 40 ms, or about 85 when each bot also uses 1,000 ms for its first answer. The games of random players are about 141 answers long and the Bronze bot's about 30 ([README](../../games/ice-and-fire/README.md), "Numbers"); a search bot's will be measured. A test usually stops at a few hundred to a few thousand pairs, but one that reaches the cap with long games would be cut off by the job's limit. When the first search bot exists, `SPRT_MAX_PAIRS` is checked against its measured game length, as [ADR 0020](0020-full-time-sprt-in-legend.md) did for 10,000 pairs; a change of the cap is a new decision.
- The replay of the history costs about 45 runner-hours for Ultimate Tic-Tac-Toe (124 pairings at 800 pairs) over many League runs, plus A Code of Ice and Fire's bootstrap. The old results and the old leaderboard are replaced, not mixed: pairings of 100 and 800 pairs would weigh differently in the rating.
- A League run (`league.yml`) must not start a bot it cannot finish in the job's 360 minutes. Set `stop_after_minutes` to 360, minus the build, minus the longest bot's time: for Ultimate Tic-Tac-Toe at 800 pairs a bot with 8 opponents takes about 3.5 hours, so about 140, and a run then plays one bot, or more while the early bots have fewer opponents; for A Code of Ice and Fire, measure the first search bot's league before choosing (about 230 holds for a bot whose league takes 2 hours at most). The input's default is 140 (it was 300, which suits bots of up to about 50 minutes).
- Results measured at the old limits are not re-run, except the league's: the journal keeps both Elo figures for experiments before [ADR 0020](0020-full-time-sprt-in-legend.md).
