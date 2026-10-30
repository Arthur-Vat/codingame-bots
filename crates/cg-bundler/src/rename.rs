//! Gives the names the bundle defines the shortest names not otherwise used
//! in it (ADR 0045, level 3).
//!
//! The renaming works on tokens ([`crate::minify::tokens`]) and replaces a
//! name everywhere at once: every identifier with the text `score` becomes
//! `a`, wherever it is, and no other token changes. Since the new name is in
//! no token of the bundle and in no name the standard library defines, the
//! renamed bundle is the same program, with other names, unless some code
//! outside the bundle, std or the compiler, reads the old name. A name is
//! therefore kept as it is when it is:
//!
//! - a keyword, a primitive type, `main`, a name that starts with `_` (rustc
//!   reads that as "unused on purpose"), a raw identifier (`r#type`), a
//!   macro fragment specifier (`expr`, `ident`...), the name of a std crate,
//!   or a name of a std prelude;
//! - a macro called with `!`, unless the bundle defines it with
//!   `macro_rules!` and std defines no name like it;
//! - after `.`, `::`, `fn` or `type`, where std defines a member of that name
//!   that code outside std can reach (a `pub` `fn`, `const`, `type` or
//!   `static`, an item of a trait or a trait impl, or a `pub` field; rustc's
//!   method lookup skips the others): `x.len()`,
//!   `u32::MAX`, `fn fmt` in an `impl Display`, `type Item`. Wherever it
//!   is, such a name may mean std's member, and renaming the bundle's own
//!   would make the call reach the bundle's instead (a trait of the bundle
//!   implemented for a std type) without an error; also in an associated
//!   type binding (`Iterator<Item = u8>`);
//! - in a path that starts with `std::`, `core::` or `alloc::`, `use` trees
//!   included, or after `::` behind such a name, a primitive type or a
//!   prelude name (`io::stdin` after `use std::io`, `Ordering::Less`,
//!   `Vec::with_capacity`, `u32::MAX`);
//! - in an attribute (`#[derive(Clone)]`, `#[allow(dead_code)]`);
//! - in braces in a string literal, where formatting macros read names
//!   (`"{score}"`, `"{:width$}"`), or in the arguments of `stringify!`,
//!   `concat!` and `module_path!`, which turn names into text.
//!
//! The std names are in `rename/std-names.txt`, written from the sources of
//! rustc 1.90.0's standard library by `cargo run -p cg-bundler --example
//! std-names`: the names that std, core and alloc define, with their kinds.
//!
//! The renamed bundle is compiled with rustc 1.90.0 ([`crate::prune`]'s
//! compiler). A renaming that does not compile is undone: the names in the
//! errors' spans are kept, and the rest is tried again; if the errors name
//! none, the renamings are halved until those that rustc rejects are found.
//! What compiles is the same program, since only the names changed. Two
//! things can still see a new name without an error, and neither changes
//! what a bot plays: the text of `{:?}` for a type that derives `Debug`
//! (bots write it to stderr only) and of `std::any::type_name` (in error
//! messages). The answers check of ADR 0045 (decision 4) would catch a bot
//! whose answers change.
//!
//! The names that occur most get the shortest new names. A new name keeps
//! the case of the old one, so rustc warns about nothing new:
//! `snake_case` names get lower-case names (`a`, `b`, ... `a0`...),
//! `CamelCase` names get `A`, `B`, ... `Aa`..., and `UPPER_CASE` names get
//! `A`, `B`, ... `AA`.... A name is renamed only when the new one is
//! shorter. The result depends only on the bundle and on rustc 1.90.0.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::minify::{tokens, Kind, Token};
use crate::prune::{Compiler, Outcome, Rustc};

/// The names of the standard library, written by the example `std-names`.
const STD_NAMES: &str = include_str!("rename/std-names.txt");

/// Error spans longer than this say nothing about one name.
const NAME_SPAN: usize = 64;

/// What a renaming did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Names given a shorter name.
    pub renamed: usize,
    /// Names whose renaming rustc rejected, kept as they were.
    pub refused: usize,
    /// Runs of rustc.
    pub compiles: usize,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "renamed {} names, {} renamings that do not compile undone, {} compiles",
            self.renamed, self.refused, self.compiles
        )
    }
}

