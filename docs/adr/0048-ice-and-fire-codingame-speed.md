# 0048. A Code of Ice and Fire: evaluations at CodinGame's speed and real limit, calibrated on each machine

- Status: accepted (owner, 2026-10-31)
- Date: 2026-10-31
- Supersedes: [0042](0042-official-time-limits-and-800-pairs.md), decision 1 for A Code of Ice and Fire only (time scale 1 and no tolerance, and a result that decides a candidate or a rating never played with them); [0038](0038-incremental-league.md), decision 8 for this replay only (the history replayed release after release)
- Scope: ice-and-fire

## Context

[ADR 0042](0042-official-time-limits-and-800-pairs.md) has every strength evaluation play at CodinGame's exact limits: 50 ms a turn and 1,000 ms for the first, counted by the arena from when it sends the input, at the speed of the machine that plays. Two facts measured on 2026-10-31 show that this is not what CodinGame plays for A Code of Ice and Fire.

**CodinGame's limit and speed.** The timing bot (`games/ice-and-fire/bots/timing`) holds each answer one millisecond longer each turn and times a fixed workload. In three self-play games that the owner ran in the CodinGame IDE, answers held to 62, 64 and 64 ms by the bot's own clock were accepted, and the games were cut at 63, 65 and 64 ms. CodinGame's limit, as a bot's clock sees it, is therefore about 63 ms, not 50. The workload ran 10.6 to 11.6 units a millisecond there, with dips to 5.0 and 8.2 ([E031](../../games/ice-and-fire/journal/E031-full-budget.md)).

**Our machines' speed.** The same workload was run on six GitHub runners (`ubuntu-latest`, scratch run [41196876937](https://github.com/Arthur-Vat/codingame-bots/actions/runs/41196876937)). Each was measured with one game at a time, and with four at a time as the SPRT and the league play. The runners have 4 CPUs on 2 physical cores (2 threads per core), so four games share two cores.

| Runner CPU | Runners | One game (units/ms) | Four at a time (units/ms) |
| --- | ---: | ---: | ---: |
| AMD EPYC 7763 | 4 | 22.8 | 13.4 |
| Intel Xeon Platinum 8573C | 1 | 22.0 | 13.5 |
| AMD EPYC 9V45 | 1 | 40.5 | 24.2 |

The league's 105 stored pairings were played on five CPU types, 20 of them on the 9V45. Session machines vary too: 18.9 units/ms on one session's machine and about 21 on another's.

At the old settings, a bot playing on a usual runner had about 1.2 times CodinGame's compute and 50 ms rather than about 63. On a 9V45 runner it had about 2.2 times CodinGame's compute. The owner first chose CodinGame's real limit and speed with a fixed time scale of 0.6, measured against a session machine. He asked for a GitHub runner to be measured before the factor was fixed. Given the table, he chose to calibrate each run.

The owner decided on 2026-10-31, in the conversation:

1. "I want to make sure we are closer to codingame limits obviously!"; then, in a form, "CG's real limit and speed (Recommended)": evaluations play at CodinGame's speed, with a tolerance so that the arena cuts about where CodinGame does.
2. In a form, "56 ms (Recommended)": the bot's hard limit by its own clock, 7 ms under the lowest cut seen.
3. On the league: "Replay the whole history with many runners in parallel, it should not take such a long time. Recall you have lets say 15 runners available (5 others for regular CI jobs)".
4. After the runners were measured, in a form, "Calibrate each run (Recommended)".

## Decision

1. **Every strength evaluation of A Code of Ice and Fire plays at CodinGame's speed and real limit.** This covers the smoke test, the SPRT, the league, and local screening.
   - The time scale is CodinGame's workload speed divided by the machine's: `scale = 11 / measured`, in units a millisecond.
   - The tolerance is the 13 ms between CodinGame's lowest cut by a bot's clock (63 ms) and its stated 50, at the machine's speed: `tolerance = 13 × scale`, rounded to the millisecond.
   - Bots receive the scale in `CG_TIME_SCALE`, as the arena gives it now, and scale their own budgets with it. The tolerance is not told to them.
