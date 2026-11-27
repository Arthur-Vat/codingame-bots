---
id: E102
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v034
hypothesis: The groups of actions that decide v034's plans (moves, blockers, cuts) and those the top players use but the search is never offered (towers off the guard filter) point to changes in what the search considers; with v034's speed, offering more of the valuable groups or exploring plans that start with a weak move beats ice-and-fire-v034.
change: Switches that leave v034 as it plays, and one variant commit for each change of what the search offers or explores, screened together against ice-and-fire-v034.
release: none
sprt: not run: no variant gained in the confirmation screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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
whole games at CodinGame's limits, 2 shards
([run 46056838757](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46056838757)):

| Variant | Change | Elo over v034 [95%] |
| --- | --- | ---: |
| `off` | none (control) | −7.8 [−13.6, −2.0] |
| `nofar` | `FAR_TRAININGS` 3 (E101's best, again) | +13.3 [+4.2, +22.3] |
| `towerall` | `GUARD_FILTER` off: towers on every free border cell | −28.9 [−38.7, −19.3] |
| `wall` | `WALL_TOWERS`: towers also where they guard an unguarded frontier cell | +7.6 [−0.5, +15.7] |
| `blknbr` | `BLOCK_NEIGHBOURS`: blockers also next to the cells a reply enters | −6.1 [−15.1, +2.9] |
| `cuts8` | `MACRO_CUTS` 8 rather than 4 | +11.3 [+2.3, +20.3] |
| `fullfirst` | `FULL_FIRST`: every action tried at a climb's first step | −0.7 [−10.7, +9.4] |
| `mem12` | `MEMORY_CAP` 12 rather than 8 | −8.0 [−17.4, +1.3] |

Commits: `off` `eb03b73`, `nofar` `088712f`, `towerall` `44e4c76`, `wall`
`9fc5358`, `blknbr` `b5392ba`, `cuts8` `3dd5650`, `fullfirst` `dc6d8f8`,
`mem12` `a272df6`. Two timeouts of v034 against `blknbr`; no other fault.

The control, which plays as v034, scored −7.8 here and +11.3 in E101's
screening: even at 1,600 pairs a screening's noise is of that size, so the
variants are read against both. Offering towers on every border cell loses
clearly (−28.9): E026's guard filter still pays, though the top players
build most of their towers where it offers none. The far trainings left
out (`nofar`, E101's variant) are positive in both screenings (+16.1 on
800 pairs, +13.3 on 1,600) and above the control each time (by 4.8 and
21.1); 8 cuts offered rather than 4 (`cuts8`, +11.3) and the wall towers
(+7.6) come next. A full first step, blockers on the neighbours and a
memory of 12 do nothing or a little less.

## Confirmation

The owner asked (2026-11-27) for `nofar`'s SPRT on its own (E101,
`ice-and-fire-v035`, #400) and, meanwhile, a larger screening of the other
variants against v034: 2,400 pairs each, 3 shards, seed 7702, with two
controls to show the noise
([run 46059408079](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46059408079)):

| Variant | Change | Elo over v034 [95%] |
| --- | --- | ---: |
| `off` | none (control) | +1.4 [−2.8, +5.7] |
| `off2` | none (a second control, the same commit) | −2.3 [−6.9, +2.3] |
| `cuts8` | `MACRO_CUTS` 8 | −1.3 [−8.4, +5.8] |
| `wall` | `WALL_TOWERS` | −4.8 [−10.7, +1.2] |
| `nofar-cuts8` | `FAR_TRAININGS` 3 and `MACRO_CUTS` 8 (`4569cda`) | −2.2 [−9.9, +5.6] |
| `nofar-cuts8-wall` | the three (`a5d158b`) | −1.4 [−9.2, +6.3] |

One timeout of `nofar-cuts8` and one of v034 against `nofar-cuts8-wall`;
no other fault.

Every variant is level with v034, and so are both controls here (+1.4,
−2.3). Meanwhile `nofar` alone (`ice-and-fire-v035`) was rejected by its
SPRT, −0.6 [−8.5, +7.2] over 2,240 pairs (E101). The gains of the first
screening (+11.3 for `cuts8`, +7.6 for `wall`, +13.3 for `nofar`) were
the noise of a screening, whose controls had scored +11.3 and −7.8.

## What was learned

Dropped. What v034 offers the search is about right: the groups that
decide its plans (moves, blockers, cuts) are all offered; more cuts, wall
towers or a larger memory change nothing, towers on every border cell lose
(−28.9), and the trainings far from the enemy it could leave out cost
nothing to keep. The search's choices are limited by what it values and by
its model of the opponent, not by what it is offered. The measures (what
the plans are made of, what the top players play that the generator never
offers) stand as a map for later work: the top players' towers away from
the front are the largest group of their commands the search never
considers.

A screening of 800 to 1,600 pairs can be off by 10 to 15 Elo in either
direction whatever its interval says; a gain that small needs a second
screening on another seed, or the SPRT.

The code stays reachable on the branch `claude/ice-and-fire-e102-action-groups`
(commits up to `e15e12b5`: the switches `GUARD_FILTER` and `FULL_FIRST`, and
the variants).
