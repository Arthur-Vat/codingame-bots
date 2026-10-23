---
id: E008
date: 2026-10-23
pull_request: "#192"
parent: ice-and-fire-v004
hypothesis: The first batch of game reviews found that blue almost never builds the mine next to its HQ while red always does; if the opening's tie-breaks (lowest cell index) cause it, breaking ties by the cell's index as red sees it makes blue play the mirror of red's opening and beats v004.
change: None released. Local screening only: v004 whose opening breaks every tie between cells by the cell's index as red would see it (blue's cell c is red's 143 − c).
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-23 ("Go on with this new experiment", then
"Seat-symmetric opening (Recommended)"), after the first batch of game
reviews ([reviews/2026-10-23-v004-v003](../reviews/2026-10-23-v004-v003/summary.md)).
Its summary's first idea for the next experiment was mines for the blue
seat: v004 built no mine in 8 of its 10 blue games.

## What was found

- **The pattern is real.** Counted from the batch's 20 records (its
  `records/` folder), both bots as red built a mine at (1,0), next to their
  HQ, at ply 10 in every game; as blue, they never built one at (10,11), its
  mirror (v004 3 mines in its 10 blue games, v003 14, none of them there).
- **Tie-breaks were part of it.** The opening chose the cell of lowest index
  among equals (in the best cell, the unit to move first, the idle units'
  order and targets, and the mine spot). Low indices lie next to red's HQ
  and toward the middle for blue. A test on 40 maps, a position symmetric
  under a half-turn, showed blue growing down a column where red grew along
  a row.
- **The rules are the rest of it.** Blue starts owning (11,10) as well as its
  HQ (RULES.md); red owns its HQ alone. On blue's first turn (10,10) has
  three free neighbours and the spot (10,11) two, so blue trains on
  (10,10) on value, not on a tie, and claims its spot late or never. Red's
  first training lands on its spot (1,0) by tie-break.
- **Blue loses nothing by it.** Over 1,800 earlier games between these bots
  and their variants (the local screening runs of E005 to E007 and the game
  study's 200, whose records were not kept), blue won 47% to 55% of the
  games, run by run.

## Screening

Against `ice-and-fire-v004` at CodinGame's limits, seed 161, 200 pairs, two
games at a time: 203 wins, 197 losses (50.7%), **+5.2 Elo [−27.7, +38.2]**,
no fault. Red's play is unchanged by the change; as blue the candidate won
111 of 200 games, v004 108. It built a mine as blue in 116 games (v004 119),
at ply 13 at the median.

## What was learned

- A review batch's ideas need checking against the numbers before they cost
  an experiment: "mines for blue" came from one batch's reviews, and the
  seats' results over many games say the missing mine does not matter.
- The opening's tie-breaks favour red's side of the board; it changes little
  in play.
- The code (the opening's `rank`, and the mirror test) is in this pull
  request's commit 67d3a6f, reverted in the next one.
