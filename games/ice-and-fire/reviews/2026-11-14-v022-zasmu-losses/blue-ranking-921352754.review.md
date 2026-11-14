# Review of blue-ranking-921352754 (zasmu red, v022 blue)

Winner: zasmu, as red, by taking blue's HQ on turn 20 (ply 38), the only win before turn 100 under the file's rules; 39 plies in all.

## Why the game was won

Red's economy pulled ahead from turn 10: it built mines at plies 20 and 22 while blue built one at ply 21 and its second at ply 23, so red's income was 41 against 27 at ply 22 and 46 against 34 at ply 24. Blue's forward cut at ply 15 gave it a +84 lead at ply 16 that red reversed by ply 18 (+45 for red). The game was decided at ply 28: blue had 79 gold and spent none (kept_share 100) after moving its units off the corridor (7,7) to (9,8), so red's nine trainings for 140 gold cut blue from 36 active cells to 4 at ply 29.

## Early game

Both sides expanded at the same pace. Blue was slightly ahead: at ply 10 it had 15 cells and 11 income to red's 13 and 10 (material 135 against 133), and at ply 12 19 cells against 17. Blue spent 10 or 20 a turn and moved units forward steadily; no play is judged bad here. Red's good play was ply 12: two level 1 trainings at (5,6) and (6,6) with four moves, which turned a 7-point deficit into a 14-point lead by ply 13 (material 167 and 174 at ply 12, 203 and 189 at ply 13).

## Middle game

- Blue, ply 15 (bad): three units cut five red cells (cum_killed_cut 30 at ply 16), and blue led by 84. But the cut group at (4,4) to (6,4) was fed only through (6,5) and (7,5). Red's two level 1 trainings at ply 16 retook those cells and cut the group off (cum_lost 30 for blue at ply 17). The lead was red +26 by ply 17.
- Red, ply 16 (good): those two trainings reversed the swing. Lead from -84 to +26.
- Red, ply 18 (good): a tower on (6,5) and a level 1 at (5,7). Blue had only level 1 and 2 units, which cannot enter a tower cell, so the centre held. Red led by 45 at ply 18 and 74 at ply 20.
- Blue, ply 19 (bad): 46 gold, nothing spent (kept_share 100) while red's tower closed the centre. The lead fell from -52 to -75 at ply 21.
- Red, ply 20 (good): two mines (1,0) and (4,1) for 30 gold. Income rose from 29 to 41 at ply 21.
- Blue, ply 21 (bad): 55 gold on two towers at (8,6) and (6,8), one mine at (10,11) and one level 1. Towers earn nothing; the lead fell from -75 to -96 at ply 23.

## Endgame

- Red, ply 28 (good, the decisive play): nine trainings for 140 gold (two level 3, one level 2, six level 1) took (7,7), (8,7), (8,8), (9,8), (9,9), (10,9), (9,10) and (9,11). Blue fell from 36 cells and material 427 to 4 cells and 137 at ply 29. Its upkeep rose from 8 to 49 by ply 30, so income dropped to 11 at ply 30. The lead fell back from +351 to +116 at ply 30 only because blue retook cells, so the judgment rests on the later lead of +256 at ply 36.
- Blue, ply 27 (bad): 79 gold kept and no training. Its moves 18, 4 and 3 left the corridor empty. At ply 28 blue had one joint, empty_joints 1 and guarded_joints 0, five inner units and one frontier unit. Blue's metrics said it could cut back little (exposed50 2, can_cut 4). Those metrics count at most five trainings and 50 gold, while red spent 140 on nine trainings.
- Blue, ply 29 (good, too late): three level 2 trainings at (10,9), (9,9) and (8,8) with a tower at (10,8). Cells rose from 4 to 30 by ply 30, material from 137 to 298. It came one turn after the gold had sat idle.
- Red, ply 38 (good): with 103 gold and hq_gap 7, red spent 90 on a chain of trainings that ended at blue's HQ (11,11). The file shows no board after ply 38, so the final move is not visible.

## Decisive moments

- **Ply 16 (blue swing, +86):** not a lasting gain. Red's two trainings at ply 16 reversed it by ply 17 (lead red +26).
- **Ply 18 (red lead +45):** it matters. Red's tower at (6,5) closed the centre for the rest of the game.
- **Ply 28 (red took 36 cells to 4):** decisive. The cause was blue's idle gold and empty joints at ply 27.
- **Ply 36 (lead moved +102 to red):** a result, not a cause. Blue's cut at ply 34 to 35 killed about 100 gold of red units (blue's exchange went from -40 to +60) but red's cells kept the lead at +256.
- **Reviewer's pick, ply 21:** blue's 55 gold went to two towers and a unit while red's mines were lifting its income to 41 against 21. The mine spots were the economic race, and blue lost it here.

## Lessons

Blue (v022):
1. Do not sit on gold. Plies 19 and 27 kept 46 and 79 gold unspent. Spend it on units that hold empty joints, such as (7,7), (8,7) and (9,8), before the enemy trains into them.
2. Take economy before towers while the front is quiet. Blue's two towers at ply 21 gave no income, and red's income lead grew by 14 a turn.
3. Check the enemy's cheapest retake before a cut. The ply 15 cut hung on the single link (6,5) and (7,5), which red retook at once.
4. Count the whole gold in the threat check. Nine trainings at 140 gold is beyond the five-training, 50-gold horizon of the exposure metrics.

Red (zasmu), for v022 to copy:
1. Build mines on home spots as soon as they are free (plies 20 and 22).
2. Put a tower on a contested link, here (6,5), to hold the centre against level 1 and 2 units.
3. Hoard for one large strike on the thinnest joint, as at plies 24 to 27, then pay for it.

```json
[
  {"ply": 12, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (5,6) and (6,6) with four moves turned a 7-point deficit into a 14-point lead by ply 13."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 15 on a tower at (5,6) and left a pocket of cells on a thin link; blue cut five of them at ply 15 and red's lead went from +2 to -84."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "Cut five red cells with three units, but the group was fed only through (6,5) and (7,5); red retook them at ply 16 and blue lost 30 gold of units, so the lead fell from -7 to -26."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (6,5) and (7,5) retook the line and cut blue's group; the lead went from -84 to +26."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Tower at (6,5) and a level 1 at (5,7); blue had no level 3 to break it, and red led by 45 then 74."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Kept all 46 gold (kept_share 100) while red's tower closed the centre; the lead fell from -52 to -75."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Two mines for 30 gold lifted income from 29 to 41 at ply 21; blue had built none."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Spent 55 on two towers, one mine and one level 1 while red's income was 41 against 21; the lead fell from -75 to -96."},
  {"ply": 27, "side": "blue", "verdict": "bad", "note": "Kept 79 gold (kept_share 100) and moved units off the corridor, leaving empty joints; red cut 28 cells at ply 28 with 140 gold."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Nine trainings for 140 gold cut blue from 36 cells to 4 (material 427 to 137); upkeep then cost red 41 a turn, and blue retook part of it, but the lead stayed at +116 or more."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Three level 2 trainings at (10,9), (9,9) and (8,8) plus a tower retook the line: cells 4 to 30, material 137 to 298 by ply 30."}
]
```
