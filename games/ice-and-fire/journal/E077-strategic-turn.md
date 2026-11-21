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
| t1 | `TURN = 1` (offered) | −5.6 [−18.4, +7.1] |  −3.5 [−15.9, +8.9] |
| t2 | `TURN = 2` (first climb) | +2.2 [−11.5, +15.9] |  −11.3 [−25.3, +2.7] |
| t3 | `TURN = 3` (both) | −3.9 [−17.8, +10.0] |  −12.6 [−26.6, +1.4] |
| t4 | `TURN = 4` (pieces) | −10.9 [−24.4, +2.6] |  −14.8 [−28.1, −1.5] |
| t7 | `TURN = 7` (all three) | −10.0 [−24.4, +4.4] |  −4.3 [−18.0, +9.3] (1 timeout) |
| t4q4 | `TURN = 4`, `OBJECTIVE_QUARTERS = 4` | −8.3 [−23.3, +6.8] |  −22.6 [−36.8, −8.5] |
| t3q4 | `TURN = 3`, `OBJECTIVE_QUARTERS = 4` | −18.3 [−33.6, −3.0] |  −9.1 [−23.3, +5.0] |
| q4 | `OBJECTIVE_QUARTERS = 4` | +7.4 [−7.7, +22.5] |  −4.3 [−19.0, +10.3] |

No variant gains in either judge; the pieces lose in both (t4 −10.9 cut,
−14.8 full; t4q4 −8.3, −22.6), and forcing the first climb to start with
the whole turn (t2, t3) is level cut at turn 12 but weaker in full games.
The objectives' term alone is level (+7.4 cut, −4.3 full). The search
already rejects the whole turn in about 99% of turns; where the pieces make
it play the rush, it plays worse. Faults: one timeout of t7 in 1,600
games, none elsewhere.

## Diagnosis after round 1

The owner chose (form, 2026-11-20) to diagnose before more variants.

**Games against v031** (local, 60 pairs each, seed 2026112102, time scale 1,
tolerance 20 ms, `ice-and-fire-review games` metrics at the start of each
ply): t4 and t3 hold the same land as a v031-against-v031 control (cells
and Voronoi share at game turns 10 and 12 within a cell), and lose more
units: by game turn 14, t3 as blue 36.9 units' worth lost against 26.7 for
the control, t4 33.2. The rush does not win the centre; it costs units.

**Positions** (the analysis mode `guide`, new in this entry: 1,629
positions of game turns 7 to 14 from the control's games, a fresh role
memory each; each plan played, then the opponent's answer by the search
from its seat at 1,000 tries; read at the start of our next turn). In the
1,094 positions with a non-empty strategic turn:

| Plan | Units lost | Of which cut off | Cell lead | Objectives held | Gold spent | HQ lost | Moves | Left cells the enemy then holds |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| The search's plan | 0.32 | 0.19 | −2.3 | 6.80 | 23.2 | 3 | 4.27 | 0.16 (4% of moves) |
| The strategic turn alone | 2.23 | 2.02 | −10.1 | 5.11 | 13.1 | 31 | 2.52 | 0.86 (34%) |
| The strategic turn, then the search's plan from there | 1.27 | 1.07 | −4.3 | 6.61 | 26.0 | 6 | 4.81 | 0.73 (15%) |

(Cell lead: our active cells less the enemy's. Objectives held: counted
objectives at the position that are our active cells after the answer.)

So the strategic turn loses units by cut-offs, not fights, and holds no
more objectives even when the search completes it: a third of its moves
take a frontline unit off a border cell the enemy then enters, cutting off
what lies behind. The race counts who arrives first, not whether the line
behind the arriving unit holds; E070's step and guard (a step with the cell
left guarded, +71.9) is the safe version of the same move.

## Round 2's rules, measured on the positions

