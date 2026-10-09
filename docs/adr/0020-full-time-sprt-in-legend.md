# 0020. In Legend, the SPRT plays at CodinGame's time limits

- Status: accepted (owner, 2026-10-09)
- Date: 2026-10-09
- Supersedes: for games whose bot is in Legend, decision 1 of [ADR 0013](0013-evaluation-time-limits.md) for the smoke test and the SPRT, and [ADR 0014](0014-full-time-confirmation.md)
- Scope: framework

## Context

The SPRT decides at a fifth of CodinGame's time limits, 20 ms per move plus 5 ms of tolerance ([ADR 0013](0013-evaluation-time-limits.md)). Since the first release reached Legend, an accepted candidate also plays 500 pairs at the real limits, and is rejected only if it is clearly weaker there ([ADR 0014](0014-full-time-confirmation.md)). ADR 0014's decision 4 asks Claude to propose moving the SPRT itself to full time if the two start to disagree.

They do. The journal's accepted experiments, at 20 ms and at full time:

| Experiment | Elo at 20 ms (SPRT) | Elo at full time (500 pairs) |
| --- | --- | --- |
| E003, solver | +31.6 | +13.6 |
| E011, playout policy | +45.4 | +95.4 |
| E014, children in the policy's order | +35.6 | +9.7 |
| E015, pattern policy, first run | +19.2 | +11.8 |
| E015, the same bot, second run | +12.8 | +5.6 |

The two disagree in both directions, by more than the gains now being measured. E016, dropped in local screening, went further: about -49 Elo at 20 ms and about -6 at full time. Its larger policy cost 20% of the search's iterations, which matters more when a move gets 18 ms than when it gets 90.

On 2026-10-09 the owner asked that a bot in Legend be tested in real conditions, for this game and the next ones, and chose that over keeping 20 ms.

A full-time pair costs about 1.7 s on GitHub's runners: E015's two SPRT jobs played 803 more 20 ms pairs in 5 more minutes, and the rest of their 22 minutes went mostly to 500 full-time pairs. An SPRT that decides in 400 to 2,000 pairs, as the recent ones did, takes about 10 minutes to an hour at full time. 10,000 pairs, today's cap, would take about 4.7 hours, close to the job's 6-hour limit.

## Decision

1. **A game's `evaluation.env` sets the time limits of its smoke test and SPRT** (`SPRT_TIME_SCALE`, `SPRT_TIME_TOLERANCE_MS`). Without them, these follow the arena's `TIME_SCALE` and `TIME_TOLERANCE_MS` as before.
2. **Once a game's bot is in Legend on CodinGame** (as reported by the owner), its smoke test and SPRT play at CodinGame's exact limits: time scale 1, no tolerance. Ultimate Tic-Tac-Toe is there now: `uttt-v001` reached Legend on its own on 2026-10-07.
3. **The full-time confirmation is dropped for such games** (`CONFIRM_PAIRS=0`): the SPRT itself plays in those conditions.
4. **The SPRT's other settings stay:** bounds of 0 and 10 Elo, 5% error rates, timeouts tolerated in up to 1% of games ([ADR 0015](0015-tolerate-rare-timeouts.md)). Its cap falls from 10,000 to 8,000 pairs, about 3.8 hours, so that the longest test ends within the job's limit. A test that reaches the cap is inconclusive and does not accept the candidate.
5. **Local screening may still use 20 ms** to choose what to test, and the league keeps the arena's `TIME_SCALE` for its ratings. Only the full-time SPRT decides.

## Consequences

- Every decision measures what CodinGame plays, so a change that helps only at short time controls is no longer accepted, and one that helps only at long ones is no longer rejected.
- Tests take longer: about 10 minutes to an hour usually, up to about 4 hours. While one runs it holds one of the account's 20 concurrent jobs, as before.
- The journal's `elo` field and the SPRT comment now give the full-time Elo; `full_time` repeats it.
- Results before this decision are not re-run: the journal keeps both numbers for them.
- Games below Legend keep the faster 20 ms tests, where gains are large enough for them.
