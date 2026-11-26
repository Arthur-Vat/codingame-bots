---
id: E083
date: 2026-11-26
pull_request: "#388"
parent: ice-and-fire-v032
hypothesis: An evaluation that counts the race to the contested cells with units (a unit at the front arrives before a chain of trainings), the map's goal cells of each front (entrance, centre, outpost) a side holds or reaches first, and units that hold the front line, makes the search push and hold the front instead of backing off, and beats ice-and-fire-v032.
change: Three terms of the search's evaluation until game turn 14, from the turn-1 planner of E080 (the owner, 2026-11-26, ideas 3, 4 and 5); each weight at 0 is v032.
release: none
sprt: not run: no variant above 0 in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

In the turn-1 planner's games against v020 (E080, "Adaptive plans
against v020"), our units backed off as soon as they lost a race, though a
unit standing on a cell blocks the enemy's level-1 units, and we were not
ready for v020's rush. The owner (2026-11-26) chose to bring the
planner's ideas into v032's evaluation, with aggressive play: the goal
cells of each front (3), a race in which units count and ties are not
losses (4), and holding the front line (5).

## The change

`bots/plan/src/front.rs`, three terms counted for a side less for the
enemy, until game turn [`FRONT_UNTIL`] (14, as the contested band of E076):

- **Race** (`RACE`, idea 4): each cell of the contested band (E073) counts
  for the side that arrives first. A side's arrival at a cell: 0 on its
  active land, 1 next to it (a training now), the steps of its nearest
  unit, and the steps from its land plus one through a chain of trainings
  (slower, and paid). Ties count for neither. It replaces the band cells
  held of E073 (`GOAL_OWNED`) in the variants that use it.
- **Goal cells** (`GOAL_E`, `GOAL_C`, `GOAL_O`, idea 3): the map's fronts,
  computed once a map as in E080's planner: the cells within a ring of
  even between the HQs, cut by place into the bottom-left, the middle and
  the top-right, a front with at least 4 of them; its centre, the
  contested cell nearest its middle (the most central for the middle
  front); its entrance, where our most central shortest path from the HQ
  to the centre enters the front's region for good; its outpost, two steps
  past the centre on the most central shortest path to the enemy's HQ. The
  enemy's are their mirrors (the maps are point-symmetric). Each front
  counts, for a side, the weight of its deepest goal cell (outpost, then
  centre, then entrance) the side holds actively or reaches first.
