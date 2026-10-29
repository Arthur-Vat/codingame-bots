---
id: E013
date: 2026-10-24
pull_request: "#203"
parent: ice-and-fire-v005
hypothesis: v005's opening plans again whenever our side of the board differs from its plan (even for an enemy mine), spending 30 ms on a plan that assumes a passing opponent just as the sides meet; fixing the trigger, keeping the plan, ending the opening on contact or after a fixed number of turns beats v005.
change: None released. Local screening only: v005 with the mine fix (its side key counts our mines only), and with the opening kept off track, ended off track, capped at 3 to 6 of our turns, or kept for at least 8 or 10 turns.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-24 ("try to fix these, and try many variant
with fix lengths opening or varying lengths openings vs the current one"),
after the time per move, measured from the game records of the 1,000-game
batch ([README](../reviews/2026-10-23-v005-v004-metrics/README.md)), showed v005
answering in under 1 ms on its turns 2 to 4 (the turn-1 plan replayed) and
in about 30 ms on 28 to 48% of its turns 5 to 7.

## What v005 does

The opening (`ice_and_fire_opening::planned`, `Planner::Solo`, E010) plans
our next 7 turns on turn 1, as if the opponent passed. At the start of each
of our turns it compares a key of our side (our cells, our units, the mines
and our gold) with the key the plan predicted for that turn. If they differ,
or the plan has run out, it makes a new 7-turn plan, an anytime beam that
runs to its deadline (30.4 ms of the 40 ms budget). The opening ends only at
the middle game, when we can pay a chain of trainings to an enemy cell (our
8th turn in the median game).

Two flaws:

1. **Enemy mines.** The key held every mine on the map, so an enemy mine
   built anywhere made the plan look off track.
2. **Only this turn is checked.** Any difference on this turn throws the
   whole plan away, whether or not it matters to the rest of the plan; and
   the new plan still assumes a passing opponent, now that the sides touch.

## The variants

All but v005 hold the mine fix: the key counts our mines only.

| Variant | Off track | Opening ends |
| --- | --- | --- |
| fix | plan again (as v005) | at the middle game |
| keep | keep playing the plan's steps; plan again only when it runs out | at the middle game |
| contact | end the opening | at the middle game, or the first turn off track |
| cap3 to cap6 | plan again (as v005) | at the middle game, or after our turn N |

A step of a plan is a weight set and a buying rule, played by rules on the
board, so a kept plan still answers the board turn by turn.

Round two, after round one showed shorter openings losing: the opening plays
at least N of our turns (min8, min10) before the middle game may end it,
plans again off track as v005 does.

## Screening

Against `ice-and-fire-v005` at CodinGame's limits, 400 seat-swapped pairs
each, seed 13013, two games at a time.

| Variant | Elo against v005 | Score as red | Score as blue | Variant's timeouts |
| --- | --- | ---: | ---: | ---: |
| fix | −10.4 [−27.8, +6.9] | 37.2% | 59.8% | 0 |
| keep | −10.4 [−30.8, +9.8] | 34.5% | 62.5% | 2 |
| contact | **−62.3 [−83.3, −41.8]** | 26.5% | 55.8% | 0 |
| cap3 | **−299.6 [−334.2, −269.3]** | 14.5% | 15.8% | 0 |
| cap4 | **−199.1 [−227.1, −173.1]** | 16.2% | 32.0% | 1 |
| cap5 | **−86.9 [−109.8, −64.7]** | 23.0% | 52.5% | 0 |
| cap6 | **−52.5 [−74.4, −31.0]** | 28.2% | 56.8% | 0 |
| min8 | **−92.5 [−116.2, −69.5]** | 33.0% | 41.0% | 1 |
| min10 | **−459.6 [−516.7, −415.2]** | 8.2% | 5.0% | 1 |

min12 and a never-ending opening were stopped after min10, which they could
only lose more than.

## What was learned

- **v005 ends its opening at the right time.** Ending it earlier (after a
  fixed turn, or on the first turn off the plan) loses from 52 to 300 Elo,
  the more the earlier; keeping it past the middle game loses 92 Elo at 8
  turns and 460 at 10. The middle-game signal (we can pay a chain of
  trainings to an enemy cell) marks when the search should take over: until
  then the solo planner plays the early contact better than the search; from
  then on, it ignores attacks that it can no longer afford to ignore.
- **The replans neither help nor hurt measurably.** The mine fix cuts the
  turns 5 to 7 replans (answers of 20 ms or more: 12 of 60 turns against
  v005's 21 of 60 in the same 10 pairs, a check run before the screening)
  and keeping the plan without replanning removes them (0 of 60); both score
  −10 Elo, within noise of v005. The 30 ms they cost buys nothing visible,
  and saving it buys nothing either.
- The time per move that started this (under 1 ms on turns 2 to 4, about
  30 ms on a third of turns 5 to 7, about 11 ms later) is not a lever: v005's
  turns are fast because its opening is planned, and spending the saved time
  elsewhere has no use before the middle game.

The variants' code was not kept: it is in this pull request's first commit
(the side key with our mines only, `OffTrack` and the minimum and maximum
opening turns in `ice_and_fire_opening::planned`, with their tests), removed
by the next.
