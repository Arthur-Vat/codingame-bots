# The 20 plies with the largest gaps

Where our search thinks the move played is worst, relative to its own (the gap is our plan's score less the played plan's).

## 1. aCat in game 914372150, turn 21 (endgame), gap 999716

- played (284): `BUILD TOWER 4 8;MOVE 1 1 2;MOVE 27 3 11`
- ours (1000000): `TRAIN 3 6 5;TRAIN 1 6 4;TRAIN 1 6 3;TRAIN 1 6 2;TRAIN 1 7 2;TRAIN 1 8 2;TRAIN 1 9 2;TRAIN 1 10 2;TRAIN 1 11 2;TRAIN 1 11 3;TRAIN 1 11 4;TRAIN 1 11 5;TRAIN 1 11 6;TRAIN 1 11 7;TRAIN 1 11 8;TRAIN 1 11 9;TRAIN 3 11 10;TRAIN 1 11 11`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## ##  . B1 B. B.  .  .
 1 R. R. R. ## ## ## B. B. B. B.  .  .
 2 R.  . R. ## ## ## B. B* B* B.  .  .
 3 R. R1 R. R. ## ## B. B1 B. B1 B.  .
 4 R. R. R. R. R. ## B. B. B. B. B.  .
 5 R. R. R. R. RT R2 B. BT BT B. B.  .
 6 R. R. R. RT R. B2 B3 B. B. B. BT  .
 7 R. R. R. R. R. R2 ## B1 B1 B. B.  .
 8  . R. R. R. R. b. ## ## B. B1 B.  .
 9  .  . R. R* R* R. ## ## ## B1 B.  .
10  .  . R. R2  . R. ## ## ## B. B. BT
11  .  .  .  . R. R. ## ## ## B. B* BH
```

## 2. _iahelka in game 917645982, turn 11 (endgame), gap 500432

- played (-500465): `MOVE 8 7 3;MOVE 3 8 3;MOVE 9 7 5;MOVE 6 8 5;MOVE 7 10 3;BUILD TOWER 8 9`
- ours (-33): `TRAIN 2 7 10;TRAIN 2 6 10;MOVE 8 7 3;MOVE 6 8 5;MOVE 3 10 3;MOVE 7 10 5;TRAIN 1 7 2`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. RT R. R. R. R1 ## ## ## ##
 2 R. R. R. R. R. ## ##  .  .  . ## ##
 3 RM RM R. R. R. R. ##  .  . B1  . ##
 4 R1 R. R. R. R. R. ## B1  . B. B1 B.
 5  .  . R1 R. R. R1 ## B.  . B.  . B.
 6  .  . R1 R. R. ## B. B1 B1 B. B. B.
 7  .  .  . R1 R. ## B. B. B. B. B. B.
 8 ##  .  . R2 R. ## B. B. B. B. BM BM
 9 ## ##  . R1 R. ## ##  . B. B. B.  .
10 ## ## ## ## R. R1 R1 R1 B.  . B. B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

## 3. Pduhard- in game 917645478, turn 13 (endgame), gap 500389

- played (-500590): `MOVE 9 10 5;MOVE 7 11 8;MOVE 6 7 5;MOVE 4 7 4`
- ours (-201): `BUILD TOWER 11 10;TRAIN 2 6 10;MOVE 9 10 5;MOVE 7 11 8;TRAIN 1 10 11;TRAIN 1 9 11`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R1 ## ## ## ## ## ## ## ##
 1 R. R. R. RT R. R. RT B2 ## ## ## ##
 2 R. R. R. R. R. ## ## B1 B. B. ## ##
 3 RM RM R. R. R.  . ## B1 B. B. B. ##
 4 R. R. RT R1 b.  . ## B1 B. B. B1  .
 5 R. R1 R. R. b.  . ## B. B. B.  .  .
 6  .  . R1 R. R1 ## B. B. B. B. B.  .
 7  .  . R1 R. b. ## B. B. B. B. B.  .
 8 ##  . R1 R. R1 ## B. B.  . B. B1  *
 9 ## ## R1 R. R. ## ## B. B. B. B.  .
10 ## ## ## ## R1 R1 R1 BT B. B. B. B.
11 ## ## ## ## ## ## ## ##  .  .  * BH
```

## 4. zasmu in game 917608826, turn 14 (endgame), gap 500220

