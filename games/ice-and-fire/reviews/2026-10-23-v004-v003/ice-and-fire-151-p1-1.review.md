# Review of ice-and-fire-151-p1-1

Blue (ice-and-fire-v004) won as blue, by taking red's HQ, in 16 turns (32 plies). Red is ice-and-fire-v003. The file has no board after ply 31; the win is inferred from blue's last command, which trains onto (0,0), red's HQ.

## Why the game was won

Blue won the centre, then the links to red's east corridor. Its level 2 training onto red's (6,4) at ply 15 moved the lead from +6 to +57 by ply 17, and its four trainings at ply 21 cut 7 red cells, tower included (lead +46 to +141 by ply 23). Red spent 30 of its gold on the east corridor at plies 14 and 20, leaving 7 and 3 gold, so it had no answer when the link cells at (2,4), (2,5) and (3,5) were left empty. The openings were within 9 points of each other through ply 10 and did not decide the game.

## Early game

Both bots trained two level 1 units on turn 1 (red at (1,0) and (1,1), blue at (10,10) and (10,9)). The file shows no reason for v004's opening weights, so I cannot say whether they were better than v003's; this game shows only a small early margin.

Blue:
- Good, ply 9: four moves and a level 1 training took cells from 11 to 15 and moved the lead from -9 (ply 9) to +6 (ply 11). Blue then took 4 more cells with unopposed moves at ply 11 (cells 15 to 19 at ply 12).
- Blue built no mine and no tower in the whole game. Its mine spot at (10,11) was still empty at ply 31.

Red:
- Bad, ply 10: BUILD MINE 1 0 and three moves cost 15 of its 17 gold. The mine added income later (income 10 at ply 10, 16 at ply 12), but blue's unopposed expansion at ply 11 moved the lead from +6 to +18 at ply 12.

## Middle game

