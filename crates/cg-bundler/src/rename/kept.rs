//! The names that the renaming keeps, everywhere or at some places (see the
//! documentation of [`super`]).

use std::collections::BTreeSet;

use super::{group_end, is_fixed, StdNames, OTHERS, TEXT_MACROS};
use crate::minify::{Kind, Token};

/// What the renaming leaves as it is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Kept {
    /// Names never renamed.
    pub names: BTreeSet<String>,
    /// The indices of tokens whose name stays though it may be renamed
    /// elsewhere: the middle segments of std paths and use trees (`cell` in
    /// `std::cell::RefCell`), which only std's modules can mean.
    pub positions: BTreeSet<usize>,
}

/// The names and places that stay as they are.
pub fn kept(tokens: &[Token], std: &StdNames) -> Kept {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let is_ident = |at: usize| tokens.get(at).is_some_and(|t| t.kind == Kind::Ident);
    let mut kept = Kept::default();
    // The names that std paths import or end with, kept everywhere, and the
    // names given with `as` to what a std path imports: both may lead into
    // std when `::` follows them.
    let mut std_names = BTreeSet::new();
    let mut aliases = BTreeSet::new();
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
    let text_macros = text_macros(tokens);

    for (at, token) in tokens.iter().enumerate() {
        let name = token.text.as_str();
        match token.kind {
            Kind::Str => {
                for word in super::braced_words(name) {
                    kept.names.insert(word.to_string());
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
                    kept.names.insert(name.to_string());
                }
                if text_macros.contains(name) && text(at + 1) == Some("!") {
                    if let Some(end) = group_end(tokens, at + 2) {
                        keep(&mut kept.names, at + 2, end);
                    }
                }
                if matches!(name, "std" | "core" | "alloc")
                    && !matches!(before, Some("::" | "."))
                    && text(at + 1) == Some("::")
                {
                    let in_use = before == Some("use");
                    let mut path = StdPath {
                        tokens,
                        names: &mut std_names,
                        positions: &mut kept.positions,
                        aliases: &mut aliases,
                        in_use,
                    };
                    path.walk(at);
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
                        keep(&mut kept.names, open, end);
                    }
                }
            }
            _ => {}
        }
    }

    // A path whose first name may lead into std keeps the names after its
    // `::`, which may lead further, until nothing changes. Only a path's
    // first name leads: `crate::board::cell::Cell` is the bundle's own path,
    // whatever `cell` is elsewhere.
    let mut chained = BTreeSet::new();
    loop {
        let before = chained.len();
        for at in 0..tokens.len() {
            let name = tokens[at].text.as_str();
            let first = !matches!(at.checked_sub(1).and_then(text), Some("::" | "."));
            let leads = std_names.contains(name)
                || aliases.contains(name)
                || chained.contains(name)
                || std.prelude(name)
                || OTHERS.contains(&name);
            if tokens[at].kind != Kind::Ident
                || !first
                || !leads
                || matches!(name, "std" | "core" | "alloc")
                || text(at + 1) != Some("::")
            {
                continue;
            }
            let mut at = at;
            while text(at + 1) == Some("::") {
                if is_ident(at + 2) {
                    chained.insert(tokens[at + 2].text.clone());
                    at += 2;
                } else {
                    if text(at + 2) == Some("{") {
                        if let Some(end) = group_end(tokens, at + 2) {
                            keep(&mut chained, at + 2, end);
                        }
                    }
                    break;
                }
            }
        }
        if chained.len() == before {
            break;
        }
    }

    // A glob import from std (`use std::cmp::Ordering::*`, `use io::*`
    // after `use std::io`) brings names that the bundle then uses bare: a
    // renamed variant in a pattern would become a binding that matches
    // anything. With one, every name that std defines is kept.
    let mut glob_from_std = false;
    for at in 0..tokens.len() {
        if text(at) != Some("use") {
            continue;
        }
        let first = if text(at + 1) == Some("::") {
            at + 2
        } else {
            at + 1
        };
        let mut end = first;
        while end < tokens.len() && text(end) != Some(";") {
            end += 1;
        }
        let from_std = text(first).is_some_and(|name| {
            std_names.contains(name)
                || aliases.contains(name)
                || chained.contains(name)
                || std.prelude(name)
                || OTHERS.contains(&name)
        });
        if from_std && (first..end).any(|i| text(i) == Some("*")) {
            glob_from_std = true;
        }
    }
    if glob_from_std {
        for token in tokens {
            if token.kind == Kind::Ident && std.has(&token.text) {
                kept.names.insert(token.text.clone());
            }
        }
    }
    kept.names.extend(std_names);
    kept.names.extend(chained);
    // A position matters only for a name renamed elsewhere.
    let names = &kept.names;
    kept.positions
        .retain(|&at| !names.contains(&tokens[at].text));
    kept
}

