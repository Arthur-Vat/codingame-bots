# Review of red-ranking-921371154

zasmu won as blue, over v022 as red, in 22 turns (44 plies). The file ends at ply 43 with blue's 13 trainings, and its last command trains a level 3 unit on (0,0), red's HQ; the file has no board after that ply, so the capture itself is not shown.

## Why the game was won

Blue out-earned red from the middle game on. At ply 26 its gross income was 66 against 37 (46 active cells against 33, and five mines against one), and at ply 34 it was 80 against 41. Red's three level 3 purchases (plies 26, 34, 40) each cost 20 upkeep a turn: its upkeep went from 10 at ply 33 to 33 at ply 35 and to 54 at ply 41, and its income fell to -1. Blue then saved from ply 35 (gold 68, rising to 318 at ply 43) and spent 270 on one chain of trainings toward red's HQ, when its hq_gap was 0 and red's was 123 with 7 gold.

## Early game

Plies 0 to 13. Both sides trained about one level 1 unit a turn, but red spent nothing at plies 4, 6, 10 and 12, letting gold rise to 23.

- **Red, bad, ply 10:** it sent its units north-west (`MOVE 4 1 4;MOVE 1 2 4;MOVE 6 3 4`) and spent 0 with 12 gold. Blue then took the open centre column (4,7) to (4,9) at ply 11, and its cells rose from 15 to 20 (ply 12, against red's 14). Lead went from -25 to -36.
- **Blue, good, ply 11:** it moved units into the centre column while red was away from it; blue's material lead went from +15 to +30 by ply 13.

## Middle game

Plies 14 to 19.

- **Red, bad, ply 14:** it spent 30 on one mine (1,0) and a tower (3,5). Three mine spots stayed empty (free_spots 3 at ply 14). Lead went from -48 to -67 by ply 16.
- **Blue, good, ply 15:** a tower on (4,6) and a level 1 unit on (4,5) cut the centre. Red's voronoi fell from 46 (ply 15) to 33 (ply 16), blue's rose to 71, and blue's lead went from +54 to +91 by ply 17.
- **Red, bad, ply 16:** it spent 0 with 28 gold while blue held (4,5) and (4,6). Lead went to -99 by ply 18.
- **Red, bad, ply 18:** it spent 0 again with 49 gold (kept_share 100). Blue's income rose from 24 to 32 (ply 16 to 18), and lead went to -137 by ply 20. Red was not short of gold to contest the corridor; it was short of decisions.

## Endgame

Plies 20 to 43. Red's 70 gold at ply 20 bought six units that took the north-east row, and red's cells rose from 20 to 29 by ply 23, so that spend was not wasted. Its later play was the problem.

- **Red, good, ply 26:** a level 3 unit entered blue's tower on (4,6), the best move red made; lead went from -140 to -101 by ply 28.
- **Blue, bad, ply 27:** it retook (4,6) with a level 3 unit (55 spent). Its upkeep went from 13 to 33 and its income from 64 to 40, and its lead fell from +186 to +127 by ply 29. Blue's towers (10 by ply 34, against red's 4) held the corridor better than its level 3 did.
- **Red, bad, ply 34:** it spent 50 on a level 3 unit that replaced blue's level 3 on (2,5) and a level 2 unit. Blue's army fell from 160 to 120, but red's upkeep went from 10 to 33 and its income from 33 to 11. Lead went from -165 to -278 by ply 36. This is the rules' decisive ply 36.
- **Blue, good, ply 35 onward:** blue stopped spending (WAIT at plies 35, 39 and 41), its income stayed at 60 or more, and its gold rose from 68 to 256. Its hq_gap fell from 92 (ply 36) to 17 (ply 39), which red could read in its own metrics.
- **Red, bad, ply 40:** it spent 45 on another level 3 unit (`TRAIN 3 7 5`). Income went from 15 to -1 and gold from 53 to 8, and lead went to -391 by ply 42. This is the rules' decisive ply 42.
- **Blue, good, ply 43:** it spent 270 on 13 trainings in one chain from (7,5) to (0,0), through red's tower on (7,3) (a level 3 can enter a tower). Its first step was on (7,5), where the ply 43 board shows a red level 3 unit; the file does not show how that step resolved.

