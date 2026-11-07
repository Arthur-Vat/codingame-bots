# Decisive moments: examples

## Examples

How to read the boards:

- A Code of Ice and Fire, Bronze rules: a 12 x 12 map; red's HQ is at (0, 0), blue's at (11, 11). Red plays first in each turn. Taking the enemy HQ wins; after 100 turns the side with more gold plus training cost of its units wins.
- Each turn a player may train units (level 1, 2 or 3 for 10, 20 or 30 gold) on its own cells or next to them, move each unit one cell, and build mines (15 gold, on mine spots, +4 income) and towers (15 gold). Upkeep each turn: 1, 4 or 20 by level.
- A unit beats a lower level only (level 3 beats all). Only level 3 may enter a cell holding an enemy tower, or an enemy active cell next to an active enemy tower; a neutral cell next to a tower is not protected. Entering an enemy cell takes it.
- A cell is active when a path of its owner's cells joins it to the owner's HQ. Inactive (cut off) cells earn nothing and the units on them die at their owner's next turn: cutting a corridor removes everything beyond it at once. A side whose gold would go below 0 at its turn loses all its units (debt).
- Income each turn: active cells, plus 4 for each active mine, less upkeep.
- Board: two characters a cell. The first is the owner: R red, B blue, upper case for an active cell, lower case (r, b) for a cut-off one, a space for nobody. The second is the content: . empty, 1 2 3 a unit of that level, M mine, T tower, H HQ, * an empty mine spot. ## is void. Columns are x (0 to 11), rows are y (0 to 11), as in the commands.
- Commands: TRAIN level x y; MOVE unit-id x y (one step towards x y); BUILD MINE x y; BUILD TOWER x y; WAIT.

### zasmu (rank 1, 9 losses, 9 with a decisive moment)

#### Game 917607785: we were red, lost at turn 17 (the rules ended the game)

Decisive moment: `HQ threat` at turn 17 (ply 33), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `MOVE 25 2 5;TRAIN 1 1 5;TRAIN 2 1 4;TRAIN 2 1 3;TRAIN 1 1 2;TRAIN 1 1 1;TRAIN 1 1 0;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 6 -> 6.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## ## ## ## ## B1 B1 ##
 1 R. R* R. R1 R* R. R. RT BT B2 B. B.
 2 R1 R. R. ## ## ## ## ## BT B. B1 B.
 3 ## R1 R1 R1 R* R. R. R3 BT B* B. B.
 4 ## R1 ## R. ## ## ## ##  . BM B. ##
 5 ## R. R. B2  . ## ## ##  .  . B. ##
 6 ## R. B1 B. ## ## ##  . B. B. B. ##
 7 ##  . BM BT ## ## ## ## B. ## B. ##
 8  .  . B1 B1 B. B. B. B* B.  . B. ##
 9  .  . r. r. ## ## ## ## ##  . B.  .
10  .  .  . r. B. B. B. BM B. B1 BM B.
11 ##  .  . ## ## ## ## ## ## B. B* BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 57 | 0 |
| income | 48 | 0 |
| cells | 47 | 26 |
| army | 110 | 90 |
| mines | 4 | 0 |
| towers | 4 | 1 |
| exposed30 | 1 | 6 |
| hq_gap | 43 | 140 |
| material | 539 | 204 |
| voronoi | 67 | 26 |

#### Game 917607947: we were red, lost at turn 20 (the rules ended the game)

Decisive moment: `HQ threat` at turn 20 (ply 39), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `MOVE 35 8 2;TRAIN 1 8 1;TRAIN 1 7 1;TRAIN 2 6 1;TRAIN 1 6 0;TRAIN 1 5 0;TRAIN 1 4 0;TRAIN 2 3 0;TRAIN 2 2 0;TRAIN 1 1 0;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 4 -> 3 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R1 R1 R. R. R* ## b. b. ## ##
 1 R. R1 R. R. R. RM R1 R. R. R2  . ##
 2 R. R. R. R. R. RT R. R1 R. B.  . ##
 3 R. R. R* R. R. R. R. R1 B1 BT ## ##
 4 R. R. R. RT R. R. BT BM r. BT ## ##
 5 R1 RT R1 R2 R. B. BT BT B. BT B. ##
 6 ## R1 R1 B. BT BT B. B. B1 BT B. B1
 7 ## ## BT B1 B* BT B. BT B1 B. B. B.
 8 ## ## B1 B1 BT BT B. B. BT BM B. B.
 9 ##  . B.  .  . B.  . B. B. B. B. B.
10 ##  .  *  .  . B. BM BT B. B. B1 B.
11 ## ##  .  . ##  *  .  . B. B. BM BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 81 | 123 |
| income | 68 | 37 |
| cells | 62 | 47 |
| army | 100 | 140 |
| mines | 4 | 2 |
| towers | 16 | 3 |
| exposed30 | 4 | 4 |
| hq_gap | 49 | 47 |
| material | 793 | 592 |
| voronoi | 78 | 50 |

#### Game 917608139: we were red, lost at turn 21 (the rules ended the game)

Decisive moment: `large cut` at turn 17 (ply 33), played by blue. blue took red's cells from 45 to 16.

Their answer: `MOVE 23 2 6;TRAIN 2 2 5;TRAIN 1 2 4;TRAIN 1 2 3;TRAIN 1 2 2;TRAIN 1 2 1;TRAIN 1 2 0;TRAIN 1 1 0;MOVE 30 2 7;MOVE 6 6 8;MOVE 3 11 7;MOVE 4 9 9`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 3 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R1 ## ## ## ## ## ## ## ##
 1 R. R1 R. R. R1 R1 R. R. ## ## ## ##
 2 R1 R. R. R. R. ## ## R2 B2 B. ## ##
 3 RM RM R. R. R. R. ## BT B. B. B. ##
 4 R. R1 R. R. R. R1 ## B. B. B. B.  .
 5 R. R1 R1 R. RT R1 ## B. B. BT B.  .
 6 R1 R1 R1 RT R. ## B. B. B. B. B. B.
 7 R1 R1 B3 B1 R1 ## B1 B. B. B. B1 B.
 8 ## R1 B. BT B1 ##  . B. B. B1 BM BM
 9 ## ## r. B. BT ## ## B. B. B. B. B.
10 ## ## ## ## B. B2 B. B1 B. B. B. B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH B1 B1 r. ## ## ## ## ## ## ## ##
 1 R. R1 B1 r. r. r. r. r. ## ## ## ##
 2 R1 R. B1 r. r. ## ## r. B2 B. ## ##
 3 RM RM B1 r. r. r. ## BT B. B. B. ##
 4 R. R1 B1 r. r. r. ## B. B. B. B.  .
 5 R. R1 B2 r. rT r. ## B. B. BT B.  .
 6 R1 R1 B3 rT r. ## B. B. B. B. B. B.
 7 R1 R1 B1 B. r. ## B. B. B. B. B. B1
 8 ## R1 B. BT B1 ## B1 B. B. B. BM BM
 9 ## ## r. B. BT ## ## B. B. B1 B. B.
10 ## ## ## ## B. B2 B. B1 B. B. B. B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 55 | 97 |
| income | 33 | 36 |
| cells | 55 | 45 |
| army | 130 | 190 |
| mines | 3 | 3 |
| towers | 4 | 2 |
| exposed30 | 1 | 2 |
| hq_gap | 65 | 53 |
| material | 544 | 595 |
| voronoi | 59 | 45 |

### Pduhard- (rank 2, 9 losses, 8 with a decisive moment)

#### Game 917642158: we were red, lost at turn 20 (the rules ended the game)

Decisive moment: `large cut` at turn 18 (ply 35), played by blue. blue took red's cells from 34 to 16.

Their answer: `TRAIN 2 2 3;MOVE 41 4 10;TRAIN 1 1 3;MOVE 36 2 8;MOVE 40 4 3;MOVE 9 11 2;MOVE 39 3 3;MOVE 38 3 4`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 2 -> 18.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B1 B. ##
 1 R. RM R. R. R* R1 R. R3 R1 RT B.  .
 2 R. R. R. ## ## ## ## ## B1 B. B. B.
 3 ## R. R1 B1 r* r. r. r. B. B* B. B1
 4 ## R. ## B1 ## ## ## ## BT B* B. ##
 5 ## R. R1 B1  . ## ## ## B. B. B. ##
 6 ## R. R2 BT ## ## ## B. B. B. B. ##
 7 ## R. R2 B2 ## ## ## ## B. ## B. ##
 8  . R. R* B2 B. BT B. B* B. B. B. ##
 9  . R1 R. R1 ## ## ## ## ## B. B.  .
