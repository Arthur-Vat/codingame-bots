---
id: E071
date: 2026-11-19
pull_request: "#362"
parent: ice-and-fire-v029
hypothesis: E069's strike reserve (keep a strike on the enemy HQ affordable next turn) was level with v028 in self-play and ranked one place higher on CodinGame; on top of v029, whose units now advance behind towers, it beats v029.
change: STRIKE_TURNS 1 in the plan search (E069's reserve) on v029.
release: none
sprt: not run: weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-19 (form, "Reserve on v029, screen"), after
[E069](E069-strike-reserve.md)'s probe ranked 4th of 88 on CodinGame with
win rates level with v028's.

## Screening

`screen.yml` run 44526416856, 800 pairs against `ice-and-fire-v029`, 2
shards, at CodinGame's speed and real limit:

| Variant | Ref | Pairs | Elo over v029 [95%] | Score | Faults |
| --- | --- | ---: | --- | ---: | --- |
| k1 (`STRIKE_TURNS` 1) | `5e5c00c` (branch `claude/ice-and-fire-e071-reserve-v029`) | 800 | −13.5 [−24.8, −2.2] | 48.1% | none |

Weaker than v029, where E069's reserve was level with v028: the reserve
keeps the gold that v029's step-and-tower action spends to free its units
(E070). The owner chose to drop it (form, 2026-11-19); the reserve stays
in the plan bot at `STRIKE_TURNS` 0.
