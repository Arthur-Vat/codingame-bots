# Review of blue-ranking-921364673 (zasmu red vs v022 blue)

Red (zasmu) won by taking blue's HQ at ply 30, in 16 turns.

## Why the game was won

Red kept spending its income on mines and towers from ply 16 on, so its lead grew each turn it spent. Blue had two free mine spots on its own land from ply 13 but built no mine until ply 23, and it kept 100% of its gold from ply 17 to ply 22 (43, 65, 86, then 108 at ply 23). When it finally spent 90 at ply 23 on two level 3 units and two mines, the level 3 upkeep (20 each) pushed its income from 22 to -6 at ply 24, and the lead went from +139 to +200. Red's 170-gold chain at ply 30 then reached the blue HQ, which had no unit within 2 steps.

## Early game

Plies 0 to 12 were close. Both sides bought two level 1 units at plies 0 and 1, and the material lead stayed between +7 and +9 (ply 7: 91 to 84; ply 9: 119 to 110). Blue's cells kept pace: 19 against red's 17 at ply 12, with income 15 against 13.

- **Red, good.** Its moves into the open centre-left (ply 6: `MOVE 1 2 0;MOVE 2 1 4;TRAIN 1 2 4`) gave it voronoi 60 against blue's 40 at ply 9, and room 20 against blue's 10 at ply 13.
- **Blue, bad (small).** At ply 13 it spent 10 of 28 gold and left both mine spots (10,8) and (10,11) idle. Its room stayed at 10, so it had less neutral land to grow into than red.

## Middle game

Red's middle game was the decisive phase, plies 14 to 22.

- **Red, good.** Each turn it spent 40 or more on a mine, a tower or a unit (ply 16: mine at 1,0; ply 18: mine at 1,8, tower at 4,6 and a level 1 unit; ply 20: towers at 5,5 and 7,3; ply 22: mines and towers at 1,3, 4,7 and 4,5). Its mines went from 0 to 3 and towers from 0 to 6 by ply 23, its active cells from 23 to 47, and its lead from +31 at ply 16 to +153 at ply 22.
- **Blue, bad.** Ply 15 spent 15 on a tower at (8,5) while two mine spots stayed empty; the lead went from +17 to +42. Plies 17 and 19 spent nothing with 43 and 65 gold, and the lead went from +42 to +68 and then to +108. Ply 21 spent nothing with 86 gold. Its cells stayed at 26 or 27 from ply 15 to ply 23 while red's rose from 29 to 47. Its only tower stayed at one. Its gross income was 27 at ply 23, against red's 59.
- **Blue, bad (ply 23).** Two level 3 units at (6,5) and (6,6) took two red towers and cut five red cells off the HQ (red's cum_lost rose to 20, blue's cum_killed_cut to 20). It was a real tactical gain, but the upkeep of 40 and the mines' delay cost it the economy: income fell from 22 to -6 and gold from 108 to 18 by ply 24, and the lead rose from +139 to +200.

## Endgame

Plies 28 to 30.

- **Red, good.** Plies 24 to 28 spent little (moves only), with gold rising from 46 to 139. Red reconnected most of its cut-off cells (cut_off 5 at ply 24, 3 at ply 28), and its hq_gap fell from 104 (ply 24) to 31 (ply 28) to 0 (ply 30). At ply 30 it spent 170 on a chain of units along column 11 to the HQ at (11,11).
- **Blue, bad.** Ply 29 spent 10 on a level 1 unit at (9,3), far from the HQ. At ply 30 no blue unit stood within 2 steps of (11,11), and blue's hq_gap was 87 with 3 gold.

## Decisive moments

- **Ply 30 · turn 16 · red to play (HQ threat).** The rules' decisive moment and the game's end: red's hq_gap became 0 for the first time, so it could pay the chain. It mattered because the chain was only possible after red's plies 16 to 28 had built its income.
- **Ply 23 · turn 12 · blue to play (the rules did not list it).** Reviewer's pick. This is where blue's game broke: 90 gold on two level 3 units and two mines, with income falling to -6 in the next turn. The fight it won (20 gold killed by a cut) did not offset the lost income.
- **Ply 17 · turn 9 · blue to play (reviewer's pick).** Blue's last cheap chance to take the mines. It had 43 gold, two free mine spots and red's lead at +42, and it spent nothing.

## Lessons

**Blue (v022).**
- Build mines on free spots as soon as they appear: a mine costs 15 and returns 4 a turn, so it pays back in about four turns. Blue's two spots were idle from ply 13 to ply 23.
- Do not hoard. Gold kept at 43 to 108 for six turns was lost tempo, while red converted the same income into cells and towers.
- Level 3 units cost 20 upkeep each. Two of them on a 22-income economy drove income negative. Buy them only when the income can carry them, or prefer level 2 units for cuts.
- Keep at least one unit within 2 steps of the HQ in the late game; at ply 30 the HQ had none.

**Red (zasmu).**
- Its strengths to copy: mines and towers bought every turn, spending kept near the income, and a patient 170-gold reserve for the final chain.
- Its weakness at ply 24 to 27 was the cut-off: it rebuilt the link only slowly (cut_off 5 to 3 over five plies). Faster repair would have kept its income higher.

```json
[
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "Spent 15 on a tower at 8,5 while two mine spots stayed idle; red's lead rose from +17 to +42 by ply 17."},
  {"ply": 17, "side": "blue", "verdict": "bad", "note": "Spent 0 of 43 gold with two free mine spots; red's lead rose from +42 to +68 by ply 19."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Spent 0 of 65 gold; red's lead rose from +68 to +108 by ply 20, while red built mines and towers."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "Spent 90 on two level 3 units and two mines; upkeep cut income from 22 to -6 and the lead rose from +139 to +200."},
  {"ply": 25, "side": "blue", "verdict": "good", "note": "Least bad option with 12 gold and negative income; its moves held the cut area, and the lead rose only from +220 to +207 in the next turn."},
  {"ply": 29, "side": "blue", "verdict": "bad", "note": "Spent 10 on a level 1 unit at 9,3, far from the HQ; the HQ had no guard when red's chain arrived."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Mine at 1,0 next to its HQ and moves; the lead rose from +31 to +59 by ply 18."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Spent 40 on a mine, a tower and a level 1 unit; the lead rose from +59 to +108 by ply 20."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Spent 40 on two towers and a level 1 unit; the lead rose from +108 to +153 by ply 22."},
  {"ply": 22, "side": "red", "verdict": "good", "note": "Spent 45 on a mine and two towers; the lead rose from +153 to +200 by ply 24."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "Held 139 gold while its hq_gap fell from 31 to 0; the lead rose from +231 to +280 by ply 30."},
  {"ply": 30, "side": "red", "verdict": "good", "note": "Spent 170 on a chain of units to blue's HQ at 11,11 and won the game."}
]
```
