---
id: E035
date: 2026-11-13
pull_request: "#297"
parent: ice-and-fire-v016
hypothesis: v016's pressure term counts a side's gold toward its chain to the enemy HQ on top of the gold's own value, so while that chain is within 200 gold of reach each gold kept scores twice and spending looks costly; v014 kept 53% of its gold in the middle game against the top 10's 37% (46% in its wins, 54% in its losses); counting less of the gold in that term, or only near the HQ, makes the search spend and beats v016.
change: None released. Screening only: the plan bot's evaluation counting a share of a side's gold (0 to all, in quarters) in the pressure term, whose lowest value was also raised from -200 in some variants.
release: none
sprt: not run: no gain over ice-and-fire-v018 in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the first of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md)),
run beside E036 and E037. E037 (`ice-and-fire-v017`, #295) and E036
(`ice-and-fire-v018`, #296) were promoted in turn; on top of v018 this
experiment no longer gained, and the owner dropped it ("Drop E035
(Recommended)").

## Why

The shared evaluation (`ice_and_fire_tactics::eval`) counts a side's gold
at 1 in what it has, and again in its pressure term,
`clamp(gold + income - cost, -200, 0)`, where `cost` is its cheapest chain
of trainings to the enemy HQ. While that chain costs between the side's
gold plus income and 200 more, a gold kept counts 2 and a level-1 training
that takes a free cell scores -6 (-10 gold, -10 pressure, +10 unit, +4
cell) rather than +4. In the report's games (`spending_kept.csv`), v014
keeps 53% of its gold in the middle game, the top 10 37%; v014 keeps 46%
in its wins and 54% in its losses. E007 (keeping 20 gold) lost; E021
changed the term's shape by how many turns the chain is away, not the
gold's weight, and gained nothing.

## The change

`tactics/src/eval.rs`: the pressure term takes a `Pressure` (the quarters
of the gold it counts, its lowest value); `evaluate` keeps v016's
(all of the gold, -200) and `evaluate_with` takes another.
`bots/plan/src/eval.rs`: `PRESSURE`, the bot's, and `shared`, the shared
evaluation with it; the analysis mode's pressure term is the bot's.

The code is commit `247e15e` of this branch (at v016's settings), each
variant one commit after it, and the commit back to v016's settings
`00e919f`; the same change on v017 is `6bb0f84` (branch
`claude/ice-and-fire-e035-on-v017`) and on v018 `1a9c301`
(`claude/ice-and-fire-e035-on-v018`). This pull request removes it and
keeps the entry.

## Screening

All on GitHub (`screen.yml`, one calibrated runner per variant and shard
at CodinGame's speed and real limit, ADR 0048; no timeout, crash or
invalid answer). "g n" counts n quarters of the gold (4 in v016); "cap"
is the term's lowest value (−200 in v016).

First round against `ice-and-fire-v016`, run 43449162131, seed 35035, 800
pairs each over two runners:

| Variant | Commit | Elo against v016 | Score |
| --- | --- | --- | ---: |
| g 0 | `9d9e8b7` | **−70.0 [−84.9, −55.3]** | 40.1% |
| g 1 | `4e4ed96` | **−31.4 [−46.1, −16.7]** | 45.5% |
| g 2 | `366226d` | +3.9 [−10.1, +17.9] | 50.6% |
| g 3 | `03b2a4c` | **+14.3 [+0.7, +28.0]** | 52.1% |
| cap 100 | `dbf320f` | −3.9 [−15.4, +7.6] | 49.4% |
| cap 50 | `144b624` | **−32.2 [−45.8, −18.7]** | 45.4% |
| g 2, cap 100 | `9afffbf` | −7.8 [−20.6, +4.9] | 48.9% |
| g 0, cap 100 | `68ed9d5` | **−84.6 [−99.2, −70.3]** | 38.1% |

g 3 again, 1,600 pairs each over four runners:

| Against | Run, seed | Commit | Elo | Score |
| --- | --- | --- | --- | ---: |
| `ice-and-fire-v016` | 43452995063, 35036 | `03b2a4c` | **+11.9 [+2.3, +21.6]** | 51.7% |
| `ice-and-fire-v017` (E037) | 43458078480, 35037 | `6bb0f84` | +8.5 [−1.2, +18.1] | 51.2% |
| `ice-and-fire-v018` (E036) | 43464481930, 35038 | `1a9c301` | −6.5 [−15.8, +2.8] | 49.1% |

## What was learned

The hypothesis is mostly wrong: counting less of the gold in the pressure
term, or capping the term nearer, lost, and heavily so for the larger
changes (g 0 −70, g 1 −31, cap 50 −32). The term's gold is not a mere
double count: it values gold kept toward the chain to the enemy HQ, which
the search needs. Only a small cut (three quarters) helped, about +12
against v016, and less on each release that followed; on top of E036,
which also makes spending cheaper in the evaluation (a level-3 unit
charged one turn of upkeep), it no longer gained. Whether v014's high
share of gold kept against the top 10 (53% against 37%) costs games
stays open.
