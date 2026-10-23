# Review of ice-and-fire-151-p6-0

Red (v004) won, as the red seat, by taking blue's HQ on the last ply (ply 42), in 22 turns. The file has no board after ply 42, so the HQ capture is read from the rules' end and red's final chain.

## Why the game was won

Red's win was decided at ply 20. Red spent 30 of its 34 gold on four moves and three level 1 trainings that cut 11 of blue's 29 active cells: blue went from 29 to 12 cells, income from 25 to 12, and units from 4 to 0 (ply 21). Blue had just spent 20 of its 22 gold on a longer western arm, with exposed30 at 8 (ply 19). From then on blue never rebuilt its economy: it had no mine all game and its income stayed at 12 from ply 21 to the end, while red reached 47 with four mines (ply 37).

## Early game

Plies 0 to 13 (turns 1 to 7). The openings mirror each other: red trained at (1,0) and (2,0) (ply 0), blue at (10,10) and (9,10) (ply 1). Material stayed within 10 points either way, and red led by -2 at ply 13 (169 to 171). This game does not show that v004's per-map opening weights changed anything, and it contains no v003 opening alone to compare with.

Red:
- Good, ply 10: BUILD MINE 1 0 (15 gold), its first mine. Income went from 9 (ply 10) to 16 (ply 11), and the lead went from -6 to +2 at ply 11.
- No bad play is clear. Red grew from 3 cells (ply 2) to 10 (ply 8) with its units.

Blue:
- Good, ply 9: MOVE 3 8 8, MOVE 6 6 8, MOVE 4 6 9 and TRAIN 1 6 7 took the centre. Voronoi rose from 37 to 41, and the lead went from +10 to -6 at ply 10.
- Its first building, at ply 13, was a tower at (7,9) (15 gold). Blue never built a mine, not even on the mine spot at (10,11) beside its HQ.

## Middle game

Plies 14 to 20 (turns 8 to 11). Both sides traded cuts here, and red finished ahead.

Red:
- Good, ply 14: three level 1 trainings at (6,5), (6,6) and (6,7) pushed cells from 17 to 23, and the lead went from -7 to +25 at ply 15 (220 to 195). The column was cut two cells back at ply 15, and red was left with 5 gold, so this is a risk noted rather than a bad play.
- Good, ply 16: after blue's cut (cut_off 2, cells 20), red trained two units and moved to reconnect. Cut_off went to 0 and cells to 26 at ply 17; the lead went from -19 to +35.
- Good, ply 18: two trainings at (7,5) and (7,6) and two moves rebuilt the link (cut_off 3 to 1). The lead went from -28 to +38 at ply 19.
- Good, ply 20: the decisive cut (see below).

Blue:
- Good, ply 15: a level 2 at (6,5) and moves took cells from 19 to 24 and cut 2 of red's. The lead went from +25 to -19 at ply 16 (199 to 218).
- Good, ply 17: moves and a level 2 training cut 3 of red's cells (red cut_off 3 at ply 18). The lead went from +35 to -28.
- Bad, ply 19: it spent 20 of its 22 gold on two level 1 trainings and two moves, extending a chain along row 6 and down to (2,8). It was left with 2 gold and exposed30 8 against red's 34 gold. Red cut 11 cells at ply 20. The lead went from +38 to +6 at ply 20 and then to +188 at ply 21.

## Endgame

Plies 21 to 42 (turns 11 to 22).

Red:
- Good, plies 22 to 28: red kept its cells at 33 to 46 while blue's income stayed at 12. The lead grew from +218 (ply 22) to +363 (ply 28).
- Good, ply 36: BUILD MINE 6 3, 5 4 and 4 5 (45 gold). Mines went from 1 to 4 and income from 35 to 47 (ply 37). The lead went from +525 to +547 at ply 38.
- Good, ply 42: red had 233 gold against a 190-gold chain (hq_gap 0 for the first time, from 4 at ply 40), and it paid for the chain. Its waits at plies 38 and 40 (139 and 186 gold) were right, since it could not yet pay.

Blue:
- Not a bad play, ply 21: WAIT with 14 gold, below the 15 a mine costs, and no unit could reconnect the 11 cut cells.
- Bad, ply 23: WAIT with 26 gold while the mine spot at (10,11) beside its HQ stayed empty. A mine there would have added 4 a turn over the last ten turns. The lead went from +238 to +272 at ply 24.

## Decisive moments

- Ply 20 (rules, large cut): the decisive play, as above.
- Ply 22 (rules, lead change): the lead had already changed at ply 21 (+188). Ply 22 is only the first turn it was shown.
- Ply 32 (rules, swing of 64 over turns 16 and 17): from ply 30 to 32, red's material went from 549 to 584 (cells 48 to 50, army 130 to 160). Blue's went from 158 to 129, mostly from army (40 to 10) as units on cut cells died. It is the aftermath of the ply 20 cut, not a new decision.
- Ply 42 (rules, HQ threat): the win.
- Ply 15 (reviewer's pick, missed by the rules): blue's counter-cut took the lead from +25 to -19, a 44-point swing in one turn. It was blue's best play and the turning point of the middle game.
- Ply 23 (reviewer's pick, missed by the rules): blue's unbuilt mine beside its HQ (see above).

## Lessons

Red:
1. Keep the cut pattern: at ply 20 red had 34 gold and a blue chain that was exposed, and it used both.
2. Build mines earlier. Mine spots (6,3), (5,4) and (4,5) were open from ply 0. Red built its first mine at ply 10 and the other three at ply 36.
3. Do not leave a unit column in reach with 5 gold left (ply 14). Blue cut two of its cells at once.

Blue:
1. Do not extend a chain across cells the enemy can reach with 30 gold while holding 2 gold (ply 19).
2. Build the mine at (10,11) beside the HQ early. Blue had none all game, so its income stayed flat at 12 from ply 21.
3. Its counter-cuts at plies 15 and 17 were its best plays. Keep that reaction, and defend its own chain as well.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "Four moves and a level 1 training took the centre; the lead went from +10 to -6 at ply 10."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "First mine at (1,0); income went from 9 to 16 by ply 11 and the lead from -6 to +2."},
  {"ply": 14, "side": "red", "verdict": "good", "note": "Three level 1 trainings at (6,5)-(6,7) took cells from 17 to 23; the lead went to +25, though the column was cut back at ply 15."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "A level 2 and moves cut 2 red cells and took 5 of its own; the lead went from +25 to -19."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "Two trainings and moves reconnected red's cells; cut_off 2 to 0 and the lead went from -19 to +35."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Moves and a level 2 training cut 3 red cells; the lead went from +35 to -28."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "Two trainings at (7,5) and (7,6) rebuilt the link; the lead went from -28 to +38."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "Spent 20 of 22 gold on a longer western chain, left with 2 gold and exposed30 8; red cut 11 cells at ply 20."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "Spent 30 gold on units and moves that cut 11 of blue's 29 cells; blue's income fell from 25 to 12."},
  {"ply": 23, "side": "blue", "verdict": "bad", "note": "WAIT with 26 gold while the mine spot (10,11) beside its HQ stayed unbuilt; the lead went from +238 to +272."},
  {"ply": 36, "side": "red", "verdict": "good", "note": "Three mines for 45 gold; income went from 35 to 47 and the lead from +525 to +547."},
  {"ply": 42, "side": "red", "verdict": "good", "note": "Paid a 190-gold chain to blue's HQ with 233 gold and won."}
]
```
