# Game analysis: CodinGame games recorded and analysed

The design of the tool that records games played on CodinGame, rebuilds them with our engines, and reports how the top players play and where our bots lose to them ([ADR 0056](adr/0056-codingame-game-analysis.md)). It is a framework for any two-player game of the repository; A Code of Ice and Fire is the first game it is applied to. Status: the record's origin and the conversion with its check are built, and so are the profiles, the weaknesses, the economy-or-fighting analysis and the versions of players (the steps are at the end).

## What it is for

The owner's goal (2026-11-06): identify the strategies of the top bots, and our weaknesses against them. The first reports give one profile per top bot: its opening, when it attacks, how it uses its resources (in A Code of Ice and Fire: towers, mines and gold), how it beats us, and what we could copy.

## What CodinGame gives

From CodinGame's API reference, read 2026-11-06 ([docs/CODINGAME.md](CODINGAME.md#api)):

| Source | What we get | Cost |
| --- | --- | --- |
| Ranking games of our submissions (`GET .../battles`, `GET /games/{id}`) | Every game our current or a past submission played for its rank, newest first, with the opponents' names and agent ids | Requests only (120 a minute) |
| Test games (`POST .../test-games`) | A game of any code we send against chosen opponents: the players of our league and the lower ones; fixed game options or random ones; our code is player 0 | One at a time; 80 an hour, 600 a day, shared with the owner |
| Replays (`replay_game_id`) | One of our ranking games played again with other code: same options, same opponents, our seat kept | As a test game; only games of the current league, against opponents we can still test against |
| Games between other players | Not listed. A ranking game whose id is known can be read (`GET /games/{id}`), unless its challenge disables replays: the owner can give ids of top players' games from CodinGame's website | Requests only |

A game's log (JSON) has the game options (for A Code of Ice and Fire, `seed=<signed 64-bit number>`), the players, and frames. A frame that carries a player's output is that player's answer for one turn (A Code of Ice and Fire: `TRAIN 1 1 0;TRAIN 1 2 0`, commands separated by `;`); the frames between them describe what the commands did. The log ends with the ranking and the scores. Debug output (stderr) is there for our own code in test games only. A log of 42 answers is about 15 KB.

## Overview

```
CodinGame API ──scripts/codingame.sh──► branch claude/codingame-games: logs + index (network)
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
| Storage | The branch `claude/codingame-games`, its layout, index and leaderboard snapshots | Nothing |
| Conversion | Reading a log; the record's origin; the check report | Frames to turns' answers; game options to a seed |
| Rebuild and check | Running the check on every game, leaving out and listing those that disagree | Its engine and referee (they exist) |
| Trace | The trace type, game files, metrics reports (`cg-review`, exist) | Its review binary's metrics, phases and moments (exist) |
| Events | Rates and first turns of each event, by player and phase | Which events a ply has |
| Openings | Counting and comparing opening sequences by player | A side's first plies as tokens, the same from both seats |
| Profiles and weaknesses | All of it | Nothing beyond the above |
| Judge comparison | Running the judge on positions, agreement and score gaps by player and phase, the largest gaps | A position as the bot reads it; reading the judge's answer; comparing two moves |
| Report | The Markdown sections and tables, labelled with the game's own names | Optionally, a short section of its own |

### What a game provides

In Rust, a game provides two sets of parts. The conversion part, built and done, is the trait `FromCodinGame` in `cg_arena::codingame` (it builds records, so it lives with them; the log's types, `to_record` and `read_log` are there too, and the record's `origin` is in `cg_arena::record`, format 2). A game's review crate implements it (A Code of Ice and Fire: `IceAndFire`, 2 functions). The profiling hooks (key metrics, events, openings, which metrics are better when lower, the turns compared, the resource and its spending) are built: the trait `Profiling` in `cg_review::profile` (`cg-review` did not need `cg-arena`; it reads traces only), implemented by A Code of Ice and Fire's `ice-and-fire-review` in `profiling.rs`. The judge hooks are built too: the trait `Judging` in `cg_review::judge`, which turned out smaller than first sketched, since the game's judge runs outside the crate (a child process fed a bank, so `cg-review` still does not need `cg-arena`) and the module aggregates ready judgements; A Code of Ice and Fire implements it in `ice-and-fire-review`'s `judge.rs`, with the driver of `codingame-judge`. Together:

```rust
pub trait FromCodinGame {
    /// The game's folder name.
    const GAME: &'static str;
    /// The seed of a game from CodinGame's game options.
    fn seed(options: &str) -> Result<u64, String>;
    /// The answers of every turn, by seat, from the frames of a log.
    fn answers(log: &Log) -> Result<Vec<Vec<RecordedAnswer>>, String>;
}

