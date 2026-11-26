---
id: E080
date: 2026-11-21
pull_request: "#NN"
parent: ice-and-fire-v031
hypothesis: An opening that may also play a roles turn (units walk to targets, trainings at the tips of our land after the moves), with targets chosen so that our side holds more land than the enemy after the early game, takes the centre before game turn 7, where the race is mostly decided, and beats ice-and-fire-v031 (Step 7 of the frontline plan).
change: The opening's beam may play, beside its 27 weight sets, the roles step (`ice_and_fire_opening::frontline::step`), by its targets (`Value`, `Matched`, `Frontier` or `Marginal`), the rest of the turn by the `econ` rules; and it may rank its plans by the most cells then the Voronoi share (`Order::Voronoi`); the bot's switch `OPENING_ROLES`.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E077's timing ([E077](E077-strategic-turn.md), "Timing"): the value of the
objectives we can still reach first falls by two thirds between game
turns 1 and 6, so the race for the centre is mostly decided in the
opening. The owner (forms, 2026-11-21) chose to park E077 and do the early
game first: "Roles in the opening. Also make sure to target relevant cells
(in the first screenshot we see that swapping objectives could have been
better), also spawn from the tip. Make multiple variants. The choice of
target is not obvious at all, the final goal is to be able to have more
territory (maybe max number of cells, then voronoi share) than the
opponent after the early game."

## Change

- The objectives, the race and the roles of Steps 2 and 3 move from the
  plan bot to the opening crate (`ice_and_fire_opening::frontline`), with
  the map's band and middle chokes (`frontline::goal_zones`, as the bot's
  `eval::goal_zones`).
- **The roles step** (`frontline::step`): each turn, our movable units and
  the trainings the gold pays are agents; they take targets; units step one
  cell nearer their target (a cell not ours first); then each training goes
  on our trainable cell nearest its target, after the moves, so at the tip
  of the land a unit has just taken; then, with `fill`, the `econ` turn of
  the first weight set with the payback rule plays the rest (units with no
  target, gold left). An agent takes a target only where it arrives no
  later than the enemy's land. The targets:
  - `Value`: the objectives by the land each gains, given out greedily by
    the roles (value, then arrival);
  - `Matched`: the same objectives, the most valuable first, matched to the
    agents by the Hungarian method for the least total arrival, so that two
    units do not cross to each other's target (the owner's first board);
  - `Frontier`: E072's targets (middle chokes, then spaced cells along the
    Voronoi frontier), matched the same way;
  - `Marginal`: objectives picked one at a time by the land each adds to
    that of the ones already picked (their values overlap), then matched.
- **In the opening's beam** (`solo::steps`): with `Objective::roles`, the
  roles step is one more step beside the weight sets, so it plays only
  when the beam rates its plan best (the opponent passing, as for every
  step). `Order::Voronoi` (the most active cells, then the Voronoi share)
  is the owner's ranking; E073's `Order::Central` is v031's.
- The bot: `OPENING_ROLES` (`None` in v031). With it off the bot plays as
  ice-and-fire-v031 (20 games against v030 at fixed tries, the same
  results and lengths); the bundle is 70.9k characters (88.3k bytes; v031's
  59.2k characters, plus E077's switched-off strategy and the opening's new
  code; "88.2k characters" in an earlier version of this entry was the byte
  count).

## The bench

