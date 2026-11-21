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
  results and lengths); the bundle is 88.2k characters (v031's 59.2k
  plus E077's switched-off strategy and the opening's new code).

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
