//! Tests of the flattening. Those of [`flatten`] run rustc 1.90.0, as the
//! minifier does; the others need no compiler.

use std::collections::BTreeSet;

use super::{flatten, render, std_imports, Plan};
use crate::minify::{tokens, Kind};
use crate::prune::Rustc;

fn rustc() -> Rustc {
    Rustc::find().expect("these tests need rustc 1.90.0: `rustup toolchain install 1.90.0`")
}

/// `source` flattened, with no compiler.
fn flattened(source: &str) -> String {
    let tokens = tokens(source).unwrap();
    let plan = Plan::of(&tokens).unwrap();
    render(source, &tokens, &plan.flat_edits()).unwrap().text
}

/// The new names of `source`'s names, as `old -> new`.
fn new_names(source: &str) -> BTreeSet<String> {
    let tokens = tokens(source).unwrap();
    let plan = Plan::of(&tokens).unwrap();
    let mut names = BTreeSet::new();
    for (key, new) in &plan.flat {
        let (super::Key::Item(_, old) | super::Key::Std(_, old)) = key;
        if new != old {
            names.insert(format!("{old} -> {new}"));
        }
    }
    names
}

/// The words of `source`'s tokens.
fn words(source: &str) -> Vec<String> {
    tokens(source)
        .unwrap()
        .into_iter()
        .filter(|t| t.kind == Kind::Ident)
        .map(|t| t.text)
        .collect()
}

/// Whether the tokens of `piece` are in those of `text`, one after the other.
fn has(text: &str, piece: &str) -> bool {
    let all: Vec<String> = tokens(text).unwrap().into_iter().map(|t| t.text).collect();
    let piece: Vec<String> = tokens(piece).unwrap().into_iter().map(|t| t.text).collect();
    all.windows(piece.len())
        .any(|window| window == piece.as_slice())
}

