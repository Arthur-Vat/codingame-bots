---
id: E019
date: 2026-10-29
pull_request: "#210"
parent: ice-and-fire-v007
hypothesis: The first large cut decides most games, and most large cuts use moves as well as trainings, which v007's replies (trainings-only cuts, the lethal chain, bronze's greedy turn) do not model; with the opponent's own plan among its replies, the search keeps plans that hold against such cuts and beats v007.
change: The opponent's replies to a plan also hold its own plan, the same climb run from its seat on the state after our turn (moves, trainings, mines, towers, cuts), scored by the evaluation from its side, with an empty memory and up to 300 tries.
release: ice-and-fire-v008
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Chosen by the owner on 2026-10-29 after a study of v007's losses ("Study
v007's losses (Recommended)", then "Opponent's full plan (Recommended)").

## The study

The review tool's metrics of the 1,000 v007 self-play games of E017 (seed
17017; `ice-and-fire-review games` then `report`):

- v007's evaluation already predicts the winner well: on the middle game's
  26,889 positions a logistic fit of its score and the seat has a held-out
  log loss of 0.292, and none of the tool's 50 metrics, added to it, lowers
  that by more than 0.004. New terms of the evaluation promised little.
- The first large cut (a side losing 8 or more cells in one turn) decides
  the game: its maker wins 89% of the games that have one (822 of the 923
  games with one);
  blue makes it in 573 of them, red in 350.
- 87% of those first cuts, and 35% of all the cells taken in the middle
  game, go beyond what the best chains of trainings alone could take with
  the cutter's gold (the 8 best cuts of up to 5 trainings): the cutter also
  moves units, taking a median 7 cells more.
- In 93% of the cuts of 8 or more cells the side cut had grown in its turn
  before (by a median 7 cells, for a median loss of 11): it spreads into
  cells it cannot hold.

v007's memory of replies holds the opponent's lethal chain, its best
chains of trainings and bronze's greedy turn, none of which moves a unit to
cut, so its plans underrate those cuts.

## The change

After a climb, the replies added to the memory now also hold the
opponent's own plan: `Search::climb` from the opponent's seat on the state
after our turn ends, with an empty memory (its plan is scored by the
evaluation after it, from its side), no banned first action, and at most
300 tries (`REPLY_EVALS`). It is remembered like the other replies (kept
if among the 8 most damaging). To keep the bundle within 100,000
characters, a few compactions change no behaviour (a default `Thought`,
the entry masks indexed by level, the lazy and full passes in one loop, a
shorter debug line): on 71 positions of 12 random games the compacted and
the uncompacted code chose the same commands, climbs, tries and memory.
The bundle is 99,973 characters.

The plan test `takes_a_cheap_gain_the_hill_climb_must_find` had a red line
one cell wide, where a new cell scores exactly 0 (+4 for the cell, −4 for
the joint it adds), so v007 trained there only to defend against blue's
replies, and E019 defended with two towers instead. The owner chose a new
position for it ("New position, real gain (Recommended)"): red's cells
close a loop when they grow, so a training is a strict gain; v007 and
E019 both pass it, with blue at 0 to 30 gold.

## Screening

Against `ice-and-fire-v007` at CodinGame's limits, 400 seat-swapped pairs
each, seed 19019, two games at a time, the best two again on 600 pairs
with seed 19020. The cap is on the opponent's climb in each reply search.

| Variant | Elo against v007 | Score as red | Score as blue | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | ---: | ---: | --- |
| c30 | +1.7 [−18.3, +21.8] | 35.2% | 65.2% | 2 | 8.7, 32.6, 35.1 ms |
| c100 | **+94.3 [+74.2, +115.0]** | 50.2% | 76.2% | 1 | 15.6, 32.3, 35.8 ms |
| c100, 600 pairs, seed 19020 | **+76.5 [+59.8, +93.5]** | 47.2% | 74.5% | 3 | 16.5, 32.3, 34.9 ms |
| c300 | **+85.0 [+63.9, +106.8]** | 48.5% | 75.5% | 3 | 17.6, 32.2, 34.8 ms |
| c300, 600 pairs, seed 19020 | **+87.5 [+69.6, +105.9]** | 51.0% | 73.7% | 1 | 19.9, 32.4, 35.4 ms |
| no cap | **+76.8 [+56.5, +97.6]** | 48.5% | 73.2% | 2 | 17.9, 32.3, 35.1 ms |

v007 had 0 to 3 timeouts in each run and answered in 7.2 to 8.1 ms in the
median. The candidate takes the cap of 300, the steadier over the two seeds
(+86 over 1,000 pairs, against +83 for 100), as the owner chose
("Candidate v008, cap 300 (Recommended)"). With 30 tries the opponent's
climb finds nothing the other replies miss.

## SPRT

Pending.