/// The names that the standard library defines, with their kinds.
pub struct StdNames {
    /// Each name and its kinds: some of `i` (an item), `m` (a member) and
    /// `p` (in a prelude).
    kinds: BTreeMap<&'static str, &'static str>,
}

impl StdNames {
    /// The names of rustc 1.90.0's standard library.
    pub fn get() -> &'static StdNames {
        static NAMES: OnceLock<StdNames> = OnceLock::new();
        NAMES.get_or_init(|| StdNames::parse(STD_NAMES).expect("rename/std-names.txt is valid"))
    }

    /// Reads the list: lines `kinds name`, and comments that start with `#`.
    pub fn parse(text: &'static str) -> Result<StdNames, String> {
        let mut kinds = BTreeMap::new();
        for (at, line) in text.lines().enumerate() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let (kind, name) = line
                .split_once(' ')
                .filter(|(kind, name)| {
                    !kind.is_empty()
                        && kind.chars().all(|c| matches!(c, 'i' | 'm' | 'p'))
                        && !name.is_empty()
                        && !name.contains(' ')
                })
                .ok_or_else(|| format!("line {}: not `kinds name`: {line:?}", at + 1))?;
            if kinds.insert(name, kind).is_some() {
                return Err(format!("line {}: {name} is listed twice", at + 1));
            }
        }
        Ok(StdNames { kinds })
    }

    /// Whether std defines `name` in any way.
    pub fn has(&self, name: &str) -> bool {
        self.kinds.contains_key(name)
    }

    /// Whether std has a member `name` that code outside std can reach: a
    /// `pub` `fn`, `const`, `type` or `static`, an item of a trait or a
    /// trait impl, or a `pub` field.
    pub fn member(&self, name: &str) -> bool {
        self.kinds
            .get(name)
            .is_some_and(|kinds| kinds.contains('m'))
    }

    /// Whether `name` is in a prelude of std.
    pub fn prelude(&self, name: &str) -> bool {
        self.kinds
            .get(name)
            .is_some_and(|kinds| kinds.contains('p'))
    }
}

/// `source` with the names it defines renamed, and what was done. It fails
/// if `source` cannot be lexed or rustc cannot be run.
pub fn rename(source: &str, rustc: &Rustc) -> Result<(String, Report), String> {
    let tokens = tokens(source).map_err(|err| format!("cannot read the bundle: {err}"))?;
    let std = StdNames::get();
    let mut candidates = candidates(&tokens, std);
    let mut compiler = Compiler::new(rustc)?;
    let mut refused = 0;
    loop {
        let mapping = assign(&tokens, &candidates, std);
        if mapping.is_empty() {
            return Ok((
                source.to_string(),
                Report {
                    renamed: 0,
                    refused,
                    compiles: compiler.compiles(),
                },
            ));
        }
        let text = apply(source, &tokens, &mapping);
        let spans = match compiler.compile(&text)? {
            Outcome::Compiled(_) => {
                return Ok((
                    text,
                    Report {
                        renamed: mapping.len(),
                        refused,
                        compiles: compiler.compiles(),
                    },
                ))
            }
            Outcome::Failed { spans, .. } => spans,
        };
        let mut blamed = blame(&text, &spans, &mapping);
        if blamed.is_empty() {
            // The errors name no renamed name: halve the renamings until
            // those that do not compile are found.
            let names: Vec<&str> = candidates
                .iter()
                .map(String::as_str)
                .filter(|name| mapping.contains_key(*name))
                .collect();
            let mut accepted = Vec::new();
            settle(
                &mut compiler,
                source,
                &tokens,
                &mapping,
                &mut accepted,
                &names,
                true,
            )?;
            let accepted: BTreeSet<&str> = accepted.into_iter().collect();
            blamed = names
                .into_iter()
                .filter(|name| !accepted.contains(name))
                .map(str::to_string)
                .collect();
        }
        let before = candidates.len();
        candidates.retain(|name| !blamed.contains(name));
        refused += before - candidates.len();
        if candidates.len() == before {
            // Nothing renamed is to blame, yet it fails: rename nothing.
            candidates.clear();
        }
    }
}

