---
id: E027
date: 2026-10-30
pull_request: "#NN"
parent: ice-and-fire-v011
hypothesis: v011 can play a plan whose own worst reply it no longer remembers (the memory keeps the 8 most damaging replies over all plans) or never searched (a plan found as the time ran out); scoring each plan also against its own replies, and searching them for the plan to play when they are missing, beats v011.
change: Each plan keeps the opponent's replies found for it; the final choice scores a plan by the worst over the memory and its own replies; while the best plan has none, its replies are searched in the time left before the hard limit (40 ms) and the choice is made again.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-30 ("Start fix 1 first"), after a reading of
the plan bot's thinking loop and a measure of v011's time per turn.

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
| full: own replies and the check | pending | | |
| own: own replies only (`VERIFY` off) | pending | | |
