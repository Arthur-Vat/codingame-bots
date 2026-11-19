---
id: E069
date: 2026-11-19
pull_request: "#361"
parent: ice-and-fire-v028
hypothesis: On zasmu's own positions our releases keep a strike on the enemy HQ affordable next turn in about half the tests (save-1 50-51%, zasmu 71%) and v028 does so in 55% of its CodinGame positions (v019-v026 62-66%); a search that may not spend a strike it could make next turn, as zasmu banks before its one cut, wins more games by that strike and beats v028.
change: When no strike of the opponent's threatens our HQ at the start of the turn and our chain of trainings to the enemy HQ (all our units stepping first) is affordable with our gold plus STRIKE_TURNS turns of income, the plan search refuses every action (and every forced first action) after which it no longer is, unless it wins; the opponent's modelled climb keeps no reserve.
release: none
sprt: not run: level in screening, judged on CodinGame as a probe
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: Legend, 4th of 88 (4th of 929 overall), 2026-11-19, as the probe ice-and-fire-e069-k1 (submission 41758936)
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

## The probe's ranking games

The ranking ended on 2026-11-19: **4th of 88** in Legend (score 29.88),
where `ice-and-fire-v028` had ended 5th (29.37). Its 159 ranking games were
fetched to the data branch (`scripts/codingame.sh fetch --submission
41758936`, then `--backfill-releases` from this branch, which holds the
probe's submission line; data branch 9a7fa3a). Read like v028's 159:

| In its ranking games | k1 | v028 |
| --- | ---: | ---: |
| save-1, its own positions | 100% (217) | 55% (286) |
| save-2 | 63% (464) | 62% (418) |
| defend-1 | 54% (93) | 52% (83) |
| defend-2 | 71% (362) | 69% (287) |
| finish | 97% (67) | 99% (74) |
| Wins: the opponent's income at the winning ply (median, red / blue) | 23 / 21 | 3 / 9 |
| Wins: distance from its land to the enemy HQ before it (median, red / blue) | 6.5 / 5 | 3 / 4 |
| Games won | 94 of 159 (59%) | 99 of 159 (62%) |
| against the top 6 of the snapshot | 29 of 78 (37%) | 27 of 64 (42%) |
| against the others | 65 of 81 (80%) | 72 of 95 (76%) |

(capability tests: `codingame-capability-suite --kinds ranking --release
... --per 1000000 --per-game 1000000`, then `codingame-capabilities` with
no bot; strike rows: `codingame-style --kinds ranking --release ...`.)

The reserve did what it was built for: k1 kept its strike in every
position it could, and it won as zasmu does, striking from farther away
while the opponent's income was still high, rather than finishing an
opponent already broken. It did not win more: one place higher in one
ranking, and win rates within noise of v028's against different mixes of
opponents. Self-play, where it was level, did not hide a gain that the
ranking games show.

## Decision

The owner chose (form, 2026-11-19) to try the reserve again on top of
`ice-and-fire-v029` (E070) as a new experiment. This one is closed:
`STRIKE_TURNS` stays in the plan bot at 0, which plays exactly as v029
(the same answers on the 200 positions of `speed-bank.txt` at 600 tries),
and the probe file stays in `probes/` (ADR 0060). After the probe,
v029 was submitted in its place (submission 41759272, #360).

