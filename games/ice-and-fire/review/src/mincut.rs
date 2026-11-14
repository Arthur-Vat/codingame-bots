//! The cheapest cuts of a side's territory, for the study of the threats
//! its search misses (`codingame-surprise`, the note's section 3c).
//!
//! A cut is a set of the side's active cells whose capture disconnects
//! others from its HQ. Each cell has a capture price: what the cheapest
//! unit able to enter it costs ([`price`]). For each of the side's active
//! cells `t`, a max-flow over the territory (each cell split in two, its
//! two halves joined by an edge of its price; neighbours joined both ways
//! at no limit) gives the cheapest set of cells whose capture separates `t`
//! from the HQ, however many paths join them. The cut removes its own cells
//! and every cell it leaves disconnected.
//!
//! With [`price`] alone ([`cut_to`], [`best_affordable`]) this ignores that
//! the enemy must reach the cut cells: a training enters only a cell next
//! to its own territory, so a cut deep inside ours costs more than its
//! cells' prices. [`reach_prices`] adds the way there, and makes the cells
//! an enemy unit can step onto free; a free cell joins a cut only when the
//! cut needs it.

use std::collections::VecDeque;

use ice_and_fire_engine::{Bits, State, CELLS, HQ};

/// More than any cut costs.
const INFINITE: i64 = 1 << 40;

/// The cheapest unit that may enter `cell`, one of `seat`'s active cells,
/// for its enemy: 30 when a tower of `seat` holds or protects it, else 10
/// for an empty cell, 20 for a level-1 unit, 30 for a level-2 or 3 unit.
pub fn price(state: &State, seat: usize, cell: usize) -> i64 {
    let active = state.active(seat);
    let towers = state.towers() & active;
    if (towers | (towers.neighbours() & active)).test(cell) {
        return 30;
    }
    match state.unit_level(cell) {
        Some(1) => 20,
        Some(_) => 30,
        None => 10,
    }
}

/// What capturing each of `seat`'s active cells costs its enemy once the
/// way there is paid (`reach`): 0 when an enemy unit next to it can step in
/// (it is no stronger than the unit and no tower protects it), else its
/// [`price`] plus the cheapest chain of trainings from the enemy's active
/// land to one of its neighbours (a neutral cell 10, a cell of `seat`'s its
/// price, the enemy's own 0). Cells it cannot reach cost more than any cut.
pub fn reach_prices(state: &State, seat: usize) -> [i64; CELLS] {
    let enemy = 1 - seat;
    let valid = state.map().valid;
    let active = state.active(seat);
    let mut step = [INFINITE; CELLS];
    for cell in valid.iter() {
        step[cell] = if state.active(enemy).test(cell) {
            0
        } else if active.test(cell) {
            price(state, seat, cell)
        } else {
            // A neutral cell, one of `seat`'s cut off, or one of the enemy's
            // own inactive cells (retaken by a training; one that holds
            // the enemy's unit or building cannot be, which this ignores):
            // a level-1 unit.
            10
        };
    }
    // Dijkstra (the costs are multiples of 10, the map small: a plain scan).
    let mut dist = [INFINITE; CELLS];
    let mut done = [false; CELLS];
    for cell in state.active(enemy).iter() {
        dist[cell] = 0;
    }
    loop {
        let mut best: Option<usize> = None;
        for cell in valid.iter() {
            if !done[cell] && dist[cell] < INFINITE && best.is_none_or(|b| dist[cell] < dist[b]) {
                best = Some(cell);
            }
        }
        let Some(v) = best else { break };
        done[v] = true;
        for w in (Bits::cell(v).neighbours() & valid).iter() {
            if w == HQ[seat] {
                continue;
            }
            let d = dist[v] + step[w];
            if d < dist[w] {
                dist[w] = d;
            }
        }
    }
    let movers = state.movable_units() & state.units(enemy);
    let towers = state.towers() & active;
    let protected = towers | (towers.neighbours() & active);
    let mut out = [INFINITE; CELLS];
    for cell in active.iter() {
        let own = price(state, seat, cell);
        let near = (Bits::cell(cell).neighbours() & valid)
            .iter()
            .map(|n| dist[n])
            .min()
            .unwrap_or(INFINITE);
        // `dist[n]` already counts `n`'s own price when `n` is ours. A
        // level-3 unit may also enter a protected cell; it is left out here,
        // so the step-in is counted only where any level may make it.
        let walk_in = !protected.test(cell)
            && (Bits::cell(cell).neighbours() & movers).iter().any(|u| {
                let level = state.unit_level(u).unwrap_or(0);
                match state.unit_level(cell) {
                    None => true,
                    Some(theirs) => level == 3 || (level == 2 && theirs == 1),
                }
            });
        out[cell] = if walk_in {
            0
        } else {
            (own + near).min(INFINITE)
        };
    }
    out
}

