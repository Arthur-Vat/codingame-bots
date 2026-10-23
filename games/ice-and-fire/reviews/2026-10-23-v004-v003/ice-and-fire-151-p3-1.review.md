# Review of ice-and-fire-151-p3-1

Blue (v004) won as blue by taking red's HQ at ply 41, in 21 turns. Red is its parent, v003.

## Why the game was won

Blue won in the middle game by cutting red's land off from its HQ, not in the opening. At ply 19 blue moved a level 1 unit onto red's empty (4,3) and trained two more on (3,3) and (2,3). Red's active cells fell from 32 (ply 19) to 14 (ply 20), with 15 cut off, and its material from 306 to 152. Red had put no unit on that row-3 gap (the ply 18 board shows empty red cells there). Blue cut the link again at ply 29 by taking (1,3), leaving red 8 active cells at ply 30 and an income of 7, while blue's income reached 62 by ply 38.

## Early game

Both sides opened with two level 1 units beside their HQ (ply 0 red, ply 1 blue). The lead swung by about 10 a turn with no side ahead for long. Blue expanded faster: 11 cells to red's 10 at ply 8, 15 to 13 at ply 10, 19 to 16 at ply 12.

Red:
- Good, ply 10: `BUILD MINE 1 0` (15 gold). Income went from 10 to 17 by ply 11.
- Good, ply 12: `BUILD MINE 3 0` with three moves. Income went from 17 to 24 at ply 13, when blue had 15.

Blue:
- Good, ply 9: three moves and `TRAIN 1 9 4` took cells from 11 to 15 by ply 10 while red stayed at 13. Red's lead went from +9 to -6 in that turn.
- Good, ply 11: four moves with no spending took cells from 15 to 19 (red 16). The lead went from +1 to -10 for red.

Opening verdict: no clear edge. Blue's extra land (19 cells to 16 at ply 12) was matched by red's income, which rose after its two mines to 24 against blue's 15 at ply 13. Blue built no mine in 41 plies. The openings differ in one way that matters, economy against land. The file shows only plays, not the weights v004 chose on turn 1, so one game cannot show whether the per-map choice helped.

## Middle game

Red:
- Bad, ply 14: `TRAIN 2 8 3` (20 gold) pushed a level 2 unit onto blue's cell (8,3) with no support. Blue answered at ply 15 with 40 gold of trainings, including `TRAIN 2 7 3`. Red's army fell from 50 at ply 15 to 20 at ply 16, with two units left, and the lead went from 0 to -19.
- Bad, ply 16: three level 1 trainings on (3,6), (3,7) and (3,8) (30 gold) gave red 5 more cells (21 to 26 at ply 17) and the lead rose from -19 to -11. But no unit stood on row 3, where (2,3) to (6,3) were empty. Blue used that gap on its next turn.
- Bad, ply 18: `MOVE 14 3 9` and three trainings in the south, on (4,9), (5,9) and (6,9), for 30 gold. This took 8 cells (24 to 32) and moved the lead from -34 to +29, but none of the gold went onto the row-3 gap. Blue's ply 19 cut the land.

Blue:
- Good, ply 15: its 40 gold of trainings killed red's forward units and took (7,3). The lead went from 0 to -19 for red (blue +19), and blue's army rose from 30 to 70.
- Good, ply 17: `TRAIN 1 6 3` and `TRAIN 1 5 3` put level 1 units on red's row 3, the setup for ply 19. The lead moved 23 towards blue in that turn.
- Good, ply 19 (decisive, see below): the lead moved from red +29 to blue +168.

## Endgame

Red:
- Good, ply 20: `TRAIN 1 5 2` and `TRAIN 1 5 3` retook (5,3) and cut three blue units off blue's HQ. Blue's cut-off cells rose from 2 to 5 at ply 21, and the lead went from -168 to -134.
- Good, ply 22: `TRAIN 1 1 3` on (1,3), with `MOVE 1 4 1`, rejoined 15 cut cells. Cut-off cells fell from 17 to 2 at ply 23, cells rose from 12 to 29, and the lead went from -186 to -91. This was red's best play, but blue took (1,3) back at ply 29.

