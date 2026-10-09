# 0018. Judge the value network by games at equal iterations

- Status: accepted (owner, 2026-10-08); decision 1 (games at 3,000 iterations) superseded by [0019](0019-value-network-gate-at-time.md)
- Date: 2026-10-08
- Supersedes: decision 5 of [ADR 0017](0017-value-network.md)
- Scope: uttt

## Context

[ADR 0017](0017-value-network.md)'s decision 5 set a gate before any SPRT. On held-out positions, the network's squared error against the game result had to be lower than that of one `uttt-v008` playout from the same positions.

The first local measurement ([pull request #28](https://github.com/Arthur-Vat/codingame-bots/pull/28)) showed that this gate says almost nothing. The data: 2,000 self-play games with v008's playouts at 10,000 iterations per move. On 4,110 positions of held-out games, the squared errors were:

- one playout: 0.374;
- the average result, a constant: 0.177;
- the network: 0.164.

A constant passes. One playout's error is mostly its own noise, since its result is 0, ½ or 1, and a search averages that noise away over many visits. The network passed. Yet in pairs of games at 3,000 iterations per move, its search lost to the playouts' search by 280 Elo with the bot's exploration constant (0.5), and by 119 Elo with the best setting tried (exploration 0.1).

What matters is whether the network's estimates make the search choose better moves. Games measure that directly, and the trainer now plays them in a few minutes of the fit job.

## Decision

1. **The gate is games against the current bot's playouts.** In each training run's report, two searches play pairs of games at the same number of iterations per move:
   - one with the network, its weights rounded as the bot will hold them;
   - one with the current bot's playouts.

   Each pair starts from 6 random opening moves, and the sides swap seats within it. The default is 400 pairs at 3,000 iterations per move. The playouts' search keeps the bot's exploration constant. The network's is set for each run (0.1 for the first); its estimates spread less than playout results.
2. **The network passes when it is not clearly weaker:** the 95% interval of its Elo advantage, computed from the pairs as the arena does, reaches 0 or above.
3. **Only a network that passes goes into a bot,** with the exploration constant it passed with. That bot is judged by an SPRT and a confirmation at CodinGame's limits, as any experiment is. Equal iterations is a cautious test: the network costs about half a playout's time (ADR 0017), and the SPRT weighs equal time.
4. **The prediction errors stay in the report, for information.**

## Consequences

- The gate measures what the bot needs, at the cost of a few minutes per run.
- With 400 pairs, the interval is about ±25 Elo wide. A network slightly weaker at equal iterations may pass; the SPRT then decides, which is its job.
- The result depends on the network's exploration constant. A run whose network fails at one constant may be measured again at another without training again.
