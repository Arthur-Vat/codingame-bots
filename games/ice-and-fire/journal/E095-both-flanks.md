---
id: E095
date: 2026-11-27
pull_request: "#395"
parent: ice-and-fire-v032
hypothesis: Holding both flanks early, as the top players do and v032 rarely does, makes our land wider and claims the flank we now leave to the enemy, and beats ice-and-fire-v032 (candidate ice-and-fire-v033).
change: The opening's plans are ranked by the most active cells, then by E073's `Frontier { cap: 8 }` (the least total distance from our land to each cell of the frontier, each distance capped at 8) rather than `Central(Linear(7))`; the search's weaker-flank term (`eval::FLANK`, `FLANK_CAP`, `FLANK_UNTIL`) is added at 0, as it lost the screening.
release: ice-and-fire-v033
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner's notes on ten short CodinGame losses (2026-11-27,
https://claude.ai/artifact/F9jwgANcMvoB6m5HfMT7xQ) said in seven of them
that we leave large parts of the map, the left or the bottom left, to the
opponent. The growth measure (`review/examples/growth.rs`, 3,000 CodinGame
games: v032's 2,350, and the ranking games of v026 to v032; each side
compared with the opponents in the same seat) confirmed it in this form:

- We hold as much land as the top players to game turn 10 (26.7 cells
  against 26.3 at game turn 8, seat 0) and reach the line even between
  the HQs a turn earlier, but on a narrower front (7.3 columns across the
  diagonal against 8.2 at game turn 8).
- Our Voronoi share is 0.50 against 0.54, and the gap is all on the
  flanks: the middle 0.51 against 0.51, our weaker flank 0.19 against
  0.30, the stronger 0.78 against 0.84.
- Our flank land is as large (14 to 15% of our cells at game turn 8
  against 15 to 19%), but on one flank: both flanks held by game turn 8 in
  12 to 14% of our games against 33 to 41% of the top players' (zasmu
  38%), by game turn 10 in 41 to 45% against 61 to 68%.
- Flank cells are taken by units walking there (trainings take about 0.1
  a turn for everyone). zasmu steps sideways from game turn 2, has the
  widest front at game turn 6 (7.3 columns against 6.6 for the other top
  players and 5.4 for us, seat 1) and fewer units at game turn 8.
- Holding both flanks by game turn 10 goes with winning for every group
  (us 68% against 63%, the other top players 32% against 22%, zasmu 94%
  against 86%; not controlled for the map).

E073 saw the same of v029's opening ("mostly reaches one flank") and duelled
orders closer to the whole frontier (`Frontier { cap }`) in games cut at
game turn 12, where every land-first order scored alike; it was never
played in whole games, which judge (E077).

## Change

`eval::flank`: the flanks are the cells `FLANK_OFF` (4) or more columns
off the diagonal between the HQs, toward (11, 0) and toward (0, 11); a
side's weaker flank is the one where it holds fewer active cells, and those
cells, at most `FLANK_CAP`, count `FLANK` each for it, net of the enemy's,
until game turn `FLANK_UNTIL`. The opening plays to game turn 6, so the
term steers the search from game turn 7, when our gap opens.

The second lever is the opening's order, `main.rs`'s `ORDER`:
`Frontier { cap: 8 }` (E073: the most land, then the least total distance
from our land to the frontier's cells), in place of `Central(Linear(7))`.

## Before the screening

A local look at the mechanism (`w4`: the term at 4, cap 4, until game turn
12; 104 games against v032, seed 95, at 3 times CodinGame's time, measured
by `review/examples/growth.rs`): both flanks held by game turn 8 in 16% of
w4's games against 12% of v032's, by game turn 10 in 61% against 56%; the
weaker flank's Voronoi share at game turn 8 0.27 against 0.26. At 4 the
term hardly moves the units, so the screening also tries 8 and 16.

## Screening

800 seat-swapped pairs per variant against `ice-and-fire-v032`, whole
games at CodinGame's limits, 4 shards
([run 45980440646](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45980440646)):

| Variant | Commit | `FLANK` | Opening order | Elo over v032 [95%] | Score |
| --- | --- | ---: | --- | ---: | ---: |
| `off` | `a54381f` | 0 | `Central(Linear(7))` | −7.8 [−17.7, +2.0] | 48.9% |
| `w4` | `51b6683` | 4 | `Central(Linear(7))` | −5.6 [−18.3, +7.0] | 49.2% |
| `w8` | `34438ae` | 8 | `Central(Linear(7))` | −15.6 [−29.5, −1.8] | 47.8% |
| `w16` | `076aea7` | 16 | `Central(Linear(7))` | −37.1 [−51.1, −23.1] | 44.7% |
| `fr8` | `a30dd55` | 0 | `Frontier { cap: 8 }` | +37.5 [+22.3, +52.8] | 55.4% |

`FLANK_CAP` 4 and `FLANK_UNTIL` 12 in every variant. No fault on either
side in any variant; the runners' time scales were 0.42 to 1.02.

The flank term loses, the more the heavier (w16 clearly); the opening
ordered by `Frontier { cap: 8 }` wins clearly, and the control is level.

## Second screening: the frontier order's settings

The owner (2026-11-27): screen the frontier order's settings before an
SPRT. 1,000 seat-swapped pairs per variant against `ice-and-fire-v032`,
whole games at CodinGame's limits, 5 shards, seed 9501 (fr8 played again
on new games)
([run 45985603993](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45985603993)):

