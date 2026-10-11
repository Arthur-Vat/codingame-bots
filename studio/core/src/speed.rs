//! Think time and iterations.
//!
//! A computer seat gets a think time in milliseconds, but a takeback needs
//! the bot to replay exactly, so it plays a fixed number of iterations. The
//! rate (iterations per millisecond) converts one to the other; measuring it
//! is the caller's job.

/// The iterations for `think_ms` at `rate` iterations per millisecond.
pub fn iterations(think_ms: u64, rate: f64) -> u64 {
    ((think_ms as f64 * rate).round() as u64).max(1)
}

/// The median of `samples`; 0 when there are none.
pub fn median(samples: &mut [f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    samples.sort_by(f64::total_cmp);
    let middle = samples.len() / 2;
    if samples.len() % 2 == 1 {
        samples[middle]
    } else {
        (samples[middle - 1] + samples[middle]) / 2.0
    }
}
