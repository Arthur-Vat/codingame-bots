---
id: E001
date: 2026-10-19
pull_request: "#179"
parent: bronze (no release yet)
hypothesis: Bronze's turn plus four steps from the shared tactics - a lethal chain that knows the towers, a defence of the HQ, cuts of the enemy's territory and gold kept for the winning chain - plays better than the greedy bronze policy alone, and each step adds to the one before.
change: A ladder of four reference bots, chain, guard, cutter and banker, each adding one step to the one before, all built from the crate games/ice-and-fire/ladder; the last, banker, is the first release.
release: ice-and-fire-v001
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

The bots are `games/ice-and-fire/bots/{chain,guard,cutter,banker}`, each a
`main` that runs the crate `games/ice-and-fire/ladder` with its steps. The
crate's turn is the greedy policy of `games/ice-and-fire/policy` (copied into
the crate, so that `bronze` stays as it is) and, switched on one by one, the
steps below. With no step it plays exactly bronze's turn (a test compares
the commands on random positions). The owner wants several modest versions
in several pull requests, so this is the first step above `bronze`, with no
search beyond the cut finder. The reference bots are published as baseline
bots, in the league (`LEAGUE_BOTS`) and in the studio; the crate's play does
not change once merged (bug fixes only, recorded here), and later bots copy
what they need.

A turn, on a scratch state, in order:

1. **Lethal:** a movable unit next to the enemy HQ steps in, else a chain of
   trainings to the HQ if the gold pays for it and it wins: bronze's static
   chain, or with `tower_chain` the cheapest chain that knows the towers
   (`chain_to`).
2. **Moves:** bronze's, then lethal again.
3. **Defence** (`defence`): the enemy could take our HQ next turn if, after
   the end of our turn, its cheapest chain to the HQ (`Reach`, with its units'
   steps free) costs at most its gold. While that holds (3 rounds at most),
   the best of the towers on our empty active cells on the enemy's cheapest
   path (or next to it, or next to the HQ) and the lowest-level trainings on
   that path is played, scored by the margin (the enemy's cost minus its gold)
   it leaves; it is kept only if it raises the margin.
4. **Cuts** (`cuts`): up to 2 times, the first cut of `cuts(state, 4,
   gold - reserve, 8)` worth at least 10 that does not expose the HQ. The
   search is skipped once the turn has taken 3 ms. That deadline is fixed and
   does not read `CG_TIME_SCALE`: the evaluation's limit is 10 ms a turn and a
   search takes about 2 ms at most, so the bot behaves the same at every scale,
   in the arena and on CodinGame.
5. **Banking** (`banking`): with no threat, if our chain to the enemy HQ is
   affordable next turn with the gold and income we have, steps 6 and 7 are
   skipped.
6. **Buildings:** bronze's mines (and its tower by the HQ when the enemy is
   within 3 steps, but only if no defence was applied in step 3).
7. **Trainings:** bronze's.
8. **Last check** (with `defence`): if steps 4 to 7 exposed the HQ when it
   was safe after step 3, they are undone.

| Bot | Steps | Played against | Score | Elo (95%) | Faults |
| --- | --- | --- | --- | --- | --- |
| `chain` | `tower_chain` | `bronze` | 200 wins, 0 draws, 200 losses (50.0%) | -0.0 [-0.0, -0.0] | none |
| `guard` | + `defence` | `chain` | 222 wins, 0 draws, 178 losses (55.5%) | +38.4 [+17.4, +59.7] | none |
| `cutter` | + `cuts` | `guard` | 306 wins, 0 draws, 94 losses (76.5%) | +205.0 [+172.5, +241.1] | none |
| `banker` | + `banking` | `cutter` | 203 wins, 0 draws, 197 losses (50.7%) | +5.2 [-8.8, +19.3] | none |
| `banker` | all | `bronze` | 301 wins, 0 draws, 99 losses (75.2%) | +193.2 [+158.3, +231.9] | none |

Each row is 200 seat-swapped pairs, seed 1, at the evaluation's limits (a
time scale of 0.2 and 5 ms of tolerance), Bronze rules, no opening plies,
the bundles compiled with Rust 1.90.0. Against random, 100 pairs: `banker`
won 200 games of 200, no fault.

What the table says: `chain` plays exactly like `bronze` (every pair split
one to one): on these maps `chain_to` and bronze's static chain find the same
chains, so the towers do not change a lethal move yet. The defence adds about
38 Elo, the cuts most of the gain, and banking is not distinguishable from
nothing (+5.2, interval across 0) at 200 pairs. Only `banker` is released,
as the best bot of the ladder and the one tested by the SPRT; the SPRT
against `bronze` decides whether the step is worth keeping, and the league
then rates the reference bots beside it.

**Time** (`banker` built with Rust 1.90.0, 300 pairs against `bronze`, 1 job,
the bot's own clock per turn, 8,138 turns): mean 0.12 ms, 99.9% under 1.8 ms, longest 3.2 ms. The
bundle is 60,282 characters. In that run each bot lost one game on time to the
machine's load (wrapped in a shell script that logs its stderr); the bot's own
clock never passed 3.3 ms.

The SPRT result, the Elo and the decision are added after the pull request's
SPRT.
