//! The format strings whose captured names may become arguments (see the
//! documentation of [`super`]): `format!("{score} at {}", turn)` reads the
//! local `score` by its name, so the name could not change; written
//! `format!("{} at {}", a, turn)` it can.

use std::collections::BTreeSet;

use super::group_end;
use crate::minify::{Kind, Token};

/// The formatting macros of std, with the place of their format string
/// among their arguments.
const FORMAT_MACROS: &[(&str, usize)] = &[
    ("format", 0),
    ("format_args", 0),
    ("print", 0),
    ("println", 0),
    ("eprint", 0),
    ("eprintln", 0),
    ("panic", 0),
    ("unreachable", 0),
    ("todo", 0),
    ("unimplemented", 0),
    ("write", 1),
    ("writeln", 1),
    ("assert", 1),
    ("debug_assert", 1),
    ("assert_eq", 2),
    ("assert_ne", 2),
    ("debug_assert_eq", 2),
    ("debug_assert_ne", 2),
];

/// A call of a formatting macro of std whose captured names may become
/// arguments: its format string is a plain string literal whose
/// placeholders are `{}` or `{name}`, with a format spec of neither `$` nor
/// `*` (`{name:>5.2}`), and its arguments after the string are positional,
/// one for each `{}`, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    /// The index of the format string's token.
    pub string: usize,
    /// The captured names, in the order of the string.
    pub captures: Vec<Capture>,
    /// Where an argument goes when it follows `j` of the explicit arguments:
    /// the token before which `,name` is written, the comma before the
    /// next explicit argument, else the trailing comma or the closing
    /// delimiter.
    pub slots: Vec<usize>,
}

/// A placeholder that captures a name: `{score}`, `{score:?}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub name: String,
    /// The byte range of the name in the string token's text.
    pub start: usize,
    pub end: usize,
    /// How many `{}` come before it, so the index in [`Call::slots`] of
    /// its argument.
    pub slot: usize,
}

impl Call {
    /// The string token's text without the names of the captures that
    /// `convert` says to make arguments.
    pub fn string_text(&self, token: &str, convert: impl Fn(&Capture) -> bool) -> String {
        let mut out = String::with_capacity(token.len());
        let mut copied = 0;
        for capture in self.captures.iter().filter(|c| convert(c)) {
            out.push_str(&token[copied..capture.start]);
            copied = capture.end;
        }
        out.push_str(&token[copied..]);
        out
    }
}

/// The calls of `tokens` whose captured names may become arguments. Calls in
/// `macro_rules!` definitions, calls of a macro that the bundle defines with
/// the same name, and calls whose arguments hold `$`, `|` or `<` (where a
/// top-level comma may not separate two arguments) are left out.
pub fn calls(tokens: &[Token]) -> Vec<Call> {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let mut own_macros = BTreeSet::new();
    let mut in_definition = vec![false; tokens.len()];
    for at in 0..tokens.len() {
        if text(at) == Some("macro_rules") && text(at + 1) == Some("!") {
            if let Some(name) = text(at + 2) {
                own_macros.insert(name);
            }
            let end = group_end(tokens, at + 3).unwrap_or(tokens.len());
            for flag in &mut in_definition[at..end] {
                *flag = true;
            }
        }
    }
    let mut calls = Vec::new();
    for (at, token) in tokens.iter().enumerate() {
        if token.kind != Kind::Ident || in_definition[at] || text(at + 1) != Some("!") {
            continue;
        }
        let Some(&(_, place)) = FORMAT_MACROS.iter().find(|(name, _)| *name == token.text) else {
            continue;
        };
        if own_macros.contains(token.text.as_str()) {
            continue;
        }
        // `std::format!` is std's; `other::format!` and `x.format!` are not.
        let before = at.checked_sub(1).and_then(text);
        let std_path = before == Some("::")
            && at
                .checked_sub(2)
                .and_then(text)
                .is_some_and(|t| matches!(t, "std" | "core" | "alloc"));
        if matches!(before, Some("::" | ".")) && !std_path {
            continue;
        }
        if let Some(call) = call(tokens, at + 2, place) {
            calls.push(call);
        }
    }
    calls
}

