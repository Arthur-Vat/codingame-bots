# Review of ice-and-fire-151-p8-1

Blue (v004) won as blue, by taking red's HQ at ply 41, in 21 turns. Red is v003.

## Why the game was won

Red bought three level 3 units for 90 gold at ply 32. Their upkeep (20 each) moved red's upkeep from 3 to 63 and its income from +27 to -30 (ply 33). Blue waited while that upkeep drained red's bank from 230 gold to 23 by ply 39. Red's units died in debt at ply 40, and at ply 41 red had no units and no gold when blue spent 200 gold on a 14-unit chain to (0,0) (hq_gap 0). Before that, red had sat on 29 to 230 gold from ply 16 to ply 30 while its cells stayed at 22 and blue grew from 22 (ply 14) to 39 (ply 28).

## Early game

Both sides bought two level 1 units on turn 1 (ply 0 red, ply 1 blue) and expanded along the diagonal. The material lead stayed within about 10 either way (ply 0 red -6, ply 9 red +9, ply 13 red -7), so the openings were close and neither decided the game.

- Red, good, ply 10: BUILD MINE 1 0 with 15 gold. Income went from 10 (ply 10) to 17 (ply 11). A second mine at ply 12 (BUILD MINE 4 0) took income to 24 at ply 13. Blue built no mine until ply 35.
- Blue, good, ply 9: MOVE 6 7 7, MOVE 3 8 8, MOVE 4 6 9 and TRAIN 1 7 6 took its cells from 11 (ply 9) to 15 (ply 10). The lead moved from red +9 to blue +6.
- Blue, bad, ply 13: four moves and no build with 28 gold in hand. Income was 15 against red's 24, and two mine spots next to its active cells, (9,7) and (10,11), were free. This is the first sign of blue's economic gap, which lasted until ply 35.

The one visible opening difference is economic: red mined from ply 10, blue did not. One game cannot show whether v004's opening weights caused this, since the middle-game plan search is shared.

## Middle game

- Blue, good, ply 15: TRAIN 2 5 5, TRAIN 1 5 6 and BUILD TOWER 7 7 for 45 gold. Cells rose from 22 (ply 15) to 26 (ply 16) while red's fell from 22 to 21.
- Red, bad, ply 22: WAIT with 110 gold and can_cut 4. Red had waited at plies 18, 20, 22, 24 and 26, with its gold rising from 56 to 164 and its cells flat at 22. Blue went from 33 cells (ply 22) to 37 (ply 26) and 39 (ply 28). Red had the cuts available but spent none of the gold on them, on mines, or on units.

## Endgame

- Red, bad, ply 32: TRAIN 3 5 5, TRAIN 3 6 5 and TRAIN 3 6 6 for 90 gold. These killed two blue units and took the tower at (6,6), cutting blue's (5,6) cell (cut_off 1 at ply 33). But red's income fell from 27 to -30 and its material lead went from -63 (ply 32) to -193 (ply 33). It then had 140 gold against a -30 income, and the bank never recovered.
- Blue, good, ply 33: WAIT. Blue kept its 162 gold and 31 income and let red's upkeep drain it. Blue's lead rose to 239 by ply 35.
- Blue, good, ply 35: BUILD MINE 4 11 and BUILD MINE 7 11 for 20 gold lifted income from 30 to 38 (ply 36). Its TRAIN 1 9 7 and TRAIN 1 11 9 added units near the front.
- Red, good, ply 36: TRAIN 1 8 7 and TRAIN 1 7 8 cut 3 blue cells (cut_off 0 to 3 at ply 37). This was red's best endgame play, but its gold went to upkeep while income was already -29. Red's gold was 23 at ply 39 against a -27 income, so it went into debt at ply 40.
- Blue, good, ply 41: TRAIN 1, 3 and 1 units in a chain of 14 with 200 gold (hq_gap 0 at the start of the turn), ending on (0,0). The game ended on red's HQ.

Mid-game note for blue: plies 29 (TRAIN 1 10 11) and 27 (no build with 108 gold) left the (10,11) mine spot occupied and held gold idle while its income stayed near 27 to 29.

## Decisive moments

- Ply 34, turn 18, swing (rules): this is the consequence of ply 32, not the cause. The lead moved from -63 to -193 in one red turn, and the rules' swing point is where it became visible.
- Ply 39, turn 20, debt (rules): red could not pay its upkeep and lost its units. Red's gold had fallen from 230 at ply 32 to 23 at ply 39 under the -28 to -30 income. Without the 30 gold red spent at plies 36 and 38, debt would have come about two turns later, so the outcome was set at ply 32.
- Ply 41, turn 21, HQ threat (rules): blue's 200 gold made the chain to red's HQ affordable. This was the win.
- Ply 32, turn 17, red to play (reviewer's pick): the real decision. Three level 3 units for 90 gold raised upkeep 60 and turned the income negative. The rules flagged ply 34, but the damage was done here.
- Ply 22, turn 12, red to play (reviewer's pick): red's idle gold with can_cut 4. The waiting from here on gave blue the room to grow its cells from 33 to 39.

## Lessons

Red (v003):
- Before buying units, compute the income after their upkeep. A level 3 unit costs 20 a turn; three of them turned +27 into -30 at ply 32.
- Do not hoard. Red had 110 gold and can_cut 4 at ply 22 and kept waiting. Spend the bank on cuts, mines or units while the income is positive.
- Keep the gold above a few turns of negative income, or keep defenders at the HQ. Red had no units when blue arrived at ply 41.

Blue (v004):
- Build mines on the free spots early. Blue had 28 gold at ply 13 with two spots free next to its cells, and built its first mine only at ply 35.
- Do not stand units on mine spots: TRAIN 1 10 11 at ply 29 blocked (10,11).
- The plan of waiting while the enemy's upkeep drains it (ply 33) was right and should be kept.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Three moves and a level 1 train took cells from 11 to 15 (ply 10); the lead moved from red +9 to blue +6."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "BUILD MINE 1 0 lifted income from 10 to 17 (ply 11)."},
  {"ply": 12, "side": "red", "verdict": "good", "note": "BUILD MINE 4 0 lifted income from 17 to 24 (ply 13)."},
  {"ply": 13, "side": "blue", "verdict": "bad", "note": "Four moves and no build with 28 gold; income 15 against red's 24; no mine until ply 35."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "45 gold on a level 2, a level 1 and a tower; cells rose from 22 to 26 by ply 16."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "WAIT with 110 gold and can_cut 4; red's cells stayed at 22 while blue grew from 33 to 39."},
  {"ply": 32, "side": "red", "verdict": "bad", "note": "Three level 3 units for 90 gold: upkeep 3 to 63, income 27 to -30; lead -63 to -193 by ply 33."},
  {"ply": 33, "side": "blue", "verdict": "good", "note": "WAIT with 162 gold while red's upkeep drained it; lead rose to blue 239 by ply 35."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Two level 1 trains cut 3 blue cells (cut_off 0 to 3 at ply 37), though the gold went to upkeep."},
  {"ply": 41, "side": "blue", "verdict": "good", "note": "200 gold on a 14-unit chain to red's HQ at (0,0), which ended the game."}
]
```
