---
id: E067
date: 2026-11-15
pull_request: pending
parent: ice-and-fire-v026
hypothesis: In v025's and v026's CodinGame games against the top 5 we hold 13 to 16 gold at game turn 7 and keep 65% (red) to 88% (blue) of it, where the top 5 in the same seat keep 14% to 24% (the red ones train, the blue ones build a tower in 59% of their games); the opening plays that turn in over 90% of our games and buys nothing in 84% (red) to 100% (blue) of them, since its purchases must pay back before the territories meet, while the search, when it plays turn 7, trains 1.7 units and at turn 8 builds a tower in half of blue's games; handing the turn to the search when the opening would keep the gold of a training spends that gold a turn sooner, and beats v026.
change: The plan bot's search plays, and the opening ends for good, from game turn 7 (screened also: from the first turn at which the opening's turn buys nothing while we hold 10 or 15 gold, and from game turns 6 and 8).
release: ice-and-fire-v027
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-15 ("Go on with ideas 1 and 2 together":
spending at turn 7, and blue's tower at turn 7), after the analysis of
v025's and v026's games against the top 5 in both seats.

## Why

From our debug lines in v026's 176 games against the top 5 (the opening
plays a turn when its line says 0 climbs):

| Game turn | Seat | Played by | Turns | Trainings a turn | Towers a turn | Without a purchase |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| 6 | red | opening | 160 | 0.97 | 0 | 3% |
| 7 | red | opening | 145 | 0.19 | 0 | 84% |
| 7 | red | search | 15 | 1.73 | 0 | 0% |
| 8 | red | search | 141 | 1.10 | 0.22 | 1% |
| 7 | blue | opening | 15 | 0 | 0 | 100% |
| 8 | blue | search | 15 | 1.13 | 0.53 | 7% |

The opening's purchases were tuned in E034, E046 and E056: buying more by
its own rule (a later payback, units expected to take cells after
contact, gold weighed less) lost, except a payback at turn 18. This
experiment leaves the rule alone and changes who plays the turn.

## The change

`bots/plan/src/search.rs`, `think`: with `HANDOVER_IDLE`, when the
opening's turn holds no training and no building, the game turn is at
least `HANDOVER_IDLE_FROM` and we hold `HANDOVER_GOLD` or more, the
opening ends for good (`Planned::end`) and the search plays the turn in
the time left; `HANDOVER_TURN` ends the opening at a fixed game turn.
The search offers towers where they guard something (E026), so blue's
tower at turn 7 is its choice.

## Screening

Against `ice-and-fire-v026` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer).

First round, run 43707210397, seed 67067, 800 pairs each over two runners:

| Variant | Commit | The search plays from | Elo against v026 | Score |
| --- | --- | --- | --- | ---: |
| d | `f94e6c3` | game turn 7 | **+23.9 [+7.7, +40.2]** | 53.4% |
| c | `e0eefd7` | the opening's first turn without a purchase with 15 gold, from turn 6 | +13.0 [−3.0, +29.1] | 51.9% |
| a | `320d38c` | the same with 10 gold | +12.6 [−3.6, +28.9] | 51.8% |
| e | `a95ce33` | game turn 8 | +6.1 [−3.4, +15.6] | 50.9% |
| b | `c191c64` | the first turn without a purchase with 10 gold, from turn 7 | +5.6 [−9.7, +21.1] | 50.8% |
| f | `7c92e87` | the same from turn 4 | +0.4 [−15.2, +16.1] | 50.1% |

The leader again with its neighbours, run 43708114034, seed 67068, 1,600
pairs each over four runners (one timeout of g and one of v026 in 9,600
games):

| Variant | Commit | The search plays from | Elo against v026 | Score | Both rounds |
| --- | --- | --- | --- | ---: | --- |
| d | `f94e6c3` | game turn 7 | **+29.4 [+17.9, +41.0]** | 54.2% | about +28 over 2,400 pairs |
| h | `f569432` | game turn 7, or turn 6 when the opening buys nothing with 10 gold | **+25.9 [+14.5, +37.3]** | 53.7% | |
| g | `04e9b9d` | game turn 6 | **−26.8 [−38.3, −15.3]** | 46.2% | |

Turn 7 gains in both rounds; turn 6 loses as much, and turn 8 is level:
the opening's sixth turn, which buys in 97% of our games, is worth
keeping, and its seventh, which buys nothing, is not.

The candidate is d (`HANDOVER_TURN = Some(7)`, `HANDOVER_IDLE = false`),
frozen as `ice-and-fire-v027` on the owner's choice ("Freeze and SPRT
(Recommended)", 2026-11-15); its sources are the screened commit's but for a comment and the tests.

## SPRT

Pending.