10  . R. R. R. R. B2 B. B* B. B. B* B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B1 B. ##
 1 R. RM R. R. R* R1 R. R3 R1 RT B.  .
 2 R. R. R. ## ## ## ## ## B1 B. B. B1
 3 ## B1 B2 B1 B1 r. r. r. B. B* B. B.
 4 ## r. ## B1 ## ## ## ## BT B* B. ##
 5 ## r. r. B.  . ## ## ## B. B. B. ##
 6 ## r. r. BT ## ## ## B. B. B. B. ##
 7 ## r. r* B2 ## ## ## ## B. ## B. ##
 8  . r. B2 B. B. BT B. B* B. B. B. ##
 9  . r. r. r. ## ## ## ## ## B. B.  .
10  . r. r. r. B2 B. B. B* B. B. B* B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 7 | 9 |
| income | 28 | 8 |
| cells | 46 | 34 |
| army | 120 | 130 |
| mines | 0 | 2 |
| towers | 3 | 1 |
| exposed30 | 9 | 18 |
| hq_gap | 63 | 91 |
| material | 425 | 309 |
| voronoi | 55 | 39 |

#### Game 917643731: we were red, lost at turn 15 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 32 to 12.

Their answer: `MOVE 18 4 3;MOVE 19 6 1;TRAIN 1 3 3;TRAIN 1 3 2;TRAIN 1 3 1;TRAIN 3 2 1;MOVE 13 7 3;MOVE 3 7 9;MOVE 8 5 5;MOVE 11 2 4;MOVE 7 3 8;MOVE 10 1 6;MOVE 4 11 7`

Our `exposed30` over our last 3 own plies before it, oldest first: 4 -> 7 -> 10.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. RT R1 R. R. RM R. R1 R1  *  . ##
 2 R. R. R. R. R1 R. B1 R. R1  .  . ##
 3 R. R. R1 R. R. B3 B. R. B2  . ## ##
 4  .  .  . R. RT R. B. R1 B.  . ## ##
 5  * B1 B1 R1 B1 B. BT  . B. B. B. ##
 6 ## B. B. B. B. B. B. B. B. B. B.  *
 7 ## ##  .  . B* B. B.  . B. B. B.  .
 8 ## ## B1 B* B. B. B. B.  . B* B. B1
 9 ##  .  .  .  . B. B1 B. B. B. B.  .
10 ##  .  *  .  .  .  *  . B. B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. RT B3 B1 r. rM B1 r. r.  *  . ##
 2 R. R. R. B1 r. r. B. r. r.  .  . ##
 3 R. R. R1 B1 B3 B. B. B2 B*  . ## ##
 4  .  . B1 r. rT r. B. r* B.  . ## ##
 5  * B. B. r. B. B1 BT  . B. B. B. ##
 6 ## B1 B. B. B. B. B. B. B. B. B.  *
 7 ## ##  .  . B* B. B.  . B. B. B. B1
 8 ## ## B. B1 B. B. B. B.  . B* B. B.
 9 ##  .  .  .  . B. B. B1 B. B. B.  .
10 ##  .  *  .  .  .  *  . B. B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 41 | 2 |
| income | 21 | 32 |
| cells | 52 | 32 |
| army | 120 | 80 |
| mines | 0 | 2 |
| towers | 1 | 2 |
| exposed30 | 8 | 10 |
| hq_gap | 39 | 108 |
| material | 442 | 326 |
| voronoi | 84 | 43 |

#### Game 917646585: we were red, lost at turn 19 (the rules ended the game)

Decisive moment: `large cut` at turn 12 (ply 23), played by blue. blue took red's cells from 19 to 8.

Their answer: `MOVE 14 3 3;TRAIN 1 2 3;TRAIN 2 2 2;MOVE 13 4 3;MOVE 7 8 2;MOVE 3 3 10;MOVE 9 5 5;MOVE 12 7 2;MOVE 6 4 9;MOVE 4 4 8;MOVE 15 5 2`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 2 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ##  .  *  .  .  .
 2 R. R. R1 R1 B1 r. r.  .  .  .  .  .
 3 ## ## R. RT B3 B1 B. B2 B1  .  . ##
 4 ## ## R1 R. RT R. B. B. B.  . ## ##
 5 ## ## ## R. RM B. BT B. ## ## ## ##
 6 ## ## ## ## R1 B1 B. B*  . ## ## ##
 7 ## ##  .  . B1 B. B. B. B.  . ## ##
 8 ##  .  .  . B1 B. B. B. B. B. ## ##
 9  .  .  . B1 B. B. B. B* B. B. B.  .
10  .  .  .  *  . ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. B* BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ##  .  *  .  .  .
 2 R. R. B2 r. B* B1 r. B2 B1  .  .  .
 3 ## ## B1 B3 B1 B. B. B. B.  .  . ##
 4 ## ## r. r. rT r. B. B. B.  . ## ##
 5 ## ## ## r. rM B1 BT B. ## ## ## ##
 6 ## ## ## ## r. B. B. B*  . ## ## ##
 7 ## ##  .  . B. B. B. B. B.  . ## ##
 8 ##  .  .  . B1 B. B. B. B. B. ## ##
 9  .  .  . B. B1 B. B. B* B. B. B.  .
10  .  .  . B1  . ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 24 | 7 |
| income | 9 | 23 |
| cells | 40 | 19 |
| army | 120 | 40 |
| mines | 0 | 2 |
| towers | 1 | 2 |
| exposed30 | 4 | 2 |
| hq_gap | 56 | 123 |
| material | 341 | 212 |
| voronoi | 75 | 19 |

### karliso (rank 3, 10 losses, 9 with a decisive moment)

#### Game 917642388: we were red, lost at turn 16 (the rules ended the game)

Decisive moment: `large cut` at turn 14 (ply 27), played by blue. blue took red's cells from 39 to 3.

Their answer: `TRAIN 3 2 6;TRAIN 1 2 5;TRAIN 2 1 5;TRAIN 2 1 4;TRAIN 1 1 3;TRAIN 2 1 2;TRAIN 1 1 1;TRAIN 1 1 0;MOVE 3 0 9;MOVE 12 11 2;MOVE 7 8 6;MOVE 6 9 8`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 3 -> 3.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R* R1 R. R* R. R. R. R1 R. R1  .
 2 R. R1 R. ## ## ## ## ## R. R1 B1  .
 3 ## R. R. R. R* R. R. R. R2 R* B.  .
 4 ## R1 ## R. ## ## ## ## B. BM BT ##
 5 ## R1 R. R1 R. ## ## ## BT B. B. ##
 6 ## R1 RT R. ## ## ## B1 B. B. B. ##
 7 ## R1 B1 B. ## ## ## ## B. ## B. ##
 8  . R2 BM BT B. B. B. BM B. B. B. ##
 9  .  .  .  . ## ## ## ## ## B1 B. B.
10 B1 B. B. B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH B1 r. ## ## ## ## ## ##  .  . ##
 1 R. B1 r. r. r* r. r. r. r. r. r.  .
 2 R. B2 r. ## ## ## ## ## r. r. B. B1
 3 ## B1 r. r. r* r. r. r. r. r* B.  .
 4 ## B2 ## r. ## ## ## ## B. BM BT ##
 5 ## B2 B1 r. r. ## ## ## BT B. B. ##
 6 ## r. B3 r. ## ## ## B. B1 B. B. ##
 7 ## r. B1 B. ## ## ## ## B. ## B. ##
 8  . r. BM BT B. B. B. BM B. B1 B. ##
 9 B1  .  .  . ## ## ## ## ## B. B. B.
10 B. B. B. B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 69 | 62 |
| income | 62 | 25 |
| cells | 44 | 39 |
| army | 60 | 140 |
| mines | 6 | 1 |
| towers | 3 | 1 |
| exposed30 | 2 | 3 |
| hq_gap | 61 | 68 |
| material | 521 | 443 |
| voronoi | 53 | 43 |

