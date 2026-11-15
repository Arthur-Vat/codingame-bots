---
id: E066
date: 2026-11-15
pull_request: pending
parent: ice-and-fire-v026
hypothesis: In v025's and v026's CodinGame games against the top 5, a quarter of our unit turns at game turns 11 to 17 are units that stay next to cells that are not ours and that they cannot enter (two thirds of them face an enemy unit, nine in ten are level 1), against 17% of the top 5's in the same seat, whose units move through their own land more often; E060's term counts such a unit as next to land to take, so measuring a unit's distance to the nearest cell it may enter makes blocked units walk to land they can take, and beats v026.
change: The plan bot's evaluation measures E060's distance of a unit to the nearest cell not its side's that a unit of its level may enter (not behind a tower for levels 1 and 2, not held by a unit it cannot beat), rather than to any cell not its side's.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-15 ("Start it now"), after the scan of unit
turns in the CodinGame games (`ice-and-fire-review codingame-units`, a
draft on this branch) that he asked for to find why our units move less
than the top players' (0.3 to 0.4 moves a unit against 0.6 to 0.7 from
game turn 11).

## Why

The scan classifies each turn of a unit that may move, for us and for the
top 5 in the same seat (v025, 22 games a seat; shares of unit turns at
game turns 11 to 13, ours against theirs, red then blue): stayed 51%
against 26%, 56% against 26%; of which stayed though it could take a
cell 7% against 5%, 12% against 8% (mostly with an enemy unit within two
steps); stayed next to cells not ours, none of which it may enter, 25%
against 17% in both seats; stayed inside our land 20% against 4%, 20%
against 1% (v026, with E060: 9% against 4%). The search does not miss
captures; the units that stand still have nothing to take where they
are. E060 made the units inside walk; a unit at a wall of towers or of
units it cannot beat still costs nothing, since a cell not ours is next
to it.

## The change

`bots/plan/src/eval.rs`, `idle`: `IDLE_ENTER` (0: v026). With 1, the
units are grouped by level and each group's distance is measured to the
cells not its side's that `State::enterable` gives a unit of that level;
with 2, only for level-1 units. Steps are still counted over any cell of
the map, so a wall is not walked around.
