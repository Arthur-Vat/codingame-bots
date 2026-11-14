# Review of red-ranking-921368792

v022 (red) lost to zasmu (blue). Blue won by taking red's HQ on turn 22 (ply 43), before the 100-turn limit.

## Why the game was won

Blue out-built red economically and then banked for one strike. From ply 15 blue put towers on its front and kept spending about 45 gold a turn, reaching 21 towers and 47 active cells against red's 5 towers and 23 cells (ply 36). Blue's income rose to 57 against red's 22, with 5 mines against red's 1. Red kept most of its gold idle (kept_share 100 on most turns, gold 225 at ply 42) and never trained a second unit after ply 14. Blue stopped spending from ply 37, and its nine trainings at ply 43 (220 gold, six of them level 3) ended the game.

## Early game (plies 0 to 13)

Both sides expanded at a similar pace. Blue's material was ahead from ply 2 (44 against 33) and led by 25 at ply 10 (135 against 110), with cells 15 against 11 and income 11 against 8.

- Blue, good: ply 13, one training and six moves. Its cells rose from 20 to 26 at ply 14 and its lead went from +19 to +38 by ply 15.
- Red, bad: ply 12, four moves and no training with 13 gold in hand. Its cells rose from 15 to 19, but blue's cells rose from 20 to 26 and the lead went from -32 to -43 at ply 14. Red's 5 level-1 units were no match for blue's 6.
- Red had no clearly good early play. Its expansion at plies 4 to 8 kept pace with blue's.

## Middle game (plies 14 to 23)

Blue's towers did the work here. A level 1 or 2 unit cannot enter a tower cell, so each tower was a wall for red's units. Blue's first tower, at (6,5) in ply 15, sat on the front.

- Blue, good: ply 15, a tower at (6,5) with a training and seven moves. The lead went from +38 to +65 by ply 17. Ply 19 (tower at (7,3), mine at (7,10), lead +67 by ply 21) and ply 23 (three towers) also grew the lead.
- Red, bad: ply 16, moves only with 36 gold. Blue's lead went from +54 to +79.
- Red, bad: ply 20, when red spent 45 gold on a mine at (4,1) and towers at (3,2) and (5,1). Its can_cut was 7 with 75 gold, the largest threat it had all game, and it was not used; red moved one unit. The lead went from +124 to +157 by ply 22.
- Red, bad: ply 22, WAIT with 52 gold and 3 idle mine spots. Blue reached 44 cells and 4 towers. The lead was +178 at ply 24.

## Endgame (plies 24 to 43)

- Red, bad: ply 38, moves only with 196 gold. Its material gap widened from +288 to +323 by ply 40.
- Red, bad: ply 40, a tower at (1,1) for 15 gold, next to its HQ. It raised red's hq_gap from 32 to 47 (ply 41). Ply 42 then spent another 15 on a mine at (1,0), and the gap was 40 at ply 43. Red's own HQ chain cost about 250 gold, and it had 225. Its three idle mine spots (free_spots 3 from ply 22 to ply 42) would have added about 12 gold a turn.
- Blue, good: ply 27, three towers and one training. Its lead went from +238 to +249 by ply 29.
- Blue, good: ply 37 and ply 39, no spending. Blue's gold rose from 63 to 120 to 177 and its hq_gap fell from 97 to 40 to 23. The lead went from +310 to +345 and then to +385.
- Blue, decisive: ply 43, nine trainings from (5,3) through (2,1) to (0,0) for 220 gold. The file does not print the capture itself, but turn 22 is before the 100-turn limit, so the HQ was taken.

## Decisive moments

- Ply 43 (rules: HQ threat, favours blue): correct. Blue's hq_gap fell from 23 at ply 42 to 0 at ply 43, and red's gap at ply 42 was 25. Red was not behind on the race by much, but it had spent 30 gold on its own HQ corner in plies 40 and 42, and blue reached the threat first.
- (reviewer's pick) Ply 15: blue's tower at (6,5). It was the first tower in contact, and blue's frontier and cell count grew after it while red never cut a blue cell.
- (reviewer's pick) Ply 20: red's can_cut of 7 with 75 gold was the one window to hit blue's cells before its towers covered them, and it was not used.

## Lessons

Red (v022):
1. Build the idle mine spots early. Three mines at ply 22 cost 45 gold and return about 120 by turn 22. Red had 3 free spots for about 10 turns.
2. Spend when can_cut is high (ply 16 to 20) rather than banking. From ply 16 red spent nothing on 10 of its 14 turns, and it finished with 225 gold and 5 level 1 units.
3. Towers stop only levels 1 and 2. Blue's final chain used level 3 units, which pass tower cells. A level 3 unit or two near the HQ is the defence against that, and towers at (1,1) and (1,0) were not.

Blue (zasmu):
1. Keep putting towers at the contact points before the enemy arrives (ply 15 and ply 19 were the decisive ones).
2. The bank from ply 37 to ply 41 won the game in one turn. Keep the hq_gap in view each turn, as blue did.

```json
[
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Four moves and no training with 13 gold; blue's cells grew faster and the lead fell from -32 to -43 by ply 14."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "One training and six moves; cells 20 to 26 at ply 14, lead +19 to +38 by ply 15."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Tower at (6,5) on the front with a training; lead +38 to +65 by ply 17."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Moves only with 36 gold while blue built towers; lead +54 to +79 by ply 18."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Tower at (7,3) and mine at (7,10); lead +109 to +176 by ply 21."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Spent 45 on a mine and two towers late with can_cut 7 and 75 gold unused; lead +124 to +157 by ply 22."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "WAIT with 52 gold and idle mine spots; lead +157 to +178 by ply 24."},
  {"ply": 27, "side": "blue", "verdict": "good", "note": "Three towers and a training; lead +238 to +249 by ply 29."},
  {"ply": 37, "side": "blue", "verdict": "good", "note": "Saved all gold for the HQ chain (hq_gap 97 to 40); lead +310 to +345 by ply 39."},
  {"ply": 38, "side": "red", "verdict": "bad", "note": "Moves only with 196 gold; lead +288 to +323 by ply 40."},
  {"ply": 39, "side": "blue", "verdict": "good", "note": "Saved again (gold 120 at ply 40, hq_gap 40); lead +345 to +385 by ply 41."},
  {"ply": 40, "side": "red", "verdict": "bad", "note": "Tower at (1,1) for 15 gold raised red's own hq_gap from 32 to 47; lead +323 to +363 by ply 42."}
]
```
