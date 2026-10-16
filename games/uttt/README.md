# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)). The rules, protocol and time limits are in [RULES.md](RULES.md).

**Status:** Legend league on CodinGame since 2026-10-07; 59th with `uttt-v008` on 2026-10-08, the last rank recorded. **Current release:** [`uttt-v017`](releases/uttt-v017.rs) (E026: E018's "jacekmax", where the NNUE judges all the children of a node at once with incremental updates, selection weighing each child's minimax value and its average half and half since E020, exploration 0.2 since E021, accumulators updated move by move as the search walks down its tree since E022, a network fitted on `uttt-v015`'s own games and their side games since E024, and children evaluated from their parent's accumulators since E026). Strength work resumed on 2026-10-14 with the NNUE plan of [ROADMAP.md](ROADMAP.md).

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [MODEL.md](MODEL.md) | The game as a search problem: numbers, a formal model, what follows for bots |
| [ROADMAP.md](ROADMAP.md) | The game's stages in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md)) |
| [research/](research/README.md) | Sources and approaches, set beside what the journal learned |
| [RETROSPECTIVE.md](RETROSPECTIVE.md) | What worked, what cost more, and changes proposed for the next game |
| [referee/](referee/) | Readable reference implementation of the rules, driven by the arena |
| [engine/](engine/) | Fast implementation of the rules for bots: bitboards, no allocation, random and decisive playouts, and the NNUE (`nnue`); the class and pattern policies are in [policy/](policy/) |
| [analysis/](analysis/) | `uttt-analysis`: the analysis program (see "Analysis" below): a JSON request in, JSON lines of reports out, natively or as WebAssembly for the studio ([ADR 0032](../../docs/adr/0032-analysis-engine.md)) |
| [arena/](arena/) | `uttt-arena`: plays bots against each other through the referee |
| [trainer/](trainer/) | `uttt-trainer`: self-play data, and training of the playout policy, the value network and the move models ([ADR 0016](../../docs/adr/0016-self-play-training.md), [ADR 0017](../../docs/adr/0017-value-network.md)) |
| [policy/](policy/) | `uttt-policy`: E011's class policy (`PlayoutPolicy`, 32 move classes) and E015's pattern policy (`PatternPolicy`), their methods on `Board` (the trait `PolicyMoves`) and the searches that use them (`PolicyBoard`, `PatternBoard`), for the trainer; no bot uses them any more, which keeps their code out of the bundles |
| [value/](value/) | `uttt-value`: E012's value network (its inputs and evaluation) and the searches that use it (`ValueBoard`, `MixBoard`), for the trainer; bots never use it, which keeps its code out of their bundles ([ADR 0017](../../docs/adr/0017-value-network.md)) |
| [studio/](studio/) | `uttt-studio`: the game's adapter for the studio, with no rules of its own: frames come from replaying the referee, a human's legal moves from its turn input ([ADR 0025](../../docs/adr/0025-studio.md)) |
| [bots/](bots/) | One crate per bot |
| [evaluation.env](evaluation.env) | Settings of the SPRT, the league and the arena for evaluations ([ADR 0012](../../docs/adr/0012-evaluation.md)) |
| [releases/](releases/) | Frozen paste-ready file of each version |
| [journal/](journal/) | One entry per experiment, with an index |
| [training/](training/) | Reports of the training runs used, one folder per run ([ADR 0021](../../docs/adr/0021-prune-finished-branches.md)) |

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed. CodinGame shuffles that list, so this plays randomly there. | Proved the paste-to-CodinGame path (Phase 0, 2026-10-07) |
| [random](bots/random/) | Plays a uniformly random valid action, seeded | Baseline for every rating; first bot built from shared crates |
| [greedy](bots/greedy/) | Looks one move ahead: wins the game or a small board when it can, avoids handing the opponent either, or a free choice; random among equals | Fixed baseline about 500 Elo above random: the first release's opponent, and the arena's controls in CI |
| [mcts](bots/mcts/) | Monte Carlo tree search (UCT, no playouts: since E018 the first visit to a node creates all its children and gives each the estimate of an NNUE, updated incrementally from the node's accumulators, a game-winning move scoring a win; selection uses each node's minimax value and its average, half and half since E020) for 90 ms of 100, keeping its tree from turn to turn and proving wins and losses; plays a forced or game-winning action at once | The real bot: released as `uttt-v001`, then improved by experiments (see the journal) |
| [wood](bots/wood/) | Perfect 3×3 tic-tac-toe; among moves that never lose, the one that wins most against random play | Promotion out of Wood league, whose boss plays randomly. Expected score against random play: 99.7% moving first, 95.8% moving second. On the 9×9 board it plays randomly. |
| [rules-check](bots/rules-check/) | Plays randomly; every turn, compares CodinGame's valid actions with the engine's | Checking the engine against real CodinGame games, from Bronze league |

