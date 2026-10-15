# Ultimate Tic-Tac-Toe: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The game was built in phases 1 to 6 of the [main roadmap](../../docs/ROADMAP.md), before the lifecycle existed; this file maps that work onto the stages, and stages 3, 8 and 11 were written afterwards as the worked example. Before the lifecycle, the owner's answers were his approvals of each pull request.

**Stage in progress:** 10. Iteration (strength work resumed by the owner with step B of the plan below, 2026-10-14)
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
| 10 | Iteration | In progress | Releases up to `uttt-v010`; experiments in the [journal](journal/README.md) |
| 11 | Retrospective | Done (2026-10-13), as the worked example | [RETROSPECTIVE.md](RETROSPECTIVE.md) |

## Current stage

Iteration resumed on 2026-10-14 with step B of the plan below. Each experiment follows "An experiment, step by step" in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Plan for iteration

Proposed on 2026-10-14 from the write-up of a player at the top of the arena ([research](research/README.md#sources)). Step A done; step B started on 2026-10-14 at the owner's request. Each step that runs experiments or training waits for his go-ahead. Each change to the bot is one experiment: a journal entry, a candidate release and an SPRT.

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

1. **Inputs, from each side's perspective** (the side to move's and the other player's):
   - each player's 81 cells and 9 won small boards;
   - the 9 drawn small boards.

   That is 189 inputs, all 0 or 1. The next small board to play is not an input: it picks one of 10 output buckets, one per small board and one for "any board".
2. **Shape: (189 → N) × 2 → 10.** One first layer is applied to the position from each side, and the two accumulators side by side feed the chosen bucket. The activation is the squared clipped ReLU, and the output a sigmoid. Weights are quantized: 16-bit accumulators, 8-bit weights where the loss allows. The write-up uses N = 1,024, about 215,000 parameters, so one first layer shared by both sides. We start at N = 256, about 54,000 parameters, which the packing fits.

   Items 1 and 2 are in `uttt_engine::nnue` (inputs keep the cells of closed boards, so a move only adds features): quantized inference with incremental accumulators, byte layout and arithmetic in the module's documentation.
3. **Data:** positions from self-play of the current bot, labelled with the game's result, read with all eight symmetries. The write-up used 300 million positions. The Train workflow gave 80 million from 2 million games in one run; how many runs is the owner's call.
4. **Trainer:** fit the NNUE and export it quantized and packed. Done by `uttt-trainer fit-nnue` and the Train workflow's `nnue` stage (the split and the held-out measurements are those of `fit-value`, so the gate's numbers are comparable).

Gate: on held-out positions, a lower error than run 37814220623's network. The cost of an update and of an evaluation is measured in the engine's speed example.

**Gate passed (2026-10-14).** Train run 38562652097 fitted a network of width 256 for 8 epochs on run 37814220623's data, in 26 minutes. On the same 4,182,825 held-out positions, the quantized network scores better than that run's rounded network:

| Network | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| Value network 217-128-32-1, run 37814220623, rounded | 0.1522 | 0.6249 |
| NNUE (189 → 256) × 2 → 10, run 38562652097, quantized | 0.1498 | 0.6199 |

The held-out error was still falling at the eighth epoch. The packed network takes 27,264 characters. At width 256, the engine's speed example measures about 12.1 million updates and 5.4 million evaluations per second. The weights are in the MCTS bot since E017.

### C. Search with the NNUE

One experiment each, judged by the SPRT.

Room: the pattern policy's weights are packed and the value network is out of the engine, so the MCTS bot's bundle (`uttt-mcts`, with `uttt_engine::nnue`) took 63,182 characters before E017. With the network and `NnueBoard`, `uttt-v011` takes 91,929, leaving 8,071 of the 100,000 for steps C2 to C4. With E018's search, `uttt-v012` takes 95,106, leaving 4,894; the pattern policy's weights, unused since E018, take about 10,000 of them. Without them, the bot's bundle takes 84,397, leaving 15,603. With the policies' code moved out of the engine into `uttt-policy`, which no bot uses, it takes 70,188, leaving 29,812.

1. **MCTS with the network at the leaves**, accumulators built from scratch: the baseline for the next steps. Done: [E017](journal/E017-nnue-leaves.md), `uttt-v011`, +38.0 Elo [+17.2, +59.0] against `uttt-v010` at CodinGame's limits (exploration 0.3; the pattern policy still orders children).
2. **"jacekmax"**, named after the CodinGame player jacek, who made it popular. When the search reaches a leaf, it expands all its children at once. It builds the leaf's accumulator once and evaluates each child with an incremental update. It then backs up the minimax value of the children instead of one result. The write-up keeps accumulator checkpoints at nodes with many visits. Done: [E018](journal/E018-jacekmax.md), `uttt-v012`, +101.3 Elo [+64.4, +140.6] against `uttt-v011` at CodinGame's limits (no checkpoints: the node's accumulator is built from scratch; selection on the minimax value only; children no longer ordered by the pattern policy).
3. **Selection on the average of backed-up values,** mixed with the minimax value by a weight. In the write-up, the best weight drifted to the average alone as the bot got stronger; its author does not know why. Screen the weight locally first. Done: [E020](journal/E020-minimax-weight.md), `uttt-v013`, weight 0.5, +88.7 Elo [+54.9, +124.4] against `uttt-v012` at CodinGame's limits. Unlike in the write-up, the average alone was clearly worse (-77.7 Elo at 20 ms).
4. **Exploration constant and time budget,** tuned again for the new search. Exploration done: [E021](journal/E021-exploration.md), `uttt-v014`, 0.2 instead of 0.3, +18.2 Elo [+5.2, +31.3] against `uttt-v013` at CodinGame's limits. The time budget is skipped (owner, 2026-10-15): each turn's 100 ms stands alone, and `uttt-v014` already searches 90 ms of it with about 5 ms of margin (99.9% of answers within 92 to 95 ms), so 2 to 3 ms more is worth a few Elo at most, below what an SPRT can measure, against more timeouts. Pondering, thinking during the opponent's turn, is forbidden ([CODINGAME.md](../../docs/CODINGAME.md#timing)).

### D. Scale

One experiment each.

1. **A larger accumulator:** 512, then 1,024 if the room allows. At 1,024, 8-bit weights take about 215 KB, more than the at most 190 KB that packing gives, code included. They then need fewer bits per weight: train with the weights rounded to 5 or 6 bits, then code them with an entropy coder (rANS or arithmetic, a few dozen lines of Rust in the bot), which spends fewer bits on the common values near zero. Expected about 4 to 5 bits per weight, to be measured on the first network. A general compressor such as LZ77 or LZMA would gain less on weights, for a larger decoder.

   Tried first at 512 on run 37814220623's data: [E019](journal/E019-nnue-512.md), dropped. The fit (run 38634035143, 12 epochs) scored only a little better than the 256 network (squared error 0.1493 against 0.1498), and the bot was no stronger: -27.9 Elo at 20 ms and -3.5 at full time against `uttt-v012`. Width waits for more data (D2) and a faster evaluation (D3). The minifier and weight compression for 1,024 (plan of 2026-10-14) wait with it.
2. **More data:** hundreds of millions of positions (the write-up's next run has a couple of billion), then self-play of the NNUE bot itself.

   The fit first got faster (2026-10-15, measured on Claude's session, 4 threads, batches of 1,024, three to five runs each): threads that live through the training and share the sum of the gradients and Adam's step gained 5% at width 256 and 12% at 512, with the same weights bit for bit; building for the processor (`-C target-cpu=native`, now in the Train workflow) brought the total to 64% and 48% (330,000 to 540,000 and 206,000 to 304,000 positions per second). Larger batches would gain more (15% to 35% at 4,096 to 16,384 before these changes) but change the training, so they wait for a sweep.

   Then self-play by the bot itself (2026-10-15): the Train workflow's `nnue` stage plays the newest release's search and network (exploration 0.2, all children judged at once, minimax weight 0.5) instead of `uttt-v010`'s playouts. Its opening moves are searched, recorded and drawn in proportion to the search's visits, as AlphaZero does, instead of uniformly random and unrecorded, so the network also learns the opening positions; a game whose first search after the opening expects above 0.9 or below 0.1 is dropped (owner, 2026-10-15). Measured on Claude's session at 5,000 iterations: about 0.65 s per game on one core; in 570 games of each kind, no position before ply 24 had a root score beyond 0.1 or 0.9 (8-ply sampled openings: 5.9% beyond 0.3 or 0.7 at plies 8 to 11; 6-ply random ones: 2.5%), and every opening differed by ply 8. The first player won 36%, drew 56% to 59% and lost 5% to 8%.

   Its first run (38733114779, 1.9M games of 8 sampled plies) held copies: only 1.30M distinct positions at ply 8, since the search with the network is deterministic from a position (40 games without an opening and with different seeds were one game), so two games whose sampled moves match are the same game, and some of them were both fitted and held out. Since then (2026-10-15, agreed with the owner): `fit-nnue` drops a game whose moves copy an earlier one, from any file, before the split, and counts them in its report; and the `nnue` stage draws its opening moves in proportion to the square root of the visits (`--opening-temperature 2`). Measured on Claude's session at 5,000 iterations, 4,000 openings of 8 plies each, from each opening's probability (the expected share of copies among M games is 1 minus the average of (1 - (1 - q)^M) / (qM) over sampled openings of probability q; it gives 31% at 1.9M games for the first run's openings, which matches the 32% that run found):

   | Opening draw | Copies at 2M games | at 5M | Root score beyond 0.3 or 0.7 at ply 8 |
   | --- | --- | --- | --- |
   | Visits (temperature 1) | 31.7% | 44.5% | 5.4% |
   | Temperature 1.5 | 8.4% | 15.9% | 5.3% |
   | Temperature 2 | 2.9% | 6.6% | 4.8% |
   | Temperature 3 | 1.1% | 2.6% | 5.2% |
   | 12 plies at temperature 1 (1,660 openings) | 1.5% | 2.9% | 12.2% (at ply 12) |

   No first search after these openings was beyond 0.1 or 0.9. The fit's report also measures the bot's current network on the same held-out positions and gives, on an even sample of them, the error and how often the network's best child is the move played, by ply, the calibration and the first layer's dead units.

   The first run's network in the bot, [E023](journal/E023-nnue-self-play.md), dropped: -177 Elo at full time in local screening against `uttt-v015`. On fresh `uttt-v015` games it predicted results a little better than the bot's network, but its best child was the move played less often (38% to 40% against 42% to 45%), with 31 to 39 dead units against 6. Fitted only on the bot's own narrow games, it learned the positions the bot plays into, not the children the search must reject. The same data at width 512 (run 38784586293, copies dropped) screened at -56 Elo: better than the 256, still weaker than `uttt-v015`, with the lower agreement again (41.0% against 42.1%). Data that also covers positions after moves the bot would not play comes before more self-play of this kind.

   So, the owner's choice (2026-10-15), self-play now plays side games: from each game kept, `--side-games N` games each leave it at one of its searched positions after the opening by a random legal move other than the one played, then play on with the same search, recording only the positions after that move. A random move inside the game itself would have given the positions before it the result of a game it changed; a side game gives each position the result of its own game, and the game stays the same. Measured on Claude's session at 5,000 iterations, 240 games of 8 sampled plies, 4 processes:

   | Side games per game | Time | Positions | Share from side games |
   | --- | --- | --- | --- |
   | 0 | 33 s | 11,896 | 0% |
   | 1 | 47 s | 16,337 | 27% |
   | 2 | 59 s | 21,370 | 44% |

   A side game records about 20 positions, against about 50 for a game: it starts after the opening, and a random move often loses quickly.
3. **Speed:** safe Rust that the compiler vectorizes, first: SSE2 only, since CodinGame compiles for basic x86-64. Then, if the network takes much of the time, AVX2 through one function compiled for it and called after a check at run time, with a fallback. That call needs `unsafe`, so an ADR.

   Measured on 2026-10-14, so that it is not redone: the engine's speed example built with AVX2, FMA, BMI1, BMI2, POPCNT, LZCNT and SSE4.2 on for the whole program, against the normal build, three runs each on Claude's session. Nothing moved beyond the machine's noise of about 10%:

   | Benchmark | Normal build | All those instructions on |
   | --- | --- | --- |
   | Random playouts per second | 976,000 to 1,095,000 | 1,081,000 to 1,139,000 |
   | Decisive playouts per second | 1,191,000 to 1,245,000 | 1,144,000 to 1,381,000 |
   | MCTS iterations per second | 718,000 to 782,000 | 686,000 to 782,000 |

   Today's bot works on 9-bit masks, random numbers and the tree, none of which AVX2 speeds up, and a bot that dispatches at run time would gain less than this build of the whole program. So AVX2 waits for a network: measure the network's share of each iteration first, and bring the ADR only if that share is large (owner, 2026-10-14).

   The network's share, measured on 2026-10-15 by the engine's speed example (its "NNUE search" rows, three runs; method in the [README](README.md#engine-speed)): in a search with the bot's network and settings, building accumulators from scratch takes 21% to 25% of the time, copying them for each child 3% to 5%, playing each child's move (its rows, and the rules) 11% to 16%, evaluating the children 30% to 37%, and the rest (tree, moves, rules) 21% to 35%. An iteration walks down 8.4 moves and expands 0.99 nodes of 7.8 children, 7.3 of which the network judges; an expanded position has 29.5 inputs on per seat. So the network takes two thirds to three quarters of the time.

   Then, accumulators carried by the position and updated with each move as the search walks down its tree, instead of built from scratch at each expanded node: [E022](journal/E022-walk-updates.md), `uttt-v015`, +7.6 Elo [+1.8, +13.5] against `uttt-v014` at CodinGame's limits. The same searches find exactly the same results in 22% to 27% less time, and at CodinGame's limits the bot runs 11% to 20% more iterations per turn in the middle game. After it, updating the accumulators on the walk takes about a fifth of the time and evaluating the children 36% to 39%; the write-up's checkpoints at nodes with many visits could save part of the walk's share. Next for D3, separately: adding the move's rows and evaluating in one pass, without the per-child copy; a loop shaped for SSE2's `pmaddwd`; width 128.
4. **Symmetry in the first layer:** neurons in groups of 8 sharing one set of weights, each turned by one of the board's 8 symmetries. The stored first layer is 8 times smaller at the same width; whether it plays as well is the experiment.
5. **Ideas the write-up lists but has not tried:** more hidden layers, other activations, and a training loss that favours weights that compress well.

### E. Maybe: alpha-beta with the NNUE

NNUE is usually paired with alpha-beta, whose depth-first order suits incremental updates. MrSubZero is in the top 3 with alpha-beta and an NNUE. The write-up's author tried it and stalled at 40% against their own MCTS: it needs move ordering, a transposition table, iterative deepening, extensions and much tuning. Only once C and D stop gaining. Then a bake-off against the MCTS bot at 20 ms, then at full time.

### Not planned

The write-up's bot is a Python file that writes and runs a compiled C program. That gave it Clang, its own compiler flags and profile-guided optimization, about 20% more simulations per second. CodinGame bans that in active contests, and our bots are Rust that CodinGame compiles ([ADR 0010](../../docs/adr/0010-codingame-rust-toolchain.md)). It would take a decision of the owner and an ADR.

## Open questions

- Resume strength work with the plan above? Answered 2026-10-14: not yet; then, the same day, "go on with step B".

## Decisions

- 2026-10-14: step B resumed ("go on with step B"); its first fit reuses run 37814220623's self-play data, no new self-play ("Reuse run 37814220623").
- 2026-10-14: the packing tools are framework work for every game: the decoder `cg-pack` and the encoder `cg-packer`, two crates, no ADR.
- 2026-10-14: no AVX2 for now. Measured no gain on today's bot; it waits for the network (step D3).
- 2026-10-14: every tool of the packing is Rust; strength work not resumed yet.
- 2026-10-13: Ultimate Tic-Tac-Toe is documented to the lifecycle's standard as its worked example, documents only ([ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)).
- 2026-10-09: strength work paused while phase 5 is built; no training or league run by hand.
