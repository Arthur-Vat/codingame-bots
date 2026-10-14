use super::*;

/// The alphabet's characters in order: a digit's value is its index.
fn alphabet() -> Vec<char> {
    (0..0x10000u32)
        .filter(|&c| in_alphabet(c))
        .map(|c| char::from_u32(c).unwrap())
        .collect()
}

#[test]
fn the_alphabet_has_base_characters() {
    assert_eq!(alphabet().len(), BASE as usize);
}

#[test]
fn base_digits_of_a_block_hold_its_bytes() {
    // BASE^64 >= 2^1016, so every block of 127 bytes has 64 digits.
    assert!(BLOCK_CHARS as f64 * f64::from(BASE).log2() >= (BLOCK_BYTES * 8) as f64);
    // And a smaller base would not do: 60,097 is the least that works.
    assert!(BLOCK_CHARS as f64 * 60_096f64.log2() < (BLOCK_BYTES * 8) as f64);
}

#[test]
fn the_alphabet_leaves_out_what_editors_or_rustc_change() {
    for c in [
        0x00, 0x22, 0x5C, 0x7F, 0xFF, 0xD800, 0xDFFF, 0x2028, 0x2029, 0xFEFF, 0xFDD0, 0xFDEF,
        0x202A, 0x202E, 0x2066, 0x2069, 0xFFF0, 0xFFFF, 0x10000, 0x1F600,
    ] {
        assert!(!in_alphabet(c), "U+{c:04X} must be left out");
    }
    for c in [
        0x100, 0x4E00, 0xD7FF, 0xE000, 0x2027, 0x202F, 0x2065, 0x206A, 0xFDCF, 0xFDF0, 0xFFEF,
    ] {
        assert!(in_alphabet(c), "U+{c:04X} must be in");
    }
}

#[test]
fn a_block_of_zero_digits_is_empty_data() {
    let first = alphabet()[0];
    let packed: String = std::iter::repeat_n(first, BLOCK_CHARS).collect();
    assert_eq!(unpack(&packed), Some(Vec::new()));
}

#[test]
fn reads_the_length_and_the_data_of_a_block() {
    // The block's number is 1 * 256^126 + 0x2A * 256^122: length 1, data 0x2A.
    // Its digits are built by dividing the 127 bytes by BASE, the slow way.
    let mut bytes = [0u8; BLOCK_BYTES];
    bytes[0] = 1;
    bytes[4] = 0x2A;
    let alphabet = alphabet();
    let mut digits = Vec::new();
    for _ in 0..BLOCK_CHARS {
        let mut remainder = 0u32;
        for byte in bytes.iter_mut() {
            let v = remainder * 256 + u32::from(*byte);
            *byte = (v / BASE) as u8;
            remainder = v % BASE;
        }
        digits.push(alphabet[remainder as usize]);
    }
    assert!(bytes.iter().all(|&b| b == 0));
    let packed: String = digits.into_iter().rev().collect();
    assert_eq!(unpack(&packed), Some(vec![0x2A]));
}

#[test]
fn rejects_what_the_encoder_never_writes() {
    let alphabet = alphabet();
    let zero = alphabet[0];
    let top = alphabet[BASE as usize - 1];
    let block = |c: char| -> String { std::iter::repeat_n(c, BLOCK_CHARS).collect() };

    assert_eq!(unpack(""), None, "no block");
    assert_eq!(
        unpack(&block(zero)[..zero.len_utf8() * 63]),
        None,
        "63 characters"
    );
    let ascii: String = block(zero).replacen(zero, "A", 1);
    assert_eq!(unpack(&ascii), None, "an ASCII character");
    let outside: String = block(zero).replacen(zero, "\u{1F600}", 1);
    assert_eq!(unpack(&outside), None, "a character above U+FFFF");
    let rejected: String = block(zero).replacen(zero, "\u{202E}", 1);
    assert_eq!(unpack(&rejected), None, "a text-direction control");
    assert_eq!(unpack(&block(top)), None, "BASE^64 - 1 is 2^1016 or more");
    // The last digit 1 gives a nonzero padding byte with a zero length.
    let mut padding = block(zero);
    padding.pop();
    padding.push(alphabet[1]);
    assert_eq!(unpack(&padding), None, "padding that is not zero");
    assert_eq!(unpack(&block(zero).repeat(2)), None, "a block of padding");
}
