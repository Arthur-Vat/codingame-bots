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

The pictures: fronts and goals https://claude.ai/artifact/CjVo3RiA1NCXV7eczk8vim,
cell values and entrances https://claude.ai/artifact/1hsAEqAvnM83ZDnRKACE6y,
the plans https://claude.ai/artifact/45xi3imE6WxsU2h4zUEDGP.
