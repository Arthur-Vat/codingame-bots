//! Bundles a bot and the workspace crates it uses into one Rust file that
//! CodinGame accepts.
//!
//! The bundler works on text. By default it then minifies the result to
//! save room under CodinGame's size limit (ADR 0016, ADR 0045). Level 2,
//! the default, first removes the items that rustc 1.90.0 reports as never
//! used (module [`prune`]); level 1 (`--minify-level 1`) skips that. Then
//! it reads the code token by token and writes back only the whitespace
//! Rust needs, without comments (module [`minify`]). `--keep-comments`
//! gives the readable bundle instead, with comments and rustfmt's layout,
//! which a release keeps beside its minified file (ADR 0045). All three
//! come from the same expanded and rewritten code: the readable bundle and
//! level 1 have the same tokens, level 2 has those left after its removals.
//! Level 2 runs rustc 1.90.0 whatever the session's compiler, found with
//! rustup or named by `CG_RUSTC`, and fails without it. The bundler relies
//! on these conventions, which keep the text transformations safe:
//!
//! - Modules live in files declared with `mod name;` on a line of their own
//!   (optionally `pub` or `pub(...)`). `#[path]` attributes are not allowed.
//! - A `mod name;` line preceded by a `#[cfg(test)]` line is dropped, so tests
//!   belong in their own files (`#[cfg(test)] mod tests;`).
//! - A line that is exactly `#[cfg(feature = "analysis")]` is dropped with
//!   the line after it, which must be the item itself on one line (a field,
//!   a field initializer, a `mod name;`, a `use`, a statement ending on that
//!   line), with no attribute, comment or blank line between. Analysis-only
//!   code goes there, so bots do not carry it. The bundler rejects, with the
//!   file and line, a marker whose next line is blank, an attribute (another
//!   marker included), a comment, or has unbalanced `()`, `[]` or `{}`. It
//!   reads lines, not Rust, so it cannot tell a marker inside a multi-line
//!   string literal from code: markers belong on lines of code. Rustc
//!   ignores the marked item when the feature is off, so a wrong drop of a
//!   plain line can only stop the bundle from compiling.
//! - A bot may only depend on workspace crates, and those crates may only
//!   depend on other workspace crates: CodinGame offers no crates.io.
//! - Code refers to other workspace crates by their crate name (`cg_core::`)
//!   and to its own crate with `crate::` or `$crate::`.
//!
//! Each library becomes a module at the root of the bundle (`mod cg_core
//! { ... }`), and paths are rewritten to match: `cg_core::x` becomes
//! `crate::cg_core::x`, and `crate::x` inside `cg_core` becomes
//! `crate::cg_core::x`.

pub mod expand;
pub mod minify;
pub mod plan;
pub mod prune;
pub mod rewrite;

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::plan::Plan;

/// CodinGame's code size limit, in characters counted as UTF-16 units
/// (ADR 0037).
pub const SIZE_LIMIT: usize = 100_000;

/// The size of `code` as CodinGame counts it: UTF-16 units, so a character
/// above U+FFFF counts as two (ADR 0037).
pub fn cg_size(code: &str) -> usize {
    code.encode_utf16().count()
}

/// How much a bundle is minified (ADR 0045).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Minify {
    /// Not at all: the bundle keeps its comments and layout.
    Off,
    /// Level 1: no comments, only the whitespace Rust needs.
    Level1,
    /// Level 2: level 1, after removing the items rustc reports as never
    /// used (module [`prune`]). It needs rustc 1.90.0.
    Level2,
}

/// How a bundle is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    /// Formats the code with `rustfmt`, which also checks that it parses.
    pub format: bool,
    /// How far the code is minified (ADR 0045).
    pub minify: Minify,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            format: true,
            minify: Minify::Level2,
        }
    }
}

