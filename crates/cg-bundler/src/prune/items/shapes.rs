//! What goes with an item when it goes: the `impl`s of a type or a trait,
//! and the patterns that name an enum's variant.
//!
//! A type or a trait that rustc reports unused can only go with its `impl`s,
//! since rustc reports neither a trait impl nor an inherent one. An `impl`
//! belongs to a type when its self type is a path that ends with the type's
//! name, and to a trait when its trait is; the `impl` must be in the item's
//! module unless no other item of the bundle defines that name.
//!
//! A variant that is never constructed can only appear in patterns. It goes
//! with each alternative of a `match` arm or of a `matches!` that names it
//! (`Enum::Variant`, or `Self::Variant` in an `impl` of the enum): such an
//! alternative never matches. An arm with no alternative left goes; a
//! `matches!` with none left is `false` when what it tests is a plain path,
//! which has no effect when read. Other uses (`if let`, `let`) are not
//! rewritten: the removal does not compile and the variant stays.
//!
//! The variants that stay keep their discriminants. An enum without fields
//! can be cast with `as`, so its first variant after a removed one is given
//! its old number (`C = 2`). An enum with fields cannot be cast; its
//! variants are offered unless it hashes its discriminant (a derived or
//! written `Hash`, or `discriminant` anywhere in the bundle). An enum is not
//! offered when another item has its name, when a `use` imports its
//! variants (a removed variant would become a binding in a pattern), when it
//! has a `repr` or a discriminant of its own, or when it is not read
//! clearly. An enum keeps at least one variant.

use std::collections::{BTreeMap, BTreeSet};

use super::{is_punct, match_forward, scan, start_of_item};
use crate::minify::{Kind, Token};

/// An `impl` block and the names it is for.
#[derive(Debug)]
pub struct Impl {
    /// Its first token (attributes included) and its closing `}`.
    pub first: usize,
    pub close: usize,
    pub open: usize,
    /// The last name of the trait's path, for a trait impl.
    pub trait_name: Option<String>,
    /// The last name of the self type's path, when the self type is a path
    /// that is not one of the impl's generic parameters.
    pub self_name: Option<String>,
}

/// An enum whose variants can be removed.
#[derive(Debug)]
pub struct Enum {
    pub variants: Vec<Variant>,
    /// Whether no variant has fields, so that the enum can be cast.
    pub fieldless: bool,
}

/// A variant: its name and its first (attributes included) and last
/// tokens, without the comma after it.
#[derive(Debug)]
pub struct Variant {
    pub name: usize,
    pub first: usize,
    pub last: usize,
}

/// A pattern made of alternatives, in a `match` arm or a `matches!`.
#[derive(Debug)]
pub struct Pattern {
    pub kind: PatternKind,
    /// The first and last tokens of each alternative.
    pub alts: Vec<(usize, usize)>,
    /// For each alternative, the names of the offered variants it names.
    pub refs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
pub enum PatternKind {
    /// An arm, from its first token to its comma or its last token.
    Arm { first: usize, last: usize },
    /// A `matches!` from its name to its `)`; `plain` when what it tests is
    /// a path of names and fields, which can be dropped.
    Matches {
        first: usize,
        last: usize,
        plain: bool,
    },
}

fn is_ident(token: &Token, text: &str) -> bool {
    token.kind == Kind::Ident && token.text == text
}

/// The index after the `>` that closes the `<` at `open`.
fn skip_angles(tokens: &[Token], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        if token.kind != Kind::Punct {
            continue;
        }
        match token.text.as_str() {
            "<" => depth += 1,
            "<<" => depth += 2,
            ">" => depth -= 1,
            ">>" => depth -= 2,
            "{" | "}" | ";" => return None,
            _ => {}
        }
        if depth == 0 {
            return Some(i + 1);
        }
        if depth < 0 {
            return None;
        }
    }
    None
}

/// The path that starts at `at` (`a::b::C<T>`): the index after it and its
/// last name.
fn path(tokens: &[Token], at: usize) -> Option<(usize, String)> {
    let mut i = at;
    if tokens.get(i).is_some_and(|t| is_punct(t, "::")) {
        i += 1;
    }
    loop {
        let token = tokens.get(i)?;
        if token.kind != Kind::Ident || matches!(token.text.as_str(), "dyn" | "impl" | "for") {
            return None;
        }
        let name = token.text.clone();
        i += 1;
        if tokens.get(i).is_some_and(|t| is_punct(t, "<")) {
            i = skip_angles(tokens, i)?;
        }
        if tokens.get(i).is_some_and(|t| is_punct(t, "::")) {
            i += 1;
        } else {
            return Some((i, name));
        }
    }
}

