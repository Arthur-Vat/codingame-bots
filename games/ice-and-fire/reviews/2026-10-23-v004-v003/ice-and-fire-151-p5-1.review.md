# Review of ice-and-fire-151-p5-1

Blue (ice-and-fire-v004) won against red (ice-and-fire-v003) by the rules ending the game on blue's ply 25, in 13 turns (26 plies). The file gives only the result; the likely cause is the HQ capture, since blue's last ply trained onto (1,0) and then (0,0), red's HQ, and hq_gap was 0 at its start.

"Lead" below means red material minus blue material at the start of a ply, so negative means blue leads.

## Why the game was won

Blue cut red's one-cell links with level 1 trainings in every middle and late turn from ply 15 on, while red's gold ran short. Red's cells fell from 24 (ply 15) to 18 (ply 16), 13 (ply 18) and 7 (ply 24), and its units from 6 (ply 15) to 1 (ply 18). Blue's income rose from 18 (ply 15) to 43 (ply 24). The lead went from +17 for red at ply 15 to -454 at ply 25. At ply 15 blue held 46 gold against red's 9, which decided who could strike first.

## Early game (plies 0 to 13)

Both sides opened with two level 1 trainings next to their HQ and expanded at similar rates: 10 cells for red against 11 for blue at ply 8, and 13 against 15 at ply 10. The lead stayed within 10 either way. The file does not show which opening weights v004 chose, so this game can only show what the openings did. Blue never built a mine, while red built one at ply 10. Red's mine kept its income ahead until ply 14 (20 against 19), but blue's extra cells made up the difference by ply 16 (22 against 20).

Red (v003):
- Good, ply 10: `BUILD MINE 1 0` (15 gold, red's only mine), with three moves. Income went from 10 to 17 by ply 11 (the mine's +4 and three more cells), and the lead from -6 to +1.
- No bad early play found. Its expansion was slower, 13 cells against 15 at ply 10.

Blue (v004):
- Good, ply 7: moves and `TRAIN 1 9 6` grew its cells from 8 to 11 and turned a lead of +7 for red into -4.
- Good, ply 13: it spent nothing and kept its gold (28 at ply 14, 46 at ply 15). The lead went from -4 to -12 in its favour.

## Middle game (plies 14 to 17)

Red (bad, ply 14): `TRAIN 1 7 3; TRAIN 1 8 3; TRAIN 1 8 4` spent 30 of its 39 gold to push three units forward. Their cells joined the HQ only through single cells at (4,3) and (5,3). The lead rose from -12 to +17 at ply 15, but red was left with 9 gold against blue's 46. This is the play that made the cut possible.

Blue (good, ply 15): `MOVE 6 6 5; TRAIN 1 6 4; TRAIN 1 5 4; TRAIN 1 5 3; TRAIN 1 4 3` (40 gold) took (4,3) and (5,3) and cut off red's forward group. At ply 16 red had 18 cells (from 24), 2 units (from 6) and an army of 20 (from 60). Red's material went from 231 to 181 and blue's from 214 to 258. The lead went from +17 to -77. The rules show this swing at ply 16 because they measure from ply 14; the cause is ply 15.

Red (bad, ply 16): `MOVE 2 4 4; TRAIN 2 5 4; MOVE 1 5 0` spent 20 on a level 2 at (5,4), a cell blue's next cut isolated. At ply 17 blue cut the link, and that unit died by ply 18 (units 3 to 1, army 40 to 10). Red had 29 gold and its own empty link cells (2,3) and (3,3). A level 1 unit on either would have made blue pay 20 for a level 2 to enter, not 10.

Blue (good, ply 17): `TRAIN 1 3 3; TRAIN 1 2 3` (20 gold) took the two link cells. Red's cells fell from 21 to 13 and its cut-off cells from 4 to 9. The lead went from -48 to -153.

## Endgame (plies 18 to 25)

Red (good, ply 18): `TRAIN 1 5 2; TRAIN 1 5 3; MOVE 1 4 0` (20 gold) put units on blue's cut cells, retaking nine. Cells went from 13 to 22 and cut-off cells from 9 to 3 by ply 19, and the lead from -153 to -80. This is red's best answer and shows a reconnect works, but it came too late against blue's gold.