- **Holding the front** (`HOLD`, `IDLE_TO_FRONT`, idea 5): each unit of a
  side on a band cell it holds and the enemy cannot take at its next turn
  (E023's `exposed`) counts `HOLD`; with `IDLE_TO_FRONT`, E060's charge
  on idle units counts their steps to the front (the enemy's cells and the
  band cells not the side's) rather than to the nearest cell not theirs,
  so that units do not walk back to free cells behind the front.

## Screening

Six variants (weights: race; band cells held of E073; goal cells entrance,
centre, outpost; hold; idle toward the front):

| Variant | Commit | Race | Band held | Goal E / C / O | Hold | Idle to front |
| --- | --- | --- | --- | --- | --- | --- |
| race | 034262270b7e | 4 | 0 | 0 | 0 | no |
| goals | 99748648773a | 0 | 4 | 4 / 8 / 12 | 0 | no |
| hold | 8c015cea8695 | 0 | 4 | 0 | 4 | yes |
| all | 9b247402b4b5 | 4 | 0 | 4 / 8 / 12 | 4 | yes |
| all-light | b412b306c341 | 2 | 4 | 4 / 8 / 12 | 4 | yes |
| all-heavy | f86dde085bfc | 6 | 0 | 6 / 12 / 18 | 6 | yes |

A local check of all-heavy against v032, 10 pairs at CodinGame's limits
(seed 83): no fault, answers after the first 41.4 ms median (v032 37.3),
6 wins and 14 losses (too few games to judge).

The screening, on the owner's approval (2026-11-26): `screen.yml` run
45818893819, full games against v032, 400 pairs per variant over 2 shards.

Results (job `Summary`, 400 pairs each, full games, no fault on either
side):

| Variant | Elo over v032 [95%] |
| --- | --- |
| race | -94.3 [-119.1, -70.4] |
| goals | +2.6 [-18.6, +23.8] |
| hold | -57.0 [-78.9, -35.4] |
| all | -130.9 [-157.8, -105.5] |
| all-light | -112.3 [-137.1, -88.7] |
| all-heavy | -208.7 [-237.6, -182.1] |

The race (with E073's band cells held removed) and holding the front
(with idle units charged toward the front) are clearly weaker, and so is
every mix of them, the more the heavier; the goal cells alone are level
with v032.

### Games of the race and hold variants

The owner (2026-11-26) chose to look at games before a second round. The
race and hold variants played v032 at CodinGame's limits on the six maps
of E080's planner study (CodinGame seeds), in both seats, with E080's
`arena/examples/one_map.rs`: race won 3 and lost 9, hold won 7 and lost 5.
From game turn 7, when the search takes over:

- The race variant falls behind from turn 9: on map 925707781 as red it
  holds 25, 27 and 30 cells at turns 9, 11 and 13 against v032's 30, 39
  and 48. It trains 0 to 2 units a turn and builds towers at turns 8, 10
  and 12, and at turn 10 the free cells on its own side (the bottom-left)
  are still untaken while v032 drives into the middle. A free cell next to
  our land already counts as won in the race (we arrive in one turn), so
  taking it adds nothing: the race pays for cells one could take, where
  E073's term, which the race replaced, pays for cells held.
- The hold variant keeps its gold (43, 59 and 60 at turns 10, 12 and 14
  on the same map) and buys towers: a unit counts only on a cell the enemy
  cannot take, which a tower gives, while a unit stepping forward onto an
  exposed cell loses its bonus.

Pictures, action by action: https://claude.ai/artifact/6bTXzn74w3CrGougMPdJuv.

### Round 2: the centre

The owner (2026-11-26): the trouble is one of priorities; value the centre
more first. Two weights more in `front.rs`: `CENTRE`, what each band cell
held in the middle (`|x - y|` at most 3, the middle front's place) counts
beyond E073's band cells held, and `GOAL_MIDDLE`, the factor of the middle
front's goal weights. `screen.yml` run 45825275537 on the owner's approval,
full games against v032, 400 pairs per variant over 2 shards:

| Variant | Commit | Goal E / C / O | Middle factor | Centre | Elo over v032 [95%] |
| --- | --- | --- | --- | --- | --- |
| centre4 | de86e87b6159 | 0 | 1 | 4 | -7.8 [-29.6, +13.9] |
| centre8 | e58955656649 | 0 | 1 | 8 | -12.2 [-33.5, +9.1] |
| goals-mid | c3e494fc3d03 | 4 / 8 / 12 | 2 | 0 | -27.9 [-49.9, -6.0] |
| goals-centre | 95cc1b9a05a8 | 4 / 8 / 12 | 2 | 4 | -39.3 [-62.3, -16.5] |

Weighing the centre more does not help: the centre cells are level or a
little weaker, and doubling the middle front's goal cells is weaker than
the goal cells alone (+2.6 in round 1). These goal cells are still the
turn-1 ones: the owner noticed (2026-11-26) that the entrance can then lie
behind our front line, where it should lie between it and the opponent;
the games' page now moves each front's goals with it (E just past the
side's furthest cell on the front's axis, C the centre or E once past it,
O two steps beyond C), and `front.rs` does not yet.

### Round 3: goals that move, and a pull toward the centre

The owner (2026-11-26): the goal cells should move with the front, the
entrance not next to our front cell but between it and the front's region;
the variants do not take the centre enough: on an open field the centre is
a must, and the land should extend toward it every turn, from the first
turns.

- `MOVING` (`front.rs`): each front's axis is the most central path from
  the side's HQ to the front's centre, then on to the enemy's HQ; the
  entrance lies halfway between the side's furthest cell on the axis and
  the front's region (its entrance cell; once in the region, its centre;
  once past the centre, the next cell), the centre is the front's centre
  (or the entrance once past it), the outpost two steps beyond.
- `CENTRE_PULL` and `CENTRE_HOLD` (`front.rs`, the search from game turn
  7): each step between a side's active land and the middle front's centre
  (the enemy's: its mirror) costs it `CENTRE_PULL`, and holding the centre
  counts `CENTRE_HOLD`, on maps with a middle front.
- `OPENING_PULL` (`main.rs`) and `Order::CentralPull`
  (`opening/src/solo.rs`): the opening (game turns 1 to 6) ranks its plans
  by their active cells less `OPENING_PULL` for each step between their
  land and the middle front's centre, then as `Order::Central`.

`screen.yml` run 45827669985, on the owner's approval (the owner then
stopped the dispatch command, which had already started it): pull4
(`CENTRE_PULL` 4, `CENTRE_HOLD` 24, commit 5a579353dd66), pull8 (8, 48,
26d89cba8641), pull4-moving (pull4 with moving goal cells 4 / 8 / 12,
fd3af5455963) and moving4 (moving goal cells 4 / 8 / 12 alone,
0faf2e07cbab), against v032, full games, 400 pairs each, no fault on
either side:

| Variant | Elo over v032 [95%] |
| --- | --- |
| pull4 | +7.8 [-3.6, +19.2] |
| pull8 | +4.3 [-8.7, +17.4] |
| moving4 | +4.3 [-17.3, +26.0] |
| pull4-moving | -42.8 [-66.1, -19.8] |

The pull alone and the moving goal cells alone lean positive, none with
an interval above 0; together they are clearly weaker. The pull's
intervals are narrower: it acts only on maps with a middle front, so many
of its games are v032's.

Games on the owner's request, against v032 at CodinGame's limits on the
six maps, both seats: open-pull2 (`OPENING_PULL` 2, commit 7ba4d1c1fc2c)
won 6 and lost 6, open-pull2-pull8 (with `CENTRE_PULL` 8 and
`CENTRE_HOLD` 48, 6641ba8f1825) won 6 and lost 6. v032's opening already
extends toward the centre nearly as fast as it can (its steps to the
middle front's centre at game turns 2, 4 and 6: 8, 6 and 3 as red, 7, 5
and 1 to 3 as blue, from 10 at the start): the opening pull changes the
first six turns in one game of the eight on maps with a middle front (map
925707781 as blue: 1 step at turn 6 rather than 3), and both pulled
variants hold the centre from game turn 8 in nearly every game (v032's
opening held it at turn 8 in 5 of the 8). Pictures, action by action:
https://claude.ai/artifact/Fi8iV1NgUNDd48v9GkD4Ks.

### The lost fights before game turn 12

The owner (2026-11-26), on the round 3 results: we still lose too many
fights in the early and middle game, expanding too much and losing our
units, where we would play safer and keep our territory after the
opponent's play. The study of v032's 1,950 CodinGame games
(research/codingame/2026-11-26-early-fights): the cells lost before turn 12
are mostly cut off rather than captured in our lost games (4.7 cut off and
4.6 captured a game, against 1.7 and 2.8 in our wins); in the median large
loss, v032 spends its 32 gold down to 5 on cells it then loses the next
enemy ply to a cut of 2 trainings paid from the enemy's 38.

## Result

Dropped: weaker or level in screening (+7.8 [-3.6, +19.2] at best, pull4; confirmed level in E088). What E083 to E088 taught is in
`research/2026-11-26-early-fights-lessons.md`.
