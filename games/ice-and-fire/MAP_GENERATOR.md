# A Code of Ice and Fire: how the source draws a map

The steps by which the game's source draws a map, written in our own words for our referee's generator ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)). They state behaviour, not code: the source is GPL, read for facts only ([ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)), and our generator is written from this page by someone who has not read the source. Maps printed by the source's generator, run unchanged, check it map by map (`referee/src/map/golden.txt`, committed with the generator). The source is [Azkellas/a-code-of-ice-and-fire](https://github.com/Azkellas/a-code-of-ice-and-fire), commit `efe9e78`, `GameState.java` (`generateMap` and the methods it calls) and `Constants.java`.

The map's facts, as bots see them, are in [RULES.md](RULES.md) ("The map"); this page adds only what reproducing the maps exactly needs.

## Terms

- The grid is 12 columns by 12 rows; a cell is `(x, y)`, x the column and y the row, both 0 to 11.
- While the map is drawn, each cell is either **land** (a cell of the map) or **void**, and may be a **mine spot**.
- The **mirror** of `(x, y)` is `(11 - x, 11 - y)`.
- "Scan the grid" means x from 0 to 11, and for each x, y from 0 to 11: **x is the outer loop**.

## The random numbers

One generator is created from the seed at the start and used by every step, in the order below. It is the linear congruential generator that Java's standard library documents for `java.util.Random`, with 48 bits of state:

- **Seed:** the seed is a 64-bit signed integer; our arena's 64-bit unsigned seed is taken bit for bit. The state starts as `(seed XOR 0x5DEECE66D) AND (2^48 - 1)`.
- **Next `b` bits** (b from 1 to 32): the state becomes `(state × 0x5DEECE66D + 0xB) mod 2^48`, computed with wrapping 64-bit arithmetic; the result is the state's top `b` bits (`state >> (48 - b)`) read as a 32-bit signed integer.
- **Integer below `n`** (n > 0): take 31 bits as `r`. If n is a power of two (1 included), the result is `(n × r) >> 31`, computed in 64 bits. Otherwise, repeat: the result candidate is `r mod n`; accept it if `r - candidate + (n - 1)`, computed as a wrapping 32-bit signed integer, is not negative; else take 31 new bits as `r` and try again. A draw below 1 still takes its 31 bits and gives 0: it is never skipped.
- **Fraction:** take 24 bits and divide by 2^24 in single precision (f32). The result is in [0, 1).

## Steps

In this order. Only steps 3 and 9 depend on the league, and only through the mine spots.

1. **Noise.** Scan the grid; for each cell draw a fraction, and the cell is land if the fraction is **strictly greater than 0.45** (single precision), else void.
2. **Home corner.** Make (0, 0), (1, 0), (0, 1) and (1, 1) land.
3. **First mine spots.** In Wood 1 and Bronze only, make (1, 0) and its mirror (10, 11) mine spots. Mine spots never change back.
4. **Smoothing, 3 passes.** In each pass, every cell's new state is computed from the state before the pass (all cells change at once): count the land among the 9 cells of the 3 × 3 block centred on the cell, the cell included, where a position off the grid counts as land; the cell becomes land if the count is **at least 6**, else void.
5. **Inversion.** Every land cell becomes void and every void cell becomes land.
6. **Mirroring, in place.** For x from 0 to 11, and for each x, y from 0 to x included: the cell at the mirror of `(x, y)` takes the state (land or void) of `(x, y)`. The cells are copied one after the other in this order, each copy seeing the copies before it; some cells are both read and written. Mine spots are not copied.
7. **Corner blocks.** For i and j from 0 to 2, make `(i, j)` and its mirror land.
8. **Joining the areas.** Find the areas, groups of land cells joined through their sides (no diagonals):
   - Scan the grid; each land cell not yet in an area starts a new area, filled by a depth-first walk.
   - The walk, on a cell: add the cell to the end of the area's list, mark it as seen, then try its neighbours in the order `(x - 1, y)`, `(x + 1, y)`, `(x, y - 1)`, `(x, y + 1)`. A neighbour is walked, at once and in full before the next neighbour is tried, if it is on the grid, land and not yet seen; whether it is seen is checked just before walking it. The order of the cells in each list matters.
   - If there is a single area, the step ends here and draws no random number.
   - Otherwise, for each area in the order found, draw an integer below the area's size and pick the cell at that position of its list.
   - Then for every pair of picked cells, `i` before `j` in that order (every pair, not only enough to join them), with `a` the i-th picked cell and `b` the j-th: for every x from the smaller to the larger of `a.x` and `b.x`, make `(x, b.y)` and its mirror land; then for every y from the smaller to the larger of `a.y` and `b.y`, make `(a.x, y)` and its mirror land.
   - The areas are not searched again afterwards.
9. **Mine spots.** In Wood 1 and Bronze only; in Wood 3 and Wood 2 the map has no mine spot and this step draws no number.
   - Count the land cells, `n`.
   - Let `max` be the larger of 6 and `n × 0.15` computed in single precision (f32) and rounded toward zero.
   - Draw an integer below `max - 5`, add 6, halve rounding down, and double: that is the number of mine spots, `count`.
   - `count / 2 - 1` times (integer division; it is at least 2): draw a column x, then a row y, each an integer below 12. While `(x, y)` is void, already a mine spot, or one of (0, 0), (11, 11), (0, 1) and (11, 10), draw a new x, then a new y. Then make `(x, y)` and its mirror mine spots.
   - No cell is its own mirror on a 12 × 12 grid, so the map holds exactly `count` mine spots.
10. The headquarters are placed afterwards and do not change the map: red's at (0, 0), blue's at (11, 11), with blue also owning (11, 10) (RULES.md, "Start of the game").

## What follows

- The map is half-turn symmetric: steps 6 to 9 keep it so, since every later change is made to a cell and its mirror.
- The corner blocks around the HQs are land (step 7), and the land is one area once step 8 has joined it.
- The league changes only the mine spots: Wood 3 and Wood 2 draw the same map as each other, and Wood 1 and Bronze the same map and mine spots as each other; all four have the same land and void for a given seed.
