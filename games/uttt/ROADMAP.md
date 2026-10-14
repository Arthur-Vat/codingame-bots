# Ultimate Tic-Tac-Toe: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The game was built in phases 1 to 6 of the [main roadmap](../../docs/ROADMAP.md), before the lifecycle existed; this file maps that work onto the stages, and stages 3, 8 and 11 were written afterwards as the worked example. Before the lifecycle, the owner's answers were his approvals of each pull request.

**Stage in progress:** 10. Iteration, paused (strength work paused by the owner, 2026-10-09)
**Owner's stopping rule:** Legend reached on 2026-10-07; further work at the owner's call

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | Done (2026-10-07) | Game chosen ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)) |
| 2 | Rules | Done (2026-10-07) | [RULES.md](RULES.md) merged (phase 1) |
| 3 | Model | Done (2026-10-13), after the fact | [MODEL.md](MODEL.md) |
| 4 | Framework gaps | Done (2026-10-07) | None: the framework was built for this game |
| 5 | Referee and engine | Done (2026-10-07) | Parity on 10,000 random games; `rules-check` matched CodinGame in Bronze (owner, 2026-10-07; phase 2) |
| 6 | Arena and baseline bots | Done (2026-10-07) | Random, greedy and first-valid bots; SPRT and league (phases 1 and 3) |
| 7 | Studio | Done (2026-10-13) | Phase 6 merged on the owner's approvals |
| 8 | Research | Done (2026-10-13), after the fact | [research/](research/README.md) |
| 9 | Approach and first bot | Done (2026-10-07) | MCTS, `uttt-v001` pasted: Legend, 182nd of 443 ([E001](journal/E001-mcts.md)) |
| 10 | Iteration | In progress, paused | Releases up to `uttt-v010`; experiments in the [journal](journal/README.md) |
| 11 | Retrospective | Done (2026-10-13), as the worked example | [RETROSPECTIVE.md](RETROSPECTIVE.md) |

## Current stage

Iteration is paused. When the owner resumes it, each experiment follows "An experiment, step by step" in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Plan for iteration

