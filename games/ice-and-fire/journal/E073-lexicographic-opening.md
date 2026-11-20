---
id: E073
date: 2026-11-20
pull_request: "#NN"
parent: ice-and-fire-v029
hypothesis: Among the many opening plans that reach the most land, choosing those that hold the most middle chokes, then the most cells of the middle band (step after the study of optimal early plans), wins the duels cut at game turn 12 against v029 without giving up land.
change: The solo opening's beam ranks positions lexicographically (`solo::Order`) instead of by its weighted sum alone.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The study of optimal early plans
([2026-11-19-optimal-early-plans.md](../research/codingame/2026-11-19-optimal-early-plans.md))
found that v029's opening reaches the most land at turn 6 on about 80% of
starts, while plans with the same land hold 1.9 to 7 times its middle chokes
and 1.45 to 1.9 times its middle cells. The owner chose a lexicographic
choice in the opening as the next step (2026-11-20).

## Change

`Objective::order` (`games/ice-and-fire/opening/src/solo.rs`) sets how the
beam compares the positions its plans reach at the end of their 9 turns
(`solo::key`):

- `Sum`: the weighted sum (v029);
- `TieBreak { within }`: the weighted sum in steps of `within`, then the
  middle chokes held, then the cells of the middle band (within 2 steps of
  even between the HQs, as the study counts them), then the sum;
- `LandFirst`: the active cells, then the middle chokes, then the middle
  band, then the weighted sum.

The plan bot sets it in `ORDER`.

## Bench against the stored best plans

`oracle --bench` on the study's 78 starts
(`research/codingame/2026-11-20-optimal-early-plans/oracle.plans.tsv`);
each opening untimed, beams up to 32. Cells short of the most, then middle
chokes and middle cells short of the most at its own cells:

| Turns | Seat | Opening | Cells short of the most | At the most cells | Chokes short at its cells | Middle cells short at its cells |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| 6 | red | v029 | 0.18 | 82% | 0.71 | 0.00 |
| 6 | red | tie1 | 0.18 | 82% | 0.71 | 0.00 |
| 6 | red | land | 0.09 | 91% | 0.64 | 0.00 |
| 6 | red | tie6 | 0.18 | 82% | 0.71 | 0.00 |
| 6 | red | tie12 | 0.18 | 82% | 0.71 | 0.00 |
| 6 | blue | v029 | 0.26 | 79% | 1.42 | 0.94 |
| 6 | blue | tie1 | 0.26 | 79% | 1.42 | 0.94 |
| 6 | blue | land | 0.09 | 94% | 0.95 | 0.44 |
| 6 | blue | tie6 | 0.26 | 76% | 1.37 | 0.85 |
| 6 | blue | tie12 | 0.26 | 74% | 1.11 | 0.71 |
| 7 | red | v029 | 0.59 | 68% | 1.68 | 1.07 |
| 7 | red | tie1 | 0.59 | 68% | 1.68 | 1.07 |
| 7 | red | land | 0.20 | 89% | 1.67 | 0.60 |
| 7 | red | tie6 | 0.59 | 66% | 1.68 | 1.07 |
| 7 | red | tie12 | 0.59 | 68% | 1.68 | 1.05 |
| 7 | blue | v029 | 0.50 | 68% | 2.33 | 1.67 |
| 7 | blue | tie1 | 0.50 | 68% | 2.33 | 1.67 |
| 7 | blue | land | 0.18 | 88% | 1.67 | 0.70 |
| 7 | blue | tie6 | 0.41 | 76% | 2.11 | 1.55 |
| 7 | blue | tie12 | 0.44 | 71% | 1.83 | 1.39 |
| 8 | red | v029 | 1.30 | 36% | 1.64 | 2.62 |
| 8 | red | tie1 | 1.20 | 39% | 1.71 | 2.67 |
| 8 | red | land | 0.23 | 84% | 1.54 | 1.18 |
| 8 | red | tie6 | 1.18 | 36% | 1.58 | 2.72 |
| 8 | red | tie12 | 1.11 | 43% | 1.62 | 2.67 |
| 8 | blue | v029 | 1.35 | 41% | 1.77 | 3.69 |
| 8 | blue | tie1 | 1.35 | 41% | 1.77 | 3.69 |
| 8 | blue | land | 0.15 | 85% | 0.82 | 1.22 |
| 8 | blue | tie6 | 1.21 | 35% | 1.07 | 3.45 |
| 8 | blue | tie12 | 1.32 | 26% | 0.40 | 3.14 |