/// A cut of `seat`'s territory: its price and the active cells it removes
/// (its own cells and those it disconnects from the HQ).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cut {
    pub cost: i64,
    pub cells: i64,
    /// The cells captured (the cut's size).
    pub links: i64,
}

/// The cells of `cells` joined to `hq` through `cells`.
fn joined(cells: Bits, hq: usize) -> Bits {
    let mut reached = Bits::cell(hq) & cells;
    loop {
        let next = (reached | reached.neighbours()) & cells;
        if next == reached {
            return reached;
        }
        reached = next;
    }
}

/// A small max-flow graph (Edmonds-Karp on adjacency lists).
struct Flow {
    head: Vec<Vec<usize>>,
    to: Vec<usize>,
    cap: Vec<i64>,
}

impl Flow {
    fn new(nodes: usize) -> Flow {
        Flow {
            head: vec![Vec::new(); nodes],
            to: Vec::new(),
            cap: Vec::new(),
        }
    }

    fn edge(&mut self, a: usize, b: usize, cap: i64) {
        self.head[a].push(self.to.len());
        self.to.push(b);
        self.cap.push(cap);
        self.head[b].push(self.to.len());
        self.to.push(a);
        self.cap.push(0);
    }

    /// The max flow from `s` to `t`, at most `limit` (a flow of `limit` or
    /// more stops the search).
    fn run(&mut self, s: usize, t: usize, limit: i64) -> i64 {
        let mut total = 0;
        while total < limit {
            let mut prev = vec![usize::MAX; self.head.len()];
            let mut queue = VecDeque::from([s]);
            prev[s] = s;
            while let Some(v) = queue.pop_front() {
                for &e in &self.head[v] {
                    let w = self.to[e];
                    if self.cap[e] > 0 && prev[w] == usize::MAX {
                        prev[w] = e;
                        queue.push_back(w);
                    }
                }
            }
            if prev[t] == usize::MAX {
                break;
            }
            let mut push = INFINITE;
            let mut v = t;
            while v != s {
                let e = prev[v];
                push = push.min(self.cap[e]);
                v = self.to[e ^ 1];
            }
            let mut v = t;
            while v != s {
                let e = prev[v];
                self.cap[e] -= push;
                self.cap[e ^ 1] += push;
                v = self.to[e ^ 1];
            }
            total += push;
        }
        total
    }

    /// The nodes reachable from `s` in the residual graph.
    fn reachable(&self, s: usize) -> Vec<bool> {
        let mut seen = vec![false; self.head.len()];
        seen[s] = true;
        let mut queue = VecDeque::from([s]);
        while let Some(v) = queue.pop_front() {
            for &e in &self.head[v] {
                let w = self.to[e];
                if self.cap[e] > 0 && !seen[w] {
                    seen[w] = true;
                    queue.push_back(w);
                }
            }
        }
        seen
    }

