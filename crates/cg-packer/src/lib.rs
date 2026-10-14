//! Packs bytes into characters outside ASCII, 127 bytes to 64 characters,
//! in the format that `cg_pack::unpack` decodes (docs/research/utf16-packing.md).
//!
//! This is a tool, for trainers and other programs that write data into
//! bots; bots never depend on it. The alphabet comes from `cg-pack`, so the
//! encoder and the decoder cannot disagree on it.

use cg_pack::{in_alphabet, BASE, BLOCK_BYTES, BLOCK_CHARS};

/// Packs `data`: its length as 4 bytes little-endian, the data, then zero
/// bytes up to a whole number of blocks of 127 bytes, each written as 64
/// base-[`BASE`] digits, most significant first.
///
/// # Panics
///
/// If `data` has 2^32 bytes or more, which the 4-byte length cannot hold.
pub fn pack(data: &[u8]) -> String {
    let len = u32::try_from(data.len()).expect("cg-pack's format holds less than 4 GiB");
    let alphabet: Vec<char> = (0..0x10000u32)
        .filter(|&c| in_alphabet(c))
        .map(|c| char::from_u32(c).expect("the alphabet has no surrogates"))
        .collect();
    debug_assert_eq!(alphabet.len(), BASE as usize);

    let mut bytes = Vec::with_capacity(data.len() + 4 + BLOCK_BYTES);
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(data);
    bytes.resize(bytes.len().div_ceil(BLOCK_BYTES) * BLOCK_BYTES, 0);

    let mut packed = String::with_capacity(bytes.len() / BLOCK_BYTES * BLOCK_CHARS * 3);
    let mut digits = [0u32; BLOCK_CHARS];
    for block in bytes.chunks(BLOCK_BYTES) {
        // 1,024 bits in 32 limbs of 32 bits, most significant first; the
        // top byte stays 0.
        let mut wide = [0u8; BLOCK_BYTES + 1];
        wide[1..].copy_from_slice(block);
        let mut limbs = [0u32; 32];
        for (limb, chunk) in limbs.iter_mut().zip(wide.chunks(4)) {
            *limb = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        // The digits come out least significant first.
        for digit in digits.iter_mut().rev() {
            let mut remainder = 0u64;
            for limb in limbs.iter_mut() {
                let v = (remainder << 32) | u64::from(*limb);
                *limb = (v / u64::from(BASE)) as u32;
                remainder = v % u64::from(BASE);
            }
            *digit = remainder as u32;
        }
        debug_assert!(limbs.iter().all(|&limb| limb == 0), "BASE^64 >= 2^1016");
        packed.extend(digits.iter().map(|&digit| alphabet[digit as usize]));
    }
    packed
}

#[cfg(test)]
mod tests;
