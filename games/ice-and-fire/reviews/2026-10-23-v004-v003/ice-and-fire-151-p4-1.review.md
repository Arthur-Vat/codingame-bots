# Review of ice-and-fire-151-p4-1: ice-and-fire-v004 (blue) against ice-and-fire-v003 (red)

Winner: ice-and-fire-v004, as blue, in 11 turns (22 plies), by taking red's HQ: TRAIN 1 1 0 then TRAIN 1 0 0 at ply 21. The file ends before the closing board, but the game stopped at turn 11, far from the turn-100 limit, so the HQ take is the only finish the stated rules allow.

## Why the game was won

Blue's 40-gold strike at ply 15 decided the middle game: it cut five of red's cells, killed three red units, and moved blue's material lead from +16 (ply 15) to +73 (ply 17). Blue had kept its bank by moving instead of spending at plies 11 and 13 (13 and 28 gold idle), so it could strike with 47 gold, while red spent its bank at plies 14 and 16 on units that were cut off again. Blue's income was the weaker (19 against red's 23 at ply 14), so the bank, not the economy, decided it.

## Early game

Both sides opened with two level 1 units on turn 1 (red at (1,0) and (1,1) at ply 0; blue at (10,10) and (10,9) at ply 1), then expanded by moves. Material is written blue minus red below. The lead stayed between -9 and +14 through ply 13. The openings look alike: the file does not show which weights v004 chose, and nothing in these 14 plies separates the two sides.

Red, good: ply 10 (BUILD MINE 1 0) and ply 12 (BUILD MINE 4 2) took income from 10 to 23 by ply 13. Red had the better economy at ply 14 (income 23 against 19). Red, bad: none clear.

Blue, good: ply 9 (one level 1 and three moves) closed a gap of -9 to -1 by ply 11; ply 11 (four moves, no spend) took cells from 15 to 19 and the lead to +14 by ply 13.

Blue, missed: it never built a mine (mines 0 at every ply), although the mine spot at (10,11) next to its HQ stayed empty to the end. Ply 13 also left 28 gold idle.

## Middle game

Red, bad (ply 14): TRAIN 1 5 3 and TRAIN 1 6 3 spent 20 of 27 gold on two level 1 units behind thin links at (4,3) and (5,4). At ply 15 blue took those links, both units died at ply 16 (red units 5 to 2, army 50 to 20), and exposed50 rose from 3 to 14. Blue's lead went from +19 to +80.

Blue, good (ply 15): TRAIN 2 5 4 took the red unit on (5,4), and TRAIN 1 4 4 and TRAIN 1 4 3 cut more red cells (red cut_off 0 to 2; red voronoi 52 to 36, blue 63 to 80). It spent 40 of 47 gold. Blue's lead went from +16 to +73 by ply 17.

Red, bad (ply 16): TRAIN 2 4 4 and TRAIN 1 4 5 put all 30 gold on the southern block at (4,4) and (4,5), leaving 0 gold. At ply 17 blue trained onto (2,3) and (3,3), cutting that six-cell block off; three units (40 gold) died at ply 18 (army 50 to 10). Blue's lead went from +80 to +183.

Blue, good (ply 17): two trainings and four moves cut red's active cells from 19 to 11. Its own two level 1 units on (2,3) and (3,3) were then cut off and died at ply 19 (army 80 to 60), so the trade was not clean.

## Endgame

Red, good (ply 18): TRAIN 1 4 3 spent 10 of 18 gold on the single link cell (4,3). It reconnected the six cut-off cells (active cells 11 to 18 at ply 19) and cut blue's two units on (2,3) and (3,3), which died. Blue's lead fell from +183 to +127 by ply 19. The drop to +312 at ply 20 comes mostly from blue's ply 19 cut, not from this play, which I judge on its own effect.

Blue, good (ply 19): TRAIN 1 2 2, TRAIN 1 2 1 and TRAIN 1 1 1 spent 30 of 32 gold to take (2,2), (2,1) and (1,1). Red's active cells fell from 18 to 3, with 10 more cut off. Blue's lead went from +127 to +312 by ply 20, and its hq_gap fell from 38 to 0 by ply 21.

Red, WAIT at ply 20 (not judged): with 15 gold, one level 1 could cover only one of the two HQ approaches, (0,1) and (1,0), and blue had 34 gold for two trainings. The HQ was lost by then; the errors sit at plies 14 to 16.

Blue, good (ply 21): TRAIN 1 1 0 and TRAIN 1 0 0 (20 of 34 gold) trained onto (1,0) and then onto red's HQ cell (0,0).

## Decisive moments

- Ply 16, swing (rules): the 61-point move was made by blue's ply 15. The decisive play is ply 15.
- Ply 17, large cut (rules): agreed. Red's ply 16 had put its units behind the (2,3)/(3,3) cells.
- Ply 18, swing (rules): caused by blue's ply 17 cut. Red's answer at ply 18 was its best move of the game, and it did not turn the game.
- Ply 19, large cut (rules): agreed; the decisive cut, 15 active cells in one ply.
- Ply 20, swing (rules): caused by ply 19.
- Ply 21, HQ threat (rules): the finish.
- Ply 14 (reviewer's pick): red's forward units, the setup for blue's ply 15 strike.
- Ply 13 (reviewer's pick): blue's idle 28 gold, which became the 47 gold used at ply 15. Blue's ply 13 is the setup for the win.

## Lessons

Red:
1. Do not train into cells behind a one-cell link (plies 14 and 16). Train on the link itself; a level 1 holding a cell cannot be taken by another level 1.
2. Keep reserve gold to reconnect. The 10-gold reconnect at ply 18 was red's best play, but the all-in at ply 16 left no gold for the cells blue then cut.
3. Garrison the (2,3)/(3,3) cells before blue's trainings take them.

Blue:
1. Keep the bank and strike at cut points, as at ply 15.
2. Build the mine at (10,11), next to its HQ. Blue built none in 11 turns while holding 13 to 47 gold.
3. Do not place units on cells that only one link feeds (ply 17; they died at ply 19).

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "One level 1 and three moves closed the material gap from -9 to -1 by ply 11."},
  {"ply": 11, "side": "blue", "verdict": "good", "note": "Four moves, no spend: cells 15 to 19 and the lead to +14 by ply 13."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "Spent 20 of 27 gold on two level 1 units behind thin links; blue cut them at ply 15 and both died; lead +19 to +80 by ply 16."},
  {"ply": 15, "side": "blue", "verdict": "good", "note": "Spent 40 of 47 gold: level 2 on (5,4), level 1s on (4,4) and (4,3); red units 5 to 2, lead +16 to +73 by ply 17."},
  {"ply": 16, "side": "red", "verdict": "bad", "note": "Spent all 30 gold on (4,4) and (4,5) with no garrison on (2,3)/(3,3); blue cut the six-cell block at ply 17, three units died; lead +80 to +183 by ply 18."},
  {"ply": 17, "side": "blue", "verdict": "good", "note": "Trainings on (2,3) and (3,3) cut red's active cells from 19 to 11; its own two units there died at ply 19."},
  {"ply": 18, "side": "red", "verdict": "good", "note": "One level 1 on (4,3) reconnected six cut-off cells (11 to 18 active) and killed two blue level 1s; the drop at ply 20 came from blue's ply 19 cut."},
  {"ply": 19, "side": "blue", "verdict": "good", "note": "Three level 1 trainings on (2,2), (2,1), (1,1) cut red's active cells from 18 to 3; lead +127 to +312 by ply 20; hq_gap 0 at ply 21."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Two level 1 trainings, onto (1,0) and then red's HQ cell (0,0), ended the game at turn 11."}
]
```
