---
id: E027
date: 2026-10-30
pull_request: "#NN"
parent: ice-and-fire-v011
hypothesis: v011 can play a plan whose own worst reply it no longer remembers (the memory keeps the 8 most damaging replies over all plans) or never searched (a plan found as the time ran out), and leaves 16 ms of its 40 ms idle; scoring each plan also against its own replies, searching them for the plan to play when they are missing, and climbing until 32 ms beats v011.
change: Each plan keeps the opponent's replies found for it; the final choice scores a plan by the worst over the memory and its own replies; while the best plan has none, its replies are searched in the time left before the hard limit (40 ms) and the choice is made again; the middle game's climbs may start until 32 ms rather than 24 ms (the opening's deadline unchanged).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-30 ("Start fix 1 first"), after a reading of
the plan bot's thinking loop and a measure of v011's time per turn. After
the first screening (below) the owner chose to add the second fix, more
time for the climbs, to the same experiment ("Stack with fix 2").

## Why

v011's middle-game search climbs to a plan, searches the opponent's replies
to it (its kill chain, its 3 best cuts, bronze's greedy turn, its own climb
of 300 tries), adds them to a memory of the 8 most damaging replies, and
climbs again against that memory. At the end it plays the plan that scores
best against the final memory. Two cases let a plan be judged without the
reply that beats it:

- **Eviction.** The memory ranks a reply by how much it hurt the plan it was
  found for and keeps 8. It was full in the median turn of 40 v011
  self-play games (seed 777), so a reply that hurts only one plan can leave
  it, and that plan is then rescored without it.
- **No replies searched.** When the soft limit (24 ms) stops a climb (57% of
  the middle-game turns in the same games), its plan is kept without
  searching the replies to it, and on a tie the latest plan wins.

The final choice took under 1 ms, so the 16 ms between the soft limit and
the hard limit were idle.

## The change

`bots/plan/src/search.rs`: a plan keeps its own replies (`Plan::own`); the
final choice scores it by the worst over the memory and its own replies
(`final_score`). While the best plan has no replies (`VERIFY`), they are
searched with the opponent's climb allowed up to the hard limit, the plan
rescored, and the best chosen again. The memory, the climbs and the time
limits are v011's. The debug line counts these checks per turn. A test
checks that a plan cut short by the tries gets its replies searched without
changing the memory.

In 20 sanity games against v011 (seed 27001) the check ran in 9 of 297
middle-game turns (3%), once each, adding 2 to 5 ms.

## Screening

Against `ice-and-fire-v011` at CodinGame's limits (50 ms a turn, 1,000 ms
the first, no tolerance), 400 seat-swapped pairs each, seed 27027, two games
at a time on a 4-core machine.

| Variant | Elo against v011 | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| full: own replies and the check | +10.4 [−9.1, +30.0] | 1 (v011 2) | 19.8, 32.0, 33.9 ms (v011 19.0, 32.0, 34.3) |
| own: own replies only (`VERIFY` off) | +11.3 [−7.9, +30.5] | 3 (v011 0) | 20.1, 32.1, 34.7 ms (v011 19.8, 32.1, 34.1) |

Both lean ahead of v011 without a clear result, and alike: the check of
the plan to play, which runs in about 3% of the middle-game turns, adds
nothing seen; any gain comes from each plan keeping its own replies.

## With the climbs until 32 ms

`search.rs`, `CLIMB_SHARE`: the middle game's soft limit, after which no
climb or reply search starts, moves from 24 ms to halfway between it and
the hard limit, 32 ms; the final choice and its check use the rest. The
opening's deadline (40% of the way from 24 to 40 ms, about 30 ms; about 610
ms on the first turn) is unchanged. E014's soft30 and E016's t45 found no
gain in more time, but then the search ended on its own in most turns
(median answers of 6 to 7 ms); since E019's opponent climbs, v011 reaches
the soft limit in 57% of its middle-game turns.

Against `ice-and-fire-v011`, as above, 400 pairs each, seed 27028; c36
climbs until 36 ms (`CLIMB_SHARE` 0.75) to see whether more helps.

| Variant | Elo against v011 | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| c32 | **+37.5 [+16.9, +58.3]** | 1 (v011 2) | 20.0, 35.6, 39.1 ms (v011 19.3, 32.1, 34.3) |
| c36 | **+43.7 [+24.7, +62.9]** | 0 (v011 2) | 19.6, 39.1, 41.3 ms (v011 18.6, 32.0, 34.1) |

Both gain clearly: the time v011 left idle is worth playing for now that
the soft limit binds. c36 leans higher, with its 99.9% answer at 41.3 ms,
8.7 ms under CodinGame's 50.
