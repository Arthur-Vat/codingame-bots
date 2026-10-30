---
id: E029
date: 2026-10-30
pull_request: "#229"
parent: ice-and-fire-v013
hypothesis: In many middle-game turns v013's climbs run out of first actions that improve the plan and stop well before the time is up, though an action that does not pay alone can open a plan that does; climbing from each remaining first action in turn until the time is up beats v013 (candidate `ice-and-fire-v014`).
change: When a climb ends on an empty plan (no first action left improves it), the next climbs, while time remains, are forced to start with each of the root's actions not yet tried as a first action, the best gain so far first, and their plans are scored, given replies and chosen among like any other.
release: ice-and-fire-v014
sprt: accepted
elo: +14.4 [+3.4, +25.4] at CodinGame's limits
pairs: 894
full_time: +14.4 [+3.4, +25.4] (the SPRT's own, ADR 0042), 1 timeout in 1,788 games, slowest answer 612.9 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-10-30 ("Go on with the most relevant experiment
next"), step 4 of the time roadmap (phase 3: use idle time).

## Why

In 800 positions of v011 self-play searched at 600 tries (the measures
before E027), 29% of the middle-game turns ended early, the climbs having
found no improving first action left, and in 40% of those a climb from
another first action found a plan the bot's own judge rated better (in 23%
by 10 points or more). Those turns stopped at a few milliseconds of their
36 ms.

## The change

`bots/plan/src/search.rs`: in `think`, a climb that ends on an empty plan
used to end the search. Now the root's actions (generated once, E028) not
yet banned as a first action are queued by the best gain each had in the
climbs so far (`hints`), and each later climb is forced to start with the
next one (`Search::forced`: the action applied, then a climb from there,
without the root's cached actions). The forced plans are scored, given
their replies (which join the memory) and chosen among like the others, up
to 32 climbs or the soft limit. The debug line says whether a turn was
forced.

In 20 sanity games against v013 (seed 29001), 66% of the middle-game turns
reached the forced climbs (the faster search of E028 runs out of first
actions more often), with 24 climbs in the median such turn; answers after
the first: median 36.9 ms against v013's 21.9, 99.9% 39.8 ms, no timeout.

## Screening

Against `ice-and-fire-v013` (#228's candidate) at CodinGame's limits
(50 ms a turn, 1,000 ms the first, no tolerance), 400 seat-swapped pairs,
seed 29029, two games at a time on a 4-core machine:

| Bot | Elo against v013 | Timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| E029 | **+24.4 [+4.8, +44.1]** | 1 (v013 3) | 36.2, 39.2, 41.1 ms (v013 14.6, 38.7, 40.7) |

The time v013 left idle in the turns whose climbs ran out of first actions
is worth climbing in. The candidate is `ice-and-fire-v014`: its readable
copy differs from the bundle screened only in `forced.sort_by_key(gain)`,
written `|a| gain(a)` before (clippy's `redundant_closure`), the same code.

## SPRT

Accepted on the pull request (run 41123558597, at 8d35062), at
CodinGame's limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v014 vs ice-and-fire-v013: 1788 games, 894 complete pairs
  ice-and-fire-v014: 931 wins, 0 draws, 857 losses (score 52.1%)
  Elo of ice-and-fire-v014 over ice-and-fire-v013: +14.4 [+3.4, +25.4] (95%, from pairs)
  pairs by ice-and-fire-v014's points (0, 1/2, 1, 3/2, 2): [86, 0, 685, 0, 123]
  ice-and-fire-v014 faults: 1 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v013 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v014 36.9, 39.6, 40.6 ms; ice-and-fire-v013 21.4, 39.1, 40.5 ms
  slowest answer: ice-and-fire-v014 612.9 ms, ice-and-fire-v013 612.7 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.99 (bounds -2.94, 2.94) after 894 pairs
Verdict: ACCEPTED: ice-and-fire-v014 is stronger than ice-and-fire-v013
```

## League

The league of the release (run 41123558597, at 8d35062, 800 pairs a
pairing) scored v014 52.7% against v013 (843 wins, 757 losses), 53.8%
against v012, 60.2% against v011, 63.7% against v010, 67.9% against v009,
71.6% against v008 and 100.0% against bronze, and rated it +2374.1 ± 182.5,
against v013's +2353.2 ([LEADERBOARD.md](../LEADERBOARD.md)). v014 lost 1
of its 11,200 games by a fault.

## Not done

From the review (#229), for a later change: a direct test of
`Search::forced` (the test here only shows that forced climbs happen; the
final pass drops any command the rules refuse); the comment on ties (the
climbs pop from the end, so ties come out in reverse order); the queue
filtered by `banned` only, so a first action whose plan taught the memory
can be forced again and rebuild its plan (filtering by the plans' first
actions too is a behaviour change to screen); `Thought::forced` is a count.
