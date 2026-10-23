# Review of ice-and-fire-151-p9-0

Blue (v003) won as the blue seat against red (v004), in 21 turns (42 plies), when the rules ended the game at ply 41. The file's last commands are blue's TRAIN 3 1 0 and TRAIN 1 0 0 against red's HQ at (0,0), and blue's hq_gap was 0 at ply 41, so the finish was most likely an HQ capture. The file shows no final board, so this is inferred.

Lead below means red's material minus blue's. "Next ply" means the same side's next ply, two plies later.

## Why the game was won

Red led at ply 33 (+9) after its ply 32 trainings. At ply 32 red spent 30 of its 33 gold on three level 1 units east of an empty column, (6,1) to (6,3), and posted no unit in that column. At ply 33 blue trained three level 1 units into the column, and red's cut-off cells went from 0 to 8, its cells from 34 to 20, its units from 7 to 3 and its income from 31 to 21. Blue kept 34 gold and grew its land, and the lead went from +9 to -157 in one ply and to -535 by ply 41.

## Early game

Both openings were near mirror images: 20 gold on two level 1 units on turn 1 (red at ply 0, blue at ply 1). The lead stayed within about 10 points through ply 13, so the file gives no sign that the v004 opening gave red an edge, and the file does not contain v003's opening, so the batch's opening question cannot be settled from this game.

Red's good play: ply 10, BUILD MINE 1 0 beside its HQ with three moves. Red's mines went from 0 to 1 and its income from 10 to 17 at ply 11, and the lead went from -6 to +1. Blue never built its mine at (10,11) beside its HQ; that spot stayed empty to the end, so blue earned 4 less a turn all game and still won.

Red's bad play: ply 12, no spending (19 gold kept) and three moves only. Blue's ply 13 trained one unit and moved four, cells went from 19 to 24, and the lead fell from -10 (ply 12) to -16 (ply 14).

Blue's good play: ply 9, three moves and TRAIN 1 9 4. Blue's cells went from 11 to 15 by ply 11 and the lead from +9 to +1.

## Middle game

Both sides cut each other's corridors almost every turn; cut-off cells reached 5 or 6. Red led on most turns (+17 at ply 18, +86 at ply 21, +83 at ply 23, +48 at ply 25), and blue's replies took much of that back each time.

Red's good play: ply 16, TRAIN 2 6 4 and three moves. The lead went from -8 to +17 at ply 18.

Red's bad play: ply 18, TRAIN 1 8 3 and moves. Its unit at (8,3) sat behind a single link, (6,3). At ply 19 blue trained a level 1 unit into (6,3), cut red's units at (7,3) and (8,3), and the lead fell from +17 to -7 by ply 20 (cells 32 to 28, cut-off 0 to 2).

Blue's bad play: ply 17, one move and no spending with 25 gold. Red's ply 18 took five cells (27 to 32), and the lead went from +4 to +35 at ply 19. Blue spent 40 at ply 19 and cut red back, but the hoard cost a turn.

Blue's good play: ply 29, MOVE 31, TRAIN 1 4 6 and TRAIN 1 4 4. Red's cut-off went from 1 to 4 and its cells from 32 to 26 by ply 30, and the lead went from +41 to -3 at ply 31.

Red's ply 20 counter (TRAIN 2 7 2 and three moves) restored cells (28 to 35), but blue's ply 21 cut them again, so the middle game stayed a series of trades.

## Endgame

Red's good play: ply 34, TRAIN 2 6 3. A level 2 unit beat blue's level 1 unit on (6,3), reconnecting five cells (cut-off 8 to 4, cells 20 to 25) and lifting the lead from -157 to -102. This was the best reply on the board, but blue's ply 35 TRAIN 2 5 3 re-cut the same link (cut-off 7 at ply 36), so the gain did not hold.

Blue's good play: ply 35, TRAIN 2 5 3 and TRAIN 1 5 2. The lead went from -102 to -223 by ply 37.

