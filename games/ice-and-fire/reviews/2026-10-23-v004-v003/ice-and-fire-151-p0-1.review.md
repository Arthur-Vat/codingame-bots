# Review of ice-and-fire-151-p0-1

Red v003 won as red, in 22 turns (43 plies), by taking blue's HQ. Blue v004 lost.

The file has no explicit "HQ taken" line. The game ended at turn 22, before the 100-turn limit, and blue never went into debt, so the HQ capture is the only ending that fits. The last board shown is the start of ply 42, so the capture itself is not shown.

## Why the game was won

Red's ply 36 trainings at (8,6), (9,6) and (10,6) cut 17 blue cells off its HQ: blue's active cells fell from 33 to 13 and its six units died (army 60 to 0 by ply 37). Red's material lead went from +30 at ply 36 to +325 at ply 38. Red had two mines from ply 12 and blue had none, so red's income led 24 to 15 at ply 13 and 42 to 24 at ply 29. Red also answered each earlier cut (ply 16, 28, 30) before it could stick.

## Early game

Both sides spent 20 gold on two level 1 units at plies 0 and 1, and expanded at about the same pace (19 cells each at ply 13). The material lead stayed within 11 either way. The only opening difference that matters is mine timing. The file does not show v004's opening weights, so it cannot say why blue built no mine.

- Red, good: ply 10, BUILD MINE 1 0. Income rose from 10 (ply 10) to 17 (ply 11), and ply 12 added a second mine (3,3), income 24 by ply 13. The one-turn lead read -4 at ply 12, because blue's ply 11 moves gained 4 cells. The income is the lasting gain.
- Blue, bad: ply 13, spent 0 with 28 gold. Its mine spot (8,8) sat beside its active cells and no mine was built all game. Its income was 15 against red's 24. The one-turn lead moved only +2, so the judgment rests on the income gap, not on the next turn.

## Middle game

- Blue, good: ply 15, four level 1 trainings (40 gold) at (6,3), (5,3), (5,2) and (5,1). Red's north-east group was cut: red cut_off 0 to 3, units 3 to 1, army 30 to 10, and blue's lead went from +9 to +86 at ply 16.
- Red, bad: ply 14, spent 0 with 28 gold while (5,2) and (5,3) stayed open. That let ply 15 happen.
- Red, good: ply 16, two level 2 trainings (50 gold) at (5,1) and (7,3). Level 2 beats level 1, so they retook (5,1) and cut blue's (5,2), (5,3), (6,3). Blue's lead fell from +86 to +17 at ply 17, and red's lead was -42 at ply 18.
- Blue, bad: ply 17, spent 30 on a level 2 and a level 1 and moves. Its three cut cells stayed cut (cut_off 3 at ply 18 and 19). Red's ply 18 (20 gold) restored the balance, and blue's lead fell from +17 to +2 at ply 19.
- Blue, good: ply 27, three level 1 trainings at (4,3), (4,2) and (4,1). Red's east block was cut: cells 38 to 15, cut_off 19 at ply 28, units 7 to 1. The swing was reversed at ply 28.

## Endgame

- Red, good: ply 28, two level 1 trainings at (4,5) and (4,6) took back the cut column. Red's cells went from 15 to 37 by ply 29, and its lead moved from -103 to -72 at ply 30.
- Blue, good: ply 29, three trainings at (6,5), (6,4) and (7,2) cut red's east again: red cells 37 to 19, cut_off 14 at ply 30. Blue's lead rose from -98 to -76 at ply 31.
- Red, good: ply 30, trainings at (4,1) and (5,4), plus moves. Cut_off fell from 14 to 2 and cells rose from 19 to 36. Red led +76 at ply 31.
- Blue, bad: ply 35, two level 1 trainings at (5,4) and (6,4), away from the row 6 link to its HQ. Red's ply 36 trainings cut that link (see the top). Blue's lead went from -128 to -281 at ply 37.
- Red, good: ply 36, the decisive cut (see the top).
- Blue, ply 37, BUILD TOWER 11 10 (15 gold) was its only real defence. It made the last step to the HQ a level 3 entry, which red paid at ply 42. It came too late to change the board.
- Red, ply 42, TRAIN 1 10 11 and TRAIN 3 11 11 (40 gold), with hq_gap 0 at the start of the ply: the winning step.