`TieBreak { within: 1 }` (exact ties) plays as v029 almost everywhere: the
weighted sum seldom ties. Wider steps close part of the gap, mostly as blue.
`LandFirst` reaches more land at every horizon (at the most land on 84 to
94% of starts against 36 to 82%) and comes nearest the best plans in middle
chokes and middle cells.

## Duels cut at game turn 12

`ice-and-fire-arena-early` against `ice-and-fire-v029` (its release
compiled with `rustc -C opt-level=3`), 400 pairs, seed 72, both seats,
time scale 0.403 and 5 ms of tolerance from `scripts/calibrate.sh` on the
session's 4 CPUs; the variants built with `cargo build --release`, `ORDER`
changed.

| Variant | Order | Elo over v029 [95%] | Pairs won / level / lost | Timeouts (variant / v029) |
| --- | --- | --- | --- | ---: |
| sum | `Sum` (v029's opening: control) | −1.7 [−14.3, +10.8] | 26 / 346 / 28 | 1 / 0 |
| land | `LandFirst` | +20.9 [−1.4, +43.3] | 98 / 228 / 74 | 2 / 4 |
| tie12 | `TieBreak { within: 12 }` | +20.9 [+2.6, +39.2] | 70 / 284 / 46 | 0 / 0 |
| tie6 | `TieBreak { within: 6 }` | +5.2 [−11.1, +21.6] | 49 / 308 / 43 | 0 / 1 |

The control plays as v029. `land` changes the most games (a pair
split in 172 of 400 against 54 for the control) and wins more than it
loses; `tie12` changes fewer and wins as much. Median answer times are
level (9 to 10 ms after the first turn).

## Several middle chokes in different places

The owner's question (2026-11-20): when the middle chokes lie in several
places, does the opening reach more than one? `opening/examples/places.rs`
groups a map's middle chokes into places (two chokes within 2 steps are in
the same place) and plays each opening, one side alone, the other passing,
for 10 of our turns. Starts: the 7 of the study's 78 with 2 or 3 places,
and 40 more maps from fresh seeds with 2 or 3 places, both seats: 87
starts. Every map found has exactly 2 places of the same size (the maps
are point-symmetric, so the middle chokes come in mirrored pairs, one on
each flank; 3 would need one on the centre): 33 of 1 choke each, 24 of 2,
12 of 3, 3 of 4, 15 of 5.

| Opening | Our turn | Places held (share) | Starts holding every place |
| --- | ---: | ---: | ---: |
| v029 | 4 | 0 (0%) | 0 of 87 |
| v029 | 5 | 1 (1%) | 0 of 87 |
| v029 | 6 | 9 (5%) | 0 of 87 |
| v029 | 7 | 42 (24%) | 1 of 87 |
| v029 | 8 | 72 (41%) | 5 of 87 |
| v029 | 9 | 96 (55%) | 12 of 87 |
| v029 | 10 | 115 (66%) | 28 of 87 |
| tie12 | 4 | 0 (0%) | 0 of 87 |
| tie12 | 5 | 2 (1%) | 0 of 87 |
| tie12 | 6 | 16 (9%) | 0 of 87 |
| tie12 | 7 | 53 (30%) | 2 of 87 |
| tie12 | 8 | 89 (51%) | 6 of 87 |
| tie12 | 9 | 105 (60%) | 18 of 87 |
| tie12 | 10 | 119 (68%) | 32 of 87 |
| land | 4 | 0 (0%) | 0 of 87 |
| land | 5 | 4 (2%) | 0 of 87 |
| land | 6 | 22 (13%) | 0 of 87 |
| land | 7 | 64 (37%) | 5 of 87 |
| land | 8 | 110 (63%) | 27 of 87 |
| land | 9 | 142 (82%) | 55 of 87 |
| land | 10 | 150 (86%) | 63 of 87 |

