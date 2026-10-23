# Review of ice-and-fire-151-p1-0

Winner: ice-and-fire-v003 as blue, which ended the game on its turn 19 (ply 37) with a training chain into red's HQ at (0, 0). Red was ice-and-fire-v004. The game lasted 38 plies.

## Why the game was won

Blue's 60-gold turn at ply 19 (banked from ply 17) cut red's east group off its HQ by training onto the one-cell neck at (3,4) and (3,5). Red's cells fell from 32 to 18, its army from 90 to 10 (units on cut cells die at the next turn), and blue's material lead went from +1 to +170 (plies 19 and 20). Red had left the neck cell (3,5) empty at ply 18 and spent its gold on a tower inside the group that was then cut off. Red retook the link twice, but blue cut it again from beside it (plies 21 and 23), and red's income fell from 25 (ply 19) to 11 (ply 36) while blue reached 96 cells and 64 income.

## Early game

Both bots trained two level 1 units on turn 1 (red at ply 0, blue at ply 1) and grew at similar rates. Material stayed within 12 of each other until ply 13. The map gave blue the larger start share (voronoi 58 against 48 at ply 0), and red's share caught up by ply 13 (56 against 50). The file does not show v004's per-map weights, so I compare only the boards: red's units moved west and south, blue's moved toward the centre. This opening does not decide the game.

Red:
- Good, ply 10 (turn 6): BUILD MINE 1 0 for 15 gold. Income went from 10 to 16 by ply 11, with two new cells. It was red's only mine for the whole game.
- Bad, ply 12 (turn 7): three moves and no training, with 18 gold unspent. Cells rose from 15 to 18, but blue took six central cells on ply 13 (19 to 25 by ply 14), and blue's lead went from +18 to +29.

Blue:
- Good, ply 11 (turn 6): four moves took cells from 15 to 19, and blue's lead went from +6 to +18 by ply 12.

## Middle game

Red:
- Good, ply 14 (turn 8): TRAIN 2 6 4 took blue's level 1 at (6,4), cells went from 18 to 22, and the lead went from +29 to +24 for blue by ply 16.
- Good, ply 16 (turn 9): three level 1 trainings for 30 gold took cells from 22 to 27 and material from 231 to 257.
- Bad, ply 18 (turn 10): MOVE 2 3 6 left the neck cell (3,5) empty, and BUILD TOWER 5 4 (15 gold) sat inside the east group rather than on the neck. Red's exposed50 read 7 against blue's 38 gold, but blue's 23 income made it 61 on its next turn. By ply 20 the lead was +170 for blue.

Blue:
- Good, ply 19 (turn 10): five trainings for 60 gold (TRAIN 1 4 7, TRAIN 1 4 6, TRAIN 2 3 6, TRAIN 1 3 5, TRAIN 1 3 4) cut red's east group. Red's cells fell from 32 to 18 and its material from 305 to 168. Blue's can_cut was 14 at ply 19.

## Endgame

Red:
- Bad, ply 20 (turn 11): TRAIN 2 3 4 retook (3,4), and cells rose from 18 to 28 by ply 21. It left blue's (3,5) on the flank, so blue's ply 21 trained onto (2,5) and (2,4) and cut red again (28 to 15 cells, cut_off 6). Lead went -96 for red over its next turn.
- Bad, ply 22 (turn 12): TRAIN 2 2 4 recaptured (2,4), and cells rose from 15 to 22. The east cells still hung on (3,4), and blue's ply 23 cut them again (22 to 14 cells by ply 24). Lead went -75 for red over its next turn.
- Bad, ply 36 (turn 19): WAIT with 34 gold. Blue's HQ chain cost 60 (hq_gap 53 at ply 36) and blue's income was 64, so the chain was affordable on ply 37. A level 3 (30 gold) on blue's B2 at (0,3) was within red's gold. By my count of the chain's costs, blue would then have needed a second level 3 on red's tower at (1,2), about 90 gold against its 71. This is my reading of the rules, not a checked line.