## Playing matches locally

```sh
cargo build --release -p uttt-arena -p uttt-bot-random -p uttt-bot-greedy
target/release/uttt-arena match \
  --bot greedy=target/release/uttt-bot-greedy \
  --bot random=target/release/uttt-bot-random \
  --pairs 100 --opening-plies 4 --out target/uttt-results.jsonl
```

Each pair plays the same seed twice with seats swapped; `--opening-plies 4` starts the pair with 4 random moves imposed by the referee. Two more commands use the same options: `sprt` plays a candidate against a baseline until the test decides, and `league` rates several bots. See `uttt-arena <command> --help`. `--records DIR` also writes each game, with every answer, as a JSON file in `DIR` for the studio to replay; `--records-sample 10` keeps about 10 games per pair of bots (each matchup of a league) plus every game lost by a fault. A record is never overwritten: use a new or empty directory.

## Evaluating a bot

With the settings of [evaluation.env](evaluation.env):

- `scripts/sprt.sh uttt CANDIDATE.rs [BASELINE.rs]`: smoke test against random, then SPRT against the baseline (by default the newest release older than the candidate, or `greedy`). Since the bot is in Legend, both play at CodinGame's exact limits ([ADR 0020](../../docs/adr/0020-full-time-sprt-in-legend.md)). Settings can be overridden from the environment, for example `SPRT_TIME_SCALE=0.2 SPRT_TIME_TOLERANCE_MS=5` for a quicker test at 20 ms, or a few milliseconds of `SPRT_TIME_TOLERANCE_MS` on a small or busy machine, where the exact limits cause timeouts that GitHub's runners do not.
- `scripts/league.sh uttt`: ratings of every release with `random` and `greedy`.
- `scripts/new-release.sh uttt BOT`: freezes a bot as the next release. The whole experiment flow is in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Training

