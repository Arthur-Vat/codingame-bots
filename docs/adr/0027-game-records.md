# 0027. Game records: one format, a sample of each run, kept on the owner's computer

- Status: accepted (owner, 2026-10-10)
- Date: 2026-10-10
- Scope: framework

## Context

The studio reviews games ([ADR 0025](0025-studio.md)): games played in the arena (SPRT, league and local runs) and the owner's own games against bots. The arena writes one JSON line per game with its seed, bots, winner, end reason, turns and answer times, but not the moves; since bots run on a clock, a game cannot be rebuilt from its seed alone. A record must therefore hold every answer.

The owner asked, on 2026-10-10, to keep only a sample of arena games (about 10 per run), to search them by release, date and result, and to save, export and load games; games not saved need not be kept. The owner asked whether the storage would be enough and acceptable under GitHub's terms. A record is a few kilobytes, so even 10,000 games fit in about 50 MB.

## Decision

1. **A game record is one versioned JSON document.** It holds the format version, the game, the seed, each seat's player (a human, or a bot with its release or source and its settings), the answers of every turn as the text the referee received, the answer times, how the game ended, the result, the date and where the game comes from. Replaying the answers with the game's referee rebuilds every position.
2. **The arena can write records, and a run keeps a sample.** About 10 games per run, mixing wins, losses and draws, plus every game that ended in a timeout or an invalid answer. Workflows that play games upload their sample as a workflow artifact; the studio imports the downloaded file.
3. **The history lives on the owner's computer, one file per game, in a folder that is never committed.** Nothing of it goes into the repository. The studio lists and filters these files; the owner's own games are kept only when saved. Exported files use the same format, and loading one adds it to the history.

## Consequences

- Any position of any recorded game can be shown, analysed or replayed, for every game the framework adds.
- The arena gains an option to write records, and the workflows that play games gain an artifact; full records of every game of a run are not kept.
- The history is not backed up by GitHub; the owner keeps what matters by exporting it.
- A change to the record format raises its version, and the studio keeps reading older versions.
