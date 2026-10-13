---
name: game-intake
description: Stage 1 of the new-game lifecycle - lists the CodinGame pages the owner must paste, gathers the game's facts with a source for each, checks the source's license, and creates the game's README and roadmap. Use right after the owner names a new game.
---

# Intake of a new game

Stage 1 of [docs/NEW_GAME.md](../../../docs/NEW_GAME.md). Claude does not open codingame.com; the owner pastes its pages (ADR 0035).

## 1. Ask for the pages

In one message, ask the owner to paste:

- the game's statement, as shown in the league where the full rules apply, and the statement of each lower league if the rules change between leagues;
- the forum thread of the game, if it has one;
- the link to the game's source, if the statement gives one.

Say why each is needed. Wait for them.

## 2. Gather the facts

From the pasted pages and the source (a GitHub repository outside this one is attached read-only with the session's `add_repo` tool, then cloned), fill the intake checklist of the playbook. For each fact, keep its source: the statement, a file and line of the source, or a forum post.

- **License of the source:** read its license file. GPL or no license: facts only, nothing translated or copied (ADR 0035). Say which in the README.
- **Disagreements** between statement and source: list them; the statement and CodinGame's behaviour win, checked later by `rules-check`.
- **Unknowns:** list them as questions for the owner. Never guess a CodinGame fact (`CLAUDE.md`).

## 3. Write

One pull request, `docs(<game>): intake of <game name>`:

- `games/<game>/README.md`: what the game is, its facts with sources (players, turn format, time limits, leagues, source and license), and short notes on the criteria the owner named on 2026-10-13 (interest and research value, source and license, how it shows in the studio, depth of the leaderboard).
- `games/<game>/ROADMAP.md` from [docs/templates/game-roadmap.md](../../../docs/templates/game-roadmap.md), stage 1 in progress, the open questions listed.
- `docs/ROADMAP.md`: the game in its phase's checklist (phase 7 for the second game), and later its line in the "Ongoing" row once it has a release.
- `scripts/check-docs.sh` passes; a light review (skill `review-pr`).

Then the gate: skill `new-game`, "At a gate".
