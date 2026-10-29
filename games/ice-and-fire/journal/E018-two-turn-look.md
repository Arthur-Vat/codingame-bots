---
id: E018
date: 2026-10-28
pull_request: "#209"
parent: ice-and-fire-v007
hypothesis: The plan bot scores a plan by the worst evaluation after the opponent's replies in its memory; scoring the best plans one turn deeper, by our best answer to each reply, chooses plans that hold up better and beats v007.
change: None released. Local screening only: v007 whose final choice among its plans scores the 2, 4 or 8 best (or all), best first and while time remains, by the worst over the memory's replies of the best evaluation after our answer (our lethal chain, our 3 best cuts of up to 5 trainings, bronze's greedy turn, or no answer), and plays the best of those.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-28 after E017 ("Drop E017, then 2-turn
look (Recommended)").

## The change

v007's climbs find up to 32 plans a turn and remember up to 8 replies of
the opponent (its lethal chain, its best cuts, bronze's greedy turn). The
plan played is the one whose worst evaluation after a reply is best. The
variants re-rank the best plans by that score: for each, and for each
reply, they try the same kinds of turns for us at the start of our next
turn (our lethal chain, our best cuts, bronze's greedy turn) and keep the
best evaluation after one of them, or after none; a plan's deeper score is
the worst of those over the replies, with the same early stop as v007's
scoring. The plans are scored best first until the turn's 40 ms are up.
After E015 the median turn took about 7 ms (E015's screening; 5.7 to
6.5 ms for v007 in the games below), so there was time: in a trial
game the variant scoring all plans scored 7 a turn on average and took up
to 37 ms.

## Screening

Against `ice-and-fire-v007` at CodinGame's limits, 400 seat-swapped pairs
each, seed 18018, two games at a time, and the most promising again on 600
pairs with seed 18019.

| Variant | Elo against v007 | Score as red | Score as blue | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | ---: | ---: | --- |
| top2 | −1.7 [−19.6, +16.1] | 38.0% | 61.5% | 0 | 7.3, 32.1, 35.1 ms |
| top4 | +13.9 [−4.9, +32.7] | 39.5% | 64.5% | 1 | 8.1, 32.5, 40.7 ms |
| top4, 600 pairs, seed 18019 | **−21.5 [−36.3, −6.7]** | 36.2% | 57.7% | 1 | 7.6, 32.5, 36.8 ms |
| top8 | −15.6 [−35.0, +3.6] | 35.2% | 60.2% | 3 | 8.0, 35.2, 41.4 ms |
| all | **−43.7 [−63.8, −23.8]** | 33.5% | 54.0% | 1 | 7.7, 40.3, 43.0 ms |

v007 answered in 5.7 to 6.5 ms in the median in these games, with 0 to 4
timeouts in each run.

## What was learned

- The more plans the deeper score re-ranks, the worse the bot plays: all
  plans lose clearly, and top4's first gain did not hold on a new seed.
  Over the 1,000 pairs together top4 scored about 49% (979 points of
  2,000), about −7 Elo.
- Our answers are five fixed kinds of turn, not a search of our next turn,
  so the deeper score is a noisy and optimistic judge: among many plans it
  picks the one whose replies happen to suit those answers, and that pick
  is often worse than v007's.
- With E016 (more time, longer cuts) and E017 (fitted weights) this is the
  third way of searching harder or weighing differently that does not beat
  v007. The owner chose next to study v007's lost games for what its
  evaluation misses ("Study v007's losses (Recommended)").

The variants' code was not kept: it is in commit aedba69 of this pull
request (`deep` and the re-ranking behind a build-time switch in the plan
bot's `search.rs`, the turns of either side shared with the replies),
removed by its last commit. The screened binaries were built before the
answers and the replies were merged into one function, which changes no
behavior.
