---
id: E090
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Counting as threatened the cells behind a cut the enemy can afford, not only the cells its chains enter, makes the search guard the cells it actually loses and beats ice-and-fire-v032.
change: With `eval::THREAT_CUTS`, the exposed cells of a side (E023, E037) also include its active cells that the loss of any one exposed cell cuts off from its HQ, at the same `THREAT_QUARTERS` each; off in v032.
release: none
sprt: not run: clearly weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The owner's cell values (2026-11-27): a cell's value is its income while
we keep it, and whether we keep it depends on its price, the cheapest
chain of trainings by which the enemy takes it or cuts it off. Step 0
([research/codingame/2026-11-27-cell-values.md](../research/codingame/2026-11-27-cell-values.md))
measured it on v032's 2,350 test games against the top 10: the price with
cuts ranks the cells lost at the next ply with an AUC of 0.85 to 0.93,
ahead of the direct price that `threatened` counts (0.82 to 0.92), and 49%
(game turns 7 to 12) to 65% (from turn 13) of the cells lost are cut off,
not captured. The shape term (E010, E081) counts joints whatever the enemy
can afford; this term counts what it can afford now.

## Change

`eval::threat` replaces the body of `exposed`: the cells the enemy's
chains reach for its gold plus income, after its movers step (as before),
and with `THREAT_CUTS` the active cells cut off by the loss of any one of
them (`with_severed`: one flood from the HQ per exposed cell with at least
two active neighbours). The mine term and the tower offers read the same
cells.

## Screening

Screened with E091's variants in one run.

| Variant | Commit | Elo over v032 [95%] | Score | Tries a ms (speed, 300 positions) |
| --- | --- | ---: | ---: | ---: |
| `off` (every switch off: plays as v032) | `26896ab` | +9.1 [+0.1, +18.2] | 51.3% | 23.8 |
| `cuts` (E090) | `562ce31` | −35.7 [−50.6, −21.1] | 44.9% | 15.1 |
| `cuts-margin` (E091) | `b1a4b2e` | −39.3 [−54.5, −24.2] | 44.4% | not measured |
| `margin` (E091) | `b91b481` | −8.7 [−23.1, +5.6] | 48.8% | 24.9 |

800 seat-swapped pairs per variant against `ice-and-fire-v032`, whole
games at CodinGame's limits, 4 shards, no faults on either side ([run 45959740011](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45959740011)). The
control `off` came out at +9.1, its interval just above 0: chance, since
it plays as v032, and a reminder that a variant's interval has about that
noise. The first run (45959541618) was cancelled: the tower offers
(`actions.rs`) counted the threats before a tower directly and after it
with cuts, so with `THREAT_CUTS` almost no tower looked useful (the test
`towers_are_offered_where_a_copy_of_the_state_says_they_guard` failed on
it); fixed in `26896ab` before this run, every test passing on every
variant.

Speed: `ice-and-fire-bot-plan speed` (analysis build) at 2,000 tries on
300 middle-game positions drawn from v032's CodinGame games
(`ice-and-fire-review bank`), the fastest of 2 runs.

## What was learned

Dropped, but the idea is not refuted: `cuts` loses 36 Elo while searching
37% fewer tries a millisecond, so at 50 ms it plays a much shallower
search. The severed cells cost one flood from the HQ per exposed joint, in
every evaluation. Before a cell's price with cuts can be judged as a term,
it has to cost about nothing: the shape term already runs a depth-first
search for the joints of each side (`weakness_weighted`), which could give
each joint's cut-off cells in the same pass. A cut that is counted should
also be the enemy's best one rather than all of them together, since the
attacker plays one chain (the study's per-ply loss rate of a fifth to two
fifths). Both are a new experiment, on the owner's word.

The code stays on the branch `claude/ice-and-fire-e090-cut-threats`
(commits `26896ab` to `b91b481`; `with_severed` and `threat` in
`bots/plan/src/eval.rs`, the test `a_cut_severs_the_cells_behind_it`).