`oracle --bench` on the 78 stored starts
([research/codingame/2026-11-20-optimal-early-plans](../research/codingame/2026-11-20-optimal-early-plans/)):
cells short of the most a plan can hold, the opponent passing, by the
number of our turns played (lower is better; `lin7` is v031's opening,
`voronoi` the owner's order alone):

| Turns | Seat | lin7 | voronoi | lin7 + Value | + Matched | + Frontier | + Marginal | + Matched, no fill | voronoi + Value | + Matched | + Frontier | + Marginal |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | red | 0.09 | 0.11 | 0.09 | 0.09 | 0.14 | 0.09 | 0.30 | 0.09 | 0.09 | 0.16 | 0.11 |
| 6 | blue | 0.09 | 0.09 | 0.03 | 0.06 | 0.21 | 0.15 | 0.71 | 0.06 | 0.06 | 0.06 | 0.12 |
| 7 | red | 0.25 | 0.25 | 0.25 | 0.25 | 0.34 | 0.25 | 0.73 | 0.25 | 0.25 | 0.41 | 0.25 |
| 7 | blue | 0.18 | 0.18 | 0.12 | 0.18 | 0.35 | 0.29 | 0.97 | 0.18 | 0.18 | 0.26 | 0.26 |
| 8 | red | 0.27 | 0.25 | 0.30 | 0.30 | 0.41 | 0.30 | 0.98 | 0.30 | 0.32 | 0.50 | 0.27 |
| 8 | blue | 0.18 | 0.12 | 0.26 | 0.32 | 0.41 | 0.44 | 1.29 | 0.18 | 0.21 | 0.35 | 0.38 |

With the `econ` fill, no target choice costs more than about 0.25 cells
against the opponent passing; without it the roles step loses 0.3 to 1.3
cells (units with no target wait, gold stays). The bench cannot show what
the roles step is for, the race against a moving enemy: that is for the
cut duels and full games.

## A local look, before any screening on GitHub

The owner asked (form, 2026-11-21) for more metrics and visual examples
before the screening. Duels cut at game turn 12 (`ice-and-fire-arena-early`,
the bot's own switches, 100 pairs each against v031, seed 2026112105, time
scale 1, tolerance 20 ms); the control `base` is the bot with the switches
off (it plays as v031). The variants: v031's order (`Central`) with the
roles step by its targets (`rv` Value, `rm` Matched, `rf` Frontier, `rg`
Marginal); the owner's order alone (`vor`: the most cells, then the Voronoi
share) and with the roles step (`vorrm`, `vorrg`, `vorrv`); all with the
`econ` fill.

| Variant | Won (of 200) | Opening turns playing the roles step | Voronoi share, ours − v031's, at game turn 8 / 10 / 12 | Units lost by game turn 12, ours / v031's | Objectives held at game turn 12 |
| --- | ---: | ---: | --- | --- | ---: |
| base | 96 | 0 of 1,199 | −4.0 / −4.1 / −3.4 | 1.12 / 0.92 | 6.20 |
| rv | 108 | 455 of 1,198 | −3.3 / −2.5 / −3.6 | 1.16 / 0.88 | 6.20 |
| rm | 115 | 467 of 1,200 | −2.8 / −1.8 / −0.8 | 1.06 / 0.92 | 6.20 |
| rf | 102 | 460 of 1,199 | −3.7 / −3.7 / −2.5 | 1.21 / 1.05 | 6.26 |
| rg | 93 | 482 of 1,200 | −3.2 / −3.3 / −2.6 | 1.28 / 0.97 | 6.43 |
| vor | 110 | 0 of 1,200 | −2.3 / −1.0 / −0.4 | 1.08 / 0.83 | 6.55 |
| vorrm | 98 | 481 of 1,200 | −3.0 / −3.6 / −1.4 | 1.07 / 1.02 | 6.42 |
| vorrg | 104 | (not counted) | −2.6 / −2.3 / −1.3 | 1.22 / 0.84 | 6.46 |
| vorrv | 102 | (not counted) | −3.4 / −3.5 / −2.6 | 1.23 / 1.09 | 6.05 |

(Measured at the start of each of our plies from the review tool's metrics:
the enemy has just played, so even the control trails v031 by about 2
cells and 4 Voronoi cells; compare the variants with `base`. Objectives
held: those of value above 0 for us at that moment, from the bot's
`strategy` mode. 100 pairs give about ±7% on the share won.)

- The roles step is chosen at about 4 opening turns in 10, yet our cells
  at game turns 4 and 6 are the same in every variant (7.5 and 13.8): the
  roles turns and the weight sets reach the same land that early, as the
  bench found. The variants differ from game turn 8 on, in the Voronoi
  share: `rm` and `vor` close most of the control's gap by game turn 12.
- The owner's order alone (`vor`) does about as well as the best roles
  variant, and the two together (`vorrm`) do not add up.
- Matched targets (`rm`) beat the same objectives given out greedily
  (`rv`), as the owner's board suggested; the marginal and frontier targets
  do no better than the control.