// Built, in cg_review::profile:
pub trait Profiling {
    /// The game turns at which a profile compares metrics (a 200-turn game picks its own).
    fn turns() -> Vec<u32>;
    /// The own plies an opening covers.
    fn opening_plies() -> usize;
    /// The metrics a profile shows, by key, in order (about 8 to 12).
    fn key_metrics() -> Vec<&'static str>;
    /// The events of one ply of a trace (the player to act's), by name (e.g. "tower", "mine", "train level 3", "large cut").
    fn events(trace: &Trace, ply: usize) -> Vec<String>;
    /// The first own plies of a seat as one token string a ply, the same from both seats (mirrored when the map is).
    fn opening(trace: &Trace, seat: usize, plies: usize) -> Vec<String>;
    /// Whether less of a metric is better (a gap, an exposure, a loss): the curves' comparisons read it the other way.
    fn lower_is_better(key: &str) -> bool;
    /// The game turns at which metrics are ranked by how well they tell our wins from our losses.
    fn compare_turns() -> Vec<u32>;
    /// The key of the metric that holds the resource spent (gold) at the start of a ply.
    fn resource() -> &'static str;
    /// What the player to act of a ply spent in the commands carried out, by category (e.g. "train level 2", "mine", "tower").
    fn spending(trace: &Trace, ply: usize) -> Vec<(String, i64)>;
    /// What the game says about its economy and its fighting (below); `None` leaves the economy tables empty.
    fn economy() -> Option<EconomyHooks> { None }
}

// Built, in cg_review::profile (the economy-or-fighting analysis):
pub struct EconomyHooks {
    /// The cumulative metrics of the exchange of units: its balance (killed less lost), what the side removed, what it lost.
    pub balance: &'static str, pub killed: &'static str, pub lost: &'static str,
    /// What a side gains (cumulative metrics, summed) for each unit of the resource it spends so far on some categories of `spending`.
    pub efficiency: Vec<Efficiency>,        // Efficiency { name, gained: Vec<&str>, spent: Vec<&str> }
    /// What each spending category is for (e.g. economy: "mine"; army: the trainings; defence: "tower").
    pub roles: Vec<Role>,                   // Role { name, categories: Vec<&str> }
    /// The upkeep burden is the first metric over the second, per turn.
    pub upkeep: &'static str, pub gross_income: &'static str,
    /// The first turn this metric is above zero is the turn of first contact.
    pub contact: &'static str,
    /// The income metric (the value of a lead is read on it and on the trace's lead metric).
    pub income: &'static str,
    /// The decisive kinds of moments the tempo counts (e.g. those of `Decisive`).
    pub decisive: Vec<&'static str>,
}

// Built, in cg_review::moments (the decisive moments of our losses; it also uses key_metrics):
pub trait Decisive: Profiling {
    /// The kinds of moments that decide a game, in order of interest (e.g. "large cut", "HQ threat", "debt").
    fn decisive_kinds() -> Vec<&'static str>;
    /// The metric key that says how much of a side the opponent can take next (e.g. "exposed30"), if the game has one.
    fn exposure_metric() -> Option<&'static str>;
    /// The exposure values that count as high, ascending (e.g. 5 and 10).
    fn exposure_thresholds() -> Vec<i64>;
}

