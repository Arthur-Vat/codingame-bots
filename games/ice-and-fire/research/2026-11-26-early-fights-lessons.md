# What E083 to E088 taught

Five experiments of 2026-11-26 on v032's early and middle game (the owner's
plan: bring the turn-1 planner's ideas into v032, then fix the fights lost
before game turn 12), about 50 variants screened against v032 (400 pairs,
full games) and several studies of v032's 1,950 CodinGame games. Their
journal entries hold the numbers: [E083](../journal/E083-race-and-hold.md),
[E084](../journal/E084-cut-threat.md), [E085](../journal/E085-secure-territory.md),
[E086](../journal/E086-racing-opening.md), [E087](../journal/E087-move-cuts.md);
the leaners are screened again in [E088](../journal/E088-leaners.md).

## What the games show

- v032 loses its fights before game turn 12 mostly to cuts, not captures:
  in its lost CodinGame games 4.7 cells a game are cut off from the HQ and
  4.6 captured (1.7 and 2.8 in its wins). In the median large loss it
  spends its 32 gold down to 5, and the enemy, with 38, trains two units
  and takes four cells taken a turn before
  (`research/codingame/2026-11-26-early-fights`, on E083's branch).
- Against zasmu (the leader; v032 won 16 or 17 of 195), v032 falls behind in the
  opening: zasmu reaches the middle front's centre at game turn 7, a turn
  before v032, and leads by 3 cells and 11 of the Voronoi share then, in the
  games v032 won as in those it lost. zasmu trains at turn 5, builds a tower
  at turn 7, and keeps more gold in the fights.
- The losses are foreseen: v032's search expects them (median surprise 0 to
  +9.5 on the plies followed by a large removal at game turns 9 to 12), and
  more search time (2,000 tries) plays the same plan in 84% of its plies.
  In zasmu's place the search plays zasmu's plan in 5% of the plies, though
  it spends as much and buys the same mix: what differs is where the units,
  trainings and towers go.
- The replies the search misses, when it misses one, are scattered cuts:
  sets of trainings each next to the side's active land as it stands,
  reconnecting its own inactive pockets and training from several places
  (20 of the 25 large surprises no cut finder matched); then cuts after a
  unit's step (11 of 60); then replies that remove none of our cells, among
  them the opponent passing and keeping its gold, which every reply
  modelled spends (6 of 60, mostly late).

## What the screenings say

- Terms that reward what one could take, rather than what one holds, make
  the search stop taking land: E083's race (-94), E085's secure territory
  (-100 to -235), and E083's goal cells unless light.
- Terms that make the search safer or more defensive lose Elo, the heavier
  the more: E083's hold (-57), E084's cut threat (-37 to -92), E085's held
  land (0 to -63), E087's second-worst reply (-85). v032's balance between
  safety and expansion (its 5/4 charge on each cell the enemy can take)
  seems near its best; removing that charge costs 60.
- Modelling the opponent's replies better (steps before cuts, scattered
  cuts, pairs of steps) halves the large removals the search misses, but
  does not make it stronger in full games, and the scattered cuts after
  each step cost timeouts on CodinGame-speed runners.
- Changing the opening (E086: ranking by the fronts' centres held, a
  simulated opponent, a supported centre) does not reach the centre sooner
  (the opening already goes as fast as its gold allows) and loses more
  middle cells than v032.
- Measures of the early game alone mislead: the second-worst charge lost
  the fewest cells in games cut at game turn 12 and was among the weakest in
  full games. Full games decide.
- Three changes leaned positive without an interval above 0: the centre
  pull (+7.8), kept gold (+11.3), steps3 (+15.6, then -4.3). E088
  screened them with three times the pairs: none beats v032 (pull4-keep2
  +10.4 [-2.0, +22.9] at best; steps3 -11.9, below 0).

## What it suggests

Hand-written terms and wider reply models have not closed the gap: the
data points at the evaluation's judgement of where to play, which zasmu's
choices contradict. Learning the evaluation's weights from those choices (a
ranking fit on the positions where zasmu and the search disagree) is the
direction the data supports, a larger project to plan with an ADR first.
Tools kept for it: `arena/examples/one_map.rs` and `plies.rs` (on E083's
branch: a CodinGame seed's map, every ply's board), `explain --moves
--played` (E086's branch: an action's gain, term by term), and the
`codingame-surprise` and `codingame-judge` runs of E084 and E087.
