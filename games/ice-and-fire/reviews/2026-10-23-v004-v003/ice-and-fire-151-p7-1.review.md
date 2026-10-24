# Review of ice-and-fire-151-p7-1

Red, ice-and-fire-v003 (the parent), won against blue, ice-and-fire-v004 (the candidate), by taking blue's HQ in 17 turns (33 plies). The file prints no closing board; the HQ is the only win before turn 100, and ply 32 trains onto blue's HQ at (11,11) with red's hq_gap at 0, so I read the end as an HQ capture.

## Why the game was won

Red's economy and its trainings on blue's links decided it. Red built mines at (1,0) and (4,2) (plies 10 and 12), lifting its income from 10 at ply 10 to 24 at ply 14; blue built none in 17 turns. From ply 16 red spent its gold on trainings that cut blue's corridors: 50 gold at ply 16 cut four blue cells and left blue with 3 units (from 8), and 40 gold at ply 24 took 12 more cells (lead +43 to +208). Blue's reconnections at plies 25 and 27 were each undone by red's next 40-gold turn.

## Early game (plies 0 to 13)

The first turn looked the same on both sides: each bought two level-1 units beside its HQ with 20 gold (red at (1,0) and (1,1), blue at (10,10) and (10,9)). Cell counts stayed within two of each other to turn 6 (red 13, blue 15 at ply 10). The openings diverge later, in the economy. The file does not show v004's opening weights, so it cannot say whether the opening rules or the shared search made blue's economic choices.

- Red, good (ply 10): BUILD MINE 1 0, next to its HQ, with 15 of 17 gold. Income 10 at ply 10, 17 at ply 12. Its second mine, at (4,2) on ply 12, took income to 24 at ply 14.
- Blue, good (ply 9): TRAIN 1 8 5 for 10 gold plus three moves. Cells rose from 11 to 15 by ply 10, and red's lead fell from +9 to +1 by ply 11.
- Blue, bad (ply 13): MOVE only, with 28 gold. Its mine spot (8,6) had been blue's since ply 12 (B*), next to its unit at (7,6), and it stayed unbuilt. Blue's income was 15 against red's 24. The spot was still empty at ply 28, when red trained onto it. A mine costs 15 and pays 4 a turn.

## Middle game (plies 14 to 24)

- Red, bad (ply 14): MOVE only, with 28 gold unspent. At ply 15 blue had 47 gold and used four level-1 trainings on (3,4), (4,4), (5,4) and the empty (5,5). Red's cells fell from 22 to 19, and the lead from -11 to -43. A level-2 train on (5,4), affordable with 28 gold, was the defence it skipped.
- Blue, bad (ply 15): the 40 gold took three red cells and raised blue's cells from 23 to 30, but the new cells hung from one link at (5,4). At ply 16 red trained a level-2 onto (5,4) and three level-1 units at (6,4), (6,5), (6,6). Four blue cells were cut and five units died (blue units 8 to 3, army 80 to 30). The lead went from blue +9 at ply 15 to red +41 at ply 17.
- Red, good (ply 16): the reviewer's pick below. Red's lead went from -43 to +7 by ply 18.
- Blue, bad (ply 19): 30 gold on three level-1 units. Blue reconnected four cells and cut three red cells, killing three red units (army 70 to 30). But blue was left with 1 gold and 34 cells, five of them exposed to 30 gold. Red's ply 20 cut seven (blue cut_off 1 to 7). The lead went from red +70 at ply 19 to red +84 at ply 21.
- Red, good (ply 20): TRAIN 1 6 4, 8 4 and 9 4 for 30 gold. Blue cells fell from 34 to 25 and its cut_off rose from 1 to 7. The lead went from blue +18 to red +84.
- Blue, good (ply 21): TRAIN 2 6 4 and two moves. Blue reconnected three cells (cut_off 7 to 4), cut five red cells and killed three red units (army 60 to 30). The lead went from red +84 to red +64 at ply 23.
- Red, good (ply 24): four level-1 trainings on column 7 for 40 gold. Blue cells fell from 35 to 23, its units from 6 to 3, and its cut_off rose from 1 to 8. Red reconnected its own four cut cells, and the lead went from +43 to +208.

## Endgame (plies 25 to 32)

- Blue, bad (ply 25): 20 gold on two level-1 trainings. Blue reconnected all eight cut cells (cells 23 to 34) and cut five red cells, so the lead fell from +208 to +108. Red's 40-gold turn at ply 26 took 12 of those cells back (34 to 22). The lead at blue's next turn was +218, so the reconnection did not hold.
- Red, good (ply 28): four trainings for 40 gold, one of them onto the empty blue mine spot (8,6). Blue cells fell from 32 to 14, its cut_off rose to 12, and the lead went from +194 to +380. Red's ply 26 (12 cells) and ply 30 (50 gold, lead +431 to +506) followed the same pattern.

