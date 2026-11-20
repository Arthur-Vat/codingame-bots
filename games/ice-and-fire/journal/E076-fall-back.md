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
- `RUSH_GOAL` (added after round 3, the owner: "Rush from tip should be
  toward the closest goal which is not claimed by one of our units"; his
  choices: goals are the middle chokes, then the band cells; a goal is
  claimed when one of our units is no farther from it than our nearest
  trainable cell and nearer to it than every enemy unit): the chains head
  for the `RUSH_STARTS` nearest unclaimed goals we do not hold, each from
  the trainable cell nearest its goal.
- `RUSH_SKIP_ENEMY` and `RUSH_LEAP` (added after round 4, from images of
  the goal rush in our positions: it was offered in 303 of 304 positions of
  game turns 7 to 10 and played whole in 7; the owner chose all three
  fixes): goals the enemy holds, or one of its units is nearer to than our
  nearest trainable cell, are left out; each chain is also offered as a
  leapfrog, a movable unit of ours stepping onto its first cell and the
  trainings following from there. The third fix is E073's choke weight
  (`eval::GOAL_CHOKE`, 0 in v030), counted until `GOAL_UNTIL`.

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
(`eval::GOAL_UNTIL`), seed 2026112002 (fb2 again as a replicate), run 44772777429:

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb2 | `FALL_BACK = 2` | +4.8 [−6.8, +16.4] | 0 / 0 |
| fb2r2 | `FALL_BACK = 2`, `RUSH_LENGTH = 2` | −5.2 [−18.5, +8.1] | 0 / 0 |
| g14 | `GOAL_UNTIL = 14` | +2.2 [−9.5, +13.9] | 0 / 0 |
| r2g14 | `RUSH_LENGTH = 2`, `GOAL_UNTIL = 14` | +6.5 [−6.6, +19.6] | 0 / 0 |
| r3g14 | `RUSH_LENGTH = 3`, `GOAL_UNTIL = 14` | +6.5 [−6.6, +19.6] | 0 / 0 |
| fb2r2g14 | `FALL_BACK = 2`, `RUSH_LENGTH = 2`, `GOAL_UNTIL = 14` | +14.3 [+1.3, +27.4] | 0 / 0 |

Eleven variants so far, so the best of them at +14.3 may partly be luck;
fb2 read +6.5 then +4.8 (+5.6 over 1,600 pairs). The rush gains only with
the goal kept to game turn 14 (fb2r2 −5.2, fb2r2g14 +14.3).

### Round 3

Confirmation on a new seed (2026112003), run 44774801182, with the two
combinations not yet played:

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb2r2g14 | `FALL_BACK = 2`, `RUSH_LENGTH = 2`, `GOAL_UNTIL = 14` | +6.1 [−7.3, +19.4] | 0 / 0 |
| fb2g14 | `FALL_BACK = 2`, `GOAL_UNTIL = 14` | +18.3 [+5.5, +31.1] | 0 / 0 |
| fb2r3g14 | `FALL_BACK = 2`, `RUSH_LENGTH = 3`, `GOAL_UNTIL = 14` | +13.5 [+0.4, +26.6] | 0 / 0 |

The fall back with the goal kept to game turn 14 gains in all three: alone
the two read +5.6 (fb2, 1,600 pairs) and +2.2 (g14); fb2r2g14 is about
+10 over its 1,600 pairs. The rush toward the enemy HQ adds nothing.

### Round 4

fb2g14 again, and the rush toward goals (`RUSH_GOAL`) on top of it and
alone, seed 2026112004, run 44776479533:

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb2g14 | `FALL_BACK = 2`, `GOAL_UNTIL = 14` | +30.0 [+17.4, +42.8] | 0 / 0 |
| fb2g14rg2 | as fb2g14, `RUSH_LENGTH = 2`, `RUSH_GOAL` | +9.6 [−3.3, +22.4] | 0 / 0 |
| fb2g14rg3 | as fb2g14, `RUSH_LENGTH = 3`, `RUSH_GOAL` | +23.5 [+10.1, +37.0] | 0 / 0 |
| rg3g14 | `GOAL_UNTIL = 14`, `RUSH_LENGTH = 3`, `RUSH_GOAL` | +20.9 [+8.0, +33.8] | 0 / 0 |

fb2g14 holds: +18.3 then +30.0, about +24 over 1,600 pairs. The goal rush
gains alone (rg3g14 +20.9, against g14 +2.2) but not on top of fb2g14.

### Round 5

The rush's fixes on top of fb2g14 with the 3-training goal rush (B), seed
2026112005, run 44778741431:

| Variant | Switches beyond fb2g14 | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fb2g14 | none | pending | |
| B | `RUSH_LENGTH = 3`, `RUSH_GOAL` | pending | |
| Bskip | B, `RUSH_SKIP_ENEMY` | pending | |
| Bleap | B, `RUSH_LEAP` | pending | |
| Bck4 | B, `GOAL_CHOKE = 4` | pending | |
| Bck8 | B, `GOAL_CHOKE = 8` | pending | |
| Ball | B, `RUSH_SKIP_ENEMY`, `RUSH_LEAP`, `GOAL_CHOKE = 4` | pending | |
| fb2g14ck4 | `GOAL_CHOKE = 4` | pending | |
