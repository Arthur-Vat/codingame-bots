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

Pending.