/// Adds to `accepted` as many of `names` as compile when renamed by
/// `mapping` together with `accepted`, halving them until the renamings
/// that do not compile are found. `failing` says that renaming all of them
/// is known not to compile.
fn settle<'n>(
    compiler: &mut Compiler<'_>,
    source: &str,
    tokens: &[Token],
    mapping: &BTreeMap<String, String>,
    accepted: &mut Vec<&'n str>,
    names: &[&'n str],
    failing: bool,
) -> Result<(), String> {
    if names.is_empty() {
        return Ok(());
    }
    if !failing {
        let trial: BTreeMap<String, String> = mapping
            .iter()
            .filter(|(old, _)| accepted.contains(&old.as_str()) || names.contains(&old.as_str()))
            .map(|(old, new)| (old.clone(), new.clone()))
            .collect();
        if let Outcome::Compiled(_) = compiler.compile(&apply(source, tokens, &trial))? {
            accepted.extend(names);
            return Ok(());
        }
    }
    if names.len() == 1 {
        return Ok(());
    }
    let (left, right) = names.split_at(names.len() / 2);
    let before = accepted.len();
    settle(compiler, source, tokens, mapping, accepted, left, false)?;
    // If all of the left half went, the right half is what failed.
    let left_went = accepted.len() == before + left.len();
    settle(
        compiler, source, tokens, mapping, accepted, right, left_went,
    )
}

/// The old names whose new names, or themselves, are in the error spans of
/// the compile of `text`.
fn blame(
    text: &str,
    spans: &[(usize, usize)],
    mapping: &BTreeMap<String, String>,
) -> BTreeSet<String> {
    let old_of: BTreeMap<&str, &str> = mapping
        .iter()
        .map(|(old, new)| (new.as_str(), old.as_str()))
        .collect();
    let mut blamed = BTreeSet::new();
    for &(start, end) in spans {
        if end - start > NAME_SPAN {
            continue;
        }
        let Some(snippet) = text.get(start..end) else {
            continue;
        };
        for token in tokens(snippet).unwrap_or_default() {
            if token.kind != Kind::Ident {
                continue;
            }
            if let Some(old) = old_of.get(token.text.as_str()) {
                blamed.insert(old.to_string());
            } else if mapping.contains_key(&token.text) {
                blamed.insert(token.text);
            }
        }
    }
    blamed
}

/// The names that may be renamed, most frequent first (ties in the order of
/// the names).
pub fn candidates(tokens: &[Token], std: &StdNames) -> Vec<String> {
    let kept = kept(tokens, std);
    let counts = counts(tokens);
    let mut names: Vec<&str> = counts
        .keys()
        .copied()
        .filter(|name| !kept.contains(*name))
        .collect();
    names.sort_by_key(|name| (std::cmp::Reverse(counts[name]), *name));
    names.into_iter().map(str::to_string).collect()
}

/// How many identifier tokens have each text.
fn counts(tokens: &[Token]) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for token in tokens.iter().filter(|t| t.kind == Kind::Ident) {
        *counts.entry(token.text.as_str()).or_insert(0) += 1;
    }
    counts
}

/// Gives the `candidates`, in their order, the next unused name of their
/// case, when it is shorter: the old name and the new for each.
pub fn assign(tokens: &[Token], candidates: &[String], std: &StdNames) -> BTreeMap<String, String> {
    let mut taken: BTreeSet<String> = tokens
        .iter()
        .filter(|t| t.kind == Kind::Ident)
        .map(|t| t.text.clone())
        .collect();
    let mut next = [0usize; 3];
    let mut mapping = BTreeMap::new();
    for name in candidates {
        let case = Case::of(name);
        let fresh = loop {
            let fresh = case.name(next[case as usize]);
            if taken.contains(&fresh) || std.has(&fresh) || is_fixed(&fresh, std) {
                next[case as usize] += 1;
            } else {
                break fresh;
            }
        };
        if fresh.len() < name.len() {
            next[case as usize] += 1;
            taken.insert(fresh.clone());
            mapping.insert(name.clone(), fresh);
        }
    }
    mapping
}