2. **The machine's speed is measured at the start of each evaluation job, at that job's own parallelism.** The measure is the timing bot's workload, played as many games at a time as the job will play, for about ten seconds, and its median is used. Each job measures its own runner: an SPRT, a league, each shard of a replay, and each local screening. The scale, the tolerance and the measured speed are recorded with every result: in the league's lines, the SPRT's report and the journal's screenings.
3. **CodinGame's figures, 11 units a millisecond and a cut at 63 ms, live in the game's `evaluation.env`, with their source.** They change only with a new measure on CodinGame. The timing bot is played there in test games through the API ([ADR 0047](0047-codingame-api.md)) or by the owner, and the new figures arrive in a pull request that cites the games.
4. **The plan bot's hard limit is 56 ms by its own clock** (scaled by `CG_TIME_SCALE` in the arena). That is 7 ms under the lowest cut seen on CodinGame. Every phase of its turn ends by it, including the opponent's reply search of the last climb and the final scoring. The first turn keeps its own budget.
5. **The league's whole history is replayed at the new settings, its stored pairings played in parallel on about 15 runners.** The owner counts about 5 runners for the other CI jobs.
   - Each of the 105 stored pairings is played again: the same two bots, 800 pairs, the stored seed.
   - The pairings are spread over parallel jobs, each calibrated as in decision 2, and their lines are merged into a new `league.jsonl`.
   - Unlike [ADR 0038](0038-incremental-league.md), decision 8, the opponents are not chosen again from the ratings as the replay goes: they are the stored ones, so that the replay can run in parallel. New releases then choose their opponents from the new ratings as ADR 0038 says.
   - The old results and leaderboard are replaced, not mixed. Each League run needs the owner's go-ahead, as usual.
6. **Results measured before this record stay as recorded.** The journal's entries name the conditions of their figures, and later entries compare only results at the new settings.

## Consequences

- An SPRT or a rating of A Code of Ice and Fire measures a bot with about the compute and the time CodinGame gives it, whatever runner GitHub assigns. A change that helped only with more compute than CodinGame's can no longer be accepted. A bot that uses CodinGame's real time is no longer cut off by the arena's 50 ms.
- On the usual runners the scale is about 0.82 and the tolerance about 11 ms. Games take a little less time than at scale 1, so the SPRT's and the league's job limits ([ADR 0042](0042-official-time-limits-and-800-pairs.md)) still hold. On a 9V45 runner the scale is about 0.45, and the same job is faster.
- Results played on different machines are comparable only as far as the workload stands for the bots' work. It is a middle-game position of the bots' own engine and tactics, and it has run steadily on each runner (spread under 3%). A bot whose work differs, for example one bound by memory, could still differ between runner types.
- The evaluation scripts need a calibration step: `scripts/lib/evaluation.sh`, `scripts/sprt.sh`, `scripts/league.sh` and `sprt.yml`. The league needs a parallel replay of stored pairings: a matrix over shards, then a merge. Both go in one pull request, with `evaluation.env` and the docs (`docs/WORKFLOW.md`, `docs/ARCHITECTURE.md`, the game's README, and the probe's facts in `docs/CODINGAME.md`).
- The replay costs at most about 24 runner-hours: v014's league played 7 pairings of search bots in 95 minutes, and the older bots' pairings are faster. That is about 1.5 to 2 hours on 15 runners.
- The bot experiment that follows (E032) moves the plan bot's hard limit to 56 ms with strict reserves, and is screened and tested at the new settings.
- CodinGame's speed has dips (5.0 and 8.2 units a millisecond in the probe's games). The calibration plays at its usual speed, and the bot's hard limit by its clock, not the scale, guards against the dips.
- Ultimate Tic-Tac-Toe keeps [ADR 0042](0042-official-time-limits-and-800-pairs.md)'s exact limits. Moving it would be a separate decision, after the same probe on CodinGame.