#### Game 917643894: we were red, lost at turn 24 (the rules ended the game)

Decisive moment: `large cut` at turn 17 (ply 33), played by blue. blue took red's cells from 42 to 13.

Their answer: `MOVE 29 2 5;TRAIN 3 2 4;TRAIN 1 2 3;TRAIN 1 2 2;TRAIN 1 2 1;TRAIN 1 2 0;MOVE 18 2 6;MOVE 25 2 9;MOVE 22 8 11;MOVE 24 5 4;MOVE 13 6 3;MOVE 16 9 4;MOVE 3 8 8;MOVE 12 7 7;MOVE 8 3 6`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 6 -> 5.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  . R1 ##  .  . ## ##
 1 R. RT  . RT RT RM R. R. R.  *  . ##
 2 R1 R. R. R. R. R. R1 R1 R1  .  . ##
 3 R. R. RM R. R. R. R. B1 R1  . ## ##
 4 R1 R1 R. RT RT R. B1 BM  .  . ## ##
 5 R1 R1 R. R. R. B. B. BT B. B1  . ##
 6 ## R. B3 B2 B. BT B. B. B. B.  .  *
 7 ## ## rT B1 BM B. B1 B. BT B. B.  .
 8 ## ## r. BM BT B. BT B1 B.  * B. B.
 9 ## r. r. B1 B. B. B. B. B. B. BT B.
10 ## r. BM BT B. B. BM B. B. B. B. B.
11 ## ## B.  . ##  * B. B1  .  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM B1 r.  .  . r* ##  .  . ## ##
 1 R. RT B1 rT rT rM r. r. r.  *  . ##
 2 R1 R. B1 r. r. r. r. r. r.  .  . ##
 3 R. R. B1 r. r. r. B1 B. r*  . ## ##
 4 R1 R1 B3 rT rT B1 B. BM  . B1 ## ##
 5 R1 R1 B3 r. r. B. B. BT B. B.  . ##
 6 ## R. B2 B1 B. BT B. B. B. B.  .  *
 7 ## ## rT B. BM B. B. B1 BT B. B.  .
 8 ## ## r. BM BT B. BT B. B1  * B. B.
 9 ## r. B1 B. B. B. B. B. B. B. BT B.
10 ## r. BM BT B. B. BM B. B. B. B. B.
11 ## ## B.  . ##  * B. B. B1  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 30 | 74 |
| income | 43 | 44 |
| cells | 55 | 42 |
| army | 130 | 100 |
| mines | 5 | 3 |
| towers | 7 | 5 |
| exposed30 | 2 | 5 |
| hq_gap | 70 | 66 |
| material | 579 | 524 |
| voronoi | 74 | 52 |

#### Game 917645592: we were red, lost at turn 19 (the rules ended the game)