- played (-499790): `MOVE 11 7 5;MOVE 4 6 9;MOVE 3 10 5;MOVE 10 9 3`
- ours (430): `TRAIN 1 6 9;TRAIN 2 4 5;TRAIN 2 4 4;TRAIN 1 4 3;TRAIN 2 4 2;TRAIN 1 3 2;TRAIN 1 2 2;TRAIN 1 1 2;MOVE 4 7 10;MOVE 6 6 2;MOVE 16 7 2`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## ## ## ##
 1 R. R. R. RM R. ## ## ## ## ## ## ##
 2 R. R. R. R. R1 R. R. B1 B.  .  . ##
 3 ## R. R. R. R. R* RT B2 B. B. B. B.
 4 ## R. R. R1 R1 R1 R. BT B. B1 B. B.
 5 ## RT R. R. R1 B1 B. B. B1 B. B. B1
 6  . R1 R. R1 B1 B. BT B. B. B. B. ##
 7  .  .  .  . B.  . B. B. BT B. B. ##
 8  .  .  .  . B. BT BM B. B. B. B. ##
 9 ##  .  .  .  .  .  . B1 BT B. B. B.
10 ## ## ## ## ## ## ##  . BM B. B. B.
11 ## ## ## ## ## ## ## ## ## B. BM BH
```

## 5. Waffle3z in game 917649342, turn 21 (endgame), gap 500208

- played (-500678): `MOVE 38 6 6;MOVE 37 7 8;TRAIN 1 7 6`
- ours (-470): `MOVE 37 9 8;TRAIN 1 7 6;MOVE 38 6 8`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ## R. R* R. R1  .
 2 R. R. R. R1 RM R. R. R. R. R. R.  .
 3 ## ## RT R. R. R. R. RT R1 R. R. ##
 4 ## ## R. R. R. RT R. R. R. R. ## ##
 5 ## ## ## R1 R* R. R. R1 ## ## ## ##
 6 ## ## ## ## RT R1 R2 R* BT ## ## ##
 7 ## ## B. B. B. BT B3 B. BT B. ## ##
 8 ## B. B. B. BT B. r. r. B2 r. ## ##
 9  .  .  .  . B. B. r. r* B. BT B. B.
10  .  .  .  * B. ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

## 6. zasmu in game 917608826, turn 13 (endgame), gap 500172

- played (-499814): `MOVE 3 11 5;MOVE 11 8 5;BUILD TOWER 5 8;MOVE 4 7 9;BUILD TOWER 8 9;MOVE 10 9 4`
- ours (358): `TRAIN 2 4 5;TRAIN 2 3 5;TRAIN 1 2 5;TRAIN 1 2 4;TRAIN 1 2 3;TRAIN 1 2 2;TRAIN 1 2 1;MOVE 3 11 5;MOVE 6 6 2;MOVE 16 8 3;MOVE 11 8 5;MOVE 4 7 9`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## ## ## ##
 1 R. R. R. R* R. ## ## ## ## ## ## ##
 2 R. R. R. R. R1 R. R. B1 B.  .  . ##
 3 ## R. R. R. R. R* RT B2 B. B. B. B.
 4 ## R. R. R. R1 R1 R. BT B1 B. B. B1
 5 ## R. R. R1 R1 B1 B. B. B. B1 B.  .
 6  .  . R1 R1 B1 B. BT B. B. B. B. ##
 7  .  .  .  . B.  . B. B. BT B. B. ##
 8  .  .  .  . B. B. BM B. B. B. B. ##
 9 ##  .  .  .  .  .  . B. B1 B. B. B.
10 ## ## ## ## ## ## ##  . BM B. B. B.
11 ## ## ## ## ## ## ## ## ## B. BM BH
```

## 7. karliso in game 917643894, turn 15 (endgame), gap 500133

