# Packing data into characters

How to fit about twice as much data in a bot by packing bytes into characters outside ASCII, about 15.9 bits per character instead of the 6 bits of base64. CodinGame is reported to limit code size in characters, not bytes ([CODINGAME.md](../CODINGAME.md#submission-limits)).

**Status:** not usable in a release yet. Three things are missing first (below, "Before a release uses it"). This note keeps the method for any game that needs room for weights or tables.

## The method

The payload is a sequence of bytes: weights, tables, anything the bot decodes once at start. Its length goes first, as 4 bytes, little-endian. Then zero bytes pad it to a multiple of 127.

Each block of 127 bytes (1,016 bits) is read as one big-endian number. That number is written in base B with exactly 64 digits, most significant first. Each digit is written as one character from an alphabet of B code points. 64 characters carry 1,016 bits, so 15.875 bits each. This needs B to the power 64 to be at least 2 to the power 1,016, so B ≥ 60,097.

The alphabet is every code point from U+0100 to U+FFEF, in order, without the following:

| Left out | Why |
| --- | --- |
| U+0000 to U+00FF | ASCII and Latin-1: quotes, backslashes, control and line-end characters, which a string literal or an editor would change |
| U+D800 to U+DFFF | Surrogates: not characters, and invalid in Rust's `char` |
| U+2028, U+2029, U+FEFF | Line and paragraph separators, and the byte order mark, which editors and web pages may treat specially (the source) |
| U+FDD0 to U+FDEF | Noncharacters (the source) |
| U+202A to U+202E, U+2066 to U+2069 | Text-direction controls: rustc rejects them in string literals (lint `text_direction_codepoint_in_literal`, an error by default; tested here, see below) |
| U+FFF0 and above | Specials, and code points above U+FFFF, which take two UTF-16 units and could count as two characters |

The alphabet then has B = 63,172 code points, 15.95 bits each, enough for 64 digits per 127 bytes. The source's alphabet, built for Python, keeps the 9 text-direction controls: B = 63,181. Both sides of the packing must use the same alphabet.

A digit's value is its rank in the alphabet. The decoder builds a table of ranks for the 65,536 code points once. For each block it multiplies a 1,016-bit number by B and adds the next digit, 64 times. Then it writes out the 127 bytes. The Rust below is standard library only, with no `unsafe`, and compiles for edition 2021:

```rust
/// Rebuilds the bytes packed 127 to 64 characters as base-B digits
/// (docs/research/utf16-packing.md).
fn unpack(packed: &str) -> Vec<u8> {
    let allowed = |c: u32| {
        !(0xD800..0xE000).contains(&c)
            && c != 0x2028
            && c != 0x2029
            && c != 0xFEFF
            && !(0xFDD0..0xFDF0).contains(&c)
            && !(0x202A..=0x202E).contains(&c)
            && !(0x2066..=0x2069).contains(&c)
    };
    let mut rank = vec![0u16; 0x10000];
    let mut base: u32 = 0;
    for c in 0x100..0xFFF0u32 {
        if allowed(c) {
            rank[c as usize] = base as u16;
            base += 1;
        }
    }
    let chars: Vec<u32> = packed.chars().map(|c| c as u32).collect();
    let mut out = Vec::with_capacity(chars.len() / 64 * 127);
    for block in chars.chunks(64) {
        // 1,016 bits in 32 limbs of 32 bits, most significant first.
        let mut limbs = [0u32; 32];
        for &c in block {
            let mut carry = u64::from(rank[c as usize]);
            for limb in limbs.iter_mut().rev() {
                let v = u64::from(*limb) * u64::from(base) + carry;
                *limb = v as u32;
                carry = v >> 32;
            }
        }
        let bytes: Vec<u8> = limbs.iter().flat_map(|l| l.to_be_bytes()).collect();
        out.extend_from_slice(&bytes[1..]); // the top byte of 1,024 bits is 0
    }
    let len = u32::from_le_bytes([out[0], out[1], out[2], out[3]]) as usize;
    out[4..4 + len].to_vec()
}
```

The encoder does the reverse with Python's big integers, or with any big-number code. The repository has no encoder yet.

## Measured here

On 2026-10-14, in Claude's session, with rustc 1.97.0 (newer than CodinGame's 1.90.0):

- **Round trip:** 190,000 random bytes packed into 95,808 characters, and unpacked to the same bytes by the decoder above.
- **Decoding time:** 2.7 ms with `-C opt-level=3`, and 43 ms without optimization, on the session's CPU. Either fits a first turn of 1 s.
- **File size:** the bot file holding those characters had 97,504 characters, and as many UTF-16 units. In UTF-8 it was 285,409 bytes, about 3 bytes per packed character. `scripts/cg-check.sh` rejects it today, since it counts bytes.
- **What rustc accepts in a string literal:** separators, the byte order mark, noncharacters, private-use and unassigned code points all compile. U+202E and U+2066 do not.

## When it fits

- **Data, not code.** It packs bytes that the bot decodes at start: network weights, pattern tables, opening books. The code itself stays plain Rust that CodinGame compiles.
- **Worth it only when the data is large.** A bot whose data fits in base64 gains nothing. Today's `uttt-v010` file is 77,026 characters, all ASCII; its weights take part of that.
- **Packing is not compression.** Quantize first: 8-bit or 16-bit weights, zigzag encoding for signed values. Then compress the bytes if a decoder small enough is written for the bot. The source got its payload under 200 KB this way.

## Pitfalls

- **One source.** That the limit counts characters comes from one player's write-up, for a Python submission. Nobody here has checked it for Rust, or whether CodinGame stores or compiles a file of 285 KB.
- **Editors.** Pasting about 95,000 characters outside ASCII into the CodinGame editor has not been tried here. Paste the file whole, from a UTF-8 copy.
- **Two alphabets.** The decoder and the encoder must leave out the same code points, or every block decodes wrong. Test a round trip in CI.
- **The source's other trick is a different thing.** The write-up's packed payload is a compiled program that its Python bot writes to a file and runs. It reports that CodinGame allows this, and that sending a compiled payload is banned in active contests. That is not this note's method: here the data stays data, read by Rust that CodinGame compiles.

## Before a release uses it

1. **A probe on CodinGame (the owner):** paste a Rust bot holding about 95,000 packed characters into the IDE and play, then submit it to the arena. It must compile and run, unpack its data on the first turn, and be accepted at submission. The bot's standard error reports how many bytes it unpacked and how long it took. Claude builds the file on request.
2. **A decision of the owner and an ADR:** `scripts/cg-check.sh` counts UTF-16 units instead of bytes, still at 100,000. That changes the limit `CLAUDE.md`, `docs/NEW_GAME.md`, the `pr-reviewer` agent and the skills state in bytes.
3. **Tools:** an encoder in the repository (in the trainer or the bundler) and the decoder in a crate that bots share, with a round-trip test.

## Sources

| Source | What was taken from it |
| --- | --- |
| Tom Alard, [Fighting for #1 in the Ultimate Tic-Tac-Toe Arena](https://tomalard.github.io/posts/fighting-for-1-in-the-ultimate-tic-tac-toe-arena/), 2026-09-20, read on 2026-10-14 | That the limit counts characters ("CodinGame has a character limit, not a byte limit"); the packing of 127 bytes into 64 characters; the alphabet U+0100 to U+FFEF without surrogates, U+2028, U+2029, U+FEFF and U+FDD0 to U+FDEF; that CodinGame lets a bot write and run files; that compiled payloads are banned in active contests. It credits Agade's CG Send Binary for the idea |
| This session's test, 2026-10-14 (above) | The text-direction controls rustc rejects, the Rust decoder, the round trip and its times |

## Games that use it

None yet. Ultimate Tic-Tac-Toe's plan needs it for an NNUE ([games/uttt/ROADMAP.md](../../games/uttt/ROADMAP.md#plan-for-iteration), step A).
