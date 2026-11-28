---
id: E115
date: 2026-11-28
pull_request: "#410"
parent: ice-and-fire-v034
hypothesis: E113 found that counting the gold held for a strike less loses, more with each step; counting it more (gold kept towards the chain to the enemy HQ) makes the search keep its strike closer and beats ice-and-fire-v034.
change: A switch in the evaluation (both sides, every turn): the pressure term and the standing threat scaled to `STRIKE_GOLD_PERCENT` of their value.
release: none
sprt: not run: every variant lost in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

E113 (the invest strategy) lost at 50%, 25% and 0% of that gold's value.
The owner chose (2026-11-28) to test the opposite before setting the weights
of the strategy portfolio.

## Change

On the branch `claude/ice-and-fire-e115-strike-gold` (from `main`), in
`eval::eval`: `strike_gold_extra` adds `(STRIKE_GOLD_PERCENT − 100)%` of the
pressure term (`margin.clamp(−200, 0)`) plus the standing threat, ours less
the enemy's. At 100, nothing changes.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46153864470](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46153864470)):

| Variant | `STRIKE_GOLD_PERCENT` | Elo over v034 [95%] |
| --- | ---: | ---: |
| `control` | 100 | +1.5 [−4.4, +7.4] |
| `g125` | 125 | −11.5 [−21.4, −1.6] |
| `g150` | 150 | −16.1 [−26.3, −5.9] |
| `g200` | 200 | −70.9 [−82.6, −59.3] |

Commits: `control` `03b4cfd`, `g125` `6ebc70a`, `g150` `62d0da6`, `g200`
`f41e312`. No fault.

## What was learned

Dropped. With E113, the weight of the gold held for a strike loses on both
sides of today's value (50%: −10.2, 125%: −11.5; further: −61.4 and −70.9):
it is at an optimum against v034. The owner chose next (2026-11-28) to fit
the evaluation's weights jointly on game outcomes, since single weights look
tuned while their combination may not be.
