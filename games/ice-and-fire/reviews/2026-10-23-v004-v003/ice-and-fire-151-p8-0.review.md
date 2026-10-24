# Review of ice-and-fire-151-p8-0

Winner: ice-and-fire-v004, as red, over ice-and-fire-v003 (blue). The rules ended the game at ply 84, in 43 turns. The file does not show the final board; the likely finish is the HQ chain red bought that turn (hq_gap 0 at ply 84, last command TRAIN 3 11 11 on blue's HQ cell).

## Why the game was won

Blue's upkeep outran its income over the last 15 turns. It bought three level 3 units at plies 77, 79 and 81 (30 gold each, 20 upkeep each), and its income fell from +15 (ply 76) to -39 (ply 82). At ply 82 it could not pay, so its 12 units died at the turn's end (ply 83: units 0, gold 0). Red had kept its upkeep at 0 to 5 since its own debt wipe at ply 39, and held 230 gold at ply 84 for the chain that won. The material lead was misleading for most of the game; upkeep discipline decided it.

## Early game (plies 0 to 13)

Both sides opened alike: two level 1 units on turn 1 (red at ply 0, blue at ply 1), then one move-out and one unit a turn. Material stayed within about 10 (red 91 to blue 84 at ply 7, blue 189 to red 185 at ply 13), and both had 19 cells at ply 13. The per-map opening weights of v004 show no visible effect here, and one game cannot show whether they matter.

- Red, good, ply 10: BUILD MINE 1 0 on the corner spot (15 gold). Income went 10 to 17 at ply 11 and cells 13 to 16; the lead went from -6 to +18 by ply 12.
- Blue, good, ply 9: three moves and TRAIN 1 7 6 took cells from 11 to 15 by ply 10; the lead went from -9 to +8 by ply 11.
- Neither side made a clear early mistake.

## Middle game (plies 14 to 28)

- Blue, good, ply 15: TRAIN 2 5 5 displaced red's level 1 on the front cell (5,5). Blue's cells went from 22 to 26 by ply 16 and red's from 23 to 22; the lead moved from +3 to +12 by ply 17.
- Red, bad, ply 22: MOVE only, with 112 gold in hand and can_cut 4, so it could take or cut four blue cells with that gold. The lead went from -4 to -7 by ply 24. From ply 16 to ply 28 red spent only at plies 14 and 28 while its gold grew from 37 to 196.
- Blue held gold as well (46 to 108) but spent no clear mistake here.

## Endgame (plies 29 to 84)

- Red, bad, ply 32: three level 3 units (TRAIN 3 at (5,5), (6,5), (6,6)) for 90 gold. Upkeep went from 3 to 63 and income from +29 to -28 (ply 33). The lead went from -24 (ply 32) to -182 (ply 34). The drain of 24 to 28 a turn put red in debt at ply 39, when it lost its 8 units.
- Red, good, ply 36: a level 3 took blue's tower at (7,7), and four level 1 units on (7,8) to (7,11) cut the bottom-left. Blue's cells went from 38 to 17, with 17 cut off (ply 37). The lead went from -227 to -189 by ply 38. The wall was weak, as blue broke it at once.
- Blue, good, ply 37: TRAIN 2 7 9 killed the red level 1 on the wall and reconnected: blue's cells went from 17 to 33 by ply 38, and the lead from +14 to +169 by ply 39.
- Blue, good, ply 41: with red at 0 units and 0 gold, blue spent 75 on six level 1 units and a tower, taking its army from 60 to 120. The lead went from +173 to +272 by ply 43, and its upkeep rose only from 8 to 14.
- Red, good, ply 60: MOVE 32 to (8,10) and TRAIN 1 8 11 closed column 8. Blue's cells went from 29 to 11, with 19 cut off (ply 61). The lead went from -233 to -167 by ply 62.
- Blue, good, ply 61: 130 gold reconnected 27 cells (11 to 38 by ply 62), and the lead check reads positive. But the spend took upkeep from 10 to 56 and income from +30 to -10. This set up the collapse.
- Blue, bad, ply 81: a third level 3 (30 gold) with 64 gold in hand and income -22. Income fell to -39 with upkeep 94, and at the end of ply 82 the debt killed its 12 units. The lead check reads flat (-102 at ply 81, -101 at ply 83) because the wipe lifted blue's income to +55 and so raised its material from 345 to 373. The metric scores a debt wipe as a gain, so judge this play by the debt.

## Decisive moments

- Ply 34 (swing, favours blue): real. Its cause is red's ply 32 spend, not blue's play.
- Ply 36 (large cut, favours red): it mattered for one turn only, since blue reconnected at ply 37.
- Ply 39 (red debt): red's own fault from ply 32. The wipe removed 8 units and 140 army; red's material rose from 254 to 284 by ply 40 because the upkeep disappeared.
- Ply 42 (swing, favours blue): blue used a defenceless red, but its later upkeep undid part of the gain.
- Ply 46 (swing, favours blue): did not decide the game. It came from unspent gold and army at face value, while red's cells stayed at 28 to 31.
- Ply 60 (large cut, favours red): real for two turns, then blue reconnected.
- Ply 62 (swing, favours red): the rules read a lead for red, but blue's material rested on income of -10. Not a real lead.
- Ply 80 (lead change, favours red): real. Blue's income had turned to -22, and red's lead of 70 held to the end.
- Ply 82 (blue debt): decisive.
- Ply 84 (HQ threat): decisive, and the finish.
- (reviewer's pick) Ply 61: the rules miss it as a turning point. Blue's 130-gold spend lifted upkeep from 10 to 56, and the three later level 3 buys at plies 77, 79 and 81 built on it.

## Lessons

Red (ice-and-fire-v004):
- Before buying level 3 units, check that income stays positive after their 20 upkeep each. Ply 32 turned +29 into -28.
- Spend idle gold on cuts while it can take cells. From ply 22 to 28 red held 112 to 196 gold with can_cut 4 to 8 and bought little.
- A cut made of level 1 units breaks to one level 2 unit (ply 37). Back a cut with level 2 or 3 units, or keep a unit ready to re-cut.

Blue (ice-and-fire-v003):
- Count upkeep before each level 3 buy. Three of them in four turns took income from +15 to -39 and ended in debt at ply 82.
- Reconnect cuts with a unit, not a large purchase. Ply 61 spent 130 gold to recover 27 cells and left upkeep at 56.
- Do not read material as a safe lead. Army counts at purchase price, while upkeep is charged against income three times over, so a debt wipe raises material.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Three moves and a level 1 unit lifted cells from 11 to 15 by ply 10; the lead went from -9 to +8 by ply 11."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "Mine on (1,0) lifted income from 10 to 17 at ply 11; the lead went from -6 to +18 by ply 12."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Level 2 on (5,5) took red's front cell; blue's cells went from 22 to 26, and the lead from +3 to +12 by ply 17."},
  {"ply": 22, "side": "red", "verdict": "bad", "note": "MOVE only with 112 gold and can_cut 4; the lead went from -4 to -7 by ply 24."},
  {"ply": 32, "side": "red", "verdict": "bad", "note": "Three level 3 units for 90 gold: upkeep 3 to 63, income +29 to -28; the debt at ply 39 wiped 8 units."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Took tower (7,7) and cut the bottom-left: blue's cells went from 38 to 17; the lead went from -227 to -189 by ply 38."},
  {"ply": 37, "side": "blue", "verdict": "good", "note": "Level 2 on (7,9) broke the wall and reconnected: blue's cells went from 17 to 33; the lead from +14 to +169 by ply 39."},
  {"ply": 41, "side": "blue", "verdict": "good", "note": "Six level 1 units and a tower while red had no units or gold; the lead went from +173 to +272 by ply 43."},
  {"ply": 60, "side": "red", "verdict": "good", "note": "Column 8 closed: blue's cells went from 29 to 11, with 19 cut off; the lead went from -233 to -167 by ply 62."},
  {"ply": 61, "side": "blue", "verdict": "good", "note": "130 gold reconnected 27 cells by ply 62, and the lead check is positive; but upkeep went from 10 to 56 and income to -10, seeding the collapse at ply 81."},
  {"ply": 81, "side": "blue", "verdict": "bad", "note": "Third level 3 (30 gold) at income -22 left 34 gold and income -39; the debt at ply 82 killed 12 units. The lead check reads flat because the wipe raises material."}
]
```
