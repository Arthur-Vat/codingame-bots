//! Tests of the renaming. Those of [`rename`] run rustc 1.90.0, as the
//! minifier does; the others need no compiler.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    apply, assign, braced_words, kept, rename, Analysis, Case, Report, StdNames, CJK_FIRST,
};
use crate::minify::tokens;
use crate::prune::Rustc;

fn rustc() -> Rustc {
    Rustc::find().expect("these tests need rustc 1.90.0: `rustup toolchain install 1.90.0`")
}

/// The names `source` keeps everywhere.
fn kept_in(source: &str) -> BTreeSet<String> {
    kept(&tokens(source).unwrap(), StdNames::get()).names
}

/// The new name of each name of `source`, with no compiler.
fn mapping(source: &str) -> BTreeMap<String, String> {
    let tokens = tokens(source).unwrap();
    let std = StdNames::get();
    let analysis = Analysis::of(&tokens, std);
    assign(&analysis, &analysis.units(), std).names
}

/// `source` renamed, with no compiler.
fn renamed(source: &str) -> String {
    let tokens = tokens(source).unwrap();
    let std = StdNames::get();
    let analysis = Analysis::of(&tokens, std);
    let edits = assign(&analysis, &analysis.units(), std);
    apply(source, &analysis, &edits).text
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
    let analysis = Analysis::of(&tokens, StdNames::get());
    assert_eq!(analysis.candidates[0], ("often_used".to_string(), 3));
    let mapping = mapping(source);
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
    let std = StdNames::get();
    let mapping = mapping(source);
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
    assert_eq!(report, Report::default());
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

#[test]
fn after_the_letters_come_cjk_ideographs_for_every_case() {
    // Forty long names of each case: the letters run out.
    let mut source = String::new();
    for i in 0..40 {
        source.push_str(&format!(
            "fn long_function_{i}() {{}} struct LongStruct{i}; const LONG_CONST_{i}: u8 = 0;\n"
        ));
    }
    let mapping = mapping(&source);
    assert_eq!(mapping.len(), 120);
    // One UTF-16 unit each, all different.
    let news: BTreeSet<&String> = mapping.values().collect();
    assert_eq!(news.len(), 120);
    assert!(news.iter().all(|new| crate::cg_size(new) == 1), "{news:?}");
    let first = char::from_u32(CJK_FIRST).unwrap().to_string();
    assert!(news.contains(&first), "{news:?}");
    // Upper-case and CamelCase names share the capital letters that std
    // does not use.
    let capitals = news
        .iter()
        .filter(|new| new.chars().all(|c| c.is_ascii_uppercase()))
        .count();
    assert!((10..=26).contains(&capitals), "{capitals}");
}

#[test]
fn cjk_names_compile_without_warnings() {
    let mut source = String::from("fn main() { let mut total = 0u64;\n");
    for i in 0..30 {
        source.push_str(&format!("let value_{i} = {i}u64; total += value_{i};\n"));
    }
    source.push_str("println!(\"{total}\"); }\n");
    let (out, report) = rename(&source, &rustc()).unwrap();
    assert_eq!(report.refused, 0, "{out}");
    assert!(out.contains('\u{4E00}'), "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn the_middle_of_a_std_path_is_kept_only_there() {
    let source = "use std::cell::RefCell;
        use std::collections::{HashMap, BTreeMap};
        fn main() {
            let cell = RefCell::new(1);
            let collections: HashMap<u8, BTreeMap<u8, u8>> = HashMap::new();
            let max = std::cmp::max(*cell.borrow(), 2);
            println!(\"{} {}\", collections.len(), max);
        }";
    let tokens = tokens(source).unwrap();
    let kept = kept(&tokens, StdNames::get());
    assert!(!kept.names.contains("cell") && !kept.names.contains("collections"));
    let positions: Vec<&str> = kept
        .positions
        .iter()
        .map(|&at| tokens[at].text.as_str())
        .collect();
    assert_eq!(positions, ["cell", "collections"]);
    let (out, report) = rename(source, &rustc()).unwrap();
    assert_eq!(report.refused, 0);
    assert!(out.contains("use std::cell::RefCell;"), "{out}");
    assert!(
        out.contains("use std::collections::{HashMap, BTreeMap};"),
        "{out}"
    );
    assert!(
        !out.contains("let cell") && !out.contains("let collections"),
        "{out}"
    );
    crate::prune::check(&out, &rustc()).unwrap();

    // A use tree that imports its segment with `self` keeps it everywhere.
    assert_kept("use std::io::{self, Read};", &["io", "Read"], &[]);
}

#[test]
fn only_a_paths_first_name_leads_into_std() {
    // `crate::board::Ordering` is the bundle's own path: its `LESSER` is
    // renamed, though `Ordering` leads into std where it comes first.
    let source = "use std::cmp::Ordering;
        mod board { pub mod Ordering { pub const LESSER: i32 = 1; } }
        fn main() { let _ = (Ordering::Less, crate::board::Ordering::LESSER); }";
    assert_kept(source, &["Ordering", "Less"], &["LESSER", "board"]);
}

#[test]
fn self_gets_a_short_name_in_methods_that_use_it_often() {
    let source = "struct Counter { hits: u64, misses: u64 }
        impl Counter {
            fn total(&self) -> u64 { self.hits + self.misses + self.hits * self.misses }
            fn bump(&mut self) { self.hits += 1; self.misses += 1; self.hits += self.misses; }
            fn into_total(mut self) -> u64 { self.hits += 1; self.misses += 1; self.hits += 1; self.misses += 1; self.hits + self.misses }
            fn once(&self) -> u64 { self.hits }
            fn nested(&self) -> u64 { fn inner() -> u64 { 1 } self.hits + self.hits + self.hits + self.hits + inner() }
            fn path(&self) -> u64 { self::helper() + self.hits + self.hits + self.hits + self.hits }
        }
        fn helper() -> u64 { 2 }
        fn main() {
            let mut counter = Counter { hits: 1, misses: 2 };
            counter.bump();
            println!(\"{} {} {} {}\", counter.total(), counter.once(), counter.nested(), counter.path());
            println!(\"{}\", counter.into_total());
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert_eq!(report.refused, 0, "{out}");
    let name = report.self_name.clone().expect("a name for self");
    assert_eq!(report.methods, 4, "{out}");
    assert!(out.contains(&format!("{{ let {name}=self;")), "{out}");
    assert!(out.contains(&format!("{{ let mut {name}=self;")), "{out}");
    // Too few uses, or a nested `fn`, keep `self`; so does a `self::` path.
    assert!(out.contains("{ self."), "{out}");
    assert!(out.contains("self::"), "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn types_get_aliases_where_they_save_room() {
    let source = "#[derive(Clone, Debug)]
        struct Grid { cells: Vec<usize>, rows: Vec<usize>, width: usize, height: usize }
        mod small { pub fn one(x: usize) -> usize { x } }
        fn area(grid: &Grid) -> usize { grid.width as usize * grid.height as usize + usize::MAX % 7 }
        fn biggest(values: &[usize]) -> Option<usize> { values.iter().copied().max() }
        fn main() {
            let grid = Grid { cells: Vec::new(), rows: Vec::<usize>::with_capacity(2), width: 3, height: 1_000 };
            let start = std::time::Instant::now();
            println!(\"{} {:?} {} {} {}\", area(&grid), biggest(&[1, 2]), small::one(2), grid.cells.len() + grid.rows.len(), start.elapsed().as_nanos() > 0);
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert_eq!(report.refused, 0, "{out}");
    let usize_alias = report
        .aliases
        .iter()
        .find(|(ty, _, _)| ty == "usize")
        .unwrap_or_else(|| panic!("{report:?}"));
    // Declared at the root only: `small` uses `usize` too rarely.
    assert_eq!(usize_alias.2, 1);
    assert!(
        out.contains(&format!("type {}=usize;", usize_alias.1)),
        "{out}"
    );
    assert!(out.contains("(x: usize) -> usize { x }"), "{out}");
    // Paths and attributes keep their names; numbers lose their separators.
    assert!(out.contains("std::time::Instant::now()"), "{out}");
    assert!(out.contains("#[derive(Clone, Debug)]"), "{out}");
    assert!(out.contains("1000") && !out.contains("1_000"), "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn an_alias_that_does_not_compile_is_undone() {
    // `Instant` is imported in `main` only: the root's alias cannot see it.
    let source = "fn main() {
            use std::time::Instant;
            let (a, b, c, d) = (Instant::now(), Instant::now(), Instant::now(), Instant::now());
            println!(\"{:?}\", (d - a) + (c - b));
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert!(report.refused >= 1, "{report:?}");
    assert!(report.aliases.is_empty(), "{report:?}");
    assert!(!out.contains("type "), "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn self_read_by_a_format_string_keeps_its_name() {
    // Renamed, `self.x += 1` would change a copy that the string does not
    // read: the program would print the old value without an error.
    let source = "#[derive(Clone, Copy, Debug)]
        struct Pos { x: i32, y: i32 }
        impl Pos {
            fn shifted(mut self) -> String { self.x += 1; self.y += 1; self.x += 3; self.y += 5; self.x *= 2; self.y *= 2; self.x -= 1; self.y -= 1; format!(\"{self:?}\") }
        }
        fn main() { println!(\"{}\", Pos { x: 1, y: 2 }.shifted()); }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert_eq!(report.methods, 0, "{out}");
    assert!(!out.contains("=self;"), "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}

#[test]
fn the_arguments_of_the_bundles_macros_keep_types_and_self() {
    // A rule of the macro matches the word itself: `width!(C)` would take
    // the other rule without an error.
    let source = "macro_rules! width { (usize) => { 8 }; ($t:ty) => { 64 }; }
        macro_rules! who { (self) => { 1 }; ($e:expr) => { 2 }; }
        struct Unit;
        impl Unit {
            fn pick(&self) -> u32 { let _ = (self, self, self, self, self); who!(self) }
        }
        fn main() {
            let sizes: [usize; 4] = [width!(usize), 1usize, 2usize, 3usize];
            let more: (usize, usize, usize, usize) = (1, 2, 3, 4);
            println!(\"{:?} {:?} {}\", sizes, more, Unit.pick());
        }";
    let (out, report) = rename(source, &rustc()).unwrap();
    assert!(out.contains("!(usize)"), "{out}");
    assert!(out.contains("!(self)"), "{out}");
    assert_eq!(report.methods, 0, "{out}");
    crate::prune::check(&out, &rustc()).unwrap();
}
