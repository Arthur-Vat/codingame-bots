---
id: E092
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Counting in the threat what the enemy's best affordable cut severs beyond the cell it takes, found in one depth-first search a side so that the search keeps its speed, beats ice-and-fire-v032 where E090's union of every cut, 37% slower, did not.
change: With `eval::BEST_CUT`, a side's threat adds, to its exposed cells (E023, E037), the cells behind the best single capture among them (`best_cut`, a depth-first search for the joints, as the shape term's), at `THREAT_QUARTERS` each; off in v032.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E090 counted as threatened every cell behind any affordable cut, with one
flood a cut in every evaluation: its search ran 37% fewer tries a
millisecond and lost 36 Elo, which left the idea untested. The owner asked
(2026-11-27) to run it again made cheap, and the attacker plays one chain
a turn, so only its best cut counts here (the cells lost at a ply in
v032's CodinGame games: a fifth to two fifths of those it could afford,
research/codingame/2026-11-27-cell-values.md).

## Change

`eval::best_cut(state, seat, taken)`: one depth-first search of `seat`'s
active land from its HQ, which sums below each joint in `taken` the
importance of the cells it cuts off (`THREAT_QUARTERS` each, or E093's
values) and keeps the largest. `threat_quarters` adds it to the exposed
cells' cost. The exposed cells themselves, read by the mine term and the
tower offers, do not change.

Speed (`ice-and-fire-bot-plan speed`, 2,000 tries on the 300 positions of
E090's bank, the fastest of 2 runs): `off` 25.3 tries a ms, `best-cut`
20.3, `best-cut-values` 20.0, `values` 26.0.

## Screening

Pending: with E093's variants in one run.
