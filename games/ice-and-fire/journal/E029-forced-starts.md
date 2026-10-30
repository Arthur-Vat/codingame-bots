---
id: E029
date: 2026-10-30
pull_request: "#NN"
parent: ice-and-fire-v013
hypothesis: In many middle-game turns v013's climbs run out of first actions that improve the plan and stop well before the time is up, though an action that does not pay alone can open a plan that does; climbing from each remaining first action in turn until the time is up beats v013.
change: When a climb ends on an empty plan (no first action left improves it), the next climbs, while time remains, are forced to start with each of the root's actions not yet tried as a first action, the best gain so far first, and their plans are scored, given replies and chosen among like any other.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
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

Pending.
