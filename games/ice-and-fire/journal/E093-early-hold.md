---
id: E093
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Up to game turn 10, offering the step and guard (a unit moves forward, a tower on the cell it leaves) from every unit near the enemy, charging more for each cell the enemy can take, or valuing a tower more, makes the bot hold the ground that sets the frontier, and beats ice-and-fire-v032.
change: Three switches active up to game turn `EARLY_UNTIL` (10), `STEP_GUARD_NEAR` (`actions.rs`), `EARLY_THREAT_QUARTERS` (`eval.rs`) and `EARLY_TOWER_VALUE` (`tactics/src/eval.rs`), on E092's branch (its `GUARD_NEAR` available, off); each at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner (2026-11-26): in game turns 5 to 10, holding ground matters
more, since it sets the frontier, possibly by moving forward and building a
tower. A study first, of v032's 195 CodinGame games against zasmu (v032
always red, zasmu always blue), command by command, and of 80 local games
of v032 against itself, both seats:

| Towers a game, game turn 7 | Towers a game, game turn 8 | Built behind a unit that moved forward, turn 7 |
|---|---|---|
| v032 red against zasmu: 0 | 0.57 | none |
| zasmu (blue): 0.76 | 0.31 | 89% |
| v032 red against itself: 0.04 | 0.31 | 100% (3 towers) |
| v032 blue against itself: 0.44 | 0.54 | 57% |

- Turn 7 is v032's search's first turn; as red it spends its gold on
  trainings (1.56 a game against zasmu) and builds no tower; as blue,
  moving second and already facing red's advance, it builds towers there
  as zasmu does. Part of zasmu's turn-7 towers is its seat.
- Plies of game turns 7 to 10 with a forward step and a tower behind it
  lose fewer cells at the enemy's next ply: 0.31 against 0.57 (v032 red
  against zasmu), 0.20 against 0.43 (red) and 0.24 against 0.36 (blue)
  against itself; those plies differ in other ways too.
- Up to game turn 10, v032's towers stand as near the border as zasmu's
  (61% to 90% on it, none 3 steps inside); from game turn 11 they drift
  inward: 14% 3 or more steps inside (zasmu's 6%), 21% 4 or more steps from
  the enemy (zasmu's 15%), which E092's guard zone addresses.

## The change

Up to game turn `EARLY_UNTIL` (10):

- `STEP_GUARD_NEAR` (0 in v032): E070's step and guard is also offered from
  our units on cells at most that many steps from the enemy's active cells
  or units, not only from those the enemy can reach at its next turn.
- `EARLY_THREAT_QUARTERS` (5 in v032, as `THREAT_QUARTERS`): what each cell
  the enemy can take at its next turn costs.
- `EARLY_TOWER_VALUE` (10 in v032, as `TOWER_VALUE`): what an active tower
  counts in the shared evaluation.

## Screening

Each variant against `ice-and-fire-v032`, 800 pairs over two shards (the
owner's choice, after three leans at 400 pairs that did not hold), full
games, no faults (run 45899985184):

| Variant | Up to game turn 10 | Elo |
|---|---|---|
| fwd2 | `STEP_GUARD_NEAR` 2 | +7.4 [-6.6, +21.4] |
| fwd3 | `STEP_GUARD_NEAR` 3 | -0.0 [-14.1, +14.1] |
| hold8 | `EARLY_THREAT_QUARTERS` 8 | -3.5 [-18.7, +11.7] |
| hold10 | `EARLY_THREAT_QUARTERS` 10 | -17.0 [-31.7, -2.3] |
| tv15 | `EARLY_TOWER_VALUE` 15 | -12.6 [-26.3, +1.1] |
| tv20 | `EARLY_TOWER_VALUE` 20 | -18.3 [-33.5, -3.1] |
| fwd2-tv15 | both | -13.0 [-27.8, +1.6] |
| fwd2-hold8-tv15 | the three | +5.2 [-10.3, +20.7] |

No variant beats v032. A dearer tower or a dearer threatened cell early
loses, the more the heavier, as the all-game versions did (E083's hold,
E084's cut threat, E092's tower value); the step and guard offered from
more units is level.
