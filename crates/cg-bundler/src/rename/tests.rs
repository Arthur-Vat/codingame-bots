//! Tests of the renaming. Those of [`rename`] run rustc 1.90.0, as level 3
//! does; the others need no compiler.

use std::collections::BTreeSet;

use super::{apply, assign, braced_words, candidates, kept, rename, Case, Report, StdNames};
use crate::minify::tokens;
use crate::prune::Rustc;

fn rustc() -> Rustc {
    Rustc::find().expect("these tests need rustc 1.90.0: `rustup toolchain install 1.90.0`")
}

/// The names `source` keeps.
fn kept_in(source: &str) -> BTreeSet<String> {
    kept(&tokens(source).unwrap(), StdNames::get())
}

/// `source` renamed, with no compiler.
fn renamed(source: &str) -> String {
    let tokens = tokens(source).unwrap();
    let std = StdNames::get();
    let mapping = assign(&tokens, &candidates(&tokens, std), std);
    apply(source, &tokens, &mapping)
}

/// Asserts that `source` keeps each of `names` and renames each of
/// `renamed_names`.
fn assert_kept(source: &str, names: &[&str], renamed_names: &[&str]) {
    let kept = kept_in(source);
    for name in names {
        assert!(kept.contains(*name), "{name} should be kept in {source}");
    }
    for name in renamed_names {
        assert!(
            !kept.contains(*name),
            "{name} should be renamed in {source}"
        );
    }
}

#[test]
fn new_names_are_shortest_first_and_keep_the_case() {
    assert_eq!(Case::Lower.name(0), "a");
    assert_eq!(Case::Lower.name(25), "z");
    assert_eq!(Case::Lower.name(26), "aa");
    assert_eq!(Case::Lower.name(26 + 35), "a9");
    assert_eq!(Case::Lower.name(26 + 36), "ba");
    assert_eq!(Case::Lower.name(26 + 26 * 36), "aaa");
    assert_eq!(Case::Camel.name(0), "A");
    assert_eq!(Case::Camel.name(26), "Aa");
    assert_eq!(Case::Upper.name(26), "AA");
    assert_eq!(Case::Upper.name(26 + 26), "A0");

    assert_eq!(Case::of("score"), Case::Lower);
    assert_eq!(Case::of("legal_moves"), Case::Lower);
    assert_eq!(Case::of("Board"), Case::Camel);
    assert_eq!(Case::of("MAX_DEPTH"), Case::Upper);
    assert_eq!(Case::of("T"), Case::Upper);
}

#[test]
fn the_std_names_are_read_from_the_list() {
    let std = StdNames::get();
    assert!(std.member("len") && std.member("checked_add") && std.member("fmt"));
    assert!(std.member("MAX") && std.member("Item") && std.member("start"));
    assert!(std.has("HashMap") && !std.member("HashMap"));
    assert!(std.prelude("Vec") && std.prelude("Some") && std.prelude("drop"));
    assert!(std.prelude("size_of") && !std.prelude("HashMap"));
    // Private names of std are not members: no code outside std reaches them.
    assert!(!std.has("score") && !std.has("legal_moves"));

    // The list is sorted, without repeats.
    let names: Vec<&str> = super::STD_NAMES
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.split_once(' ').unwrap().1)
        .collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(names, sorted);

    assert!(StdNames::parse("m len\nx bad\n").is_err());
    assert!(StdNames::parse("m len\nm len\n").is_err());
    assert!(StdNames::parse("mlen\n").is_err());
    assert!(StdNames::parse("# a comment\nim Item\n").is_ok());
}

#[test]
fn std_members_are_kept_where_std_may_be_meant() {
    let source = "struct Board { cells: Vec<u8> }
        impl Board {
            fn len(&self) -> usize { self.cells.len() }
            fn evaluate(&self) -> usize { self.len() }
        }
        impl std::fmt::Display for Board {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, \"\") }
        }
        fn main() { let x = u32::MAX; let it: Box<dyn Iterator<Item = u8>> = Box::new(0..1); }";
    assert_kept(
        source,
        &[
            "len",
            "fmt",
            "MAX",
            "Item",
            "Formatter",
            "Display",
            "Result",
            "write",
            "main",
        ],
        &["Board", "cells", "evaluate", "it"],
    );
}

