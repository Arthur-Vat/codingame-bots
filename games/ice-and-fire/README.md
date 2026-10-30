# A Code of Ice and Fire

CodinGame's [A Code of Ice and Fire](https://www.codingame.com/multiplayer/bot-programming/a-code-of-ice-and-fire) is the second game of this framework, named by the owner on 2026-10-17 (phase 7 of the [main roadmap](../../docs/ROADMAP.md)). Two players on a 12×12 map build armies, buildings and territory to capture the opposing headquarters. It began as a 10-day contest, now over; bots can still be submitted to it as a multiplayer game (source's README).

**Current release:** [`ice-and-fire-v010`](releases/ice-and-fire-v010.rs), candidate of E023 (v009 whose evaluation counts a cell the enemy can take at its next turn at 0.75 of a cell's value; SPRT pending against `ice-and-fire-v009`; [journal](journal/E023-safe-territory.md)). Before it: [`ice-and-fire-v009`](releases/ice-and-fire-v009.rs), candidate of E020 (v008 whose evaluation counts a level-2 unit at 5 rather than its training cost of 20; SPRT accepted against `ice-and-fire-v008`, +39.8 Elo at CodinGame's limits; [journal](journal/E020-level-two-units.md)). Before it: [`ice-and-fire-v008`](releases/ice-and-fire-v008.rs), candidate of E019 (v007 whose search also plans against the opponent's own plan, the same climb run from its seat with moves, trainings and cuts; SPRT accepted against `ice-and-fire-v007`, +78.5 Elo at CodinGame's limits; [journal](journal/E019-opponent-plan.md)). Before it: [`ice-and-fire-v007`](releases/ice-and-fire-v007.rs), candidate of E015 (v006 with the cheapest chain to an HQ found on bitboards in the evaluation, the same costs 10 to 20 times faster, so the search scores more plans in its time; SPRT accepted against `ice-and-fire-v006`, +57.7 Elo at CodinGame's limits; [journal](journal/E015-path-cost.md)). Before it: [`ice-and-fire-v006`](releases/ice-and-fire-v006.rs), candidate of E014 (the `plan` bot whose middle-game search climbs again from other first actions until its time is up, with cuts of up to 5 trainings; SPRT accepted against `ice-and-fire-v005`, +74.1 Elo at CodinGame's limits; [journal](journal/E014-search-restarts.md)). Before it: [`ice-and-fire-v005`](releases/ice-and-fire-v005.rs), candidate of E010 (the `plan` bot with an opening planned for our side alone over its first 7 turns, compact and toward the enemy HQ, and a shape term in the middle-game evaluation; SPRT accepted against `ice-and-fire-v004`, +44.1 Elo at CodinGame's limits; [journal](journal/E010-solo-opening.md)). Before it: [`ice-and-fire-v004`](releases/ice-and-fire-v004.rs), candidate of E004 (the `plan` bot whose opening, v003's rules, takes on the first turn the weights that reach the best middle game by a heuristic score, the economy then the Voronoi share; SPRT accepted against `ice-and-fire-v003`, +51.9 Elo at CodinGame's limits; [journal](journal/E004-planned-opening.md)). Before it: [`ice-and-fire-v003`](releases/ice-and-fire-v003.rs) (E003, an opening by rules until the territories meet), SPRT accepted against v002, +299.8 Elo.

**Status:** stage 10, iteration, in progress: E023 (candidate `ice-and-fire-v010`) counts a cell the enemy can take at its next turn at 0.75 of its value, after a study of towers and threats in v009's games ([journal](journal/E023-safe-territory.md)); E020 (candidate `ice-and-fire-v009`) counts a level-2 unit at 5 in the evaluation, after a study of v008's lost games ([journal](journal/E020-level-two-units.md)); E019 (`ice-and-fire-v008`) adds the opponent's own plan to the replies the search plans against, after a study of v007's lost games ([journal](journal/E019-opponent-plan.md)); E015 (`ice-and-fire-v007`) finds the cheapest chain to an HQ on bitboards, so the search scores more plans ([journal](journal/E015-path-cost.md)); E014 (`ice-and-fire-v006`) lets the middle-game search climb again from other first actions until its time is up, with cuts of up to 5 trainings ([journal](journal/E014-search-restarts.md)); E010 (`ice-and-fire-v005`) plans the early game for our side alone, compact and toward the enemy HQ, and counts shape in the middle game ([journal](journal/E010-solo-opening.md)); E004 (`ice-and-fire-v004`) planned the early game on the first turn, after E003's opening by rules ([research/early-game.md](research/early-game.md)); E007, a gold reserve in the middle game, was dropped ([journal](journal/E007-reserve.md)). Stage 9 ended with `plan` as the approach (`ice-and-fire-v002`), after the ladder of reference bots ([ladder/](ladder/), E001) built on the shared tactics ([tactics/](tactics/)); stage 5 ended with the `rules-check` run on CodinGame, no difference in 4 Bronze games ([ROADMAP.md](ROADMAP.md)): [RULES.md](RULES.md), [MODEL.md](MODEL.md), a readable referee with its map generator (which reproduces the source's maps), a fast engine checked against it, an arena, the `random` bot that anchors the ratings, the `wait` bot, a bot per league, the `rules-check` bot, the game's [numbers](#numbers), and its [studio](#studio) adapter.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [MAP_GENERATOR.md](MAP_GENERATOR.md) | How the source draws a map, in our own words: the spec of our referee's generator ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)) |
| [MODEL.md](MODEL.md) | The game as a search problem: numbers, a formal model, what follows for bots |
| [research/](research/README.md) | Stage 8: what strong bots do, the approaches ranked, the prototypes proposed for stage 9; [early-game.md](research/early-game.md): the economy of the opening and the openings compared (E003); [towers-and-threats.md](research/towers-and-threats.md): towers, HQ threats and cuts in v009's games, and the parameters proposed for E021 to E025 |
| [ROADMAP.md](ROADMAP.md) | The game's stages in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md)) |
| [referee/](referee/) | `ice-and-fire-referee`: readable reference implementation of the rules, with a map generator that reproduces the source's maps ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)), for the arena |
| [engine/](engine/) | `ice-and-fire-engine`: the fast, standard-library engine for bots (bitboards, random turns, CodinGame's text protocol), with parity tests against the referee and the speed example |
| [policy/](policy/) | `ice-and-fire-policy`: the greedy turn policy and the input/output loop that the per-league bots share; its tests play every league's bot against the referee |
| [tactics/](tactics/) | `ice-and-fire-tactics`: what the first real bots share, in the standard library only: the cheapest chain of trainings (aware of towers), a cut finder and a static evaluation of a state, with a timing example; the weights of the evaluation and the scores of the cuts are placeholders for later experiments |
| [opening/](opening/) | `ice-and-fire-opening`: the early game, in the standard library only: v003's `econ` opening with its weights as parameters, when the middle game starts, a hand-built score in gold that compares openings there, rollouts against v003's opening, a beam search over whole turns, a portfolio of weights, the planned opening the `plan` bot plays (E004), and `solo`, the early game planned for one side alone with its shape measures (E010); its harness example `early` plays openings against each other up to the middle game ([research/early-game.md](research/early-game.md#the-middle-game-score-e004)), and its example `solo` plays one side alone and shows the boards |
| [review/](review/) | `ice-and-fire-review`: turns recorded games into review files (a game file with every ply's board, metrics and commands, the metrics as CSV, the phases and decisive moments) and summarizes a reviewed batch ([below](#game-reviews)); from records directly, it also studies towers, threats and cuts, draws a bank of middle-game positions for the plan bot's analysis mode, and reports a screening match ([below](#towers-threats-position-bank-and-screening)) |
| [arena/](arena/) | `ice-and-fire-arena`: plays bots against each other through the referee, in Bronze (all the rules); its example `opening` measures each bot's opening from game records, up to the middle game's score ([below](#arena)) |
| [studio/](studio/) | `ice-and-fire-studio`: the game's adapter for the studio, bot seats only ([below](#studio)) |
| [ladder/](ladder/) | `ice-and-fire-ladder`: the ladder of reference bots, bronze's turn plus the steps of the shared tactics, one more step for each bot (`chain`, `guard`, `cutter`, `banker`); its play does not change once merged, since the bots and their ratings are built from it |
| [bots/](bots/) | The bots, listed in "Bots" below |
| [journal/](journal/README.md) | One entry per experiment ([E001](journal/E001-ladder.md): the ladder of reference bots) |
| [releases/](releases/) | The released bots, one file each, pasted into CodinGame; never changed |
| [evaluation.env](evaluation.env) | Settings of the SPRT, the league (opponents, pairs, canary) and the arena for evaluations ([ADR 0012](../../docs/adr/0012-evaluation.md), [ADR 0038](../../docs/adr/0038-incremental-league.md)) |
| [league.jsonl](league.jsonl) | The league's results, one line per pairing ([ADR 0038](../../docs/adr/0038-incremental-league.md)); bootstrapped on 2026-10-21 with `bronze`, `chain`, `guard`, `cutter` and `ice-and-fire-v001`, 800 pairs per pairing at CodinGame's limits (League run 39678931208, ADR 0042) |
| [LEADERBOARD.md](LEADERBOARD.md) | The ratings of the releases and the baseline bots, generated from `league.jsonl` |
| [ci.sh](ci.sh) | The game's CI steps, run after the bots are bundled ([ADR 0029](../../docs/adr/0029-ci-scoped-by-area-and-game.md)): the arena games, the evaluation controls and a small league, then the engine speed |

## Bots

Each league has its own bot, since the input never names the league; each is a small `main` that fixes its league and runs the policy of [policy/](policy/). The four bots of the ladder (`chain`, `guard`, `cutter`, `banker`) are each a small `main` that runs [ladder/](ladder/) with one more step than the bot before; they are reference bots for the league and the studio, and only `banker` was released (as `ice-and-fire-v001`). Bundles are `target/cg/ice-and-fire-<bot>.rs`.

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [random](bots/random/) | Plays `State::random_turn` (moves, trainings and buildings the rules accept), seeded by `CG_SEED` | The anchor of the ratings and the smoke test's opponent |
| [wait](bots/wait/) | Reads every turn's input and answers `WAIT` | An always-valid opponent for the arena and the CI |
| [wood3](bots/wood3/) | The greedy policy with the Wood 3 rules (level 1 only, level 1 beats level 1) | Promotion out of Wood 3 |
| [wood2](bots/wood2/) | The same policy with levels 1 to 3 | Promotion out of Wood 2 |
| [wood1](bots/wood1/) | The same policy, building mines on free mine spots | Promotion out of Wood 1 |
| [bronze](bots/bronze/) | The same policy with all the rules: a tower next to its HQ when enemy units come within 3 steps. Also serves above Bronze until real bots exist | Playing Bronze, where the `rules-check` bot runs |
| [chain](bots/chain/) | Bronze's turn, with the lethal chain of the shared [tactics](tactics/) (cheapest chain of trainings, aware of towers) | Ladder step 1 (E001), a reference bot |
| [guard](bots/guard/) | `chain`, plus a defence: while the enemy could take its HQ next turn, the tower or training that most raises what the enemy lacks, and a last check that undoes spending that exposes the HQ | Ladder step 2, a reference bot |
| [cutter](bots/cutter/) | `guard`, plus up to 2 cuts of the enemy's territory a turn that leave the HQ no more exposed | Ladder step 3, a reference bot |
| [banker](bots/banker/) | `cutter`, plus banking: no more mines or trainings when the chain to the enemy HQ is affordable next turn | Ladder step 4, the game's first release `ice-and-fire-v001` |
| [plan](bots/plan/) | A plan search after the contest's winner: lethal first; until the middle game (it can pay a chain of trainings to an enemy cell), a planned opening ([opening/](opening/)'s `planned` with the solo planner, E010): v003's opening by rules with weights for a compact shape and a pull toward the enemy HQ, chosen turn by turn by an anytime beam search over our next 7 turns as if the opponent passed (`solo`; about 610 ms on the first turn), followed while our side is as predicted and planned again within the turn's time otherwise; else hill climbing over single actions and macro actions (cuts, a lethal chain), each plan scored against a memory of the opponent's replies (its chain, cuts, bronze's greedy turn and, since E019, its own plan by the same climb from its seat), with an evaluation that counts a side unable to pay its upkeep as having lost its units, a level-2 unit as worth 5 (E020), and takes 2 for each joint of a side's territory and each cell one capture would cut off; 40 ms × `CG_TIME_SCALE` a turn | Experiments E002 (`ice-and-fire-v002`), E003 (`ice-and-fire-v003`), E004 (`ice-and-fire-v004`) and E010, candidate `ice-and-fire-v005` ([journal](journal/E010-solo-opening.md)) |
| [rules-check](bots/rules-check/) | Plays random turns and checks every turn's input against RULES.md and the engine; logs the game's map | Checking our rules against CodinGame's, in Bronze or above ([below](#checking-the-rules-on-codingame)) |

The policy plans a turn on a scratch copy of the engine's state and keeps only the commands the rules accept. In order: a lethal move or chain of trainings to the enemy HQ if the gold pays for it (found by a cheapest-path search through the cells next to the territory); moves, the unit nearest the enemy HQ first, each to the neighbouring cell that gains the most (new cells, enemy cells, kills, cuts of enemy territory, mine spots) and gets nearer; a tower by the HQ under threat, else mines; trainings on the best cells next to the territory, level 1 unless a higher level is the only way to enter (to kill a unit or pass a tower), with income after upkeep kept at least 0 and a small gold reserve. It does no search and takes a few ms per turn (tested under 20 ms). It is a means of promotion, not a strong bot; it beats the source's league bosses in our referee (below).

Measured on 2026-10-19 over 200 maps in each seat (400 games per row, Rust 1.90.0 and current, release build), each bot in its own league against the reference referee (`cargo test -p ice-and-fire-policy --release`):

| League | Against `WAIT` | Against random turns (`State::random_turn`) |
| --- | --- | --- |
| Wood 3 | 400/400 | 400/400 |
| Wood 2 | 400/400 | 400/400 |
| Wood 1 | 400/400 | 400/400 |
| Bronze | 400/400 | 400/400 |

The game's source has a boss for each league up to Bronze and none above: `config/level1/Boss.py3` (Wood 3), `config/level2/Boss.py3` (Wood 2), `config/level3/Boss.py3` (Wood 1), and the default boss `config/Boss.cpp`, which plays in Bronze since that league has none of its own. They are GPL, so they are neither copied nor translated here. Measured on 2026-10-19, on the maps of ADR 0041 (the same result as on our earlier generator's maps), with the bosses run unchanged outside the repository as processes, through a local build of the arena whose referee played each league's rules: 100 seat-swapped pairs per league at CodinGame's exact time limits, each bot against its league's boss. Each bot won 200 of 200 games, with no timeout, crash or invalid answer on either side. CodinGame's bosses may differ from the source's.

## Arena

```sh
cargo build --release -p ice-and-fire-arena -p ice-and-fire-bot-random -p ice-and-fire-bot-bronze
target/release/ice-and-fire-arena --help
target/release/ice-and-fire-arena match \
  --bot random=target/release/ice-and-fire-bot-random \
  --bot bronze=target/release/ice-and-fire-bot-bronze --pairs 100
```

The arena plays Bronze games: all the rules, on the map of the pair's seed ([ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md)). Each pair plays one seed twice with the seats swapped, so both bots play the same map from both seats. The referee ignores `--opening-plies`: the maps vary the games. `sprt` and `league` take the same options, and `scripts/sprt.sh ice-and-fire CANDIDATE.rs` uses [evaluation.env](evaluation.env): CodinGame's exact limits (50 ms per turn, 1,000 ms for the first, no tolerance) for the smoke test, the SPRT, the league and local screening, with 800 pairs per league pairing, a league of up to 8 pairings that fits the job's 300 minutes only if a game averages 5.6 s or less (about 140 answers of 40 ms), to be checked with the first search bot's game length ([ADR 0042](../../docs/adr/0042-official-time-limits-and-800-pairs.md)). `ci.sh` plays the arena's games in CI, also at CodinGame's exact limits.

To measure the openings of a match's games, keep every game with `--records DIR` and run `cargo run --release -p ice-and-fire-arena --example opening -- DIR`: for each bot, its result, its wipe-outs by debt, and at contact (the first start of its turn with the territories at most 2 steps apart) its cells, income, gold, units, mines and Voronoi share, then its exposure after that turn (the opponent's best cut and its gold against the cheapest chain to the bot's HQ); and at the middle game, both sides' score (`ice_and_fire_opening::score`), its terms and how often its sign agrees with the winner (`OPENING_CSV=FILE` writes them per game). `--maps N` prints the map facts the opening depends on, over seeds 1 to N ([research/early-game.md](research/early-game.md)).

## Game reviews

```sh
target/release/ice-and-fire-arena match --bot A=... --bot B=... --pairs 10 --records DIR
cargo run --release -p ice-and-fire-review -- games --out BATCH [--text TEXT] DIR/*.json
cargo run --release -p ice-and-fire-review -- summary BATCH
```

`games` writes, for each record, `NAME.md`, the game file a reviewer reads (in `--text`'s directory when given: the game files are large and the records rebuild them): the rules in short, the metrics' meanings, the phases, the decisive moments, then every ply (one player's turn) with the board in two characters a cell, both sides' metrics and the commands; `NAME.csv`, the metrics; and `NAME.summary.json`, the result, phases and moments. It refuses to write over a record.

- **Board:** the first character is the owner (`R` red, `B` blue, upper case when active, lower case when cut off from the HQ), the second the content (`.` empty, `1` to `3` a unit, `M` mine, `T` tower, `H` HQ, `*` an empty mine spot); `##` is void.
- **Metrics,** 50 for each side at each ply, each with its meaning in the game file (`ice-and-fire-review`'s `trace::metrics`):
  - economy: gold, income before and after upkeep, upkeep, mines and free mine spots;
  - territory: active and cut-off cells, the Voronoi share, frontier, perimeter, room to grow, cells nearer the enemy HQ, spread from the HQ, steps to the enemy HQ;
  - shape: joints (cells whose loss cuts others off), the most and the total cut off by single cells, line ends, joints on the frontier, empty and guarded;
  - army: units by level and their cost, units on the frontier, deep inside and near the HQ, units lost, enemy units in contact;
  - tactics: what the enemy could cut with 30 and 50 gold (the most cells among its 8 best cuts), what the side could cut with its gold, the gold it lacks for a chain to the enemy HQ, towers and the cells they protect;
  - its last turn: gold spent on units and on buildings, the share kept, moves, trainings, builds, cells taken and lost, and the change of its cells and material;
  - and its material (gold + 3 × income + 4 × cells + units' cost + 10 × towers), whose difference is the lead.
- **Phases:** the early game until the middle game (the player to act can pay a chain of trainings to an enemy cell, `ice_and_fire_opening::middle`); the endgame from the first ply of the middle game on at which a side has twice the other's active cells (so the endgame's `cells` go with the winner partly by construction) or can pay a chain to the enemy HQ with its gold and one turn of income.
- **Decisive moments:** a large cut (a side loses 30% of its active cells, and 6 or more, in one ply), a lead change in material (by 20 or more) or a swing of 60 in a turn, the first time a side can pay a chain to the enemy HQ, and a debt that wipes out an army.

`summary` reads a batch's files, and the reviewers' notes in `NAME.review.md` when present (the last fenced `json` block: a list of `{"ply", "side", "verdict", "note"}`), and prints which metrics go with the winner at three points of the game, how often each kind of moment's side won, and whether the plays judged good or bad moved the lead as judged.

### Metrics reports

A large batch measured by code and read by one model ([ADR 0044](../../docs/adr/0044-metrics-reports.md)):

```sh
target/release/ice-and-fire-arena match --bot A=... --bot B=... --pairs 500 --records DIR
cargo run --release -p ice-and-fire-review -- games --out GAMES --text TEXT DIR/*.json
cargo run --release -p ice-and-fire-review -- report GAMES --curves curves.json > report.md
```

`report` reads the games' `NAME.csv` and `NAME.summary.json` and prints, for each metric, from which turn it tells the winner (the winner's value higher, or lower, in at least 70% of the games still running, a tie counting half), its share at turns 5 to 20, the ten metrics that tell the winner best at turns 8, 12, 16 and 20 with the winner's and loser's means, and the two bots' means turn by turn. It then cuts the games into slices, so that an average over the whole batch does not hide a tendency: the shortest, regular and longest thirds by length, each seating, each bot's wins, and the games the side ahead in material at turn 10 went on to win or to lose; for each slice it gives when each metric tells the winner, and its share 5 turns and 1 turn before each game's last turn (games lined up on their end), and for each seating the metrics on which the two bots differ most. `--curves` writes every slice's numbers for every turn, from the start and before the end, as JSON, for charts.

### Towers, threats, position bank and screening

Commands that read the arena's records directly (`--jobs N` replays N records at a time):

```sh
cargo run --release -p ice-and-fire-review -- towers [--csv DIR] DIR/*.json > towers.md
cargo run --release -p ice-and-fire-review -- bank --out bank.txt --count 5000 --seed 22022 DIR/*.json
cargo +1.90.0 run --release -p ice-and-fire-bot-plan --features analysis -- \
  bank --positions bank.txt --tries 400 [--out bank.tsv] > bank.md
cargo run --release -p ice-and-fire-review -- screening DIR/*.json > screening.md
```

- `towers` prints the tower and threat study of the games ([research/towers-and-threats.md](research/towers-and-threats.md) has the first one, on v009's games): every tower built (when, both sides' gold and income, the enemy's cheapest chain to the builder's HQ just before and after it, the pressure term it gained) and what became of it (whether it later kept an enemy training or unit out of a cell it alone protects, whether the enemy came to afford the chain it raised, whether it was cut off or lost, and when); at every ply, the turns until the player to act could afford its cheapest chain to the enemy HQ against whether and when it took it, with a geometric fit of the chance by turns; and at every ply its best cut with its gold now and with one and two more incomes, against the cells the enemy lost. The chain is the one of the evaluation's pressure term (`cost_to` without moves), the cuts the trace's. `--csv` writes the rows as `towers.csv`, `threats.csv` and `cuts.csv`.
- `bank` draws middle-game positions (from the middle game's first ply to the endgame's, as the trace cuts them) at random with the seed, and writes them as the player to act receives them. The plan bot built with `--features analysis` reads such a bank (`bank` in its `analysis` module; bots never carry that code, since the bundler drops it): on each position it times the evaluation (in that build, which also reads the variant switch, so slightly above the shipped cost), and runs the search with a fixed number of tries and no deadline with v009's evaluation, with twice the tries (a control of the search's own changes), without the pressure term and without the towers' static value, then reports how often the plan and its first command change, and into what.
- `screening` prints, per bot of a match (a variant against the release, say): its score overall and by seat, the towers, mines and units of each level it trained a game, the gold it kept unspent after its turns on average and as a share, its timeouts and crashes, and its think times (median, 99% and maximum after the first answer, and the slowest first answer). It counts as the metrics report does: what the rules accepted. A turn that starts with no gold counts as all kept; the 99% time is the value at the rounded 99% rank, so it can differ slightly from the arena's own figure.

## Studio

The studio replays and plays this game, but only between bots: [ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md) (decision 4) leaves out human play, so the game's `human_play` is false, the server and the browser's rules program refuse a human seat, and the setup menu offers only Bot vs bot. The adapter, [studio/](studio/), replays the Bronze referee and reads each position from its protocol text (`turn_input(0)` and `initial_input(0)`), and `last` is the difference between two positions, not a second reading of the rules; the frame format is documented in the crate. Records from `ice-and-fire-arena match --records DIR` replay in the studio's History (and `studio-archive` accepts them). The game has no release and no analysis program yet, so the studio plays its baseline bots ([ADR 0040](../../docs/adr/0040-baseline-bots-in-the-studio.md)): `bronze`, `wood1`, `wood2`, `wood3` and `random`, built from the current code (the setup menu starts bronze against wood1). The `wait` and `rules-check` bots are not offered.

The board is drawn with illustrated SVG artwork, in two independent themes chosen in the studio's Board settings (**Pieces** and **Board**) and remembered in the browser: the pieces (units, HQs, mines, towers) come in seven themes, `lava` (Lava and glacier, the default), `storybook`, `ink`, `japanese`, `bauhaus`, `scandinavian` and `deco`; the ground tiles come in seven, `scandinavian` (the default), `storybook`, `lava`, `watercolour`, `japanese`, `bauhaus` and `deco`. Red is drawn as fire and Blue as ice; a cell cut off from its HQ has its own paler tile, and an owned mine spot without a mine keeps a small mark. All the drawings are our own, in [web/src/games/ice-and-fire/themes/](../../studio/web/src/games/ice-and-fire/themes/) of the studio (64 x 64 SVG files, `pieces/<theme>/` and `boards/<theme>/`).

## Numbers

Measured on 2026-10-19 with `cargo run --release -p ice-and-fire-policy --example numbers -- 1000` (the example plays Bronze games in-process through the reference referee, fed with its text as CodinGame would send it; one game per map, on maps 0 to 999; `random` is the random bot's turn and `bronze` the Bronze bot's policy). The arena agrees with real processes: `ice-and-fire-arena match` on its own maps, 500 pairs of the bot against a copy of itself, gave red 47.6% of 1,000 games with the random bot, inside the interval below, and 56.6% with the Bronze bot, a little under the interval below but near the 58.1% of the 10,000 maps (the arena's maps are others, and 1,000 games give about 1.6 points of noise).

| Red | Blue | Red's score | 95% interval (Wilson) | Red wins | Draws | Blue wins | Mean game turns | Median | Reach turn 100 | HQ captured | Ended on power |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| random | random | 48.0% | 44.9% to 51.1% | 480 | 0 | 520 | 70.6 | 71 | 42.8% | 57.4% | 42.6% |
| bronze | bronze | 61.0% | 57.9% to 64.0% | 610 | 0 | 390 | 15.0 | 13 | 0.2% | 99.8% | 0.2% |
| bronze | random | 100.0% | 99.6% to 100.0% | 1000 | 0 | 0 | 12.3 | 12 | 0.0% | 100.0% | 0.0% |

- **Seat advantage.** It depends on the bots, and it is large. With random turns blue is ahead: red scores 48.0% (46.3%, 45.3% to 47.3%, over 10,000 maps) (blue starts with one more cell, and the cause is not studied here). With the Bronze bot red is ahead: 61.0% (58.1%, 57.2% to 59.1%, over 10,000 maps), and games are a race to the enemy HQ, which moving first helps (not studied here either). The seat-swapped pairs cancel it, and each pair plays one map from both seats; no opening needs to be imposed. The score of a bot is the score of its pairs.
- **Game length** is in game turns (both players have moved), at most 100. Random turns: mean 70.6, median 71, 42.8% of the games reach turn 100 (HQ captures on that turn included). The Bronze bot ends games with a chain of trainings to the enemy HQ after a mean of 15.0 game turns (median 13), against random turns after 12.3; fewer than 1 in 100 games reach turn 100. A game between the engine's random players is 141.0 answers long on average in the arena, 70.5 game turns.
- **How games end.** 57.4% by HQ capture and 42.6% by military power at turn 100 for random turns; 99.8% by capture for the Bronze bot. The share that reaches turn 100 is the first column's figure, so a search bot that survives early rushes will play many more turns than these baselines do.
- **Draws.** Equal military power at turn 100 is a draw in our referee, pending the `rules-check` run (RULES.md): 0 of 3,000 games above, 7 of 10,000 random games (0.07%), none of 10,000 Bronze games. Draws do not matter to the SPRT.
- **Distinct answers per turn.** Counted by trying each command on a copy of the position, in games between two players of the same kind, both seats pooled, at the start of the turn (`numbers` prints the table). "Cells units can move to" is the number of cells a unit can end a move on, summed over the units that can move; "move sets" is the product over those units of (1 + their end cells), a count of the sets of moves that ignores that units get in each other's way and the order of commands. A turn also holds trainings and buildings, chosen among the trainings (level, cell) and buildings listed, and their order.

| Players | Game turn | Units that can move | Cells units can move to | Trainings (level, cell) | Buildings | log10 of the move sets |
| --- | --- | --- | --- | --- | --- | --- |
| random | 1 | 0.0 | 0.0 | 6.0 | 0.5 | 0.0 |
| random | 5 | 1.5 | 4.5 | 3.3 | 0.3 | 0.9 |
| random | 10 | 3.8 | 11.2 | 19.4 | 4.9 | 2.2 |
| random | 20 | 11.7 | 29.0 | 74.4 | 19.7 | 6.1 |
| random | 40 | 18.6 | 29.6 | 55.4 | 17.4 | 6.8 |
| random | 60 | 22.2 | 24.1 | 37.9 | 12.6 | 5.9 |
| bronze | 10 | 4.9 | 10.5 | 85.9 | 14.4 | 2.3 |

Bronze games are too short to be sampled later. In the middle of a game between random players a turn offers about 10^6 to 10^7 move sets before trainings, buildings and orders, which fits the estimate in [MODEL.md](MODEL.md).

- **How the SPRT draws its seeds.** The arena derives the seed of pair *n* from the tournament seed (`--seed`, `SEED=1` in [evaluation.env](evaluation.env)); both games of the pair use it, so the pair plays one map from both seats, and the referee builds the map from that seed alone. The seed of each bot's own random choices is derived from the pair's seed, the seat and the bot's name. Two deterministic bots (the Bronze bot) therefore tie every pair against a copy of themselves, which makes the A/A control exact: Elo 0, verdict REJECTED after 30 pairs.

## Sources

| Short name | What it is |
| --- | --- |
| Statement | The statement template in the game's source, `config/statement_en.html.tpl`. The owner chose it on 2026-10-17 over pasting CodinGame's page; the page may have changed since the source was written, so the `rules-check` bot of stage 5 compares it with CodinGame's real games |
| Wood 3 page | CodinGame's statement of the game in Wood 3, pasted by the owner on 2026-10-19. It agrees with [RULES.md](RULES.md) on every rule it states (the map, active cells, income and upkeep, the wipe-out when upkeep cannot be paid, level 1 units and their fights, training and moves, the end after 100 turns, the input and output), says that all the rules are unlocked in Bronze, and lists the rules added by Wood 2, Wood 1 and Bronze only |
| Source | The referee, [Azkellas/a-code-of-ice-and-fire](https://github.com/Azkellas/a-code-of-ice-and-fire) on its `develop` branch, commit `efe9e78`, in Java on CodinGame's game engine. File paths below are relative to `src/main/java/com/codingame/` |

**License of the source: GNU GPL version 3** (`LICENSE.md`). Under [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md), the source is read for facts only: nothing of it is translated or copied here, the statement template and its pictures included. Our rules, referee and engine are written from our own `RULES.md`.

## Facts

Each fact gives its source. [RULES.md](RULES.md) gives the full rules with their edge cases.

| Fact | Value | Source |
| --- | --- | --- |
| Players | 2 | Statement; `config/config.ini` (`max_players=2`); `antiyoy/Constants.java` (`PLAYER_COUNT`) |
| Turn format | One player after the other: red (player 0) plays a whole turn, then blue (player 1); a game turn is both | `game/Referee.java`, `computeCurrentPlayer` (lines 165-176) |
| A turn's answer | One line of commands separated by `;`: `MOVE id x y`, `TRAIN level x y`, `BUILD MINE x y` or `BUILD TOWER x y`, `WAIT`, and `MSG text` for the viewer. The commands are applied one by one in order; an invalid one is skipped | Statement, "Output" and "Order of actions"; `game/Referee.java`, `readInput` (lines 352-383) |
| Answers that lose | An unrecognised command, or no answer in time | Statement, "Lose Conditions"; `game/Referee.java` lines 368-381 |
| Hidden information | None: each turn gives both players' gold and income, the whole map, and every building and unit | Statement, "Game turn input"; `antiyoy/GameState.java`, `sendState` (lines 705-718) |
| Randomness | Only the map, generated from the game's seed; the mine spots are given once, at the start | `antiyoy/GameState.java` line 41 (`new Random(seed)`); `game/Referee.java`, `sendInitialInput` |
| Map | 12×12 cells, each void, neutral, or owned (active or not). The headquarters are at (0,0) for red and (11,11) for blue; the cells next to each headquarters, diagonals included, are never void; blue also owns (11,10) at the start | Statement, "The map" and "Expert Rules"; `antiyoy/GameState.java`, `createHQs` (lines 367-383) |
| Map symmetry | The generator makes the map symmetric | `antiyoy/GameState.java` comments at lines 227 and 299; the kind of symmetry is to be stated in stage 2 |
| Mine spots | Between 6 and 15% of the non-void cells, an even number; always one right of the red headquarters and one left of the blue headquarters. The code counts the non-void cells before the headquarters are placed, as the statement says, and places mine spots only from Wood 1 on | Statement, "Expert Rules"; `antiyoy/GameState.java` lines 316-333 |
| End of the game | A headquarters captured, a player losing by its answer, or 100 game turns. After 100 turns the higher military power wins: gold plus the training cost of the units alive | Statement, "Victory Conditions"; `antiyoy/Constants.java` (`MAX_TURNS = 100`); `game/Referee.java`, `discriminateEndGame` and `checkForHqCapture`; `antiyoy/GameState.java`, `getScores` |
| Time limits | 1,000 ms for the first turn, 50 ms for the others | Statement, "Constraints" |
| Input | At the start: the number of mine spots and their coordinates. Each turn: own gold and income, the opponent's, 12 rows of 12 characters, the buildings (owner, type, x, y) and the units (owner, id, level, x, y) | Statement, "Game Input" |
| Units | Levels 1, 2 and 3: training cost 10, 20 and 30, upkeep 1, 4 and 20 per turn | Statement, "Armies"; `antiyoy/Constants.java` |
| Buildings | Mine (15 gold, on mine spots only, +4 income) and tower (15 gold, protects its four neighbouring cells) | Statement, "Buildings"; `antiyoy/Constants.java` |

### Leagues

The rules grow with the leagues; Bronze and above play them all (statement; `game/Referee.java`, `init`, lines 66-86).

| League | Rules added |
| --- | --- |
| Wood 3 | Level 1 units only, no buildings |
| Wood 2 | Units of levels 2 and 3 |
| Wood 1 | Mines |
| Bronze | Towers: all the rules |

The Wood 3 page confirms that the rules stop growing at Bronze. Leagues above Bronze, and how many players each holds, are not in the sources (open questions in [ROADMAP.md](ROADMAP.md)).

### Disagreements and points to settle

- The statement template declares five league levels; the referee knows four and treats every level from 4 up as Bronze (`game/Referee.java`, lines 66-86).
- Equal military power after 100 turns: the referee gives both players their score (`discriminateEndGame`); whether CodinGame then reports a draw is a platform fact not seen yet ([RULES.md](RULES.md#platform-facts-to-check)), not guessed here.
- Red always plays first, and blue's extra cell at (11,10) offsets that (statement, "Expert Rules"). CodinGame gives a bot either seat: the `rules-check` bot played both (2026-10-19).
- The source is the `develop` branch at commit `efe9e78` (2020-11-25); CodinGame's referee may differ. The `rules-check` bot found no difference in 4 Bronze games (2026-10-19), and CodinGame's behaviour wins.

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. The `rules-check` bot checks that, and logs the maps CodinGame draws so that they can be set beside the generator's statistics ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md), decision 4). To run it, once, in Bronze league or above (the bot assumes the Bronze rules):

1. Paste `target/cg/ice-and-fire-rules-check.rs` (`scripts/bundle-bots.sh ice-and-fire`; CI also publishes it in the `paste-ready-bots-ice-and-fire` artifact) into the IDE and click **Play my code** a few times, against different opponents.
2. Check that the MAP lines across the games show both `red` and `blue`, and say so if they all show the same seat: the rules differ between the seats (red plays first, blue starts with one more cell), and the first turn as each is checked.
3. For each game, copy from the console: the line `rules-check: MAP ...` (once, on the first turn), the last `rules-check: turn N, X checks, M differences` line (with `(first at turn K)` after a difference), every line starting with `rules-check: DIFFERENCE`, and the result on the end screen.

Each turn the bot answers a random turn and checks:

- the input alone: each player's active cells against their connection to its HQ, no unit of either player on its own inactive cell, both incomes against the income formula (the bot's own income after a wipe-out of its units is allowed to be the one from before it), mines only on mine spots and towers never, and, on the first turn, the start of the game;
- the input against what the engine predicted after the bot's own commands last turn: gold (plus the income shown, or 0 after a wipe-out), the cells, units and buildings of the bot, the ids of the opponent's units, and that each command the engine carried out shows up unless the opponent captured the cell.

It prints `rules-check: EXPECT win for <red|blue> (HQ captured)` when its own commands capture the opponent's HQ, and on turn 100 both military powers and the result it expects, to compare with the end screen (equal powers are printed as a draw, which RULES.md does not confirm). Any difference, a turn's input that does not parse included, is a `rules-check: DIFFERENCE` line with both sides, and the bot answers `WAIT` to input it cannot read. With `CG_RULES_CHECK_STRICT` set it exits at the first difference (CI only: the owner cannot set it on CodinGame).

The MAP line is `rules-check: MAP <red|blue> <144 characters, row by row> <mine spots as x,y separated by spaces>`, with `#` for a void cell and `.` for any other.

What the bot cannot check from its input: the opponent's commands, gold and upkeep (only the opponent's income is shown), the order of the opponent's actions, the time limits, and the rules of the leagues below Bronze. Ids of the bot's own new units are the bot's guess, since it cannot see the ids of units that died, so only that they come after the ids it has seen is checked. How CodinGame chooses the seats and the map seed is not in the input.

## Engine speed

`cargo +1.90.0 run --release -p ice-and-fire-engine --example speed -- [SECONDS]` plays Bronze games on 64 of the referee's maps with random turns (`State::random_turn`, then `play_turn`), then times `turn_input` and `read` on positions from move 40 of those games. CodinGame compiles with Rust 1.90.0, so measure with it (`rustup toolchain install 1.90.0 --profile minimal`). CI runs it from [`ci.sh`](ci.sh) and shows the table in the job summary; it is a report, not a check.

Measured on 2026-10-19 with Rust 1.90.0, one thread of a shared 4-core machine (Intel(R) Xeon(R) Processor @ 2.80GHz; the figures of 2026-10-17 came from a 2.10GHz one; it varies by about 10% between runs), 2 s per row:

| Engine benchmark | Result |
| --- | --- |
| Random turns generated and played | 137,142 per second |
| Random turns generated alone (from move 40 of a game) | 274,449 per second |
| Whole random games (Bronze) | 988 per second |
| Turns per random game | 138.8 |
| `turn_input` | 202,351 per second |
| `read` (a new map every time) | 8,990 per second |
| `read` (the map of the last read) | 283,875 per second |
| `State::new` (builds the distance table) | 9,999 per second |

A turn here is one player's turn, so a game of 138.8 turns is about 69 game turns. A random turn clones the state (about 1 KB and a reference count) to choose commands on a scratch copy. `read` rebuilds the 144 × 144 distance table, about 111 microseconds, only when the map differs from the last one it read on the thread; a bot reads the same map every turn.

## Notes on the owner's criteria

The owner named these criteria on 2026-10-13. They inform; they do not grade the game he chose.

- **Interest and research value.** Unlike Ultimate Tic-Tac-Toe, a turn is a sequence of commands (moves, training, buildings) with an economy behind it, so a turn has far more possible answers than one move, and a search over single moves no longer fits as is. Territory connectivity (cells cut from the headquarters go inactive and their units die) gives tactical shots that a search can find. Stage 3 (model) gives the numbers; stage 8 (research) reads the contest's write-ups.
- **Source and license.** The referee is public but under the GPL: facts only, so the referee and engine are our own. Bots never need the map generator, since CodinGame sends the map. Our arena needs maps too: its generator is ours, written from the steps in [MAP_GENERATOR.md](MAP_GENERATOR.md) by someone who has not read the source, and it reproduces the source's maps exactly, checked against maps the source printed ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)).
- **Framework gaps already visible** (for stage 4): several commands per turn; an end of game decided on a score after a turn limit, not only a win; an asymmetric start (red first, blue's extra cell); a random map each game.
- **How it shows in the studio.** A 12×12 grid with territory, units and buildings draws well. Human play needs a way to build a turn of several commands before sending it.
- **Depth of the leaderboard.** Not in the sources; the owner can tell, or it stays unknown.
