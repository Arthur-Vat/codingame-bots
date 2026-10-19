//! The map of A Code of Ice and Fire. The generator reproduces the source's maps
//! from the spec `games/ice-and-fire/MAP_GENERATOR.md` (ADR 0041): for a seed and
//! a league it returns the map the source draws, checked against maps the source
//! printed (`map/golden.txt`). It is deterministic per seed.
//!
//! The half-turn mirror of `(x, y)` is `(11 - x, 11 - y)`. The step numbers in
//! the comments are those of the spec.

use crate::{League, SIZE};

mod java_random;

use java_random::JavaRandom;

/// Number of cells.
pub const CELLS: usize = SIZE * SIZE;

/// Smoothing passes (step 4).
const SMOOTHING_PASSES: usize = 3;

/// The map: which cells are void, and where the mine spots are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    /// Void cells, indexed by `y * 12 + x`.
    pub void: [bool; CELLS],
    /// Mine spots as `(x, y)`, sorted by y, then x.
    pub mine_spots: Vec<(usize, usize)>,
}

impl Map {
    /// Whether `(x, y)` is void. Off-map cells count as void.
    pub fn is_void(&self, x: usize, y: usize) -> bool {
        x >= SIZE || y >= SIZE || self.void[y * SIZE + x]
    }

    /// Whether `(x, y)` is a mine spot.
    pub fn is_mine_spot(&self, x: usize, y: usize) -> bool {
        self.mine_spots.contains(&(x, y))
    }
}

/// The index of `(x, y)`.
fn at(x: usize, y: usize) -> usize {
    y * SIZE + x
}

/// The index of the cell opposite `(x, y)` under the half-turn.
fn mirror(x: usize, y: usize) -> usize {
    at(SIZE - 1 - x, SIZE - 1 - y)
}

/// Makes the map of a game: deterministic for a seed and a league.
pub fn generate(seed: u64, league: League) -> Map {
    // The seed is taken bit for bit as the source's signed 64-bit seed.
    let mut rng = JavaRandom::new(seed as i64);
    let mut land = [false; CELLS];
    let mut spot = [false; CELLS];

    // Step 1: noise, x being the outer loop.
    for x in 0..SIZE {
        for y in 0..SIZE {
            land[at(x, y)] = rng.fraction() > 0.45f32;
        }
    }
    // Step 2: the home corner.
    for (x, y) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        land[at(x, y)] = true;
    }
    // Step 3: the first mine spots.
    if league.has_mine_spots() {
        spot[at(1, 0)] = true;
        spot[mirror(1, 0)] = true;
    }
    // Step 4: smoothing.
    for _ in 0..SMOOTHING_PASSES {
        land = smooth(&land);
    }
    // Step 5: inversion.
    for cell in land.iter_mut() {
        *cell = !*cell;
    }
    // Step 6: mirroring, in place and in this order.
    for x in 0..SIZE {
        for y in 0..=x {
            land[mirror(x, y)] = land[at(x, y)];
        }
    }
    // Step 7: corner blocks.
    for i in 0..3 {
        for j in 0..3 {
            land[at(i, j)] = true;
            land[mirror(i, j)] = true;
        }
    }
    // Step 8: joining the areas.
    join_areas(&mut land, &mut rng);
    // Step 9: mine spots.
    if league.has_mine_spots() {
        place_mine_spots(&land, &mut spot, &mut rng);
    }

    let mut void = [false; CELLS];
    for (cell, &is_land) in void.iter_mut().zip(&land) {
        *cell = !is_land;
    }
    let mut mine_spots = Vec::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            if spot[at(x, y)] {
                mine_spots.push((x, y));
            }
        }
    }
    Map { void, mine_spots }
}

/// Step 4, one pass: a cell becomes land if at least 6 of the 9 cells of the
/// 3 × 3 block around it, itself included, are land; off-map cells count as
/// land. Every cell is computed from the grid before the pass.
fn smooth(land: &[bool; CELLS]) -> [bool; CELLS] {
    let mut next = [false; CELLS];
    let size = SIZE as i32;
    for x in 0..size {
        for y in 0..size {
            let mut count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    let off_map = nx < 0 || ny < 0 || nx >= size || ny >= size;
                    if off_map || land[at(nx as usize, ny as usize)] {
                        count += 1;
                    }
                }
            }
            next[at(x as usize, y as usize)] = count >= 6;
        }
    }
    next
}

/// The depth-first walk of step 8 on `(x, y)`: the cell is added to `area`,
/// then the neighbours are tried left, right, up, down.
fn walk(
    land: &[bool; CELLS],
    seen: &mut [bool; CELLS],
    (x, y): (usize, usize),
    area: &mut Vec<usize>,
) {
    area.push(at(x, y));
    seen[at(x, y)] = true;
    // Wrapped subtractions land far off the grid and fail the bounds check.
    let neighbours = [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ];
    for (nx, ny) in neighbours {
        if nx < SIZE && ny < SIZE && land[at(nx, ny)] && !seen[at(nx, ny)] {
            walk(land, seen, (nx, ny), area);
        }
    }
}

/// Step 8: joins the areas of land with corridors.
fn join_areas(land: &mut [bool; CELLS], rng: &mut JavaRandom) {
    let mut seen = [false; CELLS];
    let mut areas: Vec<Vec<usize>> = Vec::new();
    for x in 0..SIZE {
        for y in 0..SIZE {
            if land[at(x, y)] && !seen[at(x, y)] {
                let mut area = Vec::new();
                walk(land, &mut seen, (x, y), &mut area);
                areas.push(area);
            }
        }
    }
    if areas.len() <= 1 {
        return;
    }
    let picked: Vec<(usize, usize)> = areas
        .iter()
        .map(|area| {
            let index = area[rng.below(area.len() as i32) as usize];
            (index % SIZE, index / SIZE)
        })
        .collect();
    for (i, &(ax, ay)) in picked.iter().enumerate() {
        for &(bx, by) in &picked[i + 1..] {
            for x in ax.min(bx)..=ax.max(bx) {
                land[at(x, by)] = true;
                land[mirror(x, by)] = true;
            }
            for y in ay.min(by)..=ay.max(by) {
                land[at(ax, y)] = true;
                land[mirror(ax, y)] = true;
            }
        }
    }
}

/// Step 9: an even number of mine spots, symmetric, added to `spot`.
fn place_mine_spots(land: &[bool; CELLS], spot: &mut [bool; CELLS], rng: &mut JavaRandom) {
    let lands = land.iter().filter(|&&cell| cell).count();
    let most = 6.max((lands as f32 * 0.15f32) as i32);
    let count = (rng.below(most - 5) + 6) / 2 * 2;
    let size = SIZE as i32;
    let forbidden = [(0, 0), (SIZE - 1, SIZE - 1), (0, 1), (SIZE - 1, SIZE - 2)];
    for _ in 0..count / 2 - 1 {
        let mut x = rng.below(size) as usize;
        let mut y = rng.below(size) as usize;
        while !land[at(x, y)] || spot[at(x, y)] || forbidden.contains(&(x, y)) {
            x = rng.below(size) as usize;
            y = rng.below(size) as usize;
        }
        spot[at(x, y)] = true;
        spot[mirror(x, y)] = true;
    }
}

#[cfg(test)]
mod tests;
