# Review of red-ranking-921358588

Blue (zasmu) won, as blue, in 20 turns (40 plies). Red was v022.

## Why the game was won

Red led on material from ply 20 to ply 28 (+61 at ply 22, +67 at ply 24) after a 50-gold cut at ply 20. Blue then built income and land while red did not: at ply 30 blue led by 46 (478 to 432), and the gap is explained by income times 3 (+45 for blue), cells times 4 (+40), towers (blue 6, red 2: +40), less red's gold idle (76 against 7: -69). Red kept winning the unit trades (cum_killed 90 against cum_lost 50 at ply 30), so the loss came from the economy, not the fights. The last turn (ply 39) put a row-0 chain of trainings onto red's HQ cell (TRAIN 1 0 0); the file has no board after ply 39, so the ending is inferred, not shown.

## Early game

Plies 0 to 11. Both sides trained two units at once and expanded at the same pace: at ply 11 red had 16 cells and material 150 against blue's 15 and 146. Neither side built a mine. Red had 3 free mine spots from ply 1 (free_spots at ply 1) and built none until ply 26. Good: blue's tower at (5,7), ply 13, was the first tower of the game and covered its frontier. Bad (red): the free mine spots were left idle while income stayed at 7 to 13 per turn.

## Middle game

Plies 12 to 36.

Red, good:
- Ply 20: 50 gold of trainings into blue's western pocket ((4,6) L2, (3,6), (3,7), (3,8) L1). Material went from -7 at ply 20 to +61 at ply 22; cum_killed_cut rose to 50.

Red, bad:
- Ply 18: no spend with 26 gold and 3 free mine spots; the lead went from +5 to -7 by ply 20.
- Ply 24: 15 gold on a tower at (5,3) and unit 5 left (5,5), which blue entered at ply 25 (TRAIN 1 5 5). The lead fell from +67 (ply 24) to +40 (ply 26).
- Ply 26: its first mine at (5,2) was right (income 25 to 30 by ply 28), but its moves left the north-east pocket open; blue's ply 27 cut followed and the lead fell to +8 at ply 28.
- Ply 28: 25 gold on a tower at (6,1), far from the fight, and one level 1 at (7,2) that did not retake (7,3). The lead fell to -46 at ply 30.
- Ply 30: a tower at (6,5) and a home mine at (1,0) (+4 income). Blue kept (7,3) and (7,4) and the lead fell to -68 at ply 32.
- Ply 32: a level 2 at (8,2) raised upkeep from 14 to 18 and took no cell; income fell 33 to 31 and the lead fell to -113 at ply 34.

Blue, good:
- Ply 21: TRAIN 2 3 7 retook (3,7) and its first mine at (10,11) was built; material went from -86 to -51 at ply 23.
- Ply 25: TRAIN 1 5 5 entered red's centre; towers at (6,8) and mine at (6,9). Material from -43 to -26 at ply 27.
- Ply 27 (reviewer's pick): TRAIN 2 7 4 and TRAIN 1 7 3 took (7,4) and (7,3) and cut red's (8,3) unit. Red lost 20 of training cost (cum_lost 30 to 50). The lead went from -26 to +31 at ply 29.
- Ply 29: towers at (7,4) and (4,9) and a level 1 at (6,6). The lead went from +31 to +90 at ply 31.

Blue's counts at ply 30: 2 mines (red 1), 6 towers (red 2), 48 cells (red 38), income 43 (red 28). Red's army at ply 36 was 10 units (upkeep 19) against blue's 6 (upkeep 12).

## Endgame

Plies 37 to 39. Blue spent nothing at plies 35 and 37, so its gold grew from 124 to 222 with no income gain, then spent 200 at ply 39 on a row-0 chain. Red's HQ approach was empty cells (x 2 to 8 on row 0) with no unit or tower. At plies 36 to 38 red was 1 to 3 gold short of a chain to blue's HQ (hq_gap) and spent 25 on a unit and a tower instead.

Red, bad (endgame): ply 36 and 38 spends did not defend the row-0 approach. Blue, good (endgame): ply 37 spent nothing and gained 14 material (+138 to +152 at ply 39).

## Decisive moments

- **Ply 22 (rules: red leads).** Correct but misleading. The lead came from the ply 20 cut and army (+70 of army in material), and it lasted only to ply 28.
- **Ply 30 (rules: blue leads).** Correct. Cause: income, cells and towers, as above.
- **Ply 39 (rules: HQ threat).** Blue's hq_gap was 0 and its final chain ended at (0,0). Most likely the decisive move, but the file does not show the result.
- **Ply 27 (reviewer's pick).** The real flip: a two-unit cut and capture in the north-east, which the rules missed because it moved no lead threshold until ply 29.

## Lessons

Red:
1. Build the free mines when gold allows: plies 12 to 20 had 2 to 3 free spots and 30 to 50 gold, and the first mine came at ply 26.
2. Guard the contact cells that blue can train into ((7,3), (7,4), (5,5)); a unit on each stops one-training captures.
3. Buy towers on the contact front, not at (6,1) or (1,1); keep upkeep near the blue's level, since 10 units cost 19 per turn.
4. Keep a unit or tower on the HQ approach (row 0 and column 1) in the endgame.

Blue:
1. Its mines and towers on contact cells (ply 13, 21, 25, 29, 31) were the economy engine; keep doing that.
2. Its hoard from ply 35 earned nothing; a mine or tower at a free spot would have been better if one was free.

```json
[
  {"ply": 18, "side": "red", "verdict": "bad", "note": "No spend with 26 gold and 3 free mine spots; lead fell from +5 to -7 by ply 20."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "50 gold of trainings into blue's western pocket cut 50 of training cost; lead rose from -7 to +61 at ply 22."},
  {"ply": 24, "side": "red", "verdict": "bad", "note": "Tower at (5,3) and unit 5 left (5,5); blue entered it at ply 25; lead fell from +67 to +40 at ply 26."},
  {"ply": 26, "side": "red", "verdict": "bad", "note": "First mine was right, but the moves left the north-east pocket open; blue cut and captured there at ply 27, lead fell to +8."},
  {"ply": 28, "side": "red", "verdict": "bad", "note": "Tower at (6,1), far from the fight; one unit at (7,2) did not retake (7,3); lead fell to -46 at ply 30."},
  {"ply": 30, "side": "red", "verdict": "bad", "note": "Centre tower at (6,5) and home mine at (1,0); blue kept (7,3)/(7,4); lead fell to -68 at ply 32."},
  {"ply": 32, "side": "red", "verdict": "bad", "note": "Level 2 at (8,2) took no cell and raised upkeep from 14 to 18; lead fell to -113 at ply 34."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Retook (3,7) with a level 2 and built its first mine; lead rose from -86 to -51 at ply 23."},
  {"ply": 25, "side": "blue", "verdict": "good", "note": "Entered red's centre at (5,5) and built a tower and a mine; lead rose from -43 to -26 at ply 27."},
  {"ply": 27, "side": "blue", "verdict": "good", "note": "Two trainings took (7,3) and (7,4) and cut red's (8,3); lead went from -26 to +31 at ply 29."}
]
```