/// The call whose arguments are the group that opens at `open`, with its
/// format string at `place`, if its captures may become arguments.
fn call(tokens: &[Token], open: usize, place: usize) -> Option<Call> {
    let end = group_end(tokens, open)?;
    let close = end - 1;
    // The top-level commas.
    let mut commas = Vec::new();
    let mut at = open + 1;
    while at < close {
        match tokens[at].text.as_str() {
            "(" | "[" | "{" => {
                at = group_end(tokens, at)?;
                continue;
            }
            "," => commas.push(at),
            "|" | "||" | "<" | "<<" | "<=" | "<<=" => return None,
            _ => {}
        }
        at += 1;
    }
    if (open + 1..close).any(|i| tokens[i].text.starts_with('$')) {
        return None;
    }
    // The argument at `place` must be the string alone.
    let string = if place == 0 {
        open + 1
    } else {
        commas.get(place - 1)? + 1
    };
    let string_end = commas.get(place).copied().unwrap_or(close);
    if string_end != string + 1 || tokens[string].kind != Kind::Str {
        return None;
    }
    let (captures, implicit) = placeholders(&tokens[string].text)?;
    // The explicit arguments after the string: the commas after it, less a
    // trailing one.
    let after: Vec<usize> = commas.iter().copied().filter(|&c| c > string).collect();
    let trailing = after.last().is_some_and(|&c| c + 1 == close);
    let explicit = after.len() - usize::from(trailing);
    if explicit != implicit {
        return None;
    }
    // A named argument (`width = 5`) is not positional.
    let named = after[..explicit].iter().any(|&c| {
        tokens.get(c + 1).is_some_and(|t| t.kind == Kind::Ident)
            && tokens.get(c + 2).is_some_and(|t| t.text == "=")
    });
    if named {
        return None;
    }
    let mut slots: Vec<usize> = after[..explicit].to_vec();
    slots.push(if trailing { after[explicit] } else { close });
    Some(Call {
        string,
        captures,
        slots,
    })
}

/// The captures of a format string token and its number of `{}`, or `None`
/// when it is not a plain string literal or has a placeholder that is
/// neither `{}` nor `{name}` (an index, a count with `$` or `*`, an escape
/// in braces).
fn placeholders(token: &str) -> Option<(Vec<Capture>, usize)> {
    if !token.starts_with('"') {
        return None;
    }
    let bytes = token.as_bytes();
    let mut captures = Vec::new();
    let mut implicit = 0;
    let mut at = 1;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => {
                at += 2;
                if bytes.get(at - 1) == Some(&b'u') {
                    // `\u{7b}`: its braces are no placeholder.
                    while at < bytes.len() && bytes[at] != b'}' {
                        at += 1;
                    }
                    at += 1;
                }
            }
            b'{' if bytes.get(at + 1) == Some(&b'{') => at += 2,
            b'}' if bytes.get(at + 1) == Some(&b'}') => at += 2,
            // A `}` alone: rustc would reject it, or a fill character that
            // this reading does not follow.
            b'}' => return None,
            b'{' => {
                let close = at + token[at..].find('}')?;
                let inside = &token[at + 1..close];
                if inside.contains(['{', '\\', '$', '*', '"']) {
                    return None;
                }
                let name_end = inside.find(':').unwrap_or(inside.len());
                let name = &inside[..name_end];
                if name.is_empty() {
                    implicit += 1;
                } else if is_name(name) {
                    captures.push(Capture {
                        name: name.to_string(),
                        start: at + 1,
                        end: at + 1 + name.len(),
                        slot: implicit,
                    });
                } else {
                    return None;
                }
                at = close + 1;
            }
            _ => at += 1,
        }
    }
    Some((captures, implicit))
}

/// Whether `word` is a plain ASCII name: a letter or `_`, then letters,
/// digits and `_`.
fn is_name(word: &str) -> bool {
    let mut chars = word.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