/// The names that the `impl` whose keyword is at `keyword` and whose body
/// opens at `open` is for: its trait's and its self type's.
pub fn impl_names(
    tokens: &[Token],
    keyword: usize,
    open: usize,
) -> (Option<String>, Option<String>) {
    let mut i = keyword + 1;
    let mut generics = BTreeSet::new();
    if tokens.get(i).is_some_and(|t| is_punct(t, "<")) {
        let Some(after) = skip_angles(tokens, i) else {
            return (None, None);
        };
        // The parameters: the names after `<` or after a `,` at the first
        // level.
        let mut depth = 0i32;
        let mut expect = false;
        for t in &tokens[i..after] {
            if t.kind == Kind::Punct {
                match t.text.as_str() {
                    "<" | "(" | "[" => {
                        depth += 1;
                        expect = depth == 1;
                        continue;
                    }
                    ">" | ")" | "]" => depth -= 1,
                    ">>" => depth -= 2,
                    "," if depth == 1 => {
                        expect = true;
                        continue;
                    }
                    _ => {}
                }
                expect = false;
            } else if expect && !is_ident(t, "const") {
                generics.insert(t.text.clone());
                expect = false;
            }
        }
        i = after;
    }
    let ends_header =
        |at: usize| at == open || tokens.get(at).is_some_and(|t| is_ident(t, "where"));
    let Some((after, first)) = path(tokens, i) else {
        return (None, None);
    };
    if tokens.get(after).is_some_and(|t| is_ident(t, "for")) {
        let mut j = after + 1;
        while tokens.get(j).is_some_and(|t| {
            is_punct(t, "&") || is_punct(t, "&&") || t.kind == Kind::Lifetime || is_ident(t, "mut")
        }) {
            j += 1;
        }
        let self_name = path(tokens, j)
            .filter(|(end, name)| ends_header(*end) && !generics.contains(name))
            .map(|(_, name)| name);
        (Some(first), self_name)
    } else if ends_header(after) && !generics.contains(&first) {
        (None, Some(first))
    } else {
        (None, None)
    }
}

/// The enum whose keyword is at `keyword`, if its variants are read
/// clearly, with whether it has a `repr`, a derived `Hash` or a
/// discriminant of its own.
pub fn parse_enum(tokens: &[Token], keyword: usize) -> Option<(Enum, bool)> {
    let name = keyword + 1;
    if tokens.get(name)?.kind != Kind::Ident {
        return None;
    }
    let open = scan(tokens, name + 1, &["{"])?;
    let close = match_forward(tokens, open)?;
    let first = start_of_item(tokens, keyword);
    let attributes = &tokens[first..keyword];
    let repr = attributes.iter().any(|t| is_ident(t, "repr"));
    let hash = attributes.iter().any(|t| is_ident(t, "Hash"));
    let mut variants = Vec::new();
    let mut fieldless = true;
    let mut explicit = false;
    let mut pos = open + 1;
    while pos < close {
        let start = pos;
        while is_punct(&tokens[pos], "#") {
            let bracket = pos + 1;
            if !tokens.get(bracket).is_some_and(|t| is_punct(t, "[")) {
                return None;
            }
            pos = match_forward(tokens, bracket)? + 1;
        }
        if tokens[pos].kind != Kind::Ident {
            return None;
        }
        let variant = pos;
        pos += 1;
        if is_punct(&tokens[pos], "(") || is_punct(&tokens[pos], "{") {
            fieldless = false;
            pos = match_forward(tokens, pos)? + 1;
        }
        if is_punct(&tokens[pos], "=") {
            explicit = true;
            pos = scan(tokens, pos, &[","]).unwrap_or(close);
        }
        variants.push(Variant {
            name: variant,
            first: start,
            last: pos - 1,
        });
        if pos < close {
            if !is_punct(&tokens[pos], ",") {
                return None;
            }
            pos += 1;
        }
    }
    let refused = repr || explicit || (hash && !fieldless);
    Some((
        Enum {
            variants,
            fieldless,
        },
        refused,
    ))
}

/// The alternatives of the pattern in tokens `lo..hi`, after a leading `|`.
fn alternatives(tokens: &[Token], lo: usize, hi: usize) -> Option<Vec<(usize, usize)>> {
    let mut start = lo;
    if start < hi && is_punct(&tokens[start], "|") {
        start += 1;
    }
    let mut alts = Vec::new();
    let mut depth = 0usize;
    let mut from = start;
    for i in start..hi {
        let t = &tokens[i];
        if t.kind != Kind::Punct {
            continue;
        }
        match t.text.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth = depth.checked_sub(1)?,
            "|" if depth == 0 => {
                if i == from {
                    return None;
                }
                alts.push((from, i - 1));
                from = i + 1;
            }
            ";" | "=" | "||" => return None,
            "," if depth == 0 => return None,
            _ => {}
        }
    }
    if from >= hi || depth != 0 {
        return None;
    }
    alts.push((from, hi - 1));
    Some(alts)
}

