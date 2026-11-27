---
id: E091
date: 2026-11-27
pull_request: "#391"
parent: ice-and-fire-v032
hypothesis: Weighing each threatened cell by how much gold the enemy has to spare when it reaches it, as the cells lost in v032's games against the top 10 say, beats ice-and-fire-v032.
change: With `eval::DEEP_MARGIN = 50`, a threatened cell that the enemy reaches with 50 gold to spare costs `DEEP_QUARTERS` (7) quarters and the others `SHALLOW_QUARTERS` (4), instead of `THREAT_QUARTERS` (5) each; with and without E090's cuts; off in v032.
release: none
sprt: not run: weaker or level in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

In step 0 of the cell values
([research/codingame/2026-11-27-cell-values.md](../research/codingame/2026-11-27-cell-values.md)),
a cell the attacker can pay for is lost at that ply 17 to 28% of the
time when its price is within 40 gold of the attacker's gold, and 27 to
44% when it is 50 gold or more within; a cell just out of reach is lost
1.5 to 3% of the time, one 40 gold beyond almost never. E023's single
weight (a loss of a quarter) is right on average; this splits it.

## Change

`eval::threat` runs the enemy's reach a second time with its budget less
`DEEP_MARGIN` (nothing when the budget is below it); those cells, with
E090's severed cells when `THREAT_CUTS` is on, cost `DEEP_QUARTERS`, the
other exposed cells `SHALLOW_QUARTERS`. Two variants: `cuts-margin` (with
E090's cuts) and `margin` (without).

## Screening

In the same run as E090 (see its entry for the method):

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

`margin` costs no speed (24.9 tries a ms against 23.8) and gains nothing:
−8.7 [−23.1, +5.6], 18 below the control of the same run. Splitting the
threat weight by the enemy's gold to spare, as the losses of real games
do, does not help the search choose its moves: a cell lost more often is
not a cell whose defence pays more. `cuts-margin` is as weak as `cuts`, for
the speed of its cuts. Dropped.
