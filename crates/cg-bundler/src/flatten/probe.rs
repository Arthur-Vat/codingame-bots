//! The check that flattening keeps what every name of the bundle means.
//!
//! Every item that can carry it gets `#[deprecated]`: free functions,
//! types, traits, constants and statics, the methods and constants of
//! inherent `impl`s and the methods and constants that traits declare, and
//! items inside function bodies; not the items of trait impls (rustc
//! rejects the attribute there), `main`, `_`, associated types and the
//! modules, which go. rustc then reports each use of each of them, with
//! the span of the name and the item's path (lint `deprecated`). The
//! bundle before and after flattening, both marked, must report the same
//! items at the same tokens: a call that would reach another function, a
//! name in a pattern that would become a constant, a type that would be
//! another, all show as a difference. Fields and variants are reported
//! through their type.

use std::collections::BTreeMap;

use super::tree::{Brace, Shape, NONE};
use super::Rendered;
use crate::minify::{Kind, Token};
use crate::prune::Diagnostic;

/// The lint that reports the uses of the marked items.
pub const LINT: &str = "deprecated";

/// The attribute that marks an item.
pub const MARK: &str = "#[deprecated]";

/// The tokens before which [`MARK`] goes: the start of each item that can
/// carry it, after its attributes.
pub fn marks(tokens: &[Token], shape: &Shape) -> Vec<usize> {
    let mut marks = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if token.kind != Kind::Ident || shape.attribute[i] {
            continue;
        }
        let next = tokens.get(i + 1).map_or("", |t| t.text.as_str());
        match token.text.as_str() {
            "fn" | "struct" | "enum" | "trait" | "type" | "const" | "static" => {}
            "union" if tokens.get(i + 1).is_some_and(|t| t.kind == Kind::Ident) => {}
            _ => continue,
        }
        if token.text == "const" && next == "fn" {
            continue;
        }
        let Some(start) = shape.item_start(tokens, i) else {
            continue;
        };
        let parent = shape.parent[start];
        let kind = match parent {
            NONE => Brace::Module,
            _ => match shape.brace.get(&parent) {
                Some(&kind) => kind,
                // In parentheses or brackets: not an item.
                None => continue,
            },
        };
        let associated = matches!(kind, Brace::Impl | Brace::Trait);
        if kind == Brace::TraitImpl || (associated && token.text == "type") {
            continue;
        }
        let name = match next {
            "mut" => tokens.get(i + 2),
            _ => tokens.get(i + 1),
        };
        let name = name.map_or("", |t| t.text.as_str());
        if name == "_" || (name == "main" && parent == NONE) {
            continue;
        }
        marks.push(start);
    }
    marks
}

/// The uses of marked items that `diagnostics` report: for each, the
/// source token where its span starts and the item's path as `path` gives
/// it, with how many times. Spans that start in no source token, or in a
/// token that `skip` names, are left out.
pub fn uses(
    diagnostics: &[Diagnostic],
    rendered: &Rendered,
    skip: impl Fn(usize) -> bool,
    path: impl Fn(&str) -> String,
) -> BTreeMap<(usize, String), usize> {
    let mut found = BTreeMap::new();
    for diagnostic in diagnostics.iter().filter(|d| d.lint == LINT) {
        let message = match diagnostic.message.split('`').nth(1) {
            Some(item) => {
                let kind = diagnostic.message.split('`').next().unwrap_or("");
                format!("{kind}`{}`", plain_impl(&path(item)))
            }
            None => diagnostic.message.clone(),
        };
        for &(start, _) in &diagnostic.spans {
            let Some(token) = rendered.token_at(start) else {
                continue;
            };
            if !skip(token) {
                *found.entry((token, message.clone())).or_insert(0) += 1;
            }
        }
    }
    found
}

/// `item` with `<impl Type>::name` written `Type::name`: rustc names a
/// method so when its `impl` is in another module than its type, which
/// flattening changes.
fn plain_impl(item: &str) -> String {
    let Some(rest) = item.strip_prefix("<impl ") else {
        return item.to_string();
    };
    match rest.split_once(">::") {
        Some((ty, name))
            if ty
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == ':') =>
        {
            format!("{ty}::{name}")
        }
        _ => item.to_string(),
    }
}

/// The first use that `before` and `after` do not share, described with
/// what the other reports at the same token.
pub fn difference(
    before: &BTreeMap<(usize, String), usize>,
    after: &BTreeMap<(usize, String), usize>,
    tokens: &[Token],
) -> Option<String> {
    let at = |map: &BTreeMap<(usize, String), usize>, token: usize| -> String {
        let found: Vec<String> = map
            .iter()
            .filter(|((t, _), _)| *t == token)
            .map(|((_, message), count)| format!("{count} {message}"))
            .collect();
        match found.is_empty() {
            true => "nothing".to_string(),
            false => found.join(", "),
        }
    };
    let missing = before
        .iter()
        .find(|(key, count)| after.get(*key) != Some(*count));
    let extra = after
        .iter()
        .find(|(key, count)| before.get(*key) != Some(*count));
    let token = missing.or(extra)?.0 .0;
    Some(format!(
        "line {} (`{}`) reports {} before flattening and {} after",
        tokens[token].line,
        tokens[token].text,
        at(before, token),
        at(after, token)
    ))
}
