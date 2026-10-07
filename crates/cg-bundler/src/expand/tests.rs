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
