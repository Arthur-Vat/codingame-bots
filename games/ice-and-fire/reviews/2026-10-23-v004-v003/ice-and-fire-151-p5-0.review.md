# Review of ice-and-fire-151-p5-0

Blue (v003) won as blue, at turn 16 after 32 plies. The file records only "the rules ended the game"; the chain of trainings to red's HQ at ply 31 is the likely cause.

## Why the game was won

Blue won the middle and endgame by cutting red's corridor from its HQ again and again (plies 15, 21, 25, 27, 29), using level 1 trainings across red's cells. Red's active cells fell from 31 at ply 21 to 10 at ply 30, while blue's rose from 37 to 65. Blue's income reached 51 against red's 8 (ply 30), which paid for the final chain: blue's hq_gap was 21 at ply 30 and 0 at ply 31. The openings did not decide it: material stayed within 13 until ply 14.

## Early game

Both sides opened with 20 gold of level 1 units (red ply 0: `TRAIN 1 1 0; TRAIN 1 1 1`; blue ply 1: `TRAIN 1 10 10; TRAIN 1 10 9`). Blue started with two owned cells to red's one (material 34 to 28, ply 0), and the lead stayed small: blue +6 at ply 4, +4 at ply 8, +6 at ply 10.

- Red, good, ply 10: `BUILD MINE 1 0` next to its HQ, 15 gold. Income went from 10 to 17 at ply 11. It was red's only mine; blue built none all game.
- Red, no verdict, ply 12: it spent nothing with 19 gold while blue's army grew to 40. The idle gold was punished later (see ply 14).
- Blue, good, ply 1: 20 gold on its own corner gave two more cells (4 at ply 2) and material 36 to 44.

## Middle game

- Red, bad, ply 14: `TRAIN 2 7 3; TRAIN 1 7 4` (30 gold) took blue's cell (7,3) and placed units one step beyond its corridor. At ply 15 blue's trainings on (6,3) and (6,4) cut both units off; they died at red's next turn, army 60 to 30 (ply 16). Lead went from red +3 (ply 15) to blue +54 (ply 16).
- Red, good, ply 16: `TRAIN 2 6 4; TRAIN 1 5 4` (30 gold) retook the cells: cut_off 2 to 0, cells 21 to 28, lead +54 to +15 (ply 17).
- Red, good, ply 18: `TRAIN 1 6 5; TRAIN 1 4 6` retook (6,5): cells 26 to 32, lead +57 to +20 (ply 19).
- Red, bad, ply 20: `TRAIN 2 6 2` (20 gold, gold 28 to 8) retook (6,2) and five cells (31 at ply 21), but left the corridor at (2,3) to (4,3) open with no gold to hold it. Lead +99 (ply 20) to +58 (ply 21); blue's answer at ply 21 cut ten cells.
- Blue, good, ply 15: `MOVE` orders plus `TRAIN 1 6 4; TRAIN 1 6 3` cut red's two units and took the pocket. Lead went from red +3 to blue +54.
- Blue, good, ply 17: `TRAIN 1 7 3; TRAIN 1 7 4` retook (7,3) and (7,4): lead +15 to +57 (ply 18).
- Blue, good, ply 19: three level 1 trainings took six red cells (32 to 26): lead +20 to +99.
- Blue, good, ply 21 (flagged): `TRAIN 1 4 3; TRAIN 1 3 3; TRAIN 1 2 3` (30 gold) across red's corridor. Red's cut_off went to 10, cells 31 to 17, lead +58 to +209 (ply 22). Note that red's exposed30 at ply 21 read only 4, so the metric understated this cut.

## Endgame

- Red, good, ply 22: `TRAIN 2 3 3` (20 gold) retook (3,3) and reconnected its pocket: cut_off 10 to 0, cells 17 to 29, lead +209 to +101 (ply 23). The rules flagged this ply as a swing for blue; the swing was blue's ply 21, and red's reply took back 108 of it.
- Red, good, ply 24: cut_off 2 to 0, cells 23 to 28, lead +181 to +159.
- Red, good, ply 26 (limiting damage): cells 15 to 21, cut_off 6 to 3, lead +292 to +243. The game was already lost.
- Red, bad, ply 28: `TRAIN 2 4 2` spent 20 of 22 gold. Cut_off stayed at 4, income fell 16 to 13 (ply 29), lead +336 to +356.
- Red, bad, ply 30: `WAIT` with 10 gold, while blue's income was 51 against red's 8. A level 1 unit on (1,1) was the only cheap block left; the file does not show whether it would have held.
- Blue, good, ply 23: four level 1 trainings and two moves: red cells 29 to 23, cut_off 0 to 2, lead +101 to +181.
- Blue, good, ply 25: `TRAIN 1 4 5; TRAIN 1 4 4; TRAIN 1 3 4` and moves: red cells 28 to 15, cut_off 6, lead +159 to +292.
- Blue, good, ply 27: red cells 21 to 14, lead +243 to +336.
- Blue, good, ply 29: red cells 15 to 10, cut_off 4 to 1, lead +356 to +448.
- Blue, good, ply 31: `TRAIN 1 1 1; TRAIN 1 1 0; TRAIN 1 0 0` (30 gold) walked a chain onto red's HQ at (0,0).

