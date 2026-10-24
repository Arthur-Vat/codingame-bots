# Review of ice-and-fire-151-p3-0

ice-and-fire-v004 (red) beat ice-and-fire-v003 (blue) in 12 turns (23 plies), by taking blue's HQ. The file ends at ply 22 without a final board, so the HQ capture is inferred from red's three trainings running down column 11 to (11,11) and from hq_gap 0 at ply 22.

## Why the game was won

Red won at ply 20. Its unit stepped onto blue's mine at (7,9) and three level 1 trainings at (8,9), (9,9) and (10,9) closed row 9. Blue's cells went from 39 to 5 (cut_off 1 to 31), its 7 units died, and its material fell from 356 to 48, so red's lead went from -96 to +266 at ply 21. Blue had the middle game until then: at ply 17 it cut six red cells, moving the lead from -6 to -93 by ply 18, because red had spent 50 of its 54 gold at ply 16 and left the corridor at (5,1), (6,1) and (5,2) without units. The early game was level (material 206 red, 207 blue at ply 14).

## Early game

Both sides trained two level 1 units on turn 1 (red at (1,0) and (1,1), blue at (10,10) and (10,9)). The openings differ in mine timing: red built a mine at ply 10 and another at ply 12, blue's first came at ply 13. Red's income was 17 against 11 at ply 11 and 24 against 15 at ply 13. Blue held more territory (voronoi 62 against 51-52 at plies 10, 12 and 14). The income edge did not become a lead by turn 8, so this opening difference did not decide the game in this file.

Red, good: ply 12, BUILD MINE 3 0 with three moves. Cells rose from 16 to 19 and the lead went from -10 (ply 12) to -1 (ply 14).

Red, bad: ply 14, three moves with 28 gold unspent. Nothing held the east link at (7,2) and (8,2), next to unit 5 on (8,1). At ply 15 blue trained three units on (8,2), (7,2) and (7,1), which cut unit 5 off and killed it. Red's lead went from -1 to -30 by ply 16.

Blue, good: ply 9, TRAIN 1 9 4 and three moves took cells from 11 to 15, lead -9 to -1 at ply 11. Ply 11, four moves with no spending, cells 15 to 19, lead -1 to +7 at ply 13.

## Middle game

Blue, good: ply 15, the three trainings above. Blue's cells rose from 22 to 28 and its material from 229 to 262. Its lead at its next turn rose only 4, because red answered at ply 16, but the cut was real.

Red, bad: ply 16, TRAIN 2 7 2 retook (7,2), and the trainings at (3,7), (3,8) and (4,8) grew red's south-west, so red's cells rose from 20 to 26 and its lead from -30 to -6 at ply 17. But the play spent 50 of 54 gold and MOVE 1 6 3 took unit 1 off (6,2) into the dead end at (6,3). Red had 4 gold left and no unit on (5,1), (6,1) or (5,2).

Blue, good: ply 17, TRAIN 1 6 1, TRAIN 1 5 1 and TRAIN 1 5 2 took the empty corridor cells. Red's cells fell from 26 to 20, cut_off rose to 3, its army from 70 to 40. Blue's cells rose from 26 to 35 and its lead from +6 to +50 by ply 19.

Red, ply 18, recovered six cells: TRAIN 1 5 9 and TRAIN 1 6 9 took two blue cells, and red's cells rose from 20 to 26 while blue's fell from 35 to 31. I give no verdict on it: red's lead was -93 at ply 18 and -96 at ply 20, because blue's ply 19 raid took back eight cells. The two units at (5,9) and (6,9) were the base for the row 9 chain at ply 20.

Blue, bad: ply 19, TRAIN 1 4 1 and TRAIN 1 3 1, plus three moves into red's north side ((4,2), (5,0), (6,0)). This took eight cells (31 to 39) and lifted blue's lead to +96 at ply 20. But it spent 20 of its 28 gold on that raid and left (9,9) empty, the gap in its own row 9 line. Its lead at its next turn was -266.

## Endgame

Red, good: ply 20, MOVE 19 7 9 took the mine at (7,9), and TRAIN 1 8 9, TRAIN 1 9 9 and TRAIN 1 10 9 closed the row. Blue fell from 39 to 5 active cells. Red's lead went from -96 to +266 at ply 21 and to +297 at ply 22.

Blue, ply 21: WAIT with 13 gold and no units. A level 1 cannot enter a level 1 (a unit beats a lower level only), so nothing blue could buy reconnected its cells. The file does not test one more option: a level 1 on (11,10) might have pushed red's chain to the HQ above its 34 gold. That is my inference from hq_gap 27 at ply 21 and is not checked by the file.

Red, ply 22: three level 1 trainings (30 of 34 gold) along column 11 to blue's HQ. No next turn, so no verdict.

## Decisive moments

- Ply 18, swing toward blue (rule): correct as a swing (lead -30 at ply 16 to -93 at ply 18), but its cause is ply 16 (red) and ply 17 (blue).
- Ply 20, large cut toward red (rule): correct and decisive.
- Ply 22, HQ threat toward red (rule): the finishing move, possible only because ply 20 left blue with no units near its HQ.
- Ply 22, lead change toward red (rule): late. The lead changed hands at ply 20 and showed at ply 21 (+266).
- Ply 15, blue cut (reviewer's pick): the rules missed it. It killed red's unit 5 and took six cells, and started the sequence that ended with ply 17.
- Ply 19, blue's (9,9) gap (reviewer's pick): the rules missed it. One unit on (9,9) would have stopped red's chain from (8,9) to (10,9), since equal levels cannot enter each other's cells.

## Lessons

Red:
1. Do not bank gold while an enemy can train next to an open corridor. Red held 28 gold at ply 14 and then spent 50 at ply 16 in one burst, leaving no reserve for the reply.
2. Keep a unit on each corridor cell the enemy can reach, and do not move the last defender into a dead end.
3. Build a southern wall early. The units placed at (5,9) and (6,9) made the row 9 chain at ply 20 possible.

Blue:
1. Close gaps in your own line before raiding. The hole at (9,9) cost 34 cells in one turn.
2. Keep training on empty corridor cells next to your own, as at plies 15 and 17, when the opponent's corridor has no units.
3. Keep at least one level 1 near the HQ when your network is at risk, so that the opponent's chain costs more.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "TRAIN 1 9 4 and three moves took cells 11 to 15; lead -9 to -1 by ply 11."},
  {"ply": 11, "side": "blue", "verdict": "good", "note": "Four moves, no spending, cells 15 to 19; lead -1 to +7 by ply 13."},
  {"ply": 12, "side": "red", "verdict": "good", "note": "BUILD MINE 3 0 plus moves; cells 16 to 19; lead -10 to -1 by ply 14."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Kept 28 gold and moved only; east link (7,2)/(8,2) open; blue cut and killed unit 5 at ply 15; lead -1 to -30 by ply 16."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Trainings at (8,2), (7,2), (7,1) cut red's unit 5 off and took cells 22 to 28."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Spent 50 of 54 gold; unit 1 left (6,2) for dead end (6,3); corridor (5,1), (6,1), (5,2) ungarrisoned; lead -30 to -93 by ply 18."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Trainings at (6,1), (5,1), (5,2) cut six red cells; blue's lead +6 to +50 by ply 19."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Raid north gained 8 cells but spent 20 of 28 gold away from the line and left (9,9) open; red's row 9 chain at ply 20 cut 34 cells; blue lead +50 to -266 by ply 21."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "MOVE 19 7 9 took the mine, trainings at (8,9), (9,9), (10,9) cut blue from 39 to 5 cells; lead -96 to +266 at ply 21."}
]
```
