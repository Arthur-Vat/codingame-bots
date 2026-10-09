use super::*;
use crate::rng::Rng;

#[test]
fn values_come_back_and_common_values_cost_less() {
    let mut rng = Rng::new(4);
    // Mostly 32, spreading out: as learned weights in quarter steps.
    let values: Vec<u8> = (0..20_000)
        .map(|_| {
            let spread = rng.below(8) as i32 - rng.below(8) as i32;
            (32 + spread * spread.abs() / 2).clamp(0, 63) as u8
        })
        .collect();
    let text = encode(&values);
    assert!(text.bytes().all(|byte| DIGITS.contains(&byte)));
    // A Huffman code takes less than the values' entropy plus one bit
    // each; the header takes 3 + 2 + 64 digits at most.
    let mut counts = [0f64; 64];
    for &value in &values {
        counts[usize::from(value)] += 1.0;
    }
    let total = values.len() as f64;
    let entropy: f64 = counts
        .iter()
        .filter(|&&count| count > 0.0)
        .map(|&count| -count * (count / total).log2())
        .sum();
    let bits = 6.0 * (text.len() - 69) as f64;
    assert!(bits < entropy + total, "{bits} bits for {entropy:.0}");
    assert!(text.len() < values.len() * 2 / 3, "{} digits", text.len());
    let (decoded, rest) = decode(&text).unwrap();
    assert_eq!(decoded, values);
    assert_eq!(rest, "");
}

#[test]
fn segments_follow_one_another() {
    let first: Vec<u8> = (0..64).collect();
    let second = vec![7u8; 1000];
    let third: Vec<u8> = Vec::new();
    let fourth = vec![0u8, 63, 63, 0, 1];
    let text = [&first, &second, &third, &fourth]
        .iter()
        .map(|values| encode(values))
        .collect::<String>();
    let segments = decode_all(&text).unwrap();
    assert_eq!(segments, vec![first, second, third, fourth]);
}

#[test]
fn a_lone_value_takes_one_bit_each() {
    let text = encode(&[5; 60]);
    // Count, smallest, largest, one length, then 60 bits.
    assert_eq!(text.len(), 3 + 2 + 1 + 10);
    assert_eq!(decode(&text).unwrap().0, vec![5; 60]);
}

#[test]
fn broken_texts_are_errors() {
    let text = encode(&[1, 2, 3, 3, 3, 3]);
    assert!(decode(&text[..text.len() - 1]).is_err());
    assert!(decode("A").is_err());
    assert!(decode("AA!").is_err());
    // Largest below smallest.
    assert!(decode("AABDA").is_err());
}

#[test]
fn strided_tables_come_back() {
    // Even entries spread, odd ones always 9: coded apart, the odd ones
    // cost one bit each.
    let table: Vec<u8> = (0..999u32)
        .map(|i| if i % 2 == 0 { (i * 7 % 64) as u8 } else { 9 })
        .collect();
    let text = encode_strided(&table, 2) + &encode(&[1, 2]);
    let (decoded, rest) = decode_strided(&text, 2).unwrap();
    assert_eq!(decoded, table);
    assert_eq!(decode(rest).unwrap().0, vec![1, 2]);
    assert!(encode_strided(&table, 2).len() < encode(&table).len());
    // Lengths that cannot interleave.
    let bad = encode(&[1, 2]) + &encode(&[3, 4, 5, 6]);
    assert!(decode_strided(&bad, 2).is_err());
}
