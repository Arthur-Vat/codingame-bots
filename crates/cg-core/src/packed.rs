//! Compact text for many small numbers, such as a bot's learned weights:
//! a Huffman code written in base64 digits, so that common values cost a
//! few bits and the text stays plain ASCII inside a Rust string.
//!
//! A text is one or more segments, one after another; each holds its own
//! code, so that numbers with different spreads (the tables of a model)
//! are coded apart. A segment is, in base64 digits ([`DIGITS`]):
//!
//! - how many numbers it holds, in 3 digits (most significant first);
//! - the smallest and the largest number, 1 digit each;
//! - the code length of each number from the smallest to the largest, 1
//!   digit each, 0 for numbers that do not occur;
//! - the numbers' canonical Huffman codes, 6 bits per digit, the first bit
//!   the most significant, the last digit padded with zeros.
//!
//! An empty segment is its count, 0, alone.

/// The 64 digits, in order.
pub const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The longest code allowed, so that codes fit a `u32`.
const LONGEST: usize = 30;

/// The value of base64 digit `byte`.
fn digit(byte: u8) -> Result<u32, String> {
    DIGITS
        .iter()
        .position(|&d| d == byte)
        .map(|value| value as u32)
        .ok_or_else(|| format!("{:?} is not a base64 digit", char::from(byte)))
}

/// Reads the numbers of `text`'s first segment; returns them and the text
/// after the segment.
pub fn decode(text: &str) -> Result<(Vec<u8>, &str), String> {
    let bytes = text.as_bytes();
    let mut at = 0;
    let mut next = || -> Result<u32, String> {
        let byte = *bytes.get(at).ok_or("the segment ends early")?;
        at += 1;
        digit(byte)
    };
    let count = (next()? << 12 | next()? << 6 | next()?) as usize;
    if count == 0 {
        return Ok((Vec::new(), &text[3..]));
    }
    let (low, high) = (next()?, next()?);
    if high < low {
        return Err(format!("values from {low} to {high}"));
    }
    // Numbers by code length, then value: the canonical order.
    let mut lengths = Vec::new();
    for value in low..=high {
        let length = next()? as usize;
        if length > LONGEST {
            return Err(format!("a code of {length} bits"));
        }
        if length > 0 {
            lengths.push((length, value as u8));
        }
    }
    lengths.sort_unstable();
    let symbols: Vec<u8> = lengths.iter().map(|&(_, value)| value).collect();
    let mut per_length = [0u32; LONGEST + 1];
    for &(length, _) in &lengths {
        per_length[length] += 1;
    }
    let mut values = Vec::with_capacity(count);
    let (mut buffer, mut bits) = (0u32, 0);
    let (mut code, mut first, mut index, mut length) = (0u32, 0u32, 0u32, 0);
    while values.len() < count {
        if bits == 0 {
            buffer = next()?;
            bits = 6;
        }
        bits -= 1;
        code = code << 1 | (buffer >> bits & 1);
        length += 1;
        if length > LONGEST {
            return Err("no code matches the bits".to_string());
        }
        // Codes of this length run from `first` to `first + count - 1`.
        let here = per_length[length];
        if code < first + here {
            values.push(symbols[(index + code - first) as usize]);
            (code, first, index, length) = (0, 0, 0, 0);
        } else {
            index += here;
            first = (first + here) << 1;
        }
    }
    Ok((values, &text[at..]))
}

/// Reads every segment of `text`.
pub fn decode_all(mut text: &str) -> Result<Vec<Vec<u8>>, String> {
    let mut segments = Vec::new();
    while !text.is_empty() {
        let (values, rest) = decode(text)?;
        segments.push(values);
        text = rest;
    }
    Ok(segments)
}

