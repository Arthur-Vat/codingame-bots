---
id: E036
date: 2026-11-13
pull_request: "#296"
parent: ice-and-fire-v017
hypothesis: v016's evaluation counts income three times and income is net of upkeep, so a level-3 training scores about -50 (-30 gold, +30 unit, -57 income, +4 cell) and a level-2 one about -20 before what they take, while the top 10 of CodinGame train more of both (Pduhard- 2.6 level-3 units a game against our 1.0; level-2 units twice as often as us) and our search scores their level-3 turns 109 below its own choice and their level-2 turns 29; charging the upkeep of strong units over fewer turns makes the search use them and beats v017 (E037).
change: The plan bot's evaluation gives back, for each level-3 unit on an active cell, two of the three turns of upkeep that its weight of income charges (40 a unit); level-2 units and E020's discount are unchanged. Screened from 3 to 0 turns for each level, with the discount kept or lowered.
release: ice-and-fire-v018
sprt: accepted
elo: +7.6 [+2.1, +13.1] at CodinGame's limits
pairs: 2800
full_time: +7.6 [+2.1, +13.1] (the SPRT's own, ADR 0042), 2 timeouts of v018 in 5,600 games (v017 none), slowest answer 504.5 ms (the first turn)
decision: promoted
cg_rank: Legend, 18th of 87 (18th of 925 overall), 2026-11-13: submitted through the API on the owner's approval (submission 41709589)
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the second of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
sections 3 and 7, and `2026-11-11-v014-judge/`), run beside E035 and E037.
E037 was promoted first (`ice-and-fire-v017`, #295); the owner then chose
to send E036 and E035 to the SPRT in turn ("SPRT both, in turn
(Recommended)"), E036 first, on top of v017.

## Why

The shared evaluation counts `3 × income`, and a unit's upkeep (1, 4 and
20 gold a turn for levels 1 to 3, RULES.md "Income") comes off the
income, so it charges three turns of upkeep. With E020's discount of 15
on a level-2 unit, a training changes the score, before anything it takes,
by about +4 at level 1 (on a free cell), -20 at level 2 and -50 at level 3.
A level-3 unit is played only where the same plan gains 50 more at once.
E020 went the other way for level-2 units, and won, in self-play against
v008; the top 10 train them twice as often as we do.

## The change

`bots/plan/src/eval.rs`: `UPKEEP_TURNS`, the turns of upkeep charged for
a level-2 and a level-3 unit; `strong_units` gives back the rest
(`(3 - turns) × upkeep` a unit) and holds E020's discount, which it
replaces in `eval` with the same value at v016's settings. The candidate
sets `UPKEEP_TURNS` to `[3, 1]`: a level-3 unit is charged one turn (20
of its upkeep given back twice, 40), a level-2 unit three as before. The
branch merges `main` after E037 and takes the screened commit `c969308`'s
code (E036's change on v017): the candidate's minified bundle is byte for
byte the bundle of `c969308` (41,402 characters). A new test checks the
term by level and for both seats; the test of debt now adds the level-3
unit's 40, given back whether or not its side can pay its upkeep, since
the debt term already takes its units.

## Screening

All on GitHub (`screen.yml`, one calibrated runner per variant and shard
at CodinGame's speed and real limit, ADR 0048; no timeout, crash or
invalid answer). "l2 t, l3 t" is the turns of upkeep charged for a level-2
and a level-3 unit (3 in v016); "d" is E020's level-2 discount (15 in
v016).

First round against `ice-and-fire-v016`, run 43449385655, seed 36036, 800
pairs each over two runners:

| Variant | Commit | Turns l2, l3 | Elo against v016 | Score |
| --- | --- | --- | --- | ---: |
| l3t2 | `ca5932a` | 3, 2 | +5.6 [−4.0, +15.3] | 50.8% |
| l3t1 | `23ef74b` | 3, 1 | **+14.3 [+3.8, +24.9]** | 52.1% |
| l3t0 | `255f744` | 3, 0 | −7.4 [−18.8, +4.0] | 48.9% |
| l2t1 | `3e0c57a` | 1, 3 | +6.1 [−7.4, +19.6] | 50.9% |
| l2t2l3t1 | `37af014` | 2, 1 | +4.3 [−8.3, +17.0] | 50.6% |
| l2t1l3t1 | `b5fe2c6` | 1, 1 | **+17.8 [+4.6, +31.0]** | 52.6% |
| d5l3t1 | `a722e46` | 3, 1, d 5 | +8.3 [−4.7, +21.3] | 51.2% |
| d5 | `0e3274b` | 3, 3, d 5 | +10.4 [−2.4, +23.3] | 51.5% |

The three best again against v016, run 43452999688, seed 36037, 1,600
pairs each over four runners:

| Variant | Commit | Elo against v016 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| l2t1l3t1 | `b5fe2c6` | +6.1 [−3.2, +15.4] | 50.9% | about +10 |
| l3t1 | `23ef74b` | +4.1 [−2.7, +10.9] | 50.6% | about +8 |
| d5 | `0e3274b` | −1.1 [−10.4, +8.3] | 49.8% | about +3 |

The two best on top of E037's candidate (`ice-and-fire-v017`), run
43458083084, seed 36038, 1,600 pairs each over four runners:

| Variant | Commit | Elo against v017 | Score |
| --- | --- | --- | ---: |
| l2t1l3t1 on v017 | `f9405bb` | +2.4 [−7.0, +11.8] | 50.3% |
| l3t1 on v017 | `c969308` | **+8.5 [+1.5, +15.5]** | 51.2% |

Charging a level-3 unit no upkeep at all (l3t0) lost; one turn was best,
and the level-2 change added nothing once v017 counted the moves of
level-2 units. The candidate is l3t1 on v017.

## SPRT

Accepted on the pull request (run 43464288756, at 9fbf20d), at CodinGame's
speed and real limit, in blocks of 10 pairs over 8 calibrated runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v018 vs ice-and-fire-v017: 5600 games, 2800 complete pairs
  ice-and-fire-v018: 2861 wins, 0 draws, 2739 losses (score 51.1%)
  Elo of ice-and-fire-v018 over ice-and-fire-v017: +7.6 [+2.1, +13.1] (95%, from pairs)
  pairs by ice-and-fire-v018's points (0, 1/2, 1, 3/2, 2): [227, 0, 2285, 0, 288]
  ice-and-fire-v018 faults: 2 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v017 faults: 0 timeouts, 0 crashes, 0 invalid answers
  slowest answer: ice-and-fire-v018 504.5 ms, ice-and-fire-v017 505.8 ms
Checked 35 rounds of 8 blocks (280 blocks counted, 24 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.25 (bounds -2.94, 2.94) after 2800 pairs
Verdict: ACCEPTED: ice-and-fire-v018 is stronger than ice-and-fire-v017
```

A small gain, close to its screening on v017 (+8.5), which needed 2,800
pairs to settle. E035, screened next on top of this candidate (three
quarters of the gold in the pressure term, commit `1a9c301`, run
43464481930, seed 35038, 1,600 pairs), scored −6.5 [−15.8, +2.8] against
it, after +14.3 and +11.9 against v016 and +8.5 against v017.
