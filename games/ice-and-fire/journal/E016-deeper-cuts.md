---
id: E016
date: 2026-10-24
pull_request: "#207"
parent: ice-and-fire-v007
hypothesis: v007's middle-game search ends at about 8 ms in most turns because no first action at the root improves its plans at the current cut length; looking then for longer cuts, ours and the opponent's, and thinking up to 45 ms a turn beats v007.
change: None released. Local screening only: v007 whose search, when a climb from the root finds nothing new, raises the cut length by one (to 8 or 10 trainings) while 8 ms remain before the soft limit, adds the opponent's longer cuts against the plans found to the memory and climbs again (up to 128 climbs); and a budget of 45 ms a turn with no new climb after 40 (the opening keeps its deadline).
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-24, after a measure of v007's time per turn
(in self-play with the real limits, 75% of its middle-game turns ended
before the time limit, at 4.3 ms in the median, because no first action at
the root improved the plan any more): "This is totally wrong, in this case
you should look for longer cuts for you or the opponent right? Try to use
45ms per turn (except for planned turns obviously)".

## The change

The cut length was 5 trainings since E014, for our macro actions and the
opponent's replies. When a climb found nothing new and its plan was empty,
the search used to stop; here it raises the cut length by one, clears the
first actions it had banned, adds to the memory the opponent's longer cuts
against each plan found, and climbs again. A first version deepened even
near the deadline: a cut search cannot be interrupted, and one turn took
51.5 ms in self-play. Deepening only while 8 ms remain before the soft
limit brought the slowest turn to 41.5 ms (to 8 trainings) and 44.6 ms (to
10).

| Variant | Longest cuts | No new climb after / answer by |
| --- | --- | --- |
| deep45 | 8 trainings | 40 / 45 ms |
| deep45x10 | 10 trainings | 40 / 45 ms |
| deep | 8 trainings | 24 / 40 ms (v007's) |
| t45 | 5 (v007's) | 40 / 45 ms |

## Screening

Against `ice-and-fire-v007` at CodinGame's limits, 400 seat-swapped pairs
each, seed 16016, two games at a time.

| Variant | Elo against v007 | Score as red | Score as blue | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | ---: | ---: | --- |
| deep45 | −8.7 [−27.5, +10.1] | 33.2% | 64.2% | 2 | 18.4, 42.1, 45.0 ms |
| deep45x10 | +4.3 [−13.9, +22.6] | 34.8% | 66.5% | 1 | 27.3, 42.1, 44.1 ms |
| deep | 0.0 [−17.7, +17.7] | 34.0% | 66.0% | 2 | 17.3, 32.2, 34.8 ms |
| t45 | +0.9 [−13.7, +15.4] | 34.8% | 65.5% | 0 | 6.7, 41.5, 42.3 ms |

v007 answered in 6.5 to 6.8 ms in the median in these games.

## What was learned

- The variants use the time (median answers of 17 to 27 ms against 7, and
  up to 45 ms), and none wins more: the plans that longer cuts and more
  time find are no better by the evaluation that chooses among them. The
  limit is now what the search is told to want, not how long it looks.
- Raising the time limits alone (t45) changes nothing, as E014's soft30
  and t35 showed for v005.
- Timeouts stay at the level of v007's (0 to 2 in 800 games) with the 8 ms
  guard; without it a deepened turn overran 50 ms.

The owner chose next to tune the evaluation's weights on a large batch of
positions ("Drop E016, then tune eval (Recommended)").

The variants' code was not kept: it is in this pull request's first commit
(`max_cut`, the deepening in `think`, the 45 ms budget and
`Limit.opening`), removed by the next.
