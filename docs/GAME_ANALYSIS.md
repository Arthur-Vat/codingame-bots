# Game analysis: CodinGame games recorded and analysed

The design of the tool that records games played on CodinGame, rebuilds them with our engines, and reports how the top players play and where our bots lose to them ([ADR 0056](adr/0056-codingame-game-analysis.md)). It is a framework for any two-player game of the repository; A Code of Ice and Fire is the first game it is applied to. Status: designed, not built (the steps are at the end).

## What it is for

The owner's goal (2026-11-06): identify the strategies of the top bots, and our weaknesses against them. The first reports give one profile per top bot: its opening, when it attacks, how it uses its resources (in A Code of Ice and Fire: towers, mines and gold), how it beats us, and what we could copy.

## What CodinGame gives

From CodinGame's API reference, read 2026-11-06 ([docs/CODINGAME.md](CODINGAME.md#api)):

| Source | What we get | Cost |
| --- | --- | --- |
| Ranking games of our submissions (`GET .../battles`, `GET /games/{id}`) | Every game our current or a past submission played for its rank, newest first, with the opponents' names and agent ids | Requests only (120 a minute) |
| Test games (`POST .../test-games`) | A game of any code we send against chosen opponents: the players of our league and the lower ones; fixed game options or random ones; our code is player 0 | One at a time; 80 an hour, 600 a day, shared with the owner |
| Replays (`replay_game_id`) | One of our ranking games played again with other code: same options, same opponents, our seat kept | As a test game; only games of the current league, against opponents we can still test against |
| Games between other players | Nothing: the API lists only our own games | |

A game's log (JSON) has the game options (for A Code of Ice and Fire, `seed=<signed 64-bit number>`), the players, and frames. A frame that carries a player's output is that player's answer for one turn (A Code of Ice and Fire: `TRAIN 1 1 0;TRAIN 1 2 0`, commands separated by `;`); the frames between them describe what the commands did. The log ends with the ranking and the scores. Debug output (stderr) is there for our own code in test games only. A log of 42 answers is about 15 KB.

## Overview

```
CodinGame API ──scripts/codingame.sh──► branch codingame-games: logs + index      (network)
                                              │
                    game's conversion ────────┤ records (ADR 0027) + origin     (offline from here)
local arena games ── already records ─────────┤
                                              ▼
                    game's engine: rebuild and check against CodinGame's result
                                              ▼
                    game's review binary: trace (boards, metrics, phases, moments)
                                              ▼
            cg-review: profiles, events, openings, weaknesses, judge comparison
                                              ▼
            Markdown report in games/<game>/research/codingame/  (main)
```

Only the first arrow touches the network. The rest reads files, so it runs in a session, in CI, or on the owner's computer.

## The framework's parts and the game's parts

| Part | Framework (any game) | Game (A Code of Ice and Fire first) |
| --- | --- | --- |
| Fetching | `scripts/codingame.sh`: ranking games, logs, test-game batches, replays, the API's limits, the index | The puzzle's id on CodinGame (`a-code-of-ice-and-fire`), in `games/<game>/evaluation.env` |
| Storage | The branch `codingame-games`, its layout and index | Nothing |
| Conversion | Reading a log; the record's origin; the check report | Frames to turns' answers; game options to a seed |
| Rebuild and check | Running the check on every game, leaving out and listing those that disagree | Its engine and referee (they exist) |
| Trace | The trace type, game files, metrics reports (`cg-review`, exist) | Its review binary's metrics, phases and moments (exist) |
| Events | Rates and first turns of each event, by player and phase | Which events a ply has |
| Openings | Counting and comparing opening sequences by player | A side's first plies as tokens, the same from both seats |
| Profiles and weaknesses | All of it | Nothing beyond the above |
| Judge comparison | Running the judge on positions, agreement and score gaps by player and phase, the largest gaps | A position as the bot reads it; reading the judge's answer; comparing two moves |
| Report | The Markdown sections and tables, labelled with the game's own names | Optionally, a short section of its own |

### What a game provides

In Rust, a game's review crate implements one trait, added to `cg-review` (names to be settled in the code):

```rust
pub trait CodinGameGame {
    /// The seed of a game from CodinGame's game options.
    fn seed(options: &str) -> Result<u64, String>;
    /// The answers of every turn, by seat, from the frames of a log.
    fn answers(log: &Log) -> Result<Vec<Vec<RecordedAnswer>>, String>;
    /// The events of one ply of a trace, by name (e.g. "tower", "mine", "level 3", "large cut").
    fn events(trace: &Trace, ply: usize) -> Vec<String>;
    /// The first plies of a seat as tokens, the same from both seats (mirrored when the map is).
    fn opening(trace: &Trace, seat: usize, plies: usize) -> Vec<String>;
    /// The judge's input for the position before a ply, and how to read and compare moves.
    fn judge_input(record: &Record, ply: usize) -> Option<String>;
    fn compare(played: &str, judged: &str) -> MoveMatch; // Same, SameFirstCommand, Different
}
```

The rebuild uses the replay the review binary already has; the check compares its end with the log's ranking. A game without a judge returns no input, and its report leaves that section out.

### Kinds of games

