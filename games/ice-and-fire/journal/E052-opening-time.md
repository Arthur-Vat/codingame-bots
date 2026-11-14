---
id: E052
date: 2026-11-14
pull_request: "#325"
parent: ice-and-fire-v022
hypothesis: The opening's plan is made on the first turn in about 610 ms of the 800 ms we use of CodinGame's 1,000, over 7 of our turns, and each later turn has 40 ms of CodinGame's 50; a longer first search (900 or 950 ms), a longer horizon (8 or 9 turns) or more time a turn (45 ms) finds better openings and better turns, and beats v022.
change: The plan bot's first-turn budget is 900 or 950 ms rather than 800, its opening plans 8 or 9 of our turns rather than 7, or each turn has 45 ms rather than 40.
release: ice-and-fire-v023 (its 9-turn opening and 45 ms a turn, with E051's spread units, through E055)
sprt: accepted, through E055
elo: +44.5 [+22.1, +67.4] at CodinGame's limits, E055's candidate
pairs: 400
full_time: see E055
decision: promoted, through E055
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

`bots/plan/src/main.rs`: `FIRST_BUDGET_MS` (800 in v022), `OPENING_TURNS`
(7) and `BUDGET_MS` (40). The variants change those constants only; the
base commit is v022. The timeouts of each variant are measured (the
screening counts them).

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer, 45 ms included). "First" is
`FIRST_BUDGET_MS`, "turns" `OPENING_TURNS`, "ms" `BUDGET_MS`.

First round, run 43575403550, seed 52052, 800 pairs each over two runners:

| Variant | Commit | First | Turns | ms | Elo against v022 | Score |
| --- | --- | ---: | ---: | ---: | --- | ---: |
| t9 | `2f0ca78` | 800 | 9 | 40 | **+23.5 [+8.7, +38.4]** | 53.4% |
| b45 | `1e3fbe3` | 800 | 7 | 45 | +10.0 [−0.0, +20.0] | 51.4% |
| f900 | `d85cbce` | 900 | 7 | 40 | +3.0 [−4.0, +10.1] | 50.4% |
| f950 | `f78a56d` | 950 | 7 | 40 | +2.6 [−5.8, +11.0] | 50.4% |
| t8 | `c3a20b1` | 800 | 8 | 40 | +0.9 [−12.6, +14.4] | 50.1% |

The two best again with a 10-turn plan, run 43576854829, seed 52053,
1,600 pairs each over four runners:

| Variant | Commit | Turns | ms | Elo against v022 | Score | Both rounds |
| --- | --- | ---: | ---: | --- | ---: | --- |
| t9 | `2f0ca78` | 9 | 40 | **+26.5 [+16.1, +37.1]** | 53.8% | about +25 over 2,400 pairs |
| b45 | `1e3fbe3` | 7 | 45 | **+8.5 [+1.6, +15.3]** | 51.2% | about +9 |
| t10 | `6b50abf` | 10 | 40 | +7.4 [−2.8, +17.5] | 51.1% | |

A longer first search did little; a plan over 9 turns gained, 8 and 10
less; 45 ms a turn gained a little. The owner chose to combine the best of
E051 and E052 before an SPRT: [E055](E055-spread-and-horizon.md)'s
candidate `ice-and-fire-v023` (spread units, 9 turns, 45 ms) was accepted,
+44.5 [+22.1, +67.4] against v022 (a first run +65.9). Its 9-turn plans
needed one more change, a cap on the opening's beam width when a turn has
no deadline, for CI's determinism check.
