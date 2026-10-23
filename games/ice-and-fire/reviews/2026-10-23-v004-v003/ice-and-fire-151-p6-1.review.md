# Review of ice-and-fire-151-p6-1

v003 (red, the parent) beat v004 (blue, the candidate) in 49 turns (97 plies). The game ended before turn 100, so under the stated rules red took blue's HQ; the file does not show the capturing order.

## Why the game was won

Blue's 100-gold cut at ply 33 cut red's active cells from 31 to 12 and put blue 143 material ahead at ply 34. Red then won by cutting back again and again: at plies 34, 58, 72, 76 and 80 it took 9 to 23 blue cells each time, while blue held 0 to 11 gold at plies 76 to 80 and could not re-link them. Blue never built a mine (0 in every ply; red had 4 by ply 93), so its income stayed at 11 to 20 late while red's ran at 32 to 46 (plies 78 to 92). Final material: red 683, blue 186 (ply 96).

## Early game (plies 0 to 13)

Both sides opened with two level 1 units next to their HQs (red `TRAIN 1 1 0; TRAIN 1 1 1` at ply 0, blue `TRAIN 1 10 10; TRAIN 1 9 10` at ply 1). Material stayed within 11 of level. The openings did not differ in a way that mattered: blue's +11 at ply 13 (171 to 160) was down to +3 by ply 16. The file does not show which opening weights v004 chose, so this judgment rests on the plays alone.

- Blue, good, ply 9: `MOVE 3 8 8; MOVE 6 6 8; MOVE 4 6 9; TRAIN 1 6 7` (10 gold). Its cells rose from 10 to 14 and the lead went from red +9 to blue +6 by ply 11.
- Red: no clear error. Its first mine at (1,0) (ply 10, 15 gold) let the lead slip from +9 to -6 over the turn, but the mine ran for the rest of the game.

## Middle game (plies 14 to 33)

