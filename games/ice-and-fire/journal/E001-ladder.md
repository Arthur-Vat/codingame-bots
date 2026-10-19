---
id: E001
date: 2026-10-19
pull_request: "TBD"
parent: bronze (no release yet)
hypothesis: A turn planned in phases, with a lethal chain that knows the towers, a defence of the HQ, cuts of the enemy's territory and gold kept for the winning chain, plays better than the greedy bronze policy, which only grows, kills and takes the HQ when it can pay for it.
change: First release, the phases bot - bronze's moves, mines and trainings, with the shared tactics added around them - a lethal chain, a defence when the enemy could take our HQ next turn, up to 2 cuts, banking, and a last check that undoes spending that exposed the HQ.
release: ice-and-fire-v001
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

The bot is `games/ice-and-fire/bots/phases`, grown from the greedy policy
of `games/ice-and-fire/policy` (its planner is copied into the bot, so that
`bronze` stays as it is) and the shared tactics of
`games/ice-and-fire/tactics`. The owner wants several modest versions in
several pull requests, so this is the first step above `bronze`, with
no search beyond the cut finder. A turn, on a scratch state, in order:

1. **Lethal:** a movable unit next to the enemy HQ steps in, else the
   cheapest chain of trainings to the HQ (`chain_to`, aware of towers) if
   the gold pays for it and it wins.
2. **Moves:** bronze's, then lethal again.
3. **Defence:** the enemy could take our HQ next turn if, after the end of
   our turn, its cheapest chain to the HQ (`Reach`, with its units' steps
   free) costs at most its gold. While that holds (3 rounds at most), the
   best of the towers on our empty active cells on the enemy's cheapest path
   (or next to it, or next to the HQ) and the lowest-level trainings on that
   path is played, scored by the margin (the enemy's cost minus its gold)
   it leaves; it is kept only if it raises the margin.
4. **Cuts:** up to 2 times, the first cut of `cuts(state, 4, gold - reserve,
   8)` worth at least 10 that does not expose the HQ.
5. **Banking:** with no threat, if our chain to the enemy HQ is affordable
   next turn with the gold and income we have, steps 6 and 7 are skipped.
6. **Buildings:** bronze's mines (and its tower by the HQ when the enemy is
   within 3 steps, but only if step 3 did not run).
7. **Trainings:** bronze's.
8. **Last check:** if steps 4 to 7 exposed the HQ when it was safe after
   step 3, they are undone.

The cut search is skipped once the turn has taken 3 ms; nothing else in a
turn comes close.

**Screening against `bronze`** (the bundles built with Rust 1.97, the
arena at the evaluation's limits, a time scale of 0.2 and 5 ms of
tolerance, Bronze rules, no opening plies), first parameters, 4 jobs:

| Games | Seed | `phases` | Elo over `bronze` (95%) | Faults |
| --- | --- | --- | --- | --- |
| 200 pairs | 1 | 301 wins, 0 draws, 99 losses (75.2%) | +193.2 [+158.3, +231.9] | none |
| 300 pairs | 7 | 483 wins, 0 draws, 117 losses (80.5%) | +246.3 [+214.3, +282.4] | none |
| 300 pairs, 1 job | 11 | 474 wins, 0 draws, 126 losses (79.0%) | +230.2 [+201.5, +261.9] | none |

Against random, 100 pairs, seed 1: 200 wins in 200 games, no fault. The
parameters of steps 3 to 5 were not changed: the first variant won.

**Time** (300 pairs, 1 job, the bot's own clock per turn, 8,153 turns): mean
0.11 ms, 99.9% under 1.7 ms, longest 4.5 ms (5.2 ms in the 4-job run). The
bundle is 57,852 characters.

The SPRT result, the Elo and the decision are added after the pull request's
SPRT.
