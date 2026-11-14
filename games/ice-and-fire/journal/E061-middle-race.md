---
id: E061
date: 2026-11-14
pull_request: "#NN"
parent: ice-and-fire-v026
hypothesis: v026's opening plans as if the opponent passed, so it values land against the enemy's land as it is, not as it will be; alone it reaches the middle of the map as the top 10 do (1.7 cells at turn 8 as red), but against them it holds 0.6 (they 1.6) and by turn 12 half their middle; counting the land we can still reach before the enemy, who grows a step a turn from its HQ, in the plans' objective or in the rules that pick each unit's cell, takes more of the middle and beats v026.
change: The opening's objective counts each neutral cell our land reaches before the enemy can (`solo::race_halves`), or its rules pull units toward the free cells the enemy could reach at most 1 or 2 turns after us (`econ::Params::race`), or both.
release: none
sprt: pending
elo: none
pairs: none
full_time: none
decision: pending
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

## The change

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
weaker, and the rules' pull does not help.
