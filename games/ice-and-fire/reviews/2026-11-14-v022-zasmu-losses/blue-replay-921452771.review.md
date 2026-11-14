# Review of blue-replay-921452771 (zasmu vs v022)

zasmu won as red, over v022 as blue, at turn 20 (ply 38). The file shows no final board, so the finish is taken from the ply 38 note: red's 220-gold chain of trainings reached blue's HQ, with red's hq_gap at 0.

## Why the game was won

Red built its lead on the middle-game front: two level 2 trainings (plies 16 and 20) killed or cut about 40 gold of blue units, and red's towers (11 by ply 29) held the line that blue never broke. Blue never trained a level 2 or level 3 unit (level2 and level3 stay 0 in every blue row), so its army of level 1 units could not take a tower cell. Blue also let its gold sit idle, from 28 at ply 13 to 108 at ply 37, while red's lead grew from 116 at ply 20 to 186 at ply 22 and to 257 at ply 38.

## Early game

Both sides opened with two level 1 units and expanded at similar speed; blue even led in cells at ply 10 (15 to 13). Red's ply 12 moved two level 1 trainings to (6,5) and (7,5) with 20 gold, and cells went from 17 to 23 by ply 13. Blue's ply 13 spent 10 of 28 gold, and its voronoi (49 to red's 68) was already behind.

- Red, good, ply 12: trained at the front and moved units outward; the lead went from -7 to +6 by ply 14.
- Blue, bad, ply 13: kept 18 of 28 gold while red's cells rose; the lead fell 11 by ply 15.

## Middle game

Red's ply 16 trained a level 2 unit at (6,6) and moved its units into blue's cells, taking 20 cost of blue units (cum_killed 20 at ply 17). Ply 20 was the swing: a level 2 unit at (5,8), a tower at (4,6) and moves cut a blue group (cum_lost 20 to 40, blue cells 28 to 25) and the lead rose from 116 to 186 by ply 22. Red kept building towers on its front (2 at ply 22, 4 by ply 24, 7 by ply 25, 9 by ply 27); blue had 3 towers at ply 23 and only 1 more at ply 31.

Blue's ply 17 spent 35 on one level 1 unit, a tower at (7,7) and another unit, while red's level 2 sat in the middle; the lead fell 29 next turn. Blue's ply 19 spent nothing with 32 gold and its contact cells at (4,8) and (5,8) were left to red, which cost 72 of lead over plies 19 to 21. Its ply 21 spent 30 on two towers at the back and the lead fell 19 more.

Blue's ply 29 was its best play: a level 1 training onto red's (5,10) cut two red units off, and the lead gap closed by 43 by ply 31. Red's ply 28 was its worst: 40 gold went into towers at (2,3) and (1,8), far from the fight, and its units were moved to the bottom left, leaving (5,10) as a thin neck. Blue's cut cost red 20 of cost (cum_lost 0 to 20) and the lead fell 33.

- Red, good, ply 16: level 2 unit plus moves; killed 20 of blue cost; lead +30 to +68.
- Red, good, ply 20: level 2 unit, a tower and moves; killed 20 more, cut blue cells; lead +70.
- Red, bad, ply 28: towers far from the front, thin neck at (5,10); blue cut it at ply 29, lead -33.
- Blue, bad, ply 17: 35 gold on one unit and a tower, no level 2; lead -29 next turn.
- Blue, bad, ply 19: no spending with 32 gold while its frontier was attacked; lead -72 over two turns.
- Blue, bad, ply 21: towers at the back instead of units at the contact; lead -19.
- Blue, good, ply 29: training onto (5,10) cut red units and closed 43 of the gap.

## Endgame

Red stopped spending at ply 30 and kept 100 percent of its gold for four turns (51 at ply 30, 204 at ply 36) while its income stayed at 47 to 55. Blue held 81 to 108 gold, spent 15, 40, 30 and 20 on towers at the back, and never built a level 2 or 3 unit. At ply 37 blue's two level 1 units cut two red cells at (5,10) and (5,11), but at ply 38 red spent 220 on 4 level 3 units, 2 level 2 units and 6 level 1 units, running down column 11 to blue's HQ at (11,11). Blue's tower cells (11,10), (10,8) and (9,8) did not stop it.

- Red, good, ply 32: hoarded for a single strike; lead rose from 203 to 232 by ply 34.
- Blue, bad, ply 35: towers at (9,10) and (10,8) while red's cut at ply 36 took 20 of cost (cum_lost 40 to 60); lead -44.

## Decisive moments

- **Ply 22 (swing, red):** a real swing, but it was made at ply 20. The rules flag the lead move, and the cause is red's level 2 and cut at ply 20.
- **Ply 38 (HQ threat, red):** the decisive finish. Blue's gold at ply 37 (108) could have bought level 2 and level 3 units near its HQ, or towers on (10,10) and (11,9), before red's chain was paid.
- (reviewer's pick) **Ply 19 (blue, no spend with 32 gold):** the largest drop for blue in the middle game (lead -72 over plies 19 to 21). Blue's frontier was not reinforced at the moment red's level 2 unit was in place.
- (reviewer's pick) **Ply 29 (blue, cut by training):** the best blue play; a cheap level 1 training onto a red cell cut two units, and the rules do not flag it.

## Lessons

**v022 (blue):**
1. Train level 2 and level 3 units once the enemy has level 2 units at the front; level 1 units could not win a single contested cell all game.
2. Do not hoard. From ply 27 blue held 65 to 108 gold; spend it on units at the contact and on towers near the HQ, not on the back.
3. Build the free mine spots (blue had 2 unused from ply 31 to 37) and keep income near red's (blue 35 to 39 against red's 47 to 55).

**zasmu (red), for reference:**
1. Keep towers on the front line and spread them early (red had 4 by ply 24).
2. Hoard when the enemy cannot attack, then strike the HQ with level 3 units in a column.

## Reviewer's note on the file

The file shows no board after ply 38, so the HQ capture is not confirmed from the board itself.

```json
[
  {"ply": 12, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (6,5) and (7,5) with 20 gold; lead -7 to +6 by ply 14."},
  {"ply": 13, "side": "blue", "verdict": "bad", "note": "Spent 10 of 28 gold; red's cells rose 23 to 29 and the lead fell 11 by ply 15."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Level 2 at (6,6) with moves killed 20 of blue cost; lead +30 to +68."},
  {"ply": 17, "side": "blue", "verdict": "bad", "note": "35 gold on one unit and a tower; no level 2 against red's; lead -29 next turn."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "No spend with 32 gold; red's level 2 and cut at ply 20 took 20 more; lead -72 over two turns."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Level 2 at (5,8), tower at (4,6) and moves killed 20 more and cut blue cells; lead 116 to 186."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Two towers at the back instead of units at the contact; lead -19 next turn."},
  {"ply": 28, "side": "red", "verdict": "bad", "note": "40 gold on towers far from the fight and units moved to the bottom left; blue cut (5,10) at ply 29, lead -33."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Level 1 training onto red's (5,10) cut two red units; lead gap closed by 43 by ply 31."},
  {"ply": 32, "side": "red", "verdict": "good", "note": "Hoarded all gold for a single strike; lead rose from 203 to 232 by ply 34."},
  {"ply": 35, "side": "blue", "verdict": "bad", "note": "Two back-line towers while red's cut at ply 36 took 20 of cost; lead -44 over the next turn."}
]
```
