---
id: E070
date: 2026-11-19
pull_request: "#NN"
parent: ice-and-fire-v028
hypothesis: At turns 8 to 10 against the top 6, v028 takes about 1.2 fewer neutral cells a ply than they do, partly because its units stay beside neutral cells they could take (0.40 a ply as red, against 0.18); its own search scores the step lower in 103 of 126 such positions, mostly because the cell left behind opens to the enemy's level-1 trainings, and its hill climb never tries the step together with a guard on that cell; offering that pair as one action lets the units advance and beats v028.
change: The plan search is offered a macro action for each unit on a cell of ours the enemy can reach at its next turn and each neutral neighbour it may enter (STEP_GUARD): the step, then a tower on the cell it left (1) or a level-1 training there (2).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-19 (forms): the land gap first ("Land:
opening/early middle"), a diagnostic before any change ("Diagnose first",
then "Find why units stay"), and the variants "Step + tower" and "Step +
level-1 refill".

## The land gap

`codingame-units` on `ice-and-fire-v028`'s 158 ranking games (data branch
e3b20fa), its side against the top 6 of the newest snapshot and theirs
against it, in the same games, by seat:

| Turns 8 to 10, a ply | v028 red | top 6 red | v028 blue | top 6 blue |
| --- | ---: | ---: | ---: | ---: |
| Units that may move | 5.0 | 5.7 | 4.9 | 5.7 |
| Neutral cells taken by units | 2.9 | 4.1 | 2.7 | 3.8 |
| Units that stayed beside a neutral cell they could take | 0.40 | 0.18 | 0.27 | 0.28 |
| Units blocked at the frontier | 0.62 | 0.57 | 0.96 | 0.54 |
| Trainings | 0.52 | 0.61 | 0.81 | 0.78 |

Up to game turn 7 both sides play alike. Of the units that stayed beside a
neutral cell, 80% had an enemy unit within 2 steps. A race term in the
evaluation was tried and dropped in [E061](E061-middle-race.md).

## Why the units stay

A scratch command (not committed) wrote a bank of the 142 plies of v028 at
game turns 8 to 13 against the top 6 where one of its units stayed beside a
neutral cell, each twice: with v028's answer, and with that answer plus the
unit's step (to the neutral neighbour nearest the enemy HQ). v028's own
search (its tag, `judge` of the analysis build, 600 tries, patched in the
scratch copy to print the worst reply) scored both:

- in 103 of the 126 positions whose scores are comparable, the step scores
  lower (median −29, mean −36); in 21 it is level or better; in 11 more it
  exposes our HQ, and in 5 the game is lost either way;
- the worst reply the search expects after the step is the enemy's
  level-1 trainings in 67 of the 103, a level-2 or level-3 kill in 19, moves
  alone in 17.

A unit stands on a cell that an enemy level-1 unit cannot enter; stepping
forward leaves it empty. Zasmu holds its front with towers, which frees its
units. The hill climb adds one action at a time, so the step and a guard on
the cell left behind are never tried as a pair.