## Decisive moments

- Ply 21 (blue, large cut): correct and decisive. Lead +58 to +209.
- Ply 22 (flagged swing): the swing was blue's ply 21. Red's ply 22 recovered 108 of it, so the flag misreads the turn.
- Ply 25 (blue, large cut): red cells 28 to 15. Correct for blue.
- Ply 26 (flagged swing): the swing was blue's ply 25. Red's reply recovered 49.
- Ply 27 (blue, large cut): red cells 21 to 14. Correct for blue.
- Ply 30 (flagged swing): blue's ply 29. Red's `WAIT` gave nothing back.
- Ply 31 (HQ threat): the decisive final move.
- Ply 15 (reviewer's pick, not flagged): blue's first real cut. Lead -3 to +54, and red's ply 14 units were lost. The rules did not flag it, probably because only three cells changed hands.
- Ply 16 (reviewer's pick, not flagged): red's counter-cut, lead +54 to +15, the best red reply of the middle game.
- Ply 20 (reviewer's pick): red's counter that set up the ply 21 cut.

## Lessons

Red (v004):
- Keep the HQ corridor garrisoned before spending on counter-attacks. Ply 20 retook five cells and left the corridor open, which cost ten cells at ply 21.
- Do not place single units beyond supported cells. The ply 14 units were lost to one cut.
- Do not hold gold idle (19 at ply 12, 39 at ply 14) and do not spend it on units that reconnect nothing (ply 28). Keep the last gold for a blocker near the HQ (ply 30).

Blue (v003):
- Keep the method: level 1 trainings across the enemy corridor won five cuts.
- Hold the cut cells. Red retook (3,3) with a level 2 at ply 22 and reconnected 12 cells. A level 2 on that cell would have stopped a level 2 entry, since a unit only beats a lower level.

## Opening comparison

Both openings bought the same material (20 gold of level 1 units), and the material gap stayed within 13 until ply 14. Red's opening did not show an edge or a disadvantage that mattered. One game cannot measure v004 against v003, because middle-game cuts dominated the result.

```json
[
  {"ply": 10, "side": "red", "verdict": "good", "note": "Built a mine next to its HQ (15 gold); income 10 to 17 at ply 11, the only mine of the game."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Trained a level 2 and a level 1 beyond its corridor at (7,3) and (7,4); blue cut both at ply 15, army 60 to 30, lead +3 red to +54 blue."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Level 2 and level 1 trainings retook cut cells: cut_off 2 to 0, cells 21 to 28, lead +54 to +15."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Retook (6,5) with trainings and reconnected: cells 26 to 32, lead +57 to +20."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Retook (6,2) for 20 gold but left (2,3) to (4,3) open with 8 gold left; blue's ply 21 cut ten cells."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "Level 2 on (3,3) reconnected 12 cells: cut_off 10 to 0, lead +209 to +101."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "Retook cells: cut_off 2 to 0, cells 23 to 28, lead +181 to +159."},
  {"ply": 28, "side": "red", "verdict": "bad", "note": "Spent 20 of 22 gold on a level 2 that reconnected nothing (cut_off stayed 4); income 16 to 13."},
  {"ply": 30, "side": "red", "verdict": "bad", "note": "WAIT with 10 gold while blue's income was 51 and its hq_gap 21; no blocker placed near the HQ."},
  {"ply": 1, "side": "blue", "verdict": "good", "note": "Two level 1 units on its own corner (20 gold): cells 2 to 4, material 36 to 44."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Trainings on (6,3) and (6,4) cut red's two units off; lead -3 to +54."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Three level 1 trainings across red's corridor at (4,3), (3,3), (2,3): red cells 31 to 17, cut_off 10; lead +58 to +209."},
  {"ply": 25, "side": "blue", "verdict": "good", "note": "Level 1 trainings and moves cut red from 28 to 15 cells; lead +159 to +292."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Four level 1 trainings near red's HQ: red cells 15 to 10, cut_off 4 to 1; lead +356 to +448."},
  {"ply": 31, "side": "blue", "verdict": "good", "note": "Three level 1 trainings at (1,1), (1,0), (0,0) reached red's HQ and ended the game."}
]
```
