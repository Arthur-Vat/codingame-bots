---
id: E053
date: 2026-11-14
pull_request: "#322"
parent: ice-and-fire-v022
hypothesis: On maps where the land between the HQs narrows to a passage of 2 or 3 cells, the side that holds it first keeps the land behind it; in our collected games against the top 10 such a passage (by the definition below) is rare (19 of 342 games) and the top 10 reached it first in 9 of the 11 decided as red; valuing those cells more in the opening's rules gets our units there first and beats v022.
change: None released. Screening only: the opening's rules valuing a cell of a narrow passage between the HQs (2 or 3 cells wide) 6 or 12 more.
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14: "Alright first work on improving the early
game (efficient planning until turn 7), by trying to have a similar
expansion as the one of the top bots. You should improve the evaluation
function and also maybe improve the search, you can try to go closer to
the time limit." He noted that our units are sometimes too close to each
other in the first turns, that on maps with corridors we should be first
at the choke points (then maybe lock them with a tower), and asked to
spend the gold and build mines at the best moment. One of four
experiments run in parallel on `ice-and-fire-v022` (E051 to E054).

## Why

The study of the first 10 turns is in E051's entry, run in parallel (its own pull request).

## The change

`opening/src/econ.rs`: `choke_cells`, the cells on paths between the HQs
at most 2 steps longer than the shortest, in the layers (cells at one
distance from red's HQ) within 3 steps of the middle that hold at most
`width` such cells; `Params::choke` and `Params::choke_width`, the bonus
of such a cell (0 in `Params::V003`); `bots/plan/src/main.rs`:
`OPENING_CHOKE` and `OPENING_CHOKE_WIDTH` (0 and 2: v022, which the base
commit keeps). A tower to hold the passage is left to the middle game's
search, which offers towers where they guard cells the enemy can take
(E026, E044).

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). Run 43575408344, seed 53053,
800 pairs each over two runners:

| Variant | Commit | Bonus | Passage of at most | Elo against v022 | Score |
| --- | --- | ---: | ---: | --- | ---: |
| c6w3 | `982b8b1` | 6 | 3 cells | +3.5 [−5.8, +12.8] | 50.5% |
| c12w2 | `23cb1cf` | 12 | 2 cells | +2.6 [−6.5, +11.7] | 50.4% |
| c6w2 | `1f2b8d7` | 6 | 2 cells | −3.5 [−12.4, +5.5] | 49.5% |
| c12w3 | `406bf77` | 12 | 3 cells | −7.4 [−17.1, +2.3] | 48.9% |

No variant gained; none was confirmed.

## What was learned

Narrow passages between the HQs are rare on this game's maps (19 of 342
of our games against the top 10 by this definition, about 6%), so a bonus
for them can change few games, and none measurably. The passage that
matters may also be another than this definition finds (a narrow layer
near the middle of the near-shortest paths); a better test would pick the
maps that have one, which the screening does not. Spreading the units
(E051) and a longer opening plan (E052) gained in the same round. The
knob was removed: the pull request holds the journal entry alone.
