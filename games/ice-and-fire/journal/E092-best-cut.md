---
id: E092
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Counting in the threat what the enemy's best affordable cut severs beyond the cell it takes, found in one depth-first search a side so that the search keeps its speed, beats ice-and-fire-v032 where E090's union of every cut, 37% slower, did not.
change: With `eval::BEST_CUT`, a side's threat adds, to its exposed cells (E023, E037), the cells behind the best single capture among them (`best_cut`, a depth-first search for the joints, as the shape term's), at `THREAT_QUARTERS` each; off in v032.
release: none
sprt: not run: weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

E090 counted as threatened every cell behind any affordable cut, with one
flood a cut in every evaluation: its search ran 37% fewer tries a
millisecond and lost 36 Elo, which left the idea untested. The owner asked
(2026-11-27) to run it again made cheap, and the attacker plays one chain
a turn, so only its best cut counts here (the cells lost at a ply in
v032's CodinGame games: a fifth to two fifths of those it could afford,
research/codingame/2026-11-27-cell-values.md).

## Change

`eval::best_cut(state, seat, taken)`: one depth-first search of `seat`'s
active land from its HQ, which sums below each joint in `taken` the
importance of the cells it cuts off (`THREAT_QUARTERS` each, or E093's
values) and keeps the largest. `threat_quarters` adds it to the exposed
cells' cost. The exposed cells themselves, read by the mine term and the
tower offers, do not change.

Speed (`ice-and-fire-bot-plan speed`, 2,000 tries on the 300 positions of
E090's bank, the fastest of 2 runs): `off` 25.3 tries a ms, `best-cut`
20.3, `best-cut-values` 20.0, `values` 26.0.

## Screening

Screened with E093's variants in one run.

| Variant | Commit | Elo over v032 [95%] | Score | Tries a ms |
| --- | --- | ---: | ---: | ---: |
| `off` (every switch off: plays as v032) | `fa2beb3` | +2.2 [−6.9, +11.2] | 50.3% | 25.3 |
| `best-cut` (E092) | `c1cc082` | −20.0 [−35.2, −4.9] | 47.1% | 20.3 |
| `best-cut-values` (E092 and E093) | `63424bc` | −45.4 [−61.7, −29.4] | 43.5% | 20.0 |
| `values` (E093) | `8614862` | −13.0 [−28.5, +2.4] | 48.1% | 26.0 |

800 seat-swapped pairs per variant against `ice-and-fire-v032`, whole
games at CodinGame's limits, 4 shards, no faults on either side ([run 45962787462](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45962787462)).
With the switches on, two tests that pin the evaluation's numbers fail, as
the formula changes (`cells_the_enemy_can_take_next_turn_count_less`,
`a_mine_counts_more_with_a_larger_territory`); they would have been
updated for a promoted variant.

## What was learned

Dropped. Made cheaper, the cut costs the search 20% of its tries instead
of E090's 37%, and loses 20 Elo instead of 36: part of the loss is speed,
but not all, since E091's `margin` and E093's `values`, which cost none,
lose too. Four ways of making the threat term weigh more (E090 to E093)
have all lost to v032's 5 quarters a directly threatened cell. A better
predictor of the cells lost does not make a better term: a heavier threat
makes the search pay for defence where v032 plays on, and the cells the
cuts add are the ones the shape term and the towers already guard against.
The threat family looks exhausted at this weight; what the study showed
(cuts decide most losses) may be better used in the search's replies than
in the evaluation.

The code stays on the branch `claude/ice-and-fire-e092-best-cut`
(commits `fa2beb3` to `8614862`; `best_cut`, `importance` and
`threat_quarters` in `bots/plan/src/eval.rs`, the test
`the_best_cut_counts_the_cells_behind_it`).
