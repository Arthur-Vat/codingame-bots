---
id: E055
date: 2026-11-14
pull_request: "#324"
parent: ice-and-fire-v022
hypothesis: E051 (our units spread in the opening, each of our units next to a cell lowering its value by 2) and E052 (the opening planned over 9 of our turns rather than 7) each gained about +25 to +30 against v022 on 2,400 pairs, by different means (how units move, how far the plan looks); together they gain more than either, and beat v022.
change: The opening's rules value a cell 2 or 3 less for each of our units next to it (E051) and the opening plans 9 of our turns (E052), with 40 or 45 ms a turn.
release: ice-and-fire-v023
sprt: accepted (and a first run, before the fix below, +65.9 [+41.0, +91.5])
elo: +44.5 [+22.1, +67.4] at CodinGame's limits
pairs: 400
full_time: +44.5 [+22.1, +67.4] (the SPRT's own, ADR 0042), no fault of either side in 800 games
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-14 (a form, after the confirmations of E051
and E052: "Combine, then SPRT (Recommended)"): the best of E051 with the
best of E052, screened against `ice-and-fire-v022` with each alone in the
same run, the best form then frozen as the candidate.

## Why

On 1,600 new pairs against v022 (runs 43576849849 and 43576854829):

| Experiment | Variant | Elo against v022 | Both rounds, 2,400 pairs |
| --- | --- | --- | --- |
| E051 | crowd 2 | +30.0 [+19.9, +40.3] | about +31 |
| E051 | crowd 3 | +27.9 [+16.3, +39.4] | about +25 |
| E052 | 9 turns | +26.5 [+16.1, +37.1] | about +25 |
| E052 | 45 ms a turn | +8.5 [+1.6, +15.3] | about +9 |

The study of the first 10 turns and each screening are in E051's and
E052's entries (their own pull requests).

## The change

`opening/src/econ.rs` and `bots/plan/src/main.rs` as in E051
(`Params::crowd`, `OPENING_CROWD`), with E052's constants
`OPENING_TURNS` and `BUDGET_MS` of `bots/plan/src/main.rs`. The base
commit is v022's settings.

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer in 16,000 games). "Crowd" is
`OPENING_CROWD`, "turns" `OPENING_TURNS`, "ms" `BUDGET_MS`.

Run 43578164479, seed 55055, 1,600 pairs each over four runners:

| Variant | Commit | Crowd | Turns | ms | Elo against v022 | Score |
| --- | --- | ---: | ---: | ---: | --- | ---: |
| c2t9b45 | `2f97b28` | 2 | 9 | 45 | **+61.2 [+50.1, +72.5]** | 58.7% |
| c2t9 | `92364b0` | 2 | 9 | 40 | **+54.1 [+43.0, +65.2]** | 57.7% |
| c3t9 | `d1506b5` | 3 | 9 | 40 | **+48.3 [+36.6, +60.1]** | 56.9% |
| c2 | `44064d4` | 2 | 7 | 40 | **+45.9 [+35.5, +56.3]** | 56.6% |
| t9 | `d8d6b25` | 0 | 9 | 40 | **+21.7 [+11.7, +31.8]** | 53.1% |

The two changes add up: spread units alone about +46 in this run, the
9-turn plan alone +22, both +54, and 45 ms a turn on top +61. The
candidate is c2t9b45 (`OPENING_CROWD = 2`, `OPENING_TURNS = 9`,
`BUDGET_MS = 45.0`).

CI's determinism check (`scripts/check-determinism.sh`: two runs at a
fixed number of tries, where a turn has no deadline, at a time scale of
50) failed once on the first frozen bundle: without a deadline the
opening replans with beams up to 64 wide, which over 9 turns took about
1.5 s around turn 7 (7 turns: 1.0 s), near the 2.5 s the arena then gives
an answer. The candidate caps that width at 32 (`OPENING_MAX_WIDTH`),
which matters only without a deadline (slowest such answer 773 ms); with
a deadline, as on CodinGame and in every screening and SPRT, the plans
search until it whatever the width. So the candidate differs from the
screened commit in that constant, and v023 was frozen again.

## SPRT

A first SPRT accepted the bundle frozen before the width fix (run
43579853180, at 1890fbf6), at CodinGame's speed and real limit, in blocks
of 10 pairs over 8 calibrated runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v023 vs ice-and-fire-v022: 640 games, 320 complete pairs
  ice-and-fire-v023: 380 wins, 0 draws, 260 losses (score 59.4%)
  Elo of ice-and-fire-v023 over ice-and-fire-v022: +65.9 [+41.0, +91.5] (95%, from pairs)
  pairs by ice-and-fire-v023's points (0, 1/2, 1, 3/2, 2): [41, 0, 178, 0, 101]
  ice-and-fire-v023 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v022 faults: 0 timeouts, 0 crashes, 0 invalid answers
Checked 4 rounds of 8 blocks (32 blocks counted, 56 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.90 (bounds -2.94, 2.94) after 320 pairs
Verdict: ACCEPTED: ice-and-fire-v023 is stronger than ice-and-fire-v022
```

The SPRT of the candidate as frozen again, accepted (run 43581613228, at
bab6959b):

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v023 vs ice-and-fire-v022: 800 games, 400 complete pairs
  ice-and-fire-v023: 451 wins, 0 draws, 349 losses (score 56.4%)
  Elo of ice-and-fire-v023 over ice-and-fire-v022: +44.5 [+22.1, +67.4] (95%, from pairs)
  pairs by ice-and-fire-v023's points (0, 1/2, 1, 3/2, 2): [63, 0, 223, 0, 114]
  ice-and-fire-v023 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v022 faults: 0 timeouts, 0 crashes, 0 invalid answers
Checked 5 rounds of 8 blocks (40 blocks counted, 64 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.05 (bounds -2.94, 2.94) after 400 pairs
Verdict: ACCEPTED: ice-and-fire-v023 is stronger than ice-and-fire-v022
```

The two runs agree within their intervals (together about +54 over 720
pairs), as the screening's +61.2 on 1,600 pairs.