Blue (good, ply 19): three trainings (30 gold) cut red's cells from 22 to 12. The lead went from -80 to -189. Red undid some of this at ply 20 (cells back to 19), and blue's ply 21 cut them again (19 to 12, lead -176 to -286).

Red (bad, ply 22): `TRAIN 1 2 1; MOVE 23 1 3` for 10 gold added two cells, but the five cut-off cells stayed cut. The lead went from -286 to -313 while blue's income was 38.

Blue (good, ply 23): four trainings (40 gold) cut red's cells from 14 to 7. The lead went from -313 to -411.

Red (bad, ply 24): `WAIT` with 13 gold. A level 1 unit on its empty mine at (1,0) would have made blue's entry cost 30 (a level 2, then a level 1) instead of 20. Blue held 45 gold at ply 25, so this would not have stopped the attack, but it was red's last chance to defend.

Blue (good, ply 25): `TRAIN 1 1 0; TRAIN 1 0 0` (20 gold), the attack on red's HQ that ended the game.

## Decisive moments

- Ply 16 (swing): accurate as a swing, but the cause is blue's ply 15 cut, not red's ply 16.
- Ply 17 (large cut): it matters. The lead went from -48 to -153 across that ply.
- Ply 18 (swing, favours blue): misleading as a blue gain. Its 76 is measured over plies 16 and 17. Red's ply 18 reconnect gained 73 back.
- Ply 19 and ply 21 (large cuts): both matter. Red's reconnects at ply 20 recovered only part of the first one.
- Ply 22 and ply 24 (swings): the 97 and 125 mostly come from blue's cuts at plies 21 and 23 and its income, not from red's own plays.
- Ply 23 (large cut): it matters; red's cells fell to 7.
- Ply 25 (HQ threat): the decisive moment, the first time blue could pay the chain to red's HQ.
- (reviewer's pick) Ply 14: the rules missed it because the lead rose right after it. Red spent 30 of its 39 gold and left 9, which set up the cut at ply 15.
- (reviewer's pick) Ply 13: blue's quiet turn that banked 28 gold, the reserve that paid for the 40-gold cut at ply 15.

## Lessons

Red (v003):
1. Keep a gold reserve before pushing units past a one-cell link. At ply 14 it left 9 gold against blue's 46.
2. Guard the link cells (2,3) and (3,3) with a unit, or a tower. Neither side built a tower all game (towers 0 in every table), and the rules say a cell next to an enemy tower can only be entered by level 3.
3. Reconnect at once with trainings onto the cut cells, as at ply 18, and keep some gold back for the HQ approach, as at ply 24.

Blue (v004):
1. Bank gold on quiet turns, as at ply 13, and spend it on cuts at chokepoints, as at plies 15, 17, 19, 21 and 23.
2. Build the mine at (10,11) next to its HQ. It built none in 26 plies, and the spot was still free at ply 25. A mine gives +4 income per turn for 15 gold. This did not decide the game, but it is free income.

```json
[
  {"ply": 10, "side": "red", "verdict": "good", "note": "Built the only mine (1 0) for 15 gold; income 10 to 17 by ply 11, lead -6 to +1."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Spent nothing and banked 28 gold; 46 gold at ply 15 paid for the decisive cut."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 30 of 39 gold on three forward units behind one-cell links; left 9 gold against blue's 46. Blue cut them at ply 15."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Four trainings (40 gold) took (4,3) and (5,3); red cells 24 to 18, units 6 to 2, lead +17 to -77."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Spent 20 on a level 2 at (5,4) in the cut area; it died by ply 18. Did not guard the link cells (2,3) and (3,3)."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Two trainings on the link cells (2,3) and (3,3) cut 9 more red cells; lead -48 to -153."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Trained onto blue's cut cells (5,2) and (5,3), retaking 9 cells; lead -153 to -80."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Three trainings (30 gold) cut red's cells 22 to 12; lead -80 to -189."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "Spent 10 gold on two cells; the five cut-off cells stayed cut. Lead -286 to -313."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Four trainings (40 gold) cut red's cells 14 to 7; lead -313 to -411."},
  {"ply": 24, "side": "red", "verdict": "bad", "note": "WAIT with 13 gold; a level 1 on its mine at (1,0) would have cost blue 10 more, not stopped it."},
  {"ply": 25, "side": "blue", "verdict": "good", "note": "Trained onto (1,0) and then red's HQ (0,0) for 20 gold; ended the game."}
]
```
