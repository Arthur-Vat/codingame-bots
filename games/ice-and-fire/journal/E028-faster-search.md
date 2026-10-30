---
id: E028
date: 2026-10-30
pull_request: "#228"
parent: ice-and-fire-v012
hypothesis: v012's search answers alike when it is faster, and since its climbs stop at 36 ms in most middle-game turns, the same search done 1.3 times faster scores more plans in its time and beats v012 (candidate `ice-and-fire-v013`).
change: Three changes that keep every answer at a fixed number of tries: the evaluation's shape term found by its own search with byte arrays and only the two numbers it uses, and kept by territory in a cache; the root's actions generated once a turn rather than once a climb; and a tower offered without copying the state where it cannot protect a cell the enemy can reach (unless the HQ guard may apply).
release: ice-and-fire-v013
sprt: accepted
elo: +12.2 [+2.5, +21.9] at CodinGame's limits
pairs: 1024
full_time: +12.2 [+2.5, +21.9] (the SPRT's own, ADR 0042), no timeout in 2,048 games, slowest answer 619.6 ms (the first turn)
decision: promoted
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

Against `ice-and-fire-v012` at CodinGame's limits (50 ms a turn, 1,000 ms
the first, no tolerance), 400 seat-swapped pairs, seed 28028, two games at
a time on a 4-core machine:

| Bot | Elo against v012 | Timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| E028 | **+19.1 [+4.2, +34.1]** | 1 (v012 1) | 15.7, 39.0, 41.0 ms (v012 20.3, 39.5, 42.9) |

The same plans found sooner: the median answer is shorter, since a turn
whose climbs end on their own ends earlier, and where the soft limit binds
the faster search scores more plans in it. The candidate is
`ice-and-fire-v013`, this change alone.

## SPRT

Accepted on the pull request (run 41101919381, at b3bd50f), at
CodinGame's limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v013 vs ice-and-fire-v012: 2048 games, 1024 complete pairs
  ice-and-fire-v013: 1060 wins, 0 draws, 988 losses (score 51.8%)
  Elo of ice-and-fire-v013 over ice-and-fire-v012: +12.2 [+2.5, +21.9] (95%, from pairs)
  pairs by ice-and-fire-v013's points (0, 1/2, 1, 3/2, 2): [89, 0, 810, 0, 125]
  ice-and-fire-v013 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v012 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v013 19.7, 39.1, 41.3 ms; ice-and-fire-v012 26.4, 39.7, 41.6 ms
  slowest answer: ice-and-fire-v013 619.6 ms, ice-and-fire-v012 626.7 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.95 (bounds -2.94, 2.94) after 1024 pairs
Verdict: ACCEPTED: ice-and-fire-v013 is stronger than ice-and-fire-v012
```

## League

The league of the release (run 41101919381, at b3bd50f, 800 pairs a
pairing) scored v013 50.7% against v012 (812 wins, 788 losses), 56.5%
against v011, 60.1% against v010, 67.6% against v009, 69.4% against v008,
76.1% against v007 and 99.8% against bronze, and rated it +2352.4 ± 182.5,
against v012's +2344.9 ([LEADERBOARD.md](../LEADERBOARD.md)): a small gain,
as the SPRT's needed 1,024 pairs. v013 lost no game by a fault.
