# Research on methods

Notes on methods that serve more than one game: search, evaluation and learning, each with its sources and what this repository learned when using it ([ADR 0035](../adr/0035-new-game-lifecycle.md)). A game's own research is in `games/<game>/research/` and links here.

## Notes

- [Packing data into characters](utf16-packing.md): about 15.9 bits per character, for weights and tables that would not fit as base64. Not usable in a release until it is checked on CodinGame and the size check counts characters.

## Format

One file per method, `docs/research/<method>.md`: what it is, when it fits (the class of game, the time per turn), its known pitfalls, the sources with what was taken from each, and the games of this repository that use it, with their journal entries.
