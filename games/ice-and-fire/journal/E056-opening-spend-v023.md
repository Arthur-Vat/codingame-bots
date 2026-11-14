---
id: E056
date: 2026-11-14
pull_request: "#326"
parent: ice-and-fire-v023
hypothesis: The top 10 spend more than we do at turns 6 and 7 (11 and 16 gold as red against our 5 and 8); E046 found that weighing gold less in our opening lost (about -50), but its units crowded each other, so that more units took no more land; with v023's units spread (E051, E055), each new unit finds a cell of its own, so spending more in the opening (gold weighed less in the plan, or trainings that pay back by a later turn, or units expected to take more cells after contact) takes more land and beats v023.
change: The opening's plan weighs gold 0 or half as much as v023 (every other weight doubled), or its purchases count a unit as paying back by game turn 18 or 21 rather than 15, or a unit as taking 2 cells after the territories meet rather than 0.
release: ice-and-fire-v024
sprt: accepted
elo: +19.7 [+6.9, +32.7] at CodinGame's limits
pairs: 1040
full_time: +19.7 [+6.9, +32.7] (the SPRT's own, ADR 0042), no fault of either side in 2,080 games
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-14 (a form after v023: "Spend more on v023
(Recommended)").

## Why

The scratch study of the first 10 turns against the top 10 (in E051's
entry) found the top 10 spending more at turns 6 and 7, and our units
crowding each other. E055 (`ice-and-fire-v023`) spread them: a cell is
worth 2 less to the opening for each of our units next to it, and the
plan looks 9 turns ahead. E046 tried spending more on v021 by weighing
gold half as much (−49) or allowing mines (−25), and E034 found gold
weighed 0 worse (−24.8), all with crowding units.

## The change

`bots/plan/src/main.rs`: `OBJECTIVE` (the opening plan's weights; gold 1
in v023), and `OPENING_HORIZON` and `OPENING_AFTER_CONTACT`, set on every
weight set of the opening (`Params::horizon`, the game turn by which a
purchase must pay back, 15; `Params::after_contact`, the cells a unit is
expected to take after the territories meet, 0). The base commit is v023.

## Screening

Against `ice-and-fire-v023` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; one timeout of h17 in 3,200 games, no other fault). "Gold" is the
opening plan's gold weight, "horizon" `OPENING_HORIZON`, "after"
`OPENING_AFTER_CONTACT`.

First round, run 43583082117, seed 56056, 800 pairs each over two runners:

| Variant | Commit | Change | Elo against v023 | Score |
| --- | --- | --- | --- | ---: |
| h18 | `d9466db` | horizon 18 | **+20.4 [+5.0, +35.9]** | 52.9% |
| h21 | `f4de392` | horizon 21 | **−27.9 [−42.8, −13.0]** | 46.0% |
| gh | `d290bc9` | gold half (the other weights doubled) | **−47.6 [−62.9, −32.6]** | 43.2% |
| g0 | `26af94a` | gold 0 | **−48.1 [−63.5, −32.8]** | 43.1% |
| ac2 | `66e1eb9` | after 2 | **−73.2 [−89.0, −57.6]** | 39.6% |

The leader again with its neighbours, run 43583915667, seed 56057, 1,600
pairs each over four runners:

| Variant | Commit | Horizon | Elo against v023 | Score | Both rounds |
| --- | --- | ---: | --- | ---: | --- |
| h18 | `d9466db` | 18 | **+24.6 [+13.9, +35.3]** | 53.5% | about +23 over 2,400 pairs |
| h17 | `9418d34` | 17 | −3.7 [−10.4, +3.0] | 49.5% | |
| h19 | `0750811` | 19 | **−20.7 [−31.1, −10.2]** | 47.0% | |

Weighing gold less in the plan lost again, as in E034 and E046, even with
spread units; so did expecting units to take cells after contact. A
slightly later payback buys a few more units and gained, at 18 only: 17
did not and 19 lost, a sharp peak that the owner chose to let the SPRT
judge ("Freeze and SPRT (Recommended)"). The candidate is h18
(`OPENING_HORIZON = 18`), whose bundle is the same, byte for byte, as the
screened commit's.

## SPRT

Accepted on the pull request (run 43591624972, at 88a28071), at
CodinGame's speed and real limit, in blocks of 10 pairs over 8 calibrated
runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v024 vs ice-and-fire-v023: 2080 games, 1040 complete pairs
  ice-and-fire-v024: 1099 wins, 0 draws, 981 losses (score 52.8%)
  Elo of ice-and-fire-v024 over ice-and-fire-v023: +19.7 [+6.9, +32.7] (95%, from pairs)
  pairs by ice-and-fire-v024's points (0, 1/2, 1, 3/2, 2): [165, 0, 651, 0, 224]
  ice-and-fire-v024 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v023 faults: 0 timeouts, 0 crashes, 0 invalid answers
Checked 13 rounds of 8 blocks (104 blocks counted, 64 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.42 (bounds -2.94, 2.94) after 1040 pairs
Verdict: ACCEPTED: ice-and-fire-v024 is stronger than ice-and-fire-v023
```

The sharp peak held: about +20 in the SPRT, as in both screenings.