- played (-500076): `MOVE 22 6 11;MOVE 16 9 6;MOVE 3 7 7;MOVE 12 6 6;MOVE 25 3 9;BUILD TOWER 10 9;BUILD TOWER 8 7;BUILD MINE 6 10;BUILD MINE 3 8`
- ours (57): `MOVE 22 6 11;TRAIN 2 6 3;TRAIN 1 6 2;TRAIN 2 6 1;TRAIN 1 10 11;MOVE 16 9 6;MOVE 25 5 9`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . RT RT RM R1 R1  .  *  . ##
 2 R. R. R. R. R. R. R. R1  .  .  . ##
 3 R. R. R* R. R. R. R1 B1  *  . ## ##
 4 R1 R1 R. RT RT R. B1 BM  .  . ## ##
 5 R1 R1 R. R. R. B. B1 BT B.  .  . ##
 6 ## R. R. B2 B. BT B. B. B.  .  .  *
 7 ## ## RT B1 BM B. B1 B. B. B1 B.  .
 8 ## ## R. B* BT B. BT B. B.  * B. B.
 9 ## R. R1 B. B1 B. B. B. B. B. B. B.
10 ## R1 BM BT B. B. B1 B. B. B. B. B.
11 ## ## B.  . ##  *  .  .  .  .  * BH
```

## 8. zasmu in game 917608907, turn 16 (endgame), gap 500124

- played (-499832): `WAIT`
- ours (292): `TRAIN 2 4 6;BUILD TOWER 5 10;MOVE 24 5 5;MOVE 25 9 9`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R. R. R1 R.  .  .  .  .  .
 1 R. R. R. R1 ## R. R. ## ## ## ##  .
 2 R. R. R. R1 R. R. R1 B1 ## ## ##  .
 3 R. ## ## R. RT R. B1 B. B. ## ##  .
 4 R. ## ## b. R. B2 BT B1 B1 B. B*  .
 5 R1  .  . b. R1 R1 B* B. B1 BT B.  .
 6  .  . b. b. R1 B1 B. B. B. BT B. B.
 7  .  *  . b. R1 B. B. B. B. ## ## B.
 8  . ## ##  . B. B. BT BT B. ## ## B.
 9  . ## ## ##  *  . B. B. B.  . B1 B.
10  . ## ## ## ## B. B. ## B. B. B. B.
11  . B1 B. B. B. B. B. B. B. B. BM BH
```

## 9. _iahelka in game 917650392, turn 14 (endgame), gap 500124

- played (-500263): `MOVE 16 2 5;MOVE 17 0 6;MOVE 25 10 10;MOVE 9 4 6;MOVE 7 2 9;MOVE 6 5 11;TRAIN 3 1 5`
- ours (-139): `BUILD TOWER 11 10;MOVE 16 3 6;MOVE 7 2 9;MOVE 6 5 11;MOVE 14 6 7;MOVE 25 10 10`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## R1 R. R. R. ## ## ##
 1 R. R. R. R. R. R* R. R* R1 R1 ## ##
 2 R. R. R. R. R. R. R. R. R1 R1 R1 ##
 3 ## R1 R. R. RT R. R. R* R. R* R.  .
 4 ## R. ## ##  * R. RT R. R. R. R.  .
 5 B2 RT  . ## ## B2 R. R1 R. R. RT  *
 6 B* R1 B1  . B. BT B1 ## ## R1 R.  .
 7 B1 B. BT B. B1 B. B. B* ## ## B2 ##
 8 B. B. B* B. B* B. B. B. B. B. B. ##
 9 ##  .  . B. B. B. B. B. B. B. B2 B.
