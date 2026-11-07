# 0057. Daily collection of CodinGame games against the top players

- Status: accepted (owner, 2026-11-06); decisions 1 and 5 superseded by [0058](0058-codingame-collection-on-github-actions.md)
- Date: 2026-11-06
- Supersedes: [0033](0033-no-daily-scheduled-session.md), decision 1, for this one task
- Completes: [0056](0056-codingame-game-analysis.md), decision 6
- Scope: framework

## Context

[ADR 0056](0056-codingame-game-analysis.md) built a framework that records CodinGame games and analyses how the top players play; its decision 6 had test-game batches run on demand. The first batch (2026-11-06: `ice-and-fire-v014` against the top 10 of A Code of Ice and Fire, 10 games each) showed large and consistent gaps, but 10 games a player is too few for finer questions, and the API cannot list games between other players, so the top players' play is only seen in games against us.

On 2026-11-06 the owner asked to gather as many games as possible every day, then to report on a large volume of games, and noted two biases: an opponent's play depends on our release, and opponents update their bots. In the 322 games recorded by 2026-11-06, each opponent appears under a single agent id, and our own agent id changed once, when the submission moved up a league; the API calls the agent id "the player's current agent".

CodinGame's limits on test games ([ADR 0047](0047-codingame-api.md); API reference, read 2026-11-06): one at a time; 80 an hour, 150 per 3 hours, 200 per 6 hours and 600 in any 24 hours, sliding windows shared with the owner's own use of the website and with other sessions. A token lives only in a session's sandbox, approved by the owner with the device flow (ADR 0047, decision 2); the sandbox does not keep it once the session's container is reclaimed after a pause, so a session woken the next day usually needs a new one. An approval link (device code) lasts 15 minutes.

[ADR 0033](0033-no-daily-scheduled-session.md) decision 1 says no scheduled session runs for this repository, and that bringing one back is a new decision; [ADR 0051](0051-weekly-league-pull-request.md) made one exception for the weekly league pull request.

Asked in a form on 2026-11-06, the owner chose: a session that runs each day and that he approves from a link; 500 test games a day; the games kept on the branch `claude/codingame-games` for now.

## Decision

1. **A scheduled task wakes a dedicated session once a day to collect games, an exception to ADR 0033 for this task.** The session reads `HANDOFF.md` first and writes a one-line summary there at the end (ADR 0024). If its token no longer works, it starts the device flow and sends the owner the approval link as a push notification; a link lasts 15 minutes, and the session sends at most three, an hour apart, then skips the day. Once it has a token, it fetches the new ranking games of our submissions, then plays the day's batch.
2. **Up to 500 test games a day: the current release against the top 10 of that day's leaderboard, 50 games each.** The games use 50 fixed maps, the same every day so that days and releases compare: a new seeds file holding the 10 maps of `games/<game>/codingame-seeds.txt` first, then 40 more drawn once at random. The batch runs as long as the limits allow, waits through their windows and resumes; it stops for the day when fewer than 100 test games of the 600 allowed in 24 hours would be left. Those 100 are the owner's and other sessions' (probes, on-demand batches); when a session needs more, the daily batch pauses for that day.
3. **Each game records our release and each opponent's agent id.** Ranking games get their release from the submission that played them; test games carry it already. Reports compare like with like: they separate or filter games by our release and by the opponent's agent id, and say when a top player's agent id changed, which the leaderboard snapshot taken each day shows.
4. **The games stay on the branch `claude/codingame-games`.** Moving them (a separate repository, or compressed daily files) is reconsidered when the branch nears 200 MB.
5. **The owner can pause or stop the collection at any time.** The scheduled task is then disabled, and the games already collected stay.

## Consequences

- The analysis gets about 500 games a day against the top players, enough within a week for per-version profiles and for questions that 10 games cannot answer.
- The owner opens one link a day; the rest is automatic, and the dedicated session must not be archived while the collection runs.
- A day's batch takes more than 12 hours, mostly because of the 6-hour window; the owner and other sessions keep about 100 test games a day.
- `scripts/codingame.sh` needs a way to stop a batch at a quota floor, a seeds file of 50 maps starting with the 10, the index records the release of ranking games, and the reports gain a filter by release and by opponent version.
- The data branch grows by about 1 to 2 MB a day in git's compressed form; full clones of the repository grow with it.
