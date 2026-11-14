# Review of blue-replay-921451652: zasmu (red) beat v022 (blue)

Winner: zasmu, as red, by the rules' end after 20 turns (39 plies). The file does not show the final board or the exact end cause.

## Why the game was won

Red grew faster from turn 7 on, banked gold through turns 12 to 14, then spent 140 gold on nine trainings at ply 28 that cut v022's network from 36 active cells to 4 (ply 29). Red's material lead went from +148 to +351 in that one turn. v022 had sat on 46 to 79 gold from ply 19 to 27, with mines unbuilt and no reserve in place on its thin spine, so it could not answer in time.

## Early game (plies 0 to 12)

zasmu:
- Good, ply 12: two level 1 trainings at (5,6) and (6,6) plus four moves into the centre. Cells went 17 to 23 (ply 12 to 13), income 13 to 17, voronoi 49 to 52. Its material lead moved from -7 to +14 by ply 13.

v022:
- Even, plies 2 to 10: it held or led in cells (ply 8: 11 against 10; ply 10: 15 against 13) and income (ply 10: 11 against 10).
- Weak, ply 12: it let red take the centre. Its voronoi share fell from 49 to 45 by ply 13.

## Middle game (plies 13 to 27)

v022:
- Good, ply 13: two level 1 trainings at (6,5) and (7,5) plus moves. Cells went 19 to 24 and the material gap closed from 14 to 2 (ply 14).
- Bad, ply 15: it trained into (4,4) and (5,4) and cut five red cells (red cells 27 to 21, cut_off 5 at ply 16). Material was 272 against 188 at ply 16. But the wedge was a single row of cells. Red cut it back at ply 16, and v022 had 3 cut-off units at ply 17 and no answer (spent 0 with 26 gold). Red led by 26 at ply 17 and by 45 at ply 18.
- Bad, ply 19: it had 46 gold, 2 free mine spots and kept 100 percent of its gold. It built nothing. Red's income was 29 against v022's 20, and the lead went from -52 to -74 by ply 20.
- Bad, ply 21: it spent 55 on two towers, one mine and one unit, but its material gap widened from -75 to -111 by ply 22. Its income was 21 against red's 41.
- Bad, ply 27: it kept 79 gold while its cheapest chain to red's HQ cost 91 (red's cost was 51). Red's can_cut was 4, so the threat was visible.

zasmu:
- Good, ply 16: two level 1 trainings on (6,5) and (7,5) and moves retook the pocket. Red's cells went 21 to 31 and cut_off went 5 to 0.
- Good, ply 20 to 23: two mines on its home cells at (1,0) and (4,1), then a third at (6,4). Income rose from 29 to 46 by ply 23 and red banked gold from ply 24 onward (52 to 99 by ply 27).

## Endgame (plies 28 to 38)

zasmu:
- Good, ply 28: nine trainings, including two level 3 units at (6,7) and (8,7), took the spine through (7,7) to (9,9). Blue's active cells went from 36 to 4 and cut_off to 23. Red's material went from 575 against 427 to 488 against 137. Its cost was upkeep: 58 at ply 29, so income fell to 8.
- Good, ply 34: 20 gold of trainings and moves cut blue to 22 cells (ply 35), and the lead went from +154 to +248.

v022:
- Good, ply 29: it spent 75 on three level 2 trainings at (8,8), (9,9) and (10,9), taking back the links to its cut-off group. Cells went from 4 to 30 by ply 30, and the lead from -351 to -116. This was the right answer, but it came one turn late and was paid with gold it could have spent before the blow.
- Good, ply 31: a level 1 training at (7,7) retook a cell, and the lead went from -162 to -136.
- Bad, ply 35: it spent 40 (a level 3 at (6,6) and a level 1 at (6,7)). The lead went from +248 to +256 by ply 36, so it bought nothing.

## Decisive moments

- Ply 16 (swing, favours blue): correct as a result, but the cause was the single-row wedge at ply 15. It was not held, so the swing did not last.
- Ply 18 (lead change, favours red): red's recapture at ply 16 and its trainings at ply 17 caused it.
- Ply 28 (large cut, favours red): the decisive moment. Red's gold reserve and the spine it cut explain it.
- Ply 36 (swing, favours red): a consequence of the cut at ply 28, not a cause. The lead moved 102 but the endgame was already settled.
- Ply 19 (reviewer's pick): v022's gold stayed idle with two free mine spots. Red's mines and income were the difference from here.

## Lessons

v022:
1. Build mines as soon as gold allows. At ply 19 it had 46 gold and 2 free spots and built nothing until ply 21. Red had 3 mines by ply 23 and income 46 against 27.
2. Guard the spine. The network was a thin diagonal line from the HQ; one cut at ply 28 removed 32 cells. Keep units on the joints that red can reach (can_cut was 4 at ply 27).
3. Keep a reserve for counter-trainings. The 79 gold at ply 27 could have bought the level 2 units before the blitz instead of after it.

zasmu:
1. Early home mines and banking for one concentrated strike worked. Repeat both.
2. Watch upkeep after a big purchase. Its 17 units cost 58 upkeep, which cut income to 8 at ply 29.

```json
[
  {"ply": 12, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (5,6) and (6,6) plus moves took the centre; cells 17 to 23 and the lead moved from -7 to +14 by ply 13."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "Two level 1 trainings at (6,5) and (7,5) plus moves; cells 19 to 24 and the gap closed from 14 to 2 by ply 14."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "Cut five red cells through a single row at (4,4) to (6,4), but red cut it back at ply 16 and the lead was -26 for blue at ply 17."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Two level 1 trainings at (6,5) and (7,5) retook the pocket; cells 21 to 31 and the lead moved from -84 to +26."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Kept 46 gold, 100 percent unspent, with 2 free mine spots; the lead went from -52 to -74 by ply 20."},
  {"ply": 21, "side": "blue", "verdict": "bad", "note": "Spent 55 on towers and a mine, but the lead fell from -75 to -111 by ply 22 while red's income was 41."},
  {"ply": 27, "side": "blue", "verdict": "bad", "note": "Kept 79 gold with a cheapest chain of 91 and red's can_cut at 4; red's 140-gold strike followed at ply 28."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Nine trainings on blue's spine cut blue from 36 active cells to 4; the lead went from +148 to +351 by ply 29."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Three level 2 trainings took back the links at (8,8), (9,9) and (10,9); cells 4 to 30 and the lead from -351 to -116 by ply 30."},
  {"ply": 30, "side": "red", "verdict": "good", "note": "Moves and a tower at (6,6) kept the lead; it moved from +116 to +162 by ply 31."},
  {"ply": 34, "side": "red", "verdict": "good", "note": "Twenty gold of trainings and moves cut blue to 22 cells; the lead moved from +154 to +248 by ply 35."},
  {"ply": 35, "side": "blue", "verdict": "bad", "note": "Spent 40 on a level 3 and a level 1 for no gain; the lead went from +248 to +256 by ply 36."}
]
```
