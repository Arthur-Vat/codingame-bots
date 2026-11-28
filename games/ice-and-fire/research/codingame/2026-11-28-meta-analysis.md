# What the experiments say, E001 to E109 (2026-11-28)

The owner (2026-11-28), after E107 to E109 were dropped: record them and
"try to make a meta analysis". This note reads the journal's 92 entries (E001
to E109; the numbers without an entry were never run or were folded into
others), including E102 and E103 from their open pull requests and E107 to
E109 from #408, for what kinds of change have paid, when, and what that says
about the next ones. A release is counted once: E051 and E052 were promoted
through E055's candidate (`ice-and-fire-v023`), so they count as experiments
run but not as separate wins.

## Wins over time

| Experiments | Dates | Run | Promoted | Share | Elo of the promoted, in all | Median |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| E001 to E020 | 2026-10-19 to 10-29 | 20 | 9 | 45% | 969 | 74.1 |
| E021 to E040 | 10-29 to 11-13 | 17 | 9 | 53% | 193 | 15.0 |
| E041 to E060 | 11-13 to 11-14 | 20 | 8 | 40% | 251 | 30.2 |
| E061 to E080 | 11-14 to 11-20 | 16 | 5 | 31% | 202 | 41.9 |
| E081 to E100 | 11-21 to 11-27 | 10 | 3 | 30% | 134 | 49.8 |
| E101 to E109 | 11-27 to 11-28 | 9 | 0 | 0% | 0 | – |

Wins were nearly half of the experiments up to E060 (26 of 57), a third up
to E100, and none of the last nine; the wins that remain are as large as before (median 42 to
50), so what is left is rarer, not smaller.

## By kind of change

Each experiment in the kind of its main change (read from its `change`
line):

| Kind | Run | Promoted | Share | Elo | Up to E060 | From E061 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| The opening | 20 | 9 | 45% | 618 | 6 of 15 | 3 of 5 |
| Terms of the evaluation | 35 | 11 | 31% | 335 | 9 of 23 | 2 of 12 |
| Actions offered to the search | 9 | 5 | 56% | 170 | 3 of 4 | 2 of 5 |
| Speed, the same play | 3 | 3 | 100% | 103 | 2 of 2 | 1 of 1 |
| The opponent's model | 8 | 2 | 25% | 112 | 2 of 3 | 0 of 5 |
| Search control (climbs, restarts, final choice, time, pruning) | 12 | 2 | 17% | 89 | 2 of 8 | 0 of 4 |
| A strategy layer or a reserve | 3 | 0 | 0% | 0 | – | 0 of 3 |
| New bots | 2 | 2 | 100% | 323 | 2 of 2 | – |

The kinds, with their experiments (* promoted):

- The opening: E003* E004* E005 E008 E010* E011 E034* E046 E051 E052 E053
  E054 E055* E056* E057 E067* E072 E073* E095* E108 (E051 and E052 promoted
  through E055).
- Terms of the evaluation: E006 E012 E013 E017 E020* E021 E023* E024 E033*
  E035 E036* E037* E038 E039 E042* E043* E047 E048 E049 E050 E058* E059 E060*
  E061 E062 E066 E068* E075 E081* E090 E091 E092 E093 E094 E096.
- Actions offered: E026* E041 E044* E045* E070* E076* E097 E102 E109.
- Speed: E015* E028* E100*.
- The opponent's model: E019* E027* E040 E063 E064 E103 E104 E105.
- Search control: E007 E009 E014* E016 E018 E029* E030 E031 E065 E101 E106
  E107.
- A strategy layer or a reserve: E069 E071 E077.
- New bots: E001* E002*.

## What the pattern says

1. **The opening and speed still pay.** The opening has won 3 of its last 5
   (E067, E073, E095: +33, +44, +50); speed won each time it was tried (E100:
   +33 for 45% fewer instructions, the same answers).
2. **Changing what the search looks at no longer pays.** Since E061 the search
   control (E065, E101, E106, E107), the opponent's model (E063, E064, E103,
   E104, E105) and the strategy layers (E069, E071, E077, the last parked
   after a +14.8 screening rather than lost) won none of 12. The opponent's
   model is the clearest: a lazier one loses about 70 (E103), a stronger one
   gains nothing (E104), a frugal or realistic one loses a little (E105). Our
   own climb lost when its options were cut to another player's habits
   (E106's kinds, E109's purchases: up to −176) and gained nothing from a new
   order or new starts (E106, E107, E109); cutting options that are bad by
   our own measure was level (E101's far trainings, −0.6 in its SPRT) or won
   earlier (E026: towers only where they guard, +24.9; E045: no trainings on
   our own cells, +30.5).
3. **Copying zasmu works better as a value than as a rule.** The wins drawn from the
   top players' play were terms of the evaluation that make the search want
   what they do (mines E033 and E058, units that walk to free land E060, the
   tower wall E068, the HQ pressure E042 and E043) or an opening that heads
   where they head (E067, E095). Imposing their behaviour as a rule lost or
   gained nothing in the last experiments: their kinds of unit action (E106),
   their purchases (E109), their opening's trainings (E108); E045, a rule
   copied from the top 10 (no trainings on our own cells), won, but it also
   removed options our own evaluation rated badly. By kind, our units already act like
   zasmu's (its three likeliest kinds hold 87.7% of our actions, E106): the
   differences are in quantities and timing (mines at game turns 9 to 14,
   idle units, early towers that fall,
   [towers](2026-11-28-towers.md) and the phases in
   [2026-11-28-type-planner.md](2026-11-28-type-planner.md)).
4. **Evaluation terms win less often** (9 of 23, then 2 of 12): the easy terms
   are taken, and new ones mostly duplicate what the search already sees.
   The two recent wins narrowed a term rather than adding one (E081: joints
   counted only near the enemy, +51) or counted early what pays later (E068).
5. **A screening's error is about ±10.** The controls (no change, 800 to
   2,000 pairs each) of the last eight screenings came out at +11.3 (E101),
   −7.8 (E102), −6.1, −1.0, −1.7, −0.2, +1.1 and −1.7: most within ±2, two of
   eight at ±8 to ±11 with intervals that exclude 0. A variant within about
   ±10 of its control is level; only a larger difference, or an SPRT, says
   more.

## What follows for the next experiments

- Prefer the opening, the evaluation's values and speed; aim them at the
  phases where we lose (game turns 5 to 14), measured by phase, not over
  whole games.
- Express a top player's habit as something the search values (a term, a
  bonus for a phase), never as a filter on what it may play.
- Leave the opponent's model and the search's control alone unless a
  measured failure points at them.
- Screen with a control and 1,600 pairs; read a result within about ±10 of
  the control as level.

E110 (no towers near our own HQ after game turn 14) and E111 (mines valued
more at game turns 9 to 14, and a small value for mine spots held) follow
the second point and were screening as this note was written.
