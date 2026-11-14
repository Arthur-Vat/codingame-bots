# Review of blue v022 against red zasmu

Red zasmu won as red, by training a level 3 chain onto blue's HQ at ply 50, in 26 turns.

## Why the game was won

Material was close until ply 23, with blue slightly ahead from ply 12 to ply 23. Blue then sat on gold: it kept 100 of 100 gold at ply 19 and 21 and spent only 30 at ply 23, while red spent almost all its income each turn. At ply 24 red put 80 gold into six trainings, including a level 3 that entered blue's tower at (6,6), and cut off blue's centre: blue cells fell from 39 to 31 at ply 25 and material swung from +17 blue to +53 red. Blue never fielded a level 3, so red's level 3 units could not be answered, and red then built towers as a wall while blue's gold grew to 247 unspent.

## Early game

Not a phase with a clear swing: material see-sawed within about 10 points (blue 174, red 167 at ply 12).

- Red, ply 12, bad: `TRAIN 1 5 6;TRAIN 1 5 7` put two level 1 units on the contact line at (5,6) and (5,7). At ply 13 blue's level 2 and moves took them; red's lost cost rose from 0 to 20 at ply 14 (exchange -20, red cum_lost 20). Red's material at ply 13 was 203, and it was 192 at ply 14 against blue's 212.
- Blue: no early play judged bad or good; its first mine spots appear only at ply 13 (free_spots 3).

## Middle game

- Blue, ply 13, good: `TRAIN 2 5 6` with moves cut red's units on the line. Blue's exchange rose to +20 at ply 14 (blue cum_killed 20, red cum_lost 20), and blue led on material (212 to 192).
- Red, ply 16, good: `BUILD MINE 2 0`, its first mine. Income went from 21 at ply 16 to 28 at ply 17, the largest jump in the game for red until ply 24.
- Blue, ply 19, bad: `MOVE 3 3 8;MOVE 4 5 11`, spent 0 with 55 gold in hand (kept_share 100). Blue had 4 free mine spots (free_spots 4) and built none, and its income stayed at 25 while red's rose to 32. Blue's gold went 30 at ply 17 to 55 at ply 19 and 82 at ply 21.
- Blue, ply 23, bad: `BUILD TOWER 11 10;BUILD TOWER 7 8`, 30 gold, with 70 kept. The tower at (11,10) sits on the HQ edge where it does nothing. Blue had can_cut 6 at this point, so it could have struck; instead red's ply 24 level 3 went onto the tower at (6,6).
- Red, ply 24, good (reviewer's pick; the rules flag only ply 26): `TRAIN 3 6 6;TRAIN 1 7 6;TRAIN 1 5 6;TRAIN 2 4 7;TRAIN 1 4 8` took blue's tower and cut 8 blue cells. Blue's cells fell 39 to 31, its cut_off rose to 3 and its cum_lost to 50 (ply 25). Red's material went from 435 to 403, but blue fell from 416 to 350.
- Red, ply 26: the lead change the rules report (red +62 at ply 26) is the result of ply 24, not a new play.

## Endgame

- Blue, ply 31, good: `TRAIN 2 8 6` and five level 1 trainings (105 gold) struck back into red's pocket. Blue's exchange went from -40 at ply 31 to +40 at ply 32, red's cum_lost rose from 20 to 100 and red's cells fell from 66 to 53 (cut_off 5 at ply 32). This was the best blue turn after ply 13, but blue spent its gold at once and did not keep pressing.
- Red, ply 30, bad: 30 gold on two towers and 13 moves thinned red's pocket. Red's material fell from 611 at ply 31 to 567 at ply 32, the first drop after its lead had grown since ply 24.
- Red, ply 32, good: `BUILD TOWER 1 6;BUILD TOWER 3 6;BUILD TOWER 4 6` (55 gold) closed the gap; material returned to 577 at ply 33 and kept rising.
- Blue, ply 33, good: `BUILD MINE 9 11`, its first mine (mines 0 to 1, income 23 to 28 by ply 34). Late, but it was the first economic move blue made after ply 13.
- Blue, ply 45, bad: `MOVE 29 8 6;MOVE 22 9 5`, 0 spent with 171 gold. Blue's hq_gap had fallen to 89 and then to 23 at ply 49, so blue was close to its own HQ chain, but it had no level 3 to enter red's towers and it only reached 247 gold while red's income reached 72.
- Red, ply 50, good: 180 gold of trainings, including level 3 units at (10,10) and (11,11), won the game. Red's hq_gap was 0 at ply 50 against blue's 23 at ply 49.

## Decisive moments

- Ply 26 (lead change, rules): a result of ply 24. Red's 80-gold strike is the cause.
- Ply 28 (swing, rules): red's cells rose from 52 to 52 and its army from 130 to 140; blue's material barely moved. Not a new play.
- Ply 30 (swing, rules): red's cells rose 52 to 59 between plies 28 and 29, and red's army reached 150. Blue's only response, ply 31, is the counterstroke above.
- Ply 50 (HQ threat, rules): the winning chain. Blue's hq_gap of 23 at ply 49 shows how close blue was to the same race with 247 gold.
- Ply 24 (reviewer's pick): the level 3 onto blue's (6,6) tower, the real turning point.
- Ply 31 (reviewer's pick): blue's counterstroke, the only time blue traded well after ply 13.

## Lessons

Blue:
- Build the free mine spots early (3 to 4 were available from ply 13); blue's first mine came at ply 33, while red had 4 mines by ply 41.
- Do not hoard. Gold kept unspent from ply 19 to ply 49 (up to 247) bought nothing; spend it on mines first, then on a level 3 to hold the (6,6) and (5,6) cells.
- Answer a level 3 with a level 3: blue never trained one, and red's level 3 units went through its towers.

Red:
- Keep the first mine and tower build going; red kept spending nearly all its income from ply 12 on.
- Do not march thin units into a pocket (ply 30): keep the cut line guarded before adding towers.

```json
[
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Two level 1 trainings at (5,6) and (5,7) were cut or killed by blue's level 2 at ply 13; red's cum_lost rose to 20 by ply 14."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Level 2 at (5,6) with moves killed red's exposed units; blue's exchange rose to +20 by ply 14."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "First mine at (2,0); income rose from 21 to 28 by ply 17."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Spent 0 with 55 gold (kept 100%) and 4 free mine spots; gold reached 82 by ply 21 while red rose to 32 income."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "Spent 30 on two towers, one at the HQ edge (11,10), keeping 70 gold; red's level 3 took the (6,6) tower at ply 24."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "80 gold: a level 3 onto blue's (6,6) tower and four others cut 8 blue cells; blue's material fell from 416 to 350."},
  {"ply": 30, "side": "red", "verdict": "bad", "note": "Two towers and 13 moves left the pocket thin; red's material fell from 611 to 567 after blue's ply 31 strike."},
  {"ply": 31, "side": "blue", "verdict": "good", "note": "105 gold of trainings struck red's pocket; blue's exchange went from -40 to +40 and red's cells from 66 to 53."},
  {"ply": 32, "side": "red", "verdict": "good", "note": "Three towers (55 gold) rebuilt the line; material back to 577 at ply 33."},
  {"ply": 33, "side": "blue", "verdict": "good", "note": "First mine at (9,11); income rose from 23 to 28 by ply 34."},
  {"ply": 45, "side": "blue", "verdict": "bad", "note": "Spent 0 with 171 gold and no level 3; blue's gold kept growing while red's income was 71."},
  {"ply": 50, "side": "red", "verdict": "good", "note": "180 gold of trainings, ending with level 3 units at (10,10) and (11,11): the HQ won."}
]
```
