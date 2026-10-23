# Review of ice-and-fire-151-p2-1

v003 (red) beat v004 (blue) as red, by the rules' HQ win at ply 54, in 28 turns (55 plies). The file has no final board; the last command was `TRAIN 3 11 11` onto blue's HQ at (11, 11), with red's `hq_gap` at 0.

## Why the game was won

The lead (red material minus blue) stayed between -7 and +9 until ply 14. Red then won through two concentrated Level 2 strikes: ply 22 (70 gold, lead +51 to +96 at ply 23) and ply 32 (60 gold, blue cells 27 to 18, cut_off 4, lead +150 to +193 at ply 33). Ply 24 added a mine capture at (10,5), which cut blue's income from 21 to 15. Blue left its gold idle (42 at ply 21, 29 at ply 31) and its exposed cells stayed open to strikes it could see coming.

## Early game

Not reached as a decisive phase; plies 0 to 13 stayed level. Both sides bought two Level 1 units on turn 1 (red at (1,0) and (1,1) at ply 0; blue at (10,10) and (10,9) at ply 1) and built a mine on turn 6 (red at ply 10 on (1,0); blue at ply 11 on (10,5)).

- Red, good, ply 10: mine on (1,0). Income went from 10 to 17 by ply 11, and the lead from -2 to +4 at ply 12.
- Blue, good, ply 9: three moves took cells from 11 to 14 and income from 8 to 11 (lead -9 to -5 at ply 11).

## Middle game

- Red, good, ply 14: three Level 1 trainings on (6,5), (7,5), (7,6). Red cells 19 to 24, voronoi 33 to 40 (blue 36 to 29), lead +6 to +33 at ply 15.
- Blue, good, ply 15: `TRAIN 2 7 5` took red's Level 1 there and cut off (7,6). Red cells 24 to 22, red army 60 to 40, lead +33 to +12 at ply 16. Blue spent 45 and fell to 1 gold, but its income of 20 let red's lead recover to +29 by ply 18.
- Blue, bad, ply 21: 20 gold on two Level 1 units, one on the empty cell (8,5). Red's ply 22 trained three Level 2 units onto blue-held cells (7,4), (8,4), (8,5). Blue army fell from 90 to 40 and cells from 27 to 22, and lead went from +25 to +96 at ply 23.
- Red, good, ply 22: 70 gold, army 60 to 130. Gold fell to 4 and upkeep rose from 6 to 19, so income dropped to 20. Blue had 22 gold and could not answer.
- Blue, bad, ply 23: 35 gold on a Level 2 at (8,6), a tower at (9,6) and a move. It repaired its cut-off cell and cut red's 30-gold exposure from 11 to 1, but it left its mine at (10,5) with no unit on it.
- Red, good, ply 24: `TRAIN 1 10 4; TRAIN 1 10 5` took the mine. Blue mines fell from 1 to 0, income from 21 to 15, lead from +96 to +123.
- Red's WAIT at plies 26, 28 and 30 banked gold (24, 45, 66) while income held at 21. Passive, but it set up the ply 32 strike.
- Blue, bad, ply 31: spent 10 of 29 gold. Its 50-gold exposure stayed at 7 while red's `can_cut` was 7.
- Red, good, ply 32: 60 gold, six commands. Blue cells 27 to 18, cut_off 4, blue army 80 to 50, blue income 17 to 11.

## Endgame

Not a separate contest: blue's economy collapsed (cells 18 at ply 33, 12 at ply 43, 6 at ply 45).

- Blue, bad, ply 37: spent 0 gold, moved two units. Cut_off rose to 2 and lead went from +259 to +313 by ply 39.
- Red, good, ply 44: 30 gold on two trainings and a move. Blue cells 12 to 6, cut_off 1 to 4, lead +352 to +437 at ply 46.
- Red's upkeep reached 45 at ply 33, with income near 1 and gold 27. No debt, but one bad turn would have brought it close.

## Decisive moments

- Ply 32 (rules, large cut): decisive. Blue lost nine cells and four were cut off; the lead rose by 43.
- Ply 44 (rules, large cut): confirmed the win, did not decide it. Red was already ahead by about 350.
- Ply 46 and ply 48 (rules, swings): did not decide the game; they reflect blue's collapse.
- Ply 54 (rules, HQ threat): the finishing move.
- Ply 22 (reviewer's pick): the first Level 2 strike that broke blue's line. The rules missed it because it fell outside their large-cut threshold.
- Ply 15 (reviewer's pick): blue's best play, a real counter that cut lead from +33 to +12. Missed by the rules.
- Ply 24 (reviewer's pick): the mine capture that cut blue's income by 6. Missed by the rules.

## Lessons

**v004 (blue)**
1. Do not leave Level 1 units on cells that a Level 2 unit can reach with 50 gold (ply 21). Three of them were lost at ply 22.
2. Keep a unit on each mine and income cell that the enemy can reach (ply 23). The (10,5) mine was lost at ply 24 and income fell from 21 to 15.
3. Spend idle gold to reduce exposure before the enemy has the gold to strike (plies 21 and 31, with 42 and 29 gold idle).

**v003 (red)**
1. Keep the concentrated Level 2 strikes on cut lines (plies 22 and 32); they decided this game.
2. Check income after each big strike (ply 33: income 1, upkeep 45) so the bot does not drift toward debt.
3. Do not bank gold for several turns unless the bank is for a specific strike; two WAIT turns gave blue time for a tower.

**Openings.** The file does not show the opening weights, so I judged only the commands. The two openings were the same in every way the file shows: same unit count on turn 1, same mine timing on turn 6, lead within -7 to +9 through ply 13. In this game the opening did not decide anything. One game cannot show more.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Three moves took cells from 11 to 14 and income from 8 to 11; lead went from -9 to -5 at ply 11."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "Mine on (1,0): income 10 to 17 by ply 11; lead from -2 to +4 at ply 12."},
  {"ply": 14, "side": "red", "verdict": "good", "note": "Three Level 1 trainings cut blue's (6,6) and took (7,6); red cells 19 to 24, lead +6 to +33."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Level 2 onto (7,5) cut red's (7,6) and took its units: red cells 24 to 22, army 60 to 40, lead +33 to +12."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Two Level 1 trainings, one on empty (8,5) within reach of red's Level 2 units; red overran three blue units at ply 22, lead +25 to +96 at ply 23."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "70 gold on three Level 2 trainings and a move: blue army 90 to 40, cells 27 to 22, lead +51 to +90 by ply 24."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "35 gold on a tower, a Level 2 and a move; the (10,5) mine was left unguarded and red took it at ply 24; lead +96 to +123 at ply 25."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "Two Level 1 trainings took blue's mine at (10,5): blue mines 1 to 0, income 21 to 15; lead +90 to +149 by ply 26."},
  {"ply": 31, "side": "blue", "verdict": "bad", "note": "10 of 29 gold spent while 7 cells stayed exposed to a 50-gold strike; red struck at ply 32, lead +147 to +193 at ply 33."},
  {"ply": 32, "side": "red", "verdict": "good", "note": "60 gold, six commands: blue cells 27 to 18, cut_off 4, lead +150 to +193 at ply 33."},
  {"ply": 37, "side": "blue", "verdict": "bad", "note": "Spent 0 gold, moved two units while red's cut left blue with cut_off 2; lead +259 to +313 at ply 39."},
  {"ply": 44, "side": "red", "verdict": "good", "note": "30 gold: blue cells 12 to 6, cut_off 1 to 4 at ply 45; lead +352 to +437 at ply 46."}
]
```
