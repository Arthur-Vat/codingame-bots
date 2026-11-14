# Review of blue-ranking-921363064 (v022 as blue, zasmu as red)

Zasmu won as red, by taking blue's HQ with a chain of trainings at ply 60, in 31 turns.

## Why the game was won

Blue's income broke under upkeep while red's held. Blue's upkeep went from 7 (ply 22) to 28 after one level 3 bought at ply 23, to 36 after 80 gold of trainings at ply 47, and to 56 at ply 60; its income fell from 44 (ply 40) to 15 (ply 48) and -1 (ply 60). Red's upkeep stayed at 13 to 30 against a gross of 60 to 75. Blue also lost its north-east pocket to cuts at ply 26 and ply 38. By ply 58 blue had lost 300 of training cost, against 170 killed by red.

## Early game

- Blue, ply 1, good: two level 1 trainings on its wall (10,10) and (10,11). Material went from red +11 at ply 1 (199 vs 188) to blue +8 at ply 3 (196 vs 188).
- Red, ply 12, bad: it trained two level 1 units onto (5,6) and (5,7), next to blue's front. Blue killed both at ply 13 (10 by fight, 10 by cut). Red's cum_lost went from 0 to 20 at ply 14, and blue's material rose to 212 against red's 192.

## Middle game

- Blue, ply 13, good: a level 2 at (5,6) and a move took the red pair. Blue's exchange went to +20 (cum_killed 20, cum_lost 0 at ply 14).
- Red, ply 20, good: two level 1 trainings (4,6) and (4,7) with a mine (35 gold) cut two blue cells (cut_off 2 at ply 21). The lead went from blue +18 at ply 20 to level (353 each) at ply 22.
- Red, ply 22, good: a level 1 and a tower at (6,4) (25 gold). Red led by 46 at ply 24.
- Blue, ply 23, bad: TRAIN 3 at (7,4) plus a level 1 (40 gold). Gold fell from 73 to 33, and upkeep rose by 21, so income went from 35 (ply 23) to 20 (ply 24). Material went from blue 374 to 385 against red 373 to 424 at ply 25.
- Blue, ply 25, bad: 70 gold on two towers, two level 1s and a level 2 (8,6), (7,5), (7,3). Gold fell to 0, and the units went into a pocket that hung on one link through (6,6) and (7,6). Blue's exchange was still even (cum_lost 70 at ply 26).
- Red, ply 26, good: a level 3 at (6,6) and three level 1s at (6,7), (6,8), (7,8) (70 gold). Cut-kills went from 70 to 110 by ply 28, and blue's cut_off went from 0 to 6. The lead went from +33 (ply 26) to +115 (ply 28) and +177 (ply 29).
- Blue, ply 27, bad: two towers at (6,9) and (8,8) (30 gold) did not reconnect the cut pocket, because red's level 3 held the link and blue had no unit that could beat it. The lead went from +86 (ply 27) to +177 (ply 29).

## Endgame

- Blue, ply 37, bad: seven level 1 trainings (70 gold) into the same north pocket (8,3) to (8,6), (7,2), (6,2), (9,5). Blue's material rose by 49 that turn, but red's ply 38 cut the pocket: cut_off 11 at ply 39, and cum_lost from 130 to 230 by ply 40. The lead went from +191 (ply 37) to +235 (ply 39). The rules' ply 38 flag (a swing favouring blue) is misleading: it is the trainings' cost on paper, not a gain.
- Red, ply 38, good: TRAIN 2 at (9,7), TRAIN 1 at (10,7), a tower at (4,6), and moves into (6,7) and (6,8). That cut 11 blue cells and took the kill count to 220 by cut. The lead went from +108 (ply 38) to +160 (ply 40).
- Blue, ply 47, bad: 80 gold on a level 3 at (7,8) and two level 2s and a level 1. Upkeep went from 7 to 36 and income from 37 to 15 at ply 48. Gold was 5 at ply 48, so blue had no reserve, and the lead went from +316 (ply 47) to +346 (ply 49).
- Red, ply 60, won: 210 gold of trainings (three level 3s and a level 2 at (10,5) to (11,10), then a level 1 at (11,11)). Red's hq_gap was 1 at ply 58 and 0 at ply 60. Blue had 64 gold and -1 income.

