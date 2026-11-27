---
id: E103
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v034
hypothesis: The opponent's own climb decides most of the search's scores and takes the largest share of its time, and half of its climbs in a turn end on a plan found before; sharing what its climbs learn within a turn and starting each from its best earlier plan keeps the replies as good for less time, and beats ice-and-fire-v034.
change: The opponent's climbs of a turn share their action gains (`OPP_HINTS`) and each starts from the best of the opponent's earlier plans of the turn that still applies to the position (`OPP_WARM`), then climbs on.
release: ice-and-fire-v035
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

After E101 and E102 found nothing to gain in the time v034 leaves unused
or in the actions it is offered, the owner chose to work on the model of
the opponent (2026-11-27). E099's counts in v033's CodinGame games: the
opponent's own climb (E019) gives the most damaging reply in 82.5% of the
reply searches and decides the score of 81.5% of our tries; the reply
searches took 43% of the turn, the opponent's climb alone 32%; 44% of the
reply searches of a turn found exactly the replies of an earlier one, and
50% of the opponent's climbs ended on a plan found earlier in the turn.
Each of those climbs started from nothing, with no memory of what the
earlier ones learnt.

## Change

`bots/plan/src/search.rs`, `Search::replies`:

- `OPP_HINTS`: the opponent's climbs of a turn share their hints (the
  best gain each action had), as our own climbs of a turn do, so that a
  later climb tries first what paid in the earlier ones and its lazy
  passes stop sooner.
- `OPP_WARM`: the opponent's plans found in the turn are kept (16 at
  most); a new reply search replays each on the position after our plan,
  keeps those the rules accept, and starts the opponent's climb from the
  one its evaluation rates best (a forced start, E029), climbing on from
  there; with none, it climbs from nothing as before.
- `OPP_DIVERSE` (off): after a warm start, one more climb barred from the
  warm plan's first action, to spend the time saved on a second reply.

With all three off the bot plays as v034 (the same answers and scores on
the 2,031 positions of v033's 159 CodinGame ranking games at 600 tries).
The candidate `ice-and-fire-v035` has `OPP_HINTS` and `OPP_WARM` on.

## Local measures

On the same 2,031 positions, each variant's search at 600 tries: the time
a search (`speed`, 407 positions, one core of the session's machine) and
the surprise check (`surprise`: on the normal plies, a large miss is a
real reply 50 or more worse for us than the worst reply the search
modeled; too low, 50 or more better):

| Variant | ms a search | Large misses | Expected 50+ too low | Median surprise |
| --- | ---: | ---: | ---: | ---: |
| v034 (all off) | 15.45 | 59 (3.2%) | 21.4% | +19 |
| `OPP_HINTS` | 13.67 | 56 (3.0%) | 22.2% | +20 |
| `OPP_WARM` | 12.21 | 57 (3.1%) | 21.4% | +18 |
| `OPP_WARM` and `OPP_DIVERSE` | 17.05 | 54 (2.9%) | 24.2% | +22 |
| `OPP_HINTS` and `OPP_WARM` (v035) | 11.04 | 56 (3.0%) | 21.1% | +17 |

The model's replies are as good with the reuse (large misses 3.0 to 3.1%
against 3.2%), for 12 to 29% less time; the second climb finds a few more
misses but turns more pessimistic (24.2% expected too low), as E098 found.
The two together are the candidate: the same replies for 29% less time a
search. Since E101 found that the turns ending early do not gain from more
time, the gain can come only from the turns the clock ends, so it is
likely small: the owner asked for the SPRT to judge it.

## SPRT

Pending.
