---
id: E061
date: 2026-11-14
pull_request: "#335"
parent: ice-and-fire-v026
hypothesis: Against the top 10, v026 holds about 1 middle cell fewer at turn 8 and about 4 fewer at turn 12 (the middle: within 2 steps of even between the HQs), with the same land on its own side; most of that gap opens in the middle game, whose evaluation has no term for the land not yet taken; counting the neutral land each side's land reaches first (the race) in the middle game's evaluation takes more of the middle and beats v026. A first form in the opening's plans gained nothing.
change: None released. Screening only: the plan bot's evaluation counting 1 to 3 for each neutral cell our active land reaches before the enemy's, less the same for the enemy's (a tie half), in every turn or until game turn 15; first, the opening's plans counting the land they reach before a growing enemy.
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form after the opening study S0,
[research/codingame/2026-11-14-opening-study.md](../research/codingame/2026-11-14-opening-study.md):
"Both in parallel"): stage S2 of the goal-directed opening, run beside
E062.

## Why

The study S0 of v024 to v026's CodinGame games, against the top 10 in the
same games:

| Seat | Who | Middle held at turn 8 | At turn 12 | Active cells at turn 12 |
| --- | --- | ---: | ---: | ---: |
| red | v026 | 0.6 | 4.3 | 31.3 |
| red | top 10 | 1.6 | 8.1 | 35.1 |
| blue | v026 | 2.2 | 6.8 | 31.2 |
| blue | top 10 | 2.9 | 10.4 | 37.2 |

The middle is the land within 2 steps of even between the HQs. Our units
do not get in each other's way, our opening matches the top 10's up to
turn 7 in cells and unit use, and its search is saturated (16 times more
time changes nothing). Played alone, v026's opening holds 1.7 middle cells
at turn 8 as red and 3.7 as blue: its plans aim at the middle, but the
enemy takes part of it first. Its plans are made with the opponent passing,
so both their objective (the Voronoi share against the enemy's land as it
is) and their rules (each cell's nearness to the enemy's land as it is)
see an enemy that never grows.

## First form: the opening

- `opening/src/solo.rs`: `Objective::race` and `race_offset`, and
  `race_halves`: each neutral cell our active land reaches (on game turn
  `t + d − 1`) before the enemy can (on the turn of its steps from its HQ
  plus the offset, or sooner from the land it holds), a tie half. The bot:
  `OPENING_RACE`, `OPENING_RACE_OFFSET` (0 and 0: v026).
- `opening/src/econ.rs`: `Params::race` and `race_slack`: the free cells
  the enemy could reach at most `race_slack` turns after our land are
  urgent, and a cell counts `race` more for each step it is nearer one than
  6 (`RACE_REACH`), so that the rules head for them. The bot:
  `OPENING_RULES_RACE`, `OPENING_RULES_RACE_SLACK` (0 and 0: v026).

A local probe on 30 maps, our side alone (the headroom example of S0):
neither term changes the middle held at turn 8 (1.5 to 1.8 as red, 3.7 to
4.0 as blue), and the pull costs up to 0.7 cells as red. Alone, the
opening already reaches the middle; what the terms change is which cells
it heads for once an enemy grows, which only games show.

## Screening

Against `ice-and-fire-v026` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). Run 43623544558, seed 61061,
800 pairs each over two runners:

| Variant | Commit | Objective race | Rules pull, slack | Elo against v026 |
| --- | --- | ---: | --- | --- |
| o2 | `8611e3b` | 2 | none | +6.1 [−9.0, +21.2] |
| o2p2 | `5448281` | 2 | 2, 1 | −0.9 [−16.6, +14.8] |
| p2 | `a3d8f90` | 0 | 2, 1 | −6.5 [−20.8, +7.7] |
| o4 | `7ad03af` | 4 | none | −13.5 [−30.1, +3.1] |

No variant gained significantly; the larger objective weight leans
weaker, and the rules' pull does not help. The code stays reachable in the
branch's commits (the variants above, and `1420a35`, the base); it was
removed from the branch afterwards.

## Second form: the middle game

Asked by the owner on 2026-11-14 ("Rethink the term", then "Yes,
middle-game race"). Why the first form could change little: at turn 8, the
end of the opening, the top 10 lead v026 by 1 middle cell (red 1.6 against
0.6, blue 2.9 against 2.2); by turn 12 by about 4 (8.1 against 4.3, 10.4
against 6.8), and their whole lead in active cells (35.1 against 31.3,
37.2 against 31.2) is in the middle. Most of the gap opens after the
opening, when the middle game's search plays. Its evaluation counts what
each side holds (gold, income, active cells, units, towers, threats, shape,
mines, E060's idle units), but nothing for the neutral land still to take,
which is most of the middle at turns 8 to 12; no journal entry tried a
Voronoi or race term in the middle game.

`bots/plan/src/eval.rs`: `race_halves`, a flood of both sides' active land
over neutral cells, a step at a time and at once, each cell to the side
that reaches it first (a tie half), our halves less the enemy's; `RACE_CELL`
(what a cell counts; 0: v026, which the base commit keeps) and
`RACE_UNTIL` (the last game turn it counts; `None`: every turn). A held
active cell counts 4 in the shared evaluation, so a cell won in the race is
worth a quarter to three quarters of one held.

### Screening

Against `ice-and-fire-v026` on GitHub, as above. Run 43625801764, seed
61062, 800 pairs each over two runners (no fault):

| Variant | Commit | A cell in the race | Until | Elo against v026 |
| --- | --- | ---: | --- | --- |
| m1 | `c5f430d` | 1 | every turn | +3.0 [−10.7, +16.8] |
| m2 | `f90989d` | 2 | every turn | **−34.0 [−48.4, −19.6]** |
| m2t15 | `33cba59` | 2 | turn 15 | **−33.5 [−48.3, −18.9]** |
| m3 | `99b3f2f` | 3 | every turn | **−65.9 [−81.6, −50.5]** |

The race in the middle game's evaluation loses, the more the larger its
weight, and as much when it stops at turn 15: the loss comes in the turns
when the middle is contested. At 1 a cell it does nothing.

## What was learned

- **The gap is not a missing term for the land to take.** Counting it in
  the opening's plans changed little (their choice among 27 weight sets
  and 8 ways of buying moved the middle held by under a cell, alone);
  counting it in the middle game lost, the more the larger its weight.
  At 2 a cell, half of a held cell, the search already gives up held land
  or gold for land it may reach first, which the enemy then takes or cuts.
- **A Voronoi-like race is a poor proxy when contact has begun:** it
  counts land that units cannot hold, and the plan search, which scores
  one turn and the enemy's replies, cannot see that the cells will be
  contested.
- **What the top 10 do in turns 8 to 12 is still open.** Their units stay
  at the edge of the land (study S0, section 6), their trainings take more
  land, and their first mine as red comes later (E062 tests the last).
  Measures of how they win the middle cells (by moves, trainings or cuts,
  and from where) would come before another term.

The code stays reachable in this pull request's commits: the opening form
at `1420a35` and its variants, the middle-game form at `a8c97e6` and its
variants. The branch's head holds the journal entry alone.

