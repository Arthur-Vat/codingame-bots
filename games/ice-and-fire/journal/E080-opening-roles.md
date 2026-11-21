---
id: E080
date: 2026-11-21
pull_request: "#NN"
parent: ice-and-fire-v031
hypothesis: An opening that may also play a roles turn (units walk to targets, trainings at the tips of our land after the moves), with targets chosen so that our side holds more land than the enemy after the early game, takes the centre before game turn 7, where the race is mostly decided, and beats ice-and-fire-v031 (Step 7 of the frontline plan).
change: The opening's beam may play, beside its 27 weight sets, the roles step (`ice_and_fire_opening::frontline::step`), by its targets (`Value`, `Matched`, `Frontier` or `Marginal`), the rest of the turn by the `econ` rules; and it may rank its plans by the most cells then the Voronoi share (`Order::Voronoi`); the bot's switch `OPENING_ROLES`.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E077's timing ([E077](E077-strategic-turn.md), "Timing"): the value of the
objectives we can still reach first falls by two thirds between game
turns 1 and 6, so the race for the centre is mostly decided in the
opening. The owner (forms, 2026-11-21) chose to park E077 and do the early
game first: "Roles in the opening. Also make sure to target relevant cells
(in the first screenshot we see that swapping objectives could have been
better), also spawn from the tip. Make multiple variants. The choice of
target is not obvious at all, the final goal is to be able to have more
territory (maybe max number of cells, then voronoi share) than the
opponent after the early game."

## Change

- The objectives, the race and the roles of Steps 2 and 3 move from the
  plan bot to the opening crate (`ice_and_fire_opening::frontline`), with
  the map's band and middle chokes (`frontline::goal_zones`, as the bot's
  `eval::goal_zones`).
- **The roles step** (`frontline::step`): each turn, our movable units and
  the trainings the gold pays are agents; they take targets; units step one
  cell nearer their target (a cell not ours first); then each training goes
  on our trainable cell nearest its target, after the moves, so at the tip
  of the land a unit has just taken; then, with `fill`, the `econ` turn of
  the first weight set with the payback rule plays the rest (units with no
  target, gold left). An agent takes a target only where it arrives no
  later than the enemy's land. The targets:
  - `Value`: the objectives by the land each gains, given out greedily by
    the roles (value, then arrival);
  - `Matched`: the same objectives, the most valuable first, matched to the
    agents by the Hungarian method for the least total arrival, so that two
    units do not cross to each other's target (the owner's first board);
  - `Frontier`: E072's targets (middle chokes, then spaced cells along the
    Voronoi frontier), matched the same way;
  - `Marginal`: objectives picked one at a time by the land each adds to
    that of the ones already picked (their values overlap), then matched.
- **In the opening's beam** (`solo::steps`): with `Objective::roles`, the
  roles step is one more step beside the weight sets, so it plays only
  when the beam rates its plan best (the opponent passing, as for every
  step). `Order::Voronoi` (the most active cells, then the Voronoi share)
  is the owner's ranking; E073's `Order::Central` is v031's.
- The bot: `OPENING_ROLES` (`None` in v031). With it off the bot plays as
  ice-and-fire-v031 (20 games against v030 at fixed tries, the same
  results and lengths); the bundle is 88.2k characters (v031's 59.2k
  plus E077's switched-off strategy and the opening's new code).

## The bench

`oracle --bench` on the 78 stored starts
([research/codingame/2026-11-20-optimal-early-plans](../research/codingame/2026-11-20-optimal-early-plans/)):
cells short of the most a plan can hold, the opponent passing, by the
number of our turns played (lower is better; `lin7` is v031's opening,
`voronoi` the owner's order alone):

| Turns | Seat | lin7 | voronoi | lin7 + Value | + Matched | + Frontier | + Marginal | + Matched, no fill | voronoi + Value | + Matched | + Frontier | + Marginal |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | red | 0.09 | 0.11 | 0.09 | 0.09 | 0.14 | 0.09 | 0.30 | 0.09 | 0.09 | 0.16 | 0.11 |
| 6 | blue | 0.09 | 0.09 | 0.03 | 0.06 | 0.21 | 0.15 | 0.71 | 0.06 | 0.06 | 0.06 | 0.12 |
| 7 | red | 0.25 | 0.25 | 0.25 | 0.25 | 0.34 | 0.25 | 0.73 | 0.25 | 0.25 | 0.41 | 0.25 |
| 7 | blue | 0.18 | 0.18 | 0.12 | 0.18 | 0.35 | 0.29 | 0.97 | 0.18 | 0.18 | 0.26 | 0.26 |
| 8 | red | 0.27 | 0.25 | 0.30 | 0.30 | 0.41 | 0.30 | 0.98 | 0.30 | 0.32 | 0.50 | 0.27 |
| 8 | blue | 0.18 | 0.12 | 0.26 | 0.32 | 0.41 | 0.44 | 1.29 | 0.18 | 0.21 | 0.35 | 0.38 |

With the `econ` fill, no target choice costs more than about 0.25 cells
against the opponent passing; without it the roles step loses 0.3 to 1.3
cells (units with no target wait, gold stays). The bench cannot show what
the roles step is for, the race against a moving enemy: that is for the
cut duels and full games.
