---
id: E019
date: 2026-10-29
pull_request: "#210"
parent: ice-and-fire-v007
hypothesis: The first large cut decides most games, and most large cuts use moves as well as trainings, which v007's replies (trainings-only cuts, the lethal chain, bronze's greedy turn) do not model; with the opponent's own plan among its replies, the search keeps plans that hold against such cuts and beats v007.
change: The opponent's replies to a plan also hold its own plan, the same climb run from its seat on the state after our turn (moves, trainings, mines, towers, cuts), scored by the evaluation from its side, with an empty memory and up to 300 tries.
release: ice-and-fire-v008
sprt: accepted
elo: +78.5 [+47.2, +111.2] at CodinGame's limits
pairs: 153
full_time: +78.5 [+47.2, +111.2] (the SPRT's own, ADR 0042), no timeout in 307 games, slowest answer 614.4 ms (the first turn)
decision: promoted
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
- The first large cut (the review tool's `large cut` moment: a side loses
  at least 30% of its active cells, and at least 6 of them, in one turn)
  decides the game: its maker wins 89% of the games that have one (822 of
  the 923 games with one);
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
the entry masks indexed by level, the HQ step of `lethal` through `xy`,
the lazy and full passes in one loop, a shorter debug line): on 260
middle-game positions of 80 random games (plies 16 to 40, each searched
by the climbs, 400 tries), the compacted and the uncompacted code chose
the same commands, climbs, tries, memory and plans.
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
(+86.5 over 1,000 pairs, against +83.6 for 100), as the owner chose
("Candidate v008, cap 300 (Recommended)"). With 30 tries the opponent's
climb finds nothing the other replies miss.

The release file itself, compiled with rustc 1.90.0, against v007 on 100
pairs (seed 19021): +81.4 [+37.1, +128.4], 1 timeout of v008 in 200 games,
median answer 15.2 ms (v007 6.7 ms).

## SPRT

Accepted on the pull request (run 40843525126, at 3fcce06), at CodinGame's
limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v008 vs ice-and-fire-v007: 307 games, 153 complete pairs
  ice-and-fire-v008: 187 wins, 0 draws, 120 losses (score 60.9%)
  Elo of ice-and-fire-v008 over ice-and-fire-v007: +78.5 [+47.2, +111.2] (95%, from pairs)
  pairs by ice-and-fire-v008's points (0, 1/2, 1, 3/2, 2): [10, 0, 99, 0, 44]
  ice-and-fire-v008 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v007 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v008 19.3, 32.0, 34.3 ms; ice-and-fire-v007 7.8, 32.0, 34.2 ms
  slowest answer: ice-and-fire-v008 614.4 ms, ice-and-fire-v007 617.1 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.01 (bounds -2.94, 2.94) after 153 pairs
Verdict: ACCEPTED: ice-and-fire-v008 is stronger than ice-and-fire-v007
```

## League

The league of the release (at 3fcce06, 800 pairs a pairing) scored v008
62.3% against v007 (997 wins, 603 losses), 69.7% against v006, 73.2%
against v005 and 77.9% against v004, and rated it +2218.8 ± 182.5,
against v007's +2133.6 ([LEADERBOARD.md](../LEADERBOARD.md)).
