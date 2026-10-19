# 0039. A Code of Ice and Fire: our own map generator, and replay before human play in the studio

- Status: accepted (owner, 2026-10-17); decisions 1 and 3 superseded by [0041](0041-ice-and-fire-source-maps-from-a-spec.md)
- Date: 2026-10-17
- Scope: ice-and-fire

## Context

A Code of Ice and Fire draws a new map for each game from its seed ([games/ice-and-fire/RULES.md](../../games/ice-and-fire/RULES.md), "The map"). Bots receive the map from CodinGame, but our arena's referee must make maps too, and the evaluation of every bot plays on them. The only generator known is the game's source, which is under the GNU GPL 3, so it is read for facts only and never translated ([ADR 0035](0035-new-game-lifecycle.md)). The game's model listed this as a framework gap, along with the studio's human play, which lists a human's moves as complete answers when a turn here is many commands ([games/ice-and-fire/MODEL.md](../../games/ice-and-fire/MODEL.md), "Framework gaps").

On 2026-10-17 the owner chose, among options with their costs:

- for maps: "Own generator + check", over a bank of real maps logged on CodinGame (faithful, but it needs many games run by the owner) and over our own generator without any comparison;
- for the studio, offered "Replay first, play later" (replay and analysis at stage 7, human play later if he wants it), "Full play at stage 7" (a turn builder for human play) and "No studio for now", he answered in his own words: "No human play for this game, replay first play later".

## Decision

1. **Our referee makes the game's maps with its own generator, written from the map facts in RULES.md, not from the source's code.** The generator is our own design; it uses the source only through the facts RULES.md states.
2. **The generator holds every map property RULES.md states, and tests check each one.** The properties are the half-turn symmetry, one connected area, the clear corner blocks around the HQs, and the count and places of the mine spots, in each league.
3. **Its maps are compared with real CodinGame maps.** Stage 5's `rules-check` bot logs the maps CodinGame sends. Their statistics are set beside those of our maps: non-void cells, mine spots, and the distance between the HQs. A clear difference goes to the owner, and the generator is tuned to the real maps, never the reverse.
4. **The studio replays and analyses this game's games; human play waits until the owner asks for it.** Stage 7 builds the board renderer and replay, with analysis when the game has an analysis program. No turn builder for human play is built before the owner asks for one.

## Consequences

- The arena does not depend on the owner's playing time, and no GPL code enters the repository.
- Our maps may differ from CodinGame's in ways the facts do not capture. Decision 3 bounds how far, and bots should not rely on details of either generator.
- Stage 5 includes the generator, its property tests and the map logging of the `rules-check` bot. The comparison needs enough logged games to mean something: the owner runs the `rules-check` bot as for Ultimate Tic-Tac-Toe.
- This game needs no change to the studio's human-move interface (`studio/game`, `HumanMove`).
