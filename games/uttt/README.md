# Ultimate Tic-Tac-Toe

CodinGame's [Ultimate Tic-Tac-Toe](https://www.codingame.com/multiplayer/bot-programming/tic-tac-toe) is the first game of this framework ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)). The rules, protocol and time limits are in [RULES.md](RULES.md).

**Status:** Phase 4. The reference referee, the fast engine (checked against it), the arena with its evaluation tools, five utility bots and the first real bot, MCTS, exist. No release yet.

## Layout

| Path | What it is |
| --- | --- |
| [RULES.md](RULES.md) | The rules in our own words, with sources |
| [referee/](referee/) | Readable reference implementation of the rules, driven by the arena |
| [engine/](engine/) | Fast implementation of the rules for bots: bitboards, no allocation, random, decisive and policy playouts, and the value network's inputs and evaluation |
| [arena/](arena/) | `uttt-arena`: plays bots against each other through the referee |
| [trainer/](trainer/) | `uttt-trainer`: self-play data and training of the playout policy ([ADR 0016](../../docs/adr/0016-self-play-training.md)) |
| [bots/](bots/) | One crate per bot |
| [evaluation.env](evaluation.env) | Settings of the SPRT, the league and the arena for evaluations ([ADR 0012](../../docs/adr/0012-evaluation.md)) |
| `releases/` | Frozen paste-ready file of each version, from the first release on |
| [journal/](journal/) | One entry per experiment |

## Bots

| Bot | Strategy | Purpose |
| --- | --- | --- |
| [first-valid](bots/first-valid/) | Plays the first valid action listed. CodinGame shuffles that list, so this plays randomly there. | Proved the paste-to-CodinGame path (Phase 0, 2026-10-07) |
| [random](bots/random/) | Plays a uniformly random valid action, seeded | Baseline for every rating; first bot built from shared crates |
| [greedy](bots/greedy/) | Looks one move ahead: wins the game or a small board when it can, avoids handing the opponent either, or a free choice; random among equals | Fixed baseline about 500 Elo above random: the first release's opponent, and the arena's controls in CI |
| [mcts](bots/mcts/) | Monte Carlo tree search (UCT; playouts take a game-winning move when there is one, draw their first 16 moves from a learned pattern policy, then random moves; the same policy orders each node's children) for 90 ms of 100, keeping its tree from turn to turn and proving wins and losses; plays a forced or game-winning action at once | The real bot: released as `uttt-v001`, then improved by experiments (see the journal) |
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

Each pair plays the same seed twice with seats swapped; `--opening-plies 4` starts the pair with 4 random moves imposed by the referee. Two more commands use the same options: `sprt` plays a candidate against a baseline until the test decides, and `league` rates several bots. See `uttt-arena <command> --help`.

## Evaluating a bot

With the settings of [evaluation.env](evaluation.env):

- `scripts/sprt.sh uttt CANDIDATE.rs [BASELINE.rs]`: smoke test against random, then SPRT against the baseline (by default the newest release older than the candidate, or `greedy`). Settings can be overridden from the environment, for example `TIME_SCALE=1 TIME_TOLERANCE_MS=0` for CodinGame's full limits.
- `scripts/league.sh uttt`: ratings of every release with `random` and `greedy`.
- `scripts/new-release.sh uttt BOT`: freezes a bot as the next release. The whole experiment flow is in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Training

Learned knowledge comes from self-play ([ADR 0016](../../docs/adr/0016-self-play-training.md)). The engine's playout policy draws each move with a weight that depends on its class: which of a few features it has (wins its small board, blocks the opponent's line, gives the opponent a free choice, sends it to a board it can win, takes the centre cell).

Since E015, the bot's policy is a pattern policy (`PatternPolicy`, `search::PatternBoard`): one weight for each pattern of the small board a move is played in (its 9 cells seen by the player to move), the move's cell, and one of 5 kinds of destination (a free choice, or a board where either side can or cannot win a small board at once). Symmetric patterns share a weight, which leaves 26,275 weights. The bot holds them as text, one base64 character per weight for a log-weight in quarter steps from -8 to 7.75, which `PatternPolicy::decode` reads.

The engine's value network (`uttt_engine::value`, [ADR 0017](../../docs/adr/0017-value-network.md)) gives the side to move's expected score from 217 inputs that describe the position from its view, through 64 and 16 hidden units. `search::ValueBoard` searches with it: a new leaf gets its estimate instead of a playout. Its weights come from the trainer as base64 text, 8 bits each with one scale per group, which `ValueNetwork::decode` reads.

