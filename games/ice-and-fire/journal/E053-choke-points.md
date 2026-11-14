---
id: E053
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: On maps where the land between the HQs narrows to a passage of 2 or 3 cells, the side that holds it first keeps the land behind it; in our collected games against the top 10 such a passage (by the definition below) is rare (19 of 342 games) and the top 10 reached it first in 9 of the 11 decided as red; valuing those cells more in the opening's rules gets our units there first and beats v022.
change: The opening's rules value a cell of a narrow passage between the HQs (2 or 3 cells wide) 6 or 12 more.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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

The study of the first 10 turns is in [E051](E051-spread-units.md).

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

Pending.
