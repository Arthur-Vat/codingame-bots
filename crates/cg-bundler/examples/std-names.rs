//! Writes the list of names that Rust's std, core and alloc define, which
//! the minifier neither renames where std could be meant nor gives as a new
//! name (ADR 0045, ADR 0046, module `cg_bundler::rename`).
//!
//! It reads the sources of the standard library of rustc 1.90.0, CodinGame's
//! version (the `rust-src` component), and prints the list:
//!
//! ```sh
//! rustup component add rust-src --toolchain 1.90.0
//! cargo run -p cg-bundler --example std-names > crates/cg-bundler/src/rename/std-names.txt
//! ```
//!
//! An argument names another `library` folder of the sources.
//!
//! The names are found in the tokens, without parsing: a name after `fn`,
//! `const`, `type` or `static` (`static mut` included) is a member when it
//! is declared `pub` or in a trait or a trait impl, as is a field `pub
//! name:`; a name after `struct`, `enum`, `union`, `trait`, `mod`, `macro`
//! or `macro_rules!` is an item; a name right in an enum's braces is a
//! variant; a name in a `use` declaration of the
//! preludes (`prelude/*.rs` of core and std, all editions) is in the
//! prelude, which code reaches by the bare name. Public names of private
//! modules and unstable names count too, which only keeps more names than
//! needed. Test files (`tests.rs`, `benches.rs`, and
//! folders `tests` and `benches`) are left out: they are not part of std.
//! So are the folders that std takes from elsewhere (`stdarch`'s
//! `core::arch`, reached only through paths that start with `std::` or
//! `core::`, and the unstable `std::simd`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use cg_bundler::minify::{tokens, Kind, Token};

/// The crates of the standard library whose names are listed.
const CRATES: [&str; 3] = ["core", "alloc", "std"];

fn main() -> ExitCode {
    let library = match std::env::args_os().nth(1) {
        Some(dir) => PathBuf::from(dir),
        None => match default_library() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("error: {err}");
                return ExitCode::FAILURE;
            }
        },
    };
    // Each name with its kinds, a set of the letters `i`, `m`, `p` and `v`.
    let mut names: BTreeMap<String, String> = BTreeMap::new();
    let mut files = 0;
    for name in CRATES {
        let mut paths = Vec::new();
        if let Err(err) = sources(&library.join(name).join("src"), &mut paths) {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
        for path in paths {
            let text = match std::fs::read_to_string(&path) {
                Ok(text) => text,
                Err(err) => {
                    eprintln!("error: cannot read {}: {err}", path.display());
                    return ExitCode::FAILURE;
                }
            };
            match tokens(&text) {
                Ok(tokens) => {
                    collect(&tokens, &mut names);
                    if path.parent().and_then(|dir| dir.file_name()) == Some("prelude".as_ref()) {
                        collect_prelude(&tokens, &mut names);
                    }
                }
                Err(err) => {
                    eprintln!("error: cannot lex {}: {err}", path.display());
                    return ExitCode::FAILURE;
                }
            }
            files += 1;
        }
    }
    if files == 0 {
        eprintln!(
            "error: no source files in {}: is the rust-src component installed?",
            library.display()
        );
        return ExitCode::FAILURE;
    }
    println!(
        "# The names that the standard library of Rust 1.90.0 (core, alloc, std) defines, for"
    );
    println!("# the minifier's renaming (ADR 0046). Written by");
    println!("# `cargo run -p cg-bundler --example std-names`; do not edit.");
    println!("# Each line is the kinds of a name, then the name. Kinds: `m`, a pub fn, const,");
    println!("# type or static, an item of a trait or trait impl, or a pub field, which `.name`");
    println!("# or `T::name` reaches; `i`, a struct, enum, union, trait, mod or macro; `p`, in a");
    println!("# prelude, reached by the bare name; `v`, an enum variant.");
    for (name, kinds) in &names {
        println!("{kinds} {name}");
    }
    eprintln!("{} names from {files} files", names.len());
    ExitCode::SUCCESS
}

/// The `library` folder of the sources of rustc 1.90.0.
fn default_library() -> Result<PathBuf, String> {
    let output = Command::new("rustup")
        .args(["run", "1.90.0", "rustc", "--print", "sysroot"])
        .output()
        .map_err(|err| format!("cannot run rustup: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "rustup cannot run rustc 1.90.0: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(Path::new(&sysroot).join("lib/rustlib/src/rust/library"))
}

/// The `.rs` files under `dir`, test files aside, in a fixed order.
fn sources(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        std::fs::read_dir(dir).map_err(|err| format!("cannot read {}: {err}", dir.display()))?;
    let mut paths: Vec<PathBuf> = entries
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|err| format!("cannot read {}: {err}", dir.display()))?;
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if name != "tests" && name != "benches" {
                sources(&path, out)?;
            }
        } else if name.ends_with(".rs") && name != "tests.rs" && name != "benches.rs" {
            out.push(path);
        }
    }
    Ok(())
}

/// Adds `kind` to the kinds of `name`, kept in alphabetical order.
fn add(names: &mut BTreeMap<String, String>, name: &str, kind: char) {
    let kinds = names.entry(name.to_string()).or_default();
    if !kinds.contains(kind) {
        let mut sorted: Vec<char> = kinds.chars().chain([kind]).collect();
        sorted.sort_unstable();
        *kinds = sorted.into_iter().collect();
    }
}

