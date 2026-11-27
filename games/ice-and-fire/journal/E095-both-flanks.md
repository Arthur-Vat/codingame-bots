---
id: E095
date: 2026-11-27
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Holding both flanks early, as the top players do and v032 rarely does, makes our land wider and claims the flank we now leave to the enemy, and beats ice-and-fire-v032.
change: With `eval::FLANK`, `FLANK_CAP` and `FLANK_UNTIL`, each active cell of a side on its weaker flank (the cells 4 or more columns off the diagonal between the HQs, on the side where it holds fewer), up to the cap, counts for it, net of the enemy's, until that game turn; and, as a second lever, the opening's plans ordered by E073's `Frontier { cap: 8 }`; v032 has neither.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner's notes on ten short CodinGame losses (2026-11-27,
https://claude.ai/artifact/F9jwgANcMvoB6m5HfMT7xQ) said in seven of them
that we leave large parts of the map, the left or the bottom left, to the
opponent. The growth measure (`review/examples/growth.rs`, 3,000 CodinGame
games: v032's 2,350, and the ranking games of v026 to v032; each side
compared with the opponents in the same seat) confirmed it in this form:

- We hold as much land as the top players to game turn 10 (26.7 cells
  against 26.3 at game turn 8, seat 0) and reach the line even between
  the HQs a turn earlier, but on a narrower front (7.3 columns across the
  diagonal against 8.2 at game turn 8).
- Our Voronoi share is 0.50 against 0.54, and the gap is all on the
  flanks: the middle 0.51 against 0.51, our weaker flank 0.19 against
  0.30, the stronger 0.78 against 0.84.
- Our flank land is as large (14 to 15% of our cells at game turn 8
  against 15 to 19%), but on one flank: both flanks held by game turn 8 in
  12 to 14% of our games against 33 to 41% of the top players' (zasmu
  38%), by game turn 10 in 41 to 45% against 61 to 68%.
- Flank cells are taken by units walking there (trainings take about 0.1
  a turn for everyone). zasmu steps sideways from game turn 2, has the
  widest front at game turn 6 (7.3 columns against 6.6 for the other top
  players and 5.4 for us, seat 1) and fewer units at game turn 8.
- Holding both flanks by game turn 10 goes with winning for every group
  (us 68% against 63%, the other top players 32% against 22%, zasmu 94%
  against 86%; not controlled for the map).

E073 saw the same of v029's opening ("mostly reaches one flank") and duelled
orders closer to the whole frontier (`Frontier { cap }`) in games cut at
game turn 12, where every land-first order scored alike; it was never
played in whole games, which judge (E077).

## Change

`eval::flank`: the flanks are the cells `FLANK_OFF` (4) or more columns
off the diagonal between the HQs, toward (11, 0) and toward (0, 11); a
side's weaker flank is the one where it holds fewer active cells, and those
cells, at most `FLANK_CAP`, count `FLANK` each for it, net of the enemy's,
until game turn `FLANK_UNTIL`. The opening plays to game turn 6, so the
term steers the search from game turn 7, when our gap opens.

The second lever is the opening's order, `main.rs`'s `ORDER`:
`Frontier { cap: 8 }` (E073: the most land, then the least total distance
from our land to the frontier's cells), in place of `Central(Linear(7))`.

## Screening

Pending.