10 ## ## B1 B. B* B1 B* B. B. B.  . B.
11 ## ## ##  .  .  .  . ## ## B. B* BH
```

## 10. _iahelka in game 917645982, turn 10 (middle game), gap 500098

- played (-500034): `MOVE 8 7 4;MOVE 3 9 3;MOVE 6 8 6;MOVE 9 7 6;MOVE 7 10 4;BUILD MINE 11 8;BUILD MINE 10 8`
- ours (64): `TRAIN 1 5 10;BUILD TOWER 7 10;MOVE 3 10 4;MOVE 8 7 4;MOVE 6 8 6;MOVE 9 7 6`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. R. R. R. R1  . ## ## ## ##
 2 R. R. R. R. R. ## ##  .  .  . ## ##
 3 RM RM R. R. R. R. ##  .  .  .  . ##
 4  . R1 R. R. R. R. ##  .  . B1  . B1
 5  .  .  . R1 R. R1 ## B1  . B.  . B.
 6  .  .  . R1 R. ## B1 B.  . B. B. B.
 7  .  .  .  . R1 ## B. B. B1 B. B. B.
 8 ##  .  .  . R2 ## B. B. B. B. B* B*
 9 ## ##  .  . R1 ## ##  . B. B. B.  .
10 ## ## ## ## R1 B. B. B. B.  . B. B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

## 11. _iahelka in game 917650392, turn 11 (middle game), gap 500095

- played (-500136): `MOVE 3 10 9;MOVE 6 5 8;MOVE 7 3 9;MOVE 9 3 8;MOVE 16 2 8;MOVE 17 1 8;MOVE 18 0 6`
- ours (-41): `MOVE 18 0 6;BUILD TOWER 10 7;MOVE 9 3 6;MOVE 16 2 6;MOVE 7 3 9;MOVE 17 0 7;MOVE 14 6 7;MOVE 3 10 9`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## R1 R.  .  . ## ## ##
 1 R. R. R. R. R. R* R. R1  .  . ## ##
 2 R. R1 R. R. R. R. R. R.  .  .  . ##
 3 ## R. R. R. R. R. R. R*  .  *  .  .
 4 ## R. ## ##  * R. RT R.  . b.  .  .
 5  . RT  . ## ## B2 R1 R. R1 R1 R1  *
 6  * B2 r.  . B. BT B1 ## ##  . R1  .
 7  . B1 B1 B1 B. B. B. B* ## ## B. ##
 8  .  .  *  .  * B. B1 B. B. B. B. ##
 9 ##  .  .  . B. B. B. B. B. B1 B. B.
10 ## ##  . B1 B*  . B* B. B. B.  . B.
11 ## ## ##  .  .  .  . ## ## B. B* BH
```

## 12. zasmu in game 917608826, turn 12 (endgame), gap 500093

- played (-499860): `BUILD TOWER 8 7;MOVE 10 9 5;MOVE 3 11 4;MOVE 11 8 4;MOVE 6 7 2;MOVE 4 8 9`
- ours (233): `MOVE 6 9 2;TRAIN 2 4 5;TRAIN 1 4 4;TRAIN 2 4 3;TRAIN 1 4 2;MOVE 3 11 4;MOVE 10 9 5;MOVE 7 3 6;MOVE 16 7 2`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## ## ## ##
 1 R. R. R. R* R. ## ## ## ## ## ## ##
 2 R. R. R. R. R. R. R. B. B1  .  . ##
 3 ## R. R. R. R1 R* RT B2 B1 B. B. B1
 4 ## R. R. R1 R. R1 R. BT B. B. B.  .
 5 ## R. R1  . R1 B1 B. B. B.  . B.  .
 6  .  .  .  . B1 B. BT B. B. B1 B. ##
 7  .  .  .  . B.  . B. B. B. B. B. ##
 8  .  .  .  . B. B. BM B. B. B. B. ##
 9 ##  .  .  .  .  .  . B. B. B1 B. B.