Decisive moment: `HQ threat` at turn 19 (ply 37), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `TRAIN 2 7 4;TRAIN 3 7 3;TRAIN 3 7 2;TRAIN 2 7 1;TRAIN 2 6 1;TRAIN 2 5 1;TRAIN 3 4 1;TRAIN 3 3 1;TRAIN 2 2 1;TRAIN 1 1 1;TRAIN 1 0 1;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 4 -> 1 -> 1.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R1 R. ## ## ## ## ## ## ## ##
 1 R. R. R1 RT R. R1 R1 R1 ## ## ## ##
 2 R. R. R. R. R. ## ## RT R1 bT ## ##
 3 RM R* R. R. RT R1 ## R. R. R. R1 ##
 4 R. R. R1 R. R. R. ## R1 R3 R3 R1 B.
 5 R. R1 R. R. R.  . ## B1 B. BT B. BT
 6 R1 R. R. R. R. ## B1 B1 B1 B. B. B.
 7 R. R. R. R. R1 ## B. B1 B. B. B. B.
 8 ##  . R. R. R1 ## B. B1 B. B. BM BM
 9 ## ## R1 RT R. ## ## B. BT B. B. B.
10 ## ## ## ## R1 R1 R1 R1 B1 B. BT B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 228 | 29 |
| income | 42 | 12 |
| cells | 37 | 64 |
| army | 70 | 260 |
| mines | 3 | 2 |
| towers | 4 | 4 |
| exposed30 | 1 | 1 |
| hq_gap | 12 | 61 |
| material | 612 | 621 |
| voronoi | 37 | 67 |

### _iaz (rank 4, 8 losses, 8 with a decisive moment)

#### Game 917644176: we were red, lost at turn 16 (the rules ended the game)

Decisive moment: `large cut` at turn 9 (ply 17), played by blue. blue took red's cells from 27 to 17.

Their answer: `MOVE 3 8 4;MOVE 8 4 5;TRAIN 2 4 4;TRAIN 1 4 3;TRAIN 1 4 2;MOVE 6 4 7;MOVE 7 6 4;MOVE 4 7 10`

Our `exposed30` over our last 3 own plies before it, oldest first: 0 -> 2 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R.  * R1  .  .  *  . ##
 2 R. R. R. R. R. R. R.  .  .  .  . ##
 3 R. R. R* R. R. R. R.  .  *  . ## ##
 4  .  . R1  . R1 R1 R. B1  . B1 ## ##
 5  *  .  .  .  . B1 BT B.  . B. B. ##
 6 ##  .  .  .  .  . B. B. B.  . B.  *
 7 ## ##  .  .  * B1 B. B. B. B. B.  .
 8 ## ##  .  *  .  .  . B. B.  * B. B.
 9 ##  .  .  .  .  .  . B1 B. B. B. B.
10 ##  .  *  .  .  .  *  .  . B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R.  * r.  .  .  *  . ##
 2 R. R. R. R. B1 r. r.  .  .  .  . ##
 3 R. R. R* R. B1 r. r.  .  *  . ## ##
 4  .  . R1  . B2 r. B1 B* B1 B. ## ##
 5  *  .  .  . B1 B. BT B.  . B. B. ##
 6 ##  .  .  .  .  . B. B. B.  . B.  *
 7 ## ##  .  . B1 B. B. B. B. B. B.  .
 8 ## ##  .  *  .  .  . B. B.  * B. B.
 9 ##  .  .  .  .  .  . B. B. B. B. B.
10 ##  .  *  .  .  .  * B1  . B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 23 | 53 |
| income | 25 | 27 |
| cells | 30 | 27 |
| army | 50 | 40 |
| mines | 0 | 1 |
| towers | 1 | 0 |
| exposed30 | 2 | 4 |
| hq_gap | 77 | 77 |
| material | 278 | 282 |
| voronoi | 77 | 53 |

#### Game 917645736: we were red, lost at turn 18 (the rules ended the game)

Decisive moment: `large cut` at turn 11 (ply 21), played by blue. blue took red's cells from 28 to 17.

Their answer: `MOVE 14 4 4;TRAIN 1 4 3;TRAIN 1 3 3;TRAIN 1 2 3;MOVE 8 1 7;MOVE 4 7 4;MOVE 3 9 3;MOVE 6 9 9;MOVE 7 3 6;MOVE 13 4 5`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 3 -> 8.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. R. R. R. R1  . ## ## ## ##
 2 R. R. R. R. R. ## ##  .  .  . ## ##
 3 RM RM R. R. R.  . ##  .  .  . B1 ##
 4  .  . R. R. R.  . ##  .  .  . B. B.
 5  . R1 R. R1 B1 r. ## B1  .  .  . B.
 6  . b. R2 R. B2 ##  . B.  .  .  . B.
 7  .  . B. B1 B. ##  . B.  .  .  . B.
 8 ## B1 B. BT BT ##  . B. B. B1 B* B*
 9 ## ## B. B. B. ## ## B. B.  . B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ## B. B.  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. R. R. R. R1  . ## ## ## ##
 2 R. R. R. R. R. ## ##  .  .  . ## ##
 3 RM RM B1 B1 B1  . ##  .  . B1 B. ##
 4  .  . r. r. B1  . ## B1  .  . B. B.
 5  . r. r. r. B2 r. ## B.  .  .  . B.
 6  . B. r. B1 B. ##  . B.  .  .  . B.
 7  . B1 B. B. B. ##  . B.  .  .  . B.
 8 ## B. B. BT BT ##  . B. B. B. B* B*
 9 ## ## B. B. B. ## ## B. B. B1 B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ## B. B.  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 4 | 71 |
| income | 30 | 33 |
| cells | 42 | 28 |
| army | 100 | 50 |
| mines | 0 | 3 |
| towers | 2 | 0 |
| exposed30 | 3 | 8 |
| hq_gap | 66 | 79 |
| material | 382 | 332 |
| voronoi | 70 | 38 |

#### Game 917650068: we were red, lost at turn 16 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 42 to 24.

Their answer: `TRAIN 3 5 4;TRAIN 2 5 3;TRAIN 1 5 2;TRAIN 1 5 1;TRAIN 1 4 2;MOVE 3 4 10;MOVE 16 10 3;MOVE 4 11 4;MOVE 13 6 5`

Our `exposed30` over our last 3 own plies before it, oldest first: 4 -> 6 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## R. R. R1  . ## ## ##
 1 R. R. R. R. R1 R* R.  *  .  . ## ##
 2 R. R1 R. R. R. R. R.  .  .  .  . ##
 3 ## R. R. R. R1 R1 R. R1 R.  *  . B1
 4 ## R. ## ## R* R. RT R. R1 R. B. B.
 5  . RT R1 ## ## BT B. R1 B1 B. BT B1
 6 b* b. R3 B. B. B. B2 ## ## B. B. B.
 7 b. b. R1 B. B. BT B. B* ## ## B. ##
 8  .  . R1 R1 B* B. B. B. B.  . B. ##
 9 ## b. b.  .  . B. B. B. B. B. B.  .
10 ## ##  .  .  * B. B* B.  . B. B. B.
11 ## ## ##  . B1 B.  . ## ##  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## r. r. r.  . ## ## ##
 1 R. R. R. R. R1 B1 r.  *  .  . ## ##
 2 R. R1 R. R. B1 B1 r.  .  .  .  . ##
 3 ## R. R. R. R1 B2 r. r* r.  * B1 B.
 4 ## R. ## ## R* B3 rT r. r. r. B. B1
 5  . RT R1 ## ## BT B2 r. B1 B. BT B*
 6 b* b. R3 B. B. B. B. ## ## B. B. B.
 7 b. b. R1 B. B. BT B. B* ## ## B. ##
 8  .  . R1 R1 B* B. B. B. B.  . B. ##
 9 ## b. b.  .  . B. B. B. B. B. B.  .
10 ## ##  .  . B1 B. B* B.  . B. B. B.
11 ## ## ##  . B. B.  . ## ##  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 54 | 2 |
| income | 42 | 14 |
| cells | 53 | 42 |
| army | 90 | 150 |
| mines | 0 | 1 |
| towers | 4 | 2 |
| exposed30 | 2 | 4 |
| hq_gap | 46 | 108 |
| material | 522 | 382 |
| voronoi | 69 | 61 |

### reCurse (rank 5, 7 losses, 7 with a decisive moment)

#### Game 917644354: we were red, lost at turn 19 (the rules ended the game)

Decisive moment: `large cut` at turn 16 (ply 31), played by blue. blue took red's cells from 37 to 11.

Their answer: `MOVE 28 3 7;TRAIN 1 2 7;TRAIN 1 2 6;TRAIN 1 2 5;TRAIN 1 2 4;TRAIN 2 2 3;TRAIN 1 2 2;TRAIN 1 2 1;TRAIN 1 2 0;TRAIN 1 1 5;MOVE 25 4 3;TRAIN 1 3 4;MOVE 3 9 0;MOVE 8 7 9`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 3 -> 3.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. RT  . RT R. RM R1 B1 B. B1  . ##
 2 R. R. R. R. R. R1 B. B. B. B.  . ##
 3 R. R. R1 R. R. B1 BT B. BM  . ## ##
 4 R. R. R. R. RT BT B. BM B.  . ## ##
 5 R1 R. R. R. RT B1 B. B. B. B. B. ##
 6 ## R1 R. R1 R. R1 B. BT B. B. B.  *
 7 ## ##  .  . R1 B2 B. B. B. B. B.  .
 8 ## ##  . B2 B. B. B. B1 B. BM B.  .
 9 ##  .  .  .  . B. B. r. B. B1 B.  .
10 ##  .  *  .  .  .  * B. B. B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM B1 r.  .  .  * ##  . B1 ## ##
 1 R. RT B1 rT r. rM r. B1 B. B*  . ##
 2 R. R. B1 r. r. r. B. B. B. B.  . ##
 3 R. R. B2 r. B1 B. BT B. BM  . ## ##
 4 R. R. B1 B1 rT BT B. BM B.  . ## ##
 5 R1 B1 B1 r. rT B1 B. B. B. B. B. ##
 6 ## r. B1 r. r. r. B. BT B. B. B.  *
 7 ## ## B1 B2 r* B2 B. B. B. B. B.  .
 8 ## ##  . B* B. B. B. B. B. BM B.  .
 9 ##  .  .  .  . B. B. B1 B. B1 B.  .
10 ##  .  *  .  .  .  * B. B. B. B. B.
11 ## ##  .  . ##  *  .  .  .  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 63 | 62 |
| income | 54 | 37 |
| cells | 52 | 37 |
| army | 100 | 80 |
| mines | 4 | 2 |
| towers | 3 | 4 |
| exposed30 | 4 | 3 |
| hq_gap | 67 | 38 |
| material | 563 | 441 |
| voronoi | 83 | 42 |

#### Game 917645840: we were red, lost at turn 15 (the rules ended the game)

Decisive moment: `HQ threat` at turn 15 (ply 29), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `TRAIN 2 2 3;TRAIN 3 1 3;TRAIN 3 1 2;TRAIN 1 0 2;TRAIN 2 0 1;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 3 -> 1.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM RT  . ## ## ## ## ## ## ## ##
 1 R1 R. R. RT R. B1 B1 B1 ## ## ## ##
 2 R. RT R. R1 B1 ## ## B. B. B. ## ##
 3 R1 R* R1 B1 BT B1 ## B. B. B. B1 ##
 4 R1 R. B1 r. B. B. ## B. B. B. B.  .
 5 R. RT B1 B1 B. B. ## B. B. B. B.  .
 6 R2 R. B1 B. BT ##  . B. B. B1 B.  .
 7 R. B1 BT B. B. ##  .  . B.  . B.  .
 8 ## B. B. B. B. ##  .  . B.  . BM  *
 9 ## ## B. B. B. ## ##  . B.  . B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ##  .  .  * BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 77 | 48 |
| income | 53 | 17 |
| cells | 62 | 22 |
| army | 130 | 70 |
| mines | 1 | 1 |
| towers | 3 | 4 |
| exposed30 | 2 | 1 |
| hq_gap | 43 | 142 |
| material | 644 | 297 |
| voronoi | 81 | 23 |

#### Game 917650244: we were red, lost at turn 25 (the rules ended the game)

Decisive moment: `HQ threat` at turn 25 (ply 49), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `TRAIN 3 1 7;TRAIN 3 1 6;TRAIN 1 1 5;TRAIN 1 1 4;TRAIN 2 1 3;TRAIN 1 1 2;TRAIN 1 0 2;TRAIN 1 0 1;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 5 -> 4 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## R. R. R. R1 ## ## ##
 1 R. R. R. R. R1 R* R. R* R. R. ## ##
 2 R. R. R. R. R. R1 R. RT R. R1 R1 ##
 3 ## R1 R. R. R1 R. R. b* R. R1 B. B.
 4 ## R. ## ## R1 R. RT R. R2 B1 BT B1
 5 R. R. R1 ## ## R. R. R1 R1 B1 BT B1
 6 R* RT R. B1 B. R3 B. ## ## B1 B1 B.
 7 R. R. B. BT B. BT B. BM ## ## B. ##
 8 R1 R. R1 B. BM BT B. B. B. B. B. ##
 9 ## R1 R1 B. B1 B. B. B. BT B. B1 B.
10 ## ## B. BT BM B. BM B. B. B. B. B.
11 ## ## ## B2 B. B. B. ## ## B. BM BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 130 | 69 |
| income | 63 | 20 |
| cells | 56 | 60 |
| army | 110 | 210 |
| mines | 5 | 0 |
| towers | 8 | 3 |
| exposed30 | 1 | 4 |
| hq_gap | 10 | 81 |
| material | 733 | 609 |
| voronoi | 56 | 61 |

### _iahelka (rank 6, 9 losses, 9 with a decisive moment)

#### Game 917642852: we were red, lost at turn 17 (the rules ended the game)

Decisive moment: `HQ threat` at turn 17 (ply 33), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `TRAIN 3 2 2;TRAIN 2 2 1;TRAIN 2 2 0;TRAIN 1 1 0;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 2 -> 1.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R1 ## ## ## ## ## ## B2  . ##
 1 R1 R* R1 R1 R1 B3 B. B. B. B. B. B1
 2 R. RT R. ## ## ## ## ## B. B. B.  .
 3 ## R. B2 B1 B1 B. B. B2 B1 B1 BT  .
 4 ## B1 ## B1 ## ## ## ## B. BM B. ##
 5 ## B1 BT B. B. ## ## ## B. B. B. ##
 6 ## B. B. B. ## ## ##  . B. B. B. ##
 7 ## B. BM B. ## ## ## ## B. ## B. ##
 8 B1 B. BM B. B. B. B. BM B. B. B. ##
 9 B. B1 B. B1 ## ## ## ## ## B. B.  .
