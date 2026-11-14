---
id: E050
date: 2026-11-14
pull_request: "#321"
parent: ice-and-fire-v022
hypothesis: In v022's losses to the top 10 on CodinGame it trains a level-3 unit with a median income of 25 against the enemy's 38, which falls to 10 (below 10 after 42% of these trainings), where the top 10 train theirs with an income of 32 to 38; the reviews of 20 losses to the top 1 name level-3 upkeep sinking our income (22 to -6, 27 to 11) in 12 of them. E036 charges a level-3 unit one turn of its 20 upkeep; charging it 2 or 3 turns again, or making each gold of income below a floor count more, keeps the search from buying a level-3 unit it cannot afford to keep, and beats v022.
change: None released. Screening only: the plan bot's evaluation charging a level-3 unit 2 or 3 turns of upkeep rather than 1 (UPKEEP_TURNS, E036), or counting each gold of a side's income below 10 or 15 another 3 or 6 times.
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form: the next lever after the reviews
of v022's losses to zasmu, "Level-3 upkeep"), on `ice-and-fire-v022`.

## Why

The reviews of 20 of v022's CodinGame losses to the top 1
([reviews/2026-11-14-v022-zasmu-losses](../reviews/2026-11-14-v022-zasmu-losses/summary.md))
name the income drop after a level-3 training in 8 of the 10 games as red
and 4 of the 10 as blue: 22 to -6, 25 to 10, 27 to 11. A scratch scan of
all 150 of v022's games against the top 10 (each level-3 training, the
side's income at that ply and at its next):

| Side | Its game | Level-3 buys | Turn (median) | Income before | Income after | Enemy income then | After below 10 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| top 10 | lost | 38 | 16 | 32 | 18 | 22 | 21% |
| top 10 | won | 140 | 17 | 38 | 23 | 23 | 13% |
| v022 | lost | 111 | 18 | 25 | 10 | 38 | 42% |
| v022 | won | 55 | 15 | 35 | 23 | 26 | 22% |

The top 10 train more level-3 units than v022 (140 in their wins), but
when they can afford the upkeep. E036 charges a level-3 unit one turn of
its upkeep of 20, against 3 for income in the shared evaluation (E036 won
+7.6 on v017; 2 turns was +5.6, 3 the old charge).

## The change

`bots/plan/src/eval.rs`: `UPKEEP_TURNS` (E036, `[3, 1]` in v022) and
`INCOME_FLOOR`, the income below which each gold of a side's income counts
`LOW_INCOME_TURNS` more times (`None`: no floor, v022, which the base
commit keeps).

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "Upkeep" is `UPKEEP_TURNS`
for a level-3 unit (1 in v022), "floor" `INCOME_FLOOR` and "more"
`LOW_INCOME_TURNS`.

Run 43574883627, seed 50050, 800 pairs each over two runners:

| Variant | Commit | Upkeep | Floor, more | Elo against v022 | Score |
| --- | --- | ---: | --- | --- | ---: |
| f10 | `a524695` | 1 | 10, 3 | +5.2 [−3.5, +14.0] | 50.7% |
| f10t6 | `5543680` | 1 | 10, 6 | +3.9 [−5.2, +13.0] | 50.6% |
| f15 | `c90add4` | 1 | 15, 3 | −0.4 [−11.1, +10.2] | 49.9% |
| l3t2 | `b42555c` | 2 | none | −6.5 [−16.0, +3.0] | 49.1% |
| l3t3 | `91402c0` | 3 | none | **−10.9 [−20.5, −1.2]** | 48.4% |

The owner dropped it on 2026-11-14 ("Drop it (Recommended)"): the best,
+5, is unlikely to pass an SPRT that asks for +10 (E049's +14 in
screening was +6 in its SPRT).

## What was learned

Charging a level-3 unit more of its upkeep lost again, as in E036: the
search needs level-3 units, and the top 10 train more of them than we do.
A floor on income did nothing measurable. The level-3 trainings that sink
our income in the losses to the top 10 come with an income already far
below the enemy's (25 against 38): the cause is the economy behind them,
which the early game decides (E051 to E054), more than the purchase. The
knob was removed: the pull request holds the journal entry alone.
