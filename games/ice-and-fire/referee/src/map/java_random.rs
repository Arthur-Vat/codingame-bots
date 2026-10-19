//! The linear congruential generator of Java's `java.util.Random`, as written in
//! `games/ice-and-fire/MAP_GENERATOR.md` ("The random numbers"), so that the
//! generator draws the same numbers as the source's.

const MULTIPLIER: u64 = 0x5_DEEC_E66D;
const INCREMENT: u64 = 0xB;
const MASK: u64 = (1 << 48) - 1;

/// A generator with 48 bits of state.
pub struct JavaRandom {
    state: u64,
}

impl JavaRandom {
    /// Starts from `seed`, a 64-bit signed integer taken bit for bit.
    pub fn new(seed: i64) -> Self {
        Self {
            state: (seed as u64 ^ MULTIPLIER) & MASK,
        }
    }

    /// The next `bits` bits (1 to 32), read as a 32-bit signed integer.
    fn next(&mut self, bits: u32) -> i32 {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MASK;
        (self.state >> (48 - bits)) as u32 as i32
    }

    /// An integer in `0..bound`; `bound` is positive.
    pub fn below(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "the bound must be positive");
        let mut r = self.next(31);
        if bound & (bound - 1) == 0 {
            return ((i64::from(bound) * i64::from(r)) >> 31) as i32;
        }
        loop {
            let candidate = r % bound;
            if r.wrapping_sub(candidate).wrapping_add(bound - 1) >= 0 {
                return candidate;
            }
            r = self.next(31);
        }
    }

    /// A fraction in `[0, 1)`, in single precision.
    pub fn fraction(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }
}

#[cfg(test)]
mod tests;
