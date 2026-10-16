---
id: E030
date: 2026-10-16
pull_request: "#149"
parent: uttt-v016
hypothesis: AlphaZero's selection rule (PUCT), with priors from the estimates the network already gives a node's children, steers the search toward the moves the network rates highly and away from the rest, so the same iterations go to better lines and the bot is stronger.
change: When a node's children are created and estimated, each gets the prior `softmax(estimate / 0.1)` over its siblings, and selection adds `0.25 · prior · sqrt(parent visits) / (visits + 1)` to UCB1's bound; a fresh root's children get their priors at once. No new network, and the same estimates.
release: uttt-v017
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Proposed to the owner on 2026-10-16 as the AlphaZero ingredient that fits the bot (AlphaZero's policy network does not fit in 100,000 characters, but the children's estimates are there already), and asked for the same day: "Also can't you try PUCT, output buckets by game phase now?"

**The change** (`cg-search`'s `Mcts::value_temperature`, with the existing `prior_weight`): with `expand_all`, a temperature `T` above 0 turns the progressive bias into PUCT. A node's children get their priors from their estimates when they are created, and the bias `prior_weight · prior / (visits + 1)` is multiplied by the square root of the parent's visits. UCB1's exploration term (0.2) and the minimax weight (0.5) stay. A temperature of 0 searches exactly as before (tested). The analysis program, the trainer's `selfplay` (`--value-temperature`, `--prior-weight`) and the Train workflow's nnue self-play search as the release does, and their tests check it.

**Screening against `uttt-v016`** (Claude's 4-core session, both bots compiled with Rust 1.90.0, 4-ply openings, seed 1, 3 games at a time, 100 pairs each). At 20 ms, 15 ms of tolerance:

| Temperature | Weight | Elo |
| --- | --- | --- |
| 0.05 | 0.25 | +41.9 [+12.4, +72.0] |
| 0.1 | 0.1 | +24.4 [-4.6, +53.6] |
| 0.1 | 0.25 | +33.1 [+3.1, +63.7] |
| 0.1 | 0.5 | +22.6 [-4.1, +49.6] |
| 0.2 | 0.25 | +41.9 [+11.6, +72.9] |
| 0.2 | 0.5 | -10.4 [-41.4, +20.4] |

The weight 0.25 did best at every temperature; the middle temperature, 0.1, went to full time: 74 wins, 83 draws, 43 losses, **+54.3 Elo [+21.7, +87.9]**, 2 timeouts each, 99.9% of answers within 94.2 ms (95.0 ms for `uttt-v016`). The bundle takes 71,871 characters.
