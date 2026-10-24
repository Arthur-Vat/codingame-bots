# Review of ice-and-fire-151-p9-1

Blue (ice-and-fire-v004, the candidate) won by taking red's HQ with trainings at ply 33, in 17 turns (34 plies). Red (ice-and-fire-v003, the parent) lost.

## Why the game was won

At ply 20 red put a level 2 unit and a level 1 unit into a pocket at (7,2) and (8,2) that reached its HQ through only a few link cells. At ply 21 blue trained level 1 units onto those links, and the ply 22 board shows them. Red's cells fell from 31 to 23, its cut-off cells rose from 2 to 5, and its army fell from 80 to 30 (units 6 to 3). After that blue's income (34 against 24 at ply 22) and cells (42 against 23) kept growing, red never reconnected, and blue's hq_gap reached 0 at ply 33.

## Early game (plies 0 to 13)

Both bots opened the same way: two level 1 trainings on turn 1 (red at ply 0 on (1,0) and (1,1); blue at ply 1 on (10,10) and (10,9)), then expansion to 5 to 7 cells by ply 5. The lead stayed between -9 and +10 through ply 13, so the openings did not separate the sides.

- Red, ply 10: BUILD MINE 1 0 (15 gold) was red's only mine. Income went from 10 to 17 at ply 11. The lead still fell in the next turn (-6 to -10), so I did not judge it.
- Blue, ply 9: TRAIN 1 9 4 and three moves raised cells from 11 to 15 and moved the lead from -9 to -1 by ply 11. Good.
- Blue, all game: it built no mine (mines 0 at every ply), while red's mine gave it 4 a turn from ply 11. Mine spots at (10,4), (11,3) and (11,4) show on the ply 0 map; the file does not say whether they were reachable.

## Middle game (plies 14 to 23)

- Blue, ply 15 (bad): with 47 gold it spent 40 on four level 1 trainings and moves onto (5,3), (6,3), (6,4) and (7,4), in front of red's pocket. Cells rose from 23 to 31 at ply 16, but red's answer at ply 16 killed four of those units (army 80 to 40 at ply 17). The lead went from +6 to -6.
- Red, ply 16 (good): red had hoarded 50 gold. It trained two level 2 units onto (6,3) and (6,4), where blue's level 1 units stood, and cut five blue cells (26 at ply 17). The lead went from +57 at ply 16 to -6 at ply 17, and red gained 26 more over its next turn. The cost was upkeep 3 to 12 and income 21 to 19.
- Blue, ply 17 (good): BUILD TOWER 7 4 and moves reconnected cells (26 to 32 at ply 18). Lead -6 to +31.
- Red, ply 20 (bad): see the summary above. Lead +41 at ply 20, +152 at ply 22.
- Blue, ply 21 (good): the cut. Lead +50 to +120 at ply 23.
- Blue, ply 23 (good): three level 1 trainings cut red again (red cells 25 to 19, cut-off 5 to 7). Lead +120 to +188 by ply 25.
- Red, ply 22 (bad): one level 2 unit and two moves, no reconnection. Lead +152 to +210.

## Endgame (plies 24 to 33)

- Red, ply 24 (bad): TRAIN 2 3 4 with 28 gold did not restore a link (cells 19, cut-off 7). Lead +210 to +266.
- Blue, ply 25 (good): four level 1 trainings for 40 gold. Income 36 to 39, lead +188 to +227.
- Blue, ply 27 (good): a 40 gold burst. Lead +227 to +329 by ply 28 and +305 by ply 29.
- Blue, ply 29 and 31 (good): cuts took red's cells from 20 to 14 and from 16 to 6. Lead +384 to +601 by ply 33.
- Red, ply 28: a tower at (3,1) for 15 gold, far from the HQ, did not stop blue's approach along row 0.
- Red, ply 32: WAIT with 12 gold. Blue's hq_gap was 14 at ply 32 and 0 at ply 33, after 52 income. The file does not show whether one level 1 unit would have stopped the chain.

## Decisive moments

- **Ply 22 (rules, swing):** it mattered. It shows the result of blue's ply 21 cut, which removed red's pocket and 50 army.
- **Ply 28 (rules, swing):** it mattered. It is the result of blue's ply 27 burst, not a red error.
- **Ply 29 (rules, large cut):** it mattered. Red's cells fell from 20 to 14, and blue's cuts cut off red's supply to the endgame.
- **Ply 30, 32 (rules, swings):** consequences of blue's cuts at ply 29 and 31. Red had no answer.
- **Ply 33 (rules, HQ threat):** it ended the game.
- **Ply 16 (reviewer's pick):** red's counter-attack reversed the lead from +57 to -6 and killed four blue units.
- **Ply 21 (reviewer's pick):** the cut itself, which the rules' ply 22 flag only shows as a swing.
- **Rule discrepancy:** the file says a cell next to an enemy tower can only be entered by level 3. At ply 31 blue moved unit 37 to (3,0), next to red's active tower at (3,1), and the ply 32 board shows a level 1 (B1) there. The file does not resolve this, so check it before trusting the tower analysis at plies 28 to 32.

## Lessons

Red (v003):
1. Do not put level 2 units in a pocket joined to the HQ by one or two link cells (ply 20). Hold the link cells first, or keep the units home.
2. Keep one defender near the HQ once the enemy can reach it by a training chain. At ply 32 red had 12 gold and waited.

Blue (v004):
1. Before spending 40 gold on level 1 units next to an enemy pocket, check what the enemy can train next turn (ply 15 lost four units to two level 2 trainings).
2. Build a mine when gold allows; blue built none all game.
3. Keep the three-training cut pattern (plies 21 and 23), which won the game.

Openings: the two openings were equivalent in shape and produced a lead within about 10 until ply 13. In this one game the v004 opening gave no visible edge; the result was decided in plies 15 to 22. One game cannot show more than that.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "TRAIN 1 9 4 and moves took cells from 11 to 15; lead -9 to -1 by ply 11."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "Spent 40 on four level 1 units into red's reach; red's level 2 trainings at ply 16 killed four (army 80 to 40); lead +6 to -6."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Two level 2 trainings onto blue's units at (6,3) and (6,4) cut five blue cells; lead -57 to -31 by ply 18."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "BUILD TOWER 7 4 and moves reconnected cells 26 to 32; lead -6 to +31 by ply 18."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Placed a level 2 unit in a pocket with few link cells; blue cut it at ply 21, army 80 to 30, cells 31 to 23; lead +41 to +152."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Three level 1 trainings and moves cut red's pocket and link cells; lead +50 to +120 by ply 23."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "One level 2 training and two moves did not reconnect the cut cells; lead +152 to +210."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Three level 1 trainings cut red again (cells 25 to 19, cut-off 7); lead +120 to +188."},
  {"ply": 24, "side": "red", "verdict": "bad", "note": "TRAIN 2 3 4 with 28 gold did not restore a link (cells 19, cut-off 7); lead +210 to +266."},
  {"ply": 25, "side": "blue", "verdict": "good", "note": "Four level 1 trainings for 40 gold; income 36 to 39; lead +188 to +227."},
  {"ply": 31, "side": "blue", "verdict": "good", "note": "Four level 1 trainings and moves cut red's cells from 16 to 6; lead +384 to +601 by ply 33."}
]
```
