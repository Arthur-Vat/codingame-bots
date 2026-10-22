---
id: E007
date: 2026-10-22
pull_request: "#186"
parent: ice-and-fire-v004
hypothesis: A side cut in the middle game with less than 20 gold left keeps losing cells while one with 20 or more holds, so a plan search that keeps 20 gold after its turn, except for attacks, beats v004.
change: In the middle game, the plan search refuses an action that spends gold, leaves less than 20 and is not an attack (a cut or chain macro action, or a training on an enemy cell).
release: ice-and-fire-v005
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-22 after E006, in a form: "Do you see any
other improvement for the early game, based on the qualitative analysis
that you did?" Of the three ideas proposed, he chose "Strike timing
(Recommended)": in the first turns of the middle game, choose between
cutting now and saving one turn. E005 (turn-1 heuristics and time) and
E006 (necks in the plan search) were dropped before it; their entries are
still to be written. The study they and this entry rely on is in
`games/ice-and-fire/research/game-study.md` on the branch
`claude/ice-and-fire-e005-first-turn`.

## What the games said

Before building, the 200 games of the study (v004 against v003, seed 121,
both bots run the same plan search in the middle game) were replayed, with
the arena's `boards` example showing what each side could cut at once with
its own gold. In 161 games one side lost at least 30% of its territory (and
6 cells or more) in one turn, the first such cut of the game:

- **Striking first was seldom possible:** at its last turn before the cut,
  the side cut could have cut 6 cells or more of the other's with its own
  gold in 13 games (3 cells or more in 34).
- **It had spent:** it spent 18.9 gold on average at that turn (20 or more
  in 108 games), mostly one or two trainings, and was left with less gold
  than the side that cut it in 154 games (32.8 before spending, against
  45.3).
- **Gold after the cut decided what followed:** sides with 20 gold or more
  at their answer kept their ground (cells 28.8 before, 15.8 after the cut,
  15.9 one turn later, 14.6 two turns later; 59 games); with less, they went
  on losing (24.9, 12.6, 11.0, 8.2; 42 games).

So the test became the saving side of the idea: keep gold to answer a cut,
and spend it only to attack.

## Screening

Against `ice-and-fire-v004` at CodinGame's limits (the arena's default),
two games at a time:

| Reserve | Seed | Pairs | Score | Elo | Faults |
| --- | --- | ---: | --- | --- | --- |
| 20 | 131 | 200 | 203 wins, 197 losses (50.7%) | +5.2 [−19.6, +30.1] | none |
| 30 | 131 | 200 | 197 wins, 203 losses (49.2%) | −5.2 [−33.6, +23.1] | 1 timeout (v004: 1) |
| 20 | 141 | 400 | 424 wins, 376 losses (53.0%) | +20.9 [+1.5, +40.3] | 1 timeout (v004: 1) |
| **20, both runs** | | **600** | **52.25%** | **+15.6 [+0.3, +31.0]** | |

With 20 gold (seed 131), the candidate held more by the middle game's
later turns (turn 17: 52 gold against 44, 36 cells against 31, Voronoi
share 54 against 46) and had 20 gold or more at its answer to its first
large cut in 43% of games (v004 34%), but it suffered more large cuts in
all (286 against 223; the first large cut of the game 168 against 161).
Answers after the first: median 4.1 ms, 99.9% 31.9 ms, slowest 61.7 ms
(v004 in the same runs: 49.8 ms).

The owner chose the SPRT on a new branch (form, 2026-10-22: "SPRT on new
branch (Recommended)").

## The candidate

`ice-and-fire-v005`: v004 with the reserve of 20 (bundle 90,339
characters, identical to the screened one).

**SPRT:** pending.