| Variant | Commit | Opening order | Turns planned | Elo over v032 [95%] | Score |
| --- | --- | --- | ---: | ---: | ---: |
| `fr4` | `d3b7046` | `Frontier { cap: 4 }` | 9 | +33.1 [+18.8, +47.5] | 54.8% |
| `fr8` | `a30dd55` | `Frontier { cap: 8 }` | 9 | +32.1 [+17.9, +46.3] | 54.6% |
| `fr12` | `6520b1b` | `Frontier { cap: 12 }` | 9 | +38.4 [+25.0, +51.9] | 55.5% |
| `fr8-t12` | `1cdb45e` | `Frontier { cap: 8 }` | 12 | +42.6 [+28.2, +57.2] | 56.1% |

`FLANK` 0 in every variant. One timeout in 2,000 games for fr8, one for
v032 against fr4; no other fault.

Every setting wins by about as much: the intervals overlap almost wholly,
and no cap or plan length is told apart by 1,000 pairs. fr8 holds on new
games (+32.1 after +37.5; about +34 over its 1,800 pairs). E073 found
that longer plans add nothing measurable to the land-first orders; here
fr8-t12 scores highest but within the noise of the others.

## Third screening: fr8 against fr8-t12

The owner (2026-11-27): fr8-t12 looks better; play the two longer before
choosing. 2,000 seat-swapped pairs per variant against
`ice-and-fire-v032`, whole games at CodinGame's limits, 8 shards (the
workflow's most), seed 9502
([run 45988059330](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45988059330)):

| Variant | Commit | Turns planned | Elo over v032 [95%] | Score |
| --- | --- | ---: | ---: | ---: |
| `fr8` | `a30dd55` | 9 | +32.8 [+23.0, +42.6] | 54.7% |
| `fr8-t12` | `1cdb45e` | 12 | +24.7 [+14.5, +35.0] | 53.5% |

No fault on either side. fr8-t12's lead in the second screening does not
hold: over all its screenings (weighted by pairs) fr8 scores about +33.6
on 3,800 pairs (+37.5, +32.1, +32.8) and fr8-t12 about +30.7 on 3,000
(+42.6, +24.7), the same within the noise, as E073 found for longer plans.
fr8 is the simpler change.

## The candidate

The owner chose fr8 (2026-11-27): `ice-and-fire-v033` is v032 with the
opening ordered by `Frontier { cap: 8 }` over 9 turns, the flank term
kept at 0.

## SPRT

Pending.

## The variants on the owner's ten games

The owner asked (2026-11-27) whether the variants behave better before the
territories meet on the ten short losses he commented. A tool
(`review/examples/replay_vs.rs` on the session branch
`claude/pensive-darwin-879zmo`) plays a bot in our seat on each game's map,
untimed with 800 tries a turn, against the opponent's recorded answers,
which hold until contact (game turn 7 or 8 in these games; the opponent's
answers start being refused after it). `w8` and `w16` play exactly as `off`
until contact: the opening is the same and the term starts at game turn 7.
`fr8` differs from game turn 2 to 6 and, in the five red games, holds both
flanks before contact where `off` holds one: at game turn 8, cells on each
flank 3 / 3 against 6 / 0, 3 / 3 against 8 / 0, 1 / 2 against 0 / 4, 1 / 2
against 3 / 0, and its land is wider (11 columns against 7 in the first
two). In the blue games it changes less (one game switches flank, one
holds both). The boards are on the owner's page
(https://claude.ai/artifact/F9jwgANcMvoB6m5HfMT7xQ, "E095 variants
compared").