Proposed on 2026-10-14 from the write-up of a player at the top of the arena ([research](research/README.md#sources)). Not started. Strength work stays paused until the owner resumes it, and each step that runs experiments or training waits for his go-ahead. Each change to the bot is one experiment: a journal entry, a candidate release and an SPRT.

**Where we stand.** The bot is MCTS with playouts, guided by a small learned policy. `uttt-v008` reached 59th place. The write-up says this design reaches about the top 50 with excellent speed work, and that the top bots evaluate positions with a network instead. Its TrueSkill gap of about 9 between the first and the 50th means the first is expected to win over 90% of their games against each other. Our value network did not beat the playouts ([E012](journal/E012-value-network.md); run [37814220623](training/runs/37814220623/report.md) only tied them when mixed half and half). Three things differ from the write-up's network:

- ours is evaluated from scratch at each leaf;
- it has 32,000 parameters, against about 215,000;
- it was fitted on 80 million positions, against 300 million.

### A. Room in the file

The networks below do not fit as base64 beside the bot's code: `uttt-v010` already takes 77,026 of the 100,000. [Packing into characters](../../docs/research/utf16-packing.md) gives about 2.6 times as much room.

1. **Probe on CodinGame (the owner): done 2026-10-14.** A Rust bot of 95,713 characters and 125,475 bytes was accepted at submission, its packed data intact ([CODINGAME.md](../../docs/CODINGAME.md#submission-limits)). The processor has AVX2, but CodinGame does not compile for it.
2. **Decision and ADR (the owner): done 2026-10-14.** `scripts/cg-check.sh` counts UTF-16 units instead of bytes, at 100,000 (ADR 0037, pull request #120).
3. **Tools, all in Rust (owner, 2026-10-14), as framework work for every game ([roadmap](../../docs/ROADMAP.md#outside-the-phases)):** done: the decoder `cg-pack`, the encoder `cg-packer` with its command, and a bundler test that proves the gate below. The trainer packs the network with `cg-packer` in step B4.
4. **If more room is needed:** the bundler removes indentation ([ADR 0016](../../docs/adr/0016-self-play-training.md), point 4).

Gate: a bot of more than 100,000 bytes and fewer than 100,000 characters is accepted in the arena (passed by the probe), and `cg-check.sh` accepts it in CI (passed: `crates/cg-bundler/tests/packed_data.rs`).

### B. An NNUE evaluation

An efficiently updatable network. Its first layer, the accumulator, changes by adding or subtracting a few rows of weights when a move is played, instead of being computed again.

1. **Inputs, from the side to move:**
   - each player's 81 cells and 9 won small boards;
   - the 9 drawn small boards.

   That is 189 inputs, all 0 or 1. The next small board to play is not an input: it picks one of 10 output buckets, one per small board and one for "any board".
2. **Shape: (189 → N) × 2 → 10.** One first layer is applied to the position from each side, and the two accumulators side by side feed the chosen bucket. The activation is the squared clipped ReLU, and the output a sigmoid. Weights are quantized: 16-bit accumulators, 8-bit weights where the loss allows. The write-up uses N = 1,024, about 215,000 parameters, so one first layer shared by both sides. We start at N = 256, about 54,000 parameters, which the packing fits.

   Items 1 and 2 are in `uttt_engine::nnue` (inputs keep the cells of closed boards, so a move only adds features): quantized inference with incremental accumulators, byte layout and arithmetic in the module's documentation.
3. **Data:** positions from self-play of the current bot, labelled with the game's result, read with all eight symmetries. The write-up used 300 million positions. The Train workflow gave 80 million from 2 million games in one run; how many runs is the owner's call.
4. **Trainer:** fit the NNUE and export it quantized and packed.

Gate: on held-out positions, a lower error than run 37814220623's network. The cost of an update and of an evaluation is measured in the engine's speed example.

### C. Search with the NNUE

One experiment each, judged by the SPRT.

Room: with `uttt_engine::nnue` the MCTS bot's bundle takes 83,952 characters, and a packed network of width 256 about 27,250, so the bot of C1 must drop or pack other data, such as the pattern policy's weights.

1. **MCTS with the network at the leaves**, accumulators built from scratch: the baseline for the next steps.
2. **"jacekmax"**, named after the CodinGame player jacek, who made it popular. When the search reaches a leaf, it expands all its children at once. It builds the leaf's accumulator once and evaluates each child with an incremental update. It then backs up the minimax value of the children instead of one result. The write-up keeps accumulator checkpoints at nodes with many visits.
3. **Selection on the average of backed-up values,** mixed with the minimax value by a weight. In the write-up, the best weight drifted to the average alone as the bot got stronger; its author does not know why. Screen the weight locally first.
4. **Exploration constant and time budget,** tuned again for the new search.

### D. Scale

One experiment each.

1. **A larger accumulator:** 512, then 1,024 if the room allows. At 1,024, 8-bit weights take about 215 KB, more than the at most 190 KB that packing gives, code included. They then need fewer bits per weight: train with the weights rounded to 5 or 6 bits, then code them with an entropy coder (rANS or arithmetic, a few dozen lines of Rust in the bot), which spends fewer bits on the common values near zero. Expected about 4 to 5 bits per weight, to be measured on the first network. A general compressor such as LZ77 or LZMA would gain less on weights, for a larger decoder.
2. **More data:** hundreds of millions of positions (the write-up's next run has a couple of billion), then self-play of the NNUE bot itself.
3. **Speed:** safe Rust that the compiler vectorizes, first: SSE2 only, since CodinGame compiles for basic x86-64. Then, if the network takes much of the time, AVX2 through one function compiled for it and called after a check at run time, with a fallback. That call needs `unsafe`, so an ADR.

   Measured on 2026-10-14, so that it is not redone: the engine's speed example built with AVX2, FMA, BMI1, BMI2, POPCNT, LZCNT and SSE4.2 on for the whole program, against the normal build, three runs each on Claude's session. Nothing moved beyond the machine's noise of about 10%:

   | Benchmark | Normal build | All those instructions on |
   | --- | --- | --- |
   | Random playouts per second | 976,000 to 1,095,000 | 1,081,000 to 1,139,000 |
   | Decisive playouts per second | 1,191,000 to 1,245,000 | 1,144,000 to 1,381,000 |
   | MCTS iterations per second | 718,000 to 782,000 | 686,000 to 782,000 |

   Today's bot works on 9-bit masks, random numbers and the tree, none of which AVX2 speeds up, and a bot that dispatches at run time would gain less than this build of the whole program. So AVX2 waits for a network: measure the network's share of each iteration first, and bring the ADR only if that share is large (owner, 2026-10-14).
4. **Symmetry in the first layer:** neurons in groups of 8 sharing one set of weights, each turned by one of the board's 8 symmetries. The stored first layer is 8 times smaller at the same width; whether it plays as well is the experiment.
5. **Ideas the write-up lists but has not tried:** more hidden layers, other activations, and a training loss that favours weights that compress well.

### E. Maybe: alpha-beta with the NNUE

NNUE is usually paired with alpha-beta, whose depth-first order suits incremental updates. MrSubZero is in the top 3 with alpha-beta and an NNUE. The write-up's author tried it and stalled at 40% against their own MCTS: it needs move ordering, a transposition table, iterative deepening, extensions and much tuning. Only once C and D stop gaining. Then a bake-off against the MCTS bot at 20 ms, then at full time.

### Not planned

The write-up's bot is a Python file that writes and runs a compiled C program. That gave it Clang, its own compiler flags and profile-guided optimization, about 20% more simulations per second. CodinGame bans that in active contests, and our bots are Rust that CodinGame compiles ([ADR 0010](../../docs/adr/0010-codingame-rust-toolchain.md)). It would take a decision of the owner and an ADR.

## Open questions

- Resume strength work with the plan above? Answered 2026-10-14: not yet.

## Decisions

- 2026-10-14: the packing tools are framework work for every game: the decoder `cg-pack` and the encoder `cg-packer`, two crates, no ADR.
- 2026-10-14: no AVX2 for now. Measured no gain on today's bot; it waits for the network (step D3).
- 2026-10-14: every tool of the packing is Rust; strength work not resumed yet.
- 2026-10-13: Ultimate Tic-Tac-Toe is documented to the lifecycle's standard as its worked example, documents only ([ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)).
- 2026-10-09: strength work paused while phase 5 is built; no training or league run by hand.