- Alternating turns (Ultimate Tic-Tac-Toe, A Code of Ice and Fire) and simultaneous turns both fit: a record's turn holds the answer of every seat that answered.
- Several commands a turn: the game's `answers` keeps the line as the referee reads it, and `events` counts what it holds.
- More than two players, or none (optimization and code golf puzzles), are outside the framework: the record and the trace have two seats. The index and the profiles keep players as lists, so the main step for such a game would be the record.

## Stored data

The branch `codingame-games` is never merged, as `league-results` and `claude/handoff` are. It holds:

```
<game>/logs/<game_id>.json        the log as the API gave it (JSON), without the response's rate limits
<game>/index.jsonl                one line a game (below)
<game>/batches/<batch>.json       a test-game batch: release, opponents with their ranks, maps, games played, failures
```

An index line: CodinGame's game id; the kind (`ranking`, `test`, `replay`); when it ended and when it was fetched; for ranking games the submission id, for test games the release and the batch; our seat; every player with its name, agent id, rank and league at the time; the ranking; the game options. The analysis reads the index to choose games (a batch, an opponent, our losses) without opening every log.

The records are not stored: they are rebuilt from the logs, so a fix in a conversion applies to every game.

The record of ADR 0027 gains an optional `origin` (CodinGame's game id, the kind, each seat's agent id and rank) and a new format version; CodinGame gives no answer times, so the record says they are unknown. The studio can then show a CodinGame game like any other, which is not part of this plan.

## Fetching and playing

`scripts/codingame.sh` (a shell script with `curl` and `jq`, like the other scripts; the token is ADR 0047's, read from `~/.config/codingame/token` and never printed):

- `fetch GAME [--submission ID]`: pages through the ranking games of the current (or a given) submission and stores the logs of the games that ended and are not stored yet. Rerunning it fetches only what is new.
- `batch GAME RELEASE.rs [--top N] [--games-per-opponent K] [--play]`: reads the leaderboard's top N (10), plans K games (10) against each on the maps of `games/<game>/codingame-seeds.txt`, and prints the plan; with `--play` it plays them one at a time, stores each log, and records the batch. Interrupted, it resumes from the batch file.
- `replay GAME RELEASE.rs [--lost] [--opponents top10] [--play]`: replays our ranking games (by default those we lost against the chosen opponents) with the release.
- On a 429 answer it waits the `Retry-After` the API gives; a failed test game (no code machine) waits a minute and counts as not played. It sends the release file unchanged, never sets `overwrite_ide_code` or `show_in_ide`, and pushes to `codingame-games` only what it fetched.

The fixed maps: 10 seeds drawn once at random, written as game options lines, so every release and every opponent play the same maps and batches compare. In test games our code is player 0; replays keep our seat in the ranking game, so the reports give results by seat.

## The analysis

All of it reads traces, which every game makes already.

- **Check.** Every converted game is replayed by the engine. The report starts with how many games were read, how many disagreed and their ids.
- **Profiles**, for each player (the top 10, or any name): games and results against us by seat and by map; the metrics' mean curves by turn and their means by phase, beside ours in the same games; when each kind of event and decisive moment first happens; how its wins come (end reasons, the moments that favoured it in its wins); its openings, as the most common token sequences and how often each one won.
- **Our weaknesses**: in our losses, the decisive moments against us by phase and by opponent, the first turn our metrics fall behind for good (as the metrics report finds it, ADR 0044), and the games where the gap was largest.
- **Judge comparison**: at each ply of a top player, the position as our bot reads it goes to the judge, the current release built for analysis (for A Code of Ice and Fire, the plan bot with `--features analysis`, which already reads banks of positions), with a fixed number of tries so that runs repeat. Its answer is compared with the top player's (same move, same first command, different), and the judge scores both moves. The report gives the agreement by player and phase and lists the plies with the largest score gap, with their boards: the moves the top player found that our evaluation underrates.
- **Report**: one Markdown file a batch, in `games/<game>/research/codingame/<date>-<name>.md`: the sources and the check, then a section per top player (profile), then our weaknesses, then the judge comparison, then what we could copy. Numbers come from code; the "what we could copy" part is written by the session from them. Every table is built from the trace's metric names and the game's event names, so a new game gets the same report with its own words.

## Steps

One pull request each; each ends with its checks and, from step 3, a run on real games.

1. This design and [ADR 0056](adr/0056-codingame-game-analysis.md).
2. `scripts/codingame.sh fetch` and its tests (the API stubbed), the branch `codingame-games` and its index, the record's `origin`; the first fetch of our ranking games.
3. A Code of Ice and Fire's conversion and the check, on every fetched game; the disagreements, if any, looked into.
4. Profiles, events, openings, weaknesses and the report on our past ranking games, the top 10 apart.
5. `batch` and `replay`, the fixed maps, and a first batch of `ice-and-fire-v014` against the top 10, with the owner's go-ahead.
6. The judge comparison.

Each step updates this page's status and [ARCHITECTURE.md](ARCHITECTURE.md).

## Questions left to the owner

- Whether CodinGame's terms allow publishing other players' game logs on a public branch (not checked). Until he decides, the branch holds only what he approves.
