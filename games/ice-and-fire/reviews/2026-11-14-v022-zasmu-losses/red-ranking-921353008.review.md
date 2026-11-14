# Review of red-ranking-921353008 (v022 red vs zasmu blue)

Zasmu won as blue, by taking red's HQ at ply 47 (turn 24), in 24 turns.

## Why the game was won

Blue out-earned red from turn 7 on: more cells, four mines by ply 24 against none until ply 32, and so an income of 49 against red's 27 at ply 25 (cells 44 vs 37 at ply 24 after red's corridor was cut). Red's gold sat idle for long stretches (35 at ply 14, 48 to 83 at plies 18 to 22, 120 to 198 at plies 40 to 46) while its spare mine spots went unbuilt. Its one big level 3 purchase at ply 26 cost 20 upkeep a turn, dropping income from 27 to 11 at ply 27. Blue saved its gold and reached red's HQ with a level 3 chain at ply 47 (hq_gap 0 with 275 gold, against red's 47 short at ply 47).

## Early game

Plies 0 to 13.

- **Red, bad: ply 12.** Red moved and trained nothing with 19 gold, while blue went from 20 to 26 cells by ply 14 (red 16 to 20). Lead went from -19 to -29 for red (material 159 vs 178 at ply 12, 187 vs 193 at ply 13).
- **Blue, good: ply 13.** One level 1 train and six moves took cells from 20 to 26 (ply 12 to 14), and blue's lead went from +6 to +21 by ply 15.

## Middle game

Plies 14 to 30.

- **Red, bad: ply 14.** Spent 0 with 35 gold. Blue's tower at (8,4) (ply 15) and mine at (9,8) (ply 17) came while red held cash: lead -13 by ply 16.
- **Red, bad: ply 18.** Spent 0 with 48 gold. Cells 30 (ply 18) against blue's 38; income 23 against 34. Lead -23 by ply 20; red's gold reached 72 at ply 20 with income 24 and blue's 42.
- **Red, good: ply 26.** Level 3 train at (3,7) (30 gold) took the corridor blue had towered at ply 25; cut 4 blue cells (blue cells 46 to 39 at ply 27). The cost was upkeep: red's upkeep 12 to 34, income 27 to 11 at ply 27, about 10 to 16 until ply 31. The lead recovered only by +5 by ply 28.
- **Red, bad: ply 30.** Spent 25 on a tower and a level 1 at (5,9), a pocket held by one cell at (3,8). Blue's ply 31 move onto (4,8) cut it: red lost 4 units at ply 32 (level 3 gone, upkeep 35 to 12), and lead fell from -109 to -156.
- **Blue, good: ply 15.** Tower at (8,4) plus moves; lead +21 at ply 15 becomes +35 at ply 17.
- **Blue, good: ply 23.** A level 2 train onto red's level 1 at (3,7) cut 3 red cells (red cells 41 at ply 23, 37 at ply 24). Red had 3 gold and could not answer: lead +50 to +112 by ply 25.
- **Blue, bad: ply 25.** Tower at (3,7), the corridor cell. A level 3 can enter a tower, and red's level 3 did so at ply 26, cutting blue's four cells. Blue's lead fell 14 by ply 27. Towers stop level 1 and 2 units, not level 3.

## Endgame

Plies 31 to 47.

- **Blue, good: ply 31.** Spent 100 on four towers and three level 1 units while red held 2 gold. Blue's lead went from +131 at ply 31 to +209 at ply 33.
- **Red, bad: ply 44.** Two towers at (0,1) and (2,1), inside its own corner, with 187 gold. Lead fell from -262 to -292 by ply 46. Red's HQ chain cost 230 against 198 gold at ply 46 (hq_gap 32). Its can_cut was 7 from ply 38 to 47 (it could take or cut seven blue cells), and it used none of it.
- **Blue, good: ply 47.** Six level 3 trains and one level 1 walked red's column 1 from blue's towers at (1,7) to (1,1) and then trained a level 3 onto red's HQ at (0,0), spending 220.

## Decisive moments

- **Ply 47, turn 24, blue HQ threat:** the rules' pick is right. Blue's hq_gap went from 47 (ply 44) to 6 (ply 45) to 0 (ply 47); red's stayed above 30 at plies 44 to 46. Red was one turn short in a race it had been losing since ply 14.
- **Ply 23:** the rules did not flag it. Blue's level 2 at (3,7) cut 3 red cells, and red's 80-gold strike at ply 22 had left only 3 gold to answer.
- **Ply 26:** (reviewer's pick) the level 3 cut that worked, but its 20 upkeep drove red's income from 27 to 11 for five turns. The cost, not the cut, is the lesson.
- **Ply 25:** (reviewer's pick) blue's tower on the corridor was the target for red's level 3.
- **Ply 44:** (reviewer's pick) red's spend on inner towers while blue's HQ race closed.

## Lessons

Red:
1. Build mines on free spots when income is cheap: red had 3 to 5 free spots from ply 20 but built its first mine at ply 32; blue had four by ply 24.
2. Do not hold 50 to 200 gold idle. Use it at turns 8 to 10 to contest the centre, and at turns 19 to 23 to cut blue's chain (can_cut 7) before blue's level 3 train closes it.
3. A level 3 is worth 20 upkeep a turn. Place it where the cut holds, not in a one-cell pocket such as (5,9).

Blue:
1. Towers do not stop level 3 units. Keep a level 3 or a second link near a tower on a corridor.
2. Keep the cash for the final chain: blue saved from ply 38 and finished with 275 gold against red's 183.

```json
[
  {"ply": 12, "side": "red", "verdict": "bad", "note": "Spent 0 with 19 gold on turn 7; blue went from 20 to 26 cells by ply 14, lead -19 to -29."},
  {"ply": 13, "side": "blue", "verdict": "good", "note": "One level 1 train and six moves took cells from 20 to 26; lead +6 to +21 by ply 15."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 0 with 35 gold; blue's tower at (8,4) and mine at (9,8) followed; lead -13 by ply 16."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Tower at (8,4) and moves; lead +21 at ply 15 became +35 at ply 17."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Spent 0 with 48 gold; cells 30 against blue's 38, income 23 against 34; lead -23 by ply 20."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Level 2 train onto red's level 1 at (3,7) cut 3 red cells; red had 3 gold to answer; lead +50 to +112."},
  {"ply": 25, "side": "blue", "verdict": "bad", "note": "Tower on the corridor at (3,7); red's level 3 entered it at ply 26 and cut 4 blue cells; lead fell 14."},
  {"ply": 26, "side": "red", "verdict": "good", "note": "Level 3 at (3,7) cut 4 blue cells (blue 46 to 39); lead +5 by ply 28, but upkeep 12 to 34 cut income 27 to 11."},
  {"ply": 30, "side": "red", "verdict": "bad", "note": "Spent 25 on a tower and a level 1 in the one-cell pocket at (5,9); blue cut it at ply 31, red lost 4 units at ply 32, lead -109 to -156."},
  {"ply": 31, "side": "blue", "verdict": "good", "note": "Spent 100 on four towers and three level 1 units while red had 2 gold; lead +131 to +209 by ply 33."},
  {"ply": 44, "side": "red", "verdict": "bad", "note": "Two inner towers at (0,1) and (2,1) with 187 gold; HQ chain 43 short; lead -262 to -292 by ply 46."},
  {"ply": 47, "side": "blue", "verdict": "good", "note": "Six level 3 trains down column 1 and a level 3 onto red's HQ at (0,0), spending 220 of 275 gold."}
]
```