    /// The nodes that reach `t` in the residual graph.
    fn reaching(&self, t: usize) -> Vec<bool> {
        let mut seen = vec![false; self.head.len()];
        seen[t] = true;
        let mut queue = VecDeque::from([t]);
        while let Some(w) = queue.pop_front() {
            for &e in &self.head[w] {
                // The edge `e ^ 1` goes from `to[e]` to `w`.
                let u = self.to[e];
                if self.cap[e ^ 1] > 0 && !seen[u] {
                    seen[u] = true;
                    queue.push_back(u);
                }
            }
        }
        seen
    }
}

/// The cheapest cut that separates `target` from `seat`'s HQ (of several,
/// the one nearest the HQ), `None` when it costs `budget` or more, or when
/// `target` is the HQ.
#[cfg(test)]
pub fn cut_to(state: &State, seat: usize, target: usize, budget: i64) -> Option<Cut> {
    let mut prices = [INFINITE; CELLS];
    for cell in state.active(seat).iter() {
        prices[cell] = price(state, seat, cell);
    }
    cut_with(state, seat, target, budget, &prices)
}

/// [`cut_to`] with each cell's capture priced by `prices`.
pub fn cut_with(
    state: &State,
    seat: usize,
    target: usize,
    budget: i64,
    prices: &[i64; CELLS],
) -> Option<Cut> {
    let active = state.active(seat);
    let hq = HQ[seat];
    if target == hq || !active.test(target) || !active.test(hq) {
        return None;
    }
    // Node 2c is the cell's entry, 2c + 1 its exit.
    let mut flow = Flow::new(2 * CELLS);
    for cell in active.iter() {
        let through = if cell == hq || cell == target {
            INFINITE
        } else {
            prices[cell]
        };
        flow.edge(2 * cell, 2 * cell + 1, through);
        for next in (Bits::cell(cell).neighbours() & active).iter() {
            flow.edge(2 * cell + 1, 2 * next, INFINITE);
        }
    }
    let cost = flow.run(2 * target + 1, 2 * hq, budget);
    if cost >= budget {
        return None;
    }
    // Of the cheapest cuts, the one nearest the HQ, which removes the most:
    // a cell is cut when its exit still reaches the HQ in the residual
    // graph and its entry does not.
    let side = flow.reaching(2 * hq);
    let mut cut = Bits::EMPTY;
    for cell in active.iter() {
        if side[2 * cell + 1] && !side[2 * cell] {
            cut.set(cell);
        }
    }
    // A free cell (price 0) meets that test whether the cut needs it or
    // not: keep it only when the target stays joined to the HQ without it.
    for cell in cut.iter() {
        if prices[cell] == 0 && cell != target {
            let mut fewer = cut;
            fewer.clear(cell);
            if !joined(active.and_not(fewer), hq).test(target) {
                cut = fewer;
            }
        }
    }
    let joined = joined(active.and_not(cut), hq);
    Some(Cut {
        cost,
        cells: i64::from(active.and_not(joined).count()),
        links: i64::from(cut.count()),
    })
}

/// The cut removing the most of `seat`'s active cells that costs at most
/// `budget` (the cheapest of those on a tie), over the cheapest cut to each
/// active cell; `None` when none is affordable.
pub fn best_affordable(state: &State, seat: usize, budget: i64) -> Option<Cut> {
    let mut prices = [INFINITE; CELLS];
    for cell in state.active(seat).iter() {
        prices[cell] = price(state, seat, cell);
    }
    best_with(state, seat, budget, &prices)
}

/// [`best_affordable`] with each cell's capture priced by `prices`.
pub fn best_with(state: &State, seat: usize, budget: i64, prices: &[i64; CELLS]) -> Option<Cut> {
    let mut best: Option<Cut> = None;
    for target in state.active(seat).iter() {
        let Some(cut) = cut_with(state, seat, target, budget + 1, prices) else {
            continue;
        };
        let better = best.is_none_or(|b| (cut.cells, -cut.cost) > (b.cells, -b.cost));
        if better {
            best = Some(cut);
        }
    }
    best
}