10 B. B. B. B. B. B. B. BM B. B. BM B.
11 ## B. B. ## ## ## ## ## ## B. BM BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 60 | 27 |
| income | 62 | 11 |
| cells | 77 | 12 |
| army | 200 | 50 |
| mines | 7 | 1 |
| towers | 2 | 1 |
| exposed30 | 1 | 1 |
| hq_gap | 30 | 173 |
| material | 774 | 168 |
| voronoi | 82 | 12 |

#### Game 917644489: we were red, lost at turn 14 (the rules ended the game)

Decisive moment: `large cut` at turn 11 (ply 21), played by blue. blue took red's cells from 33 to 19.

Their answer: `MOVE 3 5 6;MOVE 4 3 7;MOVE 6 4 6;MOVE 8 6 6;MOVE 10 5 10;MOVE 15 4 5;TRAIN 1 4 4;TRAIN 1 4 3;TRAIN 1 4 2`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 4 -> 5.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R.  *  .  . R1  *  . ##
 2 R. R. R. R. R. R. R. R. R.  .  . ##
 3 R. R. RM R. R. R. R. b. b*  . ## ##
 4 R1 R. R.  .  . R1 R1 R1 R1 R1 ## ##
 5  *  .  .  .  . B1 B1 B1 B. BT  . ##
 6 ##  .  .  .  . r. B. BT  . B.  .  *
 7 ## ##  .  . B1 B1 B. B. B. B.  .  .
 8 ## ##  . B1  . B. B. B. B. B* B.  .
 9 ##  .  . B.  . B. B. B. B.  . B.  .
10 ##  .  * B. B. B. B* B. B.  . B. B.
11 ## ##  .  . ## B1 B. B. B. B. B* BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. R.  .  .  * ##  .  . ## ##
 1 R. R.  . R. R.  *  .  . r.  *  . ##
 2 R. R. R. R. B1 r. r. r. r.  .  . ##
 3 R. R. RM R. B1 r. r. b. b*  . ## ##
 4 R1 R. R.  . B1 r. r. r* r. r. ## ##
 5  *  .  .  . B1 B. B. B1 B. BT  . ##
 6 ##  .  .  . B1 B1 B1 BT  . B.  .  *
 7 ## ##  . B1 B* B. B. B. B. B.  .  .
 8 ## ##  . B*  . B. B. B. B. B* B.  .
 9 ##  .  . B.  . B. B. B. B.  . B.  .
10 ##  .  * B. B. B1 B* B. B.  . B. B.
11 ## ##  .  . ## B* B. B. B. B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 23 | 43 |
| income | 37 | 34 |
| cells | 46 | 33 |
| army | 90 | 70 |
| mines | 0 | 2 |
| towers | 2 | 0 |
| exposed30 | 4 | 5 |
| hq_gap | 77 | 67 |
| material | 428 | 347 |
| voronoi | 73 | 56 |

#### Game 917649050: we were red, lost at turn 17 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 26 to 12.

Their answer: `MOVE 4 2 8;MOVE 11 4 3;TRAIN 2 4 2;MOVE 9 4 6`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 3 -> 3.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ##  . R1  .  .  .
 2 R. R. R. R1 R1 R. R. R. R.  .  .  .
 3 ## ## RT R. RT R. R. R. R.  .  . ##
 4 ## ##  .  . B3 BT R1 R.  .  . ## ##
 5 ## ## ##  . r* B. B1 R1 ## ## ## ##
 6 ## ## ## ## r. B. B. B1 B1 ## ## ##
 7 ## ##  . B. B1 BT B. BT B. B. ## ##
 8 ##  .  . B. B.  . B. B. B. B. ## ##
 9  .  . B1 B. B. B. B. B* B. B. B.  .
10  .  .  .  *  . ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  .  . ##
 1 R. R. R. ## ## ## ##  . r*  .  .  .
 2 R. R. R. R1 B2 r. r. r. r.  .  .  .
 3 ## ## RT R. B3 r. r. r. r.  .  . ##
 4 ## ##  .  . B. BT r. r.  .  . ## ##
 5 ## ## ##  . r* B. B1 r. ## ## ## ##
 6 ## ## ## ## B1 B. B. B1 B1 ## ## ##
 7 ## ##  . B. B. BT B. BT B. B. ## ##
 8 ##  . B1 B. B.  . B. B. B. B. ## ##
 9  .  . B. B. B. B. B. B* B. B. B.  .
10  .  .  .  *  . ## ## ## ## B. B. B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 19 | 59 |
| income | 15 | 25 |
| cells | 36 | 26 |
| army | 80 | 50 |
| mines | 1 | 1 |
| towers | 3 | 2 |
| exposed30 | 2 | 3 |
| hq_gap | 71 | 81 |
| material | 318 | 308 |
| voronoi | 53 | 41 |

### Waffle3z (rank 7, 7 losses, 6 with a decisive moment)

#### Game 914253190: we were blue, lost at turn 17 (the rules ended the game)

Decisive moment: `HQ threat` at turn 17 (ply 32), played by red. red can pay a chain of trainings to blue's HQ this turn, for the first time.

Their answer: `TRAIN 3 4 8;TRAIN 3 4 9;TRAIN 1 5 9;TRAIN 1 5 10;TRAIN 1 5 11;TRAIN 1 6 11;TRAIN 2 7 11;TRAIN 3 8 11;TRAIN 2 9 11;TRAIN 3 10 11;TRAIN 3 11 11`

Our `exposed30` over our last 3 own plies before it, oldest first: 1 -> 2 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH  *  . R. R. R. RM R. ## ## ## ##
 1 R. R. R. R. R1 R. R. R. R. ## ## ##
 2 R. R. R. R. R. R. R. R. R. ## ## ##
 3 ## ## ## ## R. R. R. R. ## ## ## ##
 4 ## ## R. R1 R. R. R. R. ## ##  . ##
 5 ## R. R.  . R. R. RT R. B1 r. r.  .
 6  . R. R. R. RT R. R. R1 B3 r. r. ##
 7 ##  . ## ## R1 B. BT B. B* r. ## ##
 8 ## ## ## ## BT B1 B. B1 ## ## ## ##
 9 ## ## ## B. B1 B. B. BT B. B1 B. B1
10 ## ## ## B. B1 B. B1 B. BT B. BT BT
11 ## ## ## ## B. B* B. B1 B. B1 B* BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 187 | 0 |
| income | 49 | 6 |
| cells | 55 | 36 |
| army | 80 | 130 |
| mines | 1 | 0 |
| towers | 3 | 6 |
| exposed30 | 1 | 4 |
| hq_gap | 33 | 150 |
| material | 664 | 352 |
| voronoi | 62 | 43 |

#### Game 917643076: we were red, lost at turn 16 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 34 to 12.

Their answer: `MOVE 17 5 3;TRAIN 1 4 3;TRAIN 2 3 3;TRAIN 1 2 3;TRAIN 2 1 3;TRAIN 1 1 2;MOVE 18 10 1;MOVE 4 1 10;MOVE 6 11 3;MOVE 16 9 1;MOVE 20 9 2;MOVE 7 8 2`

