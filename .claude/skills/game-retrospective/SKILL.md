---
name: game-retrospective
description: Stage 11 of the new-game lifecycle - looks back on a game from its roadmap, journal and pull requests, writes its RETROSPECTIVE.md, and proposes changes to the framework, the skills and the playbook. Use when the owner pauses or stops work on a game, or asks for a retrospective.
---

# Look back on a game

Stage 11 of [docs/NEW_GAME.md](../../../docs/NEW_GAME.md).

## Gather

- `games/<game>/ROADMAP.md`: each stage's dates and the owner's answers.
- The journal: which experiments were accepted, which dropped, and why.
- The game's merged pull requests: what was reworked, and what review or CI caught.
- What the next game will reuse as it is, and what had to be written for this game alone.

## Write

`games/<game>/RETROSPECTIVE.md`:

- the result (league, releases) and how long each stage took;
- what worked, what cost more than planned, and why;
- what belongs in the framework rather than the game;
- proposed changes, each one line: to the framework, to a skill, to the playbook, or to the template of a game's roadmap.

One pull request, `docs(<game>): retrospective of <game name>`, light review. At the gate the owner chooses which proposals to make; each becomes its own pull request (one topic per pull request).
