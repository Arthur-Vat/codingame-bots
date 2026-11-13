---
id: E037
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v016
hypothesis: v016's threatened cells (E023) are those the enemy's chains of trainings reach for its gold plus income, so a unit at the front threatens nothing though its move costs no gold, while the top 10 of CodinGame remove 0.53 of enemy value per gold spent on units against our 0.35 and win the exchange (+30 against our -20 at turn 15), and most large cuts use moves (E019); letting the enemy's units step first in that reach makes the search value forward units and guard against them, and beats v016.
change: In the plan bot's evaluation (and its tower guard), the enemy's units of at least a chosen level on its active cells step first onto every neighbour they may enter, at no cost, before its chains of trainings, when the cells it can take at its next turn are found; the weight of a threatened cell is screened with it.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the third of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
section 6), run beside E035 and E036.

## Why

`exposed` in `bots/plan/src/eval.rs` (E023) calls
`reach_within(state, enemy, false, budget)`: no moves, and the shared
search lets units step only for the player to act, which in the
evaluation is most often us. So the threatened cells, the mine bonus
(E033) and the tower guard (E026) ignore what the enemy's units can take
for free at its next turn, and our own forward units add nothing to the
enemy cells we threaten. The HQ threat test already counts moves.

## The change

`tactics/src/chain.rs`: `reach_within_moving`, the same search with any
set of units stepping first (`reach_within` calls it with the player to
act's movable units, as before). `bots/plan/src/eval.rs`: `reach(state,
side)`, the cells a side can take at its next turn, its units of at least
`MOVER_LEVEL` (4: none, as v016) on its active cells stepping first, all
of which can move at its next turn; `exposed` and the tower guard
(`actions.rs`) use it. `THREAT_QUARTERS` (7, as v016) is a threatened
cell's weight in quarters.

## Screening

Pending.
