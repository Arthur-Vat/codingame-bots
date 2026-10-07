//! How long a search may run.

use std::time::{Duration, Instant};

use cg_core::time::time_scale;

/// Environment variable that replaces time budgets by a fixed number of
/// iterations, for reproducible tests. CodinGame never sets it.
pub const FIXED_ITERATIONS_ENV: &str = "CG_FIXED_ITERS";

/// When a search stops.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Budget {
    /// At this instant.
    Until(Instant),
    /// After this many iterations, however long they take.
    Iterations(u64),
}

impl Budget {
    /// The budget of a turn that started at `start`, when the game allows
    /// `limit` per answer on CodinGame.
    ///
    /// With `CG_FIXED_ITERS` set, a fixed number of iterations. Otherwise
    /// the search uses `share` of the limit minus `reserve` (kept for
    /// reading, writing and timing noise on CodinGame), all scaled by the
    /// arena's time factor ([`time_scale`]). Scaling the reserve too keeps
    /// scaled games proportional to real ones; the arena absorbs the noise
    /// of its own machine with a tolerance of its own (ADR 0013).
    pub fn for_turn(start: Instant, limit: Duration, share: f64, reserve: Duration) -> Budget {
        match fixed_iterations(std::env::var(FIXED_ITERATIONS_ENV).ok().as_deref()) {
            Some(iterations) => Budget::Iterations(iterations),
            None => Budget::Until(start + search_time(limit, time_scale(), share, reserve)),
        }
    }

    /// Whether a search that ran `iterations` iterations must stop.
    #[inline]
    pub fn is_spent(&self, iterations: u64) -> bool {
        match *self {
            Budget::Until(deadline) => Instant::now() >= deadline,
            Budget::Iterations(limit) => iterations >= limit,
        }
    }
}

/// The search time for a turn: `share` of the limit minus `reserve`, times
/// `scale`, and never negative.
fn search_time(limit: Duration, scale: f64, share: f64, reserve: Duration) -> Duration {
    limit.mul_f64(share).saturating_sub(reserve).mul_f64(scale)
}

fn fixed_iterations(value: Option<&str>) -> Option<u64> {
    value?.trim().parse().ok().filter(|&n| n > 0)
}

#[cfg(test)]
mod tests;
