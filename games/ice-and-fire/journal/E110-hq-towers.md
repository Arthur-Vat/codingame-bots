---
id: E110
date: 2026-11-28
pull_request: "#410"
parent: ice-and-fire-v034
hypothesis: We build 1.3 towers a game within 3 steps of our own HQ, rarely on the border later (the tower study, [`2026-11-28-towers.md`](../research/codingame/2026-11-28-towers.md)); not offering them spends that gold better and beats ice-and-fire-v034.
change: A switch in our climbs (never the opponent's model) that offers no tower within k steps of our HQ from game turn 15 (`HQ_TOWER_BAN`, `HQ_TOWER_FROM`).
release: none
sprt: not run: the variants lost in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The tower study found that we put 18% of our towers within 3 steps of our
own HQ (28% from game turn 15, 36% from turn 21; zasmu 2%), a median of 7
steps from the enemy's land, and only 13% of them on the border within five
turns. The owner asked (2026-11-28) to price them.

## Change

On the branch `claude/ice-and-fire-e110-hq-towers` (from `main`):
`search::ban_hq_towers` removes every action that builds a tower within
`HQ_TOWER_BAN` steps of our HQ, in our climbs' actions at every step and in
the forced climbs' first actions, from game turn `HQ_TOWER_FROM` (15) by the
climb's root. Off (`None`), nothing is removed. A test checks that only
towers near our HQ are removed.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46148673245](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46148673245)):

| Variant | `HQ_TOWER_BAN` | Elo over v034 [95%] |
| --- | ---: | ---: |
| `control` | none | +3.0 [−2.3, +8.4] |
| `ban3` | 3 | −8.9 [−15.7, −2.1] |
| `ban2` | 2 | −10.2 [−16.8, −3.6] |

Commits: `control` `45e776b`, `ban3` `a52c974`, `ban2` `d134844`. One timeout
of `ban3`; no other fault.

## What was learned

Dropped: both bans lose about 12 against the control. The towers near our HQ
come from the guard against an affordable chain to our HQ (E026's second
variant, `HQ_GUARD`): they answer a threat that their presence then keeps
from happening, which replays cannot show. The owner's next step was a rule
by threat rather than by distance (E112).

The code stays on the branch (commit `45e776bd`, the switches off).
