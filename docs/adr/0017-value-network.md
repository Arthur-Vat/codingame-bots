# 0017. Stage 2: a value network instead of playouts

- Status: accepted (owner, 2026-10-08); decision 5 (the gate) superseded by [0018](0018-value-network-gate.md)
- Date: 2026-10-08
- Scope: uttt

## Context

[ADR 0016](0016-self-play-training.md) plans three stages of learned knowledge and asks that stage 2, a value network, start with a proposal to the owner: what network, how fast it evaluates, and how many positions per turn it leaves. On 2026-10-08 the owner chose stage 2 over a richer playout policy.

Stage 1 (E011, `uttt-v008`) gave the largest gain so far, +95 Elo at CodinGame's limits, and reached 59th in Legend. Its fit also shows its limit. With 23 free parameters fitted to 65,000 positions, the cross-entropy is 1.773 nats on the fitted positions and 1.775 on held-out ones: the policy is limited by what its five features can express, not by data. More features in a playout cost time at every move of every playout. A value network is computed once per new leaf instead of a whole playout, so its parameters cost far less per use.

Room: since bundles drop comments ([pull request #25](https://github.com/Arthur-Vat/codingame-bots/pull/25)), the MCTS bot is 33.9 kB of the 100 kB allowed.

Speed, measured with `cargo run --release -p uttt-engine --example value_speed -- 2` (two runs, one thread of a shared 2.1 GHz Intel Xeon). Untrained networks with random weights replace the playout at each new leaf. Positions are every position of 1,000 games of decisive moves:

| Leaf estimate | Parameters | Time per estimate | Weights in the file, 8 bits each in base64 |
| --- | --- | --- | --- |
| `uttt-v008` playout: 16 policy moves, then decisive moves | 32 | 0.84 to 0.90 µs | |
| Network 217-32-1 | 7,009 | 0.50 to 0.54 µs | 9 kB |
| Network 217-64-16-1 | 15,009 | 0.54 to 0.56 µs | 20 kB |
| Network 217-128-32-1 | 32,065 | 0.99 to 1.05 µs | 43 kB |
| Network 217-256-32-1 | 64,065 | 1.61 to 1.63 µs | 85 kB, too large |

Iterations of a 90 ms search (the bot's budget per turn), average of the two runs, each over 8 positions after that many decisive moves. With random weights the tree's shape differs from what a trained network would give, so these counts are indicative:

| Iterations per search, thousands | Move 1 | Move 11 | Move 21 | Move 31 | Move 41 |
| --- | --- | --- | --- | --- | --- |
| `uttt-v008` playouts | 63 | 66 | 83 | 99 | 99 |
| Network 217-32-1 | 127 | 132 | 102 | 94 | 74 |
| Network 217-64-16-1 | 118 | 112 | 103 | 90 | 80 |
| Network 217-128-32-1 | 93 | 81 | 70 | 64 | 57 |
| Network 217-256-32-1 | 72 | 61 | 51 | 46 | 43 |

A network of up to about 15,000 parameters costs less than a playout. It leaves about 1.8 times the iterations early in the game, when playouts are long, and about 0.8 times late, when they are short. The question is therefore not speed but quality: whether one network estimate judges a position better than one playout does.

The arithmetic is in `f32`, in loops that the compiler vectorizes with SSE2, which every x86-64 processor has. Writing the second layer as dot products made the 128-32 network 2.5 times faster than the first version. Integer arithmetic could perhaps halve the times again; that was not measured, and it is not needed yet. CodinGame's processor and compiler flags are not known ([CODINGAME.md](../CODINGAME.md)).

## Decision

1. **The network: 217 inputs, then 64 and 16 hidden units, then one output; about 15,000 parameters.** The inputs describe the position from the side to move's view:
   - its marks and the opponent's in each open small board (81 + 81);
   - who owns each closed small board: the side to move, the opponent, or nobody because it is full (27);
   - the small board the side to move is sent to, or a free choice (10);
   - the open boards where each side has a cell that wins the board (9 + 9), which the board already keeps up to date.

   Hidden units use a clipped ReLU (between 0 and 1). The output goes through a sigmoid to give the side to move's expected score. This size is as fast as the smallest one measured and leaves about 40 kB for stage 3's policy. If the training report shows it underfits (a larger network does clearly better on held-out games), 217-128-32-1 is the next candidate: about 1 µs per estimate and 43 kB of weights.
2. **In the search, the network replaces the playout.** At a new leaf, a finished game gives its exact score. A side to move that has a move winning the game scores a win. Otherwise the network gives the estimate. UCT, the proofs of wins and losses, and tree reuse stay as they are. Network values are smoother than playout results, so the exploration constant gets its own experiment afterwards. Mixing a playout with the network, as AlphaGo did, costs both; it is kept as a fallback if the network alone loses.
3. **Data: self-play games of the current bot's search,** v008's policy playouts included, at 10,000 iterations per move after 6 random moves. Each game records its moves and its result. Each searched position also records the root's average score, so that this target can be compared later. Positions where the side to move can win the game at once are left out, since the search never asks the network there. On one core, a game at 10,000 iterations per move takes about a quarter of a second and gives about 43 positions. Eight Train jobs for an hour should give about 400,000 games, or 17 million positions.
4. **Training: in `uttt-trainer`, in Rust, without a machine-learning library** ([ADR 0016](0016-self-play-training.md), decision 3):
   - The target is the game's result for the side to move (1, ½ or 0), with cross-entropy as the loss, as in AlphaZero.
   - Adam on mini-batches, spread over the fit job's 4 cores.
   - Each position is drawn in one of the board's 8 symmetries at random.
   - 5% of games are held out, whole games: positions of one game are too alike to be split between training and testing.
5. **A gate before any SPRT.** On held-out positions, the network's squared error against the game result must be lower than that of one v008 playout from the same positions. The report says how many averaged playouts the network is worth. If the network does not beat a single playout, no bot is built: the report goes to the owner, with what to change.
6. **In the bot:**
   - The trainer writes the weights as Rust source: 8 bits each, one scale per layer, as base64 in a string constant. The report gives the held-out loss after this rounding.
   - The bot decodes the weights to `f32` once at start-up.
   - The input encoding and the forward pass live in the engine and are shared by the bot and the trainer; a test checks that the two agree.
7. **Pull requests, in order:**
   1. The engine: inputs, forward pass, decoding of the weights, tests.
   2. The trainer: a second data format with results and root scores, self-play with the bot's playouts, `fit-value` with the gate in its report, and a Train workflow input to choose the stage.
   3. A Train run on GitHub, then its report to the owner.
   4. Experiment E012: the bot with the network, judged like any other by an SPRT and a confirmation at CodinGame's limits.

## Consequences

- The bot's knowledge can grow with its parameters at an almost fixed cost per leaf, and stage 3 reuses the inputs, the trainer and the workflow: a policy output is one more layer.
- Training becomes real machine learning, with overfitting and rounding to watch. A poor network would make the bot much weaker, since it replaces every playout; the gate and the SPRT stop that before it reaches CodinGame.
- The bundled MCTS bot should be about 34 kB of code, 4 kB for the network's code and 20 kB of weights: about 58 kB.
- The value learned is that of the self-play bot at 10,000 iterations per move, weaker than at CodinGame's limits. Later runs can train on the new bot's own games, the loop of stage 3.
