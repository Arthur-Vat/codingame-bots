---
id: E035
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v016
hypothesis: v016's pressure term counts a side's gold toward its chain to the enemy HQ on top of the gold's own value, so while that chain is within 200 gold of reach each gold kept scores twice and spending looks costly; v014 kept 53% of its gold in the middle game against the top 10's 37% (46% in its wins, 54% in its losses); counting less of the gold in that term, or only near the HQ, makes the search spend and beats v016.
change: The plan bot's evaluation counts a share of a side's gold (0 to all, in quarters) in the pressure term, whose lowest value may also be raised from -200; the shared evaluation keeps its own term.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the first of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md)),
run beside E036 and E037.

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

## Screening

Pending.