Red's bad play: ply 36, TRAIN 1 3 4 and TRAIN 1 2 4 (20 gold) in its corner. Units went from 2 at ply 36 to 4 at ply 37 and to 1 at ply 38, so the new units died, and the lead went from -223 to -338. Red's tower at (1,1) at ply 38 (15 gold) made the mine at (1,0) level 3 only, and blue paid TRAIN 3 1 0 at ply 41 (30 gold) to enter it.

## Decisive moments

- Ply 33 (rules: large cut, blue): valid and the decisive play, as described above.
- Ply 34 (rules: swing): a consequence of ply 33, partly reversed by red's ply 34 at ply 35.
- Ply 35 (rules: large cut, blue): valid. The level 2 re-cut kept red's pocket cut off and set the endgame.
- Plies 36, 37, 38, 39, 40 and 41 (rules): the collapse after ply 33, confirming rather than deciding the game.
- Ply 19 (reviewer's pick): the same single-link pattern at turn 10. Blue's TRAIN 1 6 3 cut red's east pocket and the lead went from +17 to -7. Red's ply 20 counter restored much of it, but it was the first warning of ply 33.
- Ply 34 (reviewer's pick): red's TRAIN 2 6 3 was the one reconnecting play; the rules did not flag it.

## Lessons

Red (v004):
1. Before spending on units beyond a link, check that the link is held. Units on the far side of a one-cell link (6,3), or of the empty column (6,1) to (6,3), died when blue trained into it (plies 18 and 32).
2. Keep gold for a counter-cut rather than spending it all on the front. Red's exposed50 rose from 5 (ply 31) to 30 (ply 33) while its can_cut fell to 0, so its land was cuttable and it could not cut back.
3. Build the HQ-side tower earlier. Red's tower at (1,1) came at ply 38, after the corner had been cut.

Blue (v003):
1. Spend when the cut is on. The hoard at ply 17 gave red a free turn of expansion.
2. Build the free mine at (10,11) beside its HQ; it was left empty for 41 plies.
3. Keep attacking the single links into red's territory (plies 19, 31, 33 and 35 all did), which decided the game.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Three moves and TRAIN 1 9 4; cells 11 to 15 by ply 11, lead from +9 to +1."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "BUILD MINE 1 0 beside the HQ; income 10 to 17 at ply 11, lead from -6 to +1."},
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Spent nothing with 19 gold and three moves; blue's ply 13 gained five cells, lead from -10 to -16 by ply 14."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "TRAIN 2 6 4 and three moves; lead from -8 to +17 at ply 18."},
  {"ply": 17, "side": "blue", "verdict": "bad", "note": "One move, 25 gold kept; red took five cells at ply 18 and the lead went from +4 to +35 at ply 19."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "TRAIN 1 8 3 behind the single link (6,3); blue cut it at ply 19, lead from +17 to -7 at ply 20."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "MOVE 31 and two level 1 trainings cut red from 32 to 26 cells; lead from +41 to -3 at ply 31."},
  {"ply": 32, "side": "red", "verdict": "bad", "note": "Spent 30 on three units east of the empty column (6,1) to (6,3); blue cut eight cells at ply 33, lead from -67 to -157 at ply 34."},
  {"ply": 33, "side": "blue", "verdict": "good", "note": "TRAIN 1 6 3, 6 2, 6 1 cut red's pocket: cells 34 to 20, cut-off 0 to 8, lead from +9 to -102 at ply 35."},
  {"ply": 34, "side": "red", "verdict": "good", "note": "TRAIN 2 6 3 reconnected five cells (cut-off 8 to 4), lead from -157 to -102; blue re-cut it at ply 35, so the gain did not hold."},
  {"ply": 35, "side": "blue", "verdict": "good", "note": "TRAIN 2 5 3 re-cut red's link; cut-off back to 7, lead from -102 to -223 by ply 37."},
  {"ply": 36, "side": "red", "verdict": "bad", "note": "Two level 1 trainings in its corner; units went from 4 to 1 by ply 38, lead from -223 to -338."}
]
```