The owner chose all four fixes (form, 2026-11-20): guarded steps, back
units walking, filling behind, a safe-step filter (`strategy::GUARD`,
`BACK_ONLY`, `FILL_BEHIND`, `SAFE_STEP`; the analysis mode `guide` takes
them as `--guard N`, `--back-only`, `--fill-behind`, `--safe-step`). On
the same positions (the strategic turn completed by the search's plan from
there, as the forced first climb does; with `--back-only` fewer positions
have a strategic turn, 712, and the search's plan is compared on those):

| Rules | Positions | Moves whose cell the enemy then holds | Units lost | Cut off | Cell lead | Objectives held | The search's plan: lost / lead / objectives |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| none (round 1) | 1,094 | 34% | 1.27 | 1.07 | −4.3 | 6.61 | 0.32 / −2.3 / 6.80 |
| guard: a tower | 1,094 | 7% | 0.71 | 0.58 | −4.4 | 6.72 | same |
| guard: a training | 1,094 | 14% | 1.39 | 1.01 | −4.8 | 6.43 | same |
| fill behind | 1,094 | 17% | 1.19 | 0.89 | −4.6 | 6.39 | same |
| safe step | 1,094 | 14% | 0.99 | 0.69 | −4.0 | 6.61 | same |
| a tower, safe step | 1,094 | 3% | 0.68 | 0.54 | −4.2 | 6.73 | same |
| back units only | 712 | 13% | 0.60 | 0.44 | −2.8 | 6.52 | 0.19 / −1.6 / 6.43 |
| back units, safe step | 712 | 5% | 0.43 | 0.27 | −2.6 | 6.52 | same |

(The first column is for the strategic turn alone; "back units, a
training" plays as "back units only", since back units leave no exposed
cell.) Each fix halves the losses or more, and back units with the safe
step hold more objectives than the search's plan, but none reaches its
units or its cell lead.

### Round 2

Seed 2026112103, 800 pairs a variant over 2 calibrated runners, against
v031, cut at game turn 12 (run 44835317605) and full games (run
44835320532):

| Variant | Switches | Cut at turn 12 | Full games |
| --- | --- | --- | --- |
| t2bs | `TURN = 2`, `BACK_ONLY`, `SAFE_STEP` | −3.9 [−18.2, +10.4] | +14.8 [+1.0, +28.6] |
| t4bs | `TURN = 4`, `BACK_ONLY`, `SAFE_STEP` | −10.9 [−23.2, +1.5] | −4.3 [−16.0, +7.3] |
| t1bs | `TURN = 1`, `BACK_ONLY`, `SAFE_STEP` | +2.2 [−9.6, +13.9] | −5.2 [−17.4, +7.0] |
| t7bs | `TURN = 7`, `BACK_ONLY`, `SAFE_STEP` | −7.4 [−21.5, +6.7] | +7.8 [−5.9, +21.6] |
| t2gs | `TURN = 2`, `GUARD = 1`, `SAFE_STEP` | −11.3 [−25.2, +2.6] | ±0.0 [−13.5, +13.5] |
| t4gs | `TURN = 4`, `GUARD = 1`, `SAFE_STEP` | −7.4 [−20.2, +5.4] | −11.7 [−24.4, +0.9] |
| t2g | `TURN = 2`, `GUARD = 1` | −19.1 [−33.2, −5.1] | −3.5 [−17.5, +10.6] |
| t4g | `TURN = 4`, `GUARD = 1` | −4.3 [−17.7, +9.0] | −17.8 [−30.8, −4.9] |

All faults 0. In full games, the first climb starting with the strategic
turn of the back units with the safe step (t2bs) leads at +14.8, the only
interval above 0, and the same rules in all three ways (t7bs) read +7.8;
the tower guard is level at best (t2gs ±0.0) and loses as pieces (t4g
−17.8). Cut at game turn 12, no variant gains (t1bs +2.2 best, t2bs −3.9):
the cut duels, which end two turns before the layer does, did not track
the full games in either round. After 16 variants, the leader needs new
seeds before any SPRT.

## Timing (after round 2)

The owner asked (2026-11-20, after round 2: t2bs +14.8 in full games,
−3.9 cut at game turn 12) to analyse the timing, with visual examples, and
suggested doing the early game first, so that the rush comes before the
defence. All plies of the control's 120 games (v031 against v031, seed
2026112102; `ice-and-fire-review bank --all-plies`, new in this entry), read
with the bot's `strategy` analysis mode: per position, the objectives of
value above 0 and their races.

| Game turn | 1 | 2 | 4 | 6 | 7 | 8 | 9 | 10 | 12 | 14 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Red: objectives we reach first | 18.2 | 19.1 | 20.6 | 20.0 | 17.4 | 13.2 | 8.9 | 7.5 | 4.6 | 2.8 |
| Red: their summed value | 468 | 358 | 280 | 159 | 94 | 75 | 50 | 57 | 26 | 10 |
| Red: held by us | 0 | 0 | 0 | 0 | 0.8 | 2.5 | 3.8 | 5.2 | 6.8 | 6.7 |
| Blue: objectives we reach first | 18.2 | 26.9 | 21.3 | 21.9 | 17.1 | 14.6 | 10.0 | 6.5 | 3.9 | 2.6 |
| Blue: their summed value | 400 | 506 | 231 | 135 | 87 | 75 | 55 | 31 | 23 | 8 |
| Blue: held by us | 0 | 0 | 0 | 0.5 | 2.0 | 4.0 | 5.4 | 6.1 | 6.9 | 7.2 |

(Value: the land each objective would add to our side, summed; objectives
overlap, so the sums are a trend.) The count of objectives we can still
reach first holds until game turn 6, but their value falls by two thirds
between game turns 1 and 6: the land that decides most is decided in the
opening, which E077 does not touch (it starts at game turn 7, when about a
fifth of the first turn's value is left). Boards: the race over one game,
and three game-turn-7 positions played by the search, round 1's strategic
turn and round 2's rules completed by the search, at
https://claude.ai/artifact/5qfr4eKjZJF83y9QZTA6dH.
