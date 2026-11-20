---
id: E076
date: 2026-11-20
pull_request: "#375"
parent: ice-and-fire-v030
hypothesis: Units that, when the enemy can reach them, step along the border leaving a tower on the cell they held, as zasmu does, with the contested band counted until game turn 14, hold more land after the first contacts and beat ice-and-fire-v030 (candidate ice-and-fire-v031).
change: The search is also offered, for each unit on a cell the enemy can reach, a step onto another of our cells next to an enemy cell together with a tower on the cell it leaves (`actions::FALL_BACK = 2`), and the evaluation counts the contested band until game turn 14 (`eval::GOAL_UNTIL`, 10 before); the rush from the tip, the centre's defence and the land term screened alongside stay behind switches that are off.
release: ice-and-fire-v031
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
| fb2g14 | none | +17.8 [+4.7, +31.0] | 0 / 0 |
| B | `RUSH_LENGTH = 3`, `RUSH_GOAL` | +5.2 [−7.5, +17.9] | 0 / 0 |
| Bskip | B, `RUSH_SKIP_ENEMY` | +12.2 [−0.4, +24.7] | 0 / 0 |
| Bleap | B, `RUSH_LEAP` | +12.2 [−1.4, +25.8] | 1 / 0 |
| Bck4 | B, `GOAL_CHOKE = 4` | +15.2 [+1.3, +29.2] | 0 / 0 |
| Bck8 | B, `GOAL_CHOKE = 8` | −0.4 [−14.0, +13.2] | 1 / 0 |
| Ball | B, `RUSH_SKIP_ENEMY`, `RUSH_LEAP`, `GOAL_CHOKE = 4` | +15.6 [+2.0, +29.4] | 0 / 0 |
| fb2g14ck4 | `GOAL_CHOKE = 4` | +6.5 [−7.0, +20.0] | 0 / 0 |

fb2g14 holds a third time: +18.3, +30.0, +17.8 on three seeds, about +22
over 2,400 pairs. Each fix lifts the goal rush (B +5.2; skip and leap
+12.2; chokes 4 +15.2; all three +15.6) but none goes beyond fb2g14 in the
same round; chokes at 8 lose the gain, and chokes at 4 without the rush add
nothing.

### Round 6

The owner, 2026-11-20: "Stops chasing lost goals, and also defend when the
opponent can grab territory?", "territory in the center is key, because we
1. Lose that cell for the opponent 2. Shrink our territory. So it shouldn't
be counted as a single cell"; his choices: value a centre cell by the land
it decides (`eval::VORONOI`: each cell nearer to a side's active land than
to the enemy's counts for it, until `VORONOI_UNTIL` = 14), and defend by
training onto it first, a tower behind it, and blocking its path
(`actions::RUSH_DEFEND`: for the chokes and band cells the enemy reaches
first or can take at its next turn, the chain that takes them this turn
and towers on or next to them; the path is blocked by the single trainings
already offered). Seed 2026112006, run 44780199145:

| Variant | Switches | Elo over v030 [95%] | Faults (variant / v030) |
| --- | --- | --- | ---: |
| fbv1 | fb2g14, `VORONOI = 1` | +15.2 [+0.2, +30.3] | 0 / 0 |
| fbv2 | fb2g14, `VORONOI = 2` | −22.2 [−37.8, −6.7] | 0 / 0 |
| fbd | fb2g14, `RUSH_DEFEND` | +13.0 [−0.7, +26.8] | 0 / 0 |
| fbdv1 | fb2g14, `RUSH_DEFEND`, `VORONOI = 1` | +3.5 [−11.2, +18.2] | 0 / 0 |
| fbdv2 | fb2g14, `RUSH_DEFEND`, `VORONOI = 2` | −36.2 [−52.0, −20.5] | 0 / 0 |
| Bdv1 | B, `RUSH_DEFEND`, `VORONOI = 1` | +3.0 [−11.6, +17.7] | 0 / 0 |
| Balldv1 | Ball, `RUSH_DEFEND`, `VORONOI = 1` | +1.3 [−13.6, +16.2] | 0 / 0 |
| v1 | `VORONOI = 1` alone | +2.6 [−11.8, +17.0] | 0 / 0 |

No fb2g14 control in this round (+22 over three seeds before): the land
term at 1 and the defence are at best level with it, and the land term at
2 loses clearly (−22, −36 with the defence). The land term alone adds
nothing (+2.6), nor does the defence on top of the rush.

### Game turns 7 to 12, after the rounds

From the 76 CodinGame games against zasmu, within a seat (our band cells
by the definition of `eval::goal_zones`' band, recomputed in a scratch
script): band cells held at game turn 10, red 5.0 (us) against 8.4
(zasmu, 7 games), blue 7.9 (us, 7 games) against 12.8; both sides first
hold a band cell at about the same turn (7 or 8 as red, 6 or 7 as blue).
The steps from each unit to its nearest own unit at game turn 8: red 2.05
against 2.44, blue 1.65 against 2.39 (level by turn 10). The owner sees two
dimensions (2026-11-20): goals reached, and back-line units spread; a third
is the gold fed to them (we hold 50 to 100 gold at game turns 11 to 13).
