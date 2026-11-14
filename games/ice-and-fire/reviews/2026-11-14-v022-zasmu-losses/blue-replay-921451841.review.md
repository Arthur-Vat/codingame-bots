# Review of blue-replay-921451841: zasmu (red) beat v022 (blue)

zasmu won as red, in 19 turns (37 plies), by a 230-gold chain of trainings that ended on blue's HQ at (11,11) at ply 36.

## Why the game was won

The material lead was close until ply 15, when v022 cut a red pocket but emptied the two cells that linked its own cut-off cells (ply 15: blue 236 vs red 243 at the start, then red 188 vs blue 272 at ply 16). Red took the lead back at ply 16 by training two units onto those cells, and then out-grew v022 on income: mines at plies 20 and 22 gave red 46 income by ply 23 against blue's 27. From ply 21 blue put most of its gold into towers (about 120 gold for 8 towers, no income), while red's income climbed to 53. Red's final 230-gold chain used level 3 units to pass blue's towers, which level 1 and 2 units could not do.

## Early game

Plies 0 to 12. Both bots spent almost all their gold each turn and grew at a similar pace (cells 17 vs 19 at ply 12).

- **Red, good, ply 12.** Two level 1 trainings at (5,6) and (6,6) plus moves took red from 17 to 23 cells (ply 13) and income from 13 to 17. Material went from 167 vs 174 at ply 12 to 203 vs 189 at ply 13.
- **Blue, good, ply 13.** Two level 1 trainings at (6,5) and (7,5) put units next to the neck that later decided the game. Material went from 189 vs 203 to 236 vs 243 at ply 15.

## Middle game

Plies 13 to 27.

- **Red, bad, ply 14.** It built a tower at (5,6) and moved units into a pocket at (4..6, 5..6), leaving its only link, (4,4), (5,4) and (6,4), empty. Blue's units stood next to that neck. At ply 16 red had lost 6 active cells (27 to 21), 3 units (army 60 to 30) and 84 material of lead.
- **Blue, good, ply 15, but a bad follow-through.** The cut was the largest single gain of the game: 5 red cells cut off, 30 gold of red units killed (cum_killed_cut 30 at ply 16). But blue moved its units off (6,5) and (7,5), so red could train onto those empty cells at ply 16. Blue then had 6 gold and three cut-off cells (row 4) at ply 17.
- **Red, good, ply 16.** Level 1 trainings on (6,5) and (7,5) reconnected the pocket (cut_off 5 to 0 by ply 17), cut blue's three cells, and gave red a 45 lead at ply 18.
- **Blue, bad, ply 17.** It spent 0 with 26 gold, moved three units away, and left its cut-off cells to die. At ply 19 it still had 46 gold and spent 0 again.
- **Red, good, ply 20.** Two mines at (1,0) and (4,1) pushed income from 29 to 41 at ply 21 (mines 0 to 2). By ply 23 red had three mines and income 46.
- **Blue, bad, ply 21.** It spent 55 of its 67 gold on two towers at (8,6) and (6,8), one mine and one unit. Income stayed at 21 (27 at ply 22) against red's 41. Its lead fell from -75 at ply 21 to -96 at ply 23.
- **Blue, bad, ply 29.** Another 55 gold went to three towers at (9,9), (9,11) and (6,9) and one unit, in the rear. Income was 38 against red's 32 after red's level 3 upkeep, but blue's lead fell from -85 to -130 by ply 31. Blue also left its third mine spot empty to the end (free_spots 1).

## Endgame

Plies 28 to 36.

- Red banked gold at plies 24 to 26 (kept 100%, 52 to 99 gold) and then trained a level 3 at (6,7) at ply 28 (killing a blue level 1). Its upkeep rose from 8 to 28 and its income fell from 49 to 32, a real cost that red could pay.
- **Red, bad, ply 32.** Red had 183 gold and hq_gap 0, so its metrics said it could already pay a chain to blue's HQ. It waited. Its lead fell from 172 to 137 over blue's turn.
- **Red, bad, ply 34.** The same again with 236 gold and hq_gap 0. It waited, and the lead fell from 170 to 132.
- **Red, good, ply 36.** Ten trainings for 230 gold, with level 3s on the cells next to blue's towers ((7,6), (7,9), (7,10), (8,11), (9,11), (11,11)). The game ended on blue's HQ.
- Blue never had a level 3 and never got its hq_gap below 73. At ply 35 it had 97 gold and spent 10.

## Decisive moments

- **Ply 16 (swing, favours blue): explained.** It came from red's ply 14 hole and blue's ply 15 cut. Blue's gain was real but was undone by its own emptied cells.
- **Ply 18 (lead change, favours red): explained.** Red's ply 16 trainings on the empty link cells did it, and blue's ply 17 spent nothing.
- **Ply 32 (HQ threat, favours red): explained.** The metrics showed red could pay a chain, but red waited two turns (plies 32 and 34). The win came one turn later than it could have.
- **Ply 20 (reviewer's pick).** Red's two mines were the economic turning point: income 29 to 41 in one turn, while blue's 55 gold on towers at ply 21 gave no income.

## Lessons

**v022 (blue):**
- Do not move units off the cells that link your cut-off pockets to the HQ. Check that a cut you make will still hold next turn.
- Spend gold in the turn you get it. Plies 17 and 19 banked 26 and 46 gold with no return, and mines paid back faster than towers.
- Limit towers to the cells red can actually reach. Towers do not add income, and a level 3 passes them. Keep level 2 or 3 units near the HQ approach, where red's chain had to pass.

**zasmu (red):**
- Do not leave a neck empty behind a pocket you have just extended (ply 14).
- When your metrics show a chain to the enemy HQ is affordable (hq_gap 0), take it that turn, not later.
- Mines first: two mines at ply 20 paid for the level 3 army later.

```json
[
  {"ply": 12, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (5,6) and (6,6) took red from 17 to 23 cells; lead +9 by ply 14."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Trainings on (6,5) and (7,5) set up the neck for the ply 15 cut; lead -14 to -7 by ply 15."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Tower and units into a pocket with its link cells (4,4), (5,4), (6,4) left empty; blue cut 5 cells at ply 15 and the lead went to -84 at ply 16."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "The cut gained 84 at once, but its moves emptied (6,5) and (7,5), which red retook at ply 16; blue was left with 6 gold and three exposed cells."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Level 1 trainings onto (6,5) and (7,5) reconnected the pocket (cut_off 5 to 0) and cut blue's three cells; lead +45 by ply 18."},
  {"ply": 17, "side": "blue", "verdict": "bad", "note": "Spent 0 with 26 gold; the three cut-off cells were lost and the lead fell from -26 to -52 by ply 19."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Two mines at (1,0) and (4,1); income 29 to 41 at ply 21, lead +75 to +96 by ply 23."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Spent 55 of 67 gold on two towers, one mine and one unit; income stayed at 21 against red's 41 and the lead fell to -96 by ply 23."},
  {"ply": 29, "side": "blue", "verdict": "bad", "note": "Spent 55 on three rear towers and one unit with no income gain; lead fell from -85 to -130 by ply 31."},
  {"ply": 32, "side": "red", "verdict": "bad", "note": "Waited with 183 gold and hq_gap 0; the lead fell from 172 to 137 and the chain was only played at ply 36."},
  {"ply": 34, "side": "red", "verdict": "bad", "note": "Waited again with 236 gold and hq_gap 0; the lead fell from 170 to 132."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Ten trainings for 230 gold, level 3 where needed next to blue's towers, ended on blue's HQ and won the game."}
]
```
