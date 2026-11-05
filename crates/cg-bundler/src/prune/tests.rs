//! Tests of the pruner. Most run rustc 1.90.0, as the pruner does.

use std::collections::BTreeSet;

use super::items::{Id, Source};
use super::{check, prune, Report, Rustc, ENV, VERSION};

fn rustc() -> Rustc {
    Rustc::find().expect("these tests need rustc 1.90.0: `rustup toolchain install 1.90.0`")
}

/// `source` pruned, and what was done.
fn pruned(source: &str) -> (String, Report) {
    prune(source, &rustc()).unwrap_or_else(|err| panic!("{err}\n{source}"))
}

/// The text without whitespace, to compare code whatever its layout.
fn squashed(text: &str) -> String {
    text.split_whitespace().collect()
}

/// The text with the item named `name` (the first token of that text)
/// removed, with no compiler.
fn without(text: &str, name: &str) -> String {
    let source = Source::new(text).unwrap();
    let start = word(text, name);
    let id = source
        .item_at((start, start + name.len()))
        .unwrap_or_else(|| panic!("{name} is not an item in {text}"));
    source.render(&BTreeSet::from([id])).text
}

/// Where `name` is in `text` as a whole identifier.
fn word(text: &str, name: &str) -> usize {
    let is_ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    text.match_indices(name)
        .map(|(at, _)| at)
        .find(|&at| {
            !is_ident(text[..at].chars().next_back())
                && !is_ident(text[at + name.len()..].chars().next())
        })
        .unwrap_or_else(|| panic!("no {name} in {text}"))
}

#[test]
fn an_item_goes_with_its_attributes_and_doc_comments() {
    let text = "fn a() {}\n\n/// Doc.\n#[inline]\n// A comment.\npub(crate) const unsafe fn b() { { } }\nfn c() {}\n";
    assert_eq!(without(text, "b"), "fn a() {}\nfn c() {}\n");
    let text = "//! Crate doc.\n/// Doc.\n#[derive(Clone)]\n#[allow(dead_code)]\npub struct S<T>(T) where T: Copy;\nfn main() {}\n";
    assert_eq!(without(text, "S"), "\nfn main() {}\n");
    let text = "fn a() {}\n#[cfg(x)]\npub static mut COUNTER: [u8; 2] = [1, 2];\n";
    assert_eq!(without(text, "COUNTER"), "fn a() {}\n");
}

#[test]
fn the_extent_of_each_kind_of_item_is_found() {
    let cases = [
        (
            "fn f<T: Fn() -> u8>(x: [u8; { 3 }]) -> u8 where T: Copy { { 1 } }",
            "f",
        ),
        ("fn g(&self);", "g"),
        ("struct U;", "U"),
        ("struct T(u8, (u8, u8));", "T"),
        ("struct B<T> where T: Copy { a: T, b: [u8; 2] }", "B"),
        ("enum E { A = 1, B { x: u8 }, C(u8) }", "E"),
        ("union Un { a: u8, b: i8 }", "Un"),
        (
            "trait Tr { fn f(&self) -> u8 { { 0 } } const C: u8; type X; }",
            "Tr",
        ),
        ("type Alias<T> = Vec<[T; { 2 }]>;", "Alias"),
        ("const C: fn() -> u8 = || { 1; 2 };", "C"),
        ("static S: &str = \"fn x() {};\";", "S"),
        ("pub(in crate::a) async unsafe extern \"C\" fn h() {}", "h"),
    ];
    for (item, name) in cases {
        let text = format!("fn before() {{}}\n{item}\nfn after() {{}}\n");
        assert_eq!(
            without(&text, name),
            "fn before() {}\nfn after() {}\n",
            "{item}"
        );
    }
}

#[test]
fn what_is_not_an_item_with_a_clear_extent_is_not_offered() {
    // An enum with a `repr` does not offer its variants.
    let text = "#[repr(u8)]\nenum E { A, B }\nstruct S { field: u8 }\nfn main() { let x = 1; }\n";
    let source = Source::new(text).unwrap();
    for name in ["A", "B", "field", "x", "main"] {
        let start = word(text, name);
        // `main` is found as the name of `fn main`; the others are not
        // items.
        assert!(
            source.item_at((start, start + name.len())).is_none(),
            "{name}"
        );
    }
    // A span that is not exactly one token is not offered either.
    assert!(source.item_at((3, 5)).is_none());
}

