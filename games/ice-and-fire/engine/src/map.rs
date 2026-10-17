//! What is fixed for a whole game: the void cells, the mine spots, the
//! neighbours of every cell and the distances between cells.
//!
//! The distance table costs about 123 microseconds to build and 20 KB to
//! keep, so a state shares it through an `Arc` instead of copying it: a search
//! clones states by the thousand. `Arc` and not `Rc` so that states can move
//! between threads. A bot that reads a state every turn would rebuild the
//! table every turn, so [`MapData::shared`] remembers the last map it built on
//! each thread and hands it out again.

use std::cell::RefCell;
use std::sync::Arc;

use crate::bits::{Bits, CELLS, SIZE};

/// No cell: padding in a neighbour list, and the distance between cells with
/// no path between them. Real distances are at most 143.
pub const NONE: u8 = u8::MAX;

/// The map of a game.
pub struct MapData {
    /// The void cells.
    pub void: Bits,
    /// The cells that are not void.
    pub valid: Bits,
    /// The mine spots.
    pub spots: Bits,
    /// `distance[from * 144 + to]`: the length of the shortest path over
    /// non-void cells, ignoring units, buildings and owners; [`NONE`] if there
    /// is none, and for every void cell.
    distance: Box<[u8]>,
    /// The non-void neighbours of each cell in the order a move prefers them,
    /// padded with [`NONE`]. Index 0 is red's order (up, right, down, left),
    /// 1 is blue's (down, left, up, right).
    pub neighbours: [[[u8; 4]; CELLS]; 2],
}

thread_local! {
    static LAST: RefCell<Option<Arc<MapData>>> = const { RefCell::new(None) };
}

impl MapData {
    /// The map with these void cells and mine spots, built or remembered.
    pub fn shared(void: Bits, spots: Bits) -> Arc<MapData> {
        LAST.with(|last| {
            let mut last = last.borrow_mut();
            if let Some(map) = last.as_ref() {
                if map.void == void && map.spots == spots {
                    return Arc::clone(map);
                }
            }
            let map = Arc::new(MapData::build(void, spots));
            *last = Some(Arc::clone(&map));
            map
        })
    }

    fn build(void: Bits, spots: Bits) -> MapData {
        let valid = !void;
        // Red's order moves by (dx, dy): up, right, down, left.
        const STEPS: [[(i32, i32); 4]; 2] = [
            [(0, -1), (1, 0), (0, 1), (-1, 0)],
            [(0, 1), (-1, 0), (0, -1), (1, 0)],
        ];
        let mut neighbours = [[[NONE; 4]; CELLS]; 2];
        for (seat, steps) in STEPS.iter().enumerate() {
            for cell in valid.iter() {
                let (x, y) = ((cell % SIZE) as i32, (cell / SIZE) as i32);
                let mut count = 0;
                for &(dx, dy) in steps {
                    let (nx, ny) = (x + dx, y + dy);
                    if (0..SIZE as i32).contains(&nx) && (0..SIZE as i32).contains(&ny) {
                        let next = ny as usize * SIZE + nx as usize;
                        if valid.test(next) {
                            neighbours[seat][cell][count] = next as u8;
                            count += 1;
                        }
                    }
                }
            }
        }
        let mut distance = vec![NONE; CELLS * CELLS].into_boxed_slice();
        let mut queue = [0u8; CELLS];
        for from in valid.iter() {
            let row = &mut distance[from * CELLS..(from + 1) * CELLS];
            row[from] = 0;
            queue[0] = from as u8;
            let (mut head, mut tail) = (0, 1);
            while head < tail {
                let cell = queue[head] as usize;
                head += 1;
                for &next in &neighbours[0][cell] {
                    if next != NONE && row[next as usize] == NONE {
                        row[next as usize] = row[cell] + 1;
                        queue[tail] = next;
                        tail += 1;
                    }
                }
            }
        }
        MapData {
            void,
            valid,
            spots,
            distance,
            neighbours,
        }
    }

    /// The distance between two cells, [`NONE`] when there is no path.
    #[inline]
    pub fn distance(&self, from: usize, to: usize) -> u8 {
        self.distance[from * CELLS + to]
    }
}
