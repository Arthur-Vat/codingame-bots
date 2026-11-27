---
id: E102
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v034
hypothesis: The groups of actions that decide v034's plans (moves, blockers, cuts) and those the top players use but the search is never offered (towers off the guard filter) point to changes in what the search considers; with v034's speed, offering more of the valuable groups or exploring plans that start with a weak move beats ice-and-fire-v034.
change: Switches that leave v034 as it plays, and one variant commit for each change of what the search offers or explores, screened together against ice-and-fire-v034.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

After E101 found no clear gain in the time v034 leaves unused, the owner
asked (2026-11-27) which groups of actions contribute to the plans played,
whether positions or actions worth looking at are never considered, and
for more screenings comparing strategies on many runners.

## Measures

**What the plans played are made of.** v034's search (600 tries, the
opening over) on the 2,031 positions of v033's 159 CodinGame ranking games,
with each step of each climb recorded (its kind and its gain in the
search's score); a step that changes the score by WIN / 4 or more counts as
decisive (an HQ threat made or removed), the others as material:

| Kind of step | Share of steps | Decisive | Mean material gain | Share of material gain |
| --- | ---: | ---: | ---: | ---: |
| Move to the front (next to the enemy) | 27.0% | 29 | 21.2 | 23.8% |
| Move 2 to 3 steps from the enemy | 22.9% | 19 | 11.9 | 11.4% |
| Move farther | 20.5% | 7 | 9.4 | 8.1% |
| Level-1 training at the front | 6.7% | 21 | 31.3 | 8.6% |
| Blocker tower (E044) | 4.6% | 75 | 100.6 | 16.4% |
| Mine | 3.9% | 0 | 6.2 | 1.0% |
| Cut or chain (macro) | 3.6% | 38 | 104.9 | 14.2% |
| Step or fall back with a tower (macro) | 2.4% | 12 | 30.1 | 2.9% |
| Tower at the front | 1.9% | 14 | 32.1 | 2.4% |
| Level-3 training at the front | 1.7% | 25 | 66.9 | 4.1% |
| Level-2 training at the front | 1.5% | 17 | 66.2 | 3.6% |
| Level-1 training 2 to 3 steps away | 1.3% | 8 | 19.4 | 1.0% |
| Tower 2 to 3 steps away | 0.8% | 13 | 55.5 | 1.5% |
| Level-1 training farther | 0.7% | 10 | 16.1 | 0.4% |
| Tower farther | 0.6% | 34 | 64.1 | 0.8% |

Moves are 70% of the steps and 43% of the material gain, and 94% of the
plans played hold one. Blockers and cuts are few (8% of the steps) but give
30% of the material gain and most of the decisive steps. 27% of the plans
played came from a forced climb (E029), most of them started by a move
(near 175, far 138, front 109 of 551): a move whose gain alone is small
opens the best plan, and a lazy first step skips it. The plan played was
the 8th found, of 18, at the median.

**What the search is never offered.** Each command played from game turn
7 in those games, checked against v034's action generator in the position
it was played from (one command at a time, in order):

| Commands of the top players (9,886) | Offered | Never offered |
| --- | ---: | ---: |
| Moves (6,451) | 100% | 0% |
| Level-1 trainings at the front | 96.8% | 3.2% |
| Level-1 trainings 2 to 3 steps away | 87.3% | 12.7% |
| Towers at the front (432) | 64.4% | 35.6% |
| Towers 2 to 3 steps away (497) | 15.9% | 84.1% |
| Towers farther (135) | 16.3% | 83.7% |

The trainings left out are on the player's own cells, which E045 stopped
offering (+30.5). Most of the top players' towers are where the guard
filter (E026: a tower only where it lowers the cells the enemy can take at
its next turn, or guards the HQ) offers none; E026 won +24.9 for v010, a
search with far fewer tries than v034.

## Change

New switches, each leaving v034 as it plays (the same answers and scores
on the 2,031 positions at 600 tries): `actions::GUARD_FILTER` (on) and
`search::FULL_FIRST` (off), beside the existing `WALL_TOWERS`,
`BLOCK_NEIGHBOURS`, `MACRO_CUTS`, `MEMORY_CAP` and E101's `FAR_TRAININGS`.

## Screening

Variants against `ice-and-fire-v034`, 1,600 seat-swapped pairs each,
whole games at CodinGame's limits, 2 shards:

| Variant | Change | Elo over v034 [95%] |
| --- | --- | ---: |
| `off` | none (control) | pending |
| `nofar` | `FAR_TRAININGS` 3 (E101's best, again) | pending |
| `towerall` | `GUARD_FILTER` off: towers on every free border cell | pending |
| `wall` | `WALL_TOWERS`: towers also where they guard an unguarded frontier cell | pending |
| `blknbr` | `BLOCK_NEIGHBOURS`: blockers also next to the cells a reply enters | pending |
| `cuts8` | `MACRO_CUTS` 8 rather than 4 | pending |
| `fullfirst` | `FULL_FIRST`: every action tried at a climb's first step | pending |
| `mem12` | `MEMORY_CAP` 12 rather than 8 | pending |