#[test]
fn names_of_a_use_are_removed_one_by_one() {
    let text = "use std::fmt::{self, Debug as D, Display};\nuse a::b;\n";
    let source = Source::new(text).unwrap();
    let leaf = |name: &str| {
        let start = word(text, name);
        source.leaves_at((start, start + name.len()))
    };
    let render = |ids: &[Id]| source.render(&ids.iter().copied().collect()).text;
    assert_eq!(
        render(&leaf("Debug as D")),
        "use std::fmt::{self,Display};\nuse a::b;\n"
    );
    assert_eq!(
        render(&leaf("self")),
        "use std::fmt::{Debug as D,Display};\nuse a::b;\n"
    );
    let mut both = leaf("self");
    both.extend(leaf("Debug as D"));
    both.extend(leaf("Display"));
    assert_eq!(render(&both), "\nuse a::b;\n");
    assert_eq!(
        render(&leaf("a::b")),
        "use std::fmt::{self, Debug as D, Display};\n"
    );
    // The whole group is one span.
    let start = text.find("{self").unwrap();
    let group = source.leaves_at((start, text.find("};").unwrap() + 1));
    assert_eq!(group.len(), 3);
}

#[test]
fn a_rendering_maps_its_bytes_back_to_the_original() {
    let text = "fn a() {}\nfn dead() {}\nfn live() {}\n";
    let source = Source::new(text).unwrap();
    let start = word(text, "dead");
    let id = source.item_at((start, start + 4)).unwrap();
    let rendered = source.render(&BTreeSet::from([id]));
    assert_eq!(rendered.text, "fn a() {}\nfn live() {}\n");
    let at = word(&rendered.text, "live");
    let original = rendered.to_original((at, at + 4)).unwrap();
    assert_eq!(&text[original.0..original.1], "live");
}

#[test]
fn dead_items_go_and_live_ones_stay() {
    let source = r#"use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::{self, Read, Write};

/// A doc comment on a dead function.
#[inline]
fn dead_free() -> u8 {
    dead_helper() + 1
}
fn dead_helper() -> u8 {
    2
}
const DEAD_CONST: u8 = 3;
static DEAD_STATIC: u8 = 4;
type DeadAlias = u8;
struct DeadStruct {
    a: u8,
}
impl DeadStruct {
    fn new() -> Self {
        DeadStruct { a: 0 }
    }
}
enum DeadEnum {
    A,
    B,
}
trait DeadTrait {
    fn t(&self);
}
struct Live {
    x: u8,
}
impl Live {
    fn live_method(&self) -> u8 {
        self.x
    }
    fn dead_method(&self) -> u8 {
        9
    }
}
#[allow(dead_code, unused_imports)]
mod nested {
    use std::cmp::Ordering;
    pub fn dead_in_mod() {}
    pub fn live_in_mod() -> u8 {
        5
    }
    pub mod deeper {
        pub fn dead_deeper() {}
    }
}
#[allow(dead_code)]
mod all_dead {
    pub fn a() {}
    pub struct B;
    impl B {
        pub fn c(&self) {}
    }
}
fn main() {
    let live = Live { x: 1 };
    let mut out = io::stdout();
    writeln!(out, "{} {}", live.live_method(), nested::live_in_mod()).unwrap();
}
"#;
    let (out, report) = pruned(source);
    for gone in [
        "HashMap",
        "Debug",
        "Display",
        "Read",
        "Ordering",
        "dead_free",
        "dead_helper",
        "DEAD_CONST",
        "DEAD_STATIC",
        "DeadAlias",
        "DeadStruct",
        "DeadEnum",
        "DeadTrait",
        "dead_method",
        "dead_in_mod",
        "deeper",
        "all_dead",
        "doc comment",
    ] {
        assert!(!out.contains(gone), "{gone} is still in\n{out}");
    }
    for kept in ["Live", "live_method", "nested", "live_in_mod", "fn main"] {
        assert!(out.contains(kept), "{kept} is gone from\n{out}");
    }
    assert!(
        squashed(&out).contains("usestd::io::{self,Write};"),
        "{out}"
    );
    assert!(
        squashed(&out).contains("#[allow(dead_code,unused_imports)]modnested{"),
        "{out}"
    );
    assert!(report.items >= 15 && report.imports == 5, "{report}");
    assert_eq!(report.refused, 0);
    // The result compiles.
    check(&out, &rustc()).unwrap();
    // Pruning what is pruned changes nothing.
    assert_eq!(pruned(&out).0, out);
}