By seat, places held after our turn 7 (v029 / tie12 / land): red 12 / 17 /
23 of 86, blue 30 / 36 / 41 of 88; after turn 8 (v029 / land): red 30 / 48,
blue 42 / 62.

So v029's opening mostly reaches one flank: by our turn 8 it holds both
places on 5 of 87 starts, and by turn 10 on 28. `land` reaches both
sooner (27 of 87 by turn 8, 63 by turn 10) and holds more places at every
turn; `tie12` is between. The bot hands over to the middle-game search at
game turn 7, so turns 8 to 10 here show where the opening's plans lead,
not what the bot plays.

### Three places and more

The owner then asked about three places. `places census 20000` groups the
middle chokes of 20,000 maps from the same seed sequence: no place 8,429
maps (42%), one 9,186 (46%), two 2,335 (12%), three none, four 50
(0.25%). Three never occurs: the maps are point-symmetric, so a place off
the centre has its mirror, and a place on the centre would merge with
nothing else to make three; the four-place maps are two mirrored pairs.

On the 50 four-place maps, both seats (100 starts), one side alone:

| Opening | ≥ 2 places at our turns 7 / 8 / 10 | ≥ 3 places at turns 7 / 8 / 10 | Places held at turn 8 / 10 (of 400) |
| --- | --- | --- | --- |
| v029 | 11 / 48 / 99 | 1 / 9 / 36 | 156 / 235 |
| tie12 | 17 / 77 / 100 | 2 / 15 / 40 | 192 / 241 |
| land | 21 / 86 / 100 | 1 / 33 / 85 | 219 / 295 |

With four places, `land` spreads to three of them by turn 10 on 85 of 100
starts, against 36 for v029; few starts hold all four (10 for `land`,
none for v029), since the far pair lies nearer the enemy.

## The owner's evaluation: land, then central cells

The owner, 2026-11-20, after the boards: "the evaluation function should
be different. Central cells should have more weight, then cells just next
to them slightly less, and so on. We should value the total number of
cells first, then the total weighted value of cells." His answers to the
details (form): central means even between the HQs; try several decays;
counted at the end of the 9-turn plans, as before; the current weighted
sum breaks the remaining ties.

`Order::Central(decay)` ranks a position by its active cells, then the
sum of their weights, then the weighted sum. A cell's ring (`solo::rings`)
is half the difference of its steps to the two HQs: always even on this
grid, whose two colours alternate, so each step away from the cells even
between the HQs adds a ring. Weights (`Decay`): `Linear(m)` m − ring,
at least 0; `Halving(m)` 2^(m − ring), 0 beyond m rings; `Square(m)`
(m − ring)², 0 beyond.

Bench, as above (`land` for comparison):

