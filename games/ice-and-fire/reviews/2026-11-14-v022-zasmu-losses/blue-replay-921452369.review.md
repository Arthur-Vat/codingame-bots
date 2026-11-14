# Review of blue-replay-921452369: zasmu (red) against v022 (blue)

zasmu won as red in 16 turns (31 plies). The file says only that the rules ended the game; the ply 30 chain ends on blue's HQ cell (11, 11), so the HQ most likely fell. The final board is not shown.

## Why the game was won

Blue hoarded while red spent every turn. Blue had two free mine spots from ply 8 to ply 22 (free_spots 2) and built no mine until ply 23, while its gold climbed from 13 (ply 12) to 108 (ply 23) and its cells stayed at 24 to 27. Red went from 17 cells and income 13 (ply 12) to 47 cells and income 51 (ply 23), with three mines and six towers. Blue's one big move, the 90-gold strike of ply 23 (two level 3 units and two mines), cut 5 red cells, but two level 3 units carry 40 upkeep: blue's income fell from 22 (ply 22) to -6 (ply 24) with 18 gold left. Red kept its income at 46 to 48, banked gold to 187 by ply 30, and paid a 170-gold chain of trainings to the HQ.

## Early game

Both sides expanded at a similar pace (cells 8 blue, 10 red at ply 7; 15 blue, 13 red at ply 10). Blue held 19 cells to red's 17 at ply 12.

- Red, good (ply 12): spent 20 on two level 1 units at (6, 5) and (7, 5) and moved others out; cells rose 17 to 23 by ply 13, and its lead went from -7 (ply 12) to -1 (ply 14).
- Blue, good (ply 9): spent 10 on a level 1 unit at (10, 5) and moved three units into the centre; cells 11 against red's 13, and the lead stayed within 9 points.

## Middle game

- Red, good (ply 14): a tower at (6, 5) with seven moves; cells 23 to 29, lead -1 to +31 by ply 16.
- Blue, bad (ply 15): spent 15 on a tower at (8, 5), the frontier edge, with two mine spots still idle (free_spots 2). Red's lead went from +17 to +42 by ply 17.
- Red, good (ply 16): its first mine at (1, 0) and a 29 to 34 cell jump; income 23 to 32.
- Blue, bad (ply 19): spent 0 with 65 gold, income 22 and 27 cells, while red's income reached 40. Lead +68 to +116 by ply 21. Blue had now passed on mines for eight turns.
- Blue, bad (ply 21): spent 0 with 86 gold, can_cut 7. Red had 1 gold and joints 13, max_cut 19, total_cut 94: a large cut was open, and blue did not take it. Lead +116 to +139.
- Red, good (ply 22): spent 45 on a mine at (1, 3) and towers at (4, 7) and (4, 5). Red's joints fell from 13 to 4 (max_cut 19 to 5, total_cut 94 to 11) in the ply 23 metrics, so the corridor was reinforced. Lead +153 to +200 by ply 24.
- Blue, bad (ply 23): the strike of two level 3 trainings at (6, 6) and (6, 5), which took red's tower cell (6, 5), and two mines at (10, 8) and (10, 11). Red's cut_off went to 5 (ply 24). Blue's cells rose from 27 to 31 and its voronoi from 35 to 50, but its upkeep rose from 5 to 45, income from 22 to -6, and gold fell to 18. Lead +139 to +220 by ply 25. The timing was right (red had 0 gold after ply 22), but the price was blue's whole economy.
- Red, good (ply 24): spent nothing, moved units to reconnect (cut_off 5 to 4 by ply 26, to 3 by ply 28), and banked 46 to 92 gold. Hq_gap fell from 104 to 78.
- Red, good (ply 26): spent nothing again; cut_off 3, gold 92 to 139, hq_gap 78 to 31. Lead +207 to +231.

## Endgame

- Red, good (ply 28): a single move; gold 139 against a chain of 170, so hq_gap 31. Blue had 11 gold, income 2 and one move; it could not raise a defence.
- Blue, bad (ply 29): one move with 13 gold. Blue had no reply to the threat; its own hq_gap was 77 and blue's income was too low to close it.
- Red, good (ply 30): twelve trainings for 170 gold (level 3 at (5, 6), a chain along row 6, then column 11 down to (11, 11)). Red's hq_gap went to 0, and the game ended in the same turn.

## Decisive moments

- **Ply 30 (rules' pick):** red could pay a chain to blue's HQ for the first time. Real. The threat was built over plies 24 to 28 (hq_gap 104, 78, 31), and blue, with 11 to 13 gold and income near zero, could not stop it.
- **Ply 23 (reviewer's pick):** the strike was the turning point. It cut 5 cells and killed 20 of cost, but it cost blue 90 gold and its income.
- **Ply 21 (reviewer's pick):** the better strike. Blue had 86 gold and red had 1 gold and a 19-cell cut open. Waiting for ply 23 let red repair its joints first.
- **Ply 22 (reviewer's pick):** red's repair (joints 13 to 4) is why blue's strike was limited to 5 cells.

## Lessons

Blue:
- Build mines on free spots at once. Two mines from ply 8 would have added about 8 gold a turn, roughly 60 gold by ply 23.
- Do not sit on 65 to 108 gold while red builds. Strike when red's gold is low and its joints are open (ply 21), not after it has repaired them.
- Watch upkeep: each level 3 unit costs 20 a turn. Two of them turned a 22 income negative. Keep income positive and a reserve to answer an HQ chain.

Red:
- Keep the corridor redundant. Joints 13 and max_cut 19 at ply 21 were the weakness blue could have hit; the ply 22 repair was right.
- Towers do not stop level 3 units; (6, 5) was taken by a level 3 unit at ply 23. Keep a second route or a level 3 reserve at a forward joint.
- Build the first mine earlier: red had one free mine spot from ply 8 and built its first mine only at ply 16.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "One level 1 at (10, 5) and three moves into the centre; lead stayed within 9 points and cells went 8 to 11."},
  {"ply": 12, "side": "red", "verdict": "good", "note": "Two level 1 units at (6, 5) and (7, 5) plus moves; lead went from -7 to -1 by ply 14 and cells from 17 to 23."},
  {"ply": 14, "side": "red", "verdict": "good", "note": "Tower at (6, 5) and seven moves; cells 23 to 29, lead -1 to +31 by ply 16."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "Spent 15 on a frontier tower at (8, 5) while two mine spots sat idle; red's lead went from +17 to +42."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "First mine at (1, 0) and a cell jump from 29 to 34; lead +31 to +59."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Spent 0 with 65 gold and two free mine spots; red's lead went from +68 to +116."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Spent 0 with 86 gold and can_cut 7 while red had 1 gold and joints 13 (max_cut 19); the cut was not made and red's lead rose to +139."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "Mine and two towers; joints fell from 13 to 4 and max_cut from 19 to 5; lead +153 to +200."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "Two level 3 units and two mines for 90 gold cut 5 red cells, but upkeep took income from 22 to -6 with 18 gold left; red's lead rose to +220."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "No spending; moves reconnected the cut (cut_off 5 to 3 by ply 28) and gold rose 46 to 92 while blue's income was negative."},
  {"ply": 26, "side": "red", "verdict": "good", "note": "Banked gold again (92 to 139) and cut_off 4 to 3; hq_gap fell from 78 to 31; lead +207 to +231."},
  {"ply": 30, "side": "red", "verdict": "good", "note": "Twelve trainings for 170 gold on a chain to blue's HQ at (11, 11); blue had 13 gold and no defence; game won."}
]
```
