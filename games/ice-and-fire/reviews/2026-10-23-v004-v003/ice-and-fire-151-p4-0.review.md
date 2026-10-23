# Review of ice-and-fire-151-p4-0

Blue v003 won as blue, against red v004, by taking red's HQ, in 23 turns (46 plies). The file's last board is the one before ply 45, so the capture is inferred from the rules and the trainings of ply 45.

## Why the game was won

Blue cut red's land off from its HQ twice in the middle game, and each cut removed every unit beyond it: after ply 17 red's cells fell from 25 to 14 and its army from 70 to 30 (ply 17 to ply 18). Red's bank went into units standing in cut-off pockets (plies 16 and 22), so it never had a lasting defence. Blue then kept taking cells and towers with level 3 units while red fell to 15 cells and 12 income, against blue's 101 cells and 61 income at ply 45.

## Early game

Both sides opened alike: two level 1 units on turn 1 (red at ply 0, blue at ply 1), then moves into the centre. The opening did not decide anything. Red led material at ply 7 (91 vs 84) and ply 9 (119 vs 110), and blue led from ply 13 (189 vs 175). Red's own differences were a forward level 1 at (3,3) (ply 6), and mines at (1,0) (ply 10) and (4,2) (ply 12), which took its income from 10 (ply 10) to 23 (ply 14). Blue built no mine until ply 41, though (10,1) and (3,7) were mine spots from ply 0.

- Red, good or bad: ply 10 is a sound mine, but the lead barely moved (-6 to -10), so it is not judged.
- Blue, good, ply 11: four moves took cells from 15 to 19 and voronoi from 57 to 64 by ply 12; its lead went from -1 to +14 by ply 13.
- Red, bad, ply 14: no spending, 27 gold idle, while (5,3) to (7,3) stayed empty next to blue's (8,3). Blue's chain used exactly those cells at ply 15.

## Middle game

- Blue, good, ply 15: four level 1 trainings (40 gold) chained west along row 3 from (8,3) to (4,3). Red's cut-off went to 3 cells, cells from 21 to 17, and its army from 30 to 20 (ply 16). The lead went from +17 to +73.
- Red, bad, ply 16: a level 2 at (4,3) retook the cell and reconnected the three cells (cut-off 3 to 0), but 30 of its 50 gold went into level 1 units at (4,6), (4,7) and (5,7), a pocket hung off row 4.
- Blue, good, ply 17: trainings and moves on (2,4), (3,4) and (4,4) cut that link. Red's cut-off went from 0 to 7, cells from 25 to 14, units from 6 to 2, and four units died at ply 18. The lead went from +47 to +182.

## Endgame

- Red, good, ply 18: moving unit 12 onto (4,4) retook the link and cut two blue cells. Red's cut-off fell from 7 (ply 18) to 4 (ply 20) and 0 (ply 21), and the lead went from +182 to +167 by ply 20.
- Red, bad, ply 20: a tower at (3,5) and a level 1 for 25 gold left red with 0 gold. Only a level 3 can enter a tower's cell, and red had none. Blue's level 3 took the tower at ply 23. The lead went from +167 to +188.
- Red, bad, ply 22: a level 2 for 20 of its 22 gold at (3,7). Blue's ply 23 cut it off with two more units: red's units went from 5 to 2, army from 70 to 20, and cells from 23 to 13 by ply 24. The lead went from +188 to +309.
- Blue, good, ply 23: a 30-gold level 3 at (3,5) took red's tower, and blue spent 60 of its 64 gold. Its upkeep rose from 9 to 32, so income fell from 36 to 23, but the lead went from +208 to +337 by ply 25.
- Blue, good, ply 45: 100 gold on two level 3 units at (3,2) and (2,2) (red's tower), then level 1 units at (2,1), (2,0), (1,0) and (0,0). Blue's hq_gap was 0 with 109 gold in hand.

## Decisive moments

- Ply 17 (flagged): it mattered. It cut 7 cells and four units, and set the endgame.
- Ply 18 (flagged): the swing was ply 17's. Red's reply won back its cells by ply 21 but not the lost army, and the lead stayed near +170.
- Ply 23 (flagged): it mattered most in the endgame. A level 3 took the tower and cut 10 cells.
- Ply 24 (flagged): a consequence of ply 23. Red's 17 gold could not buy a level 3 to answer.
- Plies 26 and 28 (flagged): no single cut. Blue's growth from 55 to 68 cells and income from 23 to 34 did the work, and red had no answer.
- Ply 45 (flagged): the finishing play.
- Ply 14 (reviewer's pick): red's idle gold and open row 3 gap set up ply 15, and it was missed by the rules.
- Ply 10 (reviewer's pick): red's first mine. It was sound, but it did not decide the game.

## Lessons

Red:
1. Before training, check that the new unit's cells stay joined after one cut. Three level 1 units in a pocket died at ply 18.
2. Guard the choke. At ply 14 red had 27 gold and an open gap; a unit on (5,3) would have stopped the chain.
3. Keep a reserve for a level 3. Red never trained one and sat at 0 to 2 gold after plies 16, 20 and 22, so blue's 30-gold level 3 took its tower.

Blue:
1. Keep the cut-and-train plan of plies 15, 17 and 23. It won the game.
2. Build the mines. Blue built none until ply 41, though a 15-gold mine pays 4 a turn.
3. Watch upkeep: a level 3 costs 20 a turn, and blue's income fell from 36 to 23 after ply 23.

```json
[
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent nothing with 27 gold; left (5,3) to (7,3) open next to blue's (8,3). Blue's four-unit chain at ply 15 cut 3 cells."},
  {"ply": 11, "side": "blue", "verdict": "good", "note": "Four moves took cells 15 to 19 and voronoi 57 to 64; lead went from -1 to +14 by ply 13."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Four level 1 trainings chained along row 3 cut 3 red cells; red's army fell 30 to 20."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Retook (4,3) and reconnected, but 30 of 50 gold went into units in a pocket that died at ply 18."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Row 4 trainings cut red's link: cells 25 to 14, army 70 to 30, lead +47 to +182."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Unit 12 to (4,4) retook the link and cut two blue cells; red's cut-off fell from 7 to 4 by ply 20."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Tower at (3,5) plus a level 1 left 0 gold; only level 3 can enter it, and blue's level 3 took it at ply 23."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "Level 2 for 20 of 22 gold at (3,7) was cut off at ply 23 with two more units; cells 23 to 13, lead +188 to +309."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "30-gold level 3 took red's tower at (3,5) and cut 10 cells; lead +208 to +337 by ply 25."},
  {"ply": 45, "side": "blue", "verdict": "good", "note": "100 gold of level 3 and level 1 trainings chained to red's HQ at (0,0); hq_gap 0 with 109 gold."}
]
```