#[test]
fn an_unformatted_bundle_is_pruned_too() {
    let (out, _) = pruned("fn dead(){}fn main(){}");
    assert_eq!(squashed(&out), "fnmain(){}");
}

#[test]
fn struct_fields_stay() {
    let source = r#"struct Pair {
    read: u8,
    unread: u8,
}
fn main() {
    let pair = Pair { read: 1, unread: 2 };
    println!("{}", pair.read);
}
"#;
    let (out, report) = pruned(source);
    assert_eq!(out, source);
    assert_eq!(report.items + report.imports, 0, "{report}");
}

#[test]
fn an_enum_without_fields_loses_only_its_last_variants() {
    // `B`, `D` and `E` are never built: `D` and `E` go, `B` stays so that
    // `C` keeps its number.
    let source = r#"#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    A,
    B,
    #[allow(unused)]
    C,
    D,
    E,
}
fn main() {
    let kinds = [Kind::A, Kind::C];
    for kind in kinds {
        let name = match kind {
            Kind::A => "a",
            Kind::B | Kind::C => "b or c",
            Kind::D => {
                "d"
            }
            Kind::E => "e",
        };
        println!("{} {} {:?}", kind as u8, name, matches!(kind, Kind::D | Kind::E | Kind::C));
    }
}
"#;
    let (out, report) = pruned(source);
    let squashed = squashed(&out);
    assert!(
        squashed.contains("enumKind{A,B,#[allow(unused)]C,}"),
        "{out}"
    );
    assert!(
        squashed.contains("Kind::B|Kind::C=>\"borc\",") && !squashed.contains("Kind::D"),
        "{out}"
    );
    assert!(squashed.contains("matches!(kind,Kind::C)"), "{out}");
    assert_eq!(report.items, 2, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_variant_with_fields_goes_from_arms_and_matches() {
    let source = r#"enum Budget {
    Until(u64),
    Iterations(u32),
}
impl Budget {
    fn spent(&self, done: u32) -> bool {
        match *self {
            Self::Until(limit) if limit > 3 => true,
            Self::Until(_) => false,
            Budget::Iterations(n) => done >= n,
        }
    }
    fn timed(&self) -> bool {
        matches!(self, Budget::Until(_) | Budget::Iterations(0))
    }
}
fn main() {
    let budget = Budget::Iterations(3);
    println!("{} {}", budget.spent(4), budget.timed());
}
"#;
    let (out, report) = pruned(source);
    assert!(!out.contains("Until"), "{out}");
    assert!(
        squashed(&out).contains("matches!(self,Budget::Iterations(0))"),
        "{out}"
    );
    assert_eq!(report.items, 1, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn variants_that_a_use_imports_or_a_hash_numbers_stay() {
    // A removed `B` would make the pattern `B` a binding that matches all.
    let source = r#"#[derive(Clone, Copy)]
enum Imported {
    A,
    B,
}
use Imported::*;
#[derive(Hash)]
enum Hashed {
    X(u8),
    Y,
}
fn main() {
    let x = A;
    let n = match x {
        A => 1,
        B => 2,
    };
    let h = Hashed::X(1);
    println!("{} {}", n, matches!(h, Hashed::Y));
}
"#;
    let (out, _) = pruned(source);
    assert!(
        squashed(&out).contains("enumImported{A,B,}") && out.contains("Y,"),
        "{out}"
    );
}

#[test]
fn a_removal_that_rustc_rejects_is_undone() {
    // Nothing calls `f`, but an impl of the trait still defines it.
    let source = r#"trait Tr {
    fn f(&self) -> u8;
    fn id(&self) -> u8 {
        1
    }
}
impl Tr for u8 {
    fn f(&self) -> u8 {
        2
    }
}
fn dead() {}
fn main() {
    println!("{}", std::mem::size_of::<&dyn Tr>());
}
"#;
    let (out, report) = pruned(source);
    assert!(!out.contains("dead"), "{out}");
    assert!(out.contains("fn f(&self) -> u8;"), "{out}");
    // `dead` and the default method `id`, which nothing calls.
    assert_eq!(report.items, 2, "{report}");
    assert_eq!(report.refused, 1, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_chain_of_unused_items_goes_even_when_one_removal_is_rejected() {
    // `below` and `next` are used by `dead` only; `f` stays for its impl.
    let source = r#"trait Tr {
    fn f(&self) -> u8;
}
impl Tr for u8 {
    fn f(&self) -> u8 {
        2
    }
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
fn dead(rng: &mut Rng) -> u64 {
    rng.below(3)
}
fn main() {
    println!("{}", std::mem::size_of::<&dyn Tr>());
}
"#;
    let (out, report) = pruned(source);
    for gone in ["Rng", "below", "next", "dead"] {
        assert!(!out.contains(gone), "{gone} is still in\n{out}");
    }
    assert!(out.contains("fn f(&self) -> u8;"), "{out}");
    assert_eq!(report.refused, 1, "{report}");
}

#[test]
fn the_result_depends_on_the_text_only() {
    let source = "use std::fmt::Debug;\nfn a() { b(); }\nfn b() {}\nstruct S;\nimpl S { fn m() {} }\nfn main() {}\n";
    let first = pruned(source);
    let second = pruned(source);
    assert_eq!(first.0, second.0);
    assert_eq!(first.1.items, second.1.items);
    assert_eq!(first.1.compiles, second.1.compiles);
    assert_eq!(squashed(&first.0), "fnmain(){}");
}

#[test]
fn text_in_literals_and_macros_is_not_touched() {
    let source = r#"macro_rules! twice {
    ($x:expr) => {
        $x + $x
    };
}
fn used() -> u8 {
    2
}
fn dead() -> &'static str {
    "fn inside() {}"
}
fn main() {
    println!("fn dead() {{}} {}", twice!(used()));
}
"#;
    let (out, _) = pruned(source);
    assert!(!out.contains("-> &'static str"), "{out}");
    assert!(
        out.contains("fn dead() {{}}") && out.contains("macro_rules! twice"),
        "{out}"
    );
}

#[test]
fn trait_impls_main_and_inner_attributes_stay() {
    let source = r#"#![allow(dead_code)]
use std::fmt;
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropped");
    }
}
struct Shown;
impl fmt::Display for Shown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shown")
    }
}
fn main() {
    let _guard = Guard;
    println!("{}", Shown);
}
"#;
    let (out, report) = pruned(source);
    assert_eq!(out, source);
    assert_eq!(report.items + report.imports, 0, "{report}");
}