/// `source` with each identifier token in `mapping` replaced by its new
/// name; everything else, comments and whitespace included, stays.
pub fn apply(source: &str, tokens: &[Token], mapping: &BTreeMap<String, String>) -> String {
    let mut out = String::with_capacity(source.len());
    let mut copied = 0;
    for token in tokens.iter().filter(|t| t.kind == Kind::Ident) {
        if let Some(new) = mapping.get(&token.text) {
            out.push_str(&source[copied..token.start]);
            out.push_str(new);
            copied = token.end;
        }
    }
    out.push_str(&source[copied..]);
    out
}

/// The case of a name, which its new name keeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    /// Starts with a lower-case letter (`snake_case`).
    Lower = 0,
    /// Starts with an upper-case letter and has a lower-case one
    /// (`CamelCase`).
    Camel = 1,
    /// Starts with an upper-case letter and has no lower-case one
    /// (`UPPER_CASE`).
    Upper = 2,
}

impl Case {
    pub fn of(name: &str) -> Case {
        let mut chars = name.chars();
        match chars.next() {
            Some(first) if first.is_uppercase() => {
                if name.chars().any(char::is_lowercase) {
                    Case::Camel
                } else {
                    Case::Upper
                }
            }
            _ => Case::Lower,
        }
    }

    /// The `index`-th name of this case, shortest first: one letter, then
    /// a letter and one letter or digit, and so on.
    pub fn name(self, index: usize) -> String {
        const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let first = if self == Case::Lower { LOWER } else { UPPER };
        let rest: Vec<u8> = match self {
            Case::Upper => UPPER.iter().chain(b"0123456789").copied().collect(),
            _ => LOWER.iter().chain(b"0123456789").copied().collect(),
        };
        // Find the length: 26 names of one character, 26 * 36 of two...
        let mut index = index;
        let mut count = first.len();
        let mut length = 1;
        while index >= count {
            index -= count;
            count *= rest.len();
            length += 1;
        }
        let mut tail = Vec::with_capacity(length - 1);
        for _ in 1..length {
            tail.push(rest[index % rest.len()]);
            index /= rest.len();
        }
        let mut name = vec![first[index]];
        name.extend(tail.iter().rev());
        String::from_utf8(name).expect("ASCII")
    }
}

/// Words of Rust that are not names to change: keywords, reserved words and
/// the weak keywords that can be names.
const KEYWORDS: &[&str] = &[
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "gen",
    "macro",
    "override",
    "priv",
    "try",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "union",
    "macro_rules",
    "raw",
    "safe",
    "auto",
    "default",
];

/// The primitive types, the std crates and the entry point.
const OTHERS: &[&str] = &[
    "bool", "char", "str", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
    "u128", "usize", "f16", "f32", "f64", "f128", "std", "core", "alloc", "main",
];

/// The fragment specifiers of `macro_rules!`.
const FRAGMENTS: &[&str] = &[
    "block",
    "expr",
    "expr_2021",
    "ident",
    "item",
    "lifetime",
    "literal",
    "meta",
    "pat",
    "pat_param",
    "path",
    "stmt",
    "tt",
    "ty",
    "vis",
];

/// Macros whose arguments become text.
const TEXT_MACROS: &[&str] = &["stringify", "concat", "module_path"];

/// Names never renamed, whatever the code around them.
fn is_fixed(name: &str, std: &StdNames) -> bool {
    KEYWORDS.contains(&name)
        || OTHERS.contains(&name)
        || FRAGMENTS.contains(&name)
        || name.starts_with('_')
        || name.starts_with("r#")
        || std.prelude(name)
}

