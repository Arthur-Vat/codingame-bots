---
id: E107
date: 2026-11-28
pull_request: "#408"
parent: ice-and-fire-v034
hypothesis: Our units crowd from game turn 4 and fall behind in land at turns 9 to 12; whole turns sampled from zasmu's unit tables, with a matching that keeps units apart, as starts of our climbs (nothing pruned), and the search playing from game turn 5 instead of the opening, spread our units and beat ice-and-fire-v034.
change: Switches in our climbs (never the opponent's model): at the root, 32 sampled whole turns (each unit's kind drawn from its type's table, its cell chosen against crowding, no step uncovering a cell the enemy can enter), the best 2 or 4 started as forced climbs; and the handover from the opening at game turn 5.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

Stages 4 and 5 of the type planner
([2026-11-28-type-planner.md](../research/codingame/2026-11-28-type-planner.md)),
after E106 found that pruning by zasmu's per-unit kinds loses and that by
kind our units already act like zasmu's: the gap is in where units go
together (our units have 0.73 own units next to them at game turn 8, zasmu's
0.34) and in the early game. The owner chose (2026-11-28) a matching that
keeps units apart, sampled whole turns as climb starts with nothing pruned,
and the search from game turn 5.

## Change

`bots/plan/src/search/matching.rs` (new) and `search.rs`, on the branch
`claude/ice-and-fire-e107-matching` (from E106's): `MATCH_STARTS` (0),
`MATCH_SAMPLES` (32), `MATCH_FROM_TURN` (7). A sampled turn takes the side's
movable units in a seeded random order; each draws a kind from its type's
prior among the kinds it can do, and the kind becomes the legal cell of
lowest crowding cost (3 for each own unit next to it, 2 for each cell chosen
this turn within 2 steps, −1 for each free neutral cell next to it), never a
step that leaves empty a cell the enemy can enter at its next turn unless it
captures. The samples are scored against the memory of replies (empty before
the first climb) and the best `MATCH_STARTS` distinct ones start forced
climbs (E029). `HANDOVER_TURN` (E067) set to `Some(5)` in two variants.

With every switch off the answers are those of the base (`judge`, 600
tries, 2,031 positions: byte-identical). A search at 600 tries: 11.20 ms off,
10.95 and 10.93 with 2 and 4 starts. The bundle: 86,777 characters.

Twenty local pairs of `m5` against v034: +17.4 [−86.2, +124.3]; land lead at
game turn 12 −1 (v034 −3), units at turn 7 4.42 (4.20), own units next to
each unit at turn 8 0.65 (0.78).

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46144227415](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46144227415)):

| Variant | Matched starts | From game turn | Handover | Elo over v034 [95%] |
| --- | ---: | ---: | ---: | ---: |
| `control` | 0 | – | 7 | −0.2 [−6.1, +5.7] |
| `m7` | 2 | 7 | 7 | −1.1 [−11.2, +9.1] |
| `h5` | 0 | – | 5 | −35.3 [−46.7, −24.0] |
| `m5` | 2 | 5 | 5 | −33.5 [−45.2, −22.0] |
| `m5x4` | 4 | 5 | 5 | −10.6 [−22.2, +0.9] |

Commits: `control` `8781790`, `m7` `4781aea`, `h5` `6b5f6dd`, `m5` `b56816f`,
`m5x4` `e3130af`. No fault in any variant.

## What was learned

Dropped. From game turn 7 the matched starts change nothing. The opening
plays game turns 5 and 6 better than our search (−35 when the search takes
them), and four matched starts win back about 25 of those 35 Elo: once the
search plays the early turns, keeping units apart helps it, but not enough to
beat the opening, which already plays them well. The code stays on the
branch (commit `b964438d`, the switches off).
