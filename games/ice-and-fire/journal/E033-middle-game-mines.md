---
id: E033
date: 2026-11-12
pull_request: pending
parent: ice-and-fire-v014
hypothesis: The top 10 of CodinGame build a mine in about a fifth of their games at every turn from turn 9, once their territory is large, and v014 almost never does, because its evaluation scores a mine below the gold it costs; counting a mine above its income, where the enemy cannot take it at once and the more so the larger its side's territory, makes the search build mines in the middle game and beats v014 (candidate `ice-and-fire-v015`).
change: The plan bot's evaluation adds, for each of a side's mines on an active cell that the enemy cannot take at its next turn, a quarter of that side's active cells.
release: ice-and-fire-v015
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Chosen by the owner on 2026-11-12 ("Mines experiment (Recommended)"), the
first of the experiments proposed by the report on 2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
"What to try in our bot").

## Why

From turn 9 to turn 15 the top 10 build a mine in about 20% of their games
at every turn, the rest of Legend in 6% to 9%, and v014 in 0% to 2% against
the top 10; under 1% of our spending goes to the economy in the middle
game, against 14% of theirs. Our search scores their turns with a mine 35
below its own choice, against 22 for their plain turns. The report warns
that the top 10 are already ahead when their mines show, so the mines may
follow their lead as much as make it: only an experiment says.

v014's search rarely gains by a mine. The shared evaluation
(`ice_and_fire_tactics::eval`) counts gold at 1 and income at 3, so a mine
is worth 12 for the 15 gold it costs, and the pressure term counts the
gold spent against it too. A climb adds only actions that raise the plan's
score. In the games below v014 builds 2.6 to 3.0 mines a game, nearly all
in the opening, by its rules (a mine in 36% of games at its turn 6, 74% at
turn 7, 28% at turn 8), then in 1% to 3% of games a turn from its turn 10
to 14 and 5% to 10% later.

## The change

`bots/plan/src/eval.rs`: each mine of a side on one of its active cells
that the enemy cannot take at its next turn counts, beyond its income, one
for every `MINE_PER_CELLS` (4) active cells of that side (`mine_bonus`):
5 with 21 cells, 9 with 36, 12 with 48. It counts in both sides' scores,
so the search also sees more in taking the enemy's mines. The cells the
enemy can take at its next turn are the threatened cells of E023 (those
its chains of trainings reach for its gold plus its income), which the
evaluation already finds, so the term costs two counts of bits. A mine
pays its 15 gold back in four turns, and one the enemy takes is destroyed.

A test checks the bonus by territory and the term on the corridor: a mine
out of the enemy's reach adds the bonus to its side and takes as much from
the other, a mine on a cell the enemy can take next turn adds nothing, and
neither does a mine of a territory of 3 cells.

The screening went through other forms of the term: a fixed bonus for
every mine ("flat"), for the mines out of the enemy's reach for one, two
or three of its incomes ("safe 1" to "safe 3"), and the second round's
forms that tie the bonus to the territory. Those screened on GitHub are
commits of this pull request, named in the tables below. The candidate's
code keeps only the form chosen; it plays the same games as the screened commit `efbbb86` at 500
fixed tries (12 games against v014, 558 turns, compared as
`scripts/check-minified.sh` compares records).

## Screening

All against `ice-and-fire-v014` at CodinGame's speed and real limit,
calibrated on each machine (ADR 0048).

**A local look** (150 seat-swapped pairs each, four matches at a time on
a 4-core machine, time scale 0.575, tolerance 7 ms; seed 33033, and 33035
for the last four; variants bundled from the working tree; read with
`ice-and-fire-review screening`). "Flat"
counts every mine on an active cell; "safe 1" leaves out the mines on
cells the enemy can take at its next turn, "safe 2" and "safe 3" those its
chains reach for its gold plus two or three incomes. Mines are those
built a game, v014's in the same games in parentheses.

