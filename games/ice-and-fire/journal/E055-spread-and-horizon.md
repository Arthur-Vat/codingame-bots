---
id: E055
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: E051 (our units spread in the opening, each of our units next to a cell lowering its value by 2) and E052 (the opening planned over 9 of our turns rather than 7) each gained about +25 to +30 against v022 on 2,400 pairs, by different means (how units move, how far the plan looks); together they gain more than either, and beat v022.
change: The opening's rules value a cell 2 or 3 less for each of our units next to it (E051) and the opening plans 9 of our turns (E052), with 40 or 45 ms a turn.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 (a form, after the confirmations of E051
and E052: "Combine, then SPRT (Recommended)"): the best of E051 with the
best of E052, screened against `ice-and-fire-v022` with each alone in the
same run, the best form then frozen as the candidate.

## Why

On 1,600 new pairs against v022 (runs 43576849849 and 43576854829):

| Experiment | Variant | Elo against v022 | Both rounds, 2,400 pairs |
| --- | --- | --- | --- |
| E051 | crowd 2 | +30.0 [+19.9, +40.3] | about +31 |
| E051 | crowd 3 | +27.9 [+16.3, +39.4] | about +25 |
| E052 | 9 turns | +26.5 [+16.1, +37.1] | about +25 |
| E052 | 45 ms a turn | +8.5 [+1.6, +15.3] | about +9 |

The study of the first 10 turns and each screening are in E051's and
E052's entries (their own pull requests).

## The change

`opening/src/econ.rs` and `bots/plan/src/main.rs` as in E051
(`Params::crowd`, `OPENING_CROWD`), with E052's constants
`OPENING_TURNS` and `BUDGET_MS` of `bots/plan/src/main.rs`. The base
commit is v022's settings.

## Screening

Pending.