Our `exposed30` over our last 3 own plies before it, oldest first: 5 -> 4 -> 4.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  . B1 ##
 1 R. RM R. R. RM R. RT B1 B1 B. B.  .
 2 R. R. R1 ## ## ## ## ## B1 B1 B. B1
 3 ## R1 R. R1 R* R1 B2 B. B1 B* B.  .
 4 ## R. ## R. ## ## ## ## B. B* B. ##
 5 ## R. R. R.  . ## ## ## B. B. B. ##
 6 ## R. R. R. ## ## ##  . B. B. B. ##
 7 ## R. R1 R. ## ## ## ## B. ## B. ##
 8 R. R1 R* R1 B1 BT B. BM B. B. B. ##
 9 R1 B1 B. BT ## ## ## ## ## B. B.  .
10  .  .  . B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ##  . B. ##
 1 R. RM R. R. RM R. RT B1 B1 B1 B1  .
 2 R. B1 R1 ## ## ## ## ## B1 B1 B. B.
 3 ## B2 B1 B2 B1 B2 B. B. B. B* B. B1
 4 ## r. ## r. ## ## ## ## B. B* B. ##
 5 ## r. r. r.  . ## ## ## B. B. B. ##
 6 ## r. r. r. ## ## ##  . B. B. B. ##
 7 ## r. r* r. ## ## ## ## B. ## B. ##
 8 r. r. r* r. B1 BT B. BM B. B. B. ##
 9 r. B. B. BT ## ## ## ## ## B. B.  .
10  . B1  . B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 29 | 77 |
| income | 46 | 38 |
| cells | 47 | 34 |
| army | 110 | 80 |
| mines | 3 | 3 |
| towers | 2 | 1 |
| exposed30 | 3 | 4 |
| hq_gap | 71 | 53 |
| material | 485 | 417 |
| voronoi | 58 | 36 |

#### Game 917646172: we were red, lost at turn 14 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 19 to 13.

Their answer: `MOVE 26 4 2;MOVE 24 3 3;MOVE 8 2 4;TRAIN 1 1 4;TRAIN 1 1 3;TRAIN 1 1 2;TRAIN 1 0 2;MOVE 25 5 4;MOVE 11 4 5;MOVE 4 0 7;MOVE 17 10 3;MOVE 6 8 7;MOVE 19 9 3;MOVE 23 4 6;MOVE 18 8 5`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 7 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. RT R. R. R1 B1 ## ## ## ##
 2 R. R. R. R1 R. ## ## B. B. B1 ## ##
 3 R* R* R1 R. B1 r. ## B. B.  .  . ##
 4  .  .  . B2 B1 r. ## B. BT B. B1  .
 5  .  . B1 B1 r. r. ## B1 B. B.  .  .
 6  .  . B. BT B. ## B. B. B1 B.  .  .
 7  . B1 B. B. B1 ## B. B. r. B.  .  .
 8 ##  . B. B. BT ## B. B.  . B. BM  *
 9 ## ## B. B. B. ## ## B. B.  . B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ##  .  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. RT R. R. R1 B1 ## ## ## ##
 2 B1 B1 R. R1 B1 ## ## B. B. B. ## ##
 3 r* B1 R1 B2 B. r. ## B. B. B1 B1 ##
 4  . B1 B1 B. B. B1 ## B. BT B. B.  .
 5  .  . B. B. B1 r. ## B. B1 B.  .  .
 6  .  . B. BT B1 ## B. B. B. B.  .  .
 7 B1 B. B. B. B. ## B. B. B1 B.  .  .
 8 ##  . B. B. BT ## B. B.  . B. BM  *
 9 ## ## B. B. B. ## ## B. B.  . B.  .
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ##  .  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 4 | 41 |
| income | 43 | 20 |
| cells | 54 | 19 |
| army | 130 | 30 |
| mines | 1 | 1 |
| towers | 3 | 1 |
| exposed30 | 3 | 2 |
| hq_gap | 66 | 139 |
| material | 509 | 217 |
| voronoi | 81 | 23 |

### Zylo (rank 8, 9 losses, 9 with a decisive moment)

#### Game 917643294: we were red, lost at turn 19 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 22 to 15.

Their answer: `MOVE 23 2 6;MOVE 12 7 1;MOVE 4 2 7;MOVE 3 10 0;MOVE 6 8 6;MOVE 9 9 9;TRAIN 2 3 5;TRAIN 2 3 4;TRAIN 1 3 3`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 4 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B1  . ##
 1 R. R* R. R. R1 RT B2 B. B. B1 B1  .
 2 R. R1 R. ## ## ## ## ## B. B. B.  .
 3 ## R. R1 R. R* R. R. R1 B. B* B.  .
 4 ## RT ## R1 ## ## ## ## BT B* B. ##
 5 ## B2 B1 R1  . ## ## ## B. B. B. ##
 6 ## B1 r. B1 ## ## ## B1 B. B. B. ##
 7 ## B1 r* B1 ## ## ## ## B. ## B. ##
 8  . B. B* BT  .  .  .  * B. B. B. ##
 9  .  .  . B. ## ## ## ## ## B. B1 B.
10  .  .  . B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B1 B1 ##
 1 R. R* R. R. R1 RT B. B2 B. B1 B.  .
 2 R. R1 R. ## ## ## ## ## B. B. B.  .
 3 ## R. R1 B1 r* r. r. r. B. B* B.  .
 4 ## RT ## B2 ## ## ## ## BT B* B. ##
 5 ## B2 B1 B2  . ## ## ## B. B. B. ##
 6 ## B. B1 B1 ## ## ## B. B1 B. B. ##
 7 ## B. B1 B1 ## ## ## ## B. ## B. ##
 8  . B. B* BT  .  .  .  * B. B. B. ##
 9  .  .  . B. ## ## ## ## ## B1 B. B.
10  .  .  . B. B. B. B. BM B. B. BM B.
11 ##  .  . ## ## ## ## ## ## B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 4 | 9 |
| income | 46 | 20 |
| cells | 52 | 22 |
| army | 140 | 60 |
| mines | 3 | 1 |
| towers | 2 | 2 |
| exposed30 | 2 | 2 |
| hq_gap | 56 | 141 |
| material | 510 | 237 |
| voronoi | 71 | 23 |

#### Game 917644936: we were red, lost at turn 25 (the rules ended the game)

Decisive moment: `large cut` at turn 15 (ply 29), played by blue. blue took red's cells from 37 to 18.

Their answer: `MOVE 20 5 0;MOVE 19 6 2;MOVE 9 8 2;MOVE 6 9 1;MOVE 25 8 1;MOVE 18 2 10;MOVE 26 10 2;MOVE 8 3 11;TRAIN 2 5 4;TRAIN 1 4 4;TRAIN 2 4 3;TRAIN 1 3 3;TRAIN 1 2 3;TRAIN 1 1 3`

Our `exposed30` over our last 3 own plies before it, oldest first: 3 -> 2 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. RT R1 R. B1 ## B1 B1 ## ##
 1 R. RT  . R. R. RM B2 B. B1 B* B1 ##
 2 R. R. R. R. R. RT B. B1 B. B.  . ##
 3 R. R. R* R. R1 R. B. BT B* B. ## ##
 4  . R. R. R. R. R1 BT B* B.  . ## ##
 5  * R1 R. RT R. B1 B. BT B.  .  . ##
 6 ##  . R1 R. R1 B. BT B. B.  .  .  *
 7 ## ## R. R1 B* B. B. B. B.  .  .  .
 8 ## ## B. B* BT B. B. B. B. BM  .  .
 9 ##  . B2 B.  .  . B. B. B. B. B.  .
10 ##  .  * B1 B. B. BM B. B. B. B. B.
11 ## ##  .  . ##  *  . B. B. B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. RT R1 B1 B* ## B. B1 ## ##
 1 R. RT  . R. R. RM B. B. B1 B1 B. ##
 2 R. R. R. R. R. RT B2 B. B1 B. B1 ##
 3 R. B1 B1 B1 B2 R. B. BT B* B. ## ##
 4  . r. r. r. B1 B2 BT B* B.  . ## ##
 5  * r. r. rT r. B1 B. BT B.  .  . ##
 6 ##  . r. r. r. B. BT B. B.  .  .  *
 7 ## ## r. r. B* B. B. B. B.  .  .  .
 8 ## ## B. B* BT B. B. B. B. BM  .  .
 9 ##  . B. B.  .  . B. B. B. B. B.  .