Learned knowledge comes from self-play ([ADR 0016](../../docs/adr/0016-self-play-training.md)). The playout policy (`uttt_policy::PlayoutPolicy`) draws each move with a weight that depends on its class: which of a few features it has (wins its small board, blocks the opponent's line, gives the opponent a free choice, sends it to a board it can win, takes the centre cell).

From E015 to E017, the bot's policy was a pattern policy (`uttt_policy::PatternPolicy`, `uttt_policy::PatternBoard`): one weight for each pattern of the small board a move is played in (its 9 cells seen by the player to move), the move's cell, and one of 5 kinds of destination (a free choice, or a board where either side can or cannot win a small board at once). Symmetric patterns share a weight, which leaves 26,275 weights. A weight is a 6-bit digit, a log-weight in quarter steps from -8 to 7.75. The trainer writes the weights as text, one base64 character per weight, which `PatternPolicy::decode` reads (the releases up to `uttt-v010` hold them so); `uttt-v011` and `uttt-v012` hold them packed, the 19,707 bytes of the digits one after the other (`PatternPolicy::decode_packed`) in 9,984 characters by `cg-pack`, which is 16,291 fewer than the text. The packed weights are the same as `uttt-v010`'s, bit for bit (a test of the trainer reads them from the release). Since E018 the bot asks for no priors, so the weights left it: they are in [`training/pattern_weights.rs`](training/pattern_weights.rs), for the trainer and the Train workflow.

The value network (`uttt_value`, [ADR 0017](../../docs/adr/0017-value-network.md)) gives the side to move's expected score from 217 inputs that describe the position from its view, through 64 and 16 hidden units. `ValueBoard` searches with it: a new leaf gets its estimate instead of a playout. Its weights come from the trainer as base64 text, 8 bits each with one scale per group, which `ValueNetwork::decode` reads.

- `uttt-trainer selfplay --games N --iterations K --out FILE`: plays self-play games with the bot's search and records, for every searched position, the root's average score and how many visits each move got. `--policy games/uttt/training/class_weights.rs` searches with the class policy's playouts instead of decisive ones, `--patterns games/uttt/training/pattern_weights.rs` as `uttt-v010` did (pattern playouts, from E015 to E016, and children ordered by the same policy, from E015 to E017), and `--no-visits` leaves the visits out. `--network games/uttt/bots/mcts/src/nnue_weights.rs --exploration 0.2 --expand-all --minimax-weight 0.5` searches as the newest release does, its network judging the new nodes (the Train workflow's `nnue` stage plays so, and a test checks these settings against the newest file in `releases/`). `--opening-plies N` moves open each game, uniformly random and unrecorded (`--opening random`, the default) or searched, recorded and each drawn in proportion to the root's visits (`--opening sampled`), raised to `1 / T` with `--opening-temperature T` (flatter above 1); `--balance 0.9` drops a game whose first search after the opening expects above 0.9 or below 0.1 for the side to move. `--side-games N` plays N side games from each game kept: each leaves it at one of its searched positions after the opening, drawn uniformly, by a legal move other than the one played, drawn uniformly, then plays on as the game does, with new search trees, and records only the positions after that move, each with its own game's result (the Train workflow's `side_games` input); `fit-nnue` holds a game out with its side games. `--name "core 2"` starts each progress line, such as `core 2: 10,800 of 25,000 games (43%), 528,079 positions, 5,649 s`, so that the processes sharing a log can be told apart.
- `uttt-trainer fit-policy --data FILE... --weights-out weights.rs --report-out report.md`: fits the class weights to those visits, holding a quarter of the positions out to check the fit, and writes them as Rust source with a report.
- `uttt-trainer fit-patterns --data FILE... --models all --temperature T --weights-dir DIR --report-out report.md`: fits move models to the same visits, holding every 20th game out, and compares them in the report: the 32 classes, E015's patterns, and larger models for E016 (more kinds of destination, two game phases, the pattern of the board the opponent is sent to, the roles of both boards in the big board). Each model's weights are written divided by `T` (sharper below 1) to `DIR/<model>.rs` as a Rust module of packed characters (`PATTERN_PACKED`); `patterns.rs` is `training/pattern_weights.rs`. The report gives each model's size as base64 text, as packed characters, and about what an entropy coder would make of the text.
- `uttt-trainer fit-value --data FILE... --policy games/uttt/training/class_weights.rs --weights-out value_weights.rs --report-out report.md`: trains the value network (`--size 64-16` or `128-32`) on the games' results (or the root scores, `--target`), each position in a random symmetry, holding every 20th game out. The report plays pairs of games between a search with the network and one with the bot's playouts, at the bot's time budgets (900 ms on the first move, then 90 ms), for the network alone and half network, half playout: the gate of [ADR 0019](../../docs/adr/0019-value-network-gate-at-time.md). It also compares the network's predictions with averages of playouts on held-out positions.
- `uttt-trainer fit-nnue --data FILE... --width 256 --weights-out nnue_weights.rs --report-out report.md`: trains the NNUE of `uttt_engine::nnue` (`--width 128`, `256` or `512`) on the games' results (or the root scores, `--target`), each position in a random symmetry, holding every 20th game out as `fit-value` does. A game whose moves are those of a game read before, from any file, is dropped first and counted in the report: with the network, self-play is deterministic after its sampled opening, so games that open alike are copies. It writes the network quantized and packed as a Rust module (`NNUE_WEIGHTS`, for `cg_pack::unpack`) and a report whose first table compares, on the held-out positions, the average result, the float network and the quantized one (measured by the engine's own arithmetic), and the network of `--compare-network FILE` (width 256, such as the bot's, which the Train workflow passes). On an even sample of the held-out positions it then gives, for the quantized networks, the error and the share of moves played that the network would choose (the child it scores best) by ply, the calibration (average prediction against average result by tenth of the prediction), and the first layer's units that never rise above 0 (dead) or never fall below full scale. The report ends with the fitted positions by ply: how far the root scores are from even, the shares beyond 0.3 or 0.7 and beyond 0.1 or 0.9, the first player's results and how many distinct positions there are at plies 4, 8 and 12. Its threads live as long as the training: each sums the gradient of its part of a batch, then the batch's gradient and Adam's step over its share of the parameters, so the result depends on the number of threads (`--threads`, one per core by default) and not on their timing. The Train workflow builds the trainer for the runner's own processor (`-C target-cpu=native`).
- The [Train workflow](../../.github/workflows/train.yml), started by hand from the Actions tab, runs these on GitHub: self-play in up to 20 parallel jobs, then the fit of the chosen stage (`policy`, `value`, `patterns` or `nnue`), whose results it commits to a new branch `claude/train/<run id>`. Weights reach a bot only through an experiment and its SPRT. Reports of the runs used are kept in [training/runs/](training/runs/), after which the Prune branches workflow deletes the run's branch ([ADR 0021](../../docs/adr/0021-prune-finished-branches.md)).

On one core, a game at 10,000 iterations per move takes about a quarter of a second with decisive playouts (0.37 s with the bot's playouts, measured on a slower processor) and gives about 42 positions.

## Analysis

`uttt-analysis` ([analysis/](analysis/)) is the game's analysis program ([ADR 0032](../../docs/adr/0032-analysis-engine.md)): it searches a position with the current engine and the settings of the latest release (exploration 0.2, all the children of a node judged at once by the NNUE, minimax values and averages in selection; the bot's own network weights file is reused, and a test fails when the settings or the network differ from the newest file in `releases/`), counts draws, and streams, as JSON lines, the best moves with their visits, win, draw and loss chances and expected lines. Since `uttt-v011` a leaf gets the network's expected score, which is never exactly a draw, so the draw chances count only the games the search plays to their end: in the middle game they are 0 and the win and loss chances share the draws. The protocol is in [studio/README.md](../../studio/README.md), "The analysis program". Try it:

```sh
echo '{"moves":["4 4"]}' | cargo run --release -p uttt-analysis
```

It counts iterations, not time, so the same request gives the same lines. Speed, measured on 2026-10-13 with `uttt-v010`'s settings (playouts; the network's search answers the default request below in about 1.6 s instead of 2.3) from the start position on one thread of the shared machine (it varies by about 10% between runs): about 360,000 iterations per second natively from the start (500,000 iterations in 1.4 s). For the default request (1,000,000 iterations, a report every 20,000) on a mid-game position (`["4 4","4 3","3 1","1 5"]`), the whole answer takes about 2.3 s natively (2,068,686 nodes, 76 MB resident). The tree stops at its limit of 8,000,000 nodes, which that position reaches after 3,886,000 iterations in about 9.7 s natively, at a peak of 284 MB resident (a node then took 36 bytes, 40 since E018; the limit stays under 2^23 nodes so that the nodes' vector never doubles to 2^24, which would now take 671 MB and a copy peak of about 1 GB). With `uttt-v012`'s settings (E018, measured on 2026-10-14 on Claude's session), every iteration creates all the children of a node, so the tree grows far faster: the default request on that mid-game position takes about 3.5 s and ends with 7,877,979 nodes, just under the limit, at a peak of 309 MB resident; from the start, 500,000 iterations take 2.0 s and 4,185,151 nodes (165 MB). In the browser, about 200,000 to 260,000 per second as WebAssembly under Node (V8) through the studio's own WASI layer, so roughly 1.4 to 1.8 times slower. `scripts/build-studio-wasm.sh` builds it as `analysis.wasm` for the browser engine. The test that compares the settings with the newest release checks the constants it knows (exploration, expanding all children, the minimax weight, no priors, the network) only: it does not catch a setting that a later release adds, so whoever makes a release must check that the analysis program still searches as the bot does.

## Engine speed

`cargo run --release -p uttt-engine --example speed` runs random playouts from the start position for a few seconds, then decisive playouts (a move that wins the game when there is one, as the search plays them), then 100 ms MCTS searches from there, then the cost of an NNUE of width 256 (updates, evaluations, accumulators from scratch), then where the time of the MCTS bot's search goes. Moves per playout are averaged over 10,000 games played one move at a time. CI runs it on every pull request and push to `main` and shows the table in the job summary of the "uttt: bots" job (from `ci.sh`); it is a report, not a check.

Baseline, measured on 2026-10-08 on one thread of an Intel Xeon at 2.8 GHz (this shared machine varies by about 10% between runs), after E006 sped up selection and playouts:

| Engine benchmark | Result |
| --- | --- |
| Random playouts from the start | about 1,190,000 per second (1,180,605 to 1,196,417 over three runs) |
| Moves per playout | 58.9 |
| Moves per second | about 70,000,000 |
| Decisive playouts from the start | about 1,234,000 per second (1,214,458 to 1,247,103), 54.2 moves each |
| MCTS iterations from the start, 100 ms searches | about 755,000 per second (719,982 to 803,399) |
| NNUE 256: incremental updates (one move) | about 12,100,000 per second (11,441,466 to 13,102,714) |
| NNUE 256: evaluations | about 5,400,000 per second (4,860,759 to 5,686,432) |
| NNUE 256: accumulators from scratch | about 907,000 per second (867,389 to 943,153) |
| NNUE search: the MCTS bot's network and settings, 20 searches of up to 20,000 iterations | about 471,000 iterations per second (460,930 to 492,240) |
| NNUE search: per iteration | 9.4 moves walking down, 0.85 nodes expanded, 6.4 children, 6.2 judged by the network |
| NNUE search: share of the time, updating the accumulators with each move walking down, and the rules | 28.6% to 30.1% |
| NNUE search: share of the time, playing each child's move: the rules | 0.5% to 5.8% |
| NNUE search: share of the time, evaluating each child: its move's rows and the output layer | 41.7% to 46.5% |
| NNUE search: share of the time, the rest: tree, moves, rules | 19.0% to 27.7% |

The three NNUE rows (`uttt_engine::nnue`, a random network of width 256, built in the example) were added and measured on 2026-10-14 on Claude's session, three runs, machine noise about 10%; the other rows are from the earlier measurement above. The seven "NNUE search" rows were added on 2026-10-15 (step D3 of the [roadmap](ROADMAP.md#d-scale)), three runs on Claude's session. They search positions of three games of the bot's search against itself (every eighth position from the fourth ply), with the bot's own network, exploration and minimax weight, 20,000 iterations each, about one turn's worth (6 of the 19 stop earlier, once they prove the position; the rates count the iterations run). A part's share is measured by doing it 4 more times for nothing, in a copy of `NnueBoard::child_scores` that must find exactly what `NnueBoard` finds: the search builds the same tree, so the time it takes more, divided by 4, is that part's. Repeats find their data in the cache, so these shares lean low, and the rest takes what they miss. The rows are those of E022's search (`uttt-v015`), measured on 2026-10-15 after it, three runs. Before E022, the search built a node's accumulators from scratch when it expanded it: about 305,000 iterations per second (296,829 to 311,021), with these shares: accumulators from scratch 20.9% to 24.8%, copies 2.8% to 5.1%, each child's move 11.4% to 15.5%, evaluations 30.0% to 37.3%, the rest 20.6% to 34.9%. E022 removed the accumulators from scratch and the same searches took 22% to 27% less time, after paying for the walk's updates: the old probe underestimated the share of the accumulators from scratch. The rows are now those of E026's search (`uttt-v017`), measured on 2026-10-16, three runs alternating with the code before it: E026 evaluates a node's children from its accumulators, adding each child's move rows as the output layer reads them, so a child's accumulators are neither copied nor stored. With `uttt-v016`'s network (whose games give 20 positions), the code before E026 ran 382,354 to 407,081 iterations per second, with these shares: the walk 23.4% to 34.1%, copies 1.3% to 3.2%, each child's move (its rows, and the rules) 15.9% to 18.1%, evaluations 33.5% to 36.9%, the rest 9.6% to 22.3%. The same searches now run about 18% more iterations per second.

Before E006 (after E005): about 925,000 random and 976,000 decisive playouts, and 620,000 MCTS iterations per second. After E004: about 985,000 random playouts and 578,000 MCTS iterations per second. Before E004: 441,681 playouts and about 350,000 MCTS iterations per second.

`cargo run --release -p uttt-value --example value_speed -- [SECONDS]` measures what a value network would cost the search, with untrained networks of several sizes against `uttt-v008`'s playouts: estimates per second, and iterations of 90 ms searches at several stages of the game. Its last row is `uttt-value`'s own network. Results and their use are in [ADR 0017](../../docs/adr/0017-value-network.md).

## Determinism

With a fixed seed and a fixed number of iterations (`CG_SEED`, `CG_FIXED_ITERS`), every release replays identically ([ADR 0025](../../docs/adr/0025-studio.md)). `scripts/check-determinism.sh uttt [ITERS] [RELEASE.rs...]` proves it: it compiles each release as CodinGame does, plays it against itself twice (2 pairs of games, seed 7, 500 iterations by default) with the arena's `--records`, and fails if the two runs differ in the lines answered, the winner or the way a game ended. It also prints the release's iteration rate: the iterations divided by the median time of its answers, leaving out each seat's first. CI runs it on the release a pull request adds.

Measured on 2026-10-10 on the development machine (all ten releases identical; 500 iterations per answer, 4 games per run). Rates depend on the machine, and the studio measures them on the owner's computer. They also depend on the iteration count, because each answer has a fixed cost that weighs more at small counts (`uttt-v010` measured 574 per millisecond at 200 iterations, 789 at 500, 918 at 2,000 and 830 at 10,000): compare rates only at the same count, and take them as rough below a few thousand iterations:

| Release | Iterations per millisecond | Median answer |
| --- | --- | --- |
| `uttt-v001` | 696 | 0.72 ms |
| `uttt-v002` | 631 | 0.79 ms |
| `uttt-v003` | 662 | 0.75 ms |
| `uttt-v004` | 1,198 | 0.42 ms |
| `uttt-v005` | 1,233 | 0.41 ms |
| `uttt-v006` | 1,557 | 0.32 ms |
| `uttt-v007` | 1,409 | 0.35 ms |
| `uttt-v008` | 890 | 0.56 ms |
| `uttt-v009` | 960 | 0.52 ms |
| `uttt-v010` | 789 | 0.63 ms |

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. To check that, once in Bronze league or above:

1. Paste the bundled `rules-check` bot into the IDE and click **Play my code** a few times.
2. In the console, every turn prints a line ending with `N turns checked, M with differences`. Any difference is printed on a line starting with `rules-check: DIFFERENCE`, with the actions only one side listed.
3. When the bot's own move ends the game, it prints the result the engine expects, to compare with CodinGame's.

## Putting a bot on CodinGame

Each release has a GitHub release with its paste-ready file attached; the same file is in `releases/`. For any other bot:

1. Bundle it into one file: `scripts/bundle-bots.sh` writes `target/cg/uttt-<bot>.rs` for every bot. CI also publishes these files as the `paste-ready-bots-uttt` artifact of each run. The steps of CI that only this game needs (the arena games, the evaluation controls, the engine speed) are in [`ci.sh`](ci.sh) ([ADR 0029](../../docs/adr/0029-ci-scoped-by-area-and-game.md)).
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
