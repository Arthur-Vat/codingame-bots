---
id: E002
date: 2026-10-07
pull_request: "#10"
parent: uttt-v001
hypothesis: Keeping the search tree between turns adds strength, since part of the previous search (and the whole first-turn search) is about the positions that follow.
change: After each search, the subtree of the new position, found within two moves of the previous root, becomes the new tree with its visits; otherwise the search starts afresh.
release: uttt-v002
sprt: accepted
elo: +49.6 [+25.5, +74.3] at 20 ms
pairs: 303
full_time: +73.3 [+55.8, +91.2] over 500 pairs, no fault
decision: promoted
cg_rank: pending
---

Kept visits are modest: a few hundred to a few thousand per turn at
20 ms, against 6,000 to 13,000 new iterations, so the gain comes mostly
from the positions the search explored most.

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs): +43.7 Elo [+12.7, +75.3] against
v001.

Result, from the SPRT comment on #10: smoke test 200 wins in 200 games;
SPRT accepted after 303 pairs (285 wins, 122 draws, 199 losses), LLR 2.98;
confirmation at CodinGame's limits 483 wins, 242 draws, 275 losses, no
fault, slowest answer 95.2 ms. The gain is larger at full time, where
each turn keeps more visits.

After the merge, the CI check playing every bot against random at full
time failed once: most likely a timeout, since the same check passed on
the pull request and answers reached 95 ms of the 100 allowed. v001 and
v002 time out equally often in a local test (2 games in 400 each, on a
busy 2-core machine), so tree reuse is not the cause: the 88 ms search
budget leaves too little margin. The next pull request lowers it to 82 ms.