/// The names that stay as they are (see the module's documentation).
pub fn kept(tokens: &[Token], std: &StdNames) -> BTreeSet<String> {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let is_ident = |at: usize| tokens.get(at).is_some_and(|t| t.kind == Kind::Ident);
    let mut kept = BTreeSet::new();
    // The names that may lead into std when `::` follows them: those of
    // paths that start with a std crate, and those found behind them.
    let mut std_paths = BTreeSet::new();
    let keep = |kept: &mut BTreeSet<String>, from: usize, to: usize| {
        for token in &tokens[from..to] {
            if token.kind == Kind::Ident {
                kept.insert(token.text.clone());
            }
        }
    };

    // The macros of the bundle.
    let macros: BTreeSet<&str> = (0..tokens.len())
        .filter(|&at| {
            text(at) == Some("macro_rules") && text(at + 1) == Some("!") && is_ident(at + 2)
        })
        .map(|at| tokens[at + 2].text.as_str())
        .collect();

    for (at, token) in tokens.iter().enumerate() {
        let name = token.text.as_str();
        match token.kind {
            Kind::Str => {
                for word in braced_words(name) {
                    kept.insert(word.to_string());
                }
            }
            Kind::Ident => {
                let before = at.checked_sub(1).and_then(text);
                if is_fixed(name, std)
                    || (text(at + 1) == Some("!") && (!macros.contains(name) || std.has(name)))
                    || (matches!(before, Some("." | "::" | "fn" | "type")) && std.member(name))
                    || (matches!(before, Some("<" | ","))
                        && text(at + 1) == Some("=")
                        && std.member(name))
                {
                    kept.insert(name.to_string());
                }
                if TEXT_MACROS.contains(&name) && text(at + 1) == Some("!") {
                    if let Some(end) = group_end(tokens, at + 2) {
                        keep(&mut kept, at + 2, end);
                    }
                }
                if matches!(name, "std" | "core" | "alloc")
                    && !matches!(before, Some("::" | "."))
                    && text(at + 1) == Some("::")
                {
                    let end = path_end(tokens, at);
                    keep(&mut std_paths, at, end);
                }
            }
            Kind::Punct if name == "#" => {
                let open = if text(at + 1) == Some("!") {
                    at + 2
                } else {
                    at + 1
                };
                if text(open) == Some("[") {
                    if let Some(end) = group_end(tokens, open) {
                        keep(&mut kept, open, end);
                    }
                }
            }
            _ => {}
        }
    }

    // A name after `::` behind a name that may lead into std is kept, and
    // may lead further, until nothing changes.
    loop {
        let before = std_paths.len();
        for at in 0..tokens.len() {
            let name = tokens[at].text.as_str();
            let leads = std_paths.contains(name) || std.prelude(name) || OTHERS.contains(&name);
            if tokens[at].kind != Kind::Ident || !leads || text(at + 1) != Some("::") {
                continue;
            }
            if is_ident(at + 2) {
                std_paths.insert(tokens[at + 2].text.clone());
            } else if text(at + 2) == Some("{") {
                if let Some(end) = group_end(tokens, at + 2) {
                    keep(&mut std_paths, at + 2, end);
                }
            }
        }
        if std_paths.len() == before {
            break;
        }
    }
    kept.extend(std_paths);
    kept
}

/// The index after the group that opens at `open` (`(`, `[` or `{`), or
/// `None` when no group opens there or it does not close.
fn group_end(tokens: &[Token], open: usize) -> Option<usize> {
    if !matches!(tokens.get(open)?.text.as_str(), "(" | "[" | "{") {
        return None;
    }
    let mut depth = 0usize;
    for (at, token) in tokens.iter().enumerate().skip(open) {
        match token.text.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth -= 1;
                if depth == 0 {
                    return Some(at + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// The index after the path that starts at `start`: names joined by `::`,
/// with a final `{...}` group (a `use` tree) included.
fn path_end(tokens: &[Token], start: usize) -> usize {
    let mut at = start;
    loop {
        // `at` is a name of the path.
        if tokens.get(at + 1).map(|t| t.text.as_str()) != Some("::") {
            return at + 1;
        }
        match tokens.get(at + 2) {
            Some(token) if token.kind == Kind::Ident => at += 2,
            Some(token) if token.text == "{" => {
                return group_end(tokens, at + 2).unwrap_or(at + 2);
            }
            _ => return at + 2,
        }
    }
}

/// The words in braces in a string literal's text: those that formatting
/// macros may read as names (`{score}`, `{0:width$}`).
fn braced_words(literal: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut rest = literal;
    while let Some(open) = rest.find('{') {
        let inside = &rest[open + 1..];
        let close = inside.find('}').unwrap_or(inside.len());
        let mut chars = inside[..close].char_indices().peekable();
        while let Some((start, c)) = chars.next() {
            if c.is_alphabetic() || c == '_' {
                let mut end = start + c.len_utf8();
                while let Some(&(at, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        end = at + c.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                words.push(&inside[start..end]);
            }
        }
        rest = &inside[close.min(inside.len())..];
    }
    words
}

#[cfg(test)]
mod tests;
