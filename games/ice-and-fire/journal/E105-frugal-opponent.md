---
id: E105
date: 2026-11-28
pull_request: "#404"
parent: ice-and-fire-v034
hypothesis: The opponent's climb, the reply that decides most of the search's scores, spends far more on trainings than real opponents do (zasmu above all) and misses replies made of moves; a model that trains at most two units, or places its moves and buildings before training, predicts real replies better and beats ice-and-fire-v034.
change: Switches in the opponent's climb that leave v034 as it plays; variants where it may train at most two units (`OPP_MAX_TRAINS`), where it trains only once its moves, towers and mines gain nothing more (`OPP_MOVES_FIRST`), and both.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

After E103 (a lazier opponent's climb, −73.5) and E104 (a stronger one,
level), the owner asked for an opponent model like zasmu's play, perhaps by
roles (2026-11-27). zasmu won 213 of its 232 collected games against our
bot. On the 3,648 positions of v032's 235 test games against it, the reply
the search models for the opponent (its climb, 300 plans scored, E019) was
compared with the reply zasmu played: the same commands, the kinds of
action, each of its units' action by role, and our evaluation after each.
The same was done on all 29,545 positions of v032's collected games.

### The model against the real replies

| | zasmu | All opponents |
| --- | ---: | ---: |
| The same reply | 2.3% | |
| Commands shared (of either's) | 0.26 | |
| Modeled reply 50 or more worse for us | 26.8% | 15.9% |
| Real reply 50 or more worse for us (a miss) | 9.2% | 6.1% |
| Trainings a reply, real against modeled | 1.19 / 2.15 | 0.96 / 1.46 |
| Towers a reply, real against modeled | 0.64 / 0.16 | 0.52 / 0.18 |
| Mines a reply, real against modeled | 0.25 / 0.03 | 0.18 / 0.04 |

Both gaps grow with the opponent's strength: against the opponents who beat
v032 three times in four or more (zasmu alone), the model was 50 or more too pessimistic
in 26.8% of the positions and missed in 9.2%; against those who lost more
than they won, 12.8% and 4.8%.

Real opponents train in 45% of their replies and three units or more in
10%; the model trains in 68% and three or more in 18%. Our own bot, in the
same games, made 0.84 trainings, 0.58 towers and 0.11 mines a turn: it is
the model that is spendthrift, not our play. The opponent's climb looks
one move ahead with no reply after it, so a tower or a mine is worth
little to it and gold spent on trainings is always worth something.

### Unit by unit, by role

Each of zasmu's units was given a role on the position (a pusher next to
our land; a guard on or next to one of its joints within 3 steps of us,
the cells whose loss cuts land off its HQ; a reserve within 3 steps; else
back), and its action in each reply was read as a capture, a step towards
our land, sideways, away from it, or none:

| Role | Units | Same action | Away (real / modeled) | Towards (real / modeled) |
| --- | ---: | ---: | ---: | ---: |
| Pusher | 10,023 | 66% | 12% / 12% | captures 23% / 25% |
| Guard | 2,629 | 66% | 27% / 29% | 38% / 35% |
| Reserve | 3,245 | 60% | 30% / 20% | 34% / 42% |
| Back | 6,878 | 57% | 32% / 21% | 36% / 44% |

The pushers and guards are modeled about right; the units behind them
take neutral land away from us in zasmu's play, where the model sends them
forward. 43% of zasmu's towers stand on or next to its joints, nearly all
within 3 steps of us.

### Where the misses come from

In the largest group of the misses (258 of 1,800 in all the games: four
trainings or more modeled, none real), the real reply trained nobody while
the modeled one trained four units or more: our evaluation rates the real
reply worse for us, so the climb had a better reply in reach and missed
it, having started down the trainings. The misses come with more gold: 99
on average against 57 elsewhere in all the games, 136 against 79 (the
positions within 50) against zasmu.

## Change

`bots/plan/src/search.rs`, the opponent's climb only (`Search::opponent`):

- `OPP_MAX_TRAINS` (`None`): the most trainings in its plan; actions that
  would train more are not offered.
- `OPP_MOVES_FIRST` (false): its climb is offered no training until its
  moves, towers and mines gain nothing more; then it climbs on with every
  action.

With both off the bot plays as v034 (nothing is filtered).

## Local measures

The same comparison on zasmu's 3,648 positions, for each variant's
opponent climb:

| Variant | Misses | Modeled 50+ worse | Shared | Trainings | Towers | Mines |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| v034 (off) | 9.2% | 26.8% | 0.26 | 2.15 | 0.16 | 0.03 |
| `frugal` (2 at most) | 11.9% | 25.6% | 0.24 | 1.09 | 0.20 | 0.06 |
| `movesfirst` | 8.5% | 27.4% | 0.27 | 1.96 | 0.36 | 0.07 |
| `both` | 11.6% | 26.0% | 0.25 | 0.99 | 0.37 | 0.08 |

The cap brings the trainings down to zasmu's, but the model then misses
more of the replies that hurt us and is hardly less pessimistic: the
pessimism does not come from the spending. Moving first misses a little
less and builds twice the towers.

## Screening

Variants against `ice-and-fire-v034`, 2,000 seat-swapped pairs each, whole
games at CodinGame's limits, 5 shards, with a control that has every
switch off
([run 46118921865](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46118921865)):

| Variant | `OPP_MAX_TRAINS` | `OPP_MOVES_FIRST` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `control` | none | off | −1.0 [−5.8, +3.7] |
| `frugal` | 2 | off | −9.4 [−16.9, −1.9] |
| `movesfirst` | none | on | −5.7 [−14.8, +3.3] |
| `both` | 2 | on | −9.0 [−18.1, +0.0] |

Commits: `control` `2d08682`, `frugal` `5d42712`, `movesfirst` `99d751d`,
`both` `2c2e977`. No fault in any variant.

## What was learned

Dropped. Every variant loses a little to v034, and the frugal ones lose
the most, as their misses foretold: capping the opponent's trainings
hides from the search the attacks that cost it most, even though real
opponents rarely make them. Moving first, which missed a little less,
does not lose clearly but gains nothing. Restricting what the opponent's
climb may do, by spending or by order, makes it weaker before it makes it
more realistic, as in E103.

The study says where a better model must come from: the climb is
pessimistic about the spending of real opponents because it looks one
move ahead, and the units behind the front are the ones it models worst.
The owner proposed (2026-11-28) a model by unit type: each unit's type
first, then for each type a set rule or a set of likely actions, by the
phase of the game and other measures, for both sides; it is the next
step to design with him.

The code stays reachable on the branch
`claude/ice-and-fire-e105-frugal-opponent` (the switches `OPP_MAX_TRAINS`
and `OPP_MOVES_FIRST`, commit `1b16bfa`).
