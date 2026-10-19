# Research on methods

Notes on methods that serve more than one game: search, evaluation and learning, each with its sources and what this repository learned when using it ([ADR 0035](../adr/0035-new-game-lifecycle.md)). A game's own research is in `games/<game>/research/` and links here.

## Notes

- [Planning a turn of many commands](multi-action-turns.md): greedy construction with a memory of opponent replies, online evolution, evolutionary and constrained MCTS, for turns that hold many commands. Researched for A Code of Ice and Fire on 2026-10-19; not used by a bot yet.
- [Packing data into characters](utf16-packing.md): about 15.9 bits per character, for weights and tables that would not fit as base64. Checked on CodinGame on 2026-10-14, and the size check counts characters ([ADR 0037](../adr/0037-size-limit-in-characters.md)); not usable in a release until the Rust tools exist.

## Format

One file per method, `docs/research/<method>.md`: what it is, when it fits (the class of game, the time per turn), its known pitfalls, the sources with what was taken from each, and the games of this repository that use it, with their journal entries.
