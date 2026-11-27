---
id: E098
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v033
hypothesis: The search's model of the opponent's reply misses most of the large cuts it suffers because the opponent's own plan is one climb, which always ends on the same local best; several climbs, each barred from the earlier first actions, find about half of those cuts at little cost, and beat ice-and-fire-v033.
change: In `Search::replies` the opponent climbs `REPLY_CLIMBS` times during the search and `CHECK_CLIMBS` times in the reply search of the plan chosen before it is played, each climb barred from the first actions of the earlier ones, with 300 more tries each; v033 is 1 and none.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked to keep working on the cuts we fail to see coming, with
the enemy's moves (2026-11-27), after his notes on v033's CodinGame games
(cuts we missed in 6 of 14 annotated losses). E064 had modeled more
threats (longer cuts, cuts after the enemy's units step toward our HQ)
and lost: every reply modeled counts at its worst, so threats a strong
opponent never plays make the search defensive. Its lesson was that the
model needs better guesses of the cut the opponent will choose, not more
cuts.

Measured on v033's 159 CodinGame ranking games (the surprise check:
59 large misses, plies where the real reply cost us 50 or more beyond the
worst reply the search modeled), from the state after our turn, with a
scratch analysis build of v033's search:

| Opponent's planning | Misses found | Control: 50+ worse than real | Time |
| --- | ---: | ---: | ---: |
| v033's model: one climb, 300 tries (1,200 or 4,800 the same) | 7% | 21% | 0.9 ms |
| 2 climbs, each barred from the earlier first actions | 22% | 23% | 1.4 ms |
| 4 climbs | 46% | 26% | 2.5 ms |
| 8 climbs | 63% | 28% | 4.6 ms |
| The opponent's whole search, 600 or 2,400 tries | 47%, 54% | 21%, 20% | 37, 68 ms |

*Found*: the variant's best reply leaves us at most 20 above the real
reply's evaluation. *Control*: 300 other plies of the same games; how
often the variant finds a reply 50 or more worse for us than the reply
played, which the search would then plan against (v033's own model: 21%).

One climb converges: more tries change nothing. Climbs that start
elsewhere find the cut in half the misses and more, while the control's
alarms rise by 2 to 7 points only. The misses they find hang on the
enemy's moves (without them the reply removes 3 cells on average); the
others are rich long chains of trainings (73 gold spent, 14 cells removed
without the moves), a separate question.

The cost is the catch: the search models the replies of every plan its
climbs find (up to 32 a turn), so several climbs there take time from
the search; the reply search of the plan chosen runs once or twice a
turn, so it can afford more.

## Change

`bots/plan/src/search.rs`: `Search::replies` runs the opponent's climb
`climbs` times, each barred from the first actions of the earlier climbs
(as our own climbs are, E014), with a budget of `REPLY_EVALS` (300) tries
a climb, and keeps each distinct plan as a reply. `REPLY_CLIMBS` is used
during the climbs; `CHECK_CLIMBS`, when set, searches the replies of
the plan chosen again with that many climbs before it is played (the
E027 check), and the choice is made again until the best plan has been
checked or the time is up. With 1 and `None` the bot plays as v033: the same
answers on 300 positions of v033's games at 600 tries.

## Screening

Variants (one commit each on this branch), 800 seat-swapped pairs each
against `ice-and-fire-v033`, whole games at CodinGame's limits, 4 shards:

([run 46026478326](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46026478326)):

| Variant | Commit | `REPLY_CLIMBS` | `CHECK_CLIMBS` | Elo over v033 [95%] |
| --- | --- | ---: | ---: | ---: |
| `off` | `490d49d` | 1 | none | +3.0 [−6.0, +12.1] |
| `c2` | `59e17e5` | 2 | none | −8.3 [−22.0, +5.5] |
| `c4` | `8bd4047` | 4 | none | −28.3 [−43.1, −13.6] |
| `k4` | `06a63c9` | 1 | 4 | +0.9 [−11.8, +13.6] |
| `k8` | `386eb73` | 1 | 8 | +4.8 [−9.0, +18.6] |

One timeout of `k4` in its 1,600 games; no other fault. The median answer
of `k4` and `k8` is about 2 ms longer than v033's (30 against 28 ms on the
same runner: the deeper check of the plan chosen); `c2` and `c4` take the
same time, their cost being fewer climbs of ours.

No variant beats v033. More climbs of the opponent during the search lose
(`c2` −8.3, `c4` −28.3, clearly): the search plans against more threats and
has less time to find its own plans, as E064 found with more cuts. More
climbs only for the plan chosen are level (`k4` +0.9, `k8` +4.8): the
check sees more of the cuts it would suffer, but the next best plan is
then as exposed, or no better plan is at hand in the time left.
