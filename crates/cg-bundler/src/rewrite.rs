//! Rewrites paths so that code still resolves once every crate is a module
//! of the bundle.

/// Rewrites `source`, which belongs to the library `this_crate` (or to the
/// bot when `None`), for a bundle containing `libraries` as root modules.
///
/// - Inside a library, `crate::x` becomes `crate::lib::x` and `$crate::x`
///   becomes `$crate::lib::x`.
/// - Everywhere, a path starting with a library name, `lib::x`, becomes
///   `crate::lib::x`.
/// - `extern crate` lines for those libraries are removed.
pub fn rewrite(source: &str, this_crate: Option<&str>, libraries: &[String]) -> String {
    let mut text = source.to_string();
    if let Some(name) = this_crate {
        text = replace_path_start(&text, "$crate::", &format!("$crate::{name}::"));
        text = replace_path_start(&text, "crate::", &format!("crate::{name}::"));
    }
    for library in libraries {
        text = replace_path_start(
            &text,
            &format!("{library}::"),
            &format!("crate::{library}::"),
        );
    }
    text.lines()
        .filter(|line| {
            let line = line.trim();
            !libraries
                .iter()
                .any(|library| line == format!("extern crate {library};"))
        })
        .flat_map(|line| [line, "\n"])
        .collect()
}

/// Replaces `prefix` by `replacement` where it starts a path: not preceded
/// by an identifier character, `:` or `$`.
fn replace_path_start(text: &str, prefix: &str, replacement: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(position) = rest.find(prefix) {
        out.push_str(&rest[..position]);
        let starts_path = out
            .chars()
            .next_back()
            .is_none_or(|c| !(c.is_alphanumeric() || c == '_' || c == ':' || c == '$'));
        out.push_str(if starts_path { replacement } else { prefix });
        rest = &rest[position + prefix.len()..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests;