## Decisive moments

- Ply 24, lead change: correct. The cause is blue's ply 23 level 3 purchase, not the lead itself.
- Ply 28, swing: correct. The cut was red's ply 26 play.
- Ply 30, swing: correct. Blue's ply 27 towers did not reconnect the cut pocket.
- Ply 38, swing favouring blue: misleading. See the note under ply 37.
- Ply 46, swing favouring red: correct. Blue's cut_off was 4 at ply 45, and red's towers held the frontier.
- Ply 60, HQ threat: correct, and it was the end of the game.
- (reviewer's pick) Ply 35: blue kept 57 gold with 2 free mine spots and built no mine. Its mines stayed at 2 from ply 22 to the end, while red built 3. Two mines are +8 a turn, roughly 280 gold over the game.
- (reviewer's pick) Ply 47: the upkeep jump described above, with no reserve, was the point where blue could no longer buy back its cells.

## Lessons

Blue (v022):
1. Work out income after upkeep before buying a level 3 or spending most of the gold. A 30 gold level 3 cost 20 a turn, and blue's income fell from 35 to 20 in one purchase.
2. Build the idle mine spots (2 free from ply 22 to the end) and keep about 30 gold back to reconnect a cut pocket.
3. Do not train into a pocket that hangs on one link (ply 25, ply 37). Count the joints and cut-offs before committing gold there.

Red (zasmu, to copy):
1. Cut by training onto the enemy's link cells (ply 20, ply 26, ply 38), not by attacking its units.
2. Build towers along the frontier (ply 22, ply 28, ply 42): 25 towers by the end, and blue could not enter them without a level 3.
3. Keep upkeep low with level 1 units, buy level 3 only at the end, and save for the HQ chain (gold 260 at ply 60).

```json
[
  {"ply": 1, "side": "blue", "verdict": "good", "note": "Two level 1 trainings on its wall (10,10) and (10,11); material went from red +11 to blue +8 by ply 3."},
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Two level 1s trained onto (5,6) and (5,7) next to blue's front; blue killed both at ply 13 (cum_lost 20 at ply 14)."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "A level 2 at (5,6) and moves took the red pair; blue's exchange went to +20 at ply 14."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Trained onto blue's (4,6) and (4,7) with a mine (35 gold); blue cut_off 2 at ply 21; lead moved from blue +18 to level by ply 22."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "A level 1 and a tower at (6,4) for 25 gold; red led by 46 at ply 24 after blue's ply 23 error."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "Bought a level 3 (30) plus a level 1; upkeep rose by 21 and income fell from 35 to 20; red led by 46 at ply 24."},
  {"ply": 25, "side": "blue", "verdict": "bad", "note": "Spent 70 (two towers, three units) into a north-east pocket on one link, leaving 0 gold; red cut it at ply 26."},
  {"ply": 26, "side": "red", "verdict": "good", "note": "A level 3 at (6,6) and three level 1s (70 gold) cut 6 blue cells at ply 28; the lead went from +33 to +115."},
  {"ply": 27, "side": "blue", "verdict": "bad", "note": "Two towers (30 gold) at (6,9) and (8,8) did not reconnect the cut pocket; the lead went from +86 to +177 by ply 29."},
  {"ply": 37, "side": "blue", "verdict": "bad", "note": "Seven level 1 trainings (70 gold) into the north pocket; red cut 11 cells at ply 38; cum_lost 130 to 230 by ply 40."},
  {"ply": 38, "side": "red", "verdict": "good", "note": "Cut 11 blue cells with trainings and a tower; the lead went from +108 to +160 by ply 40."},
  {"ply": 47, "side": "blue", "verdict": "bad", "note": "Spent 80 on a level 3 and two level 2s; upkeep went from 7 to 36 and income from 37 to 15; no reserve left."}
]
```
