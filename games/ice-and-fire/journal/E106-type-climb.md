---
id: E106
date: 2026-11-28
pull_request: "#406"
parent: ice-and-fire-v034
hypothesis: zasmu's unit-type tables hold 91% of its unit actions in each unit's three likeliest kinds; offering our climbs only the moves of each unit's likeliest kinds, in the tables' order, spends the search on the moves a strong player makes and beats ice-and-fire-v034.
change: Switches in our own climbs (never the opponent's model), from game turn 7: each unit offered only the moves of its type's k likeliest kinds (`TYPE_KINDS`), and moves tried in the tables' order among actions of equal last gain (`TYPE_ORDER`); left off, with the screening report's new land, units and crowding measures.
release: none
sprt: not run: every pruning variant lost in screening, the order alone level
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

Stage 2 of the type planner
([2026-11-28-type-planner.md](../research/codingame/2026-11-28-type-planner.md)),
whose stage 1 (#405) put zasmu's action tables in the plan bot: from game
turn 5, on held-out games, the three likeliest kinds of a unit's type hold
90.8% of zasmu's unit actions, the four likeliest 95.8%. The owner chose
(2026-11-28) to prune our own climb with them, since pruning our options
cannot hide a danger from us, and to order the moves by them.

## Change

`bots/plan/src/search.rs` and `search/steer.rs`, in our climbs only (the
opponent's model is built with `opponent: true` and is never touched), when
the climb's root is at game turn `TYPE_FROM_TURN` (7) or later; one
`Typing` (`types.rs`) per climb step:

- `TYPE_KINDS` (`None`): an `Action::One(Command::Move)` is kept only if its
  kind is among the k kinds of highest prior for the unit's key; trainings,
  buildings and macro actions (cuts, the lethal chain, blockers, the
  strategic turn's pieces) are always kept; the cached root actions are
  filtered in a copy, and the forced climbs' first actions too.
- `TYPE_ORDER` (false): among actions of equal last gain, moves by their
  kind's prior, highest first, then the other actions in generated order.

`mod types;` is no longer behind the analysis feature. With both switches
off the bot gives the same answers as v034 (`judge`, 600 tries, on the 2,031
positions of v033's 159 CodinGame ranking games: byte-identical output), but
its bundle grows from 69,361 to 83,969 characters, the module and its table
being compiled in.

The screening report (`ice-and-fire-review screening`) gains, per bot, by
game turn: the median land lead at game turn 12, the units at turns 7 and
12, and from turn 5 to 12 the own units next to each unit and the share of
units with no free neutral neighbour (`types.rs`'s `adj` and `free1`).

## Local measures

The time a search takes at 600 tries on the speed bank (median ms, one
core): off 11.35, `TYPE_ORDER` alone 11.76, k = 4 12.38, k = 3 12.55,
k = 2 14.34 (with the order): the typing at every step costs more than the
pruning saves.

Twenty local pairs of k = 3 against v034 at CodinGame's speed: −88.7
[−184.0, −5.2]; median land lead at game turn 12 −4 (v034 0); units at turn
12 6.85 (7.17); the share of units with no free neutral neighbour at turns
8 to 12 23, 36, 47, 50, 55% (v034 18, 37, 47, 45, 48%): no less crowded.

## Screening

Variants against `ice-and-fire-v034`, 1,600 seat-swapped pairs each, whole
games at CodinGame's limits, 4 shards
([run 46131712007](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46131712007)):

| Variant | `TYPE_KINDS` | `TYPE_ORDER` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `control` | none | off | −1.7 [−7.5, +4.0] |
| `order` | none | on | +3.0 [−6.5, +12.5] |
| `k4` | 4 | on | −33.8 [−43.9, −23.6] |
| `k3` | 3 | on | −75.2 [−86.1, −64.5] |
| `k2` | 2 | on | −175.5 [−188.3, −163.2] |

Commits: `control` `0122aa3`, `order` `5f0bf65`, `k4` `e372c1c`, `k3`
`35de20b`, `k2` `4c8d1bf`. No fault in any variant.

## What was learned

Dropped: the fewer kinds kept, the more it loses, and the order alone gains
nothing. A diagnosis after the screening, on our own 95,099 unit decisions
from game turn 7 in the 1,038 games against zasmu: zasmu's three likeliest
kinds already hold 87.7% of our own unit actions (zasmu's own: 90.8%),
93.3% at game turns 7 and 8, 80.6% at turns 9 to 12, 74.8% for our guards
and 79.7% for our reserves. So by kind of action our units already play
much as zasmu's do; the 12% outside are the moves our own evaluation finds
(staying 28% of them, stepping away from the enemy 32%, sideways 25%), and
removing them costs our search what it was finding, while zasmu's choices
rest on its own search and evaluation. The difference with zasmu is not in
the kinds of single units' actions but in where they go together, how many
units there are and how the gold is spent (crowding from game turn 4, one
unit fewer from turn 6, half the mines by turn 16): the matching, the start
at game turn 5 and the spending of the design's stages 4 to 6.

The owner chose (2026-11-28) to keep the switches, off, and the new
measures on `main` for the later stages.