#[test]
fn a_bundle_that_does_not_compile_is_an_error() {
    let err = prune("fn main() { missing(); }", &rustc()).unwrap_err();
    assert!(
        err.contains("does not compile") && err.contains("missing"),
        "{err}"
    );
    let err = prune("fn main() { let s = \"open", &rustc()).unwrap_err();
    assert!(err.contains("cannot read the bundle"), "{err}");
}

/// A script that is `rustc` for the pruner, in a folder of its own.
#[cfg(unix)]
fn fake_rustc(name: &str, body: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let dir = std::env::temp_dir().join(format!("cg-bundler-fake-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("rustc");
    std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

#[cfg(unix)]
#[test]
fn a_wrong_or_missing_rustc_is_a_clear_error() {
    let old = fake_rustc("old", "echo 'rustc 1.80.0 (abcdef 2024-07-01)'");
    let err = Rustc::find_with(Some(old.clone().into())).unwrap_err();
    assert!(
        err.contains(VERSION) && err.contains("1.80.0") && err.contains(ENV),
        "{err}"
    );
    let newer = fake_rustc("newer", "echo 'rustc 1.90.01 (fake)'");
    assert!(Rustc::find_with(Some(newer.into())).is_err());
    let broken = fake_rustc("broken", "echo oops >&2; exit 3");
    let err = Rustc::find_with(Some(broken.into())).unwrap_err();
    assert!(err.contains(ENV) && err.contains("oops"), "{err}");
    let err = Rustc::find_with(Some("/nonexistent/rustc".into())).unwrap_err();
    assert!(
        err.contains(ENV) && err.contains("/nonexistent/rustc"),
        "{err}"
    );

    let right = fake_rustc("right", "echo 'rustc 1.90.0 (fake 2025-09-01)'");
    assert!(Rustc::find_with(Some(right.into())).is_ok());
    // An empty value is no value.
    assert_eq!(
        Rustc::find_with(Some("".into())).is_ok(),
        Rustc::find_with(None).is_ok()
    );
}

#[cfg(unix)]
#[test]
fn a_compiler_that_fails_stops_the_pruning() {
    let right = fake_rustc("fails", "case \"$1\" in --version) echo 'rustc 1.90.0 (fake)';; *) echo 'no way' >&2; exit 1;; esac");
    let fake = Rustc::find_with(Some(right.into())).unwrap();
    let err = prune("fn main() {}", &fake).unwrap_err();
    assert!(
        err.contains("does not compile") && err.contains("no way"),
        "{err}"
    );
}

#[test]
fn an_item_that_shares_a_name_with_a_blocked_one_still_goes() {
    // `A::get` stays for a trait impl that is not offered with `A`; `B::get`
    // has the same name and nothing keeps it.
    let source = r#"trait Tr {
    fn t(&self) -> u8;
}
struct A;
impl A {
    fn get(&self) -> u8 {
        1
    }
}
impl Tr for Option<A> {
    fn t(&self) -> u8 {
        self.as_ref().map_or(0, A::get)
    }
}
struct B;
impl B {
    fn get(&self) -> u8 {
        2
    }
}
fn main() {}
"#;
    let (out, report) = pruned(source);
    assert!(
        !out.contains("struct B") && !out.contains("impl B"),
        "{out}"
    );
    assert!(
        out.contains("impl Tr for Option<A>") && out.contains("struct A;"),
        "{out}"
    );
    assert!(report.refused >= 1, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_dead_type_or_trait_goes_with_its_impls() {
    let source = r#"use std::fmt;
mod shapes {
    pub trait Area {
        fn area(&self) -> u32;
    }
    pub struct Square(pub u32);
    impl Area for Square {
        fn area(&self) -> u32 {
            self.0 * self.0
        }
    }
    pub struct Error;
    impl Error {
        pub fn code(&self) -> u8 {
            helper()
        }
    }
    fn helper() -> u8 {
        7
    }
    impl super::fmt::Display for Error {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            write!(f, "{}", self.code())
        }
    }
    impl Drop for Error {
        fn drop(&mut self) {}
    }
}
mod user {
    // An import and an impl of the dead trait, in another module.
    use crate::shapes::Area;
    impl Area for u8 {
        fn area(&self) -> u32 {
            0
        }
    }
}
mod other {
    // Another `Error`, in use: its impls stay.
    pub struct Error;
    impl Drop for Error {
        fn drop(&mut self) {
            println!("dropped");
        }
    }
}
fn main() {
    let side = shapes::Square(3);
    let _guard = other::Error;
    println!("{}", side.0);
    let _ = fmt::Error;
}
"#;
    let (out, _) = pruned(source);
    for gone in [
        "Area",
        "area",
        "helper",
        "code",
        "Display",
        "fn drop(&mut self) {}",
    ] {
        assert!(!out.contains(gone), "{gone} is still in\n{out}");
    }
    assert!(out.contains("println!(\"dropped\")"), "{out}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn an_arm_left_unreachable_goes() {
    let source = r#"enum Mode {
    Fast(u8),
    Slow(u8),
}
fn speed(mode: &Mode) -> u8 {
    match mode {
        Mode::Fast(n) => *n,
        _ => 0,
    }
}
fn main() {
    println!("{}", speed(&Mode::Fast(2)));
}
"#;
    let (out, report) = pruned(source);
    assert!(!out.contains("Slow") && !out.contains("_ =>"), "{out}");
    assert!(out.contains("Mode::Fast(n) => *n,"), "{out}");
    assert_eq!(report.items, 2, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn an_unreachable_alternative_alone_keeps_its_arm() {
    let source = r#"fn kind(n: u8) -> u8 {
    match n {
        0 | 1 => 1,
        2 | 1 => 2,
        _ => 3,
    }
}
fn main() {
    println!("{}", kind(2));
}
"#;
    let (out, report) = pruned(source);
    assert_eq!(out, source);
    assert_eq!(report.items, 0, "{report}");
}

#[test]
fn an_impl_of_a_std_item_stays_beside_a_dead_item_of_the_same_name() {
    // `b`'s `Drop` is std's and `Rc` is std's: neither impl goes with the
    // dead local items of those names.
    let source = r#"mod a {
    pub trait Drop {
        fn gone(&self);
    }
    pub struct Rc;
}
mod b {
    use std::rc::Rc;
    pub struct G;
    impl Drop for G {
        fn drop(&mut self) {
            println!("dropped");
        }
    }
    pub trait Tr {
        fn n(&self) -> u8;
    }
    pub struct N;
    impl Tr for N {
        fn n(&self) -> u8 {
            1
        }
    }
    impl Tr for Rc<N> {
        fn n(&self) -> u8 {
            2
        }
    }
    pub fn show() {
        let _g = G;
        println!("{}", Rc::new(N).n());
    }
}
fn main() {
    b::show();
}
"#;
    let (out, _) = pruned(source);
    assert!(
        !out.contains("fn gone") && !out.contains("pub struct Rc;"),
        "{out}"
    );
    assert!(
        out.contains("impl Drop for G") && out.contains("impl Tr for Rc<N>"),
        "{out}"
    );
    check(&out, &rustc()).unwrap();
}

#[test]
fn an_enum_with_empty_brackets_keeps_its_variants() {
    // `A()` has no field, so `E` can be cast: `C` must stay 2.
    let source = r#"enum E {
    A(),
    B,
    C,
}
fn main() {
    for e in [E::A(), E::C] {
        println!("{}", e as u8);
    }
}
"#;
    let (out, _) = pruned(source);
    assert_eq!(out, source);
}

#[test]
fn a_pattern_of_a_std_enum_is_not_taken_for_a_dead_variant() {
    // The local `Ordering` has a `Less` that is never built; the match is on
    // std's `Ordering`.
    let source = r#"mod a {
    pub enum Ordering {
        More,
        Less,
    }
    pub fn more() -> Ordering {
        Ordering::More
    }
}
fn sign(x: i8) -> i8 {
    match x.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        _ => 1,
    }
}
fn main() {
    let _ = a::more();
    println!("{}", sign(-3));
}
"#;
    let (out, _) = pruned(source);
    assert!(out.contains("std::cmp::Ordering::Less => -1"), "{out}");
    assert!(!out.contains("Less,") && out.contains("More"), "{out}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_matches_with_no_alternative_left_keeps_its_variant() {
    let source = r#"enum Mode {
    Fast(u8),
    Slow(u8),
}
fn make() -> Mode {
    Mode::Fast(1)
}
fn main() {
    println!("{}", matches!(make(), Mode::Slow(_)));
}
"#;
    let (out, _) = pruned(source);
    assert!(
        out.contains("Slow(u8)") && out.contains("Mode::Slow(_)"),
        "{out}"
    );
    check(&out, &rustc()).unwrap();
}

