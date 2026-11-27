---
id: E099
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v033
hypothesis: Removing or shrinking each part of the search in turn, at CodinGame's limits, shows what each part is worth against the time it takes; the search with half its time shows what a faster search would be worth; and a check of the plan chosen that starts only when it fits before the hard limit removes most turns that end past it without losing strength.
change: Switches in `search.rs` that leave v033 unchanged (`REPLY_OWN`, `LAZY`, `VERIFY_FITS`), and one variant commit per ablation, each against ice-and-fire-v033.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked how the search spends its time and how to make it more
efficient (2026-11-27). A timer-instrumented v033 in 200 local games at
CodinGame's speed (page https://claude.ai/artifact/RqrWMARqi4Fw8d5DtETKeC)
found, from game turn 7: a median turn of 42.1 ms; the reply searches 43%
of the turn (the opponent's own climb alone 32%, about 1.3 ms a search);
scoring the tries 28%; generating the actions 18%, rising through the
game; the final choice and its check 4.5%. 3% of the plies ended past
45 ms (worst 54 ms, past CodinGame's 50), mostly because the check of the
plan chosen (E027's `VERIFY`) started a reply search near the hard limit.

Before speeding any part up, two questions: what a faster search is worth
at all (the same bot with half its time), and what each part is worth
against its time (an ablation of each, at the same limits).

## Change

Switches, each leaving v033 as it plays (the same answers on 300 positions
of v033's CodinGame games at 600 tries):

- `REPLY_OWN`: whether the replies hold the opponent's own climb (E019).
- `LAZY`: whether a climb's steps are lazy.
- `VERIFY_FITS`: the check of the plan chosen starts a reply search only
  when the longest reply search of the turn so far still fits before the
  hard limit; otherwise the best plan whose replies were searched is
  played.

## Screening

Two runs against `ice-and-fire-v033`, whole games at CodinGame's limits.

| Variant | Change | Elo over v033 [95%] |
| --- | --- | ---: |
| `off` | none (control) | pending |
| `half` | `BUDGET_MS` 22.5 (half the time; the first turn unchanged) | pending |
| `vfix` | `VERIFY_FITS` | pending |
| `noopp` | `REPLY_OWN` off: no opponent climb in the replies | pending |
| `opp100` | `REPLY_EVALS` 100 rather than 300 | pending |
| `cut1` | `REPLY_CUTS` 1 rather than 3 | pending |
| `mem4` | `MEMORY_CAP` 4 rather than 8 | pending |
| `noblock` | `BLOCK_TOWERS` off (E044's blockers) | pending |
| `fullpass` | `LAZY` off: every action tried at every step | pending |
