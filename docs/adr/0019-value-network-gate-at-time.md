# 0019. Judge the value network at the bot's time budget

- Status: accepted (owner, 2026-10-08)
- Date: 2026-10-08
- Supersedes: decision 1 of [ADR 0018](0018-value-network-gate.md)
- Scope: uttt

## Context

[ADR 0018](0018-value-network-gate.md) judged a value network by pairs of games against the bot's playouts at 3,000 iterations per move. The first network ([E012](../../games/uttt/journal/E012-value-network.md)) passed that gate by +134 Elo. In local screening as the bot, against `uttt-v008`, it then scored:

- about -21 Elo at 20 ms per move;
- -96 Elo at CodinGame's limits.

Its lead shrinks as the search grows. The bot searches 60,000 to 120,000 iterations per move, so games at 3,000 iterations judged the network far from where it plays.

The network also does not search faster than playouts over a whole game. At full time, the bot with it ran about as many iterations per turn as v008. Only games at the bot's time budgets weigh both its judgement and its cost.

E012 also showed that the network mixed with a playout, the fallback of [ADR 0017](0017-value-network.md), did better than the network alone at CodinGame's limits: -21 Elo against -96, both within wide intervals.

The owner agreed on 2026-10-08 to judge networks at the bot's real budget, together with a larger network and a longer training run.

## Decision

1. **The gate's games are played at the bot's time budgets:** 900 ms of search on each side's first searched move, then 90 ms per move, as the bot searches at CodinGame's limits. The trainer plays them in its own process, one game per thread, so both sides of a game share the same machine and its load. The default is 300 pairs from 6 random opening moves, seats swapped within each pair.
2. **Two leaf estimates are measured:** the network alone, and half network, half playout. Both run against the current bot's playouts.
3. **ADR 0018's criterion stays.** A variant passes when it is not clearly weaker: the 95% interval of its Elo advantage reaches 0 or above. The network's exploration constant is set for each run, 0.3 by default after E012's screening.
4. **A passing variant goes into a bot,** with the exploration constant it passed with. That bot is judged by an SPRT and a confirmation at CodinGame's limits, as before.

## Consequences

- The gate costs more: about 50 minutes of the fit job for both variants, against a few minutes before. The job's 6 hours leave room.
- The gate now measures what the confirmation at full time measures, but on GitHub's runners in one process rather than through the arena. It screens a network before a bot is built; the SPRT and the confirmation still decide.
- Iteration-count games stay available (`--duel-iterations`), for quick comparisons.