| Variant | Elo against v014 | Score as red | Score as blue | Mines | Timeouts |
| --- | --- | ---: | ---: | ---: | ---: |
| flat, 6 | −32.5 [−62.8, −2.8] | 32.0% | 58.7% | 3.31 (2.92) | 6 (3) |
| flat, 10 | −23.2 [−58.1, +11.2] | 30.7% | 62.7% | 4.21 (2.94) | 2 (4) |
| flat, 16 | −23.2 [−58.1, +11.2] | 32.0% | 61.3% | 4.46 (3.03) | 4 (4) |
| safe 1, 10 | +27.9 [−0.6, +56.7] | 38.0% | 70.0% | 3.54 (2.78) | 5 (2) |
| safe 1, 40 | −41.9 [−74.9, −9.6] | 24.0% | 64.0% | 3.74 (3.03) | 2 (2) |
| safe 2, 24 | −18.5 [−50.2, +12.8] | 34.0% | 60.7% | 3.51 (2.75) | 0 (2) |
| safe 3, 10 | +11.6 [−22.7, +46.1] | 35.3% | 68.0% | 3.37 (2.65) | 1 (0) |
| safe 3, 16 | +6.9 [−24.2, +38.2] | 38.7% | 63.3% | 3.60 (2.80) | 0 (0) |

The three flat variants together won 416 of their 900 games (46.2%). The
flat bonus of 10 and 16 scored the same by chance: 206 of their 300 games
had the same winner.

**On GitHub** (`screen.yml`, 800 pairs each over two calibrated runners,
time scales 0.43 to 0.99; no timeout, crash or invalid answer on either
side in any match).

First round, run 43256901773, seed 33034:

| Variant | Commit | Elo against v014 | Score |
| --- | --- | --- | ---: |
| safe 1, 6 | `215a5d1` | +3.0 [−9.3, +15.4] | 50.4% |
| safe 1, 10 | `4fd8509` | −0.4 [−13.7, +12.8] | 49.9% |
| safe 1, 16 | `7ffca2a` | +2.6 [−10.6, +15.9] | 50.4% |
| safe 1, 24 | `b15cb49` | **−19.1 [−32.5, −5.8]** | 47.2% |
| safe 2, 10 | `38894a6` | −6.5 [−20.2, +7.1] | 49.1% |
| safe 2, 16 | `ecee9cb` | −10.0 [−23.9, +3.9] | 48.6% |

Second round, run 43259971659, seed 33036: the report's "when the
territory is large", three ways, each with the "safe 1" rule. "From 36
cells" counts a side's mines only when it has at least 36 active cells (a
quarter of the board; the top 10's median at turn 10 is 38); "ahead" only
for the side with more active cells than the other; "a quarter of the
cells" counts each mine its side's active cells divided by 4 (9 at 36
cells) in place of a fixed bonus.

| Variant | Commit | Elo against v014 | Score |
| --- | --- | --- | ---: |
| from 36 cells, 10 | `f5f833c` | **+13.9 [+2.0, +25.9]** | 52.0% |
| from 36 cells, 16 | `c29b718` | −4.3 [−17.3, +8.6] | 49.4% |
| from 44 cells, 16 | `afe2a7d` | −0.4 [−10.7, +9.8] | 49.9% |
| ahead, 10 | `74613e1` | +7.8 [−5.9, +21.6] | 51.1% |
| ahead, 16 | `6b43093` | +8.7 [−5.6, +23.0] | 51.2% |
| a quarter of the cells | `efbbb86` | +11.3 [−2.3, +24.9] | 51.6% |

Confirmation of the second round's four best, run 43261955814, seed
33037, 1,600 pairs each over four runners (one timeout, of "ahead, 16"):