| Turns | Seat | Opening | Cells short of the most | At the most cells | Chokes short at its cells | Middle cells short at its cells |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| 6 | red | v029 | 0.18 | 82% | 0.71 | 0.00 |
| 6 | red | land | 0.09 | 91% | 0.64 | 0.00 |
| 6 | red | lin3 | 0.07 | 93% | 0.71 | 0.00 |
| 6 | red | lin6 | 0.09 | 91% | 0.68 | 0.00 |
| 6 | red | lin10 | 0.09 | 91% | 0.68 | 0.00 |
| 6 | red | half4 | 0.09 | 91% | 0.68 | 0.00 |
| 6 | red | half8 | 0.09 | 91% | 0.68 | 0.00 |
| 6 | red | sq6 | 0.09 | 91% | 0.68 | 0.00 |
| 6 | blue | v029 | 0.26 | 79% | 1.42 | 0.94 |
| 6 | blue | land | 0.09 | 94% | 0.95 | 0.44 |
| 6 | blue | lin3 | 0.09 | 94% | 0.95 | 0.18 |
| 6 | blue | lin6 | 0.09 | 94% | 1.16 | 0.29 |
| 6 | blue | lin10 | 0.09 | 94% | 1.05 | 0.29 |
| 6 | blue | half4 | 0.09 | 94% | 0.89 | 0.18 |
| 6 | blue | half8 | 0.09 | 94% | 0.74 | 0.06 |
| 6 | blue | sq6 | 0.09 | 94% | 0.89 | 0.15 |
| 7 | red | v029 | 0.59 | 68% | 1.68 | 1.07 |
| 7 | red | land | 0.20 | 89% | 1.67 | 0.60 |
| 7 | red | lin3 | 0.20 | 89% | 2.00 | 0.35 |
| 7 | red | lin6 | 0.25 | 86% | 1.89 | 0.30 |
| 7 | red | lin10 | 0.25 | 86% | 1.89 | 0.30 |
| 7 | red | half4 | 0.25 | 86% | 1.93 | 0.35 |
| 7 | red | half8 | 0.25 | 86% | 1.85 | 0.33 |
| 7 | red | sq6 | 0.25 | 86% | 1.93 | 0.33 |
| 7 | blue | v029 | 0.50 | 68% | 2.33 | 1.67 |
| 7 | blue | land | 0.18 | 88% | 1.67 | 0.70 |
| 7 | blue | lin3 | 0.18 | 88% | 1.83 | 0.33 |
| 7 | blue | lin6 | 0.18 | 88% | 2.00 | 0.36 |
| 7 | blue | lin10 | 0.18 | 88% | 1.89 | 0.33 |
| 7 | blue | half4 | 0.21 | 85% | 1.78 | 0.33 |
| 7 | blue | half8 | 0.21 | 85% | 1.67 | 0.21 |
| 7 | blue | sq6 | 0.21 | 85% | 1.78 | 0.33 |
| 8 | red | v029 | 1.30 | 36% | 1.64 | 2.62 |
| 8 | red | land | 0.23 | 84% | 1.54 | 1.18 |
| 8 | red | lin3 | 0.25 | 84% | 2.14 | 0.89 |
| 8 | red | lin6 | 0.27 | 84% | 2.07 | 0.80 |
| 8 | red | lin10 | 0.27 | 84% | 2.07 | 0.77 |
| 8 | red | half4 | 0.27 | 84% | 2.11 | 0.86 |
| 8 | red | half8 | 0.30 | 82% | 2.04 | 0.82 |
| 8 | red | sq6 | 0.27 | 84% | 2.14 | 0.86 |
| 8 | blue | v029 | 1.35 | 41% | 1.77 | 3.69 |
| 8 | blue | land | 0.15 | 85% | 0.82 | 1.22 |
| 8 | blue | lin3 | 0.15 | 85% | 1.06 | 0.69 |
| 8 | blue | lin6 | 0.18 | 85% | 0.88 | 0.72 |
| 8 | blue | lin10 | 0.21 | 82% | 0.94 | 0.69 |
| 8 | blue | half4 | 0.18 | 82% | 0.94 | 0.81 |
| 8 | blue | half8 | 0.21 | 82% | 0.94 | 0.78 |
| 8 | blue | sq6 | 0.18 | 82% | 0.94 | 0.78 |

The central orders keep `land`'s land and come much nearer the best plans'
middle band (blue at turn 7: 0.21 to 0.36 middle cells short against 0.70
for `land` and 1.67 for v029), but no nearer their middle chokes, which
they do not count. The decays differ little.

### Duels of the central orders

As above (400 pairs, seed 72, scale 0.403, tolerance 5 ms). Runs marked
* played while the session ran other work on the same CPUs, as their
timeouts show: read them as noisier.

| Variant | Order | Elo over v029 [95%] | Timeouts (variant / v029) |
| --- | --- | --- | ---: |
| lin3* | `Central(Linear(3))` | −6.9 [−29.1, +15.1] | 10 / 6 |
| lin5* | `Central(Linear(5))` | +2.6 [−19.8, +25.0] | 10 / 11 |
| lin7 | `Central(Linear(7))` | +34.9 [+11.9, +58.1] | 0 / 0 |
| lin9 | `Central(Linear(9))` | +26.1 [+2.4, +50.1] | 1 / 0 |
| lin10 | `Central(Linear(10))` | +21.7 [−2.4, +46.1] | 1 / 2 |
| half8* | `Central(Halving(8))` | +16.5 [−6.3, +39.5] | 24 / 26 |
| sq6 | `Central(Square(6))` | +17.4 [−4.6, +39.5] | 4 / – |

