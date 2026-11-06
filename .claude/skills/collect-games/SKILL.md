---
name: collect-games
description: The daily collection of CodinGame games, followed by the dedicated session that a scheduled task wakes once a day - gets a token, fetches our new ranking games, plays the day's test games of the current release against the top 10, and writes one line in the handoff. Use only in that session, or when the owner asks for the day's collection.
---

# Collect the day's CodinGame games

The design is [ADR 0057](../../../docs/adr/0057-daily-collection-of-codingame-games.md); the script and the files are in `docs/GAME_ANALYSIS.md` ("Fetching and playing") and [ADR 0047](../../../docs/adr/0047-codingame-api.md). The session never submits a bot, never touches the owner's IDE and never prints the token.

## 1. Start

Read `HANDOFF.md` (skill `handoff`). If it says the collection is paused, or that a session needs test games today, do nothing more except what the note says (before each chunk below, read it again for such a note: the daily batch then pauses for the day). Only the owner pauses the collection, in the conversation; the main session then disables the scheduled task.

## 2. A token

1. If `~/.config/codingame/token` works (`GET /me` answers 200), go on.
2. Else start the device flow as the CodinGame Agent Skill says (<https://www.codingame.com/api/v1/docs/skill>; read it from there into the scratchpad, since installing it into `~/.claude/skills` may be blocked). A link lasts 15 minutes. Send the owner the approval link and the code in one push notification (tool `PushNotification`), and poll until it is approved or expires.
3. If it expires, start the flow again one hour after the previous link was sent, with a new notification. At most 3 links in all, each in one notification, and no other reminder. If none is approved, skip the day and say so in the handoff (step 5).

## 3. Fetch, then play

`GAME` is the game with the collection (`ice-and-fire`) and `RELEASE` the newest file in `games/GAME/releases/`.

```sh
scripts/codingame.sh fetch GAME --push
```

If it warns that a submission has no release, tell the owner in the handoff line: the line `<submission id> <release> <date>` goes in `games/GAME/codingame-submissions.txt`.

Then the day's batch, in chunks that run in the background (the session's background commands stop after 2 hours):

```sh
export CODINGAME_COMMIT_TRAILERS='<the session's attribution lines>'
scripts/codingame.sh batch GAME games/GAME/releases/RELEASE.rs --top 10 --games-per-opponent 50 \
  --seeds games/GAME/codingame-seeds-50.txt --keep 100 --max-hours 1.5 \
  --name "$(date -u +%F)-RELEASE-daily" --play --push
```

`--keep 100` leaves 100 of the 600 test games of 24 hours to the owner and other sessions; the batch stops by itself when starting one more would take them. After each chunk, run the same command again (same `--name`) until the output says the batch is done, or "stopped: 100 test games of the day left to the owner". A day that begins after midnight UTC uses its own name; a batch of an earlier day that is unfinished is not resumed. A chunk that exits 1 (games failed, the service looks down, the limits cannot be read) ends the day's play: say it in the handoff and in one push notification.

## 4. The day's line

Write one line in the handoff (skill `handoff`) with: the date, the release, the games played, won and lost by opponent (the batch's totals), and any top-10 player whose agent id changed since the previous snapshot (compare the agent ids by nickname in the two newest files of `leaderboard/` on the data branch). If the day was skipped or stopped, say why instead.

Send one push notification only if something needs the owner: a failure, a missing release line, a top player's new version is fine to mention. Otherwise stay quiet.

## 5. Never

Submit, change the IDE, print or store the token outside `~/.config/codingame/token`, or play test games beyond the floor.