/// A cut on a territory: its cost, the cells captured, the cells removed
/// (those captured and those left without a path to the HQ).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Taken {
    pub cost: i64,
    pub captured: Bits,
    pub removed: Bits,
}

impl Taken {
    fn of(active: Bits, hq: usize, captured: Bits, prices: &[i64; CELLS]) -> Taken {
        let removed = active.and_not(joined(active.and_not(captured), hq));
        let cost = captured.iter().map(|c| prices[c]).sum();
        Taken {
            cost,
            captured,
            removed,
        }
    }

    /// Drops from the cut the free cells (price 0) that remove nothing but
    /// themselves: an enemy unit can step onto them, but that is a capture,
    /// not a cut, and one unit cannot make them all.
    fn pruned(self, active: Bits, hq: usize, prices: &[i64; CELLS]) -> Taken {
        let mut out = self;
        for cell in self.captured.iter() {
            if prices[cell] != 0 {
                continue;
            }
            let mut fewer = out.captured;
            fewer.clear(cell);
            let other = Taken::of(active, hq, fewer, prices);
            if other.removed.count() + 1 >= out.removed.count() {
                out = other;
            }
        }
        out
    }
}

/// The cut on `active` (joined to `hq`) that maximizes `lambda` for each
/// cell removed less the capture prices: one min-cut, where a source pays
/// `lambda` to every cell and the cut either captures a cell (its price) or
/// gives up its `lambda` by leaving it joined to the HQ. A larger `lambda`
/// buys more land at a higher price; several regions may go at once.
pub fn lambda_cut(active: Bits, hq: usize, lambda: i64, prices: &[i64; CELLS]) -> Taken {
    if !active.test(hq) {
        return Taken::of(active, hq, Bits::EMPTY, prices);
    }
    let source = 2 * CELLS;
    let mut flow = Flow::new(2 * CELLS + 1);
    for cell in active.iter() {
        let through = if cell == hq { INFINITE } else { prices[cell] };
        flow.edge(2 * cell, 2 * cell + 1, through);
        if cell != hq {
            flow.edge(source, 2 * cell, lambda);
        }
        for next in (Bits::cell(cell).neighbours() & active).iter() {
            flow.edge(2 * cell + 1, 2 * next, INFINITE);
        }
    }
    flow.run(source, 2 * hq, INFINITE);
    let side = flow.reachable(source);
    let mut captured = Bits::EMPTY;
    for cell in active.iter() {
        if side[2 * cell] && !side[2 * cell + 1] {
            captured.set(cell);
        }
    }
    Taken::of(active, hq, captured, prices).pruned(active, hq, prices)
}

/// The values of `lambda` [`best_lambda`] tries, in gold a cell.
pub const LAMBDAS: [i64; 16] = [1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 25, 30, 40, 60, 100];

/// Most regions of one cut whose subsets [`best_lambda`] tries.
const MAX_REGIONS: usize = 10;

/// The connected groups of `cells`.
fn regions(cells: Bits) -> Vec<Bits> {
    let mut left = cells;
    let mut out = Vec::new();
    while let Some(seed) = left.iter().next() {
        let mut region = Bits::cell(seed);
        loop {
            let next = (region | region.neighbours()) & left;
            if next == region {
                break;
            }
            region = next;
        }
        left = left.and_not(region);
        out.push(region);
    }
    out
}

