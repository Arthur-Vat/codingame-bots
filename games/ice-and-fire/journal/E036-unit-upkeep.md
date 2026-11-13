---
id: E036
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v016
hypothesis: v016's evaluation counts income three times and income is net of upkeep, so a level-3 training scores about -50 (-30 gold, +30 unit, -57 income, +4 cell) and a level-2 one about -20 before what they take, while the top 10 of CodinGame train more of both (Pduhard- 2.6 level-3 units a game against our 1.0; level-2 units twice as often as us) and our search scores their level-3 turns 109 below its own choice and their level-2 turns 29; charging the upkeep of strong units over fewer turns makes the search use them and beats v016.
change: The plan bot's evaluation gives back, for each level-2 and level-3 unit on an active cell, the upkeep of the turns beyond a number per level (3, as v016, to 0), with E020's level-2 discount kept or lowered.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the second of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
sections 3 and 7, and `2026-11-11-v014-judge/`), run beside E035 and E037.

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
replaces in `eval` with the same value at v016's settings.

## Screening

Pending.
