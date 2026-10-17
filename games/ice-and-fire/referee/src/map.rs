//! The map of A Code of Ice and Fire and our own generator for it (ADR 0039).
//!
//! The generator is designed from the map facts of `games/ice-and-fire/RULES.md`
//! ("The map") and imitates no other generator. It is deterministic per seed.
//! The half-turn mirror of `(x, y)` is `(11 - x, 11 - y)`.

use cg_core::rng::Rng;

use crate::{League, SIZE};

/// Number of cells.
pub const CELLS: usize = SIZE * SIZE;

/// Chance that a cell of the first half is void before smoothing. To be tuned
/// against real maps (ADR 0039, decision 3).
pub const VOID_SHARE: f64 = 0.42;

/// Number of smoothing passes over the grid.
pub const SMOOTHING_PASSES: usize = 2;

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

/// The index of the cell opposite `index` under the half-turn.
fn mirror(index: usize) -> usize {
    CELLS - 1 - index
}

/// Whether `index` is in one of the two clear corner blocks.
fn in_corner_block(index: usize) -> bool {
    let (x, y) = (index % SIZE, index / SIZE);
    (x <= 2 && y <= 2) || (x >= SIZE - 3 && y >= SIZE - 3)
}

/// Makes the map of a game: deterministic for a seed and a league.
pub fn generate(seed: u64, league: League) -> Map {
    let mut rng = Rng::new(seed);
    let mut void = random_voids(&mut rng);
    for _ in 0..SMOOTHING_PASSES {
        void = smooth(&void);
    }
    for (index, cell) in void.iter_mut().enumerate() {
        if in_corner_block(index) {
            *cell = false;
        }
    }
    connect(&mut void);
    let mine_spots = if league.has_mine_spots() {
        place_mine_spots(&void, &mut rng)
    } else {
        Vec::new()
    };
    Map { void, mine_spots }
}

/// Step 1: voids drawn for the first half and mirrored.
fn random_voids(rng: &mut Rng) -> [bool; CELLS] {
    let mut void = [false; CELLS];
    for index in 0..CELLS / 2 {
        let is_void = rng.unit() < VOID_SHARE;
        void[index] = is_void;
        void[mirror(index)] = is_void;
    }
    void
}

/// Step 2: one cellular-automaton pass, computed from a copy of the grid. A
/// cell with at least 5 non-void neighbours among its 8 becomes non-void, one
/// with at most 2 becomes void. Off-map neighbours count as void.
fn smooth(void: &[bool; CELLS]) -> [bool; CELLS] {
    let mut next = *void;
    for y in 0..SIZE as i32 {
        for x in 0..SIZE as i32 {
            let mut open = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    let on_map = (0..SIZE as i32).contains(&nx) && (0..SIZE as i32).contains(&ny);
                    if (dx, dy) != (0, 0) && on_map && !void[ny as usize * SIZE + nx as usize] {
                        open += 1;
                    }
                }
            }
            let index = y as usize * SIZE + x as usize;
            if open >= 5 {
                next[index] = false;
            } else if open <= 2 {
                next[index] = true;
            }
        }
    }
    next
}

/// The indices of the non-void cells 4-connected to `start`.
fn component(void: &[bool; CELLS], start: usize) -> [bool; CELLS] {
    let mut seen = [false; CELLS];
    seen[start] = true;
    let mut stack = vec![start];
    while let Some(index) = stack.pop() {
        let (x, y) = (index % SIZE, index / SIZE);
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push(index - 1);
        }
        if x + 1 < SIZE {
            neighbours.push(index + 1);
        }
        if y > 0 {
            neighbours.push(index - SIZE);
        }
        if y + 1 < SIZE {
            neighbours.push(index + SIZE);
        }
        for next in neighbours {
            if !void[next] && !seen[next] {
                seen[next] = true;
                stack.push(next);
            }
        }
    }
    seen
}

/// The pair `(a, b)` with `a` in `area` and `b` a non-void cell outside it
/// that has the smallest Manhattan distance. Ties go to the smaller index of
/// `a`, then of `b`. `None` when no non-void cell is outside `area`.
fn closest_pair(void: &[bool; CELLS], area: &[bool; CELLS]) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize, usize)> = None;
    for a in (0..CELLS).filter(|&a| area[a]) {
        for b in (0..CELLS).filter(|&b| !void[b] && !area[b]) {
            let distance = (a % SIZE).abs_diff(b % SIZE) + (a / SIZE).abs_diff(b / SIZE);
            // Ascending scans with a strict comparison break ties as stated.
            if best.is_none_or(|(smallest, _, _)| distance < smallest) {
                best = Some((distance, a, b));
            }
        }
    }
    best.map(|(_, a, b)| (a, b))
}

/// Step 4: joins the areas to the one holding (0, 0) with L-shaped corridors,
/// closest pair first, until there is one area.
fn connect(void: &mut [bool; CELLS]) {
    while let Some((a, b)) = closest_pair(void, &component(void, 0)) {
        let (ax, ay, bx, by) = (a % SIZE, a / SIZE, b % SIZE, b / SIZE);
        // Along a's row to b's column, then along that column to b.
        let mut carved = Vec::new();
        carved.extend((ax.min(bx)..=ax.max(bx)).map(|x| ay * SIZE + x));
        carved.extend((ay.min(by)..=ay.max(by)).map(|y| y * SIZE + bx));
        for index in carved {
            void[index] = false;
            void[mirror(index)] = false;
        }
    }
}

/// Step 5: an even number of mine spots, symmetric, with (1, 0) and (10, 11).
fn place_mine_spots(void: &[bool; CELLS], rng: &mut Rng) -> Vec<(usize, usize)> {
    let open = void.iter().filter(|&&cell| !cell).count();
    let most = (open * 15 / 100).max(6);
    let pairs = 3 + rng.below((most / 2 - 2) as u64) as usize;
    let count = 2 * pairs;

    let forbidden = [0, SIZE, CELLS - 1, CELLS - 1 - SIZE];
    let mut spot = [false; CELLS];
    spot[1] = true;
    spot[mirror(1)] = true;
    let mut placed = 2;
    while placed < count {
        let index = rng.below(CELLS as u64) as usize;
        if void[index] || forbidden.contains(&index) || spot[index] {
            continue;
        }
        spot[index] = true;
        spot[mirror(index)] = true;
        placed += 2;
    }
    (0..CELLS)
        .filter(|&index| spot[index])
        .map(|index| (index % SIZE, index / SIZE))
        .collect()
}

#[cfg(test)]
mod tests;
