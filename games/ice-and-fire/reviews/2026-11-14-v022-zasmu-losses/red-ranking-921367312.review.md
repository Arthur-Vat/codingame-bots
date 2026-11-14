# Review of red-ranking-921367312: v022 (red) against zasmu (blue)

Zasmu won as blue, by the rules' end on turn 23 (46 plies). Its last ply, 45, was a 190-gold chain of trainings, the first ply where blue could pay a chain to red's HQ (hq_gap 0).

## Why the game was won

Zasmu spent nearly all its gold every turn from ply 15 to ply 37 on towers, mines and level 3 units. v022 hoarded: kept_share was 100 at plies 14, 16, 20 and 24, with 43 gold idle at ply 16 and 48 at ply 24. Its income fell behind (16 at ply 20 against 27 for blue). At ply 21 a level 3 unit that v022 could not answer took (4,6), cut off three red cells and removed 30 gold of units. The material lead went from -44 at ply 16 to -165 at ply 24 and -388 at ply 42.

## Early game

Plies 0 to 13. Both sides trained one level 1 unit per turn at 10 gold. Blue was ahead from the start of the middle game, not because of a bad early opening by v022. Early in the phase, v022's gold piled up instead of going to units: 14 gold at ply 10, 25 at ply 12, 28 at ply 14. Blue had more active cells (11 at ply 8, against 9; 18 against 14 at ply 12), and its lead was already -12 at ply 8 and -19 at ply 12.

- Red, good: ply 0 put two level 1 units out at once, the most tempo of the opening.
- Red, bad: ply 8 and ply 10 spent nothing with 6 and 14 gold in hand (material -12 and -16 by ply 10). Red's lead then fell 16 more by ply 14.
- Blue, good: ply 7 to ply 13, a level 1 unit each turn into the middle columns (ply 7 TRAIN 1 8 7, ply 9 TRAIN 1 7 6). Blue's lead rose from -19 to +33 (its own view, ply 15).

## Middle game

Plies 14 to 21. This is where v022 lost the ground that decided the game.

- Red, bad, ply 16: WAIT with 43 gold and four level 1 units, while blue had six. Blue's level 2 at ply 17 took (5,4) (cum_lost 10 for red). Red's lead fell 36 by ply 18 (-44 to -80). A level 2 at (4,4) for 20 gold would have held the cell.
- Red, good in intent, bad in result, ply 18: two towers at (2,4) and (4,5) plus one unit. The towers made (4,6) level 3 only, which is why blue had to spend 30 on a level 3 at ply 21. But red's lead still fell 15 by ply 20 and red had 34 gold idle afterwards.
- Red, bad, ply 20: a single move (MOVE 5 4 8) with 34 gold kept. It had gold for a level 3 next to (4,6) and did not use it. Red's lead fell 8 by ply 22.
- Blue, good, ply 15: a tower at (6,6), the first of its wall. Lead rose 11 by ply 16.
- Blue, good, ply 19: a mine at (10,11) on its home spot. Lead rose 15 by ply 20; blue income went from 21 to 27.

## Endgame

Plies 22 to 45.

- Red, bad, ply 22: a tower at (4,4) with 49 gold in hand, after the cut. The three cut cells at (4,7), (5,7), (4,8) were not relinked. Lead fell 62 by ply 24 (-103 to -165).
- Red, bad, ply 24: WAIT with 48 gold and 2 units, while blue built a tower and kept growing. Lead fell 35 by ply 26.
- Red, good, ply 28: three level 1 units at (2,6), (3,6), (3,7) re-closed the south pocket. Lead rose 15 by ply 30 (-209 to -194).
- Red, bad, ply 40: a level 3 (30 gold) at (4,4). Its upkeep of 20 dropped red's income from 22 at ply 40 to 3 at ply 41 (material 304 to 251), and the lead fell 82 by ply 42. Red could no longer pay for anything at ply 44 (gold 13, income 3).
- Blue, good, ply 23: a tower at (7,4) and nine moves. Lead rose to +62 by ply 25.
- Blue, good, ply 29: a mine at (8,6) and a level 1 unit at (10,6). Lead rose 46 by ply 31.
- Blue, good, ply 45: the 9-command chain of trainings, which the file flags as the decisive move.