/// The macros whose arguments become text: those of std, and those of the
/// bundle that call one of them, until nothing changes.
pub fn text_macros(tokens: &[Token]) -> BTreeSet<&str> {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let is_ident = |at: usize| tokens.get(at).is_some_and(|t| t.kind == Kind::Ident);
    let mut text_macros: BTreeSet<&str> = TEXT_MACROS.iter().copied().collect();
    loop {
        let before = text_macros.len();
        for at in 0..tokens.len() {
            if text(at) != Some("macro_rules") || text(at + 1) != Some("!") || !is_ident(at + 2) {
                continue;
            }
            let Some(end) = group_end(tokens, at + 3) else {
                continue;
            };
            let calls_text = (at + 3..end).any(|i| {
                text(i).is_some_and(|n| text_macros.contains(n)) && text(i + 1) == Some("!")
            });
            if calls_text {
                text_macros.insert(tokens[at + 2].text.as_str());
            }
        }
        if text_macros.len() == before {
            return text_macros;
        }
    }
}

/// A walk along a path that starts with `std`, `core` or `alloc`.
struct StdPath<'a> {
    tokens: &'a [Token],
    /// The names the path imports or ends with.
    names: &'a mut BTreeSet<String>,
    /// The middle segments.
    positions: &'a mut BTreeSet<usize>,
    /// The names given with `as` in a `use`.
    aliases: &'a mut BTreeSet<String>,
    /// Whether the path is that of a `use` declaration.
    in_use: bool,
}

impl StdPath<'_> {
    fn text(&self, at: usize) -> Option<&str> {
        self.tokens.get(at).map(|t| t.text.as_str())
    }

    fn is_ident(&self, at: usize) -> bool {
        self.tokens.get(at).is_some_and(|t| t.kind == Kind::Ident)
    }

    /// Walks the path from its name at `at`.
    fn walk(&mut self, mut at: usize) {
        loop {
            let name = self.tokens[at].text.clone();
            if self.text(at + 1) != Some("::") {
                // The last segment.
                self.names.insert(name);
                if self.in_use && self.text(at + 1) == Some("as") && self.is_ident(at + 2) {
                    self.aliases.insert(self.tokens[at + 2].text.clone());
                }
                return;
            }
            if self.is_ident(at + 2) {
                self.positions.insert(at);
                at += 2;
                continue;
            }
            match self.text(at + 2) {
                Some("{") => self.tree(at, at + 2),
                Some("*") => {
                    self.positions.insert(at);
                }
                // `std::mem::size_of::<T>`: the name ends the path.
                _ => {
                    self.names.insert(name);
                }
            }
            return;
        }
    }

    /// Walks the use tree that opens at `open`, behind the segment at `at`.
    fn tree(&mut self, at: usize, open: usize) {
        let Some(end) = group_end(self.tokens, open) else {
            // Not a tree that closes: keep everything.
            for token in &self.tokens[at..] {
                if token.kind == Kind::Ident {
                    self.names.insert(token.text.clone());
                }
            }
            return;
        };
        // The start of each element: after the `{` and after each comma at
        // the tree's own depth.
        let mut starts = vec![open + 1];
        let mut i = open + 1;
        while i < end - 1 {
            match self.text(i) {
                Some("(" | "[" | "{") => {
                    i = group_end(self.tokens, i).unwrap_or(end - 1);
                    continue;
                }
                Some(",") => starts.push(i + 1),
                _ => {}
            }
            i += 1;
        }
        // `{self, ...}` imports the segment itself.
        let imports_self = starts.iter().any(|&s| self.text(s) == Some("self"));
        if imports_self {
            self.names.insert(self.tokens[at].text.clone());
        } else {
            self.positions.insert(at);
        }
        for start in starts {
            if start >= end - 1 {
                continue;
            }
            if self.text(start) == Some("self") {
                if self.text(start + 1) == Some("as") && self.is_ident(start + 2) {
                    self.aliases.insert(self.tokens[start + 2].text.clone());
                }
            } else if self.is_ident(start) {
                self.walk(start);
            } else if self.text(start) == Some("{") {
                // A tree in a tree, `std::{{a, b}}`: keep all of it.
                let close = group_end(self.tokens, start).unwrap_or(end);
                for token in &self.tokens[start..close] {
                    if token.kind == Kind::Ident {
                        self.names.insert(token.text.clone());
                    }
                }
            }
        }
    }
}