#[test]
fn names_of_a_derive_list_are_removed_one_by_one() {
    let text = "#[derive(Clone, Copy, std::fmt::Debug,)]\n#[allow(dead_code)]\npub struct S;\n\
                #[derive(Default)]\nenum E {\n    #[default]\n    A,\n}\nfn main() {}\n";
    let source = Source::new(text).unwrap();
    let trials = source.trials();
    // The enum's `Default` is not offered: its `#[default]` needs it.
    assert_eq!(trials.len(), 3, "{trials:?}");
    let render = |ids: &[Id]| source.render(&ids.iter().copied().collect()).text;
    assert!(
        render(&trials[1..2]).starts_with("#[derive(Clone,std::fmt::Debug)]\n#[allow"),
        "{}",
        render(&trials[1..2])
    );
    assert!(
        render(&trials).starts_with("\n#[allow(dead_code)]"),
        "{}",
        render(&trials)
    );
    for (id, subject) in trials.iter().zip(["Clone", "Copy", "Debug"]) {
        assert_eq!(source.subject(*id), Some(("S", subject)));
    }
}

#[test]
fn derived_traits_that_nothing_needs_go() {
    let source = r#"#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct P(u8);
#[derive(Debug, Default)]
struct Unused(u8);
fn main() {
    let p = P(1);
    let q = p;
    println!("{}", p.0 + q.0 + (p == q) as u8);
    println!("{}", Unused(2).0);
}
"#;
    let (out, report) = pruned(source);
    assert!(
        out.starts_with("#[derive(Clone,Copy,PartialEq)]\nstruct P(u8);\nstruct Unused(u8);"),
        "{out}"
    );
    assert_eq!(report.derives, 6, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_clone_that_would_copy_a_reference_keeps_its_derive() {
    // Without `Clone`, `r.clone()` compiles and clones the reference.
    let source = r#"#[derive(Clone)]
struct S(u8);
fn show(s: &S) -> u8 {
    s.0
}
fn main() {
    let s = S(1);
    let r = &s;
    let c = r.clone();
    println!("{}", show(&c));
}
"#;
    let (out, report) = pruned(source);
    assert_eq!(out, source);
    assert_eq!(report.derives, 0, "{report}");
    // `to_owned` does the same, and rustc does not report it.
    let source = source.replace("r.clone()", "r.to_owned()");
    assert_eq!(pruned(&source).0, source);
}

#[test]
fn impls_of_std_traits_go_when_unused_but_drop_stays() {
    let source = r#"use std::fmt;
use std::ops::Deref;
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropped");
    }
}
struct Shown(u8);
impl fmt::Display for Shown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
struct List(Vec<u8>);
impl Deref for List {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}
trait Named {
    fn name(&self) -> u8;
}
impl Named for Shown {
    fn name(&self) -> u8 {
        self.0
    }
}
fn total(xs: &[u8]) -> u8 {
    xs.iter().sum()
}
fn main() {
    let _guard = Guard;
    let list = List(vec![1, 2]);
    println!("{} {}", total(&list), Shown(3).name());
}
"#;
    let (out, report) = pruned(source);
    assert!(
        !out.contains("Display") && !out.contains("use std::fmt"),
        "{out}"
    );
    for kept in [
        "impl Drop for Guard",
        "impl Deref for List",
        "impl Named for Shown",
    ] {
        assert!(out.contains(kept), "{kept} is not in\n{out}");
    }
    assert_eq!(report.trait_impls, 1, "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn a_chain_of_debug_impls_goes_together() {
    // `Inner`'s `Debug` is needed while `Outer`'s impl stays.
    let source = r#"use std::fmt;
#[derive(Debug)]
struct Inner(u8);
struct Outer(Inner);
impl fmt::Debug for Outer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Outer").field(&self.0).finish()
    }
}
fn main() {
    let o = Outer(Inner(1));
    println!("{}", (o.0).0);
}
"#;
    let (out, report) = pruned(source);
    assert!(!out.contains("Debug") && !out.contains("fmt"), "{out}");
    assert_eq!((report.derives, report.trait_impls), (1, 1), "{report}");
    check(&out, &rustc()).unwrap();
}

#[test]
fn debug_assertions_go_unless_the_types_need_them() {
    // Without the second assertion, `n`'s type is not known.
    let source = r#"fn small(n: u8) -> bool {
    n < 5
}
fn main() {
    let x = 3u8;
    debug_assert!(x > 1, "x is {x}");
    let n = Default::default();
    debug_assert_eq!(small(n), true);
    println!("{} {}", x, n);
}
"#;
    let (out, report) = pruned(source);
    assert!(
        !out.contains("debug_assert!") && out.contains("debug_assert_eq!"),
        "{out}"
    );
    assert_eq!(report.asserts, 1, "{report}");
    check(&out, &rustc()).unwrap();
}