/// Adds the names in the `use` declarations of a prelude file to `names`.
fn collect_prelude(tokens: &[Token], names: &mut BTreeMap<String, String>) {
    let mut in_use = false;
    for token in tokens {
        match token.text.as_str() {
            "use" => in_use = true,
            ";" => in_use = false,
            name if in_use && token.kind == Kind::Ident && !is_path_word(name) => {
                add(names, name, 'p')
            }
            _ => {}
        }
    }
}

/// Words of `use` paths that are not names.
fn is_path_word(name: &str) -> bool {
    matches!(name, "crate" | "self" | "super" | "as" | "_")
}

/// What a `{` opens, which says whether the members in it are public.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    /// A `trait`: its items are as public as the trait.
    Trait,
    /// An `impl Trait for Type`: its items are those of the trait.
    TraitImpl,
    /// An `enum`: its variants are names.
    Enum,
    Other,
}

/// Adds the names that `tokens` define to `names`, as members (`m`) or
/// other items (`i`). A `fn`, `const`, `type` or `static` is a member only
/// when code outside std can reach it: it is `pub` (not `pub(crate)` or
/// the like), or an item of a trait or of a trait impl. Rustc's method
/// lookup skips the methods it cannot reach.
fn collect(tokens: &[Token], names: &mut BTreeMap<String, String>) {
    let ident = |at: usize| {
        tokens
            .get(at)
            .filter(|t| t.kind == Kind::Ident)
            .map(|t| t.text.as_str())
    };
    // Each open block, with the depth of `(` and `[` where it opened.
    let mut blocks: Vec<(Block, usize)> = Vec::new();
    let mut nest = 0usize;
    // Where the tokens since the last `;`, `{` or `}` start: the header of
    // the next block.
    let mut header = 0;
    for (at, token) in tokens.iter().enumerate() {
        match token.text.as_str() {
            "{" => {
                let words = &tokens[header..at];
                let has = |word: &str| {
                    words
                        .iter()
                        .any(|t| t.kind == Kind::Ident && t.text == word)
                };
                let block = if has("trait") {
                    Block::Trait
                } else if has("impl") && has("for") {
                    Block::TraitImpl
                } else if has("enum") {
                    Block::Enum
                } else {
                    Block::Other
                };
                blocks.push((block, nest));
                header = at + 1;
            }
            "}" => {
                blocks.pop();
                header = at + 1;
            }
            ";" => header = at + 1,
            "(" | "[" => nest += 1,
            ")" | "]" => nest = nest.saturating_sub(1),
            _ => {}
        }
        // A variant: a name right in an enum's braces, after `{`, `,` or an
        // attribute, before what may follow a variant.
        if let Some(&(Block::Enum, depth)) = blocks.last() {
            let after = tokens.get(at + 1).map(|t| t.text.as_str());
            let before = at.checked_sub(1).map(|at| tokens[at].text.as_str());
            if token.kind == Kind::Ident
                && nest == depth
                && matches!(before, Some("{" | "," | "]"))
                && matches!(after, Some("," | "(" | "{" | "=" | "}"))
                && !is_keyword(&token.text)
            {
                add(names, &token.text, 'v');
            }
        }
        // `*const T`, `&raw const x` and the const generic in `<const N:
        // usize>` name no member.
        let before = at.checked_sub(1).map(|at| tokens[at].text.as_str());
        if token.text == "const" && matches!(before, Some("*" | "raw" | "<" | ",")) {
            continue;
        }
        let public = || {
            matches!(blocks.last(), Some((Block::Trait | Block::TraitImpl, _)))
                || is_pub(tokens, at)
        };
        let (name, member) = match token.text.as_str() {
            "fn" | "const" | "type" if public() => (ident(at + 1), true),
            "static" if public() => match ident(at + 1) {
                Some("mut") => (ident(at + 2), true),
                name => (name, true),
            },
            "pub" if tokens.get(at + 2).is_some_and(|t| t.text == ":") => (ident(at + 1), true),
            "struct" | "enum" | "union" | "trait" | "mod" | "macro" => (ident(at + 1), false),
            "macro_rules" if tokens.get(at + 1).is_some_and(|t| t.text == "!") => {
                (ident(at + 2), false)
            }
            _ => continue,
        };
        let Some(name) = name else { continue };
        if is_keyword(name) {
            continue;
        }
        add(names, name, if member { 'm' } else { 'i' });
    }
}

/// Whether the item whose keyword is at `at` is declared `pub`: `pub`
/// comes before it, past `const`, `unsafe`, `async`, `extern "C"` and the
/// like.
fn is_pub(tokens: &[Token], at: usize) -> bool {
    let mut at = at;
    while at > 0 {
        at -= 1;
        let token = &tokens[at];
        match token.text.as_str() {
            "pub" => return true,
            "const" | "unsafe" | "async" | "extern" | "default" | "safe" => {}
            _ if token.kind == Kind::Str => {}
            _ => return false,
        }
    }
    false
}

/// Words that follow those above without being names (`const fn`,
/// `type` in `impl Trait for type`... and the like).
fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "fn" | "unsafe" | "extern" | "async" | "mut" | "for" | "impl" | "where" | "self" | "Self"
    )
}