## Decisive moments

- **Ply 36 (rules):** a real loss. It followed red's ply 34 level 3 and level 2 purchases, which cut red's income from 33 to 11. The kill at (2,5) did not pay for the upkeep.
- **Ply 42 (rules):** a real loss, caused by ply 40. Red's income was -1 at ply 41.
- **Ply 43 (rules):** blue's strike, made possible by its saved gold. Red's 10 units and 7 gold could not answer it.
- **Ply 15 (reviewer's pick):** the tower on (4,6) that moved the voronoi share from red to blue. It is the first step of the game's real swing, and red's ply 16 did not answer it.
- **Plies 16 to 18 (reviewer's pick):** red held 28 to 49 gold through two turns while the corridor changed hands.

## Lessons

**Red (v022):**
1. Build a mine on every free mine spot as soon as it is on an active cell: red had 2 to 6 free spots for the whole game and built one mine. Each mine costs 15 and returns 4 a turn; blue had six by ply 28.
2. Do not hoard while the centre is contested. When blue's tower and one unit took (4,5) and (4,6) at plies 15 to 17, red held 28 to 49 gold and bought nothing; a level 2 or level 3 unit could have contested it.
3. Keep level 3 for taking a tower at the right moment, not as a standing army. Each one costs 20 upkeep a turn; three of them took red's income to -1. At ply 39, with blue's hq_gap at 17 and blue holding 193 gold, red should have answered the HQ threat rather than spend 45 on a new level 3 unit at ply 40.

**Blue (zasmu), for comparison:**
1. Take the open centre column early (plies 7 to 11) and build a mine on every spot that opens (six by ply 28).
2. Its own level 3 retake at ply 27 cost 20 upkeep a turn for the rest of the period in which it was held; towers and level 1 units held the corridor more cheaply.
3. Saving income from ply 35 and striking once with a chain of trainings worked; red's metrics showed the threat from ply 39.

```json
[
  {"ply": 10, "side": "red", "verdict": "bad", "note": "Spent 0 with 12 gold and sent its units north-west; blue took the centre column (4,7) to (4,9) at ply 11, cells 15 to 20."},
  {"ply": 11, "side": "blue", "verdict": "good", "note": "Moved into the open centre column while red was away from it; lead went from +15 to +30 by ply 13."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 30 on one mine and a tower; three mine spots stayed empty, and lead went from -48 to -67."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Tower on (4,6) and a level 1 unit on (4,5) cut red's voronoi from 46 to 33; lead went from +54 to +91 by ply 17."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Spent 0 with 28 gold while blue held the corridor; lead went to -99 by ply 18."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Spent 0 with 49 gold; blue's income rose from 24 to 32 and lead went to -137 by ply 20."},
  {"ply": 26, "side": "red", "verdict": "good", "note": "Level 3 unit entered blue's tower on (4,6); lead went from -140 to -101 by ply 28."},
  {"ply": 27, "side": "blue", "verdict": "bad", "note": "Level 3 retake of (4,6) for 55 spent; upkeep 13 to 33 and income 64 to 40; lead fell from +186 to +127 by ply 29."},
  {"ply": 34, "side": "red", "verdict": "bad", "note": "Spent 50 on a level 3 and a level 2; upkeep 10 to 33, income 33 to 11; lead went from -165 to -278 by ply 36."},
  {"ply": 40, "side": "red", "verdict": "bad", "note": "Spent 45 on another level 3; income 15 to -1 and gold 53 to 8; lead went to -391 by ply 42."},
  {"ply": 43, "side": "blue", "verdict": "good", "note": "Spent 270 saved since ply 35 on a 13-unit chain to red's HQ at (0,0); the game ended on this ply."}
]
```