Linear weights with different tops rank plans of equal cells the same
way: a plan's weight is top × cells − the sum of its cells' rings, for
cells within `top` rings, so after the cells it is the sum of rings that
decides, whatever the top; the top matters only for cells beyond it. On
the 10 boards shown to the owner, lin5, lin7 and lin9 made the same plans
on every map. Their duels differ by the noise of the clock and of the
machine (lin5 was disturbed), so they read together as about +20 to +30.

On those boards the central orders reach the most land and are at least
as central as the study's best plans for the chokes, but hold fewer
chokes at equal central weight (map 1: weights 42 / 75 / 113 both, chokes
1 of 6 against 6 of 6): a choke weighs no more than any cell of its ring.

## Closer to the frontier

The owner (2026-11-20): other evaluations, "to be closer to the
frontier"; his choice (form): all three of

- `Frontier { cap }`: the active cells, then the least sum over the
  frontier's cells (ring 0) of the steps from our land to each, at most
  `cap`, then the weighted sum: near the whole line, not one flank;
- `ByRing`: the active cells, then our cells on ring 0, then ring 1, up to
  ring 6, compared in that order, then the weighted sum;
- convex weights (`Halving`, `Square`), duelled again cleanly.

Duels, as above, all clean (at most 2 timeouts a side):

| Variant | Order | Elo over v029 [95%] |
| --- | --- | --- |
| fr4 | `Frontier { cap: 4 }` | +10.4 [−12.8, +33.7] |
| fr8 | `Frontier { cap: 8 }` | +17.4 [−6.3, +41.2] |
| ring | `ByRing` | +17.4 [−4.9, +39.8] |
| half8 | `Central(Halving(8))`, again | +16.5 [−6.6, +39.8] |

Every order that puts the land first scores between +10 and +35 in the
cut duels, and no two differ by more than the noise of 400 pairs (about
±23). What they share, the most land first, gains about +20; the
tie-breaks after it are not told apart by this judge.

## Other measures and longer plans

The owner's questions (2026-11-20), each shown on the 10 boards
(`opening/examples/places.rs dump`, pictures sent in the conversation):

- static weight maps from the HQs' steps (`Order::Field`): `Corridor`
  (rings less half the detour off the shortest paths between the HQs; on
  these maps nearly every cell lies on such a path, so it equals the
  rings), `Reachable` (rings within 9 steps of our HQ: too strict, most of
  the even line is farther), `Outward` (steps from our HQ up to halfway:
  every cell beyond is equal, so it plays like `land`);
- `Voronoi` (the share against the enemy's start, weak: with the opponent
  passing, almost all our half is already ours) and `BehindFront` (the
  cells of our side of the frontline nearer our land than the frontline:
  the most of them on 5 of the 10 maps, within half a cell elsewhere);