10 ## ## ## ## ## ## ##  . BM B. B. B.
11 ## ## ## ## ## ## ## ## ## B. BM BH
```

## 13. Waffle3z in game 917643076, turn 12 (middle game), gap 500093

- played (-500035): `MOVE 20 8 2;MOVE 17 6 3;MOVE 19 8 1;MOVE 16 9 2;MOVE 7 8 3;MOVE 6 11 2;MOVE 18 10 0;BUILD MINE 7 10`
- ours (58): `BUILD TOWER 3 10;MOVE 20 8 2;MOVE 17 6 3;MOVE 19 9 2;TRAIN 1 10 11;MOVE 18 10 0;MOVE 6 11 2;TRAIN 1 9 11;MOVE 16 10 3`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. RM R. R. RM R. RT B1 B1 B1 B1  .
 2 R. R1 R. ## ## ## ## ## r. r. B1  .
 3 ## R. R. R. R1  .  . B2 B. B1 B.  .
 4 ## R. ## R. ## ## ## ## B1 B* B. ##
 5 ## R. R. R.  . ## ## ## B. B. B. ##
 6 ## R. R. R. ## ## ##  . B. B. B. ##
 7 ## R. R1 R. ## ## ## ## B. ## B. ##
 8 R. R1 R* R1 B1 BT B. BM B. B. B. ##
 9 R1 B1 B. BT ## ## ## ## ## B. B.  .
10  .  .  . B. B. B. B. B* B. B. BM B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

## 14. zasmu in game 917608907, turn 17 (endgame), gap 500084

- played (-499823): `TRAIN 1 4 7;MOVE 15 5 5;TRAIN 3 4 4;MOVE 19 7 3;MOVE 3 0 11;MOVE 20 8 3;MOVE 28 8 4;BUILD TOWER 7 6;BUILD TOWER 3 4;BUILD MINE 6 5;MOVE 25 9 9`
- ours (261): `TRAIN 1 5 5;TRAIN 2 4 5;TRAIN 2 3 5;TRAIN 1 4 7;MOVE 25 9 9;TRAIN 1 3 8;MOVE 3 0 11;BUILD TOWER 5 10;TRAIN 1 11 5`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R. R. R1 R.  .  .  .  .  .
 1 R. R. R. R1 ## R. R. ## ## ## ##  .
 2 R. R. R. R1 R. R. R1 B1 ## ## ##  .
 3 R. ## ## R. RT R. B1 B. B. ## ##  .
 4 R. ## ## b. R. B2 BT B1 B1 B. B*  .
 5 R.  .  . R1 R1 R. B* B. B1 BT B.  .
 6 R1  . b. R1 R1 B1 B. B. B. BT B. B.
 7  .  *  . b. R. B. B. B. B. ## ## B.
 8  . ## ##  . B. B. BT BT B. ## ## B.
 9  . ## ## ##  *  . B. B. B.  . B1 B.
10  . ## ## ## ## B. B. ## B. B. B. B.
11  . B1 B. B. B. B. B. B. B. B. BM BH
```

## 15. karliso in game 917643894, turn 14 (middle game), gap 500084

- played (-500083): `MOVE 12 6 5;MOVE 22 6 10;MOVE 16 9 7;MOVE 3 6 7;MOVE 25 4 9;BUILD MINE 7 4;BUILD MINE 4 7`
- ours (1): `TRAIN 1 10 11;MOVE 12 6 5;MOVE 16 11 7;MOVE 22 6 10;TRAIN 1 9 5;TRAIN 1 9 4;TRAIN 1 11 10;MOVE 24 7 4`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R. RM R1 R1  .  *  . ##
 2 R. R. R. R. R. R. R. R1  .  .  . ##
 3 R. R. R* R. R. R. R1 B1  *  . ## ##
 4 R1 R1 R. RT RT R. B1 B*  .  . ## ##
 5 R1 R. R. R. R. B1 r. BT B.  .  . ##
 6 ## R1 R. B2 B. BT B. B. B.  .  .  *
 7 ## ## RT B1 B* B1 B. B. B.  . B1  .
 8 ## ## R. B* BT B. BT B. B.  * B. B.
 9 ## R. R1 B1 B. B. B. B. B. B. B. B.
10 ## R1 BM BT B. B1  * B. B. B. B. B.
11 ## ## B.  . ##  *  .  .  .  .  * BH
```

## 16. reCurse in game 917648864, turn 20 (endgame), gap 500083

- played (-500528): `MOVE 28 8 9`
- ours (-445): `MOVE 29 9 7`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R1 ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ##  .  *  .  .  .
 2 R. R. R1 R. RM R1 R. R.  .  .  .  .
 3 ## ## R. R. R. R. RT R.  .  .  . ##
 4 ## ## R. R. R. RT R. R2 R.  . ## ##
 5 ## ## ## R2 R1 R. B. BT ## ## ## ##
 6 ## ## ## ## R. R. R3 BM B. ## ## ##
 7 ## ## R. R. R. RT R. B. B2 B. ## ##
 8 ## R1 R. R. R1 R. R. R. R2 B2 ## ##
 9  .  . b. b. bT b. bT R* R3 B3 B. B.
10  .  .  .  *  . ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

## 17. karliso in game 917645592, turn 10 (middle game), gap 500081

- played (-499975): `MOVE 7 10 4;MOVE 4 7 9;MOVE 3 3 8;MOVE 10 4 6;MOVE 6 6 7;MOVE 9 9 3;MOVE 8 8 5`
- ours (106): `TRAIN 1 10 11;BUILD TOWER 7 10;TRAIN 2 7 1;TRAIN 1 6 1;MOVE 7 10 4;MOVE 4 7 9;MOVE 3 3 8;MOVE 9 8 2;MOVE 10 3 7`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. R. R1 R. R. R1 ## ## ## ##
 2 R. R. R. R. R. ## ## B. B. B1 ## ##
 3 RM R* R. R. R.  . ## BT B. B. B1 ##
 4  .  . R. R1  .  . ## B. B. B.  .  .
 5  .  .  .  .  .  . ## B1 B.  .  .  .
 6  .  .  .  .  . ## B. B. B. B.  .  .
 7  .  .  .  . B1 ## B. B. B. B.  .  .
 8 ##  .  .  . B. ## B1 B. B. B.  *  *
 9 ## ##  . B1 B. ## ##  . B1 B. B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ##  .  .  * BH
```

