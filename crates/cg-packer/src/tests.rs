use super::*;

/// Bytes from a fixed xorshift64 sequence, the same on every platform.
fn noise(len: usize, seed: u64) -> Vec<u8> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 56) as u8
        })
        .collect()
}

/// The packing done the slow way, one byte at a time: an independent check
/// of the format that `pack` writes.
fn pack_slowly(data: &[u8]) -> String {
    let alphabet: Vec<char> = (0..0x10000u32)
        .filter(|&c| in_alphabet(c))
        .map(|c| char::from_u32(c).unwrap())
        .collect();
    let mut bytes = (data.len() as u32).to_le_bytes().to_vec();
    bytes.extend_from_slice(data);
    while !bytes.len().is_multiple_of(BLOCK_BYTES) {
        bytes.push(0);
    }
    let mut packed = String::new();
    for block in bytes.chunks(BLOCK_BYTES) {
        let mut number = block.to_vec();
        let mut digits = Vec::new();
        for _ in 0..BLOCK_CHARS {
            let mut remainder = 0u32;
            for byte in number.iter_mut() {
                let v = remainder * 256 + u32::from(*byte);
                *byte = (v / BASE) as u8;
                remainder = v % BASE;
            }
            digits.push(alphabet[remainder as usize]);
        }
        assert!(number.iter().all(|&b| b == 0));
        packed.extend(digits.iter().rev());
    }
    packed
}

#[test]
fn writes_the_documented_format() {
    for (len, seed) in [(0, 1), (1, 2), (122, 3), (123, 4), (124, 5), (500, 6)] {
        let data = noise(len, seed);
        assert_eq!(pack(&data), pack_slowly(&data), "{len} bytes");
    }
    assert_eq!(pack(&[0xFF; 300]), pack_slowly(&[0xFF; 300]));
}

#[test]
fn round_trips_every_length_around_the_block_edges() {
    for len in (0..=260).chain([381, 1_000, 4_096]) {
        let data = noise(len, len as u64 + 7);
        let packed = pack(&data);
        let blocks = (len + 4).div_ceil(BLOCK_BYTES);
        assert_eq!(packed.chars().count(), blocks * BLOCK_CHARS, "{len} bytes");
        assert_eq!(cg_pack::unpack(&packed), Some(data), "{len} bytes");
    }
}

#[test]
fn round_trips_extreme_bytes() {
    for data in [vec![0u8; 1_000], vec![0xFF; 1_000]] {
        assert_eq!(cg_pack::unpack(&pack(&data)), Some(data));
    }
}

#[test]
fn round_trips_a_bot_sized_payload_in_characters_codingame_counts_once() {
    // 190,000 bytes and the length fill 1,497 blocks: 95,808 characters,
    // near the 100,000 limit (the research note's measure).
    let data = noise(190_000, 42);
    let packed = pack(&data);
    assert!(packed.chars().all(|c| in_alphabet(c as u32)));
    assert_eq!(packed.encode_utf16().count(), packed.chars().count());
    assert_eq!(packed.chars().count(), 95_808);
    assert_eq!(cg_pack::unpack(&packed), Some(data));
}

#[test]
fn a_changed_character_does_not_go_unnoticed_or_spread() {
    let data = noise(1_000, 9);
    let packed: Vec<char> = pack(&data).chars().collect();
    let mut changed = packed.clone();
    changed[BLOCK_CHARS + 10] = if packed[BLOCK_CHARS + 10] == '\u{100}' {
        '\u{101}'
    } else {
        '\u{100}'
    };
    let changed: String = changed.into_iter().collect();
    let unpacked = cg_pack::unpack(&changed).expect("still a valid block");
    assert_ne!(unpacked, data);
    // Only the second block's bytes differ.
    let second = BLOCK_BYTES - 4..2 * BLOCK_BYTES - 4;
    for (i, (a, b)) in unpacked.iter().zip(&data).enumerate() {
        if !second.contains(&i) {
            assert_eq!(a, b, "byte {i}");
        }
    }
}

#[test]
fn writes_a_module_with_one_constant() {
    let data = noise(300, 11);
    let module = rust_module(&data, "WEIGHTS", "`weights.bin`").unwrap();
    let lines: Vec<&str> = module.lines().collect();
    assert_eq!(lines.len(), 3, "{module}");
    assert_eq!(
        lines[0],
        "// Written by cg-packer from `weights.bin`: 300 bytes in 192 characters."
    );
    let literal = lines[2]
        .strip_prefix("pub const WEIGHTS: &str = \"")
        .and_then(|rest| rest.strip_suffix("\";"))
        .expect("one constant");
    assert_eq!(cg_pack::unpack(literal), Some(data));
    assert!(module.ends_with(";\n"));
}

#[test]
fn a_source_with_a_newline_stays_in_its_comment() {
    let module = rust_module(&[1], "W", "`a\nb\r.bin`").unwrap();
    assert_eq!(module.lines().count(), 3, "{module}");
    assert!(module.starts_with("// Written by cg-packer from `a b .bin`: "));
}

#[test]
fn a_module_needs_a_constants_name() {
    for name in ["", "weights", "Weights", "_W", "9W", "W-1", "W 1", "WÉ"] {
        assert!(rust_module(&[1], name, "x").is_err(), "{name:?}");
    }
    for name in ["W", "WEIGHTS", "NET_2", "A_B_C"] {
        assert!(rust_module(&[1], name, "x").is_ok(), "{name:?}");
    }
}

#[test]
fn the_format_stays_the_one_codingame_accepted() {
    // Every byte value, packed. The expected hash was taken from this
    // encoder; the research note's decoder, the one of the probe CodinGame
    // accepted, reads that output back (checked 2026-10-14).
    // A change to the alphabet, the byte order, the length or the padding,
    // on both sides at once, changes it.
    let packed = pack(&(0..=255).collect::<Vec<u8>>());
    let hash = packed.bytes().fold(0xCBF2_9CE4_8422_2325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01B3)
    });
    assert_eq!(packed.chars().count(), 3 * BLOCK_CHARS);
    assert_eq!(hash, 14_360_896_311_314_249_363);
}
