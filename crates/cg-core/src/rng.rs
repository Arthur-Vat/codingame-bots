//! Seeded pseudo-random numbers: xoshiro256++, seeded through SplitMix64.
//!
//! Not for cryptography. The same seed gives the same sequence on every
//! platform, which makes local matches reproducible.

use std::time::{SystemTime, UNIX_EPOCH};

/// Environment variable the arena sets to give each bot a reproducible seed.
/// CodinGame never sets it.
pub const SEED_ENV: &str = "CG_SEED";

/// The seed from `CG_SEED` when it is set to an integer, otherwise one
/// derived from the clock.
pub fn seed_from_env_or_clock() -> u64 {
    parse_seed(std::env::var(SEED_ENV).ok().as_deref()).unwrap_or_else(clock_seed)
}

fn parse_seed(value: Option<&str>) -> Option<u64> {
    value?.trim().parse().ok()
}

fn clock_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u64)
        .unwrap_or(0);
    // The address of a local variable differs between runs, which separates
    // two processes started in the same nanosecond.
    let local = 0u8;
    let address = &local as *const u8 as u64;
    nanos ^ address.rotate_left(32)
}

/// Fast, seedable random number generator (xoshiro256++).
#[derive(Clone, Debug)]
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// A generator whose whole sequence is determined by `seed`.
    pub fn new(seed: u64) -> Self {
        let mut mix = seed;
        Rng {
            state: [
                split_mix(&mut mix),
                split_mix(&mut mix),
                split_mix(&mut mix),
                split_mix(&mut mix),
            ],
        }
    }

    /// The next 64 random bits.
    pub fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = &mut self.state;
        let result = s0.wrapping_add(*s3).rotate_left(23).wrapping_add(*s0);
        let t = *s1 << 17;
        *s2 ^= *s0;
        *s3 ^= *s1;
        *s1 ^= *s2;
        *s0 ^= *s3;
        *s2 ^= t;
        *s3 = s3.rotate_left(45);
        result
    }

    /// A uniformly random integer in `0..n`, without bias (Lemire's method).
    ///
    /// # Panics
    ///
    /// If `n` is 0.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "Rng::below(0) has no possible value");
        let mut product = u128::from(self.next_u64()) * u128::from(n);
        if (product as u64) < n {
            let threshold = n.wrapping_neg() % n;
            while (product as u64) < threshold {
                product = u128::from(self.next_u64()) * u128::from(n);
            }
        }
        (product >> 64) as u64
    }

    /// A uniformly random float in `[0, 1)`.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// A uniformly random element of `items`, or `None` if it is empty.
    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            items.get(self.below(items.len() as u64) as usize)
        }
    }

    /// Shuffles `items` uniformly (Fisher-Yates).
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i as u64 + 1) as usize;
            items.swap(i, j);
        }
    }
}

fn split_mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests;
