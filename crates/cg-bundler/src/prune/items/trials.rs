//! The removals that rustc does not report but that the code may not need:
//! a trait name in a `#[derive(...)]`, and a `debug_assert!`,
//! `debug_assert_eq!` or `debug_assert_ne!` statement. (Trait impls are
//! read with the other `impl`s.) Only a compile tells whether one can go.
//!
//! A derived `Default` of an enum is not offered: its `#[default]` variant
//! needs it.

use std::collections::BTreeMap;

use super::{is_opener, is_punct, match_forward};
use crate::minify::{Kind, Token};

/// The macros of debug assertions.
const ASSERTS: [&str; 3] = ["debug_assert", "debug_assert_eq", "debug_assert_ne"];

/// A `#[derive(...)]` attribute whose list holds only paths, on a
/// `struct`, an `enum` or a `union`.
#[derive(Debug)]
pub struct Derive {
    /// Its `#`, its `(` and its `]`.
    pub first: usize,
    pub open: usize,
    pub close: usize,
    /// The first and last tokens of each path of the list, in order.
    pub names: Vec<(usize, usize)>,
    /// The token of the type's name.
    pub subject: usize,
    pub is_enum: bool,
}

/// The `#[derive(...)]` attributes of `tokens` whose lists hold only paths
/// (names and `::`), separated by commas, on a type whose name is found
/// after the other attributes and the visibility.
pub fn derives(tokens: &[Token]) -> Vec<Derive> {
    let mut out = Vec::new();
    for first in 0..tokens.len() {
        let at = |i: usize| tokens.get(first + i);
        let shape = is_punct(&tokens[first], "#")
            && at(1).is_some_and(|t| is_punct(t, "["))
            && at(2).is_some_and(|t| t.kind == Kind::Ident && t.text == "derive")
            && at(3).is_some_and(|t| is_punct(t, "("));
        if !shape {
            continue;
        }
        let open = first + 3;
        let Some(paren) = match_forward(tokens, open) else {
            continue;
        };
        if !tokens.get(paren + 1).is_some_and(|t| is_punct(t, "]")) {
            continue;
        }
        let Some(names) = paths(tokens, open + 1, paren).filter(|n| !n.is_empty()) else {
            continue;
        };
        let Some(keyword) = keyword_after(tokens, paren + 2) else {
            continue;
        };
        let typed = matches!(tokens[keyword].text.as_str(), "struct" | "enum" | "union");
        if typed
            && tokens
                .get(keyword + 1)
                .is_some_and(|t| t.kind == Kind::Ident)
        {
            out.push(Derive {
                first,
                open,
                close: paren + 1,
                names,
                subject: keyword + 1,
                is_enum: tokens[keyword].text == "enum",
            });
        }
    }
    out
}

/// The token after the attributes and the visibility that start at `at`.
fn keyword_after(tokens: &[Token], mut at: usize) -> Option<usize> {
    loop {
        let token = tokens.get(at)?;
        if is_punct(token, "#") && tokens.get(at + 1).is_some_and(|t| is_punct(t, "[")) {
            at = match_forward(tokens, at + 1)? + 1;
        } else if token.kind == Kind::Ident && token.text == "pub" {
            at += 1;
            if tokens.get(at).is_some_and(|t| is_punct(t, "(")) {
                at = match_forward(tokens, at)? + 1;
            }
        } else {
            return (token.kind == Kind::Ident).then_some(at);
        }
    }
}

/// The paths of the list `from..end`, or `None` if it holds anything else.
/// A trailing comma is allowed.
fn paths(tokens: &[Token], from: usize, end: usize) -> Option<Vec<(usize, usize)>> {
    let mut names = Vec::new();
    let mut lo = from;
    for i in from..=end {
        if i < end && !is_punct(&tokens[i], ",") {
            let token = &tokens[i];
            if !(token.kind == Kind::Ident || is_punct(token, "::")) {
                return None;
            }
            continue;
        }
        if i > lo {
            if tokens[i - 1].kind != Kind::Ident {
                return None;
            }
            names.push((lo, i - 1));
        } else if i < end {
            // An empty element before a comma.
            return None;
        }
        lo = i + 1;
    }
    Some(names)
}

/// The debug assertions that are statements, by the token of the macro's
/// name, with the token of their `;`: after `;`, `{` or `}`, so that no
/// attribute or label comes with them.
pub fn asserts(tokens: &[Token]) -> BTreeMap<usize, usize> {
    let mut out = BTreeMap::new();
    for (at, token) in tokens.iter().enumerate().skip(1) {
        if token.kind != Kind::Ident || !ASSERTS.contains(&token.text.as_str()) {
            continue;
        }
        let statement = ["}", "{", ";"]
            .iter()
            .any(|text| is_punct(&tokens[at - 1], text));
        let call = tokens.get(at + 1).is_some_and(|t| is_punct(t, "!"))
            && tokens.get(at + 2).is_some_and(is_opener);
        if !statement || !call {
            continue;
        }
        let Some(close) = match_forward(tokens, at + 2) else {
            continue;
        };
        if tokens.get(close + 1).is_some_and(|t| is_punct(t, ";")) {
            out.insert(at, close + 1);
        }
    }
    out
}
