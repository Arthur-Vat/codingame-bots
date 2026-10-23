# Review of ice-and-fire-151-p0-0

ice-and-fire-v004 won as red, ended by the rules at ply 30 (turn 16) after red's trainings reached blue's HQ; ice-and-fire-v003 played blue. The file does not show the board after ply 30.

## Why the game was won

The opening was even: the material lead stayed within about ±10 until turn 7 and changed sign six times. The game turned in the middle game on cuts. Blue cut two red units off at ply 19 and red cut five of its cells off at ply 21, but red reconnected at ply 22 with two trainings. At ply 24 red spent 30 of its 32 gold on three level 1 trainings at (8,6), (9,6) and (10,6), the empty blue corridor cells, which took them. Blue's cells fell from 42 to 11 (cut_off 1 to 28) and its material from 361 to 91 (ply 24 to ply 25). Blue had 3 gold and could not reconnect; red's material lead then grew to 535 by ply 30.

## Early game

Plies 0 to 13 (turns 1 to 7). Both sides opened with two level 1 trainings and then expanded.

- Red, good, ply 6: `MOVE 1 3 1;MOVE 2 2 3;TRAIN 1 3 3` took cells from 7 to 10 and gave red the lead, 73 to 91 material (ply 6 to ply 7).
- Blue, good, ply 9: four moves and one training grew its cells from 11 to 15 and its material from 110 to 135 (ply 9 to ply 10).
- Red, good, ply 10: `BUILD MINE 1 0` plus three moves lifted income from 10 to 17 and cells from 13 to 16; material went 129 to 147 and red led again at ply 11.
- Blue, good, ply 11: moves only, cells 15 to 19, material 146 to 174 by ply 12. Blue led at every turn-7 check.

The openings did not separate the sides: both reached 19 to 24 cells by turn 8, and the lead at ply 14 was blue's 16. The file has only this one game, so it cannot show whether v003's opening differs.

## Middle game

Plies 14 to 24 (turns 8 to 13).

- Blue, good, ply 19: `TRAIN 1 8 2;TRAIN 1 7 2;TRAIN 1 6 2;TRAIN 1 6 3;MOVE 3 6 7` (40 gold) cut red's (7,3) and (8,3) units off. Red's units fell from 7 to 5 and army from 90 to 60; the lead swung from red +35 (ply 19) to blue +14 (ply 20).
- Red, bad, ply 18: `TRAIN 1 8 3;TRAIN 1 6 5` gained 18 material, but the group at (7,3) and (8,3) hung on a link that blue could take with its 40 gold, and ply 19 did exactly that.
- Red, bad, ply 20: three level 1 trainings at (6,6), (7,6) and (7,7) took cells from 28 to 36 (material 277 to 324), but the group hung on the cells at (5,4) and (6,4). Blue trained onto them at ply 21 and cut five more cells off; red's units fell from 8 to 3 and its material from 324 to 243 (ply 21 to ply 22).
- Blue, good, ply 21: the trainings at (5,4) and (6,4) with a move chain were the cut. Blue's material went from 259 to 309.
- Red, good, ply 22: `TRAIN 1 3 6;TRAIN 1 4 6` plus moves reconnected the five cut cells: cut_off 5 to 0, cells 26 to 35, material 243 to 300.
- Blue, bad, ply 23: `TRAIN 1 5 2;TRAIN 1 5 3` and six moves took five red cells back (material 304 to 361 by ply 24), but it moved its units away from the corridor (8,6) to (10,6). Those cells were empty when red's trainings arrived at ply 24, and blue was left with 3 gold.
- Red, good, ply 24: three trainings at (8,6), (9,6) and (10,6) (30 gold) took the corridor. Blue's cut_off went from 1 to 28 and its cells from 42 to 11.

## Endgame

Plies 25 to 30 (turns 13 to 16).

- Blue, bad, ply 25 and ply 27: with 14 and 11 gold, blue trained single units (`TRAIN 1 9 8`, `TRAIN 1 11 10`) that could not reconnect 28 cut cells. Its cells fell to 8 by ply 27 and to 2 by ply 29. The game was already lost.
- Red, good, ply 26: `TRAIN 1 8 7;TRAIN 1 8 8;TRAIN 1 8 9` and six moves took the centre column: cells 35 to 44, material 340 to 394.
- Red, good, ply 28: four trainings and eight moves: cells 44 to 57, material 432 to 511, blue's cells 8 to 2 by ply 29.
- Red, good, ply 30: the last two trainings, `TRAIN 1 10 11` and `TRAIN 1 11 11`, next to and onto blue's HQ, with hq_gap 0 at its start and 49 gold in hand.

## Decisive moments

- Ply 24 (large cut, the rules' pick): correct. It is the moment the game was decided: blue's cells fell from 42 to 11 and red led by 217 material at ply 25.
- Ply 26 (lead change, the rules' pick): correct, but it only recorded the result of ply 24. The lead had already changed.
- Ply 28 and ply 30 (the rules' picks): the same process finishing the game; red's cuts and HQ threat were already available.
- Ply 19 (reviewer's pick): blue's four trainings and move cut two red units and swung the lead by 49 in blue's favour. The rules did not flag it.
- Ply 21 (reviewer's pick): blue's trainings at (5,4) and (6,4) cut five red cells and ten active cells; red lost 81 material in one turn. Red's reconnection at ply 22 is why this did not decide the game.

## Lessons

ice-and-fire-v004 (red):
- Do not hang a group of units on one link that the enemy can reach with trainings. Check the link cells (plies 18 and 20) before placing units beyond them.
- Keep trainings for cutting blue's corridor. The ply 24 play, with 30 of 32 gold, was the one that decided the game.

ice-and-fire-v003 (blue):
- When the enemy has gold for a cut, keep a unit on the cells that link your network to the HQ (ply 23). An empty corridor cost 31 cells in one turn.
- Keep a reserve of 10 to 20 gold after a cut-off move so that reconnection training is possible (ply 24, when blue had 3 gold).

## Reviewer's note on the openings

The v004 opening weights are not separable from this game: red's opening (two level 1 trainings at ply 0, a mine at ply 10) produced a small lead at ply 7 and ply 9 that blue erased by ply 12. The difference the batch is about would need the v003 game to compare.

```json
[
  {"ply": 6, "side": "red", "verdict": "good", "note": "Moves and a level 1 training took cells from 7 to 10; material 73 to 91 at ply 7."},
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Four moves and a training grew cells from 11 to 15; material 110 to 135 at ply 10."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "Mine at (1,0) plus moves: income 10 to 17, material 129 to 147 at ply 11."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Gained 18 material, but left the (7,3)-(8,3) units on a link blue cut at ply 19 (units 7 to 5)."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Four trainings and a move cut two red units; lead swung from red +35 to blue +14."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "Three trainings took cells 28 to 36, but the group hung on (5,4) and (6,4); blue cut five cells at ply 21."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Trainings at (5,4) and (6,4) cut five red cells; red material 324 to 243."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "Two trainings and moves reconnected the five cut cells; material 243 to 300 at ply 23."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "Cut five red cells, but moved units off the (8,6)-(10,6) corridor; red cut 31 blue cells at ply 24."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "Three trainings at (8,6), (9,6), (10,6) cut blue from 42 to 11 cells; material 247 to 308."},
  {"ply": 26, "side": "red", "verdict": "good", "note": "Three trainings and six moves took cells 35 to 44; material 340 to 394."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Four trainings and eight moves took cells 44 to 57; material 432 to 511."}
]
```
