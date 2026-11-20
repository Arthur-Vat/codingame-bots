---
id: E076
date: 2026-11-20
pull_request: "#NN"
parent: ice-and-fire-v030
hypothesis: Units that rush to the front and, when the enemy can reach them, step back (or along the border) leaving a tower on the cell they held, as zasmu does, and rushes trained from our most advanced cell toward the enemy HQ, hold more land after the first contacts and beat ice-and-fire-v030.
change: Two macro actions offered to the search, each behind a switch that is off in v030: fall back and guard (a step from a cell the enemy can reach onto an owned cell, then a tower on the cell left), and the rush from the tip (chains of level-1 trainings into neutral cells from the trainable cells nearest the enemy HQ).
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: none
---

## Why

The owner asked on 2026-11-20 for an analysis of zasmu's games before any
new change, then to look at the first contacts, and proposed: "1. Rush as
far as possible without losing unit 2. Go back the next turn and build a
tower at the border", seen in a zasmu game; then "try several variants for
fall back, since maybe there are other movement (along the border) that may
be better", and "try to rush again more on the center (ideally by spawning
units from the most advanced)".

From the 76 CodinGame games of v029 and v030 against zasmu (we won 9), read
with scratch scripts (not committed):

- Compared within a seat, the two sides are level at game turns 6 and 8
  (units, cells, income, gold): the opening is not where we fall behind.
  By turn 12, as red: units 5.4 against 6.7, cells 32 against 36, income
  32 against 37 (zasmu as red in 7 games only; the blue seat alike).
- Up to turn 12 we lose 2.5 units a game (57 of 191 by turn 9; 116 cut
  off, 57 taken by level-2 trainings), zasmu 1.5 (20 of 117 by turn 9).
- zasmu moves a unit forward to a cell next to our land 7.3 times a game,
  we 4.1 times. At its next turn the unit stays 33% of the time (we 48%),
  steps back 33% (we 25%), goes forward again 34% (we 27%).
- When it steps back and builds a tower on or next to the cell it left
  (46 times), the unit is alive and the cell still held two turns later
  every time (ours: 19 times, 84%). A plain step back: unit alive 95%, cell
  held 88% (ours 82%, 67%). Example: game 925708447, turn 10, zasmu's unit
  12 steps to (3, 6) next to our land; turn 11, `MOVE 12 3 7` back and
  `BUILD TOWER 3 6` on the cell it left.
- In zasmu's positions of turns 7 to 12, our search plays as many moves
  and reaches as deep as zasmu, but at turn 7 only 29% of our own
  trainings are at the tip of our land (4.4 steps behind it on average).

Why the search does not find the fall back: it climbs one action at a time,
and a step back alone leaves the front cell empty and exposed, while the
tower on that cell is not offered until the unit has left it. E070 (step
and guard: a step forward, a tower on the cell left; +71.9) is the same
pair the other way.

## Change

In `games/ice-and-fire/bots/plan/src/actions.rs`, all off in v030:

- `FALL_BACK`: from each cell of ours the enemy can reach at its next turn
  (`eval::reach`), not a mine spot, a unit's step, then a tower on the cell
  it left. 1: onto our cells farther from the enemy HQ (back); 2: onto our
  cells next to an enemy cell (along the border); 3: onto any of our cells
  or an enemy cell other than its HQ. E070 already offers the steps into
  neutral cells.
- `RUSH_LENGTH`: chains of 2 up to this many level-1 trainings into neutral
  cells, from each of the `RUSH_STARTS` (2) trainable neutral cells nearest
  the enemy HQ, each next cell the neutral neighbour nearest the enemy HQ.

## Screening

### Round 1

Full games against v030 on GitHub (`screen.yml`, run 44770695725), 800
pairs a variant over 2 calibrated runners, the workflow's own seed:

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb1 | `FALL_BACK = 1` (back) | −3.0 [−15.6, +9.5] | 0 / 0 |
| fb2 | `FALL_BACK = 2` (along the border) | +6.5 [−5.2, +18.2] | 0 / 0 |
| fb3 | `FALL_BACK = 3` (any step) | −1.3 [−14.1, +11.5] | 1 / 0 |
| r2 | `RUSH_LENGTH = 2` | +4.3 [−7.8, +16.4] | 0 / 0 |
| r3 | `RUSH_LENGTH = 3` | +0.9 [−10.7, +12.5] | 0 / 0 |

How often the actions are played, from 30 local pairs a variant against
v030 (`ice-and-fire-arena match --records`, time scale 0.430, tolerance
6 ms), counted in 60 games: a step onto an owned cell with a tower on the
cell left, the variant against v030 in the same games, fb1 134 against 73,
fb2 100 against 73, fb3 171 against 58 (v030's climb already finds the pair
about once a game); turns with two adjacent level-1 trainings into neutral
cells, r2 82 against 86, r3 90 against 78: the search rarely takes the
rush, whose cells the evaluation values no more than other land after game
turn 10 (E073's goal ends there).

### Round 2

The leaders combined, and the rush with E073's goal kept until game turn 14
(`eval::GOAL_UNTIL`), seed 2026112002 (fb2 again as a replicate):

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb2 | `FALL_BACK = 2` | pending | |
| fb2r2 | `FALL_BACK = 2`, `RUSH_LENGTH = 2` | pending | |
| g14 | `GOAL_UNTIL = 14` | pending | |
| r2g14 | `RUSH_LENGTH = 2`, `GOAL_UNTIL = 14` | pending | |
| r3g14 | `RUSH_LENGTH = 3`, `GOAL_UNTIL = 14` | pending | |
| fb2r2g14 | `FALL_BACK = 2`, `RUSH_LENGTH = 2`, `GOAL_UNTIL = 14` | pending | |
