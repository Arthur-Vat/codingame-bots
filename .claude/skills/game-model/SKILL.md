---
name: game-model
description: Stages 3 and 4 of the new-game lifecycle - writes the game's MODEL.md (plain English, numbers, a short formal model, what follows for bots) and lists the framework gaps the game opens, each to be decided by the owner. Use once a game's RULES.md is merged.
---

# Model a game and find the framework's gaps

Stages 3 and 4 of [docs/NEW_GAME.md](../../../docs/NEW_GAME.md).

## Stage 3: the model

Start `games/<game>/MODEL.md` from [docs/templates/game-model.md](../../../docs/templates/game-model.md), working from `RULES.md` only.

- Plain English first: what a player sees, chooses and wants each turn.
- Numbers: estimate what the engine does not measure yet, and say "estimate"; stage 6 replaces them with measures.
- The formal part stays short: sets, observation, transition, payoff, and the class of game in standard terms (alternating or simultaneous, perfect or imperfect information, deterministic or stochastic, zero-sum or not, number of players).
- What follows for bots: symmetries, what can be precomputed, what fits in 100,000 bytes, and the families of methods the class suits. Do not choose a method: that is stage 8 and 9.

One pull request, `docs(<game>): model of <game name>`, light review. Then the gate.

## Stage 4: framework gaps

Compare the model with what the framework assumes, as listed in the playbook's stage 4 checklist; read the code when unsure (`crates/cg-arena`, `crates/cg-search`, `studio/game`, `studio/web`). For each gap:

- where it is, and what would change;
- whether the game can wait for it (the studio can come later than the arena, for example);
- the options, for the owner (skill `ask-decision`), then a decision record (skill `write-adr`) once he has chosen.

Add the gaps to `MODEL.md`'s table, with their records. A game with no gap says so. Then the gate.
