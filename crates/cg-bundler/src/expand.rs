//! Inlines a crate's module files into one source text.

use std::fs;
use std::path::{Path, PathBuf};

/// The source of the crate rooted at `root` (`lib.rs` or `main.rs`), with
/// every `mod name;` replaced by `mod name { ... }` holding that module's
/// file, recursively. Test modules are dropped.
pub fn expand_crate(root: &Path) -> Result<String, String> {
    expand_file(root, true)
}

/// `owns_directory` is true for files whose submodules live next to them
/// (`lib.rs`, `main.rs`, `mod.rs`), false for `name.rs` files, whose
/// submodules live in `name/`.
fn expand_file(path: &Path, owns_directory: bool) -> Result<String, String> {
    let source =
        fs::read_to_string(path).map_err(|err| format!("cannot read {}: {err}", path.display()))?;
    let directory = if owns_directory {
        path.parent().map(Path::to_path_buf).unwrap_or_default()
    } else {
        let stem = path.file_stem().unwrap_or_default();
        path.with_file_name(stem)
    };

    let mut out = String::with_capacity(source.len());
    // A `#[cfg(test)]` line is held back until we know what it applies to.
    let mut held_cfg_test: Option<&str> = None;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[path") {
            return Err(format!(
                "{}:{}: #[path] attributes are not supported",
                path.display(),
                index + 1
            ));
        }
        if trimmed == "#[cfg(test)]" {
            if let Some(previous) = held_cfg_test.replace(line) {
                push_line(&mut out, previous);
            }
            continue;
        }
        match parse_mod_declaration(line) {
            Some(_) if held_cfg_test.take().is_some() => {} // a test module: dropped
            Some(declaration) => {
                let (file, child_owns_directory) =
                    find_module_file(&directory, declaration.name)
                        .map_err(|err| format!("{}:{}: {err}", path.display(), index + 1))?;
                let body = expand_file(&file, child_owns_directory)?;
                push_line(
                    &mut out,
                    &format!(
                        "{}{}mod {} {{",
                        declaration.indent, declaration.visibility, declaration.name
                    ),
                );
                out.push_str(&body);
                push_line(&mut out, &format!("{}}}", declaration.indent));
            }
            None => {
                if let Some(held) = held_cfg_test.take() {
                    push_line(&mut out, held);
                }
                push_line(&mut out, line);
            }
        }
    }
    if let Some(held) = held_cfg_test {
        push_line(&mut out, held);
    }
    Ok(out)
}

fn push_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}

/// A `mod name;` line, split into its parts.
#[derive(Debug, PartialEq, Eq)]
struct ModDeclaration<'a> {
    indent: &'a str,
    /// Empty, or a visibility followed by a space, such as `pub(crate) `.
    visibility: &'a str,
    name: &'a str,
}

fn parse_mod_declaration(line: &str) -> Option<ModDeclaration<'_>> {
    let code = line.trim_start();
    let indent = &line[..line.len() - code.len()];
    let declaration = code.trim_end().strip_suffix(';')?;
    let (visibility, name) = match declaration.rsplit_once(' ') {
        Some((before, name)) => (before.strip_suffix("mod")?, name),
        None => return None,
    };
    let visible = visibility.trim();
    if !(visible.is_empty()
        || visible == "pub"
        || (visible.starts_with("pub(") && visible.ends_with(')')))
    {
        return None;
    }
    let is_identifier = name
        .chars()
        .next()
        .is_some_and(|c| c.is_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_alphanumeric() || c == '_');
    is_identifier.then_some(ModDeclaration {
        indent,
        visibility,
        name,
    })
}

/// The file of module `name` declared in a file whose submodules live in
/// `directory`, and whether that file owns a directory of its own.
fn find_module_file(directory: &Path, name: &str) -> Result<(PathBuf, bool), String> {
    let flat = directory.join(format!("{name}.rs"));
    let nested = directory.join(name).join("mod.rs");
    match (flat.is_file(), nested.is_file()) {
        (true, false) => Ok((flat, false)),
        (false, true) => Ok((nested, true)),
        (true, true) => Err(format!(
            "module {name:?} exists as both {} and {}",
            flat.display(),
            nested.display()
        )),
        (false, false) => Err(format!(
            "module {name:?} not found: expected {} or {}",
            flat.display(),
            nested.display()
        )),
    }
}

#[cfg(test)]
mod tests;
