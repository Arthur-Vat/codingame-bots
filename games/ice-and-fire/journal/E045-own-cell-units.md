---
id: E045
date: 2026-11-14
pull_request: "#314"
parent: ice-and-fire-v021
hypothesis: In the collected CodinGame games against the top 10, from turn 7, we spend 7% to 9% of our gold on units trained on our own cells and the top 10 1%, while they spend 30% on towers and 13% on mines against our 18% and 6% to 9%; a level-1 unit on our own cell costs 10 and an upkeep every turn and only makes the enemy pay a level-2 training for that cell, so offering trainings on our own active cells only on the border, or not at all (E044's blockers may still put a unit where a reply enters), spends the gold where it pays and beats v021.
change: The plan bot's generator offers trainings on our own empty active cells everywhere (v021), only on cells next to a cell that is not active for us, or nowhere, with or without a level-1 blocker on the cells the remembered replies enter.
release: ice-and-fire-v022
sprt: accepted
elo: +30.5 [+14.1, +47.0] at CodinGame's limits
pairs: 400
full_time: +30.5 [+14.1, +47.0] (the SPRT's own, ADR 0042), 3 timeouts of v022 in 800 games (v021 none), slowest answer 625.2 ms (v022's first turn)
decision: promoted
cg_rank: Legend, 10th of 87 (10th of 926 overall), 2026-11-14: submitted through the API on the owner's approval (submission 41710997)
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
add a level-1 unit on the cells the remembered replies enter; the
release leaves it unset (`None`), so its blockers are towers only, as in
v021. The same generator builds the opponent's own plan (E019), so the
search no longer expects the opponent to train on its own active cells
either.

## Screening

All against `ice-and-fire-v021` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048). The variants nb1 and bb1 add E044's level-1 blocker (`BLOCK_LEVEL`
`Some(1)`) on the cells the remembered replies enter.

First round, run 43525437644, seed 45045, 800 pairs each over two runners:

| Variant | Commit | Own cells offered | Level-1 blocker | Elo against v021 | Score |
| --- | --- | --- | --- | --- | ---: |
| n | `8865600` | none | no | **+25.2 [+14.4, +36.1]** | 53.6% |
| b | `d90cf3c` | border | no | **+15.2 [+5.4, +25.0]** | 52.2% |
| nb1 | `54cbccf` | none | yes | +6.5 [−4.8, +17.8] | 50.9% |
| bb1 | `7d3b863` | border | yes | +5.6 [−4.8, +16.1] | 50.8% |

(bb1 had one timeout in 1,600 games; no other timeout, crash or invalid
answer.)

The two best again, run 43526765969, seed 45046, 1,600 pairs each over
four runners (no fault):

| Variant | Commit | Elo against v021 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| n | `8865600` | **+25.5 [+17.7, +33.2]** | 53.7% | about +25 |
| b | `d90cf3c` | **+17.8 [+10.6, +25.1]** | 52.6% | about +17 |

Fewer trainings on our own cells gained, and none at all gained most; the
level-1 blocker gave most of the gain back. The candidate is n
(`OWN_CELLS = 2`), whose bundle is the same, byte for byte, as the
screened commit's.

## SPRT

Accepted on the pull request (run 43529176353, at 91f3a010), at
CodinGame's speed and real limit, in blocks of 10 pairs over 8 calibrated
runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v022 vs ice-and-fire-v021: 800 games, 400 complete pairs
  ice-and-fire-v022: 435 wins, 0 draws, 365 losses (score 54.4%)
  Elo of ice-and-fire-v022 over ice-and-fire-v021: +30.5 [+14.1, +47.0] (95%, from pairs)
  pairs by ice-and-fire-v022's points (0, 1/2, 1, 3/2, 2): [30, 0, 305, 0, 65]
  ice-and-fire-v022 faults: 3 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v021 faults: 0 timeouts, 0 crashes, 0 invalid answers
  slowest answer: ice-and-fire-v022 625.2 ms, ice-and-fire-v021 616.5 ms
Checked 5 rounds of 8 blocks (40 blocks counted, 48 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.66 (bounds -2.94, 2.94) after 400 pairs
Verdict: ACCEPTED: ice-and-fire-v022 is stronger than ice-and-fire-v021
```
