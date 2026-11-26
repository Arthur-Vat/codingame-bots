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

Pending.
