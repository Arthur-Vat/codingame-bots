# Review of red-ranking-921365300

v022 (red) lost to zasmu (blue), which took red's HQ with `TRAIN 1 0 0` at ply 47, in 24 turns.

## Why the game was won

zasmu built four mines (plies 17, 19, 27 and so on) and kept its army at level 1, so its upkeep stayed at 13 while its gross income reached 65 (ply 47). v022 built no mine in 24 turns, although it had mine spots next to its HQ from turn 1 and 4 free spots by ply 12. Its two level 3 trainings (plies 18 and 28) cost 20 upkeep each turn, which cut its net income from 22 to 5 at ply 19 and from 21 to 4 at ply 29. Its material ended at 320 against blue's 789.

## Early game

Plies 0 to 11.

- **Red, good:** ply 0, two level 1 units for 20 gold, the same as blue. Neutral.
- **Red, bad:** ply 8 and ply 10 spent 0 with 7 and 17 gold. At ply 10 blue had 15 active cells to red's 13, and it kept its lead through the next turn (ply 12: 20 against 16).
- **Blue, good:** ply 9 and ply 11 each spent 10 on units while it expanded down the east side.

## Middle game

Plies 12 to 32.

- **Red, bad:** ply 12, 30 gold, income 13, `free_spots` 4, mines 0. It trained one level 1 unit and built nothing. Its four free mine spots (including (1,0) and (2,1) beside its HQ) stayed empty for the rest of the game.
- **Red, bad:** ply 14, 36 gold kept with nothing spent. Ply 16, 55 gold: it built a tower at (4,4) (15) instead of a mine. Blue had 30 cells to red's 23 at ply 16.
- **Red, bad:** ply 18, a level 3 train at (4,6) for 30 gold, plus a level 1 unit. The unit captured one cell and cut two blue cells, but upkeep went from 4 to 25 at ply 19, income from 22 to 5, and material fell from 282 to 247 while blue's rose from 326 to 337. The lead moved 44 to blue at ply 20 (352 against 252 for red, a gap of 100).
- **Red, bad:** ply 28, a second level 3 for 30 gold. Upkeep went to 30 at ply 29, and income fell from 21 to 4.
- **Blue, good:** ply 13, a tower at (6,5) on the frontier. Cells went from 20 to 25 by ply 14, while red stayed at 20.
- **Blue, good:** ply 17, a mine at (10,11) and a tower at (5,6) for 30 gold. Income rose from 24 to 33 at ply 18.
- **Blue, good:** ply 19, mines at (9,4) and (6,8). Blue had three mines at ply 20 and income 43, against red's 5.
- **Blue, bad:** ply 21, a level 3 train for 30 gold plus level 1 units, spending 50 of 51 gold. Upkeep went to 27 at ply 22, and the lead fell from 102 to 70 by ply 23. Blue recovered by ply 24, so the level 3 cost it only a turn.

## Endgame

Plies 33 to 47.

- **Blue, good:** ply 33, 130 gold for nine level 1 units and two towers. Its army went from 50 to 150 and the lead rose from 203 to 263 by ply 34.
- **Red, bad:** ply 34, a level 3 trained onto blue's tower at (5,5) for 30 gold. Blue fell to 49 cells with 4 cut off, but spent 55 at ply 35 on three towers and reconnected them by ply 36. The lead rose 59 to blue.
- **Red, good:** ply 36, its units moved to (5,5), (5,6) and (5,4), cutting 5 blue cells at ply 37 (blue 48 cells). The lead fell from 322 to 258 by ply 38.
- **Red, bad:** ply 38, its level 3 moved alone to (5,7) inside blue's territory. Blue retook (5,6) and (6,6) with 35 gold at ply 39, and red's level 3 was cut and lost (cum_lost 70 to 100). The lead rose 72 to blue by ply 40.
- **Blue, good:** ply 39, the retake above. The lead rose from 285 to 330 by ply 40.
- Blue spent 0 at ply 37 with 54 gold while 5 cells were cut off. Its only level 3 was none, so its 12 level 1 units could not enter (5,6), which red's level 3 held. The lead still rose 21 by ply 39, so I did not judge it.
- Blue then waited at plies 43 and 45, and at ply 47 it spent 180 on a chain of trainings through (3,3), (3,1) and (2,0) and onto (0,0).

## Decisive moments

- **Ply 38 (rules: swing for red):** Real, but temporary. Red's cut at ply 36 was the best play of the endgame, and red's ply 38 move undid it by exposing its level 3.
- **Ply 40 (rules: swing for blue):** Blue's ply 39 retake and red's loss of its level 3 explain it. The lead was already lost by ply 20.
- **Ply 45 (rules: blue's HQ threat):** Accurate. Blue could pay the chain (hq_gap 0 with 195 gold), and it won at ply 47.
- **Ply 16 (reviewer's pick):** The tower instead of a mine, with 55 gold in hand, is where the income gap started. Blue had 4 mines by ply 36 and red had none.
- **Ply 18 (reviewer's pick):** The first level 3 train. It is the largest single swing in the middle game (material 282 to 247 for red, 326 to 337 for blue), and the upkeep lasted until ply 25.

## Lessons

Red (v022):
- Build a mine on each free spot next to the HQ as soon as gold allows (ply 12 had 30 gold and 4 spots; four mines cost 60 and return 16 a turn).
- Do not train level 3 as a raider. Each costs 30 gold and 20 upkeep every turn, which outweighs the cells it cuts; keep it for a chokepoint that only it can hold, and check upkeep before buying.
- Do not keep 36 to 62 gold idle. Spend it on mines first, then towers at the middle line.

Blue (zasmu) for comparison:
- Keep the army at level 1 (upkeep 1 each) and let mines carry the income.
- Re-link cut cells at once with towers and level 1 units (ply 35), and retake vacated cells straight away (ply 39).

```json
[
  {"ply": 10, "side": "red", "verdict": "bad", "note": "Spent 0 with 17 gold while blue led 15 cells to 13; blue went to 20 cells against 16 by ply 12."},
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Held 30 gold and 4 free mine spots, trained one level 1 unit and built no mine; income stayed at 13."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Built a tower at (4,4) with 55 gold in hand instead of a mine; blue went to 30 cells against 23."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Level 3 train for 30 gold: upkeep 4 to 25, income 22 to 5, material 282 to 247; blue led by 100 at ply 20."},
  {"ply": 34, "side": "red", "verdict": "bad", "note": "Level 3 on blue's tower at (5,5) cut 4 blue cells; blue reconnected them with 3 towers at ply 35, and the lead rose 59 to blue."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Moved units onto (5,5), (5,6) and (5,4), cutting 5 blue cells at ply 37; the lead fell from 322 to 258 by ply 38."},
  {"ply": 38, "side": "red", "verdict": "bad", "note": "Sent its lone level 3 to (5,7) alone; blue retook (5,6) and (6,6), red's level 3 was cut and lost, and the lead rose 72 to blue."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Tower at (6,5) on the frontier while red stayed at 20 cells; blue went to 25."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Mine at (10,11) and tower at (5,6) for 30 gold; income 24 to 33 and the lead rose from 39 to 90."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Two mines, (9,4) and (6,8); blue had 3 mines and income 43 at ply 20, against red's 5."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Level 3 train plus level 1 units, spending 50 of 51 gold; upkeep 5 to 27 and the lead fell from 102 to 70."},
  {"ply": 39, "side": "blue", "verdict": "good", "note": "Retook (5,6) and (6,6) with two level 1 trains and a tower (35 gold); the lead rose from 285 to 330 by ply 40."}
]
```