/// The [`lambda_cut`]s over [`LAMBDAS`], each split into its separate
/// regions (a min-cut takes regions of equal worth all or none at a given
/// `lambda`): of every subset of a cut's regions, the one removing the most
/// cells for at most `budget` (the cheapest on a tie); `None` when none
/// removes a cell.
pub fn best_lambda(active: Bits, hq: usize, budget: i64, prices: &[i64; CELLS]) -> Option<Taken> {
    let mut best: Option<Taken> = None;
    let key = |t: &Taken| (t.removed.count(), -t.cost);
    let mut seen: Vec<Bits> = Vec::new();
    for lambda in LAMBDAS {
        let cut = lambda_cut(active, hq, lambda, prices);
        if cut.removed.is_empty() || seen.contains(&cut.captured) {
            continue;
        }
        seen.push(cut.captured);
        let parts: Vec<Bits> = regions(cut.removed)
            .into_iter()
            .map(|region| region & cut.captured)
            .filter(|part| !part.is_empty())
            .take(MAX_REGIONS)
            .collect();
        for mask in 1u32..(1 << parts.len()) {
            let mut captured = Bits::EMPTY;
            for (i, part) in parts.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    captured |= *part;
                }
            }
            let taken = Taken::of(active, hq, captured, prices);
            if taken.cost > budget || taken.removed.is_empty() {
                continue;
            }
            if best.as_ref().is_none_or(|b| key(&taken) > key(b)) {
                best = Some(taken);
            }
        }
    }
    best
}

/// The cheapest cut on `active` separating `target` from `hq` (nearest the
/// HQ of the cheapest), if it costs less than `budget`.
pub fn target_cut(
    active: Bits,
    hq: usize,
    target: usize,
    budget: i64,
    prices: &[i64; CELLS],
) -> Option<Taken> {
    if target == hq || !active.test(target) || !active.test(hq) {
        return None;
    }
    let mut flow = Flow::new(2 * CELLS);
    for cell in active.iter() {
        let through = if cell == hq || cell == target {
            INFINITE
        } else {
            prices[cell]
        };
        flow.edge(2 * cell, 2 * cell + 1, through);
        for next in (Bits::cell(cell).neighbours() & active).iter() {
            flow.edge(2 * cell + 1, 2 * next, INFINITE);
        }
    }
    let cost = flow.run(2 * target + 1, 2 * hq, budget);
    if cost >= budget {
        return None;
    }
    let side = flow.reaching(2 * hq);
    let mut captured = Bits::EMPTY;
    for cell in active.iter() {
        if side[2 * cell + 1] && !side[2 * cell] {
            captured.set(cell);
        }
    }
    for cell in captured.iter() {
        if prices[cell] == 0 {
            let mut fewer = captured;
            fewer.clear(cell);
            if !joined(active.and_not(fewer), hq).test(target) {
                captured = fewer;
            }
        }
    }
    Some(Taken::of(active, hq, captured, prices))
}

/// The [`target_cut`] removing the most cells for at most `budget`.
pub fn best_target(active: Bits, hq: usize, budget: i64, prices: &[i64; CELLS]) -> Option<Taken> {
    let mut best: Option<Taken> = None;
    for target in active.iter() {
        let Some(cut) = target_cut(active, hq, target, budget + 1, prices) else {
            continue;
        };
        let key = |t: &Taken| (t.removed.count(), -t.cost);
        if best.as_ref().is_none_or(|b| key(&cut) > key(b)) {
            best = Some(cut);
        }
    }
    best
}

/// How a cut is chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The cheapest cut to each cell, the best affordable ([`best_target`]).
    Target,
    /// The budget-aware cut ([`best_lambda`]).
    Lambda,
}

/// Up to `rounds` cuts in a row, each the best affordable on what the ones
/// before left, with the gold they left: the cells removed in all, and the
/// cuts made. The prices are not updated as the enemy's land grows.
pub fn several(
    active: Bits,
    hq: usize,
    budget: i64,
    prices: &[i64; CELLS],
    kind: Kind,
    rounds: usize,
) -> (i64, i64) {
    let (mut land, mut gold, mut removed, mut cuts) = (active, budget, 0, 0);
    for _ in 0..rounds {
        let cut = match kind {
            Kind::Target => best_target(land, hq, gold, prices),
            Kind::Lambda => best_lambda(land, hq, gold, prices),
        };
        let Some(cut) = cut else { break };
        removed += i64::from(cut.removed.count());
        cuts += 1;
        gold -= cut.cost;
        land = land.and_not(cut.removed);
    }
    (removed, cuts)
}

#[cfg(test)]
mod tests;
