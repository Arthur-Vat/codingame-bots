use std::sync::atomic::{AtomicU32, Ordering};

use super::*;

/// A fresh directory holding `files` (relative path, content).
fn crate_dir(files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "cg-bundler-test-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    for (path, content) in files {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }
    dir
}

#[test]
fn parses_module_declarations() {
    let parsed = parse_mod_declaration("    pub(crate) mod engine;").unwrap();
    assert_eq!(
        parsed,
        ModDeclaration {
            indent: "    ",
            visibility: "pub(crate) ",
            name: "engine"
        }
    );
    assert_eq!(parse_mod_declaration("mod a;").unwrap().visibility, "");
    assert_eq!(parse_mod_declaration("pub mod a; ").unwrap().name, "a");
    for not_a_declaration in [
        "mod a { }",
        "// mod a;",
        "/// mod a;",
        "let model;",
        "fn mod_a();",
        "mod 1a;",
        "unsafe mod a;",
    ] {
        assert_eq!(
            parse_mod_declaration(not_a_declaration),
            None,
            "{not_a_declaration:?}"
        );
    }
}

#[test]
fn inlines_nested_modules_from_both_layouts() {
    let dir = crate_dir(&[
        ("src/lib.rs", "//! Root.\npub mod a;\nmod b;\n"),
        ("src/a.rs", "pub mod deep;\npub fn a() {}\n"),
        ("src/a/deep.rs", "pub fn deep() {}\n"),
        ("src/b/mod.rs", "    pub(super) mod c;\n"),
        ("src/b/c.rs", "fn c() {}\n"),
    ]);
    let source = expand_crate(&dir.join("src/lib.rs")).unwrap();
    assert_eq!(
        source,
        "//! Root.\n\
         pub mod a {\n\
         pub mod deep {\n\
         pub fn deep() {}\n\
         }\n\
         pub fn a() {}\n\
         }\n\
         mod b {\n    \
             pub(super) mod c {\n\
         fn c() {}\n    \
             }\n\
         }\n"
    );
}

#[test]
fn drops_test_modules_but_keeps_other_cfg_test_items() {
    let dir = crate_dir(&[
        (
            "src/main.rs",
            "fn main() {}\n#[cfg(test)]\nmod tests;\n#[cfg(test)]\nfn helper() {}\n",
        ),
        ("src/tests.rs", "this file is never read\n"),
    ]);
    let source = expand_crate(&dir.join("src/main.rs")).unwrap();
    assert_eq!(source, "fn main() {}\n#[cfg(test)]\nfn helper() {}\n");
}

#[test]
fn reports_missing_ambiguous_and_path_modules() {
    let missing = crate_dir(&[("src/lib.rs", "mod gone;\n")]);
    let error = expand_crate(&missing.join("src/lib.rs")).unwrap_err();
    assert!(
        error.contains("lib.rs:1") && error.contains("not found"),
        "{error}"
    );

    let both = crate_dir(&[
        ("src/lib.rs", "mod twice;\n"),
        ("src/twice.rs", ""),
        ("src/twice/mod.rs", ""),
    ]);
    let error = expand_crate(&both.join("src/lib.rs")).unwrap_err();
    assert!(error.contains("both"), "{error}");

    let path = crate_dir(&[("src/lib.rs", "#[path = \"x.rs\"]\nmod x;\n")]);
    let error = expand_crate(&path.join("src/lib.rs")).unwrap_err();
    assert!(error.contains("#[path]"), "{error}");
}

const ANALYSIS: &str = "#[cfg(feature = \"analysis\")]";

#[test]
fn drops_analysis_lines_with_the_attribute_line() {
    // `report.rs` does not exist: the module is dropped before its file is
    // looked for.
    let dir = crate_dir(&[
        (
            "src/lib.rs",
            &format!(
                "pub mod search;\n\
                 {ANALYSIS}\n\
                 pub mod report;\n\
                 {ANALYSIS}\n\
                 pub use report::Report;\n"
            ),
        ),
        (
            "src/search.rs",
            &format!(
                "pub struct Node {{\n    \
                     visits: u32,\n    \
                     {ANALYSIS}\n    \
                     draws: f32,\n    \
                     value: f32,\n\
                 }}\n\
                 pub fn new() -> Node {{\n    \
                     let mut node = Node {{\n        \
                         visits: 0,\n        \
                         {ANALYSIS}\n        \
                         draws: 0.0,\n        \
                         value: 0.0,\n    \
                     }};\n    \
                     {ANALYSIS}\n    \
                     record(&mut node.draws);\n    \
                     node.visits += 1;\n    \
                     node\n\
                 }}\n"
            ),
        ),
    ]);
    let source = expand_crate(&dir.join("src/lib.rs")).unwrap();
    assert!(!source.contains("analysis"), "{source}");
    assert!(!source.contains("report") && !source.contains("draws"));
    assert!(source.contains("visits: u32,\n    value: f32,"), "{source}");
    assert!(
        source.contains("visits: 0,\n        value: 0.0,"),
        "{source}"
    );
    assert!(source.contains("    };\n    node.visits += 1;"), "{source}");
}