10 ##  . B2 B. B. B. BM B. B. B. B. B.
11 ## ##  . B1 ##  *  . B. B. B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 32 | 59 |
| income | 58 | 38 |
| cells | 62 | 37 |
| army | 120 | 70 |
| mines | 3 | 2 |
| towers | 5 | 4 |
| exposed30 | 2 | 2 |
| hq_gap | 78 | 81 |
| material | 624 | 431 |
| voronoi | 84 | 41 |

#### Game 917646278: we were red, lost at turn 19 (the rules ended the game)

Decisive moment: `HQ threat` at turn 19 (ply 37), played by blue. blue can pay a chain of trainings to red's HQ this turn, for the first time.

Their answer: `TRAIN 3 2 8;TRAIN 1 2 7;TRAIN 1 2 6;TRAIN 1 1 6;TRAIN 1 1 5;TRAIN 3 1 4;TRAIN 1 1 3;TRAIN 1 0 3;TRAIN 3 0 2;TRAIN 1 0 1;TRAIN 1 0 0`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 2 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R.  . ## ## ## ## ## ## ## ##
 1 R. R. R. R. R. R. R1 R. ## ## ## ##
 2 R. RT R. R. R. ## ## RT R. R1 ## ##
 3 R*  *  . RT R. R. ## R1 B1 B. B. ##
 4 R1 R. RT R. R. R. ## R1 B. BT B. B.
 5 R1 R. R. R1 RT R. ## B. BT B2 B. B1
 6 R. R. R. R1 R1 ## B. B. B. B. B. B1
 7 R. R. R. R2 B. ## B. B. B. B. B. B1
 8 ## R1 R2 B. BT ## B. B. BT B. BM B1
 9 ## ## R1 B. BT ## ## B. B. B. B. B1
10 ## ## ## ## B. B. B. B. B. B. B. B.
11 ## ## ## ## ## ## ## ## B. B. BM BH
```

No board after: the game ended on this ply.

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 154 | 79 |
| income | 50 | 34 |
| cells | 52 | 49 |
| army | 80 | 150 |
| mines | 2 | 1 |
| towers | 5 | 5 |
| exposed30 | 1 | 2 |
| hq_gap | 6 | 71 |
| material | 642 | 577 |
| voronoi | 52 | 52 |

### NinjaDoggy (rank 9, 10 losses, 10 with a decisive moment)

#### Game 914359832: we were blue, lost at turn 25 (the rules ended the game)

Decisive moment: `large cut` at turn 10 (ply 18), played by red. red took blue's cells from 32 to 22.

Their answer: `MOVE 15 6 7;MOVE 5 4 6;MOVE 1 7 3;MOVE 2 3 6;TRAIN 1 7 7;TRAIN 1 8 7`

Our `exposed30` over our last 3 own plies before it, oldest first: 1 -> 4 -> 9.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. R.  .  .  .  .  * ## ## ##
 1  . R. R. R. R. R.  .  .  . ## ## ##
 2  .  . R. R. R. R. R.  .  . ## ## ##
 3  .  . R. R. R.  . R1  * ## ## ## ##
 4  *  . R. R. R. RT R. R1 ## ## ## ##
 5  .  *  . R1 R1 R* B. B1 B1  .  *  .
 6  .  *  .  . B. R1 B1 B. B.  .  *  .
 7 ## ## ## ## B. R2 B. B. B. B.  .  *
 8 ## ## ## ## B1 B. B. B. B. B. B.  .
 9 ## ## ##  . B1 B. B. B. B. B. B.  .
10 ## ## ##  .  .  .  .  .  . B. B. B.
11 ## ## ##  *  .  .  .  .  . B. B* BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. R.  .  .  .  .  * ## ## ##
 1  . R. R. R. R. R.  .  .  . ## ## ##
 2  .  . R. R. R. R. R.  .  . ## ## ##
 3  .  . R. R. R.  . R. R1 ## ## ## ##
 4  *  . R. R. R. RT R. R1 ## ## ## ##
 5  .  *  . R. R. R* b. b. b.  .  *  .
 6  .  *  . R1 R1 R1 b* b. b.  .  *  .
 7 ## ## ## ## B. R. R2 R1 R1 B.  .  *
 8 ## ## ## ## B1 B. B. B. B. B. B.  .
 9 ## ## ##  . B1 B. B. B. B. B. B.  .
10 ## ## ##  .  .  .  .  .  . B. B. B.
11 ## ## ##  *  .  .  .  .  . B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 3 | 29 |
| income | 20 | 27 |
| cells | 29 | 32 |
| army | 70 | 50 |
| mines | 0 | 0 |
| towers | 1 | 0 |
| exposed30 | 3 | 9 |
| hq_gap | 97 | 71 |
| material | 259 | 288 |
| voronoi | 57 | 57 |

#### Game 914372859: we were blue, lost at turn 17 (the rules ended the game)

Decisive moment: `large cut` at turn 10 (ply 18), played by red. red took blue's cells from 36 to 24.

Their answer: `MOVE 15 6 7;MOVE 5 4 8;MOVE 8 6 4;MOVE 2 3 8;TRAIN 1 3 9;TRAIN 1 3 10`

Our `exposed30` over our last 3 own plies before it, oldest first: 1 -> 2 -> 9.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R.  .  .  .  .  .  . ## ## ##
 1  . R. R.  *  .  .  .  .  .  .  . ##
 2  . R. R*  .  .  .  .  .  .  .  . ##
 3  . R. R. R.  . R. R1  .  .  .  . ##
 4 ## R. R. R. R. R.  .  .  .  .  . ##
 5 ## R. R. RT ## R. b. r.  .  .  . ##
 6 ## B1 B1 R. R. R1 R2 ##  .  .  . ##
 7 ## r. B1 R1 R1 B1 B. B1  .  .  . ##
 8 ## r. B1 B. B. B. B.  .  .  .  .  .
 9 ##  . B1 B. B. B. B. B.  .  *  .  .
10 ##  . B1 B. B. B. B. B. B* B. B. B.
11 ## ## ## B1 B. B. B. B. B. B. B* BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R.  .  .  .  .  .  . ## ## ##
 1  . R. R.  *  .  .  .  .  .  .  . ##
 2  . R. R*  .  .  .  .  .  .  .  . ##
 3  . R. R. R.  . R. R.  .  .  .  . ##
 4 ## R. R. R. R. R. R1  .  .  .  . ##
 5 ## R. R. RT ## R. b. r.  .  .  . ##
 6 ## b. b. R. R. R1 R. ##  .  .  . ##
 7 ## r. b. R. R. B1 R2 b.  .  .  . ##
 8 ## r. b. R1 R1 B. B.  .  .  .  .  .
 9 ##  . b. R1 B. B. B. B.  .  *  .  .
10 ##  . b. R1 B. B. B. B. B* B. B. B.
11 ## ## ## B1 B. B. B. B. B. B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 4 | 0 |
| income | 23 | 27 |
| cells | 32 | 36 |
| army | 70 | 90 |
| mines | 0 | 0 |
| towers | 1 | 0 |
| exposed30 | 3 | 9 |
| hq_gap | 106 | 70 |
| material | 281 | 315 |
| voronoi | 73 | 56 |

#### Game 917643422: we were red, lost at turn 18 (the rules ended the game)

Decisive moment: `large cut` at turn 13 (ply 25), played by blue. blue took red's cells from 35 to 15.

Their answer: `MOVE 7 10 1;TRAIN 1 3 7;TRAIN 1 3 6;TRAIN 3 3 5;TRAIN 1 3 4;TRAIN 1 3 3;TRAIN 1 2 3;TRAIN 1 1 3`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 2 -> 3.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B. B1 ##
 1 R. R* R1 R1 R* R. R. R. R1 B.  .  .
 2 R. R1 R. ## ## ## ## ## B1 BT B.  .
 3 ## R. R. R. R* R. R1 R. R1 B* BT  .
 4 ## R. ## R. ## ## ## ## B. B1 B. ##
 5 ## R. RT R. R1 ## ## ## BT B1 B. ##
 6 ## R1 R. R. ## ## ##  . B. B. B. ##
 7 ## b. R1 R. ## ## ## ##  . ## BT ##
 8  .  . R1 B.  .  .  .  *  .  . B. ##
 9  .  .  . BT ## ## ## ## ##  . B.  .
10  .  .  . B. B. B. B. B* B. B. B* B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH RM R. ## ## ## ## ## ## B. B. ##
 1 R. R* R1 R1 R* R. R. R. R1 B. B1  .
 2 R. R1 R. ## ## ## ## ## B1 BT B.  .
 3 ## B1 B1 B1 r* r. r. r. r. B* BT  .
 4 ## r. ## B1 ## ## ## ## B. B1 B. ##
 5 ## r. rT B3 r. ## ## ## BT B1 B. ##
 6 ## r. r. B1 ## ## ##  . B. B. B. ##
 7 ## b. r* B1 ## ## ## ##  . ## BT ##
 8  .  . r* B.  .  .  .  *  .  . B. ##
 9  .  .  . BT ## ## ## ## ##  . B.  .
10  .  .  . B. B. B. B. B* B. B. B* B.
11 ##  .  . ## ## ## ## ## ##  .  * BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 62 | 84 |
| income | 30 | 29 |
| cells | 35 | 35 |
| army | 50 | 100 |
| mines | 0 | 1 |
| towers | 5 | 1 |
| exposed30 | 4 | 3 |
| hq_gap | 58 | 56 |
| material | 392 | 421 |
| voronoi | 61 | 39 |

### aCat (rank 10, 8 losses, 8 with a decisive moment)

#### Game 914336316: we were blue, lost at turn 17 (the rules ended the game)

Decisive moment: `large cut` at turn 12 (ply 22), played by red. red took blue's cells from 32 to 18.

Their answer: `TRAIN 2 6 6;TRAIN 2 7 6;TRAIN 1 7 5;TRAIN 1 8 5;TRAIN 1 8 4;TRAIN 1 9 4;TRAIN 1 7 7;MOVE 1 2 8;MOVE 8 5 7;MOVE 5 4 7;MOVE 14 6 3;MOVE 2 3 8`

Our `exposed30` over our last 3 own plies before it, oldest first: 2 -> 2 -> 1.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## ##  .  .  . ## ## ##
 1 R. R. R. ## ##  .  .  .  .  . ## ##
 2 R.  . R. ## ##  *  .  .  . B1  . ##
 3 R. R. R. ##  .  .  . BT B. B.  .  .
 4 R. R. R* R. R. RT R1 B1 B. B*  *  .
 5 R* R* R. R. R. R. B2 B. B. B. B.  .
 6 R. R. R. R. R1 R1 B1 B1 B1 B. B*  *
 7  . R* R* R1  .  . B1 B. B. B* BT  .
 8  . R1  .  .  .  .  .  . ## B. B.  .
 9 ##  .  .  .  .  .  * ## ## B. B.  .
10 ## ##  .  .  .  .  . ## ## B. B. B.
11 ## ## ##  .  .  . ## ## ## B. B* BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## ##  .  .  . ## ## ##
 1 R. R. R. ## ##  .  .  .  .  . ## ##
 2 R.  . R. ## ##  *  .  .  . b.  . ##
 3 R. R. R. ##  .  . R1 bT b. b.  .  .
 4 R. R. R* R. R. RT R. b. R1 R1  *  .
 5 R* R* R. R. R. R. b* R1 R1 B. B.  .
 6 R. R. R. R. R. R* R2 R2 B1 B. B*  *
 7  . R* R* R. R1 R1 b. R1 B. B* BT  .
 8  . R. R1 R1  .  .  .  . ## B. B.  .
 9 ##  .  .  .  .  .  * ## ## B. B.  .
10 ## ##  .  .  .  .  . ## ## B. B. B.
11 ## ## ##  .  .  . ## ## ## B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 64 | 36 |
| income | 29 | 22 |
| cells | 34 | 32 |
| army | 50 | 80 |
| mines | 0 | 0 |
| towers | 1 | 2 |
| exposed30 | 1 | 1 |
| hq_gap | 66 | 94 |
| material | 347 | 330 |
| voronoi | 58 | 57 |

#### Game 914353947: we were blue, lost at turn 19 (the rules ended the game)

Decisive moment: `large cut` at turn 16 (ply 30), played by red. red took blue's cells from 28 to 18.

Their answer: `MOVE 23 6 7;TRAIN 2 6 8;TRAIN 1 6 9;TRAIN 1 6 10;MOVE 1 9 4;MOVE 22 3 5;MOVE 36 1 8`

Our `exposed30` over our last 3 own plies before it, oldest first: 19 -> 2 -> 2.

Board before (the opponent to play):

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. R.  .  .  .  . ##  .  * ##
 1  . R. R. R. R. R. R.  . R. R*  .  .
 2  .  . R.  .  .  * R. R. R. R.  .  .
 3  .  . R. R. R. R. R. R* R. R1  .  .
 4 ##  . R.  .  . R.  . ## R1  .  .  .
 5  *  . R1  .  * RT R* R1 B1  .  .  .
 6  .  . R. R. R. R* R2 R* B.  *  .  *
 7  .  . bT R. ## R1 B. B. BT  .  . ##
 8  .  . R1 R2 B* B1 B1 B. B.  .  .  .
 9  .  . R1 R1 BT B. B* B. B. B.  .  .
10  .  . R1 R. B. B1 B. B. B. B. B. B.
11 ##  *  . ##  .  .  .  . B. B. BM BH
```

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. R.  .  .  .  . ##  .  * ##
 1  . R. R. R. R. R. R.  . R. R*  .  .
 2  .  . R.  .  .  * R. R. R. R.  .  .
 3  .  . R. R. R. R. R. R* R. R.  .  .
 4 ##  . R.  .  . R.  . ## R1 R1  .  .
 5  *  . R* R1  * RT R* R1 B1  .  .  .
 6  .  . R. R. R. R* R. R* B.  *  .  *
 7  .  . bT R. ## R1 R2 B. BT  .  . ##
 8  . R1 R. R2 b* b. R2 B. B.  .  .  .
 9  .  . R1 R1 bT b. R1 B. B. B.  .  .
