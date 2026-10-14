//! Bytes packed into characters outside ASCII, 127 bytes to 64 characters,
//! which a bot decodes once at start (docs/research/utf16-packing.md).
//! CodinGame counts a bot's size in characters, so this carries about 2.6
//! times as much data as base64 (ADR 0037).
//!
//! The packed text is a sequence of blocks of 64 characters. Each block is
//! one number below 2^1016 written in base [`BASE`], most significant digit
//! first, a digit being a character's rank in the alphabet ([`in_alphabet`]).
//! The number's 127 bytes, big-endian, follow each other; the first 4 give
//! the data's length, little-endian, then come the data and zero bytes up to
//! the end of the last block.
//!
//! This crate uses the standard library only, and `cg-bundler` copies it into
//! the bots that depend on it. The encoder is in `cg-packer`, which bots
//! never use; both read the alphabet from here.

/// Bytes in a block.
pub const BLOCK_BYTES: usize = 127;

/// Characters in a block.
pub const BLOCK_CHARS: usize = 64;

/// The number of characters in the alphabet, the base of each digit.
pub const BASE: u32 = 63_172;

/// Whether code point `c` is in the alphabet: U+0100 to U+FFEF, without
/// surrogates, line and paragraph separators, the byte order mark,
/// noncharacters and the text-direction controls that rustc rejects in
/// string literals.
pub fn in_alphabet(c: u32) -> bool {
    (0x100..0xFFF0).contains(&c)
        && !(0xD800..0xE000).contains(&c)
        && c != 0x2028
        && c != 0x2029
        && c != 0xFEFF
        && !(0xFDD0..0xFDF0).contains(&c)
        && !(0x202A..=0x202E).contains(&c)
        && !(0x2066..=0x2069).contains(&c)
}

/// The bytes packed in `packed`, or `None` if it is not the output of the
/// encoder: a length that is not a whole number of blocks, a character
/// outside the alphabet, a block of 2^1016 or more, a length beyond the
/// blocks, or padding that is not zero or fills a whole block.
pub fn unpack(packed: &str) -> Option<Vec<u8>> {
    let mut rank = vec![u32::MAX; 0x10000];
    let mut next = 0;
    for c in 0..0x10000u32 {
        if in_alphabet(c) {
            rank[c as usize] = next;
            next += 1;
        }
    }
    let chars: Vec<char> = packed.chars().collect();
    if chars.is_empty() || !chars.len().is_multiple_of(BLOCK_CHARS) {
        return None;
    }
    let mut bytes = Vec::with_capacity(chars.len() / BLOCK_CHARS * BLOCK_BYTES);
    for block in chars.chunks(BLOCK_CHARS) {
        // 1,024 bits in 32 limbs of 32 bits, most significant first.
        let mut limbs = [0u32; 32];
        for &c in block {
            let digit = *rank.get(c as usize)?;
            if digit == u32::MAX {
                return None;
            }
            let mut carry = u64::from(digit);
            for limb in limbs.iter_mut().rev() {
                let v = u64::from(*limb) * u64::from(BASE) + carry;
                *limb = v as u32;
                carry = v >> 32;
            }
            if carry != 0 {
                return None;
            }
        }
        if limbs[0] >> 24 != 0 {
            return None;
        }
        for (i, limb) in limbs.iter().enumerate() {
            let limb_bytes = limb.to_be_bytes();
            bytes.extend_from_slice(if i == 0 {
                &limb_bytes[1..]
            } else {
                &limb_bytes
            });
        }
    }
    let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    let end = len.checked_add(4)?;
    if end > bytes.len() || bytes.len() - end >= BLOCK_BYTES || bytes[end..].iter().any(|&b| b != 0)
    {
        return None;
    }
    bytes.truncate(end);
    bytes.drain(..4);
    Some(bytes)
}

#[cfg(test)]
mod tests;
