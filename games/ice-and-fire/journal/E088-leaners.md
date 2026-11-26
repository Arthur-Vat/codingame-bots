---
id: E088
date: 2026-11-26
pull_request: "#388"
parent: ice-and-fire-v032
hypothesis: The three changes of E083 to E087 that leaned positive in their screenings (the centre pull, kept gold, the opponent's steps before its cuts), alone or together, beat ice-and-fire-v032 once screened with three times the pairs.
change: E083's centre pull (`front.rs`), E084's kept gold (`eval.rs`) and E087's replies with steps (`search.rs`, with its cut finder's scattered cuts in `tactics`, unused here), gathered on one branch from main; each setting at its v032 value is v032.
release: none
sprt: not run: no variant above 0 in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

Of the 50 or so variants screened in E083 to E087 (each against v032, 400
pairs), three leaned positive without an interval above 0: E083's pull4
(+7.8 [-3.6, +19.2]), E084's keep2 (+11.3 [-9.9, +32.6]) and E087's steps3
(+15.6 [-4.0, +35.4] in its first screening, -4.3 [-24.0, +15.3] in its
second). The owner (2026-11-26) chose to confirm them with more pairs, and
to record what E083 to E087 taught (`research/2026-11-26-early-fights-lessons.md`).

## The change

- `bots/plan/src/front.rs` (E083, as at its pull4 commit `5a579353dd66`):
  `CENTRE_PULL`, each step between a side's active land and the middle
  front's centre (the enemy's: its mirror), and `CENTRE_HOLD`, holding it,
  on maps with a middle front, until game turn 14; its other terms off.
- `bots/plan/src/eval.rs` (E084): `KEEP_QUARTERS`, each gold kept up to
  `KEEP_CAP` (30) counting more than its value.
- `bots/plan/src/search.rs` (E087, as at `65d86725800b`): `REPLY_STEPS` 3,
  the opponent's replies with all its units stepping toward our land, and
  each step alone, before its best cuts.

## Screening

Each variant against `ice-and-fire-v032`, 1,200 pairs over three shards,
full games, no faults on either side (run 45871092842):

| Variant | Elo |
|---|---|
| pull4 | -2.0 [-9.4, +5.4] |
| keep2 | +3.8 [-8.5, +16.0] |
| steps3 | -11.9 [-22.9, -0.9] |
| pull4-keep2 | +10.4 [-2.0, +22.9] |
| keep2-steps3 | +0.6 [-12.1, +13.2] |
| pull4-keep2-steps3 | -3.5 [-15.8, +8.8] |

## Result

No variant beats v032: the leaners of E083 to E087 were noise. steps3 is
the only one whose interval lies below 0, the opponent's modelled steps
costing more search time than they return. pull4-keep2 leans positive
again, but its interval contains 0 at 1,200 pairs, and pull4 alone is
level with v032. Dropped, as E083 to E087: their lessons are in
`research/2026-11-26-early-fights-lessons.md`.