/// A bundle and what the minifier did to it.
#[derive(Clone, Debug)]
pub struct Bundled {
    pub code: String,
    /// What level 2 removed, when it ran.
    pub pruned: Option<prune::Report>,
}

/// Builds the bundled source of `plan`. Its first line is a comment that
/// names the package, minified or not (the release checks read it).
pub fn bundle(plan: &Plan, options: Options) -> Result<String, String> {
    bundle_report(plan, options).map(|bundled| bundled.code)
}

/// As [`bundle`], with what level 2 did.
pub fn bundle_report(plan: &Plan, options: Options) -> Result<Bundled, String> {
    let libraries: Vec<String> = plan.libraries.iter().map(|lib| lib.name.clone()).collect();
    // Before bundling, so that a missing compiler fails early.
    let rustc = match options.minify {
        Minify::Level2 => Some(prune::Rustc::find()?),
        _ => None,
    };

    let bot = expand::expand_crate(&plan.bot.root)?;
    let mut code = rewrite::rewrite(&bot, None, &libraries);

    for library in &plan.libraries {
        let source = expand::expand_crate(&library.root)?;
        // A bot rarely uses all of a library.
        code.push_str("\n#[allow(dead_code, unused_imports)]\n");
        code.push_str(&format!("mod {} {{\n", library.name));
        code.push_str(&rewrite::rewrite(&source, Some(&library.name), &libraries));
        code.push_str("}\n");
    }

    if options.format {
        code = rustfmt(&code)?;
    }
    if options.minify != Minify::Off {
        let header = format!(
            "// Bundled by cg-bundler from the `{}` package. Minified (ADR 0045).\n",
            plan.package
        );
        let mut pruned = None;
        if let Some(rustc) = &rustc {
            let (text, report) = prune::prune(&code, rustc)
                .map_err(|err| format!("cannot remove unused items (level 2): {err}"))?;
            code = text;
            pruned = Some(report);
        }
        let code =
            minify::minify(&code).map_err(|err| format!("cannot minify the bundle: {err}"))?;
        if let Some(rustc) = &rustc {
            prune::check(&code, rustc)
                .map_err(|err| format!("the minified bundle (level 2) is wrong: {err}"))?;
        }
        return Ok(Bundled {
            code: header + &code,
            pruned,
        });
    }
    let mut header = format!(
        "// Bundled by cg-bundler from the `{}` package. Do not edit this file:\n\
         // change the sources in the repository and bundle again.\n",
        plan.package
    );
    if !libraries.is_empty() {
        header.push_str(&format!(
            "// Workspace crates included below: {}.\n",
            libraries.join(", ")
        ));
    }
    header.push('\n');
    Ok(Bundled {
        code: header + &code,
        pruned: None,
    })
}

/// Formats Rust source with `rustfmt`, 2021 edition.
fn rustfmt(source: &str) -> Result<String, String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2021", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("cannot run rustfmt ({err}); use --no-format to skip it"))?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let input = source.to_string();
    // Write from another thread so a large output cannot block the input.
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
    let output = child
        .wait_with_output()
        .map_err(|err| format!("rustfmt failed: {err}"))?;
    writer
        .join()
        .map_err(|_| "rustfmt input thread panicked".to_string())?
        .map_err(|err| format!("cannot send the bundle to rustfmt: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "rustfmt rejected the bundle, so it would not compile either:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|err| format!("rustfmt output is not UTF-8: {err}"))
}

/// Bundles the bot package `package` of the workspace containing `manifest`.
pub fn bundle_package(
    package: &str,
    manifest: Option<&Path>,
    options: Options,
) -> Result<String, String> {
    bundle_package_report(package, manifest, options).map(|bundled| bundled.code)
}

/// As [`bundle_package`], with what level 2 did.
pub fn bundle_package_report(
    package: &str,
    manifest: Option<&Path>,
    options: Options,
) -> Result<Bundled, String> {
    let plan = plan::plan(package, manifest)?;
    bundle_report(&plan, options)
}
