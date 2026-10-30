//! Shrinks a bundle to the tokens Rust needs (ADR 0045, level 1). Level 2
//! ([`crate::prune`]) removes unused items first and hands its text to this
//! module, which checks its own step as ever: the output has the tokens of
//! what it was given.
//!
//! The text is lexed ([`tokens`]) and written again with only the
//! whitespace that keeps neighbouring tokens apart. Comments go, doc
//! comments included. String, byte string, raw string and character
//! literals are copied byte for byte, so data packed into them (ADR 0037)
//! passes through unchanged.
//!
//! Two tokens are written without whitespace between them when lexing their
//! texts together gives the same two tokens. Beyond that, whitespace stays
//! between a name or number and a following `"`, `'` or `#` (the reserved
//! prefixes of the 2021 edition: `b "x"` is not `b"x"`), between a literal
//! and a name that would become its suffix, and between `/` and a following
//! `/` or `*`.
//!
//! CodinGame's limits on the length of a line are not known, so no line gets
//! long: once a line has [`SOFT_LINE`] characters, the next whitespace is a
//! newline, and when it has [`HARD_LINE`] a newline goes between any two
//! tokens, though none was needed. Literals that span lines stay as they
//! are.
//!
//! [`minify`] checks its work: the tokens of the result must be those of
//! the input, or it fails and names the first token that differs. Same
//! tokens are not always the same program: `stringify!` and `line!` see
//! the whitespace and the lines, so bots do not use them (the answers check
//! of ADR 0045, decision 4, would catch a change in what a bot sends).

mod lex;

pub use lex::{first_difference, tokens, Kind, Token};

use lex::is_ident_continue;

/// A line with this many characters ends at the next whitespace.
pub const SOFT_LINE: usize = 150;
/// A line with this many characters ends before the next token.
pub const HARD_LINE: usize = 300;

/// `source` with comments removed and only the whitespace that Rust needs
/// between tokens, in lines of moderate length. It fails when `source`
/// cannot be lexed, or when the result would not read back as the same
/// tokens.
pub fn minify(source: &str) -> Result<String, String> {
    let input = tokens(source)?;
    let mut out = String::with_capacity(source.len() / 2);
    // The characters on the current line.
    let mut line_length = 0;
    for (i, token) in input.iter().enumerate() {
        if i > 0 {
            let needed = separated(&input[i - 1], token);
            if line_length >= if needed { SOFT_LINE } else { HARD_LINE } {
                out.push('\n');
                line_length = 0;
            } else if needed {
                out.push(' ');
                line_length += 1;
            }
        }
        out.push_str(&token.text);
        line_length = match token.text.rfind('\n') {
            Some(at) => token.text[at + 1..].chars().count(),
            None => line_length + token.text.chars().count(),
        };
    }
    if !out.is_empty() {
        out.push('\n');
    }

    let output = tokens(&out).map_err(|err| format!("the minified text cannot be lexed: {err}"))?;
    if let Some(at) = first_difference(&input, &output) {
        let describe = |tokens: &[Token]| {
            tokens.get(at).map_or("the end".to_string(), |t| {
                format!("`{}` ({:?}, line {})", t.text, t.kind, t.line)
            })
        };
        return Err(format!(
            "minifying changed the tokens: token {at} was {} and became {}",
            describe(&input),
            describe(&output)
        ));
    }
    Ok(out)
}

/// Whether `next` needs whitespace after `prev`.
fn separated(prev: &Token, next: &Token) -> bool {
    let first = next.text.chars().next();
    let reserved_prefix = matches!(prev.kind, Kind::Ident | Kind::Number | Kind::Lifetime)
        && matches!(first, Some('"' | '\'' | '#'));
    let suffix =
        matches!(prev.kind, Kind::Str | Kind::Char) && first.is_some_and(is_ident_continue);
    let comment = prev.text.ends_with('/') && matches!(first, Some('/' | '*'));
    if reserved_prefix || suffix || comment {
        return true;
    }
    let joined = format!("{}{}", prev.text, next.text);
    match tokens(&joined) {
        Ok(two) => !(two.len() == 2 && two[0].same(prev) && two[1].same(next)),
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests;
