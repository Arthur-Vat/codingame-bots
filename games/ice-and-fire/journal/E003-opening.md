---
id: E003
date: 2026-10-22
pull_request: "#183"
parent: ice-and-fire-v002
hypothesis: An opening by rules that takes free cells with every unit and buys trainings and mines only while they pay back, then hands over to the plan search when the territories meet, reaches the middle game with more income, more gold and a less exposed territory than v002's hill climb, and so beats v002.
change: The `plan` bot plays the `econ` opening (games/ice-and-fire/bots/plan/src/opening.rs) while the territories are more than 3 steps apart, and its evaluation counts a side that cannot pay its upkeep as having lost its units.
release: ice-and-fire-v003
sprt: accepted
elo: +299.8 [+212.9, +435.6] at CodinGame's limits
pairs: 43
full_time: +299.8 [+212.9, +435.6] (the SPRT's own, ADR 0042), no timeout in 86 games, slowest answer 26.4 ms
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-10-22, after a CodinGame replay of v002 (as blue:
3 gold and an income of −9 at turn 7, a thin line of units, then its army
wiped out by the debt rule): research the early game, compare several
approaches in theory, benchmark them, put the best in a full bot and keep the
best of those. The research and the theory are in
[research/early-game.md](../research/early-game.md); this entry has the
measures.

## What v002 did wrong

Its evaluation pays +10 for each step that brings the chain to the enemy HQ
closer, against +4 for a training and +7 for a move onto a free cell, and −3
for a mine (`ice_and_fire_tactics::eval::evaluate`). So in the opening it
sends a line of units along an edge toward the enemy HQ, never builds a mine
(it trains on the mine spot next to its HQ on turn 1), and keeps few units
taking new cells. At game turn 12 of the round robin below (against all the
candidates) it holds about 22 cells, an income of 16
and 25 gold, where the idealized model of the research note reaches 49
cells, 37 and 76.

## The tools

- **The opening benchmark**, `cargo run --release -p ice-and-fire-arena
  --example opening -- DIR`: replays the games that `ice-and-fire-arena
  match --records DIR` kept and reports, for each bot, its score, its
  wipe-outs by debt, and at contact (the first start of its turn with the
  territories at most 2 steps apart) its cells, income, gold, units, mines
  and Voronoi share, then its exposure (the opponent's best cut, and its gold
  against its cheapest chain to the bot's HQ). `--maps N` gives the map
  facts: over 2,000 source maps, 105.6 cells, 48.6 on each side, the HQs 22
  steps apart, 4.8 mine spots on each side.
- **The candidates**, each an opening of the `plan` bot, chosen by
  environment variables during the screening only (commit 277d510 of this
  pull request keeps them all): `rush`, `fill`, `econ` and `rollout`, as the
  research note describes them. From contact on, the plan search of v002
  plays.

## Screening

Rust 1.90.0, `ice-and-fire-arena`, CodinGame's limits (50 ms, 1,000 ms for
the first turn, no tolerance), two games at a time on a 4-CPU machine,
seat-swapped pairs.

**Each opening against v002**, 200 pairs (seed 1, then seed 7 for the later
rows):

| Opening | Score | Elo | At contact: cells, income, gold, units, mines |
| --- | --- | --- | --- |
| `rush` | 68.0% | +130.9 [+83.0, +184.2] (100 pairs) | 18.4, 13.7, 18.2, 4.7, 0 |
| `fill`, hand-over at a gap of 1 to 5 | 76.0% to 81.2% | +200 to +255, best at 2 and 3 | 20.2 to 20.5, 15.1 to 15.3, 17.8 to 19.2, 5.2, 0 |
| `fill`, trainings stopped at 1 to 6 free cells per unit | 79.2% to 81.0% | +233 to +252 | the same: the rule does not bind before contact |
| `econ`, a unit valued for the cells of its whole side | 79.5% | +235.4 [+178.1, +306.8] (100 pairs) | 20.3, 15.5, 18.1, 5.1, 0.07 |
| `econ`, a unit valued for the cells it takes before contact, plus 0, 2, 4 or 8 | 83.2%, 80.8%, 81.0%, 81.2% | +278.5, +249.1, +251.9, +254.7 | with 0: 19.6, 18.9, 28.3, 3.9, 0.81 |
| `econ` (plus 0), seed 7 | 84.5% | +294.6 [+252.8, +344.8] | 19.5, 18.8, 28.6, 3.9, 0.81 |
| the same with the debt check | 84.5% | the same games | the check never changed a plan in these games |
| the same, keeping 15 or 30 gold within 5 steps of contact | 85.8%, 85.0% | +311.8, +301.3 | 19.7, 17.8, 30.3, 4.0, 0.52 |
| `rollout`, its end states valued by gold and income | 74.2% | +184.0 | 19.0, 19.6, 26.2, 3.8, 1.11 |
| `rollout`, 3 turns, also 10 a unit, or 5 a cell of Voronoi share, or both | 76.2% to 78.8% | +203 to +228 | 18.8 to 19.6, 18.0 to 19.8, 24.8 to 27.9 |
| v002 with the debt check alone | 50.0% | 0.0 [−8.3, +8.3] | the same play as v002 |

**Round robin** (seed 11, 200 pairs per pairing, Bradley-Terry with v002 at
0, all with the debt check): `econ` +255.7 ± 21.6, `econ` with a reserve of
15 +245.8, `rollout` (3 turns, 10 a unit) +230.1, `fill` +223.9, `rush`
+143.2, v002 0, v001 −209.5. Head to head, `econ` scored 54.2% against
`fill`, 69.5% against `rush`, 50.2% against `rollout` and 50.5% against the
reserve. The opening benchmark over all those games (2,400 per bot):

| Bot | Score | Contact turn | Cells | Income | Gold | Units | Mines | Voronoi cells | Exposure: cut cells, unit value |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `econ` | 66.8% | 8.0 | 20.3 | 20.0 | 30.5 | 3.8 | 0.89 | 52.2 | 1.0, 9.5 |
| `econ`, reserve 15 | 65.4% | 8.1 | 20.5 | 18.9 | 32.1 | 4.0 | 0.60 | 52.7 | 1.1, 9.5 |
| `rollout` | 63.2% | 8.0 | 19.8 | 21.0 | 29.6 | 3.6 | 1.22 | 50.9 | 1.0, 9.6 |
| `fill` | 62.3% | 7.7 | 21.1 | 16.0 | 19.9 | 5.1 | 0 | 55.1 | 1.0, 8.0 |
| `rush` | 50.7% | 7.8 | 20.4 | 15.4 | 20.4 | 5.0 | 0 | 50.3 | 1.0, 7.7 |
| v002 | 31.3% | 7.9 | 18.7 | 13.8 | 18.6 | 4.8 | 0 | 47.7 | 1.2, 8.3 |
| v001 | 10.4% | 8.7 | 17.3 | 23.7 | 32.5 | 3.0 | 2.41 | 43.6 | 2.9, 16.6 |

What they say:

- **The opening is worth about +250 Elo** over v002's: every rule-based
  opening beats the hill climb in the opening, `rush` the least.
- **Income and gold at contact go with strength, but not alone.** `econ` has
  the most gold and nearly the most income, and wins; `rollout`, which
  maximised gold plus income by search, reached slightly more income with
  fewer cells and units and lost a little; v001 has the most income (its
  mines) and loses by far, with the most exposed territory (2.9 cells and
  16.6 of units in the opponent's best cut) and the smallest share of the
  map. A search with a proxy objective did worse than the rule it searched
  over, as MSmits found ("messy, hard to defend").
- **Mines pay** once a unit is valued for what it takes before contact: the
  mine spot next to the HQ gets its mine in most games (0.8 a game at
  contact), for +4 income and +10 gold at contact over `fill`, and +32 Elo
  over it in the round robin.
- **The reserve, the debt check and the hand-over gap made no measured
  difference.** The debt check never changed a plan in these games, but it
  fixes the flaw the owner saw on CodinGame (a plan that leaves gold +
  income below 0 loses the army), at no cost, so it stays. The first turn's
  1,000 ms found no use: the first turn trains two units next to the HQ
  whatever the search.

**The bot as built** (`econ`, purchases paying back by game turn 15 but at
least 6 turns away, so that a long opening still buys; that change scored
50.9% against the screened `econ`, +6.1 [−4.5, +16.7], 400 pairs, seed 13):

| Opponent | Pairs | Score | Elo | Faults | Answers after the first: median, 99.9%, slowest |
| --- | ---: | --- | --- | --- | --- |
| v002 (seed 21) | 400 | 657 wins, 143 losses (82.1%) | +264.9 [+234.1, +299.6] | none | 3.0, 26.6, 30.7 ms |
| v001 (seed 21) | 200 | 377 wins, 23 losses (94.2%) | +485.8 [+422.7, +579.7] | none | 0.2, 26.4, 29.0 ms |
| v002, the release file (seed 1) | 100 | 167 wins, 33 losses (83.5%) | | none | |

At contact against v002 it has 19.5 cells, an income of 18.8, 28.4 gold and
0.81 mines (v002: 18.4, 13.8, 18.8, 0), a Voronoi share of 54.2 cells (46.1),
and the opponent's best cut takes 0.5 cells and 4.4 of units (1.5 and 10.4
from v002). It lost its units to debt 0.02 times a game, v002 0.09. At game turn
12 it holds 35.4 cells, an income of 27.3 and 38.5 gold (v002: 18.5, 14.2,
22.9).

## Not done, and ideas

- After contact the plan search never builds a mine (its evaluation scores a
  mine −3): an evaluation that values income over more turns would.
- The opening's cell values (3 a free neighbour, 2 a step of contest) were
  not tuned; nor was the horizon.
- The first turn's 1,000 ms and the 40 ms of the opening's turns are unused.

**SPRT** (head 2becdb6, CodinGame's limits: time scale 1, no tolerance,
against `ice-and-fire-v002`): accepted after 43 pairs (LLR 2.97 after 41),
84.9% of 86 games, +299.8 Elo [+212.9, +435.6]. Smoke test: 200 of 200
against `random`. No timeout, crash or invalid answer; answers after the
first: median 3.3 ms, 99.9% 26.3 ms, slowest 26.4 ms.

**League** of v003 (800 pairs each, CodinGame's limits): 82.4% against v002,
94.5% against v001, 96.2% against `cutter`, 99.3% against `guard`, 99.3%
against `chain`, 99.2% against `bronze`, 100% against `random`; rated
+1975.5 against v002's +1607.1 (random at 0).

**Decision:** promoted, `ice-and-fire-v003`.
