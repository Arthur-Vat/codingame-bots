---
name: new-game
description: Starts a new CodinGame game or resumes one in the new-game lifecycle - reads the game's roadmap, names the stage in progress and the skill that does it, and brings each finished stage to the owner's gate. Use when the owner names a new game, and whenever work on a game in its lifecycle resumes.
---

# Start or resume a game

The lifecycle is in [docs/NEW_GAME.md](../../../docs/NEW_GAME.md) ([ADR 0035](../../../docs/adr/0035-new-game-lifecycle.md)). Read the playbook's section for the stage at hand only, not the whole file.

## A new game

1. The owner names the game; Claude never picks or compares games.
2. Choose the folder name with the owner (short, lowercase, as `uttt`); it is the scope of the game's pull requests and decision records (ADR 0022).
3. Start stage 1 with skill `game-intake`. Its pull request creates `games/<game>/ROADMAP.md` from [docs/templates/game-roadmap.md](../../../docs/templates/game-roadmap.md) and adds the game's line to `docs/ROADMAP.md`.

## Resuming

1. Read `games/<game>/ROADMAP.md`: the stage in progress, its checklist, open questions and decisions.
2. Act on the stage's status:
   - `In progress`: carry on with the stage's skill (table in the playbook).
   - `Waiting for the owner`: ask him at the gate (below); do not start the next stage.
   - `Done`: start the next stage, after asking the owner if it is heavy work (`CLAUDE.md`).

## At a gate

When a stage's outputs are merged:

1. Bring the gate to the owner (skill `ask-decision`): what the stage produced, with links; what the next stage would do and roughly how large it is; the questions it opened. Recommend an answer.
2. Record his answer in the game's roadmap, under "Decisions" with its date, and the stage's status as `Done (YYYY-MM-DD)`; this rides with the next stage's first pull request (skill `review-pr`, "Small bookkeeping"). Note what waits in `HANDOFF.md`.
3. Only then start the next stage.

A gate the owner answered "stop" or "pause" sets the game's status in `docs/ROADMAP.md` and stops the lifecycle until he resumes it.
