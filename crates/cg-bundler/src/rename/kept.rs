//! The names that the renaming keeps, everywhere or at some places (see the
//! documentation of [`super`]).

use std::collections::{BTreeMap, BTreeSet};

use super::{captures, group_end, is_fixed, StdNames, OTHERS, TEXT_MACROS};
use crate::minify::{Kind, Token};

/// What the renaming leaves as it is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Kept {
    /// Names never renamed.
    pub names: BTreeSet<String>,
    /// The indices of tokens whose name stays though it may be renamed
    /// elsewhere: the middle segments of std paths and use trees (`cell` in
    /// `std::cell::RefCell`), which only std's modules can mean, and the
    /// member places of a name that std defines as a member (`next` in
    /// `it.next()`, `fn next`, a field `next: u8`), where std's member may
    /// be meant.
    pub positions: BTreeSet<usize>,
    /// The field shorthands of those member names (`Link { next }`), each
    /// with the index of the token where `next:` goes when the name is
    /// renamed (`Link { next: a }`): the name itself, or the `ref` or `mut`
    /// before it.
    pub shorthands: BTreeMap<usize, usize>,
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
    // The places where a name may mean a member of std.
    let mut members = Vec::new();
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
    // The format strings whose captured names may become arguments: their
    // other braced words are kept.
    let calls: BTreeMap<usize, captures::Call> = captures::calls(tokens)
        .into_iter()
        .map(|call| (call.string, call))
        .collect();

    for (at, token) in tokens.iter().enumerate() {
        let name = token.text.as_str();
        match token.kind {
            Kind::Str => {
                let text = match calls.get(&at) {
                    Some(call) => call.string_text(name, |_| true),
                    None => name.to_string(),
                };
                for word in super::braced_words(&text) {
                    kept.names.insert(word.to_string());
                }
            }
            Kind::Ident => {
                let before = at.checked_sub(1).and_then(text);
                if is_fixed(name, std)
                    || (text(at + 1) == Some("!") && (!macros.contains(name) || std.has(name)))
                {
                    kept.names.insert(name.to_string());
                } else if std.member(name)
                    && (matches!(before, Some("." | "::" | "fn" | "type"))
                        || (matches!(before, Some("<" | ",")) && text(at + 1) == Some("=")))
                {
                    members.push(at);
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
    // The names that a use tree of such a path imports, used bare.
    let mut imported = BTreeSet::new();
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
                            keep(&mut imported, at + 2, end);
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
    // A member of std after `::` (`from` in `u64::from`) stays at its member
    // places only, below; what a use tree imports stays everywhere.
    kept.names.extend(
        chained
            .into_iter()
            .filter(|name| imported.contains(name) || !std.member(name)),
    );
    // A free `fn` or `type` is no member: it is renamed with its bare uses.
    let fields = Fields::of(tokens);
    members.retain(|&at| !matches!(text(at - 1), Some("fn" | "type")) || fields.in_impl(at));
    // A name that std defines as a member keeps it at its member places
    // and at its field places, and may be renamed elsewhere: a local
    // `next` is not `it.next()`.
    let split: BTreeSet<&str> = members
        .iter()
        .map(|&at| tokens[at].text.as_str())
        .filter(|name| !kept.names.contains(*name))
        .collect();
    kept.positions.extend(
        members
            .iter()
            .copied()
            .filter(|&at| split.contains(tokens[at].text.as_str())),
    );
    for (at, token) in tokens.iter().enumerate() {
        if token.kind != Kind::Ident || !split.contains(token.text.as_str()) {
            continue;
        }
        if fields.is_field(at) {
            kept.positions.insert(at);
        } else if let Some(start) = fields.shorthand(at) {
            kept.shorthands.insert(at, start);
        }
    }
    // A position matters only for a name renamed elsewhere.
    let names = &kept.names;
    kept.positions
        .retain(|&at| !names.contains(&tokens[at].text));
    kept
}

/// Where the fields of structs are named: each token's innermost group.
struct Fields<'a> {
    tokens: &'a [Token],
    /// The index of the `(`, `[` or `{` of each token's innermost group.
    parent: Vec<Option<usize>>,
    /// The index of the opening token of each group's closing token.
    opener: BTreeMap<usize, usize>,
}

impl<'a> Fields<'a> {
    fn of(tokens: &'a [Token]) -> Fields<'a> {
        let mut parent = Vec::with_capacity(tokens.len());
        let mut opener = BTreeMap::new();
        let mut stack: Vec<usize> = Vec::new();
        for (at, token) in tokens.iter().enumerate() {
            match token.text.as_str() {
                ")" | "]" | "}" if token.kind == Kind::Punct => {
                    let open = stack.pop();
                    if let Some(open) = open {
                        opener.insert(at, open);
                    }
                    parent.push(stack.last().copied());
                }
                text => {
                    parent.push(stack.last().copied());
                    if token.kind == Kind::Punct && matches!(text, "(" | "[" | "{") {
                        stack.push(at);
                    }
                }
            }
        }
        Fields {
            tokens,
            parent,
            opener,
        }
    }

    fn text(&self, at: usize) -> Option<&str> {
        self.tokens.get(at).map(|t| t.text.as_str())
    }

    /// Whether the token at `at` is in braces.
    fn in_braces(&self, at: usize) -> bool {
        self.parent[at].is_some_and(|open| self.text(open) == Some("{"))
    }

    /// Whether the token at `at` is directly in the braces of an `impl` or
    /// a `trait`: the item it names is a member. The header of the braces
    /// runs back to the end of the item before them.
    fn in_impl(&self, at: usize) -> bool {
        let Some(open) = self.parent[at].filter(|&o| self.text(o) == Some("{")) else {
            return false;
        };
        let mut i = open;
        while i > 0 {
            i -= 1;
            if self.parent[i] != self.parent[open] {
                continue;
            }
            match self.text(i) {
                Some("impl" | "trait") => return true,
                Some(";" | "{" | "}") | None => return false,
                _ => {}
            }
        }
        false
    }

    /// Whether the name at `at` names a field: `next` in `struct Link {
    /// pub next: u8 }`, `Link { next: 1 }` or `Link { next: n, .. }`, a
    /// name after `{` or a comma in braces, and before one `:` (not `::`).
    /// Nothing else in braces is a name and a colon there: a block holds
    /// no such statement.
    fn is_field(&self, at: usize) -> bool {
        if self.text(at + 1) != Some(":") || !self.in_braces(at) {
            return false;
        }
        let mut before = at.checked_sub(1);
        // A visibility: `pub` or `pub(crate)`.
        if let Some(b) = before {
            if self.text(b) == Some(")") {
                let open = self.opener.get(&b).copied();
                if let Some(open) = open.filter(|&o| o > 0 && self.text(o - 1) == Some("pub")) {
                    before = open.checked_sub(2);
                }
            } else if self.text(b) == Some("pub") {
                before = b.checked_sub(1);
            }
        }
        before.is_some_and(|b| matches!(self.text(b), Some("{" | ",")))
    }

    /// When the name at `at` may be a field shorthand, `next` in `Link {
    /// next }` or `Link { ref mut next, .. }`, the index where `next:` goes
    /// to name the field: the name or its first `ref` or `mut`. The braces
    /// follow a `CamelCase` name or `Self` that is not a type being defined
    /// or returned (`enum Kind { Next }`, `-> Link { next }`). A guess:
    /// rustc's rejection undoes a wrong one.
    fn shorthand(&self, at: usize) -> Option<usize> {
        if !matches!(self.text(at + 1), Some("," | "}")) || !self.in_braces(at) {
            return None;
        }
        let mut start = at;
        while start > 0 && matches!(self.text(start - 1), Some("ref" | "mut")) {
            start -= 1;
        }
        if !matches!(self.text(start.checked_sub(1)?), Some("{" | ",")) {
            return None;
        }
        let open = self.parent[at]?;
        let ty = self.tokens.get(open.checked_sub(1)?)?;
        let names_a_struct = ty.kind == Kind::Ident
            && (ty.text == "Self" || super::Case::of(&ty.text) == super::Case::Camel);
        let defined = open
            .checked_sub(2)
            .and_then(|b| self.text(b))
            .is_some_and(|b| {
                matches!(
                    b,
                    "struct" | "enum" | "union" | "trait" | "impl" | "for" | "dyn" | "->" | "+"
                )
            });
        (names_a_struct && !defined).then_some(start)
    }
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
