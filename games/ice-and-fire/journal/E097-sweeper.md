---
id: E097
date: 2026-11-27
pull_request: "#NN"
parent: ice-and-fire-v033
hypothesis: A layer before the search that sends each unit behind the front to its own free cell, on its own path, takes the free land far from the front as the top players do without pulling units from the middle, and beats ice-and-fire-v033.
change: `sweep::sweep` matches our level-1 units more than `DIST` steps from the enemy's land to free cells as far from it, nearest pairs first, each cell and unit once, and steps each matched unit toward its cell (a neutral step first, never onto another unit's step); with `sweep::MODE` 1 its moves are played first and the search plans the rest of the turn, with 2 they are offered to the search as one macro action; 0, nothing, in v033.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E096 measured, on v032's CodinGame games against the top 10, that our
units crowd at game turns 8 to 12 and stand still later, that we take the
free land far from the front later than zasmu and karliso, and that the
more of our units rush into the middle the less far land we take; but no
term in the search's evaluation closed the gap (far land −16 to −64, unit
room level at a weight of 1, a unit kept at home level). The owner asked
for a structural change (2026-11-27). The search spends its time where the
score moves, near the front; units behind it get what is left. The top
players behave as if their units had roles.

## Change

`bots/plan/src/sweep.rs`, from game turn `FROM` (7, the search's first)
to `UNTIL` (20):

1. The units behind the front: ours, level 1, on active cells more than
   `DIST` (4) steps from the enemy's land, that can still move.
2. Their targets: the neutral cells as far from the enemy's land.
3. A matching: every (unit, target) pair within `RANGE` (6) steps, nearest
   first; a pair is kept when neither its unit nor its target is taken. No
   two units head for the same cell.
4. Each kept unit steps onto a neighbour one step nearer its target, a
   neutral one first (taken on the way), never a cell another unit steps
   onto or one of our units stands on: units spread over the free land
   rather than walk the same path (the owner's collaborative snake).
5. The moves the engine accepts, in order.

`session.rs` uses them by `MODE`: 1, fixed, plays them on a copy of the
position and lets the search plan the rest of the turn from there, the
sweeper's moves sent first; 2, offered, gives them to the search as one
macro action at the climbs' root (E077's guide), which it takes only when
it scores better.

## Local check

Each form against v033 in local games (seed 97, twice CodinGame's time, one
game at a time; about 46 games each), measured with E096's tools, the
variant's side against v033's in the same games:

| Measure | fixed | v033 | offered | v033 |
| --- | ---: | ---: | ---: | ---: |
| Far free cells taken a turn, game turns 8 to 12 | 0.88 | 0.51 | 0.47 | 0.47 |
| Near free cells taken a turn, game turns 8 to 12 | 2.60 | 2.91 | 2.76 | 2.75 |
| Own units next to each unit, game turns 8 to 12 | 0.55 | 0.60 | 0.62 | 0.61 |
| Units at home at game turn 8 | 1.04 | 0.38 | 0.39 | 0.37 |
| Units in the middle at game turn 8 | 2.83 | 3.34 | 3.46 | 3.48 |
| Cells, game turns 13 to 18 | 46.9 | 45.8 | 45.8 | 45.3 |
| Wins | 22 of 47 | 25 | 22 of 46 | 24 |

The fixed form does what it was built for: 70% more far land at game turns
8 to 12, a unit kept at home as zasmu does (1.47 at turn 8 in v032's
CodinGame games), slightly less crowding and as much land, for a little
less of the middle. The offered form changes nothing: the search almost
never takes the macro action. Only the fixed form is screened.

## Screening

800 seat-swapped pairs per variant against `ice-and-fire-v033`, whole
games at CodinGame's limits, 4 shards
([run 46017255301](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46017255301)):

| Variant | Commit | `MODE` | `DIST` | `UNTIL` | Elo over v033 [95%] | Score |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `off` | `7af1128` | 0 | 4 | 20 | −0.9 [−10.9, +9.1] | 49.9% |
| `d4` | `5806d9b` | 1 | 4 | 20 | −6.9 [−21.5, +7.5] | 49.0% |
| `d3` | `97d77c3` | 1 | 3 | 20 | −32.7 [−48.7, −16.8] | 45.3% |
| `d5` | `29dd6ef` | 1 | 5 | 20 | −6.1 [−20.5, +8.3] | 49.1% |
| `d4-u14` | `ef127aa` | 1 | 4 | 14 | −8.7 [−23.3, +5.8] | 48.8% |

One timeout of v033 against `off` in 8,000 games; no other fault.

No variant beats v033. Sweeping from 4 or 5 steps is level or slightly
below (−6 to −9), and from 3 steps, which sends more units away from the
fight, loses clearly (−32.7). So taking the far land earlier, which the
sweeper does (70% more at game turns 8 to 12 in the local games), does not
win games: in the top players' games it goes with their strength rather
than causing it, and every unit sent to it is missing from the middle.