## Decisive moments

Rules' moments:
- Ply 22 (lead change): it marks when red's lead started to hold, not the cause. Red had led at plies 17 to 19 and at 21 already; the swings came at plies 16, 20 and 24.
- Ply 24 (cut, 35 to 23): decisive. The lead went from +43 to +208.
- Ply 26 (cut, 34 to 22) and ply 28 (cut, 32 to 14): ply 28 was the largest single cut, and the lead went from +194 to +380.
- Ply 30 and ply 32 (swings): results of earlier cuts. Blue had 14 active cells from ply 29 on.
- Ply 32 (HQ threat): the finish. Red had hq_gap 0 with 51 gold and trained TRAIN 2 9 10, TRAIN 1 10 10, TRAIN 1 11 10 and TRAIN 1 11 11.

Reviewer's picks:
- Ply 16 (reviewer's pick): red's 50-gold turn was the largest early swing, from -43 to +41 at ply 17. The rules' first lead change is at ply 22, so they missed the sign flip here.
- Ply 13 (reviewer's pick): blue's unbuilt mine was not decisive on its own, but it set the income gap that stayed (15 against 24 at ply 13, 19 against 27 at ply 15).
- Ply 15 (reviewer's pick): blue's 40-gold extension created the one-link corridor that ply 16 cut.

## Lessons

Blue (ice-and-fire-v004):
1. Build on an owned mine spot as soon as 15 gold allows. At ply 13 it had 28 gold and (8,6) was its own; red's two mines gave it +8 income a turn.
2. Before extending a corridor, check the one cell that links the new cells to the HQ, as (5,4) was at ply 15, and spend the rest of the gold covering it, not down to 1 gold as at ply 19.

Red (ice-and-fire-v003):
1. Do not leave gold idle at the front when the enemy's next-turn gold can buy a cut. At ply 14 it held 28 gold while blue had 47 for ply 15.
2. Keep the ply 16 pattern: when a corridor is cut, retake the cell that joins it, with a level-2 unit.

```json
[
  {"ply": 9, "side": "blue", "verdict": "good", "note": "TRAIN 1 8 5 for 10 gold plus moves; cells 11 to 15 by ply 10; red's lead fell from +9 to +1 by ply 11."},
  {"ply": 10, "side": "red", "verdict": "good", "note": "Mine at (1,0) beside the HQ with 15 of 17 gold; income 10 to 17 by ply 12."},
  {"ply": 13, "side": "blue", "verdict": "bad", "note": "MOVE only with 28 gold; owned mine spot (8,6) left unbuilt; income 15 against red's 24; the spot was taken by red at ply 28."},
  {"ply": 14, "side": "red", "verdict": "bad", "note": "No training with 28 gold; blue's 40-gold turn at ply 15 took three frontier cells; lead -11 to -43."},
  {"ply": 15, "side": "blue", "verdict": "bad", "note": "40 gold on four level-1 units hung from the single link (5,4); red's level-2 cut at ply 16 took four cells and five units."},
  {"ply": 16, "side": "red", "verdict": "good", "note": "50 gold: level-2 retake of (5,4) plus three level-1 units; blue units 8 to 3; lead -43 to +41 at ply 17."},
  {"ply": 19, "side": "blue", "verdict": "bad", "note": "30 gold left blue at 1 gold with 34 cells, 5 exposed to 30 gold; red cut 7 at ply 20; lead +70 to +84 for red."},
  {"ply": 20, "side": "red", "verdict": "good", "note": "30 gold on three level-1 units cut seven blue cells (cut_off 1 to 7); lead -18 to +84."},
  {"ply": 21, "side": "blue", "verdict": "good", "note": "Level-2 train and moves reconnected three cells and cut five red cells; lead +84 to +64."},
  {"ply": 24, "side": "red", "verdict": "good", "note": "40 gold on four level-1 units on column 7 took 12 blue cells and killed three units; lead +43 to +208."},
  {"ply": 25, "side": "blue", "verdict": "bad", "note": "20 gold reconnected eight cells, but red's 40-gold turn at ply 26 took 12 back; lead +218 at blue's next turn."},
  {"ply": 28, "side": "red", "verdict": "good", "note": "40 gold on four trainings, one onto empty mine spot (8,6); blue cells 32 to 14; lead +194 to +380."}
]
```
