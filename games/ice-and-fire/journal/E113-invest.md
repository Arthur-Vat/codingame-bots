---
id: E113
date: 2026-11-28
pull_request: "#410"
parent: ice-and-fire-v034
hypothesis: Our evaluation counts gold twice (as material and in the pressure term towards the chain to the enemy HQ), so it never builds a mine when it could; at game turns 9 to 14, where zasmu builds 2.7 times our mines, counting the gold held for a strike less (an "invest" strategy, the first of a portfolio) makes us build mines and beats ice-and-fire-v034.
change: A switch in the evaluation (both sides): from game turn 9 to 14, the pressure term and the standing threat scaled to `INVEST_PERCENT` of their value (100 elsewhere and when off).
release: none
sprt: not run: every variant lost in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

E111's mine bonus did not make us build more mines. On 800 of our turns at
game turns 9 to 14 in the games against zasmu with 15 gold and a free safe
mine spot where we built none, a mine added to the plan our search chose
lowered its score (median −11): the shared evaluation counts 15 gold spent
about 30 (once as material, once in the pressure term while the chain to the
enemy HQ is not affordable), a mine's 4 of income about 16. The owner asked
(2026-11-28) for a framework change, a portfolio of strategies chosen by a
rule from the phase and the state; this is its first strategy alone, by game
turn.

## Change

On the branch `claude/ice-and-fire-e113-invest` (from `main`), in
`eval::eval`: `invest_discount` takes off `(100 − INVEST_PERCENT)%` of the
pressure term (`margin.clamp(−200, 0)`) plus the standing threat
(`standing`), ours less the enemy's, from `INVEST_FROM` (9) to `INVEST_TO`
(14). At 100, nothing changes.

On the same 800 turns, the plans the search chose built a mine in 103 (13%)
with today's weights, 166 (21%) at 50% and 206 (26%) at 0%.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46152701174](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46152701174)):

| Variant | `INVEST_PERCENT` | Elo over v034 [95%] |
| --- | ---: | ---: |
| `control` | 100 | +1.5 [−4.0, +7.0] |
| `inv50` | 50 | −10.2 [−20.1, −0.3] |
| `inv25` | 25 | −18.5 [−28.5, −8.4] |
| `inv0` | 0 | −61.4 [−72.1, −50.9] |

Commits: `control` `16e7998`, `inv50` `9b36d9a`, `inv25` `ebed26b`, `inv0`
`7df0777`. One timeout of the control; no other fault.

## What was learned

Dropped: the less the gold held for a strike counts, the more it loses.
Against v034 that gold is worth more than the mines it would buy: it keeps a
strike on the enemy HQ within reach and forces the enemy to answer it. The
test is against our own bot; whether investing pays against zasmu is not
known. The owner chose next to test the opposite, gold for the strike
counted more (E115), before setting the weights of the portfolio's
strategies.