#[test]
fn std_paths_use_trees_and_the_prelude_are_kept() {
    let source = "use std::collections::HashMap;
        use std::io::{self, BufRead};
        use std::cmp::Ordering;
        fn pick(a: i32, b: i32) -> bool {
            let stdin = io::stdin();
            let map: HashMap<i32, i32> = HashMap::with_capacity(4);
            let lines = stdin.lock().lines();
            a.cmp(&b) == Ordering::Less && Option::<i32>::None.is_none()
        }";
    assert_kept(
        source,
        &[
            "std",
            "collections",
            "HashMap",
            "io",
            "BufRead",
            "stdin",
            "cmp",
            "Ordering",
            "Less",
            "lock",
            "lines",
            "with_capacity",
            "Option",
            "None",
            "is_none",
            "i32",
        ],
        &["pick", "map", "a", "b"],
    );
}

#[test]
fn a_kept_name_that_std_cannot_reach_does_not_keep_the_names_behind_it() {
    // `board` is kept by the format string, but `board::Board` is the
    // bundle's own path.
    let source = "mod board { pub struct Board; }
        fn show(board: u8) { let piece = board::Board; eprintln!(\"{board}\"); }";
    assert_kept(source, &["board", "eprintln"], &["Board", "show", "piece"]);
}

#[test]
fn attributes_formats_and_text_macros_keep_their_names() {
    let source = "#![allow(dead_code)]
        #[derive(Clone, Debug)]
        #[inline(always)]
        fn go(width: usize, score: i64, other: i64) -> String {
            let label = stringify!(alpha);
            format!(\"{score:>width$} {0} {{literal}}\", other)
        }";
    assert_kept(
        source,
        &[
            "allow",
            "dead_code",
            "derive",
            "Clone",
            "Debug",
            "inline",
            "always",
            "score",
            "width",
        ],
        &["go", "other", "label"],
    );
    assert!(kept_in(source).contains("alpha"));
    assert!(kept_in(source).contains("literal"));
    assert_eq!(braced_words("\"{a:>w$} {} {0:?} {{b}}\""), ["a", "w", "b"]);
}

#[test]
fn keywords_macros_and_special_names_are_kept() {
    let source = "macro_rules! twice { ($value:expr) => { $value * 2 } }
        fn main() {
            let _unused = 1;
            let r#type = twice!(3);
            let numbers = vec![r#type];
            println!(\"{}\", numbers.len());
        }";
    assert_kept(
        source,
        &[
            "macro_rules",
            "expr",
            "main",
            "_unused",
            "r#type",
            "vec",
            "println",
            "let",
            "fn",
        ],
        // The bundle's own macro and its variables get new names.
        &["twice", "value", "numbers"],
    );
}

#[test]
fn the_most_frequent_names_get_the_shortest_new_names() {
    let source = "fn rarely_used() {}
        fn often_used() { often_used(); often_used(); rarely_used(); }
        const MAX_DEPTH: usize = 3;
        struct BoardState;
        fn i() {}";
    let tokens = tokens(source).unwrap();
    let std = StdNames::get();
    let names = candidates(&tokens, std);
    assert_eq!(names[0], "often_used");
    let mapping = assign(&tokens, &names, std);
    assert_eq!(mapping["often_used"], "a");
    assert_eq!(mapping["rarely_used"], "b");
    // Upper-case names get upper-case letters (some, like `A`, are std
    // names).
    let (depth, board) = (&mapping["MAX_DEPTH"], &mapping["BoardState"]);
    assert!(
        depth.len() == 1 && depth.chars().all(|c| c.is_ascii_uppercase()),
        "{depth}"
    );
    assert!(
        board.len() == 1 && board.chars().all(|c| c.is_ascii_uppercase()),
        "{board}"
    );
    assert_ne!(depth, board);
    // A name is renamed only to a shorter one.
    assert!(!mapping.contains_key("i"));
}

#[test]
fn new_names_are_not_used_in_the_bundle_or_by_std() {
    // `a` and `b` are taken by the bundle; `c` and `e` are std names.
    let source = "fn a() {} fn b() {} fn long_name() { long_name(); }";
    let tokens = tokens(source).unwrap();
    let std = StdNames::get();
    let mapping = assign(&tokens, &candidates(&tokens, std), std);
    let new = &mapping["long_name"];
    assert!(new != "a" && new != "b" && !std.has(new), "{new}");
    assert_eq!(new.len(), 1);
}

#[test]
fn only_names_change_not_literals_or_comments() {
    let source = "// long_name in a comment\nfn long_name() -> &'static str { \"long_name\" }\n";
    let out = renamed(source);
    assert!(
        out.starts_with("// long_name in a comment\nfn a() -> &'static str"),
        "{out}"
    );
    assert!(out.contains("\"long_name\""), "{out}");
}

#[test]
fn a_renamed_program_compiles_and_is_shorter() {
    let source = "struct GameState { player_score: i64, opponent_score: i64 }
        impl GameState {
            fn difference(&self) -> i64 { self.player_score - self.opponent_score }
        }
        fn main() {
            let state = GameState { player_score: 3, opponent_score: 1 };
            println!(\"{}\", state.difference());
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert!(out.len() < source.len(), "{out}");
    assert_eq!(report.refused, 0);
    assert_eq!(report.compiles, 1);
    assert!(report.renamed >= 4, "{report:?}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn a_renaming_that_does_not_compile_is_undone() {
    // `start` and `end` are fields of std's `Range`, here only in a struct
    // literal, where the rules do not see std: rustc rejects their renaming.
    let source = "struct Span { start: usize, width: usize }
        fn main() {
            let range = std::ops::Range { start: 1, end: 4 };
            let Span { start, width } = Span { start: 2, width: range.len() };
            println!(\"{}\", start + width);
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert!(out.contains("Range { start: 1, end: 4 }"), "{out}");
    assert!(report.refused >= 1, "{report:?}");
    assert!(report.renamed >= 2, "{report:?}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn nothing_to_rename_leaves_the_text() {
    let source = "fn main() { let x = 1; println!(\"{x}\"); }\n";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert_eq!(out, source);
    assert_eq!(
        report,
        Report {
            renamed: 0,
            refused: 0,
            compiles: 0
        }
    );
}

#[test]
fn names_a_std_glob_import_may_bring_are_kept() {
    // Renamed, the variants would become bindings that match anything.
    let source = "use std::cmp::Ordering::*;
        fn compare_numbers(a: i32, b: i32) -> i32 {
            match a.cmp(&b) { Less => -1, Equal => 0, Greater => 1 }
        }
        fn main() { println!(\"{}\", compare_numbers(2, 1)); }";
    assert_kept(source, &["Less", "Equal", "Greater"], &["compare_numbers"]);
    let (out, _) = rename(source, &rustc()).unwrap();
    assert!(
        out.contains("Less => -1, Equal => 0, Greater => 1"),
        "{out}"
    );

    // Through a name that leads into std, and in a use tree.
    let source = "use std::cmp;
        use cmp::Ordering::*;
        use std::{io::*, num::FpCategory};
        fn main() { let _ = (Less, stdin(), FpCategory::Nan); }";
    assert_kept(source, &["Less", "stdin", "Nan"], &[]);
    // A glob of the bundle's own enum keeps nothing.
    let source = "mod shapes { pub enum Shape { Round, Square } }
        use shapes::Shape::*;
        fn main() { let _ = (Round, Square); }";
    assert_kept(source, &[], &["Round", "Square", "Shape", "shapes"]);
}

#[test]
fn names_that_a_macro_of_the_bundle_turns_into_text_are_kept() {
    let source = "macro_rules! show { ($n:ident) => { println!(\"{}\", stringify!($n)) } }
        macro_rules! again { ($n:ident) => { show!($n) } }
        fn main() { let player_score = 1; show!(player_score); again!(opponent_score); }";
    assert_kept(
        source,
        &["player_score", "opponent_score"],
        &["show", "again"],
    );
}

#[test]
fn std_enum_variants_are_listed() {
    let std = StdNames::get();
    for name in ["Less", "Equal", "Greater", "Nan", "V4", "Relaxed"] {
        assert!(std.has(name), "{name}");
    }
}
