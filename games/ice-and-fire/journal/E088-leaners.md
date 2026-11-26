---
id: E088
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: The three changes of E083 to E087 that leaned positive in their screenings (the centre pull, kept gold, the opponent's steps before its cuts), alone or together, beat ice-and-fire-v032 once screened with three times the pairs.
change: E083's centre pull (`front.rs`), E084's kept gold (`eval.rs`) and E087's replies with steps (`search.rs`, with its cut finder's scattered cuts in `tactics`, unused here), gathered on one branch from main; each setting at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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

Pending.