/// Reads `stride` segments into one table: segment `k` holds the numbers
/// at `k`, `k + stride`, `k + 2 * stride`... ([`encode_strided`]). Returns
/// the table and the text after it.
pub fn decode_strided(text: &str, stride: usize) -> Result<(Vec<u8>, &str), String> {
    let mut segments = Vec::with_capacity(stride);
    let mut rest = text;
    for _ in 0..stride {
        let (values, after) = decode(rest)?;
        segments.push(values);
        rest = after;
    }
    let len: usize = segments.iter().map(Vec::len).sum();
    let mut table = vec![0; len];
    for (k, values) in segments.iter().enumerate() {
        for (i, &value) in values.iter().enumerate() {
            *table
                .get_mut(k + i * stride)
                .ok_or("the segments' lengths do not interleave")? = value;
        }
    }
    Ok((table, rest))
}

/// Writes `values` as `stride` segments, segment `k` holding the numbers at
/// `k`, `k + stride`... each with its own code: for tables whose
/// neighbouring entries spread differently, such as weights by kind.
pub fn encode_strided(values: &[u8], stride: usize) -> String {
    (0..stride)
        .map(|k| {
            let segment: Vec<u8> = values.iter().skip(k).step_by(stride).copied().collect();
            encode(&segment)
        })
        .collect()
}

/// Code lengths of a Huffman code for `counts`, by value; 0 for values
/// that do not occur, 1 for a lone value.
fn code_lengths(counts: &[u64]) -> Vec<usize> {
    // Nodes: (weight, values below), merged two lightest at a time.
    let mut nodes: Vec<(u64, Vec<usize>)> = counts
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count > 0)
        .map(|(value, &count)| (count, vec![value]))
        .collect();
    let mut lengths = vec![0; counts.len()];
    if nodes.len() == 1 {
        lengths[nodes[0].1[0]] = 1;
        return lengths;
    }
    while nodes.len() > 1 {
        nodes.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
        let (light, light_values) = nodes.pop().expect("two nodes");
        let (other, mut values) = nodes.pop().expect("two nodes");
        values.extend(light_values);
        for &value in &values {
            lengths[value] += 1;
        }
        nodes.push((light + other, values));
    }
    lengths
}

/// Writes `values`, each below 64, as one segment.
pub fn encode(values: &[u8]) -> String {
    assert!(
        values.len() < 1 << 18,
        "a segment holds fewer than 2^18 numbers"
    );
    let mut text = String::new();
    let mut push = |value: u32| text.push(char::from(DIGITS[value as usize]));
    let count = values.len() as u32;
    push(count >> 12);
    push(count >> 6 & 63);
    push(count & 63);
    if values.is_empty() {
        return text;
    }
    let mut counts = [0u64; 64];
    for &value in values {
        assert!(value < 64, "{value} is not below 64");
        counts[usize::from(value)] += 1;
    }
    let lengths = code_lengths(&counts);
    assert!(lengths.iter().all(|&length| length <= LONGEST));
    let low = lengths
        .iter()
        .position(|&length| length > 0)
        .expect("a value");
    let high = lengths
        .iter()
        .rposition(|&length| length > 0)
        .expect("a value");
    push(low as u32);
    push(high as u32);
    for &length in &lengths[low..=high] {
        push(length as u32);
    }
    // Canonical codes: by length, then value, counting up.
    let mut order: Vec<usize> = (low..=high).filter(|&value| lengths[value] > 0).collect();
    order.sort_by_key(|&value| (lengths[value], value));
    let mut codes = [0u32; 64];
    let (mut code, mut length) = (0u32, lengths[order[0]]);
    for &value in &order {
        code <<= lengths[value] - length;
        length = lengths[value];
        codes[value] = code;
        code += 1;
    }
    let (mut buffer, mut bits) = (0u32, 0);
    for &value in values {
        let (code, length) = (codes[usize::from(value)], lengths[usize::from(value)]);
        for bit in (0..length).rev() {
            buffer = buffer << 1 | (code >> bit & 1);
            bits += 1;
            if bits == 6 {
                push(buffer);
                (buffer, bits) = (0, 0);
            }
        }
    }
    if bits > 0 {
        push(buffer << (6 - bits));
    }
    text
}

#[cfg(test)]
mod tests;
