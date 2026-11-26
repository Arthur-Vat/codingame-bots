---
id: E085
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: An evaluation that counts first the territory each side holds securely (its active cells and the free cells nearer to it than to the enemy, that the enemy cannot take at its next turn), the contested band more, without E023's charge on each threatened cell, makes the search hold an advanced front the enemy cannot take back rather than land it loses, and beats ice-and-fire-v032.
change: A secure-territory term in the search's evaluation, a band bonus on it, and E023's threat term optionally removed; each weight at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E084's checks (its journal entry, "Are the cuts unexpected?"): v032's
losses before game turn 12 are foreseen by its search, more search time
does not change what it plays, and its evaluation disagrees with zasmu's
choices (the leader, who beats v032 in 178 of 195 games). The owner
(2026-11-26): value much more the territory share, how it is protected
included, then the economy; a good territory is an advanced front line the
enemy cannot take back; the cells each side can lose at the enemy's next
turn (5/4 each) may not belong in the evaluation, the danger to the HQ
does; and consider the areas of interest.

## The change

In `bots/plan/src/eval.rs`, for each side, less the enemy's:

- `SECURE`: each cell of the side's secure territory ([`secure`]): its
  active cells and the free cells nearer its active land than the enemy's
  (in steps over valid cells), less those the enemy can take at its next
  turn (E037's `reach`, chains of trainings for its gold and income after
  its strong units step).
- `SECURE_BAND`: each secure cell in the contested band (E073's band, the
  areas of interest), more.
- `THREAT_QUARTERS` (E023, E037; 5 in v032) at 0 in some variants. The
  danger to the HQ stays (the shared evaluation's threat and pressure,
  E042's standing threat).

## Screening

`screen.yml` run 45848004354 on the owner's approval (2026-11-26), full
games against v032, 400 pairs per variant over 2 shards, no fault on either
side:

| Variant | Commit | Secure | Band | Threat (quarters) | Elo over v032 [95%] |
| --- | --- | --- | --- | --- | --- |
| sec4 | aeddf17148eb | 4 | 0 | 5 | -100.0 [-124.3, -76.5] |
| sec8 | de59f295625b | 8 | 0 | 5 | -176.1 [-202.4, -151.5] |
| sec16 | a0180d00bc11 | 16 | 0 | 5 | -231.5 [-261.6, -204.1] |
| sec8-nothreat | 11d4279acd2f | 8 | 0 | 0 | -156.6 [-183.7, -131.2] |
| sec8-band-nothreat | 8a4f9a74db5f | 8 | 4 | 0 | -197.9 [-226.7, -171.3] |
| sec16-band-nothreat | 0d3639e7bf83 | 16 | 8 | 0 | -235.4 [-264.6, -208.9] |
| nothreat | 9b1528e3bc30 | 0 | 0 | 0 | -59.6 [-82.1, -37.6] |

Every variant is clearly weaker, the more the heavier the secure term,
and removing E023's charge on threatened cells alone costs about 60 Elo:
that charge is worth keeping. A likely cause, as for E083's race: a free
cell nearer to a side counts as its territory as soon as the enemy cannot
reach it, so taking it adds nothing, and the search stops taking land.

### Round 2: held land only

The owner (2026-11-26): shouldn't there be an incentive to grow the
territory anyway? It is there (a taken cell counts 4 and its income 3 in
the shared evaluation), but a cell taken toward the enemy often comes
within the enemy's reach, with its neighbours, and leaves the secure count:
with the secure weight at 8 or more, expanding lowers the score. Round 2
rewards held land only, so that taking a cell and making it safe pays more
than leaving it exposed (E023's charge kept):

- `SECURE_HELD`: each active cell of a side the enemy cannot take at its
  next turn;
- `SECURE_FRONT`: each such cell next to a cell not the side's (its secure
  front line);
- `SECURE_BAND`: each such cell in the contested band, more (on held cells
  now);
- `SECURE_FREE`: each free cell nearer the side's land than the enemy's
  that the enemy cannot take, at a weight below a taken cell's 7.

`screen.yml` run 45851250571 on the owner's approval (2026-11-26), full
games against v032, 400 pairs per variant over 2 shards, no fault on either
side:

| Variant | Commit | Held | Front | Band | Free | Elo over v032 [95%] |
| --- | --- | --- | --- | --- | --- | --- |
| held2 | 906e51729e40 | 2 | 0 | 0 | 0 | -0.0 [-22.4, +22.4] |
| held4 | 4711e79ed6d1 | 4 | 0 | 0 | 0 | -63.2 [-85.8, -41.1] |
| held4-band2 | 1aaec24fbe8a | 4 | 0 | 2 | 0 | -48.1 [-70.5, -26.1] |
| held4-free1 | 0e084b263095 | 4 | 0 | 0 | 1 | -43.7 [-65.9, -21.8] |
| front4 | 16ccfee86b7b | 0 | 4 | 0 | 0 | -34.9 [-58.1, -11.9] |
| front8 | 9f289e5370cf | 0 | 8 | 0 | 0 | -82.3 [-106.4, -58.9] |
| held2-front4 | 333afb5dbc1e | 2 | 4 | 0 | 0 | -60.5 [-84.2, -37.4] |

Rewarding held land only is far less harmful than round 1, but nothing
beats v032: the lightest weight (held2) is level, and every heavier one is
weaker, the more the heavier. Along this axis (safety against expansion)
v032's balance, its threat charge of 5/4 on each cell the enemy can take,
already seems near the best; what separates v032 from zasmu is where it
plays, not how much it values safety (E084's `codingame-judge`).

### The games against zasmu: when are we behind?

The owner (2026-11-26): when does our evaluation tell us that we are
behind? On some maps we lose from the early game, letting the opponent reach
the areas of interest early; couldn't the evaluation hold this? v032's 195
CodinGame games against zasmu (16 or 17 won, by source; v032 red, the first
player, in all of them):

- `codingame-surprise` on all of them (3,060 of our plies, v032's search as
  the judge, 600 tries): the search's score of the turn it played (after
  the worst reply) is below 0 from game turn 7, when it takes over from the
  opening, in 57% of the lost games (median -7; +5 in the won ones), 77% at
  turn 8 (-17), and below -50 at a median of turn 11. The evaluation sees
  that we are behind as soon as it plays, mildly.
- The boards (medians, our side less zasmu's): a cell behind from game
  turn 2 (blue starts with two cells to red's one), -2 and -3 at turns 6 and
  7; the Voronoi share -7 at turn 2, -10 and -11 at turns 6 and 7; the steps
  to the middle front's centre (each side to its own, the maps being
  point-symmetric) 8 against 7 at turn 2, 3 against 1 at turn 6, 2 against
  0 at turn 7: zasmu reaches the middle centre at turn 7, a turn before us,
  having gained a step on us between turns 5 and 6. These numbers are the
  same in the games we won and the games we lost: the early deficit is
  systematic against zasmu, decided in turns 1 to 6, which the opening
  plays without a model of the opponent.

