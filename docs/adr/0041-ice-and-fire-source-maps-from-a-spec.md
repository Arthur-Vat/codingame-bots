# 0041. A Code of Ice and Fire: the source's maps, rebuilt from a written spec

- Status: accepted (owner, 2026-10-19)
- Date: 2026-10-19
- Supersedes: [0039](0039-ice-and-fire-maps-and-studio.md), decisions 1 and 3
- Scope: ice-and-fire

## Context

[ADR 0039](0039-ice-and-fire-maps-and-studio.md) gave our referee its own map generator, written from the map facts of RULES.md, and planned to compare its maps with the maps that the `rules-check` bot logs on CodinGame. The game's source, under the GNU GPL 3, is read for facts only and never translated or copied ([ADR 0035](0035-new-game-lifecycle.md)), and the repository is distributed under MIT or Apache 2.0 ([ADR 0001](0001-license.md)).

On 2026-10-19 the owner asked for the arena's maps to be exactly the source's, given the same seed, and offered a prompt to port the source's game state to Rust line by line. A line-by-line port would be a derivative of GPL code. Among the options brought to him the same day (a written spec implemented without the source, a separate GPL crate used only by the arena and the studio, comparing statistics first, or the full port as written), he chose "Clean-room spec".

Maps printed by the source's generator, run unchanged outside the repository, can check a rebuilt generator map by map: the source's map depends only on the seed and the league.

## Decision

1. **Our referee makes the source's maps: for every seed and league, the same void cells and mine spots as the source's generator.** The source's generator takes a 64-bit signed seed; ours takes the same 64 bits as an unsigned number.
2. **The generator is written from a spec in our own words, [games/ice-and-fire/MAP_GENERATOR.md](../../games/ice-and-fire/MAP_GENERATOR.md), by an implementer that never reads the source.** The spec describes the steps and their order, the numbers they use and the random number generator they draw from (the one Java's standard library documents); it holds no code of the source. The session that reads the source writes and checks the spec; the implementer works from the spec and the test data only.
3. **Maps printed by the source's generator, run unchanged outside the repository, are committed as test data, and the generator must match them exactly.** Nothing of the source's code, nor any harness built on it, enters the repository; only its printed maps do. The data covers every league on a list of chosen seeds, and many more seeds in Bronze.
4. **Real CodinGame maps are still compared.** The maps that the `rules-check` bot logs are checked against the map properties of RULES.md, and their statistics (non-void cells, mine spots, the distance between the HQs) are set beside the generator's. A clear difference goes to the owner: it would mean CodinGame no longer draws its maps as the source does.

Decisions 2 and 4 of ADR 0039, the map properties tested in each league and replay before human play in the studio, stay.

## Consequences

- The arena's games are played on the maps the source would draw, without the owner's playing time, and the repository stays under MIT or Apache 2.0.
- A spec this close to the source's steps is a legal grey zone that the owner accepted; the spec states behaviour, never code, and the implementer works without the source.
- Every game played on our maps changes: tests and fixtures built on the old maps are regenerated, and the game's numbers and engine speed are measured again.
- Bots receive the map from CodinGame and never need the generator.