## Decisive moments

- Ply 16 (swing, blue): mattered. It was blue's ply 15 cut, and red's ply 16 recovered most of it.
- Ply 27 (large cut, blue): mattered for one turn only; reversed by ply 28.
- Ply 28 (swing, favours blue): the figure is blue's ply 27 cut. Red's ply 28 reconnection is what mattered.
- Ply 29 (large cut, blue): mattered; red's ply 30 reversed it.
- Ply 32 (lead change, red): not decisive. Red led by only 30, and the lead was still open until ply 36.
- Ply 36 (large cut, red): decisive.
- Ply 38 (large cut and swing, red): follow-through; the game was set.
- Ply 40 (swing, red): follow-through.
- Ply 42 (HQ threat, red): the end.
- Reviewer's pick, ply 14 (red): spent 0 with 28 gold while the corridor was open. A level 1 unit on (5,2) would have blocked blue's level 1 trainings there, by the rule that a unit beats a lower level only.
- Reviewer's pick, ply 13 (blue): 28 gold unspent with a mine spot in reach. This is the income gap the rules did not flag.

## Lessons

Blue (v004):
1. Build the mine once gold reaches 15 (ply 13 had 28 and a spot at (8,8)). Blue never built one, and red's income led all game.
2. Guard the row 6 link to the HQ before spending on the west. The ply 35 units went west and the link was cut at ply 36.
3. When a cut is pending, spend on a level 2 or on cell reconnection. The ply 17 cells stayed cut.

Red (v003):
1. Keep gold for a defensive level 1 on an open corridor cell (ply 14).
2. Keep adding mines; red stayed at two from ply 12 to 42, with income 24 to 51 by the end.
3. Keep units out of lanes blue can cut. At ply 23 red lost 3 of 6 units (army 80 to 40).

```json
[
  {"ply": 10, "side": "red", "verdict": "good", "note": "BUILD MINE 1 0: income 10 to 17 by ply 11. The one-turn lead read -4 at ply 12 because blue's ply 11 gained 4 cells; the income is the lasting gain."},
  {"ply": 13, "side": "blue", "verdict": "bad", "note": "Spent 0 with 28 gold and a mine spot at (8,8); no mine all game. The one-turn lead moved only +2; the judgment rests on the income gap (15 vs 24)."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 0 with 28 gold while (5,2) and (5,3) stayed open; blue cut 3 cells and 2 red units at ply 15, lead -11 to -86."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Four level 1 trainings cut red's north-east group (cut_off 3, units 3 to 1); lead +9 to +17 by blue's next turn."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Two level 2 trainings retook (5,1) and cut blue's three cells; lead -86 to -42 by red's next turn."},
  {"ply": 17, "side": "blue", "verdict": "bad", "note": "Left its three cut cells cut; red's ply 18 restored the balance; lead +17 to +2 by blue's next turn."},
  {"ply": 27, "side": "blue", "verdict": "good", "note": "Three trainings at (4,1) to (4,3) cut red's east block (cells 38 to 15); red undid it at ply 28, but the lead still moved +8 for blue by ply 29."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (4,5) and (4,6) reconnected 22 cells; lead -103 to -72 by red's next turn."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Three trainings re-cut red's east block (cells 37 to 19, cut_off 14); lead -98 to -76 by blue's next turn."},
  {"ply": 30, "side": "red", "verdict": "good", "note": "Trainings at (4,1) and (5,4) reconnected cells (cut_off 14 to 2); lead -72 to +30 by red's next turn."},
  {"ply": 35, "side": "blue", "verdict": "bad", "note": "Two trainings went west to (5,4) and (6,4), away from the row 6 link; red cut 17 blue cells at ply 36 and blue's army fell 60 to 0; lead -128 to -281."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Three trainings at (8,6), (9,6), (10,6) cut 17 blue cells; lead +30 to +325 by red's next turn."}
]
```
