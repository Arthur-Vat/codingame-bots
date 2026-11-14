# Review of red-ranking-921367560

zasmu (blue) beat v022 (red) by taking red's HQ at ply 45, turn 23 (46 plies), with a chain of 9 level 3 trainings and one level 2 training costing 290 gold.

## Why the game was won

v022 was level with blue until turn 6 (lead within 20 to ply 12) and fell behind on turns 7 and 8 (plies 12 to 14), when blue trained into the centre at (5,4) and (6,4) and the lead went from +17 to +26 for blue. It never won it back, because it spent little: it kept 66 to 165 gold unspent from ply 20 to 36 (kept_share 100 at ply 20), and its four free mine spots stayed empty until ply 30. Blue turned its income (54 against 24 at ply 34) into 17 towers, which only level 3 units can enter, so v022's level 1 units could not cross the middle. Blue then hoarded from ply 35 to 269 gold and bought the chain that reached red's HQ.

## Early game

- Red, ply 10 (good): one level 1 unit and three moves; cells rose from 11 to 15 by ply 11 (blue 14), and the material gap fell from +18 to +3.
- Blue, ply 1 (good): two level 1 units for 20 gold on the south-east side. At ply 4 blue had 6 cells and income 4, red 4 cells and income 2.
- Red, ply 12 (bad): 13 gold, no training, four moves along row 3 that left (5,4) and (6,4) open. Blue trained at (5,4) on ply 13 and went from 18 to 24 cells by ply 14.

## Middle game

- Red, ply 14 (bad): 28 gold; it bought a tower at (5,3) for 15 and kept 13, with no unit contesting the centre. Blue's towers at (5,4) and (7,4) followed, and the lead went from +26 to +48 by ply 16.
- Red, ply 16 and 18 (bad): spent 0 with 30 and then 48 gold, two moves each. Blue spent 30 on towers at (7,4) and (5,6) on ply 17 and built its first mine on ply 19 (lead +64 to +76).
- Red, ply 20 (bad, reviewer's pick): WAIT with 66 gold and 4 level 1 units, 22 cells, income 18. Blue spent 35 on a mine and two level 1 units and grew from 32 to 38 cells; the lead went from +101 to +137 by ply 22. The rules did not flag this turn.
- Red, ply 30 (late): its first mine, at (1,0); income 19 to 24 on ply 31. Spots had been free since ply 14, and red had 3 towers against blue's 12.
- Blue, ply 13 (good): one level 1 unit at (5,4) plus five moves took the centre; lead +2 to +35 by ply 15.
- Blue, ply 17 (good): two towers at (7,4) and (5,6) for 30 gold formed the wall that held the centre (lead +64 to +76 by ply 19).
- Blue, ply 21 (good): a mine at (8,10) and two level 1 units for 35 gold; cells 38 to 42 on ply 22, lead +136 to +179 by ply 23.

## Endgame

- Red, ply 38 (bad): 70 gold for one level 3 unit (army 30, upkeep 20), two towers next to its HQ and one level 1. Upkeep went from 6 to 27 on ply 39, income from 24 to 5, and to -13 by ply 41. Material fell from 450 to 391 and the lead rose from +293 to +388 by ply 39. The rules' ply 40 swing comes from this spend, not from blue's play.
- Red, ply 40 (bad): its level 3 at (7,4) and a move to (6,4) cut 4 blue cells off (cut_off 4 on ply 41) and removed 20 of blue's cost. The lead still went from +383 to +400 by ply 42, and blue kept its gold (221 on ply 41, 269 on ply 43). Red had 64 gold and income -13, so it could not repeat the cut.
- Red, ply 44 (no effect): 25 gold on a tower and a level 1 unit; blue's hq_gap was 1 on ply 43 and 0 on ply 45.
- Blue, ply 35 (good): spent 0 with 65 gold and kept its income; gold rose to 119 (ply 37) and 269 (ply 43). Its upkeep stayed at 10 (ten level 1 units), so the 54 income was nearly all free for the chain.
- Blue, ply 45: the trainings walked down column x=4 and row y=1 to (0,0). Blue's hq_gap had been 38 on ply 39, 19 on ply 41, 1 on ply 43.

## Decisive moments

- Ply 40 (rules, swing favours blue): the 90-material swing started on red's ply 38 (income collapse from its level 3 upkeep). Blue's ply 39 WAIT changed nothing.
- Ply 45 (rules, HQ threat): visible three turns earlier through blue's hq_gap (38, 19, 1). Red had 26 gold on ply 44 and could not answer it.
- Ply 20 (reviewer's pick): red's WAIT with 66 gold. Blue's gap in income and cells grew from here (income 18 to 35 by ply 20).
- Ply 13 (reviewer's pick): blue's unit at (5,4) took the centre cell that red had left open on ply 12; the towers built on it at plies 15 to 17 kept red's units out for the rest of the game.

## Lessons

Red:
1. Spend. Keep gold near 20 at the end of a turn, build the home mines by ply 16 (15 gold, +4 a turn each), and never WAIT with 60 or more gold (plies 20 and 34).
2. Take the centre before towers go up (plies 12 to 16): with 13 to 28 gold, put level 1 or 2 units on (5,4) and (6,4). Only level 3 can enter an enemy tower, so the first towers there are what stop a level 1 army.
3. Count upkeep before a level 3: it costs 20 a turn, nearly all of red's income. Buy it only when it decides the game. With hq_gap at 38 (ply 39), keep gold and units on the HQ approaches (column x=4, row y=1) instead.

Blue:
1. Keep spending the income on towers and mines while building the wall (plies 17 to 33, 40 to 45 a turn).
2. Keep the army at level 1 (upkeep 10) until a chain is affordable, then hoard for it, as on plies 35 to 45.

```json
[
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Spent 0 with 13 gold; four moves left (5,4) and (6,4) open. Blue took them on ply 13 (cells 18 to 24); red's lead went from +17 to +26."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "One level 1 unit at (5,4) and five moves took the centre; lead +2 to +35 by ply 15."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Tower at (5,3) for 15 with 28 gold, 13 kept, no unit into the centre; lead +26 to +48 by ply 16."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Two towers at (7,4) and (5,6) for 30 gold held the centre; lead +64 to +76 by ply 19."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "WAIT with 66 gold, kept_share 100; blue spent 35 and grew 32 to 38 cells; lead +101 to +137 by ply 22."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Mine at (8,10) and two level 1 units for 35 gold; cells 38 to 42; lead +136 to +179 by ply 23."},
  {"ply": 34, "side": "red", "verdict": "bad", "note": "WAIT with 141 gold while blue had 17 towers; lead +228 to +258 by ply 36."},
  {"ply": 35, "side": "blue", "verdict": "good", "note": "Spent 0 with 65 gold and kept income for the level 3 chain; lead +258 to +317 by ply 37."},
  {"ply": 38, "side": "red", "verdict": "bad", "note": "Level 3 unit (upkeep 20) plus two towers and a level 1 for 70 gold; income 24 to 5, then -13; lead +293 to +388 by ply 39."},
  {"ply": 40, "side": "red", "verdict": "bad", "note": "Level 3 cut 4 blue cells and removed 20 of blue's cost, but the lead went +383 to +400 by ply 42 and red had 64 gold at -13 income."}
]
```
