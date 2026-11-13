---
id: E045
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v021
hypothesis: In the collected CodinGame games against the top 10, from turn 7, we spend 7% to 9% of our gold on units trained on our own cells and the top 10 1%, while they spend 30% on towers and 13% on mines against our 18% and 6% to 9%; a level-1 unit on our own cell costs 10 and an upkeep every turn and only makes the enemy pay a level-2 training for that cell, so offering trainings on our own active cells only on the border, or not at all (E044's blockers may still put a unit where a reply enters), spends the gold where it pays and beats v021.
change: The plan bot's generator offers trainings on our own empty active cells everywhere (v021), only on cells next to a cell that is not active for us, or nowhere, with or without a level-1 blocker on the cells the remembered replies enter.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 ("Go on with #2 in parallel"), the
second of the proposals made after a study of the gold spent per turn,
run on top of E044's candidate (`ice-and-fire-v021`).

## Why

The same scratch scan of the collected games as E044's (each turn of
either side: the gold held, spent and kept, and what it bought), in the
games against the top 10, from turn 7:

| Share of the gold spent | v014: us | Top 10 | v020: us | Top 10 |
| --- | ---: | ---: | ---: | ---: |
| Trainings on neutral or enemy cells | 66% | 55% | 66% | 56% |
| Trainings on our own cells | 9% | 1% | 7% | 1% |
| Towers | 19% | 31% | 18% | 30% |
| Mines | 6% | 13% | 9% | 13% |

(v014's 2,750 test games; v020's first 70.) We also spend less per turn
in the middle game (a median 20 against 30 at turns 10 to 15) with as
much gold at hand. The generator trains at the lowest level that may
enter a cell (`actions.rs`), which on our own empty cells is level 1, all
over our territory.

## The change

`bots/plan/src/actions.rs`: `OWN_CELLS` sets which of our own empty
active cells get a training offered: 0, all (v021, which the base commit
keeps); 1, those next to a cell that is not active for us; 2, none. Our
inactive cells stay offered (a training there joins them again), and the
cells offered towers and mines are unchanged. E044's `BLOCK_LEVEL` can
add a level-1 unit on the cells the remembered replies enter.

## Screening

Pending.