Zasmu's late waits at plies 39, 41 and 43 (66, 115 and 164 gold) did not cost it: its income of 49 to 50 covered them and the lead grew each turn.

## Decisive moments

- **Ply 24 (rules)**: a real swing, but it is the result of ply 21 and ply 22, not the cause. Lead -103 to -165.
- **Ply 42 (rules)**: the red level 3 upkeep from ply 40 is visible here; income 3, lead -388.
- **Ply 45 (rules)**: blue's chain was the end, not a turning point.
- **Ply 16 (reviewer's pick)**: the WAIT with 43 gold. The rules missed it because the lead moved only 36 at the next turn. This is the last cheap chance to stop blue's wall on (5,4) and (4,6).
- **Ply 21 (reviewer's pick)**: blue's level 3 at (4,6) cut three red cells and took 30 gold of units. Blue's own income fell from 27 to 10 for a turn (upkeep 30), but the trade was clearly in blue's favour. Red's exchange was -30 at ply 22.
- **Ply 40 (reviewer's pick)**: red's own level 3, which a level 2 or a tower could have replaced at less cost to income.

## Lessons

**v022 (red)**
- Spend each turn. Plies 14, 16, 20, 24 and 36 kept nearly all their gold while blue spent. Keep 10 to 20 in hand at most.
- Build the three free mine spots at home (free_spots was 3 from ply 14). Red built its first mine at ply 38. Each mine is +4 income for 15 gold.
- Answer a contested cell with a unit of the same level at once (ply 16). Keep a level 2 or towers at (4,4) and (4,6) before blue reaches them.
- Count the upkeep before buying a level 3: 20 per turn cut red's income from 22 to 3 at ply 40.

**zasmu (blue)**
- Keep the tempo it had from ply 15: towers on the contested edge, mines on the home spot, and level 3 units for the cut. Its spending matched its income each turn.
- Its level 3 at ply 21 cost 30 gold and 20 upkeep. It worked here because red had no answer; against a player who holds a level 2 at (4,4) it would cost more.

```json
[
  {"ply": 16, "side": "red", "verdict": "bad", "note": "WAIT with 43 gold and 4 level 1 units; blue took (5,4) at ply 17 and red's lead fell 36 by ply 18."},
  {"ply": 18, "side": "red", "verdict": "bad", "note": "Two towers were the right idea, but the lead fell 15 by ply 20 and 34 gold stayed idle afterwards."},
  {"ply": 20, "side": "red", "verdict": "bad", "note": "One move with 34 gold kept; blue's level 3 at (4,6) came at ply 21 and the lead fell 8 by ply 22."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "A tower at (4,4) with 49 gold, while the cut cells were not relinked; lead fell 62 by ply 24."},
  {"ply": 24, "side": "red", "verdict": "bad", "note": "WAIT with 48 gold and 2 units; lead fell 35 by ply 26."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Three level 1 units at (2,6), (3,6), (3,7) re-closed the south pocket; lead rose 15 by ply 30."},
  {"ply": 40, "side": "red", "verdict": "bad", "note": "A level 3 for 30 gold: upkeep cut income from 22 to 3 at ply 41; lead fell 82 by ply 42."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Tower at (6,6) started the wall; lead rose 29 by ply 17."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Level 3 at (4,6) cut three red cells and killed 30 gold of units; blue's lead rose 3 by ply 23 despite its own upkeep."},
  {"ply": 23, "side": "blue", "verdict": "good", "note": "Tower at (7,4) and nine moves; lead rose 62 by ply 25."},
  {"ply": 29, "side": "blue", "verdict": "good", "note": "Mine at (8,6) and a level 1 at (10,6); lead rose 46 by ply 31."},
  {"ply": 45, "side": "blue", "verdict": "good", "note": "The 190-gold chain of trainings that ended the game."}
]
```