Blue:
- Good, ply 21 (turn 11): two level 1 trainings at (2,5) and (2,4) cut the link again, and blue's lead went from +111 to +266.
- Good, ply 23 (turn 12): two level 1 trainings on (3,4) and (4,4), plus a tower at (2,5), cut red to 14 cells. Lead went from +236 to +354 by ply 25.
- Good, ply 37 (turn 19): TRAIN 3 0 2, TRAIN 2 0 1, TRAIN 1 0 0 (60 gold) reached red's HQ.

Not judged: red's WAITs at plies 28, 32 and 34 with 12 to 23 gold, while blue's income rose from 45 to 55.

## Decisive moments

- Ply 19 (the rules' large cut): this decided the game. Ply 20 is its swing.
- Ply 21 and ply 23 (large cuts): red's recaptures held one turn each. The later swings at plies 22, 24 and 26 to 34 came from the same link. From ply 26 on, red's HQ group had 9 to 11 cells.
- Ply 37 (HQ threat): the end of the game.
- Ply 18 (reviewer's pick): the setup. The neck cell was left open and the 15 gold went into the wrong place, a turn before the cut. The rules did not flag it because the cut was not yet visible.
- Ply 36 (reviewer's pick): red's last chance to hold the HQ with the gold it had.

## Lessons

Red:
1. Keep a unit on or beside the neck (3,4) to (3,5) whenever blue can train near it. Judge exposure with the enemy's next-turn gold (current gold plus income), not only its current gold.
2. Spend on the chokepoint, not inside the group that depends on it. The tower at (5,4) was cut off within two plies.
3. Keep about 30 gold for a level 3 near the HQ once blue's income passes 40, as the HQ-side towers at (0,2) and (1,2) raised the cost but red had no reserve to answer the chain.

Blue:
1. Keep the bank-then-strike cycle. Its 38 gold at plies 17 and 18 paid for the 60-gold cut at ply 19.
2. Build mines earlier. Blue had no mine until ply 35, although it held 38 gold at plies 17 and 18. Each 15-gold mine adds 4 income per turn.
3. Keep at least one of its own cells beside the neck each turn. The cuts at plies 19, 21 and 23 all came from there.

```json
[
  {"ply": 10, "side": "red", "verdict": "good", "note": "BUILD MINE 1 0 for 15 gold; income 10 to 16 by ply 11, red's only mine all game."},
  {"ply": 11, "side": "blue", "verdict": "good", "note": "Four moves took cells from 15 to 19; lead +6 to +18 for blue by ply 12."},
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Moves only, 18 gold unspent; blue took six central cells on ply 13 and its lead went +18 to +29."},
  {"ply": 14, "side": "red", "verdict": "good", "note": "TRAIN 2 6 4 captured blue's level 1 at (6,4); cells 18 to 22."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Three level 1 trainings for 30 gold: cells 22 to 27, material 231 to 257."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Left neck cell (3,5) empty and put a 15-gold tower inside the group that was cut; read 50-gold exposure 7 against blue's 38 gold, with 61 due next turn."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Five trainings for 60 gold cut red's east group: cells 32 to 18, material 305 to 168 for red; lead +1 to +170 for blue."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "TRAIN 2 3 4 retook (3,4) but left blue's (3,5) on the flank; blue cut again on ply 21; lead -96 for red over its next turn."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Two level 1 trainings at (2,5) and (2,4) cut red again: cells 28 to 15; lead +111 to +266."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "TRAIN 2 2 4 recaptured (2,4) but the east cells still hung on (3,4); blue cut them on ply 23; lead -75 for red over its next turn."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Two level 1 trainings on (3,4) and (4,4) plus a tower at (2,5): red cells 22 to 14; lead +236 to +354."},
  {"ply": 36, "side": "red", "verdict": "bad", "note": "WAIT with 34 gold while blue's 60-gold HQ chain was affordable next turn; a level 3 on blue's (0,3) was within reach."},
  {"ply": 37, "side": "blue", "verdict": "good", "note": "TRAIN 3 0 2, TRAIN 2 0 1, TRAIN 1 0 0 (60 gold) reached red's HQ and ended the game."}
]
```