// Built, in cg_review::judge:
pub trait Judging {
    /// How the played move compares with the judge's own, both as the game writes a move.
    fn compare(played: &str, ours: &str) -> MoveMatch; // Same, First, Different
    /// A score gap (ours less played) above which the played move counts as scored much lower.
    fn gap_threshold() -> i64;
    /// A score at or beyond which (either sign) a position is decided (a forced win or loss).
    fn decided_bound() -> i64;
}
// The game's driver builds `Judgement`s (player, rank, game id, ply, turn, phase, both moves,
// both scores, the board before the ply) and writes the files of the output folder with
// `judge::judgements_csv::<Game>`, `summary_csv::<Game>`, `largest_gaps::<Game>` and `readme::<Game>`.
```

The trace and the rebuild come from the game's review binary (A Code of Ice and Fire has one, `ice-and-fire-review`; a new game adds its own, as ADR 0043 planned); the check compares its end with the log's ranking. The judge's input, a bank of positions with the move played, and its output, one line a position, are the game's to define (A Code of Ice and Fire's: `games/ice-and-fire/bots/plan/src/analysis.rs` and `analysis/judge.rs`). A game without a judge leaves that section out.

### Kinds of games

- Alternating turns (Ultimate Tic-Tac-Toe, A Code of Ice and Fire) and simultaneous turns both fit: a record's turn holds the answer of every seat that answered.
- Several commands a turn: the game's `answers` keeps the line as the referee reads it, and `events` counts what it holds.
- More than two players, or none (optimization and code golf puzzles), are outside the framework: the record and the trace have two seats. The index and the profiles keep players as lists, so the main step for such a game would be the record.

## Stored data

The branch `claude/codingame-games` has no pull request and is never merged, like `claude/handoff` (ADR 0024): sessions may push only to `claude/` branches (ADR 0021), and the pruning keeps a branch that has no pull request. It holds:

```
<game>/logs/<game_id>.json        the log as the API gave it (JSON), without the response's rate limits
<game>/index.jsonl                one line a game (below)
<game>/batches/<batch>.json       a test-game batch: release, opponents, maps, games played, failures
<game>/leaderboard/<date>.json    the leaderboard's top 100 at each fetch: ranks, names, agent ids
```

An index line: CodinGame's game id; the kind (`ranking`, `test`, `replay`); when it ended and when it was fetched; for ranking games the submission id, for test games the release and the batch; our seat; every player with its name and agent id; the ranking; the game options. Ranks come from the leaderboard snapshots, since logs and the list of ranking games give none; players are matched by name, since a player's agent id changes when they submit again. "The top 10" of a report is the top 10 of the snapshot it names. The analysis reads the index to choose games (a batch, an opponent, our losses) without opening every log. The lines of test games and replays keep the keys of ranking games, in the same order, and add optional ones at the end: `batch` (the batch's name) and `release` for a test game, `replay_of` (the ranking game replayed) and `release` for a replay. A test game's `ended_at` is when the script saw it done; a game stored by `fetch-ids` has `ended_at` null and `our_seat` null when we are not a player.

A batch file, `<game>/batches/<name>.json`, holds the name, game, puzzle, release and the SHA-256 of the file sent, when it was made, the leaderboard snapshot it took its opponents from, the opponents (name, agent id, rank), the maps (`game_options` lines) and one entry per planned game: opponent, agent id, options, `status` (`planned`, `done`, or `failed` after 3 attempts), attempts, the test game id, the game id of the log, seconds and the failure. It is rewritten after every game, so an interrupted batch resumes with its planned games only.

The records are not stored: they are rebuilt from the logs, so a fix in a conversion applies to every game.

The record of ADR 0027 gains an optional `origin` (CodinGame's game id, the kind, each seat's agent id; no rank, since ranks come from the leaderboard snapshots, as "Stored data" says) and a new format version; CodinGame gives no answer times, so the record says they are unknown. The studio can then show a CodinGame game like any other, which is not part of this plan.

## Fetching and playing

Games are also collected every day, by a scheduled GitHub workflow with a token the owner stored ([ADR 0058](adr/0058-codingame-collection-on-github-actions.md)): up to 500 test games of the current release against that day's top 10 ([ADR 0057](adr/0057-daily-collection-of-codingame-games.md)). The workflow `codingame-collect.yml` runs every 3 hours: it fetches our new ranking games, plays the day's batch as far as the quota floor (`--keep 100`) and the job's time (`--max-hours 5.25`) allow, resuming the same batch run after run, pushes to `claude/codingame-games`, and writes a job summary (games fetched and played, won and lost by opponent, quota left, top-10 agent changes). `scripts/codingame.sh` exits 3 when CodinGame refuses the token; `batch --max-games N` stops after N games (a trial). Setup and renewal are in [CODINGAME.md](CODINGAME.md), section "The collection's token"; the checks, in skill `collect-games`.

`scripts/codingame.sh` (a shell script with `curl` and `jq`, like the other scripts; the token is read from `~/.config/codingame/token` (a session's, ADR 0047) or from `CODINGAME_TOKEN_FILE` (the workflow's, ADR 0058) and never printed):

- `fetch GAME [--submission ID]`: takes a leaderboard snapshot, pages through the ranking games of the current (or a given) submission and stores the logs of the games that ended and are not stored yet. Rerunning it fetches only what is new.
- `fetch-ids GAME [--kind ranking|test|replay] ID...`: stores the logs of games whose ids the owner gives (for example top players' games seen on CodinGame's website), with index lines of that kind (`ranking` by default). Ids are never guessed or scanned.
- `batch GAME RELEASE.rs [--top N] [--opponents NAME,...] [--games-per-opponent K] [--seeds FILE] [--name NAME] [--play]`: takes a leaderboard snapshot, picks the top N (10) players but us, or the named ones (nicknames in the top 100), plans K games (10) against each on the maps of `games/<game>/codingame-seeds.txt` (cycled if it has fewer), and prints the plan: opponents, games, the quota and the time at about 25 s a game. Without `--play` it plays and writes nothing. With `--play` it plays the games one at a time, interleaved (every opponent on the first map, then the second...), stores each log and index line, rewrites the batch file, prints each game's result and the quota left, then each opponent's totals; it exits 1 if a game failed. A batch of the same name is resumed, not made again; without `--name`, the newest unfinished batch of the same release, maps and opponents is resumed (whatever day it was made), else a new one is named date-release-topN. A resume plays the planned games and the failed ones whose failure was not a refusal of the request (a code such as `INVALID_OPPONENT`). Three games in a row that fail (after 3 attempts each, or a 409 other than a running test game) stop the run with exit 1 and stay planned, so an outage burns about 9 starts. The release must be a file of `games/<game>/releases/`.
- `replay GAME RELEASE.rs [--opponents NAME,...] [--all] [--limit N] [--play]`: replays with the release our stored ranking games that we lost (`--all`: won too), against the named opponents (default: any), newest first, at most N (20), skipping those already replayed with this release. Without `--play` it lists them.
- Daily collection ([ADR 0057](adr/0057-daily-collection-of-codingame-games.md)): `batch --keep N` stops cleanly (exit 0, games left planned) before a test game that would leave fewer than N of the 600 of the last 24 hours, reading `GET /limits` before every start (a limit it cannot read stops it with exit 1), and `--max-hours H` stops it cleanly after H hours, and before any wait (the quota window, a retry, a running game) that would cross that time; a TERM or INT publishes what was stored before it exits; `games/<game>/codingame-submissions.txt` lists the release of each ranking submission (`<submission id> <release> <date>`), which `fetch` writes as `release` in the index lines it adds and `fetch GAME --backfill-releases` adds to the lines that lack it, with no request; `games/<game>/codingame-seeds-50.txt` holds the 50 maps of the daily batch; the dedicated session follows the skill `collect-games`.
- All take `--data DIR` and `--push` as `fetch` does; `--push` commits `chore(repo): codingame games: <game>, <what>` with the logs, index, snapshots and batch files only.
- On a 429 answer it waits the `Retry-After` the API gives; a failed test game (no code machine) waits a minute (`CODINGAME_RETRY_WAIT`) and is played again, at most 3 attempts, and it still counts against the limits, as every start does; before a start with no test games left in the tightest window it waits that window's reset. A refused request (`INVALID_OPPONENT`...) fails its game at once. It sends the release file unchanged, never sets `overwrite_ide_code` or `show_in_ide`, never makes a submission, and pushes to `claude/codingame-games` only what it fetched. A rejected push keeps the worktree and prints its path, the commit and the command to push it by hand; a run that fails midway still commits what it stored.

The fixed maps: 10 seeds drawn once at random, written as game options lines, so every release and every opponent play the same maps and batches compare. In test games our code is player 0; replays keep our seat in the ranking game, so the reports give results by seat.

## The analysis

All of it reads traces, which a game's review binary makes (A Code of Ice and Fire's today).

- **Check.** Every converted game is replayed by the engine. The report starts with how many games were read, how many disagreed and their ids.
- **Profiles**, for each player (the top 10, or any name): games and results against us by seat and by map; the metrics' mean curves by turn and their means by phase, beside ours in the same games; when each kind of event and decisive moment first happens; how its wins come (end reasons, the moments that favoured it in its wins); its openings, as the most common token sequences and how often each one won.
- **Our weaknesses**: in our losses, the decisive moments against us by phase and by opponent, the first turn of each loss from which a metric stays behind to the end (new, computed per game from its trace), and the games where the gap was largest.
- **Curves** (static SVG charts, written as text, no dependency): for each key metric, the median at each game turn (1 to the last turn that at least half of the games reach, each side read right after its own ply) with a light band for the middle half of the games, in two sets: us against the top players in the games between them, and our wins against our losses; and for each top player, us against it on the three metrics that tell our wins from our losses best. For each event the game names, the share of games in which a side does it at each turn, in the same two sets. Blue is always the first group (us, wins) and orange the second (the top players, losses); the colours switch to a dark palette when the viewer prefers it; a dashed line marks the turn from which the two groups stay apart.
- **What separates our wins from our losses**: for each metric and turn, the AUC of our wins against our losses (the share of pairs, one value from each group, where the win's value is the better one, higher or lower by `lower_is_better`, ties counting half; 0.5 is no difference; a turn needs 5 values on each side). The metrics are ranked by their mean AUC at the game's compared turns, with the first turn from which the AUC stays at 0.7 or more to the end ("separate for good"). The same is done for the top players against us (either side counts: 0.7 or more, or 0.3 or less). In "Our weaknesses", the "stays below to the end" share is also given for our wins, as a control: a metric that is as often below in wins does not decide the games.
- **Spending**: from the commands carried out, the mean amount spent per own turn on each category (the game's `spending`) by phase, and the share of the resource in hand left unspent at the end of the turn (mean over turns), for the top players, us in the same games, our wins and our losses, and for each focus player.
- **Economy or fighting** (`economy_*.csv`, the charts `economy-SET-*.svg`, a section of `signals.md`): is a strategy better because of its economy or because of its fighting? The two explanations side by side, from the game's `economy` hooks:
  - *Exchange*: the value (training cost) of the enemy units a side removed less the value of its own it lost, cumulated, by phase (the mean at the end of each phase) and as a curve by turn, for the top players against us and our wins against our losses. A game decides what counts as a kill; in A Code of Ice and Fire, units taken by a move or a training, and units that die because the side's play cut them off from their HQ (they die at their owner's next turn: the kill goes to the side whose turn cut them); a debt wipe-out is a loss, not a kill.
  - *Efficiency*: what a side gained (cells taken; value killed) for each unit of resource it spent (all of it; on units), cumulated: the median over games at each turn of the ratio of the two running totals.
  - *Investment mix*: the share of its spending on each role the game names (economy, army, defence) in each phase.
  - *Upkeep burden*: upkeep as a share of gross income, by phase and by turn.
  - *Tempo*: the turn of first contact, and the turns from it to the first decisive moment in the side's favour (the decisive kinds the game names), by group.
  - *Value of a lead*: in our games, for the lead metric and the income at each compared turn, the wins out of the games from each size of lead (sizes relative to the median lead at that turn).
  - `signals.md` gives one line for each set of curves that sets the numbers of the two explanations side by side at the last compared turn, with no interpretation.
- **Versions** (`profile_versions.csv`, `versions.csv`; [ADR 0057](adr/0057-daily-collection-of-codingame-games.md), decision 3): `--release NAME` keeps only our games played by that release (the index's `release`); a player who appears with several agent ids is listed by version (games, results against us, first and last day seen); `versions.csv` gives each top player's agent id in the leaderboard snapshot of each day with a `changed` flag when it differs from the previous day's, and `signals.md` lists the changes.
- **Data**: the long tables are not the report. The generator writes a folder: one CSV for each kind of table (long format, a `player` or `group` column), the turn-by-turn data of every metric and event (n, median, quartiles, mean) for each focus player against us, all of them, our wins and our losses, the charts, a `README.md` with each file's columns, and `signals.md`, the strongest findings picked by code (the metrics that separate our wins from our losses, the biggest gaps between the top players and us, the turns where an event's rate differs by 20 points or more), each pointing to its chart and data file, with no interpretation.
- **Judge comparison**: at each ply of a top player, the position as our bot reads it goes to the judge, the current release built for analysis (for A Code of Ice and Fire, the plan bot with `--features analysis`, which already reads banks of positions), with a fixed number of tries so that runs repeat. The plan bot's analysis build has a `judge` mode that, besides choosing its own plan, scores a given one; the bank's block carries the move the top player played (a `played` line). The played plan is scored as the search scores a candidate: its accepted commands applied to the position, the opponent's replies to it searched, and the worst evaluation over the search's final memory of replies and those replies taken (`search::judge`). The opening is treated as over, as `bank` has it. `codingame-judge` runs it (in several processes) on every ply of the chosen players and compares the answers with the top player's (same move, same first command, different), with the scores of both. The output (`cg_review::judge`) is a folder: `judgements.csv` (a row a ply), `summary.csv` (the agreement and the score gap by player and phase: median, and the share of plies where the played plan scores lower than ours by more than the game's threshold), `largest-gaps.md` (the 20 plies with the largest gap, with their boards) and a `README.md`; the committed report refers to it and embeds what it needs. The largest gaps point at the moves the top player found that our evaluation underrates. A position is decided when either score is a forced result (the game gives the bound: for A Code of Ice and Fire, a quarter of `WIN`, which separates a lethal plan and the `WIN / 2` penalty for letting the opponent take the HQ from every board evaluation); decided plies are counted apart in `summary.csv` and left out of the gap statistics and of the list. On a lethal position the search stops before its final choice, so the played plan is scored with the evaluation of its end state, not as a candidate; that does not matter, since those plies are decided and left out.
- **Decisive moments** (`codingame-moments`): for each of our losses, the first moment against us of the game's decisive kinds (`Decisive::decisive_kinds`), with the board before the play that made it and the board after it, our exposure (`Decisive::exposure_metric`) right after each of our last 3 own plies before it (did it grow?), both sides' key metrics read right after their own last ply before it, and the moment's text. The summary counts the kinds and their median turn for the focus players (the top N of the snapshot) and for the others, counts the losses with no such moment, and gives for each kind the median exposure at the last own ply before it and, for each threshold the game gives (`Decisive::exposure_thresholds`), the share of losses at or above it at each of the last 3 own plies, beside the same numbers in our wins that reached the turn of each such moment, read before the first opponent ply of that turn and pooled (the matched control: is high exposure special to losses?), each split into focus players and others. With `--out DIR` these go to files (`moments.csv`, one row per loss and per control win; `summary.csv` in long format; `examples.md`; a `README.md` that names the columns) for a report written by hand to link, and the examples are, for each focus player, up to K losses (3 by default), the boards one after the other in code blocks. It does not propose the defence move (that is a question for the judge).
- **Report**: one Markdown file a batch, in `games/<game>/research/codingame/<date>-<name>.md`, with its data folder beside it (`<date>-<name>/`): text written by the session from the generated charts, CSV and signals, embedding the curves and linking the CSV, then what we could copy. The judge comparison and the decisive moments write their own folders (`<date>-<name>-judge/`, `<date>-<name>-decisive-moments/`), which the report refers to and embeds what it needs from. Numbers come from code; the reading of them is the session's.

## Steps

One pull request each; each ends with its checks and, from step 3, a run on real games.

1. This design and [ADR 0056](adr/0056-codingame-game-analysis.md).
2. `scripts/codingame.sh fetch` and its tests (the API stubbed), the branch `claude/codingame-games`, its index and leaderboard snapshots, the record's `origin` (done: record format 2, read by the studio in formats 1 and 2); the first fetch of our ranking games.
3. A Code of Ice and Fire's conversion and the check, on every fetched game; the disagreements, if any, looked into. Done: `ice-and-fire-review codingame` converts logs and checks 4 things (every answer used, the winner, every command read, and per ply the commands carried out and refused against CodinGame's summaries). On 222 of our ranking games, all 222 agree; the fetch script is another pull request.
4. Profiles, events, openings, weaknesses and the report on our past ranking games, the top 10 apart. Done: `ice-and-fire-review codingame-report` (generic code in `cg_review::profile`, the hooks in `games/ice-and-fire/review/src/profiling.rs`); the first report, on 322 games (222 ranking games and 100 test games against the top 10), is in `games/ice-and-fire/research/codingame/2026-11-06-v014-against-the-top-10.md`. It differs from the plan above in two ways: the profile gives results by seat only (not by map), and the metrics are medians (theirs and ours) at game turns 5, 10, 15, 20 and 30 and means by phase, not whole curves (a side's value "at turn t" is read right after its own ply of turn t, in the snapshot of the next ply of the trace, which shows the first side's values as its play left them and only changes the other side's; a side's last ply is read at its own snapshot; the phase means average the values at the starts of a side's own plies; events and openings count the commands carried out, not the refused ones); `--json` gives the numbers for charts.
5. `batch`, `replay` and `fetch-ids` (done in `scripts/codingame.sh`, tested with the API stubbed) and the fixed maps (`games/ice-and-fire/codingame-seeds.txt`); the first batch of `ice-and-fire-v014` against the top 10 waits for the owner's go-ahead.
6. The plan bot's `judge` mode and the judge comparison.

Each step updates this page's status and [ARCHITECTURE.md](ARCHITECTURE.md).

## The owner's answers (2026-11-06)

- Other players' logs: the branch stores them in full, as the API gives them ("Store full logs"); CodinGame shows the same games to anyone as replays. Its terms on republishing logs were not checked.
- Games between top players: no API call lists them, and a test game always plays our code, so they cannot be played either. `fetch-ids` stays for any game id that turns up later; the analysis rests on our ranking games and test-game batches.