Red:
- Bad, ply 14: TRAIN 2 7 4 and TRAIN 1 7 5 spent 30 of 37 gold on units in the centre. Cells rose from 18 to 23, but red was left with 7 gold against blue's 47. Blue answered at ply 15 and the lead went from +21 to +51 at ply 16.
- Bad, ply 16: a tower at (4,4) and 25 gold of moves and trainings. Red's cells recovered to 23, but the two cut cells stayed cut (cut_off 2 at ply 17), and blue took 5 more cells at ply 17. Lead +51 to +70 at ply 18.
- Good, ply 18 (not in the list): two level 1 trainings and a move reconnected the cut cells (cut_off 2 to 0 at ply 19). The lead stayed flat (+70 to +71 at ply 20).
- Bad, ply 20: TRAIN 2 6 4, TRAIN 1 7 4 and a move spent 30 of 33 gold. This won some material in the fight (army at the front fell from blue's 90 to 60 by ply 21, and the lead went from +71 to +46), but (2,4), (2,5) and (3,5) were left without a red unit, and red had 3 gold left. Lead +185 at ply 22.

Blue:
- Good, ply 15 (reviewer's pick, not flagged by the rules): TRAIN 2 6 4 took red's level 1 unit on (6,4), and two level 1 trainings at (6,5) and (6,6) followed. Red's (7,4) and (7,5) were cut. The lead went from +6 to +57 at ply 17.
- Bad, ply 19: blue spent none of its 26 gold and moved five units forward. Red's ply 20 answer took 30 army (90 to 60) and cells (36 to 32 by ply 21). The lead fell from +75 to +46 at ply 21, which blue then recovered.
- Good, ply 21: TRAIN 1 2 4, TRAIN 1 2 5, TRAIN 1 3 5 and TRAIN 2 3 6 (50 gold) took the link cells. Seven red cells were cut, including the tower at (4,4). Red's active cells fell from 28 to 16.

## Endgame

Red:
- Good, ply 22: TRAIN 2 2 4 (20 of 22 gold) retook (2,4), which a level 2 can take from a level 1. Cut cells went from 7 to 0, cells from 16 to 24, material from 153 to 207. The lead still rose at ply 24 (to +216) because blue's ply 23 took red's west cells (1,5) and (2,6). I judge the play good because it was the largest repair available, but the next turn went against red.
- Bad, ply 26: TRAIN 2 1 4 (20 of 23 gold) reconnected one of five cut cells (cut_off 5 to 4). Lead +327 to +396 at ply 28. Red's army was 10 to 30 against blue's 100 to 150 from ply 22 on.
- Ply 30: red played WAIT with 15 gold. The file has no next turn, so it is not in the list. A level 1 unit on (2,0) would have forced blue to pay a level 2 there, and blue had 50 gold, so it would not have stopped the attack.

Blue:
- Good, ply 23: moves and trainings took (1,5) and (2,6) and cut two more red cells. Lead +141 to +199 at ply 25.
- Good, ply 25: 30 gold of trainings and moves cut red further (cut_off 2 to 5). Lead +199 to +325 at ply 27.
- Good, ply 29 (HQ threat flagged): with hq_gap 0 at ply 29, blue trained (2,2) and (2,1) towards red's HQ. Lead +397 to +495 by ply 31. The three trainings at ply 31 finished it.

## Decisive moments

- Ply 21 (blue, large cut): agreed, the main moment. It was enabled by red's ply 20 spending.
- Ply 22 (red, swing): this is the lead change from ply 20 and 21, not a red error at ply 22. Red's reply (ply 22) recovered part of it (lead +185 to +141 at ply 23).
- Ply 25 (blue, large cut): a follow-up of the west pressure from ply 23; it confirmed a lead already held.
- Ply 26 and 28 (red, swings): red's army was too small to answer blue's cuts. Consequence of plies 20 and 21, not a separate mistake.
- Ply 29 (blue, HQ threat): agreed; blue's chain was a good plan and completed at ply 31.
- Ply 15 (reviewer's pick): blue's centre move, the first decisive exchange of the middle game.

## Lessons

Red (v003):
1. Keep 10 to 15 gold after a front training. Plies 14 and 20 left 7 and 3 gold.
2. Guard the cells linking an outlying corridor to the HQ. (2,4), (2,5) and (3,5) were empty at ply 21.
3. Contest the centre before spending on a mine (ply 10); blue's ply 11 took the open cells.

Blue (v004):
1. Build the mine at (10,11). Blue had 0 mines and 0 towers all game and relied on cells for income.
2. Train behind forward units. Ply 19 moved five units forward with 26 gold unspent, and the answer cost 30 army.
3. Keep the HQ chain plan from ply 29, which worked.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Moves and one level 1 training took cells from 11 to 15; lead went from -9 to +6 by ply 11."},
  {"ply": 10, "side": "red", "verdict": "bad", "note": "Spent 15 of 17 gold on a mine and three moves; blue took 4 cells unopposed at ply 11; lead +6 to +18 at ply 12."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 30 of 37 gold on centre units, leaving 7 against blue's 47; blue answered at ply 15; lead +21 to +51 at ply 16."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Level 2 onto red's (6,4) and two level 1 trainings took the centre and cut (7,4) and (7,5); lead +6 to +57 at ply 17."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Tower and trainings spent 25 of 29 gold; two cut cells stayed cut; blue took 5 more cells; lead +51 to +70 at ply 18."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Spent none of 26 gold while moving five units forward; red's ply 20 answer took army 90 to 60; lead +75 to +46 at ply 21."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Spent 30 of 33 gold at the front and left (2,4), (2,5) and (3,5) empty; blue cut 7 cells at ply 21; lead +71 to +185 at ply 22."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Four trainings took the link cells and cut 7 red cells including the tower at (4,4); lead +46 to +141 at ply 23."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "Level 2 on (2,4) retook the link: cut cells 7 to 0, cells 16 to 24, material 153 to 207. The lead rose again at ply 24 after blue's ply 23 cuts in the west."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Moves and trainings took (1,5) and (2,6) and cut two more red cells; lead +141 to +199 at ply 25."},
  {"ply": 26, "side": "red", "verdict": "bad", "note": "A 20-gold level 2 reconnected one of five cut cells; lead +327 to +396 at ply 28."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "With hq_gap 0 it trained (2,2) and (2,1) towards red's HQ; lead +397 to +495 at ply 31; won at ply 31."}
]
```
