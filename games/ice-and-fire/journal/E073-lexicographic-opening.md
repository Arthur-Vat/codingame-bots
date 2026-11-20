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
