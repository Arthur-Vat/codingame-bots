---
id: E030
date: 2026-10-16
pull_request: "#148"
parent: uttt-v016
hypothesis: AlphaZero's selection rule (PUCT), with priors from the estimates the network already gives a node's children, steers the search toward the moves the network rates highly and away from the rest, so the same iterations go to better lines and the bot is stronger.
change: When a node's children are created and estimated, each gets the prior `softmax(estimate / 0.1)` over its siblings, and selection adds `0.25 · prior · sqrt(parent visits) / (visits + 1)` to UCB1's bound; a fresh root's children get their priors at once. No new network, and the same estimates.
release: uttt-v017
sprt: accepted
elo: +58.5 [+32.0, +85.6] at CodinGame's limits
pairs: 117
full_time: +58.5 [+32.0, +85.6] (the SPRT's own, ADR 0020), no timeout in 234 games, 99.9% of answers within 93.0 ms
decision: promoted
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

**SPRT** ([run 38887044759](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38887044759)), at CodinGame's limits, 4-ply openings, seed 1:

```
Smoke test against uttt-random:
uttt-v017 vs random: 200 games, 100 complete pairs
  uttt-v017: 200 wins, 0 draws, 0 losses (score 100.0%)

SPRT against uttt-v016:
uttt-v017 vs uttt-v016: 234 games, 117 complete pairs
  uttt-v017: 78 wins, 117 draws, 39 losses (score 58.3%)
  Elo of uttt-v017 over uttt-v016: +58.5 [+32.0, +85.6] (95%, from pairs)
  pairs by uttt-v017's points (0, 1/2, 1, 3/2, 2): [2, 13, 54, 40, 8]
  uttt-v017 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v016 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v017 90.1, 91.2, 93.0 ms; uttt-v016 90.1, 91.7, 93.0 ms
  slowest answer: uttt-v017 93.9 ms, uttt-v016 96.3 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.00 (bounds -2.94, 2.94) after 117 pairs
Verdict: ACCEPTED: uttt-v017 is stronger than uttt-v016
```

The SPRT agrees with the screening (+54.3). It ran on d8f7ecf; the later commits change only the studio's e2e test and this entry, and CI reused its result.

**What follows.** The first AlphaZero ingredient that fits the bot pays as much as a new network did (E024, +58.0). The temperature and weight were screened coarsely at 20 ms (six settings), so finer tuning at full time may add more. A learned policy is the next AlphaZero ingredient; the network's own estimates now play its part.