- Red, good, ply 14: `TRAIN 1 6 4; TRAIN 2 6 5` and three moves (30 gold). The lead went from -22 (ply 14) to -3 (ply 16).
- Blue, good, ply 17: `TRAIN 2 4 5` and four moves (20 gold) entered red's centre at (4,5) and (5,5). Blue's cells rose from 24 to 28; the lead went from -2 to -35 for red by ply 19.
- Red, bad, ply 18: `BUILD TOWER 4 4; BUILD TOWER 6 4` (30 gold, gold 39 to 9). The towers did not stop blue's units; the lead fell from -12 to -28 by ply 20.
- Red, bad, ply 22 (reviewer's pick; the rules did not flag it): `TRAIN 3 6 6; TRAIN 1 5 6; TRAIN 1 4 6` (50 gold). A level 3 unit costs 20 upkeep: red's upkeep went from 7 to 29 and its income from 22 to 4 (ply 23). Income stayed at 4 to 7 through ply 33, and red held 1 to 18 gold. The material barely moved (-35 at ply 22, -37 at ply 24), but the cost was in gold: blue's reserve grew from 66 (ply 23) to 107 (ply 33).
- Blue, good, ply 33: `MOVE 10 5 6; TRAIN 2 5 5; TRAIN 3 5 4; TRAIN 2 5 3; TRAIN 1 5 2; TRAIN 1 4 2; TRAIN 1 3 2` (100 gold). Red's cells fell from 31 to 12 (12 cut off) and its army from 90 to 10. Red's counter at ply 34 took back 9 blue cells, so the lead was blue +52 at ply 35. The cut was real, but blue spent its reserve on it.

## Endgame (plies 34 to 96)

- Red, good, ply 34: `TRAIN 2 4 2; TRAIN 1 4 6` (30 gold). The level 2 at (4,2) rejoined red's north side; the lead went from -143 to -91 by ply 36.
- Red, good, ply 58: ten trainings and a move (140 gold). Blue's cells fell from 39 to 16 (13 cut off), and the lead went from -49 to +97 for red at ply 59. Blue re-linked at ply 59 (`TRAIN 2 5 9; TRAIN 2 5 8; TRAIN 2 5 7`), so the lead was -29 at ply 60. Red's income had also gone to -15 for a turn.
- Red, good, ply 72: `TRAIN 1 7 3` (10 gold) took (7,3) and cut blue's north-east pocket: blue's cells fell from 37 to 26. The lead went from -88 to +2 over the turn.
- Red, good, ply 76: `MOVE 75 8 4; MOVE 76 5 6; TRAIN 2 4 7; TRAIN 1 4 8` (30 gold). Blue's cells fell from 37 to 24; the lead went from -1 to +107 at ply 77.
- Red, good, ply 80: 30 gold, blue's cells from 33 to 22; the lead went from +45 to +125 by ply 82.
- Red, bad, ply 62: `MOVE 44 5 6; TRAIN 1 5 7; TRAIN 1 5 8` (20 gold, 26 left). Blue's 62 gold could cut 7 red cells (can_cut 7 at ply 62), and the spend went to the centre, not the north-east. Blue's 50-gold play at ply 63 cut 10 red cells (39 to 29, 3 cut off by ply 64); the lead went from -71 to -168 for red.
- Blue, good, ply 59: the re-link above, which took back 122 of the lead over the turn (reviewer's pick; the rules did not flag it). Blue, good, ply 63: its 50 gold cut the north-east (+55 over the turn).
- Blue, bad, ply 75: `TRAIN 2 5 7; TRAIN 1 4 7; MOVE 62 9 3; MOVE 61 8 5; MOVE 39 5 8` (30 gold, 0 left). It left 9 cells cuttable with 30 gold (exposed30 9 at ply 76) against red's 36 gold; red cut 37 to 24 at ply 76. Blue's lead over its next turn fell by 47.
- Blue, bad, ply 77: `MOVE 39 5 9; TRAIN 1 4 6; MOVE 78 5 6` (10 gold, 0 left). It retook 10 cells (24 to 34), but red's 41 gold cut 10 back at ply 78.
- Blue, bad, ply 81: `TRAIN 1 6 9; MOVE 61 8 3` (10 gold, 4 left). It reconnected 22 to 30 cells, but red's ply 82 (30 gold) cut 9 back; the lead over blue's next turn fell by 45.

## Decisive moments

- Ply 33 (large cut, blue): the biggest middle-game swing, and correct, but red's ply 34 took back 9 cells, so blue's lead was +52 at ply 35.
- Ply 34 (swing, blue): the same swing seen from red's seat. Red could only counter-cut.
- Ply 58 (large cut, red): the turning point, but blue's ply 59 took back 126 of the lead within one turn.
- Ply 64 (swing, blue): caused by blue's ply 63 cut of red's north-east. Red's ply 62 had left that corridor open.
- Ply 74 (swing, red): caused by red's ply 72 (`TRAIN 1 7 3`, 10 gold), not by a later play.
- Plies 76, 80 and 82 (cuts, red): the real swing plays. Plies 78, 84 to 94 are consolidation: each cut left blue with 0 to 16 gold to reply.
- Ply 22 (reviewer's pick): red's level 3 upkeep, which the rules did not flag.

## Lessons

Blue (v004):
1. Build mines. It built none in 49 turns, though it owned mine spots at (6,7) from ply 12, (5,8) from ply 14, and (10,11) next to its HQ from ply 0. Each mine costs 15 and returns 4 a turn.
2. Keep a reserve for the link. Plies 75 to 81 show that cells re-linked with 0 to 10 gold left were cut again the next turn.
3. Keep making cuts (ply 33, ply 59, ply 63 were right); the mistake was running out of gold between them.

Red (v003):
1. Do not buy level 3 units on an income of 22: ply 22 cut income to 4 for ten turns.
2. Keep a guard on the north-east corridor: at ply 62 red left it open and blue cut 10 cells at ply 63.
3. Repeat the cut when blue is low on gold (plies 72, 76, 80 all worked that way).

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Ten gold of moves and one level 1 grew its cells from 10 to 14; the lead went from red +9 to blue +6 by ply 11."},
  {"ply": 14, "side": "red", "verdict": "good", "note": "Two trainings and three moves for 30 gold; the lead went from -22 to -3 by ply 16."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "A level 2 entered red's centre at (4,5); blue's lead was +35 by ply 19."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Two towers for 30 gold did not stop blue's centre push; red's lead fell from -12 to -28 by ply 20."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "A level 3 unit (20 upkeep) cut income from 22 to 4 for ten turns; the material barely moved, but red had little gold left to answer blue."},
  {"ply": 33, "side": "blue", "verdict": "good", "note": "100 gold cut 19 red cells (31 to 12) and put blue +143 at ply 34; red took 9 back, leaving blue +52."},
  {"ply": 34, "side": "red", "verdict": "good", "note": "A level 2 at (4,2) and a level 1 re-joined red's north side and took 9 blue cells; the lead went from -143 to -91 by ply 36."},
  {"ply": 58, "side": "red", "verdict": "good", "note": "140 gold cut blue from 39 to 16 cells and moved the lead to red +97 at ply 59; blue re-linked, so the lead was -29 at ply 60."},
  {"ply": 62, "side": "red", "verdict": "bad", "note": "Spent 20 in the centre while blue's 62 gold could cut 7 red cells; blue cut 10 north-east cells at ply 63 and the lead fell by 97."},
  {"ply": 72, "side": "red", "verdict": "good", "note": "One 10-gold level 1 took (7,3) and cut blue's north-east pocket (37 to 26 cells); the lead went from -88 to +2 over the turn."},
  {"ply": 75, "side": "blue", "verdict": "bad", "note": "Spent its 30 gold and left 9 cells cuttable with 30 gold; red cut 37 to 24 at ply 76 and blue's lead fell by 47."},
  {"ply": 81, "side": "blue", "verdict": "bad", "note": "Re-linked 22 to 30 cells but kept only 4 gold; red's 30-gold cut at ply 82 took 9 back and the lead fell by 45."}
]
```