| Variant | Commit | Elo against v014 | Score | With the second round, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| from 36 cells, 10 | `f5f833c` | +4.1 [−4.6, +12.8] | 50.6% | +7.4 |
| a quarter of the cells | `efbbb86` | +5.2 [−3.8, +14.2] | 50.7% | +7.2 |
| ahead, 10 | `74613e1` | +6.7 [−2.5, +16.0] | 51.0% | +7.1 |
| ahead, 16 | `6b43093` | −0.0 [−9.4, +9.4] | 50.0% | +2.9 |

Their neighbours, run 43262029341, seed 33038, 800 pairs each:

| Variant | Commit | Elo against v014 | Score |
| --- | --- | --- | ---: |
| a third of the cells | `cf07e29` | **+13.9 [+0.6, +27.3]** | 52.0% |
| a sixth of the cells | `d635543` | +9.1 [−4.0, +22.3] | 51.3% |
| from 30 cells, 10 | `a5056d9` | +11.3 [−2.1, +24.8] | 51.6% |
| from 36 cells, 6 | `e3b8c0b` | +4.8 [−7.8, +17.3] | 50.7% |

The three "share of the cells" forms again, run 43264990487, seed 33039,
1,600 pairs each:

| Variant | Commit | Elo against v014 | Score | All its pairs on GitHub |
| --- | --- | --- | ---: | --- |
| a third of the cells | `cf07e29` | **+13.0 [+3.4, +22.7]** | 51.9% | +13.3 over 2,400 |
| a quarter of the cells | `efbbb86` | **+12.2 [+2.8, +21.6]** | 51.7% | +9.2 over 4,000 |
| a sixth of the cells | `d635543` | **+13.9 [+4.8, +23.1]** | 52.0% | +12.3 over 2,400 |

Together the three scored +11.2 over 8,800 pairs (the pairs' mean of the
runs' estimates, about ±4). The divisor hardly matters between 3 and 6.

In two local matches of 150 pairs with game records (seed 33040), "a
third" built 0.98 mines a game at its turns 9 to 15 and "a quarter" 0.77,
against v014's 0.16 and 0.20: a mine in 7% to 14% of games at each of
those turns, against v014's 1% to 7%.

## The candidate

`ice-and-fire-v015`: a quarter of the cells, the middle of the three and
the one with the most games (4,000 pairs, +9.2), rather than the best of
three estimates that do not differ. Frozen by `scripts/new-release.sh`
(40,952 characters; the same answers as its readable copy in 4 games).

## What was learned

- **A mine is worth more only to a side with a large territory.** A fixed
  bonus for every safe mine, at 6, 10 or 16, was level with v014 (+3.0,
  −0.4, +2.6 over 800 pairs each), and weaker at 24 and 40; the same
  bonus tied to the territory (from 30 or 36 cells, for the side ahead, or
  as a share of the cells) scored +4 to +14 in 14 matches of 17 (the
  others: 16 from 36 or 44 cells, and "ahead, 16" once), and the share of
  the cells +11 over 8,800 pairs. This is the report's "when the
  territory is large".
- **A mine the enemy can reach should not count.** The flat bonus was
  weaker (46.2% of 900 local games) and built 3.3 to 4.5 mines a game;
  all that "safe 1" changes is to leave out the mines on cells the enemy
  can take at its next turn. Looking two incomes ahead instead of one did
  not help (−6.5 and −10.0 on GitHub).
- **Mines go with a lead, in our games too.** With a fixed bonus of 10
  for safe mines, the bot built a mine at its turns 9 to 15 in 155 of 300
  games and won 75% of those, against 32% of the others, and was no
  stronger than v014, whose few such games it also won at 72%. The
  report's caution holds: most of what the top 10's mines show is their
  lead. What mines built on a large territory add by themselves is about
  +10 Elo in the screening.
- **A local look of 150 pairs misleads.** "Safe 1, 10" scored +27.9
  locally and −0.4 over 800 pairs on GitHub. A screening run of 800 pairs
  for six variants takes 11 minutes there.