/// What `source`, a program, prints, compiled with rustc 1.90.0.
fn output_of(source: &str, name: &str) -> String {
    let dir = std::env::temp_dir().join(format!("cg-flatten-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("main.rs");
    std::fs::write(&file, source).unwrap();
    let binary = dir.join("main");
    let compiled = rustc()
        .command()
        .args(["--edition", "2021", "-A", "warnings", "-o"])
        .arg(&binary)
        .arg(&file)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}\n{source}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let run = std::process::Command::new(&binary).output().unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(run.status.success());
    String::from_utf8(run.stdout).unwrap()
}

/// Flattens `source` with rustc and checks that it was flattened and prints
/// what `source` prints.
fn assert_flattens(source: &str, name: &str) -> String {
    let (text, report) = flatten(source, &rustc()).unwrap();
    assert_eq!(report.skipped, None, "{text}");
    assert_eq!(
        output_of(&text, name),
        output_of(source, &format!("{name}-before"))
    );
    text
}

const LIBRARY: &str = r#"
use std::fmt::Write as _;

fn main() {
    let mut out = String::new();
    // A comment.
    let b = crate::lib::board::Board::new();
    write!(out, "{}", b.score()).unwrap();
    println!("{out} {}", crate::lib::LIMIT);
}

#[allow(dead_code)]
mod lib {
    //! A library.
    pub mod board {
        use std::io;

        /// A board.
        pub struct Board {
            pub cells: Vec<u8>,
        }

        impl Board {
            pub fn new() -> Board {
                Board { cells: vec![1, 2] }
            }
            pub fn score(&self) -> usize {
                self.cells.len() + super::LIMIT
            }
        }

        pub(crate) fn nothing() -> io::Result<()> {
            Ok(())
        }
    }
    pub use board::Board;
    pub(crate) const LIMIT: usize = 3;
}
"#;

#[test]
fn modules_uses_paths_visibilities_and_comments_go() {
    let text = flattened(LIBRARY);
    let words = words(&text);
    for gone in ["mod", "pub", "crate", "super", "lib", "board"] {
        assert!(
            !words.iter().any(|w| w == gone),
            "`{gone}` is left in:\n{text}"
        );
    }
    assert!(!text.contains("//"), "{text}");
    assert!(
        text.starts_with("use std::io;\nuse std::fmt::Write as _;\n"),
        "{text}"
    );
    assert!(has(&text, "Board::new()"), "{text}");
    assert!(has(&text, "self.cells.len() + LIMIT"), "{text}");
    assert!(has(&text, "fn nothing() -> io::Result<()>"), "{text}");
    assert_eq!(new_names(LIBRARY), BTreeSet::new());
    assert_flattens(LIBRARY, "library");
}

const CLASH: &str = r#"
fn main() {
    let a = crate::x::play(1);
    let b = crate::y::play(2);
    let mut game = crate::y::Game { play: 3 };
    game.play();
    println!("{a} {b} {} {}", game.play, crate::y::twice(4));
}

mod x {
    pub fn play(n: u8) -> u8 {
        n + 10
    }
}

mod y {
    pub fn play(n: u8) -> u8 {
        n * 2
    }

    pub struct Game {
        pub play: u8,
    }

    impl Game {
        pub fn play(&mut self) {
            self.play += play(1);
        }
    }

    pub fn twice(play: u8) -> u8 {
        play + play
    }
}
"#;

#[test]
fn the_second_of_two_names_gets_a_new_name_where_it_is_bound() {
    assert_eq!(
        new_names(CLASH),
        BTreeSet::from(["play -> play1".to_string()])
    );
    let text = flattened(CLASH);
    // The function, its call and the local of the same name; not the field,
    // the method or the field of the struct literal.
    assert!(has(&text, "fn play1(n: u8)"), "{text}");
    assert!(has(&text, "self.play += play1(1)"), "{text}");
    assert!(
        has(&text, "fn twice(play1: u8) -> u8 { play1 + play1"),
        "{text}"
    );
    assert!(has(&text, "struct Game { play: u8"), "{text}");
    assert!(has(&text, "fn play(&mut self)"), "{text}");
    assert!(has(&text, "Game { play: 3 }"), "{text}");
    assert!(has(&text, "let b = play1(2);"), "{text}");
    assert!(has(&text, "let a = play(1);"), "{text}");
}

#[test]
fn a_program_with_clashing_names_prints_the_same() {
    assert_flattens(CLASH, "clash");
}

const PATTERN: &str = r#"
fn main() {
    println!("{} {} {}", crate::b::kind(1), crate::b::kind(7), crate::a::N);
}

mod a {
    pub const N: u8 = 7;
}

mod b {
    #[allow(non_snake_case, unreachable_patterns)]
    pub fn kind(x: u8) -> u8 {
        match x {
            N => 0,
            _ => 1,
        }
    }
}
"#;

#[test]
fn a_constant_that_a_pattern_could_take_gets_a_new_name() {
    // At the root, `N =>` would match the constant only.
    assert_eq!(new_names(PATTERN), BTreeSet::from(["N -> N1".to_string()]));
    let text = assert_flattens(PATTERN, "pattern");
    assert!(has(&text, "N => 0"), "{text}");
    assert!(has(&text, "const N1: u8 = 7;"), "{text}");
}

const TWO_WRITES: &str = r#"
use std::fmt::Write;

fn main() {
    let mut s = String::new();
    write!(s, "a").unwrap();
    crate::m::out(&s);
}

mod m {
    use std::io::Write;

    pub fn out(s: &str) {
        let mut o = std::io::stdout();
        write!(o, "{s}").unwrap();
        o.flush().unwrap();
    }
}
"#;

#[test]
fn std_imports_that_clash_are_renamed_with_as() {
    assert_eq!(
        new_names(TWO_WRITES),
        BTreeSet::from(["Write -> Write1".to_string()])
    );
    let text = assert_flattens(TWO_WRITES, "writes");
    assert!(
        text.starts_with("use std::fmt::Write;\nuse std::io::Write as Write1;\n"),
        "{text}"
    );
}

const MODULE_AND_FN: &str = r#"
fn main() {
    println!("{}", crate::lib::plan(2));
}

mod lib {
    mod plan {
        pub fn plan(n: u8) -> u8 {
            n + 1
        }
    }

    pub fn plan(n: u8) -> u8 {
        plan::plan(n) * 3
    }
}
"#;

#[test]
fn a_module_and_a_function_can_share_a_name() {
    assert_eq!(
        new_names(MODULE_AND_FN),
        BTreeSet::from(["plan -> plan1".to_string()])
    );
    let text = assert_flattens(MODULE_AND_FN, "module-and-fn");
    assert!(has(&text, "plan(n) * 3"), "{text}");
    assert!(has(&text, "fn plan1(n: u8)"), "{text}");
}

/// In `c`, `Foo.val()` reaches the inherent method, since `T` is not in
/// scope there; at the root it would reach `T::val`, which takes `self` by
/// value and comes first in the method lookup.
const TRAIT_IN_SCOPE: &str = r#"
fn main() {
    println!("{}", crate::c::value());
}

mod a {
    pub struct Foo;

    impl Foo {
        pub fn val(&self) -> u8 {
            1
        }
    }
}

mod t {
    pub trait T {
        fn val(self) -> u8;
    }

    impl T for crate::a::Foo {
        fn val(self) -> u8 {
            2
        }
    }

    pub fn by_trait() -> u8 {
        crate::a::Foo.val()
    }
}

mod c {
    pub fn value() -> u8 {
        crate::a::Foo.val() + crate::t::by_trait() * 10
    }
}
"#;

#[test]
fn the_check_refuses_a_call_that_a_trait_would_take() {
    // Flattened without the check, the program prints something else.
    assert_eq!(output_of(TRAIT_IN_SCOPE, "trait-before"), "21\n");
    assert_eq!(output_of(&flattened(TRAIT_IN_SCOPE), "trait-after"), "22\n");
    let (text, report) = flatten(TRAIT_IN_SCOPE, &rustc()).unwrap();
    assert_eq!(text, TRAIT_IN_SCOPE);
    let why = report.skipped.unwrap();
    assert!(
        why.contains("method `Foo::val`") && why.contains("method `T::val`"),
        "{why}"
    );
}

#[test]
fn forms_the_flattener_does_not_read_leave_the_bundle_as_it_is() {
    for (source, why) in [
        (
            "mod a { pub fn f() {} }\nmod b { use crate::a::*; }\nfn main() {}",
            "glob",
        ),
        (
            "mod a { macro_rules! m { () => {} } }\nfn main() {}",
            "macro_rules",
        ),
        ("mod a;\nfn main() {}", "without a body"),
        (
            "mod a { pub fn f() {} }\nmod b { use crate::a::f as g; }\nfn main() {}",
            "`as`",
        ),
        ("#[cfg(test)]\nmod a {}\nfn main() {}", "other than `allow`"),
        (
            "mod a { fn f() { use crate::b::g; } }\nmod b { pub fn g() {} }\nfn main() {}",
            "function body",
        ),
    ] {
        let tokens = tokens(source).unwrap();
        let err = Plan::of(&tokens)
            .err()
            .unwrap_or_else(|| panic!("read: {source}"));
        assert!(err.contains(why), "{err} (for {source})");
    }
}

#[test]
fn std_imports_are_grouped_by_their_parent() {
    let imports = [
        ("std::io", "io"),
        ("std::io::BufRead", "BufRead"),
        ("std::io::Write", "_"),
        ("std::fmt::Write", "Write"),
        ("std::fmt::Write", "_"),
        ("std::time::Instant", "Clock"),
    ];
    let imports = imports
        .iter()
        .map(|(path, name)| (path.to_string(), name.to_string()))
        .collect();
    assert_eq!(
        std_imports(&imports).unwrap(),
        "use std::fmt::Write;\nuse std::io::{self,BufRead,Write as _};\nuse std::time::Instant as Clock;"
    );
}
