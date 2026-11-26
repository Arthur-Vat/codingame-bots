---
id: E094
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: The tower changes of E092 and E093 that did not lose, together (guard towers near the enemy, the step and guard from more units early, the HQ guard on the enemy's next turn, an early hold), beat ice-and-fire-v032 where none did alone.
change: Combinations of E092's `GUARD_NEAR` and `HQ_GUARD_TURNS` and E093's `STEP_GUARD_NEAR`, `EARLY_THREAT_QUARTERS` and `EARLY_TOWER_VALUE`, on E093's branch; each at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner (2026-11-26): mix the best tower ideas. Alone, none beat v032;
those that did not lose were E092's near4 (+3.2 [-4.9, +11.2] at 1,200
pairs), hqnow (+3.5 [-9.3, +16.2] at 400), and E093's fwd2 (+7.4
[-6.6, +21.4] at 800) and fwd2-hold8-tv15 (+5.2 [-10.3, +20.7] at 800).

## The change

No new code: the switches of [E092](E092-tower-value.md) and
[E093](E093-early-hold.md), on E093's branch (whose variant commits are
named E094).

## Screening

Each variant against `ice-and-fire-v032`, 800 pairs over two shards, full
games (run 45903603606):

| Variant | Settings | Elo | Timeouts |
|---|---|---|---|
| near4-fwd2 | `GUARD_NEAR` 4, `STEP_GUARD_NEAR` 2 | +10.9 [-3.1, +24.9] | 0 |
| near3-fwd2 | `GUARD_NEAR` 3, `STEP_GUARD_NEAR` 2 | +2.6 [-11.1, +16.3] | 1 |
| near4-fwd3 | `GUARD_NEAR` 4, `STEP_GUARD_NEAR` 3 | -0.4 [-14.3, +13.4] | 0 |
| near4-hqnow | `GUARD_NEAR` 4, `HQ_GUARD_TURNS` 1 | -1.3 [-12.4, +9.8] | 0 |
| near4-fwd2-hqnow | the three | +6.1 [-8.4, +20.6] | 0 |
| fwd2-hold8 | `STEP_GUARD_NEAR` 2, `EARLY_THREAT_QUARTERS` 8 | -11.3 [-26.4, +3.7] | 0 |
| near4-fwd2-hold8 | the three | -12.2 [-27.1, +2.7] | 0 |
| near4-fwd2-hold8-tv15 | the four, `EARLY_TOWER_VALUE` 15 | -6.5 [-21.7, +8.7] | 0 |

No mix beats v032; the early hold costs every mix it joins. near4-fwd2
leans positive, its parts (+3.2 and +7.4 alone) adding up, without an
interval above 0.
