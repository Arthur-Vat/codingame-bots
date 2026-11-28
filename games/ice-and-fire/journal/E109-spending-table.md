---
id: E109
date: 2026-11-28
pull_request: "#408"
parent: ice-and-fire-v034
hypothesis: zasmu spends its gold differently (fewer trainings, more mines and towers, and nothing at all in 47% of its turns with 80 gold or more); ordering our purchases by its spending table, or keeping only its likeliest kinds of purchase, spends ours better and beats ice-and-fire-v034.
change: A table of zasmu's purchases by game turn band and gold band (trainings by level and cell, towers by cell, mines), packed in the bot; switches in our climbs that order purchases by it (`SPEND_ORDER`) or offer only the band's k likeliest kinds (`SPEND_KINDS`).
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

Stage 6 of the type planner
([2026-11-28-type-planner.md](../research/codingame/2026-11-28-type-planner.md)).

## Change

On the branch `claude/ice-and-fire-e109-spending` (from E106's): purchase
kinds for each training (level 1 to 3; capture, front, near, far), tower (on
a joint, next to one, front, near, far) and mine; an analysis mode `spending`
and a counting script beside stage 1's; the table packed as stage 1's. With
the switches off the answers are those of the base (`judge`, 600 tries,
2,031 positions: byte-identical). The bundle of a variant: 86,750
characters.

The table predicts zasmu's purchases poorly: on held-out games from game turn
5, its most frequent kind is right 28.7% of the time, its three most
frequent 61.1% (the unit tables: 56.5% and 90.8%). zasmu's replies that buy
nothing, by gold band on all games: 2% at a turn where it can pay a chain to
our HQ, 4% under 40 gold, 14% at 40 to 79, 47% at 80 or more.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46146564520](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46146564520)):

| Variant | `SPEND_ORDER` | `SPEND_KINDS` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `control` | off | none | −1.7 [−8.0, +4.5] |
| `order` | on | none | +5.4 [−2.9, +13.8] |
| `keep4` | on | 4 | −29.4 [−40.1, −18.7] |
| `keep6` | on | 6 | −7.4 [−17.8, +3.0] |

Commits: `control` `dd8984f`, `order` `d9a0bca`, `keep4` `060f2f9`, `keep6`
`faf18e4`. One timeout of `order`'s side; none otherwise. With purchases
pruned, `keep4` answered faster (median 14.6 against 21.7 ms on one shard):
its climbs ran out of actions sooner.

## What was learned

Dropped: the order is within the screening's error, and pruning purchases
loses, as pruning moves did in E106. The code stays on the branch (commit
`f70aaebc`, the switches off).