10  .  . R1 R. b. b. R1 B. B. B. B. B.
11 ##  *  . ##  .  .  .  . B. B. BM BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 15 | 29 |
| income | 29 | 28 |
| cells | 46 | 28 |
| army | 130 | 40 |
| mines | 0 | 1 |
| towers | 1 | 2 |
| exposed30 | 6 | 2 |
| hq_gap | 85 | 111 |
| material | 426 | 285 |
| voronoi | 91 | 45 |

#### Game 914372150: we were blue, lost at turn 22 (the rules ended the game)

Decisive moment: `HQ threat` at turn 21 (ply 40), played by red. red can pay a chain of trainings to blue's HQ this turn, for the first time.

Their answer: `BUILD TOWER 4 8;MOVE 1 1 2;MOVE 27 3 11`

Our `exposed30` over our last 3 own plies before it, oldest first: 1 -> 1 -> 1.

Board before (the opponent to play):

```
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

Board after:

```
    0  1  2  3  4  5  6  7  8  9 10 11
 0 RH R* R. ## ## ##  . B1 B. B.  .  .
 1 R. R. R. ## ## ## B. B. B. B.  .  .
 2 R. R1 R. ## ## ## B. B* B* B.  .  .
 3 R. R. R. R. ## ## B. B1 B. B1 B.  .
 4 R. R. R. R. R. ## B. B. B. B. B.  .
 5 R. R. R. R. RT R2 B. BT BT B. B.  .
 6 R. R. R. RT R. B2 B3 B. B. B. BT  .
 7 R. R. R. R. R. R2 ## B1 B1 B. B.  .
 8  . R. R. R. RT b. ## ## B. B1 B.  .
 9  .  . R. R* R* R. ## ## ## B1 B.  .
10  .  . R. R.  . R. ## ## ## B. B. BT
11  .  .  . R2 R. R. ## ## ## B. B* BH
```

Metrics right after each side's last own ply before the moment:

| metric | theirs | ours |
|---|---:|---:|
| gold | 191 | 60 |
| income | 34 | 16 |
| cells | 47 | 47 |
| army | 70 | 120 |
| mines | 0 | 0 |
| towers | 2 | 4 |
| exposed30 | 1 | 1 |
| hq_gap | 19 | 90 |
| material | 571 | 456 |
| voronoi | 59 | 61 |

