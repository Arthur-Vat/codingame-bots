---
id: E077
date: 2026-11-20
pull_request: "#NN"
parent: ice-and-fire-v031
hypothesis: A strategic turn for game turns 7 to 14 (the frontline units step toward their objectives, then units are trained at their tips), offered to the search, which stays the judge, takes and holds more of the centre and beats ice-and-fire-v031 (Step 4 of the frontline plan).
change: The strategic layer of Steps 2 and 3 (objectives, race, roles) is turned on from game turn 7 to 14; its turn is offered to the search as a macro action, as the first climb's start, or in pieces (each frontline unit's step and tip training) at every step of the climbs, and an objectives' term may join the score (`strategy::TURN`, `TRAININGS`, `OBJECTIVE_QUARTERS`, `OBJECTIVE_RACE`, `FROM_TURN`, `UNTIL_TURN`).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

Step 4 of the frontline plan
([research/codingame/2026-11-20-frontline-plan.md](../research/codingame/2026-11-20-frontline-plan.md)),
asked for by the owner on 2026-11-20 ("Go on with step 4"). Steps 2 and 3
(#382) built the objectives (every middle choke and band cell, valued by
the land the side gains by holding it), the race to each (a unit walking,
or a training then walking, the enemy's gold and income counted) and the
roles (at most one frontline unit per objective we reach first or at the
same time, kept between turns, with the handover to a nearer unit), with no
behaviour change. Against zasmu we hold fewer band cells at game turn 10
(red 5.0 against 8.4, blue 7.9 against 12.8) and train at the tip less
(68% of game turn 4's trainings against 93%).

## Change

In `games/ice-and-fire/bots/plan/src/strategy.rs` and
`strategy/turn.rs`, all off in v031 (with them off the bot plays as v031:
20 games against v030 at fixed tries, identical results and lengths):

- The **strategic turn** (`strategy::turn::strategic_turn`): each
  frontline unit, in role order, steps to a neighbour one step nearer its
  objective (a cell not ours first); then, while the gold pays 10, a
  level-1 training next to each frontline unit, on a cell nearer its
  objective. With `TRAININGS = 2`, the trainings the roles give the
  objectives no unit covers follow, on our trainable cell nearest each.
  Every command is checked on a copy, so the plan applies whole.
- **How the search gets it** (`TURN`, bits added): 1, one macro action at
  the climbs' root; 2, the first climb of the turn is forced to start
  with it (E029's forced climbs); 4, its pieces (one frontline unit's step
  and its tip training) offered as macro actions at every step of the
  climbs, so that the search takes some and not others.
- **The objectives' term** (`OBJECTIVE_QUARTERS`, in quarters of a cell):
  each counted objective adds its value if we hold it and subtracts it if
  the enemy does; with `OBJECTIVE_RACE`, a neutral one counts its value
  times our race margin at the turn's start clamped to −1..1 (the plan's
  definition). It is added to every evaluation of our search, replies
  included, not to the opponent model's.
- The layer runs at game turns `FROM_TURN` (7) to `UNTIL_TURN` (14); the
  bot's debug line says whether the plan played starts with the strategic
  turn and how many of its pieces it holds.

The back-line units are left to the search (Step 5, E078, gives them
targets).

## A first look

Locally (time scale 1, tolerance 20 ms, against v031), with `TURN = 3`
the strategic turn has 1 to 8 commands at most game turns 7 to 11 (often
4 or 5 moves and 1 or 2 trainings) and is played whole at 2 of about 290
of the bot's turns: the search judges the whole rush worse than its own
plan against the replies. With the pieces (`TURN = 4` or `7`), about a
fifth of game turns 7 to 14 play at least one piece (22 of 125, 25 of 128),
27 to 30 pieces in 8 pairs; the whole turn 3 or 4 times. The time per turn
is as v031's (99th percentile 43 ms).

## Screening

### Round 1

Seed 2026112101, 800 pairs a variant over 2 calibrated runners, against
v031, in duels cut at game turn 12 (`arena: early`, run 44827862662) and
in full games (run 44827866653):

| Variant | Switches | Cut at turn 12 | Full games |
| --- | --- | --- | --- |
| t1 | `TURN = 1` (offered) | | |
| t2 | `TURN = 2` (first climb) | | |
| t3 | `TURN = 3` (both) | | |
| t4 | `TURN = 4` (pieces) | | |
| t7 | `TURN = 7` (all three) | | |
| t4q4 | `TURN = 4`, `OBJECTIVE_QUARTERS = 4` | | |
| t3q4 | `TURN = 3`, `OBJECTIVE_QUARTERS = 4` | | |
| q4 | `OBJECTIVE_QUARTERS = 4` | | |