Boards of three maps that the control lost and `rm` won, played by both
from the same seat, turn by turn:
https://claude.ai/artifact/P26EXuMo7PiCEu9C2Kp2oj.

The owner noticed trainings in the back line on the boards (2026-11-21).
Of about 1,200 trainings a variant in game turns 1 to 8, none is on a cell
we own and none in turns 1 to 6 is behind our front: the back-line ones
come at game turns 7 (88) and 8 (22), from the middle-game search, which
plays from game turn 7 (E067). As red at game turn 7 it trains at (0, 1),
next to our HQ, in 80 of 100 games of the control (v031) and 82 of `rm`'s.
Not E080's change. The diagnosis (the owner asked for it and for
solutions, 2026-11-21), with a new analysis mode of the bot, `explain`
(each training of the search's plan against the same plan without it,
term by term of the evaluation): on the 800 positions of game turns 7 and
8 of the control's games, the search's plans hold 764 level-1 trainings;
266 of them (35%) are 6 or more steps from enemy land ((0, 1) as red 160
times, (10, 11) and (11, 9) as blue 84 times). Each gains +31.8 on
average, of which the shape term (`eval::shape`, E010: twice the side's
joints plus the most cells one capture cuts off) gives +39.2, the rest
−7.4; 258 of the 266 would not pay without it. Our land joins the HQ
through one cell, which then cuts everything off; a unit beside the HQ
closes a loop. The 498 trainings nearer the enemy gain +4.0 (+16.2 after
the opponent's answer), from the cells and the band's goal. A fix is its
own experiment (E081).

## Who takes the flanks on CodinGame

The owner, 2026-11-21, after v032 (E081): "This bot is much much better,
however i often notice see the correct goals are not targeted. Could it be
possible to identify goals from the start of the game and assign them
afterwards?" (a game against zasmu where we leave the bottom-left corner).
He approved a measurement first. A new review command,
`ice-and-fire-review codingame-sectors`, cuts each map into three lanes
across the line between the HQs by a cell's `x − y` (the bottom-left flank
at −4 and below, the centre, the top-right flank at 4 and above) and writes,
per game, side, game turn and lane, the cells held, the lane's cells nearer
the side's land than the enemy's (its Voronoi share there), and the first
turn the side held a cell of the lane's middle (`--boards` writes chosen
games' boards as JSON).

Our games as red of v029 to v031 against the top 6 of the leaderboard of
2026-11-21 (zasmu, Pduhard-, karliso, _iaz, reCurse; 489 games from the
collection, 76 as blue left aside): a lane's split at game turn 1 is the
map's and the seat's; what our play changes is the difference by game
turn 10.

| Lane | Voronoi at game turn 1 | Change by game turn 10 | Games losing 5+ cells after turn 1 |
| --- | ---: | ---: | ---: |
| Bottom-left flank | −3.3 | −4.4 | 240 (49%) |
| Centre | −3.6 | −2.7 | 154 (31%) |
| Top-right flank | −1.3 | −2.9 | 199 (41%) |

| Flanks lost after game turn 1 (5+ cells) | Games | Won | All lanes' change |
| --- | ---: | ---: | ---: |
| Neither | 130 | 53% | −0.7 |
| Top-right only | 119 | 49% | −8.3 |
| Bottom-left only | 160 | 38% | −12.3 |
| Both | 80 | 35% | −22.3 |

The opponent first holds a cell of the bottom-left lane's middle in 298
games (us 122, the same turn 68), of the top-right's in 246 (us 147, 95).
Against zasmu alone (98 games) the centre loses most (−3.9 after turn 1),
then the top-right (−2.7) and the bottom-left (−1.4). Boards of four games
and the tables: https://claude.ai/artifact/P7UuovV76ZbNy7t5X7zYax.

So the flanks are lost after the start in about half the games, the
bottom-left more often and at a higher cost (38% won when it alone is
lost); this is what goals fixed at game turn 1 should prevent.

### Per seat, and a bias found

The owner asked for the same per seat (2026-11-21), suspecting a bias.
Lanes lost by 5+ Voronoi cells after game turn 1, against the top 6:

| Lane | As red (489 games) | As blue (76 games) |
| --- | ---: | ---: |
| Bottom-left flank | 49% | 20% |
| Centre | 31% | 42% |
| Top-right flank | 41% | 43% |

The map is symmetric by a half turn, which swaps the corners between the
seats: red's bottom-left (straight down its HQ's column) matches blue's
top-right (straight up its column). In both seats the flank along our
HQ's column is lost more often (49% and 43%) than the one along its row
(41% and 20%); as red, losing it costs most (won 37% when lost, 51%
otherwise; the row flank 43% and 45%).

A direct check (a scratch example, not committed): on 100 referee maps and
on each map mirrored across the HQs' diagonal (x and y swapped: the same
game), both sides playing v032's opening untimed for 8 turns, red's active
cells in the column flank were 387 and in the row flank 532, summed over
the maps and their mirrors, and 82 of the 100 mirrors were not played as
the mirror of the map. The cause found: the `econ` rules' choice of the
best cell for a unit or a training breaks ties by the lowest cell index
(`econ::Turn::best`), so from (0, 0) the row's (1, 0) beats the column's
(0, 1) turn after turn. Breaking ties instead toward the side of the HQs'
diagonal where we hold fewer cells (then the index) gives 473 against 446
on the same check, the same 919 cells in all; 80 mirrors still differ
(other ties remain). This is a change of behaviour for its own experiment.

### The tie-break, folded in

The owner chose (form, 2026-11-21) to fold the tie-break fix into E080
rather than run it alone: `econ::BALANCED_TIES` (off in v032), with the
mirror check `opening/examples/mirror.rs` (cherry-picked from the branch
`claude/ice-and-fire-e082-balanced-ties`, whose journal entry was never
merged). With it on, the mirror check gives 476 against 452 cells (v032:
387 against 532), the same land in all. Local duels against v032, 150
pairs each, seed 2026112109: full games +13.9 [−20.0, +48.1], cut at game
turn 12 +25.5 [−6.0, +57.5] (the switched-off control −2.3 and 0.0); no
fault, the same time per turn. E080's variants are screened with it on and
off.

## The design agreed in the conversation (2026-11-21)

From the owner's messages after the flank measurement, the next variant
of E080, the goals of game turn 1:

1. **Goals fixed at game turn 1 and assigned afterwards** (the owner: "Could
   it be possible to identify goals from the start of the game and assign
   them afterwards?").
2. **Fronts from the map, not fixed lanes** (the owner: "On some maps we do
   not have a contested middle, we only have two flanks"; "Also some maps
   don't have contested flanks, only middle"): the cells of the contested
   line (within one ring of the cells even between the HQs) grouped into
   connected stretches, the void cutting them apart; one, two or three
   fronts, by the map.
3. **Prioritized per map** (the owner: "the goals should be prioritized
   carefully depending on the map"): each front valued by the land it
   decides (the cells that change sides when we hold it rather than the
   enemy), raced from game turn 1 against the enemy's units and gold; a
   front the map gives to one side is left out; the winnable ones ranked by
   land per turn of travel, wide or valuable fronts getting more agents.
4. **Kept between turns**, changed only when a front is secured, lost, or
   the race changes.
5. **Measured per seat** (the owner: "could you do the same per seat? I
   assume there is a bias otherwise"), the flanks read from our HQ (the
   column flank and the row flank).
6. With the earlier choices: trainings spawned from the tip, targets
   matched so that units do not cross, the balanced tie-break (E082
   folded in); the goal, more territory than the opponent after the early
   game (the most cells, then the Voronoi share).

First step: the same measurement as above on the map's own fronts, per
seat and per kind of map (one, two or three fronts).

### By the map's own fronts

`codingame-sectors --fronts`: the contested line (within one ring of the
cells even between the HQs) cut by its place into the bottom-left flank,
the middle and the top-right flank; a place is a front when it has at
least 4 valid contested cells, and every cell of the map goes to its
nearest front (a tie to the front in its own place, then the middle). The
same games (v029 to v031 against the top 6; flanks read from our HQ):
442 games on maps with three fronts, 96 with only a middle, 27 with only
the two flanks. A front's figures: its region's Voronoi cells, ours less
the enemy's, at game turn 1 and their change by game turn 10.

| Seat, map | Front | Games | Region | Turn 1 | Change by turn 10 | Lost 5+ after turn 1 | Won if lost / not | First to its line: us / same / them |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| red, three fronts | column flank | 383 | 23 | −3.1 | −4.7 | 55% | 39% / 49% | 106 / 36 / 240 |
| | middle | | 63 | −3.8 | −2.8 | 34% | 33% / 49% | 27 / 236 / 119 |
| | row flank | | 23 | −2.2 | −3.2 | 46% | 45% / 42% | 129 / 48 / 205 |
| red, two flanks | column flank | 26 | 59 | −1.8 | −10.5 | 88% | 13% / 100% | 4 / 1 / 21 |
| | row flank | | 35 | −6.4 | −2.3 | 62% | 12% / 40% | 5 / 15 / 6 |
| red, middle only | middle | 80 | 89 | −3.9 | −5.1 | 39% | 39% / 65% | 1 / 59 / 20 |
| blue, three fronts | column flank | 59 | 25 | +4.2 | −8.4 | 54% | 47% / 37% | 13 / 7 / 38 |
| | middle | | 63 | +3.7 | −2.3 | 42% | 32% / 50% | 7 / 41 / 10 |
| | row flank | | 25 | +2.2 | +2.6 | 25% | 27% / 48% | 24 / 10 / 24 |
| blue, middle only | middle | 16 | 92 | +4.2 | −3.8 | 31% | 40% / 73% | 2 / 13 / 1 |

(Red's games won: 43% on three-front maps, 23% on two-flank maps, 55% on
middle-only maps; blue's 42% and 62%.)

- On every kind of map and in both seats, the column flank is lost after
  turn 1 most often (55% and 54% on three-front maps; 88% of red's
  two-flank maps), and the enemy reaches its line first about twice as
  often as we do: the opening's tie-break (above) is one cause.
- On two-flank maps (no middle), red wins 23% of 26 games: it loses the
  column flank in 23 of them (won 13% then; the 3 others were all won).
- On middle-only maps, holding the middle is worth most: won 65% when it
  is not lost, 39% when it is.

### The goals drawn on six maps

The owner asked to see the goals on various maps (2026-11-21). A first
version of the rule, in a scratch script for the pictures, on six maps of
our red games (two with three fronts, two with the two flanks only, two
with a middle only): each front valued by the cells that change sides
between our holding its line and the enemy holding it, ranked by value,
beside our board at game turn 10:
https://claude.ai/artifact/CjVo3RiA1NCXV7eczk8vim.

A finding for the design: at game turn 1 every front's nearest cell is 10
steps from red's land and 9 from blue's, on every map, since the contested
line is equidistant from the HQs by definition (blue starts with two
cells). The race cannot rank fronts at turn 1; turn-1 goals come from the
fronts' values (which fronts, how many agents each), and the race is
followed from about game turn 3, as the enemy commits its units (a front
it will reach first dropped, the others reinforced).

### Cell values and the two kinds of option

The owner (2026-11-21): within a region, cells differ (edge cells are
weaker); two kinds of option, "reaching a cell at the entrance of a region
of interest (to defend this area), or rush deep on such a region (attack)
to get a strong position and take a lead", compared by territory share;
later a dynamic opening based on the opponent's replies ("if an enemy
doesn't claim an area we could go deeper"), but first "full planning from
turn one", with the cell values drawn in absolute terms.

The static value of a cell at game turn 1 (scratch script, the six maps
above): we hold the shortest chain of cells from our land to it, the chain
blocking the enemy, and count the map's cells nearer our land than the
enemy's, less the count as it is. Per front, the entrance option (the best
cell of its contested line) and the deep option (the best cell one or two
rings past the line, 4 to 8 steps nearer the enemy than us):
https://claude.ai/artifact/1hsAEqAvnM83ZDnRKACE6y. On these maps the
entrance gains 16 to 50 cells in 10 to 12 steps; going 2 steps deeper adds
0 to 8 cells. With an enemy that does not move, the value only grows
toward the enemy's HQ (a chain to it blocks everything), and edge cells are
worth less (a chain along the border decides little land); the planning
needs the enemy's growth in time to give the deep options their real
price.

### Entrances: on our side, and often several

The owner (2026-11-21): "On one of the maps you showed I actually see four
choke points, so maybe there are several entrances there right?" and "Also
the entrance should be on our side right?". A front's entrances are now the
fewest cells on our side (neutral cells we reach no later than the enemy,
a step later as red, the seat's own step) that every enemy path into the
region's backfield (its cells 3 or more steps nearer us) must cross,
within the region: a minimum vertex cut by max-flow (scratch script). On
the six maps: one to three entrances a front, 9 to 12 steps from our land;
for example on the two-flank map 925707764, three entrances to the
bottom-left region ((1, 10), (2, 9), (3, 8)) and two to the top-right
((8, 1), (8, 3)); the middle-only maps, two each. Drawn on the cell-value
page (black diamonds): https://claude.ai/artifact/1hsAEqAvnM83ZDnRKACE6y.

Next: the turn-1 planner (scripts and pictures first): the enemy growing
in time, each front's options (skip it, hold its entrances, go deep) tried
in every combination, the plan leaving us the largest share of the map at
the end of the early game; then the same in the opening's code, then the
dynamic version following the enemy's moves.

## The turn-1 planner: the problem (agreed in the conversation, 2026-11-21)

The owner: "We have a contraint programming problem: we have a limited
number of train that can accelerate towards a goal (maybe several goals at
once during the early turns); we have a limited number of goals. We could
simply check all possibilities to grow in order to maximize the final
position (based on where the opponent could end up)"; then "Ideally some
options bring closer to two goals at once? Also I presume in an open field
the middle is absolutely critical", and "sometimes the most advanced unit
will not be able to get closer to two goals at once". The formulation:

- **The game, reduced:** the valid cells as a graph (four neighbours),
  distances `d` in steps; game turns `t = 1 … T`, the end of the early game
  (`T` about 8 to 10), red playing first in each turn. Our land `L_t`
  (joined to the HQ), units `U_t`, gold `g_t`; `g_{t+1} = g_t − 10 k_t +
  |L_{t+1}| − |U_{t+1}|` (`k_t` trainings, upkeep 1 a unit).
- **Decisions in a turn:** each unit steps to a neighbour or stays (a
  neutral cell entered joins the land); then `k_t ≤ ⌊g_t / 10⌋` trainings,
  each on a free cell next to the land after the moves (from the tip); a
  new unit cannot move that turn.
- **Goals** `Γ = {γ_1 … γ_m}`, found at game turn 1: a front's entrances on
  our side (a set of cells `C_j`, the minimum cut within its region) or a
  deep cell; each with the land `v_j` it decides. Our arrival `a_j`: the
  first turn all of `C_j` is ours; the enemy's `b_j`: the first turn it can
  take a cell of `C_j`; won when `a_j ≤ b_j` (we play first).
- **Objective, lexicographic (the owner's):** the most land at `T`, then
  the Voronoi share against where the enemy can be: maximise
  `(|L_T|, V(L_T, B_T))`; a softer form `α |L_T| + β V + Σ_j v_j [a_j ≤ b_j]`.
- **Growth as a tree:** to reach a set of goals `S`, our land grows as a
  tree from our land to every `C_j` of `S` (a Steiner tree): a shared trunk,
  then branches; two goals cost less than two separate chains, and an open
  map's middle is the natural trunk (the most land, a few steps from both
  flanks' entrances, on the enemy's way to them). A step brings a unit
  nearer to two goals only while both lie ahead of it; past a fork it serves
  one branch, so the plan needs a second agent at each fork (a unit behind,
  or a training at the fork: the leapfrog).
- **The plan:** a set of goals, hub cells where the routes split, and each
  agent (our movable units and the trainings the gold pays over the next
  turns, about 6 early) assigned a route (trunk, then one branch) with its
  timing. Every plan is simulated turn by turn with simple rules (each
  agent steps along its route, trainings at the tip toward their branch,
  free agents to the nearest neutral cells, gold spent as it comes), which
  gives `a_j`, `L_T` and the objective; a few breadth-first searches on 144
  cells each, so tens of thousands of plans fit in the first turn's time;
  plans dominated on every goal are pruned.
- **The opponent, three models from cheap to exact:** (1) its earliest
  possible arrival at each cell (its units and trainings with its gold,
  straight there): pessimistic and cheap; (2) a few plausible enemy plans
  (its natural growth, a rush to each front): our plan's worst result over
  them; (3) both sides choose a plan at turn 1: the zero-sum matrix of
  results, our plan maximising the minimum (or the mixed strategy).

The first version: model (1), then the plans drawn on the six maps, then
the planner in Rust in `opening/src/frontline/`, played by the opening.

### The planner's first prototype

[`research/codingame/2026-11-25-turn1-planner/planner.py`](../research/codingame/2026-11-25-turn1-planner/README.md) (model (1) for the
enemy; plans as ordered targets among the fronts' entrances and deep
cells, at most 4, with the trainings either all to the first unfinished
target or spread to the least advanced; our turns 1 to 9 simulated; kept,
the most land plus Voronoi share at game turn 10 against the enemy's
reach), on the six maps: 2,192 to 18,052 plans a map with flanks, a few
seconds in Python.

| Map | The best plan's targets, in order | Cells, Voronoi at turn 10 | Plain growth toward the middle line |
| --- | --- | --- | --- |
| three fronts, 925707781 | middle (5, 5), top-right (7, 2), spread | 27, 32 | 27, 28 |
| three fronts, 925707787 | middle (6, 4), bottom-left (3, 8), (2, 9), spread | 26, 31 | 24, 25 |
| two flanks, 925677145 | top-right (8, 3), bottom-left (3, 8), (1, 9), spread | 26, 33 | 27, 29 |
| two flanks, 925707764 | top-right (8, 3), bottom-left (3, 8), (1, 10), spread | 26, 28 | 26, 26 |
| middle only, 925708443 and 925709109 | none: plain growth is best | 30, 30 and 22, 29 | the same |

On every map with flanks the best plan spreads its trainings over two
fronts' entrances, the middle first on three-front maps, and gains 2 to 6
Voronoi cells over plain growth in this model; on middle-only maps no
target beats growing toward the middle line. Pictures:
https://claude.ai/artifact/45xi3imE6WxsU2h4zUEDGP. Limits: the enemy is a
rush in every direction at once (pessimistic), the rules of growth are
simple, and the plan's land is not checked against the engine; the Rust
version plays the plans through the engine.

### The planner's second prototype: no enemy, macro and micro

The owner (2026-11-26): the first version's enemy spreads far more than a
real one can, so the first version leaves the enemy out; there are two
levels of decision, "macro-decisions (when training: which area should be
targeted)" and "micro-decisions (which path should be followed by the
units)", few of the first and many of the second ("sometimes several micro
decisions will be good, however there is certainly a best, which could be
arbitrated later with other heuristics"); and the entrances were badly
placed ("in game 925707787, entrance 3 in the bottom left is not even the
first cell of the region, and is very far. Ideally entrance 1 for the
middle region should be more central, there should be an entrance for the
top right region, and entrance for the bottom left region should be above
top 2").

`planner2.py` (research/codingame/2026-11-25-turn1-planner): macro, every
sequence of areas (or free growth) for the first 6 trainings (64 to 4,096
a map); micro, each unit walks the shortest path to its area's current
target (the entrance, then the front centre, then the area's nearest cell
not ours), the most central path among equals; units with no area walk to
the nearest free cell. An area's entrance: where our shortest path from the
HQ to its front centre enters it for good, the centre itself when the
whole path lies inside (the middle on open maps). On 925707787: the
middle's centre (5, 5), the bottom-left's entrance (3, 7) above its centre
(3, 8), the top-right's entrance (7, 3).

| Map | The best macro sequence (trainings 1 to 6) | Cells, Voronoi at game turn 10 | Free growth only |
| --- | --- | --- | --- |
| three fronts, 925707781 | middle, middle, then free | 40, 103 | 36, 95 |
| three fronts, 925707787 | middle, middle, free, free, free, bottom-left | 40, 80 | 36, 80 |
| two flanks, 925677145 | free, free, top-right, free, bottom-left, top-right | 39, 82 | 35, 83 |
| two flanks, 925707764 | free, free, top-right, bottom-left, free, free | 39, 78 | 34, 71 |
| middle only, 925708443 | middle and free alternating | 31, 49 | 27, 49 |
| middle only, 925709109 | free and middle alternating | 33, 49 | 32, 49 |

The best sequences gain 1 to 5 cells over free growth and reach the
centres of one or two areas, the middle first on three-front maps and both
flanks on two-flank maps. Pictures, playable action by action:
https://claude.ai/artifact/45xi3imE6WxsU2h4zUEDGP.

## The planner's third prototype: sub-goals, two goals at once, room

The owner (2026-11-26): each area gets three sub-goals, the entrance, the
centre and an offensive cell between the centre and the opponent (static
at turn 1, later dependent on the enemy's play); the advance of the most
advanced units is part of the macro decisions, since one step can bring a
unit nearer two goals; among plans with close scores, avoid building along
void cells where the way is wide enough; and maximize the gold earned in
the long run, so that units do not end inside our land ("multiple
coordinated snakes that must not collide").

The problem, written out. At most 3 areas `a`, each with sub-goals
`E_a, C_a, O_a`. A plan is the macro choice `m_i` of each of the first `K`
trainings, in `{(a, s)} ∪ {(C_a, C_b), a ≠ b} ∪ {free}`, so at most
9 + 6 + 1 = 16 options and `16^K` plans; the micro rules then decide every
move. Score at game turn `T + 1`: `|L_T| + Σ_u min(room_u, 3)`, then the
Voronoi share, then the fewest cells of `L_T` along the void, where
`room_u` is the free cells nearer to unit `u` than to our other units.
Many plans end the same way: the distinct endings (land and unit cells)
are counted below.

`planner3.py` (research/codingame/2026-11-25-turn1-planner), `K = 4`,
`T = 9`:

| Map | Plans | Distinct endings | The best macro sequence (trainings 1 to 4) | Score (land + room), Voronoi, cells along the void | Free growth only |
| --- | --- | --- | --- | --- | --- |
| three fronts, 925707781 | 50,625 | 2,906 | bottom-left C; middle C and top-right C; middle O; middle C and top-right C | 40 + 29 = 69, 106, 5 | 63, 102, 13 |
| three fronts, 925707787 | 50,625 | 847 | top-right O; middle C and top-right C (twice); middle O | 32 + 24 = 56, 81, 5 | 53, 80, 11 |
| two flanks, 925677145 | 4,096 | 460 | top-right O; top-right E; bottom-left E; bottom-left O | 37 + 20 = 57, 83, 20 | 40, 81, 13 |
| two flanks, 925707764 | 4,096 | 448 | top-right O; top-right E; bottom-left E; free | 39 + 21 = 60, 78, 25 | 41, 71, 20 |
| middle only, 925708443 | 81 | 11 | middle E four times | 26 + 14 = 40, 48, 19 | 34, 49, 19 |
| middle only, 925709109 | 81 | 17 | free; middle E; free; middle E | 31 + 11 = 42, 49, 21 | 38, 49, 16 |

What it shows:
- The two-goal option is chosen on both three-front maps: the middle and
  top-right centres share a first stretch of path.
- Room changes the picture most on the two-flank maps (57 and 60 against
  40 and 41 for free growth): free growth bunches the units behind each
  other, while the plan sends one unit to each flank's entrance and one past
  it, so they spread out.
- The endings are far fewer than the plans (17 times fewer on 925707781,
  60 on 925707787), and on middle-only maps there are only 11 to 17: a
  search could run over endings, or prune plans that reach the same land.
- On middle-only maps the best plan holds less land than free growth (26
  against 34) for more room: the room term, capped at 3 a unit, can
  outweigh land. Its weight is a choice to settle before the Rust version.
- The micro rule away from the void cuts the cells along it on three-front
  maps (5 against 11 and 13), not on the others, where the plan goes along
  the edges to reach the flanks.

Pictures, playable action by action, with each decision's sub-goals, the
forks and each unit's room:
https://claude.ai/artifact/45xi3imE6WxsU2h4zUEDGP.

