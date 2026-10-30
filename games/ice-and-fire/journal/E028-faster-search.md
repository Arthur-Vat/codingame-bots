---
id: E028
date: 2026-10-30
pull_request: "#NN"
parent: ice-and-fire-v012
hypothesis: v012's search answers alike when it is faster, and since its climbs stop at 36 ms in most middle-game turns, the same search done 1.3 times faster scores more plans in its time and beats v012.
change: Three changes that keep every answer at a fixed number of tries: the evaluation's shape term found by its own search with byte arrays and only the two numbers it uses, and kept by territory in a cache; the root's actions generated once a turn rather than once a climb; and a tower offered without copying the state where it cannot protect a cell the enemy can reach (unless the HQ guard may apply).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Phase 2 of the time roadmap the owner approved on 2026-10-30 ("Merge 227,
and do phase 2 right now"): a faster search with the same answers, judged
by the speed benchmark of #226.

## Why

A profile of v012's search (callgrind, 30 positions of the speed bank at
600 tries) gave the evaluation 63% of the time, the shape term
(`ice_and_fire_opening::solo::severed`, run for both sides in every
evaluation) 31%, the action generator 16% (mostly the tower check of E026,
a copy of the state and a reach search per candidate cell), and zeroing
arrays 10%. The study of the next ideas (E027's) found that twice the tries
gives a better plan by the bot's own judge in 21% of positions and a worse
one in 7%.

## The change

- `eval.rs`, `weakness`: the joints plus the most cells one capture cuts
  off, found by the same depth-first search as `severed` with arrays of
  bytes, keeping only those two numbers (the cells a capture cuts off do not
  depend on the order of the search), and a cache of 4,096 slots by
  territory and seat, kept between turns, since most actions leave one
  side's territory as it was. The engine's `Bits` gains `words`, the key.
- `search.rs`: the actions of the climbs' root are generated once a search
  rather than once a climb (every climb starts from the turn's state).
- `actions.rs`, `guards`: a tower only protects its own cell and the cells
  next to it (RULES.md), so where none of them is among the cells the
  enemy's chains reach, it cannot lower the cells the enemy can take, and the
  state is not copied unless the HQ guard may apply.

Tried and dropped: one end of our turn per plan scored rather than per
reply (slower: a plan's scoring mostly stops at its first reply).

On the speed bank (`speed`, 200 positions, 600 tries, fastest of 3 runs,
three runs each alternating), every answer is v012's, in 2,080 ms against
2,789: 51.8 of our tries a millisecond against 38.6, 1.34 times as many.
Tests check, on positions of 16 random games, that the shape term equals
`severed`'s joints and cuts (through the cache twice) and that the
generator offers exactly the towers E026's check on a copy offers.

## Screening

Pending.