- the owner's three goals (`Order::Claim`): the most cells, then the
  share of the contested band (rings 0 to 2) and of the whole map against
  an enemy assumed to hold as many cells as us, those nearest its HQ (an
  enemy holding the mirror of our land gives every plan exactly half, by
  the maps' symmetry: discarded);
- longer plans (the owner: plan longer, the search still taking over at
  game turn 7): over 12 turns claim2 reaches both choke places of map 5 by
  turn 10 (5 chokes of 6 against 3); 15 turns plays as 12 up to turn 10.

Cut duels, as above (400 pairs, seed 72, local):

| Variant | Order, turns planned | Elo over v029 [95%] |
| --- | --- | --- |
| sum12 | `Sum` (v029's), 12 | −23.5 [−45.6, −1.6] |
| claim2-9 | `Claim { band: 2 }`, 9 | +22.6 [−1.0, +46.5] |
| claim2-12 | the same, 12 | +8.7 [−14.4, +31.9] |
| claim2-15 | the same, 15 | +11.3 [−11.3, +34.0] |
| fr8-12 | `Frontier { cap: 8 }`, 12 | +4.3 [−17.8, +26.5] |
| lin7-12 | `Central(Linear(7))`, 12 | +22.6 [+0.1, +45.3] |

Planning longer loses with v029's order (as in E052 and E055) and adds
nothing measurable to the land-first orders. The owner then had the cut
duels moved to GitHub (`screen.yml` with `arena: early`, added on this
branch) and the study run of the central goals cancelled to free the
runners.

## The opening's goal kept in the search

The owner's plan (2026-11-20): (1) replan every turn from game turn 4 or
5 (`REPLAN_FROM`), (2) carry the opening's goal into the search, (3) plan
against a predicted opponent, (4) then measure the passing model against
real games. The search's evaluation gained a goal term (`eval::goal`),
counted until game turn `GOAL_UNTIL`, each part the side's count less the
enemy's, over the contested band (rings 0 to 2): `GOAL` for each band
cell nearer the side's active land by at least `GOAL_MARGIN` steps,
`GOAL_OWNED` for each band cell it holds, `GOAL_GUARD` for each it holds
that a tower of its guards, `GOAL_CHOKE` for each middle choke it holds.
After seeing a game where the near term left the top-right pocket
neutral (the search was paid for being near, not for owning), the owner
asked for strategies that own the critical cells, mainly by towers.

Cut duels against v029, the opening `Central(Linear(7))` over 9 turns
(lin7), local, 800 pairs (seed 81) unless noted:

| Variant | Goal term | Until | Elo over v029 [95%] |
| --- | --- | ---: | --- |
| lin7 (400 pairs, seed 72) | none | – | +34.9 [+11.9, +58.1] |
| lin7 + replan from turn 4 (400) | none | – | +38.4 [+15.4, +61.6] |
| lin7 + replan from turn 5 (400) | none | – | +32.2 [+9.7, +55.0] |
| lin7-g2 (400, seed 72) | near 2 | 10 | +53.4 [+30.1, +77.2] |
| lin7-g4 (400, seed 72) | near 4 | 10 | +37.5 [+12.8, +62.6] |
| lin7-g1 | near 1 | 10 | +71.8 [+55.4, +88.5] |
| lin7-g2 | near 2 | 10 | +68.2 [+51.4, +85.3] |
| check-g2 (the term rewritten) | near 2 | 10 | +63.2 [+46.4, +80.4] |
| lin7-g2-r4 | near 2, replan from turn 4 | 10 | +63.7 [+47.5, +80.2] |
| claim2-g2 | near 2 (claim2's opening) | 10 | +58.7 [+41.9, +75.9] |
| own2 | owned 2 | 10 | +55.2 [+38.5, +72.1] |
| own4 | owned 4 | 10 | +74.5 [+58.1, +91.3] |
| safe2 | near 2, 2 steps nearer | 10 | +64.1 [+47.5, +81.0] |
| own2-gd2 | owned 2, guarded 2 | 10 | +59.6 [+43.1, +76.4] |
| own2-gd4 | owned 2, guarded 4 | 10 | +61.4 [+44.6, +78.5] |
| own2-ck6 | owned 2, chokes 6 | 10 | +56.5 [+39.7, +73.6] |
| near2-gd2 | near 2, guarded 2 | 10 | +80.0 [+63.0, +97.4] |
| own2-gd2-ck6 | owned 2, guarded 2, chokes 6 | 10 | +88.3 [+71.1, +105.9] |
| lin7-g2u12 | near 2 | 12 | +93.4 [+76.0, +111.2] |

Timeouts: 0 to 8 a side in each run, as for v029. The goal term adds
about +30 to +50 to lin7 in these duels; its parts are not told apart at
800 pairs, though the strategies with towers or chokes lead. A term
counted until game turn 12 is counted on the judge's own turn, so its
gain needs full games. `tests::the_search_respects_its_deadline_on_a_heavy_position`
failed once while the duels used every CPU and passed 4 times of 4 on
the idle machine.

The GitHub duel screening (run 44710005153, `arena: early`) was
cancelled by the owner after 2 hours 11 minutes: its log shows every
runner still in the bundler, no game played. The bundler on the plan bot
of that commit runs over 10 minutes locally too, while it bundled
without trouble at 75bf3d6 earlier the same day: a change to the opening's code since makes the
minifier slow (being bisected).