Blue:
- Good, ply 21: `TRAIN 1 6 1` and `TRAIN 1 5 1` took (5,1) and (6,1). Red's active cells fell from 16 to 12, and the lead went from -134 to -186.
- Good, ply 23: three trainings on row 0 took red's mine at (3,0). Red's income fell from 34 to 25 at ply 24, and the lead went from -91 to -160.
- Good, ply 29: `TRAIN 1 1 3`, `TRAIN 1 2 2` and `TRAIN 1 2 3` retook (1,3), (2,2) and (2,3). Red's active cells fell from 27 to 8 and 17 cells were cut off at ply 30. The lead went from 229 to 427 for blue.
- Good, ply 41 (final): `TRAIN 3 1 0` onto red's mine (1,0) and `TRAIN 1 0 0` onto red's HQ at (0,0). Blue's hq_gap was 0 here, and the rules ended the game.

## Decisive moments

- Ply 19 (large cut, blue): confirmed. This is the game. Red's material went from +29 to -168 in one blue turn.
- Ply 20 (swing, red): the swing is ply 19's cut carried forward. Red's counter at ply 20 won back 34 of it.
- Ply 28 (swing, red): not a separate play. Red's active cells fell from 27 to 22, with 4 cut off, during blue's ply 27.
- Ply 29 (large cut, blue): confirmed. This was the second cut, and it held to the end.
- Plies 30 to 40 (swings): blue's income went from 37 to 56 while red's stayed at 7 or 8. No single play decided them.
- Ply 41 (HQ threat, blue): the move that ended the game.
- Reviewer's pick, ply 22 (red): the rules list no moment in red's favour, but this was the biggest swing for red, about +95 in the lead. It held only until ply 29.
- Reviewer's pick, ply 15 (blue): the 40-gold counter turned red's forward push into a lead of 19 for blue.

## Lessons

Red (v003):
- Guard the chokepoints that split your land, here row 3 at (2,3) to (4,3) and (1,3), before spending elsewhere. At ply 18 red had 32 gold and spent it in the south.
- Do not push a level 2 unit into the enemy front without support (ply 14).
- Keep the mines from the opening. Red's income led blue's from ply 11 to ply 18.

Blue (v004):
- Build mines. Blue built none in 41 plies, and red's income was 24 to 27 against blue's 15 to 22 in plies 13 to 18. Blue had 28 gold at ply 13, enough for a mine on its own cell at (7,9).
- Keep cutting with level 1 units on one-cell chokepoints. Plies 19, 21 and 29 won the game.

```json
[
  {"ply": 10, "side": "red", "verdict": "good", "note": "BUILD MINE 1 0 raised income from 10 to 17 by ply 11; the first of two mines that later outearned blue."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "TRAIN 2 8 3 pushed a level 2 unit into blue's front without support; blue's ply 15 trainings took both forward units and red's army fell from 50 to 20."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "30 gold went to a west column for 5 cells; row 3 at (2,3) to (6,3) stayed empty and blue trained onto it at ply 17."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Took 8 cells and reached a lead of +29, but spent its gold in the south; blue cut 15 cells off at ply 19 through the empty row-3 gap."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "TRAIN 1 5 2 and TRAIN 1 5 3 cut blue's three row-3 units off from its HQ and won back 34 of the lead."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "TRAIN 1 1 3 on (1,3) rejoined 15 cut cells; the lead went from -186 to -91."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "40 gold of trainings killed red's forward level 2 and level 1 units; blue's lead went to +19."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "TRAIN 1 6 3 and TRAIN 1 5 3 put level 1 units on red's row 3, setting up the cut at ply 19."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Moved onto (4,3) and trained on (3,3) and (2,3), cutting 15 red cells off; the lead moved from red +29 to blue +168."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "TRAIN 1 6 1 and TRAIN 1 5 1 took (5,1) and (6,1); red's active cells fell from 16 to 12."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Retook the (1,3) link with level 1 units; red fell from 27 to 8 active cells and 17 were cut off at ply 30."},
  {"ply": 41, "side": "blue", "verdict": "good", "note": "Level 3 onto red's mine at (1,0), then a level 1 onto red's HQ at (0,0); the game ended."}
]
```
