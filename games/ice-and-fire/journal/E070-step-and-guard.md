---
id: E070
date: 2026-11-19
pull_request: "#359"
parent: ice-and-fire-v028
hypothesis: At turns 8 to 10 against the top 6, v028 takes about 1.2 fewer neutral cells a ply than they do, partly because its units stay beside neutral cells they could take (0.40 a ply as red, against 0.18); its own search scores the step lower in 103 of 126 such positions, mostly because the cell left behind opens to the enemy's level-1 trainings, and its hill climb never tries the step together with a guard on that cell; offering that pair as one action lets the units advance and beats v028.
change: The plan search is offered a macro action for each unit on a cell of ours the enemy can reach at its next turn and each neutral neighbour it may enter (STEP_GUARD): the step, then a tower on the cell it left (1, released) or a level-1 training there (2, screened weaker).
release: ice-and-fire-v029
sprt: accepted
elo: +71.9 [+46.6, +98.0]
pairs: 240
full_time: +71.9 [+46.6, +98.0] (the SPRT's own, at CodinGame's limits)
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-19 (forms): the land gap first ("Land:
opening/early middle"), a diagnostic before any change ("Diagnose first",
then "Find why units stay"), and the variants "Step + tower" and "Step +
level-1 refill".

## The land gap

`codingame-units` on `ice-and-fire-v028`'s 158 ranking games (data branch
e3b20fa), its side against the top 6 of the newest snapshot and theirs
against it, in the same games, by seat:

| Turns 8 to 10, a ply | v028 red | top 6 red | v028 blue | top 6 blue |
| --- | ---: | ---: | ---: | ---: |
| Units that may move | 5.0 | 5.7 | 4.9 | 5.7 |
| Neutral cells taken by units | 2.9 | 4.1 | 2.7 | 3.8 |
| Units that stayed beside a neutral cell they could take | 0.40 | 0.18 | 0.27 | 0.28 |
| Units blocked at the frontier | 0.62 | 0.57 | 0.96 | 0.54 |
| Trainings | 0.52 | 0.61 | 0.81 | 0.78 |

Up to game turn 7 both sides play alike. Of the units that stayed beside a
neutral cell, 80% had an enemy unit within 2 steps. A race term in the
evaluation was tried and dropped in [E061](E061-middle-race.md).

## Why the units stay

A scratch command (not committed) wrote a bank of the 142 plies of v028 at
game turns 8 to 13 against the top 6 where one of its units stayed beside a
neutral cell, each twice: with v028's answer, and with that answer plus the
unit's step (to the neutral neighbour nearest the enemy HQ). v028's own
search (its tag, `judge` of the analysis build, 600 tries, patched in the
scratch copy to print the worst reply) scored both:

- in 103 of the 126 positions whose scores are comparable, the step scores
  lower (median −29, mean −36); in 21 it is level or better; in 11 more it
  exposes our HQ, and in 5 the game is lost either way;
- the worst reply the search expects after the step is the enemy's
  level-1 trainings in 67 of the 103, a level-2 or level-3 kill in 19, moves
  alone in 17.

A unit stands on a cell that an enemy level-1 unit cannot enter; stepping
forward leaves it empty. Zasmu holds its front with towers, which frees its
units. The hill climb adds one action at a time, so the step and a guard on
the cell left behind are never tried as a pair.

## Screening

`screen.yml` run 44516824781, 800 pairs per variant against
`ice-and-fire-v028`, 2 shards each, at CodinGame's speed and real limit:

| Variant | Ref | Pairs | Elo over v028 [95%] | Score | Faults |
| --- | --- | ---: | --- | ---: | --- |
| tower (`STEP_GUARD` 1) | `c56c551` | 800 | +38.4 [+24.8, +52.1] | 55.5% | none |
| refill (`STEP_GUARD` 2) | `14f3ea2` (branch `claude/ice-and-fire-e070-refill`) | 800 | −6.5 [−16.3, +3.3] | 49.1% | none |

A freed unit pays only when a tower holds the cell it leaves; a level-1
unit put there is one more unit standing at the front. The owner chose to
freeze the tower variant (form, 2026-11-19): `ice-and-fire-v029`.

## Review

Bot tier, an independent reviewer: approve with nits. It checked the
macro's legality against the engine's `build`, that a failed second
command never leaves a half-applied plan (the search applies an action on a
clone), and the speed: 28.0 tries a millisecond against 28.5 on main
(`speed` example, `speed-bank.txt`, 600 tries), about 1.7% slower per try.
Kept for the next change to the bot, since the release is frozen: the
step-and-guard block does not check `allows_towers()` (harmless in Bronze
and above, wasted tries below) and computes the enemy's reach a second
time; the test does not check that the moving unit stood on the guarded
cell; one doc line of `actions.rs` is over 100 columns.

## SPRT

SPRT run 44520605622 at 32adfd5, in blocks of 10 pairs over 8 calibrated
runners (ADR 0049):

```
Smoke test: ice-and-fire-v029 vs random: 200 wins of 200, 0 faults
ice-and-fire-v029 vs ice-and-fire-v028: 480 games, 240 complete pairs
  ice-and-fire-v029: 289 wins, 0 draws, 191 losses (score 60.2%)
  Elo of ice-and-fire-v029 over ice-and-fire-v028: +71.9 [+46.6, +98.0] (95%, from pairs)
  pairs by ice-and-fire-v029's points (0, 1/2, 1, 3/2, 2): [18, 0, 155, 0, 67]
  ice-and-fire-v029 faults: 1 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v028 faults: 2 timeouts, 0 crashes, 0 invalid answers
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 4.19 (bounds -2.94, 2.94) after 240 pairs
Verdict: ACCEPTED: ice-and-fire-v029 is stronger than ice-and-fire-v028
```