/// The first `if` at the first level in tokens `lo..hi`, or `hi`.
fn before_guard(tokens: &[Token], lo: usize, hi: usize) -> usize {
    let mut depth = 0usize;
    for i in lo..hi {
        let t = &tokens[i];
        if t.kind == Kind::Punct {
            match t.text.as_str() {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => depth = depth.saturating_sub(1),
                _ => {}
            }
        } else if depth == 0 && is_ident(t, "if") {
            return i;
        }
    }
    hi
}

/// The arms of the `match` whose keyword is at `keyword`, or nothing if one
/// is not read clearly.
fn parse_match(tokens: &[Token], keyword: usize) -> Option<Vec<Pattern>> {
    let open = scan(tokens, keyword + 1, &["{"])?;
    let close = match_forward(tokens, open)?;
    let mut arms = Vec::new();
    let mut pos = open + 1;
    while pos < close {
        let arrow = scan(tokens, pos, &["=>"]).filter(|&at| at < close)?;
        let alts = alternatives(tokens, pos, before_guard(tokens, pos, arrow))?;
        let body = arrow + 1;
        let last = if is_punct(tokens.get(body)?, "{") {
            let end = match_forward(tokens, body)?;
            if is_punct(&tokens[end + 1], ",") {
                end + 1
            } else {
                end
            }
        } else {
            match scan(tokens, body, &[",", "=>"]) {
                Some(at) if is_punct(&tokens[at], ",") && at < close => at,
                // The last arm, without a comma.
                None => close - 1,
                _ => return None,
            }
        };
        arms.push(Pattern {
            kind: PatternKind::Arm { first: pos, last },
            refs: vec![BTreeSet::new(); alts.len()],
            alts,
        });
        pos = last + 1;
    }
    Some(arms)
}

/// The pattern of the `matches!` whose name is at `name`.
fn parse_matches(tokens: &[Token], name: usize) -> Option<Pattern> {
    if !tokens.get(name + 1).is_some_and(|t| is_punct(t, "!")) {
        return None;
    }
    let open = name + 2;
    if !tokens.get(open).is_some_and(|t| is_punct(t, "(")) {
        return None;
    }
    let close = match_forward(tokens, open)?;
    let comma = scan(tokens, open + 1, &[","]).filter(|&at| at < close)?;
    let mut end = before_guard(tokens, comma + 1, close);
    if end == close && is_punct(&tokens[close - 1], ",") {
        end = close - 1;
    }
    let alts = alternatives(tokens, comma + 1, end)?;
    let plain = comma > open + 1
        && tokens[open + 1..comma].iter().all(|t| {
            t.kind == Kind::Ident || is_punct(t, ".") || is_punct(t, "*") || is_punct(t, "&")
        });
    Some(Pattern {
        kind: PatternKind::Matches {
            first: name,
            last: close,
            plain,
        },
        refs: vec![BTreeSet::new(); alts.len()],
        alts,
    })
}

/// Every pattern of `match` arms and `matches!` that is read clearly.
pub fn patterns(tokens: &[Token]) -> Vec<Pattern> {
    let mut out = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if is_ident(token, "match") {
            out.extend(parse_match(tokens, i).unwrap_or_default());
        } else if is_ident(token, "matches") {
            out.extend(parse_matches(tokens, i));
        }
    }
    out
}

/// Fills the `refs` of `patterns`: the variants of `enums` (by the token of
/// the enum's name) that each alternative names. `impls` resolve `Self`.
pub fn resolve(
    tokens: &[Token],
    patterns: &mut [Pattern],
    enums: &BTreeMap<usize, Enum>,
    impls: &[Impl],
) {
    let mut by_name: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    for (&name, e) in enums {
        let variants = by_name.entry(tokens[name].text.as_str()).or_default();
        for v in &e.variants {
            variants.insert(tokens[v.name].text.as_str(), v.name);
        }
    }
    let self_at = |at: usize| -> Option<&str> {
        impls
            .iter()
            .filter(|b| b.open < at && at < b.close)
            .max_by_key(|b| b.open)
            .and_then(|b| b.self_name.as_deref())
    };
    for pattern in patterns {
        for (k, &(lo, hi)) in pattern.alts.iter().enumerate() {
            for i in lo..hi.saturating_sub(1) {
                let ends = tokens.get(i + 3).is_none_or(|t| !is_punct(t, "::"));
                if !is_punct(&tokens[i + 1], "::") || tokens[i + 2].kind != Kind::Ident || !ends {
                    continue;
                }
                let head = tokens[i].text.as_str();
                let owner = if is_ident(&tokens[i], "Self") {
                    self_at(i)
                } else if tokens[i].kind == Kind::Ident {
                    Some(head)
                } else {
                    None
                };
                if let Some(&variant) = owner
                    .and_then(|owner| by_name.get(owner))
                    .and_then(|variants| variants.get(tokens[i + 2].text.as_str()))
                {
                    pattern.refs[k].insert(variant);
                }
            }
        }
    }
}
