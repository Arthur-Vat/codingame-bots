---
id: E002
date: 2026-10-07
pull_request: pending
parent: uttt-v001
hypothesis: Keeping the search tree between turns adds strength, since part of the previous search (and the whole first-turn search) is about the positions that follow.
change: After each search, the subtree of the new position, found within two moves of the previous root, becomes the new tree with its visits; otherwise the search starts afresh.
release: uttt-v002
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

Kept visits are modest: a few hundred to a few thousand per turn at
20 ms, against 6,000 to 13,000 new iterations, so the gain comes mostly
from the positions the search explored most.

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs): +43.7 Elo [+12.7, +75.3] against
v001.
