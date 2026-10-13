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
