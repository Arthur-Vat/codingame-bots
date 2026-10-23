# Review of ice-and-fire-151-p2-0

Blue (ice-and-fire-v003) won as blue, most likely by taking red's HQ, in 23 turns (46 plies); red (ice-and-fire-v004) lost as red. The file says only "the rules ended the game". Its last board, at ply 45, still shows red's HQ at (0,0), so the capture is inferred from blue's last commands (TRAIN 1 1 0; TRAIN 1 0 0); no board after ply 45 confirms it.

## Why the game was won

Blue won the corridor fights. Three times red moved its only unit off a one-cell link (plies 16, 22, 26), and blue trained into the gap with 30 to 50 gold, cutting 3 to 6 red cells each time. The lead (red material minus blue material, at the ply's start) went from -3 at ply 17 to -71 at ply 18 and never rose above -27 again. The largest single cut came at ply 29: a blue level 3 entered red's tower at (3,5), and by ply 30 red's active cells fell from 26 to 15 and the lead from -88 to -176.

## Early game (plies 0 to 13)

Both sides bought two level 1 units for 20 gold (red at ply 0, blue at ply 1), then expanded. The lead stayed between -10 and +9 from ply 0 to ply 14. I judge each play by the lead at its side's next turn, two plies later.

- Red, good: ply 10, BUILD MINE 1 0. Its income rose from 10 (ply 10) to 17 (ply 12), a turn before blue's first mine. Its lead still slipped to -10 by ply 12, because blue's units were ahead.
- Blue, good: ply 13, BUILD MINE 10 5. Income rose from 15 (ply 13) to 23 (ply 14); the lead was -4 at ply 13 and -17 at ply 15, so blue gained 13.
- Bad plays: none clear for either side.

The openings are alike in shape. This file does not show what the v004 opening weights changed on the first turn, and nothing in plies 0 to 13 shows the opening deciding the game.

## Middle game (plies 14 to 29)

- Red, good: ply 20, TRAIN 1 4 7 and TRAIN 1 5 7 refilled (4,7) and (5,7). The lead went from -75 to -48 by ply 22.
- Red, good: ply 24, MOVE 1 4 5, MOVE 23 4 4 and BUILD TOWER 5 5 retook (4,4) and (4,5). The lead went from -83 to -49 by ply 26.
- Red, bad: ply 16. MOVE 5 3 5 took its unit off (4,5), and three trainings spent 40 of 40 gold. At ply 17 blue trained level 1 units at (4,7), (4,6) and (4,5) for 30 gold. Red's active cells fell from 27 to 21, its units from 7 to 3 and its army from 80 to 30. The lead went from -30 (ply 16) to -71 (ply 18).
- Red, bad: ply 22. MOVE 21 3 7 left (4,7) empty. At ply 23 blue spent 50 on four trainings through (4,7), (4,6) and (3,6). Red's cells fell from 29 to 23, and the lead from -48 (ply 22) to -83 (ply 24).
- Red, bad: ply 26. MOVE 2 1 6 left (2,6) empty. Blue's ply 27 (TRAIN 2 1 6, TRAIN 1 2 6) cut three cells, and the lead went from -49 to -105 by ply 28.
- Red, bad (narrowly): ply 28. TRAIN 2 2 6 retook (2,6) and reconnected three cells (cells 22 to 26 by ply 29), but it spent 30 of its 34 gold. Blue then had 46 gold.
- Blue, good: ply 17 (above), ply 23 and ply 27. Ply 25 spent nothing, which let blue keep gold for ply 29.
- Blue, good: ply 29, TRAIN 3 3 5 (30 gold). Only a level 3 can enter a tower cell, and red had no level 3 and 4 gold. Red's towers on active cells fell from 2 to 0, and the lead from -88 to -176 by ply 30.

## Endgame (plies 30 to 45)

- Red, good: ply 30, TRAIN 2 4 4 retook (4,4) and cut red's cut-off cells from 6 to 4 (lead -176 to -153 by ply 31). Blue took the tower at (5,5) and three cells on ply 31, so the gain held one turn only.
- Red, bad: ply 40, BUILD TOWER 1 1 with one unit left (army 10). A tower stops only levels 1 and 2, and blue's units reached the HQ approaches after it. The lead went from -370 to -424 by ply 42, and red's cells from 8 to 6.
- Red's WAIT at plies 42 and 44, with 13 and 20 gold, is not judged. The file shows no purchase that a level 3 would not beat.
- Blue, good: ply 33, a level 3 on (4,4) cut red from 18 to 12 cells (lead -181 to -280 by ply 35).
- Blue, good: ply 39, two level 2 units at (2,1) and (2,2) for 40 gold (lead -349 to -384 by ply 41).
- Blue's upkeep grew to 32 by ply 30, and its income fell from 25 to 10. The army still counts fully in material, and blue won.

## Decisive moments

- Ply 29 (rules): agree. See the middle game.
- Ply 30 (rules): the 71-point swing is ply 29's result, measured from ply 28 (-105) to ply 30 (-176). It was not a red choice, and red's ply 30 reply was the only real recovery.
- Ply 33 (rules): agree. The lead went from -181 to -280 by ply 35.
- Ply 17 (reviewer's pick): missed by the rules, and the largest swing before ply 29. The lead went from -3 to -71 on four cells and four units.
- Ply 23 (reviewer's pick): missed by the rules. Fifty gold cut six cells, and the lead went from -48 to -83 by ply 24.
- Ply 16 (reviewer's pick): the move that made ply 17 possible.

## Lessons

Red:
1. Do not vacate a link cell ((4,5), (4,7) or (2,6)) while blue can train into it. This cost three cuts, at plies 16, 22 and 26.
2. Keep about 30 gold, or a unit beside each link, so a cut can be answered at once. Spending to 0 at ply 16 and to 4 at ply 28 left no answer.
3. A tower stops only levels 1 and 2. Keep a level 3 near a tower that blue can reach with 30 gold; ply 29 shows the cost of not doing so.

Blue:
1. Keep training into cells red has just left, as at plies 17, 23 and 27.
2. Keep saving for a level 3 strike. Blue spent nothing at ply 25 and had 46 gold at ply 29.
3. Watch upkeep. It reached 32 at ply 30 and cut income to 10. Blue won here, but a longer game would test it.

```json
[
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Mine at (10,5) lifted income from 15 to 23; the lead went from -4 to -17 by ply 15, so blue gained 13."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Moved the unit off (4,5) and spent all 40 gold; blue cut 4 cells and 4 units at ply 17, lead -30 to -71."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Three level 1 trainings at (4,7), (4,6), (4,5) cut 4 red cells; lead -3 to -40 by ply 19 (reviewer's pick)."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Two level 1 trainings refilled (4,7) and (5,7); lead -75 to -48 by ply 22."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "MOVE 21 3 7 left (4,7) empty; blue's 50-gold strike at ply 23 cut 6 cells, lead -48 to -83."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Four trainings for 50 gold through the empty corridor cut red from 29 to 23 cells (reviewer's pick)."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "Retook (4,4) and (4,5) and built a tower at (5,5); lead -83 to -49 by ply 26."},
  {"ply": 26, "side": "red", "verdict": "bad", "note": "MOVE 2 1 6 left (2,6) empty; blue cut 3 cells at ply 27, lead -49 to -105 by ply 28."},
  {"ply": 28, "side": "red", "verdict": "bad", "note": "Retook (2,6) but spent 30 of 34 gold; blue's level 3 at ply 29 took the tower at (3,5) with 46 gold, lead -105 to -176 by ply 30."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "TRAIN 3 3 5 took red's tower at (3,5); red cells 26 to 15, lead -88 to -153 by ply 31."},
  {"ply": 33, "side": "blue", "verdict": "good", "note": "Level 3 on (4,4) cut red from 18 to 12 cells; lead -181 to -280 by ply 35."},
  {"ply": 40, "side": "red", "verdict": "bad", "note": "Spent 15 on a tower at (1,1) with one unit left; a tower stops only levels 1 and 2, lead -370 to -424 by ply 42."}
]
```
