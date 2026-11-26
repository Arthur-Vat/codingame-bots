# The turn-1 planner's prototype (E080)

Scratch Python scripts, kept as the reference for the planner's Rust
version in `games/ice-and-fire/opening/src/frontline/` ([E080](../../../journal/E080-opening-roles.md),
"The turn-1 planner: the problem"). They read and write JSON in a working
folder and ran on six maps of our CodinGame games as red (boards from
`ice-and-fire-review codingame-sectors --boards ... --at 1,10`); they are
not part of the build or the checks.

- `fronts.py`: the fronts of a map at game turn 1 (the contested line, cells
  within one ring of even between the HQs, cut by place into bottom-left /
  middle / top-right; a front from 4 valid contested cells), the region
  each decides (nearest front), its value (the cells that change sides
  between our holding its line and the enemy holding it).
- `cells.py`: each neutral cell's static value (we hold the shortest chain
  from our land to it, the chain blocking the enemy; the Voronoi share
  gained), and each front's deep option.
- `entrances.py`: each front's entrances on our side: a minimum vertex cut
  (max-flow) within its region, between its cells 3+ steps nearer the
  enemy and its backfield, only neutral cells we reach no later than the
  enemy (red a step later) being cuttable.
- `planner.py`: the planner, the enemy as model (1) (each cell taken at the
  earliest turn a rush with all its gold reaches it); a plan is an ordered
  list of targets (entrances, deep cells) and a sharing of the trainings
  (all to the first unfinished target, or spread); every plan played for our
  turns 1 to 9 with simple rules; kept: the most land plus Voronoi share at
  game turn 10 against the enemy's reach. The best plan and plain growth are
  played again with every action logged, for the page's step-by-step
  player.
- `planner2.py`: the second version (the owner, 2026-11-26), without the
  enemy, with two levels of decision: macro (which area each of the first
  6 trainings serves: every sequence tried) and micro (each unit walks the
  shortest path to its area's current target, the most central path among
  equals). An area's entrance is where our shortest path from the HQ to its
  front centre enters it for good (the centre itself when the whole path is
  inside); the front centre, the front's contested cell nearest its middle
  (the most central for the middle). Objective: the most cells at game turn
  10, then the Voronoi share against the enemy's starting land.
- `planner3.py`: the third version (the owner, 2026-11-26), still without
  the enemy, reading `plans2.json`. Each area has three sub-goals: its
  entrance (E), its front centre (C) and an outpost (O), two steps past the
  centre toward the enemy's HQ. Macro: for each of the first 4 trainings, a
  sub-goal, two front centres at once (the unit steps nearer both while a
  step does, then keeps the first at the fork), or free growth; a unit that
  reaches its sub-goal goes on to its area's next one. Micro: a new cell
  first, then the cell with the most valid neighbours. Objective: land plus
  each unit's room (the free cells nearer to it than to our other units, up
  to 3), then the Voronoi share, then the fewest cells along the void. It
  also counts the distinct endings (land and unit cells).
- `planner4.py`: the fourth version (the owner, 2026-11-26), reading
  `plans3.json`, run on four processes. The same macro plans in two modes,
  independent (version 3) and coordinated (a training opens its sub-goals;
  every turn they are matched to our units, the nearest pair first; the
  other units grow into their own share of the free cells, each target
  reserved), each played 3 turns past game turn 10 with free growth, and
  five calibrations of the score (land, land + room, tolerance 1 and 2,
  later land).
- `planner5.py`: the fifth version (the owner, 2026-11-26): past game turn
  10 we grow behind our front line. The enemy at turn 10 is a stand-in, our
  free growth mirrored through the map's centre; our territory, the free
  cells nearer our land than it; from turn 11 units and trainings take only
  cells of our territory, and a unit's room is its share of it.

The pictures: fronts and goals https://claude.ai/artifact/CjVo3RiA1NCXV7eczk8vim,
cell values and entrances https://claude.ai/artifact/1hsAEqAvnM83ZDnRKACE6y,
the plans https://claude.ai/artifact/45xi3imE6WxsU2h4zUEDGP.
