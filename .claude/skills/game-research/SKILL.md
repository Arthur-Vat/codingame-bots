---
name: game-research
description: Stage 8 of the new-game lifecycle - researches how to play a game well from CodinGame write-ups and academic papers, records the sources and what was taken from each, and ranks the approaches to prototype. Use once the game's studio stage is done, or when the owner asks for research on a game or a method.
---

# Research a game

Stage 8 of [docs/NEW_GAME.md](../../../docs/NEW_GAME.md); the policy is [ADR 0035](../../../docs/adr/0035-new-game-lifecycle.md) decisions 4 and 6.

## Depth

- **Default, a focused scan:** about 10 to 15 sources, read by the main session.
- **Deep research:** a multi-agent research, only when the owner asks; say first that it costs much more. The agent guard (`scripts/agent-guard.sh`) allows only the project's agents and the read-only built-in ones, so research agents run as `Explore` agents (they can search the web); a research skill or tool that starts general-purpose agents is refused by the guard, and allowing it is a change to ask the owner.

## Sources

- CodinGame: the owner pastes forum post-mortems and the game's thread; players' blogs can be searched on the web.
- Academic papers: search engines and arXiv, for the game itself and for its class (from `MODEL.md`).
- Other players' published bots may be read for ideas, never copied. A source that cannot be reached is named, not guessed (`CLAUDE.md`).

Search first in `docs/research/` for methods already written up, and link them instead of repeating them.

## Write

From [docs/templates/research-note.md](../../../docs/templates/research-note.md), in `games/<game>/research/README.md`:

- every source with a link and what was taken from it;
- what strong bots do, each claim with its source;
- candidate approaches ranked by expected strength and cost, given the engine's measured speed and the time per turn;
- the two or three prototypes proposed for the bake-off of stage 9.

A method that would serve other games gets its own note in `docs/research/<method>.md` (format in [docs/research/README.md](../../../docs/research/README.md)).

One pull request, `docs(<game>): research on <game name>`, light review. Then the gate, where the owner chooses the prototypes.
