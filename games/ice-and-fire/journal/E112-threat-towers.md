---
id: E112
date: 2026-11-28
pull_request: "#410"
parent: ice-and-fire-v034
hypothesis: A tower should stand where the enemy can strike; offering towers only within k steps of the enemy's land or where they cover a cell of the enemy's affordable cuts or chain to our HQ, and trying the nearest first, spends our gold better and beats ice-and-fire-v034.
change: Switches in our climbs (never the opponent's model): towers kept only within `TOWER_KEEP` steps of the enemy's land or covering a threat cell (the enemy's four best affordable cuts and its affordable chain to our HQ at its next turn, from our turn's start), and `TOWER_NEAR_FIRST`, towers nearest the enemy tried first among actions of equal last gain.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

After E110 the owner (2026-11-28): "No towers within 3 steps is a bad rule,
it should be based on threat by the opponent. No tower within k steps from
the border or potential cuts, ideally closer to our border would already
better." His choices in a form: near the border or on a threat; k = 2 and 3;
a variant that orders towers by distance; the whole game.

## Change

On the branch `claude/ice-and-fire-e112-threat-towers` (from `main`), the
module `search/towers.rs`: `threat_cells` (the cells of the enemy's four best
cuts with its gold and income, `tactics::cut::cuts`, and of its chain to our
HQ if it can pay it, `chain::chain_to`, from our turn's start, once a
search), `keep_towers` (a tower action kept if every tower it builds stands
within k steps of the enemy's land or covers a threat cell) and `rank` (a
tower action's distance to the enemy's land). With every switch off the
answers are those of `main` (`judge`, 600 tries, 2,031 positions:
byte-identical). Two tests.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46151356579](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46151356579)):

| Variant | `TOWER_KEEP` | `TOWER_NEAR_FIRST` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `control` | none | off | −2.4 [−8.5, +3.7] |
| `keep2` | 2 | off | +2.2 [−6.2, +10.5] |
| `keep3` | 3 | off | −1.3 [−8.9, +6.3] |
| `near` | none | on | +2.2 [−4.1, +8.5] |
| `keep2near` | 2 | on | −1.3 [−9.6, +7.0] |

Commits: `control` `022b71c`, `keep2` `331b9ac`, `keep3` `2043ea2`, `near`
`9cc3ab4`, `keep2near` `9ea07a3`. One timeout of v034 against `keep3` and one
against `near`; no other fault.

## What was learned

Dropped: all level. Our search's towers already answer threats (the tower
study: 93% change the enemy's next turn), so a rule by threat or by distance
removes few of them, and ordering them changes little.
