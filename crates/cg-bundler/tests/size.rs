//! The size CodinGame counts: UTF-16 units (ADR 0037).

#[test]
fn counts_utf16_units() {
    assert_eq!(cg_bundler::cg_size(""), 0);
    assert_eq!(cg_bundler::cg_size("fn main() {}"), 12);
    // One unit for U+4E00 (3 bytes in UTF-8), two for U+1F600 (4 bytes).
    assert_eq!(cg_bundler::cg_size("a\u{4E00}\u{1F600}"), 4);
}