- `uttt-trainer selfplay --games N --iterations K --out FILE`: plays self-play games with the bot's search and records, for every searched position, the root's average score and how many visits each move got. `--policy games/uttt/bots/mcts/src/weights.rs` searches with the bot's playouts instead of decisive ones, and `--no-visits` leaves the visits out.
- `uttt-trainer fit-policy --data FILE... --weights-out weights.rs --report-out report.md`: fits the class weights to those visits, holding a quarter of the positions out to check the fit, and writes them as Rust source with a report.
- `uttt-trainer fit-patterns --data FILE... --temperature T --weights-out pattern_weights.rs --report-out report.md`: fits the pattern policy's log-weights to the same visits, holding every 20th game out, and writes them divided by `T` (sharper below 1) as the bot's text, with a report comparing them with the class weights fitted on the same data.
- `uttt-trainer fit-value --data FILE... --policy games/uttt/bots/mcts/src/weights.rs --weights-out value_weights.rs --report-out report.md`: trains the value network on the games' results (or the root scores, `--target`), each position in a random symmetry, holding every 20th game out. The report plays pairs of games between a search with the network and one with the bot's playouts at equal iterations, the gate of [ADR 0018](../../docs/adr/0018-value-network-gate.md), and compares the network's predictions with averages of playouts on held-out positions.
- The [Train workflow](../../.github/workflows/train.yml), started by hand from the Actions tab, runs these on GitHub: self-play in up to 20 parallel jobs, then the fit of the chosen stage, whose results it commits to a new branch `claude/train/<run id>`. Weights reach a bot only through an experiment and its SPRT.

On one core, a game at 10,000 iterations per move takes about a quarter of a second with decisive playouts (0.37 s with the bot's playouts, measured on a slower processor) and gives about 42 positions.

## Engine speed

`cargo run --release -p uttt-engine --example speed` runs random playouts from the start position for a few seconds, then decisive playouts (a move that wins the game when there is one, as the search plays them), then 100 ms MCTS searches from there. Moves per playout are averaged over 10,000 games played one move at a time. CI runs it on every push and shows the table in the job summary of the "CodinGame compatibility" job.

Baseline, measured on 2026-10-08 on one thread of an Intel Xeon at 2.8 GHz (this shared machine varies by about 10% between runs), after E006 sped up selection and playouts:

| Engine benchmark | Result |
| --- | --- |
| Random playouts from the start | about 1,190,000 per second (1,180,605 to 1,196,417 over three runs) |
| Moves per playout | 58.9 |
| Moves per second | about 70,000,000 |
| Decisive playouts from the start | about 1,234,000 per second (1,214,458 to 1,247,103), 54.2 moves each |
| MCTS iterations from the start, 100 ms searches | about 755,000 per second (719,982 to 803,399) |

Before E006 (after E005): about 925,000 random and 976,000 decisive playouts, and 620,000 MCTS iterations per second. After E004: about 985,000 random playouts and 578,000 MCTS iterations per second. Before E004: 441,681 playouts and about 350,000 MCTS iterations per second.

`cargo run --release -p uttt-engine --example value_speed -- [SECONDS]` measures what a value network would cost the search, with untrained networks of several sizes against `uttt-v008`'s playouts: estimates per second, and iterations of 90 ms searches at several stages of the game. Its last row is the engine's own network. Results and their use are in [ADR 0017](../../docs/adr/0017-value-network.md).

## Checking the rules on CodinGame

The parity tests prove the engine and the referee agree with each other, not that both agree with CodinGame. To check that, once in Bronze league or above:

1. Paste the bundled `rules-check` bot into the IDE and click **Play my code** a few times.
2. In the console, every turn prints a line ending with `N turns checked, M with differences`. Any difference is printed on a line starting with `rules-check: DIFFERENCE`, with the actions only one side listed.
3. When the bot's own move ends the game, it prints the result the engine expects, to compare with CodinGame's.

## Putting a bot on CodinGame

Each release has a GitHub release with its paste-ready file attached; the same file is in `releases/`. For any other bot:

1. Bundle it into one file: `scripts/bundle-bots.sh` writes `target/cg/uttt-<bot>.rs` for every bot. CI also publishes these files as the `paste-ready-bots` artifact of each run.
2. In the CodinGame IDE for this game, select **Rust**, replace the editor content with the file, then click **Play my code** or **Submit**.
