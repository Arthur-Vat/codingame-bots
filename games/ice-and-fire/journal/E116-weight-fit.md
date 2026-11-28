---
id: E116
date: 2026-11-28
pull_request: "#NN"
parent: ice-and-fire-v034
hypothesis: The evaluation's terms were tuned one at a time and their combination is not; their weights fitted jointly on the outcomes of v034's games against itself, a quarter of the way from today's, value land, working units and safe cells more and banked gold, units and towers less, and beat ice-and-fire-v034.
change: Each of the evaluation's 17 terms gets a weight in percent (all 100 is v034's evaluation exactly); the release sets them a quarter of the way from 100 to a logistic fit of the game outcome on the terms.
release: ice-and-fire-v035
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

Single weights looked tuned: the gold held for a strike lost when counted
at 50% (E113, −10.2) and at 125% (E115, −11.5) against v034. The owner
chose (2026-11-28) to fit every term's weight together on game outcomes
("Tune all weights jointly"), the data from arena self-play and CodinGame
with self-play deciding ("Self-play first": CodinGame games were played on
by older releases against opponents of every strength), fitted offline and
then screened.

## The change

On the branch `claude/ice-and-fire-e116-weight-fit`, in
`bots/plan/src/eval.rs`: `WEIGHTS`, one percentage per term, applied as
`term * weight / 100` to each term, ours less the enemy's. The terms are
the shared evaluation's five parts of material (gold, 3 × income, 4 ×
active cells, units by training cost, 10 × active towers;
`ice_and_fire_tactics::eval::side_parts`), its pressure term, the standing
threat, the debt, the shape, the strong units, the idle units, the
threatened cells, the mines, the wall, the goal, the Voronoi and the flank
terms. The threat of losing the HQ (`-WIN / 2`) and finished games are not
weighed. With all 100 the answers are those of v034, byte for byte
(`judge --tries 600` on 2,031 positions).

The analysis mode `terms` prints every term of every position of a bank,
and `research/codingame/2026-11-28-weight-fit/fit.py` (numpy) fits
P(the side to act wins) = sigmoid(k × Σ w_i × term_i): first the scale k
with today's weights, then all w_i with k fixed and a pull towards 100
chosen on held-out games (games split 80/20). Positions from game turn 7,
not finished and without the `-WIN / 2` threat.

| Source | Games | Positions | Held-out log-loss, today's | Fitted |
| --- | ---: | ---: | ---: | ---: |
| v034 against itself, 2,400 pairs at CodinGame's speed | 4,798 | 162,673 | 0.40315 | 0.38617 |
| Our CodinGame games, all | 10,644 | 145,815 | 0.39374 | 0.38621 |
| Our CodinGame games, v030 and later against the top 10 | 2,241 | 31,623 | 0.39705 | 0.38484 |

The self-play weights also predict the held-out CodinGame games better than
today's (0.38999 on all of them, 0.38554 on the filtered ones).

| Term | Self-play fit | CodinGame fit (`fitcg`) | Release (25% of the way) |
| --- | ---: | ---: | ---: |
| gold | 26 | 67 | 82 |
| income | 140 | 170 | 110 |
| cells | 196 | 67 | 124 |
| units | 42 | 111 | 86 |
| towers | 25 (−11 unclamped) | 39 | 81 |
| pressure | 86 | 95 | 97 |
| standing threat | 190 | 25 (−47) | 123 |
| debt | 69 | 85 | 92 |
| shape | 84 | 68 | 96 |
| strong units | 129 | 177 | 107 |
| idle units | 203 | 77 | 126 |
| threatened cells | 182 | 149 | 121 |
| mines | 136 | 139 | 109 |
| wall | 53 | 48 | 88 |
| goal | 101 | 101 | 100 |
| Voronoi, flank | never nonzero | | 100 |

## Screening

Against `ice-and-fire-v034`, 1,600 pairs a variant, 4 shards, at
CodinGame's speed:

| Run | Variant | Weights | Elo over v034 [95%] |
| --- | --- | --- | ---: |
| [46161383275](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46161383275) | `control` | 100 | −1.5 [−7.2, +4.2] |
| | `fit` | self-play fit | −25.5 [−36.5, −14.4] |
| | `half` | 50% of the way | +16.3 [+5.7, +27.0] |
| | `fitcg` | CodinGame fit | −74.5 [−86.1, −63.1] |
| [46163223188](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46163223188), seed 1161 | `control` | 100 | +0.4 [−4.8, +5.7] |
| | `p25` | 25% | +20.9 [+10.8, +31.0] |
| | `half` | 50% | +8.0 [−2.6, +18.7] |
| | `p75` | 75% | +4.3 [−6.5, +15.2] |
| [46165551270](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46165551270), seed 1162 | `control` | 100 | +1.5 [−4.4, +7.4] |
| | `p15` | 15% | +11.1 [+0.9, +21.3] |
| | `p25` | 25% | +14.1 [+3.8, +24.5] |
| | `p35` | 35% | +11.1 [+0.9, +21.3] |

Commits: `control` `07c96e9`, `fit` `cac83cd`, `half` `6eb3d7d`, `fitcg`
`3196cef`, `p15` `840b7fa`, `p25` `d861b81`, `p35` `da92746`, `p75`
`ea1cabd`. Faults: a timeout of the control in the first and third runs
and of `p25` in the second, three of v034 in all; no crash or invalid
answer.

Every point from 15% to 75% of the way gained, the full fit lost: the fit
gives a direction, and the search, playing positions unlike those it was
fitted on, overshoots along it. The owner chose 25% as the candidate
(2026-11-28, "p25 becomes v035").

## SPRT

Pending.
