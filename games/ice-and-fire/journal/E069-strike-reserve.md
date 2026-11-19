---
id: E069
date: 2026-11-19
pull_request: "#NN"
parent: ice-and-fire-v028
hypothesis: On zasmu's own positions our releases keep a strike on the enemy HQ affordable next turn in about half the tests (save-1 50-51%, zasmu 71%) and v028 does so in 55% of its CodinGame positions (v019-v026 62-66%); a search that may not spend a strike it could make next turn, as zasmu banks before its one cut, wins more games by that strike and beats v028.
change: When no strike of the opponent's threatens our HQ at the start of the turn and our chain of trainings to the enemy HQ (all our units stepping first) is affordable with our gold plus STRIKE_TURNS turns of income, the plan search refuses every action (and every forced first action) after which it no longer is, unless it wins; the opponent's modelled climb keeps no reserve.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-19, after the sparring bot was dropped
(#355): stage B of the curriculum in
[research/codingame/2026-11-18-zasmu-strategy.md](../research/codingame/2026-11-18-zasmu-strategy.md),
a strategic layer whose first goal is the strike, judged by the capability
tests on zasmu's positions and by the SPRT against v028.

## Why

- The capability tests ([capabilities/README.md](../capabilities/README.md)):
  on suite v2, zasmu's own positions, v026 to v028 keep the strike
  affordable next turn in 50-51% of the save-1 tests, zasmu in 71% (76% over
  all its positions); in their CodinGame ranking games, v028 in 55%, v019 to
  v026 in 62-66%. The tower wall of E068 spends gold the strike needs.
- The banker's rule (keep the gold when the chain is affordable next turn)
  passes save-1 on 88% of zasmu's positions, in v001 and in the dropped
  sparring bot.
- The evaluation already counts the shortfall to that chain (pressure) and
  the gold left over (standing, E042), but the search trades them away for
  a training or a tower that scores more after the opponent's replies.
- E007 (a flat 20-gold reserve, except for attacks) lost: this reserve
  holds only when a strike is within reach.

## The change

`STRIKE_TURNS` in [bots/plan/src/search.rs](../bots/plan/src/search.rs),
0 for v028. The test of affordability is the capability tests' own (chain
cost with every unit stepping first, `cost_to_moving`), on the state the
plan ends in, before the opponent's replies. A threat on our HQ lifts the
reserve, as in the banker.

## Capability tests

Suite v2 (zasmu's own positions), `--tries 600`, the bot built from this
branch with `STRIKE_TURNS` 1 (k1) and 2 (k2), v028 compiled from its
release file:

| Capability | k1 | k2 | v028 | zasmu |
| --- | ---: | ---: | ---: | ---: |
| finish | 100% | 100% | 100% | 100% |
| save-1 | 100% | 53% | 51% | 71% |
| save-2 | 69% | 100% | 69% | 43% |
| defend-1 | 78% | 78% | 78% | 81% |
| defend-2 | 93% | 94% | 96% | 62% |
| wall | 49% | 47% | 51% | reference |

Each variant does what its rule enforces, and defence and the wall stay
within noise of v028's.

## Screening

`screen.yml` run 44510100851, 800 pairs per variant against
`ice-and-fire-v028`, 2 shards each, at CodinGame's speed and real limit:

| Variant | Ref | Pairs | Elo over v028 [95%] | Score | Faults |
| --- | --- | ---: | --- | ---: | --- |
| k1 | `9a44a5c` | 800 | −6.5 [−16.8, +3.7] | 49.1% | none |
| k2 | `d07e561` (branch `claude/ice-and-fire-e069-k2`) | 800 | −65.0 [−78.4, −51.8] | 40.8% | none |

k2 is clearly weaker: two turns of saving hand v028 two turns of
spending. k1 is level in self-play, where v028 defends against a strike
two turns away in 96% of the tests (zasmu 62%), so self-play may hide what
it is worth against the top players.

## Probe on CodinGame

The owner chose to judge k1 on CodinGame (form, 2026-11-19) and approved
the probe, under [ADR 0060](../../../docs/adr/0060-probe-submissions.md):
[`probes/ice-and-fire-e069-k1.rs`](../probes/ice-and-fire-e069-k1.rs),
the minified bundle of `ice-and-fire-bot-plan` at commit 69ba44f
(`STRIKE_TURNS` 1; SHA-256
22cdaf5c730d2e81ff16390b0885aacca767719a159b7d97fb308617c1dff80e), 44,070
characters; 20 pairs against v028 in the arena, no fault. Three test games on CodinGame against the top 3 (zasmu, karliso,
Pduhard-), 2026-11-19: no fault, every answer after the first in 45 ms or
less, all three lost. Submitted through the API on 2026-11-19 at 11:28 UTC:
submission 41758936, replacing `ice-and-fire-v028` (5th of 88 in Legend).