## 18. Zylo in game 917654833, turn 18 (endgame), gap 500073

- played (-499800): `MOVE 35 5 5;MOVE 7 2 9;MOVE 23 5 6;MOVE 36 7 5;MOVE 32 5 8;MOVE 3 10 3;MOVE 31 7 8;MOVE 39 11 5;MOVE 16 8 9;MOVE 8 10 9;BUILD MINE 6 8`
- ours (273): `MOVE 35 5 5;MOVE 32 4 7;MOVE 39 11 5;MOVE 10 6 4;TRAIN 1 10 4;TRAIN 1 11 9;MOVE 8 10 7`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## ## ## ##
 1 R. R. R. R* R. ## ## ## ## ## ## ##
 2 R. R. R. R. R. R1 R1 B1 B. B. B1 ##
 3 ## R. R. R. RT R* B1 B. BT B. B. B1
 4 ## R. R1 R. R. B2 B. B1 B. B. B. B1
 5 ## R1 R1 RT R. R1 B. B. BT B. B.  .
 6 R. R. R. R. B2 B. BT B. B. B. B. ##
 7 R1 RT R. B. B. B. B. B. B. B. B. ##
 8 R1 R. B1 BT B2 B. B1 B. B2 B. B1 ##
 9 ## B. B. B. BT B. BT B. B. B. B.  .
10 ## ## ## ## ## ## ## B. BM B. B. B.
11 ## ## ## ## ## ## ## ## ## B. BM BH
```

## 19. reCurse in game 917644354, turn 12 (middle game), gap 500069

- played (-500104): `TRAIN 2 5 6;TRAIN 1 6 2;MOVE 3 9 2;TRAIN 1 5 3;MOVE 8 6 6;MOVE 6 8 7`
- ours (-35): `MOVE 6 9 6;TRAIN 3 4 5;TRAIN 1 3 5;TRAIN 1 4 6;MOVE 3 7 2`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R. RM R1  .  .  *  . ##
 2 R. R. R. R. R. R1 R.  . B1  .  . ##
 3 R. R. R* R. R. R. BT B. B*  . ## ##
 4 R. R1 R1 R. RT BT B. B* B.  . ## ##
 5 R1  . R. R. RT B1 B1 B. B. B. B. ##
 6 ##  .  .  . R. R1 B. BT B1 B. B.  *
 7 ## ##  .  . b* R2 R1 B. B. B. B.  .
 8 ## ##  .  * b. b. R1 R1 B. BM B.  .
 9 ##  .  .  .  . b. b. R1  .  . B.  .
10 ##  .  *  .  .  .  * B. B. B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

## 20. zasmu in game 917608618, turn 16 (endgame), gap 500067

- played (-500473): `BUILD TOWER 9 11`
- ours (-406): `BUILD TOWER 9 9`

```text
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## R1 R1  . ## ##
 1 R. R. R. ## ## R. R1 R. R.  .  . ##
 2 R. R. R. R1 R1 R. R. R. R.  .  . ##
 3 ## ## ## R. R. R. R. R. R.  *  . ##
 4 ## ## ## RT R. RT R* R. R* R* ## ##
 5 ## ## ## b. b. b. b. R. RT R1 ## ##
 6 ## ## b. b. b. bT R1 R. R. ## ## ##
 7 ## ## bM b* b. bM b. R. R2 ## ## ##
 8 ##  .  *  .  . b. bT R. R3 ## ## ##
 9 ## b.  .  . b. b. r. b. R1 B. B.  .
10 ## b. b. b. b. b. b. ## ## B. B. B.
11 ## ## b. b. b. ## ## ## ## B. BM BH
```