#[test]
fn text_with_analysis_lines_dropped_compiles_alone() {
    let dir = crate_dir(&[
        (
            "src/lib.rs",
            &format!(
                "pub mod core;\n\
                 {ANALYSIS}\n\
                 pub mod extra;\n\
                 pub fn run() -> u32 {{\n    \
                     let node = core::Node {{\n        \
                         visits: 1,\n        \
                         {ANALYSIS}\n        \
                         extra: 0.0,\n    \
                     }};\n    \
                     {ANALYSIS}\n    \
                     extra::touch(&node.extra);\n    \
                     node.visits\n\
                 }}\n"
            ),
        ),
        (
            "src/core.rs",
            &format!(
                "pub struct Node {{\n    \
                     pub visits: u32,\n    \
                     {ANALYSIS}\n    \
                     pub extra: f32,\n\
                 }}\n"
            ),
        ),
    ]);
    let source = expand_crate(&dir.join("src/lib.rs")).unwrap();
    let file = dir.join("bundle.rs");
    fs::write(&file, &source).unwrap();
    let compiled = std::process::Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--emit",
            "metadata",
        ])
        .arg("-o")
        .arg(dir.join("bundle.rmeta"))
        .arg("-D")
        .arg("warnings")
        .arg(&file)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{source}\n{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
}

#[test]
fn an_analysis_line_must_mark_a_line_and_not_follow_cfg_test() {
    let dangling = crate_dir(&[("src/lib.rs", &format!("fn a() {{}}\n{ANALYSIS}\n"))]);
    let error = expand_crate(&dangling.join("src/lib.rs")).unwrap_err();
    assert!(
        error.contains("lib.rs:2") && error.contains("marks nothing"),
        "{error}"
    );
    let stacked = crate_dir(&[("src/lib.rs", &format!("#[cfg(test)]\n{ANALYSIS}\nmod a;\n"))]);
    let error = expand_crate(&stacked.join("src/lib.rs")).unwrap_err();
    assert!(error.contains("cannot follow"), "{error}");
}

/// The error of expanding a crate whose `lib.rs` is `source`.
fn analysis_error(source: &str) -> String {
    let dir = crate_dir(&[("src/lib.rs", source)]);
    expand_crate(&dir.join("src/lib.rs")).unwrap_err()
}

#[test]
fn a_marker_must_precede_one_plain_line_of_code() {
    for (after_marker, reason) in [
        ("\n", "blank"),
        ("   \n", "blank"),
        ("#[inline]\n    fn one() {}\n", "attribute"),
        (&format!("{ANALYSIS}\n    *x += 1;\n"), "attribute"),
        ("// a comment\n    *x += 1;\n", "comment"),
        ("/// a doc comment\n    fn one() {}\n", "comment"),
        ("/* a comment */ *x += 1;\n", "comment"),
        ("fn opens() {\n    *x += 1;\n}\n", "opens a bracket"),
        ("let v = [\n    1,\n];\n", "opens a bracket"),
        ("}\n", "closes a bracket"),
        ("(a, b]\n", "closes a bracket"),
    ] {
        let error = analysis_error(&format!("fn a() {{}}\n{ANALYSIS}\n{after_marker}"));
        assert!(
            error.contains("lib.rs:2")
                && error.contains("one plain line")
                && error.contains(reason),
            "{after_marker:?}: {error}"
        );
    }
    // Plain lines are accepted: items, fields, statements, brackets that
    // close on the line.
    for plain in [
        "mod analysis;",
        "pub use report::{MoveReport, Report};",
        "draws: f32, // the share of draws",
        "record(&mut node.draws, [1, 2][0], {3});",
    ] {
        let dir = crate_dir(&[(
            "src/lib.rs",
            &format!("{ANALYSIS}\n{plain}\nfn kept() {{}}\n"),
        )]);
        let source = expand_crate(&dir.join("src/lib.rs")).unwrap();
        assert_eq!(source, "fn kept() {}\n", "{plain}");
    }
}

#[test]
fn a_marker_inside_a_string_literal_is_not_told_apart_from_code() {
    // The bundler reads lines, not Rust: it cannot know that this marker is
    // text. This is why the markers belong on lines of code, and why the
    // documentation asks for that rather than promising to catch it.
    let source = format!("fn a() -> &'static str {{\n    \"x\n{ANALYSIS}\ny\"\n}}\n");
    let dir = crate_dir(&[("src/lib.rs", &source)]);
    let expanded = expand_crate(&dir.join("src/lib.rs")).unwrap();
    assert_eq!(expanded, "fn a() -> &'static str {\n    \"x\n}\n");
    // Where what follows is not a plain line, the marker is rejected.
    let source = format!("fn a() -> &'static str {{\n    \"x\n{ANALYSIS}\n\ny\"\n}}\n");
    assert!(analysis_error(&source).contains("blank"));
}
