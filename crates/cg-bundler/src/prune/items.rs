//! Where the removable items of a bundle are, found by reading its tokens,
//! and the text that results from removing some of them.
//!
//! rustc names an unused item by the span of its name. This module finds
//! the whole item from that name: back to its attributes and visibility,
//! forward to its closing `}` or `;`. An item whose extent is not clear is
//! not offered. The text between the previous token and the item (its doc
//! comments and the whitespace before it) goes with the item, and so do
//! the `impl`s and patterns of [`shapes`].

mod shapes;

use std::collections::{BTreeMap, BTreeSet};

use crate::minify::{tokens, Kind, Token};
use shapes::{Enum, Impl, Pattern, PatternKind};

/// A removal that can be tried. The numbers are token indexes in the
/// original text, so that an id means the same in every round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Id {
    /// A `fn`, `const`, `static`, `struct`, `enum`, `union`, `type` or
    /// `trait`; the index is that of its name.
    Item(usize),
    /// An inherent `impl` or a `mod` that has no item left; the index is
    /// that of the keyword.
    Container(usize),
    /// One name of a `use` declaration, or a `*`; the index is that of
    /// the first token of the name's tree.
    Leaf(usize),
    /// An enum's variant that is never constructed, with the alternatives
    /// of patterns that name it; the index is that of its name.
    Variant(usize),
    /// A `match` arm that rustc reports unreachable; the index is that of
    /// its first token.
    Arm(usize),
}

/// A part of the text of a rendering.
#[derive(Clone, Debug)]
enum Piece {
    /// The original text in these bytes.
    Copy(usize, usize),
    Lit(String),
}

/// A change to the original text: the bytes `start..end` become `pieces`.
#[derive(Debug)]
struct Edit {
    start: usize,
    end: usize,
    pieces: Vec<Piece>,
    /// Whether the edit only deletes, which can leave a container empty.
    removal: bool,
}

/// The tree of a `use` declaration.
#[derive(Debug)]
enum Node {
    /// A path, a glob or a renamed path: tokens `lo..=hi`.
    Leaf { lo: usize, hi: usize },
    /// `prefix::{children}`: the prefix starts at `lo`.
    Group {
        lo: usize,
        open: usize,
        close: usize,
        children: Vec<Node>,
    },
}

impl Node {
    fn range(&self) -> (usize, usize) {
        match self {
            Node::Leaf { lo, hi } => (*lo, *hi),
            Node::Group { lo, close, .. } => (*lo, *close),
        }
    }

    fn leaves(&self, out: &mut Vec<usize>) {
        match self {
            Node::Leaf { lo, .. } => out.push(*lo),
            Node::Group { children, .. } => children.iter().for_each(|c| c.leaves(out)),
        }
    }

    /// The node with exactly the tokens `range`, if there is one below.
    fn find(&self, range: (usize, usize)) -> Option<&Node> {
        if self.range() == range {
            return Some(self);
        }
        match self {
            Node::Leaf { .. } => None,
            // The braces alone name the group too.
            Node::Group { open, close, .. } if (*open, *close) == range => Some(self),
            Node::Group { children, .. } => children.iter().find_map(|c| c.find(range)),
        }
    }
}

/// A `use` declaration: its keyword and its `;`.
#[derive(Debug)]
struct UseItem {
    keyword: usize,
    semicolon: usize,
    root: Node,
}

/// An inherent `impl` or a `mod` with a body.
#[derive(Debug)]
struct Container {
    keyword: usize,
    open: usize,
    close: usize,
}

/// A text and what is known about its items.
pub struct Source {
    text: String,
    tokens: Vec<Token>,
    uses: Vec<UseItem>,
    containers: Vec<Container>,
    impls: Vec<Impl>,
    /// The `impl`s (indexes in `impls`) that go with the type or trait
    /// whose name is at the key.
    attached: BTreeMap<usize, Vec<usize>>,
    /// The names of `use` declarations (as [`Id::Leaf`]) that go with the
    /// type or trait whose name is at the key: those that import it.
    imports: BTreeMap<usize, Vec<usize>>,
    /// The enums whose variants are offered, by the token of their name.
    enums: BTreeMap<usize, Enum>,
    /// The enum (the token of its name) of each offered variant.
    variant_of: BTreeMap<usize, usize>,
    patterns: Vec<Pattern>,
    /// Every `match` arm read clearly: the bytes of its whole pattern, and
    /// its first and last tokens.
    arms: BTreeMap<(usize, usize), (usize, usize)>,
}

/// A text made from the original by some removals, and the way back from
/// its bytes to the original's.
pub struct Rendered {
    pub text: String,
    /// `(start in text, start in the original, length)` of the stretches
    /// that are copied from the original, in order.
    copies: Vec<(usize, usize, usize)>,
    /// The byte ranges of the original that were deleted, in order.
    removals: Vec<(usize, usize)>,
}

impl Rendered {
    /// The bytes of the original that the bytes `range` of the text are
    /// copied from, when they are all in one stretch.
    pub fn to_original(&self, range: (usize, usize)) -> Option<(usize, usize)> {
        let at = self.copies.partition_point(|copy| copy.0 <= range.0);
        let &(from, original, length) = self.copies.get(at.checked_sub(1)?)?;
        (range.0 >= from && range.1 <= from + length && range.0 <= range.1)
            .then(|| (original + range.0 - from, original + range.1 - from))
    }
}

fn is_punct(token: &Token, text: &str) -> bool {
    token.kind == Kind::Punct && token.text == text
}

fn is_opener(token: &Token) -> bool {
    token.kind == Kind::Punct && matches!(token.text.as_str(), "(" | "[" | "{")
}

fn is_closer(token: &Token) -> bool {
    token.kind == Kind::Punct && matches!(token.text.as_str(), ")" | "]" | "}")
}

/// The index of the bracket that closes the one at `open`.
fn match_forward(tokens: &[Token], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        if is_opener(token) {
            depth += 1;
        } else if is_closer(token) {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// The index of the bracket that opens the one at `close`.
fn match_backward(tokens: &[Token], close: usize) -> Option<usize> {
    let mut depth = 0usize;
    for i in (0..=close).rev() {
        if is_closer(&tokens[i]) {
            depth += 1;
        } else if is_opener(&tokens[i]) {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// The first token from `from` on that is one of the punctuation `stops`
/// outside any bracket, or `None` if a bracket closes first.
fn scan(tokens: &[Token], from: usize, stops: &[&str]) -> Option<usize> {
    let mut depth = 0usize;
    for (i, token) in tokens.iter().enumerate().skip(from) {
        if token.kind != Kind::Punct {
            continue;
        }
        if depth == 0 && stops.contains(&token.text.as_str()) {
            return Some(i);
        }
        if is_opener(token) {
            depth += 1;
        } else if is_closer(token) {
            depth = depth.checked_sub(1)?;
        }
    }
    None
}

/// The index of the first token of the item whose keyword is at `keyword`:
/// its modifiers, visibility and attributes come first.
fn start_of_item(tokens: &[Token], keyword: usize) -> usize {
    let mut first = keyword;
    while first > 0 {
        let prev = &tokens[first - 1];
        let before = match prev.kind {
            Kind::Ident
                if matches!(
                    prev.text.as_str(),
                    "pub" | "const" | "async" | "unsafe" | "extern" | "default" | "safe"
                ) =>
            {
                first - 1
            }
            Kind::Str if first >= 2 && tokens[first - 2].text == "extern" => first - 2,
            Kind::Punct if prev.text == ")" => match match_backward(tokens, first - 1) {
                Some(open) if open > 0 && tokens[open - 1].text == "pub" => open - 1,
                _ => break,
            },
            Kind::Punct if prev.text == "]" => match match_backward(tokens, first - 1) {
                Some(open) if open > 0 && is_punct(&tokens[open - 1], "#") => open - 1,
                _ => break,
            },
            _ => break,
        };
        first = before;
    }
    first
}

/// Whether an item can start at token `first`: after `;`, `{`, `}` or an
/// inner attribute, or at the start.
fn item_position(tokens: &[Token], first: usize) -> bool {
    first == 0
        || ["}", "{", ";", "]"]
            .iter()
            .any(|text| is_punct(&tokens[first - 1], text))
}

/// The tree that starts at `*pos` and ends before `end`, or before a `,` or
/// `}` at its own level. `*pos` is left after it.
fn parse_tree(tokens: &[Token], pos: &mut usize, end: usize) -> Option<Node> {
    let lo = *pos;
    while *pos < end {
        let token = &tokens[*pos];
        if is_punct(token, ",") || is_punct(token, "}") {
            break;
        }
        if is_punct(token, "{") {
            let open = *pos;
            *pos += 1;
            let mut children = Vec::new();
            loop {
                if *pos >= end {
                    return None;
                }
                if is_punct(&tokens[*pos], "}") {
                    break;
                }
                children.push(parse_tree(tokens, pos, end)?);
                if *pos < end && is_punct(&tokens[*pos], ",") {
                    *pos += 1;
                } else if *pos >= end || !is_punct(&tokens[*pos], "}") {
                    return None;
                }
            }
            let close = *pos;
            *pos += 1;
            return Some(Node::Group {
                lo,
                open,
                close,
                children,
            });
        }
        *pos += 1;
    }
    (*pos > lo).then(|| Node::Leaf { lo, hi: *pos - 1 })
}

impl Source {
    /// Reads `text`. It fails when the text cannot be lexed.
    pub fn new(text: &str) -> Result<Source, String> {
        let tokens = tokens(text)?;
        let mut uses = Vec::new();
        let mut containers = Vec::new();
        let mut impls = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            if token.kind != Kind::Ident {
                continue;
            }
            match token.text.as_str() {
                "use" if item_position(&tokens, start_of_item(&tokens, i)) => {
                    let Some(semicolon) = scan(&tokens, i + 1, &[";"]) else {
                        continue;
                    };
                    let mut pos = i + 1;
                    if let Some(root) = parse_tree(&tokens, &mut pos, semicolon) {
                        if pos == semicolon {
                            uses.push(UseItem {
                                keyword: i,
                                semicolon,
                                root,
                            });
                        }
                    }
                }
                "mod" => {
                    let named = tokens.get(i + 1).is_some_and(|t| t.kind == Kind::Ident);
                    if named && tokens.get(i + 2).is_some_and(|t| is_punct(t, "{")) {
                        if let Some(close) = match_forward(&tokens, i + 2) {
                            containers.push(Container {
                                keyword: i,
                                open: i + 2,
                                close,
                            });
                        }
                    }
                }
                "impl" if item_position(&tokens, start_of_item(&tokens, i)) => {
                    let Some(open) = scan(&tokens, i + 1, &["{"]) else {
                        continue;
                    };
                    let Some(close) = match_forward(&tokens, open) else {
                        continue;
                    };
                    // A trait impl is not removed on its own: it can matter
                    // without any use (`Drop`, a marker trait, a blanket
                    // impl). It goes with its type or its trait.
                    let inherent = !tokens[i + 1..open]
                        .iter()
                        .any(|t| t.kind == Kind::Ident && t.text == "for");
                    if inherent {
                        containers.push(Container {
                            keyword: i,
                            open,
                            close,
                        });
                    }
                    let (trait_path, self_path) = shapes::impl_paths(&tokens, i, open);
                    impls.push(Impl {
                        first: start_of_item(&tokens, i),
                        open,
                        close,
                        trait_path,
                        self_path,
                    });
                }
                _ => {}
            }
        }
        let mut source = Source {
            text: text.to_string(),
            tokens,
            uses,
            containers,
            impls,
            attached: BTreeMap::new(),
            imports: BTreeMap::new(),
            enums: BTreeMap::new(),
            variant_of: BTreeMap::new(),
            patterns: Vec::new(),
            arms: BTreeMap::new(),
        };
        source.find_shapes();
        Ok(source)
    }

    /// The innermost `mod` that holds token `at`, by its keyword, or `None`
    /// for the crate's root.
    fn module_of(&self, at: usize) -> Option<usize> {
        self.containers
            .iter()
            .filter(|c| c.open < at && at < c.close && self.tokens[c.keyword].text == "mod")
            .max_by_key(|c| c.open)
            .map(|c| c.keyword)
    }

    /// Fills what goes with types, traits and variants (see [`shapes`]).
    fn find_shapes(&mut self) {
        let tokens = &self.tokens;
        // The items that define each name, as (keyword, name) tokens.
        let mut defined: BTreeMap<&str, Vec<(usize, usize)>> = BTreeMap::new();
        for (i, token) in tokens.iter().enumerate() {
            let kinds = ["struct", "enum", "union", "trait", "type"];
            if token.kind == Kind::Ident && kinds.contains(&token.text.as_str()) {
                if let Some(name) = tokens.get(i + 1).filter(|t| t.kind == Kind::Ident) {
                    defined
                        .entry(name.text.as_str())
                        .or_default()
                        .push((i, i + 1));
                }
            }
        }
        // The bundle's modules, which a `use` path may start with.
        let modules: BTreeSet<&str> = self
            .containers
            .iter()
            .filter(|c| tokens[c.keyword].text == "mod")
            .map(|c| tokens[c.keyword + 1].text.as_str())
            .collect();
        // The imports of a name that the bundle defines once, along paths
        // of the bundle, without renaming; and the names that each module
        // imports from elsewhere (std).
        let mut imports: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut std_imports: BTreeSet<(Option<usize>, &str)> = BTreeSet::new();
        for u in &self.uses {
            let root = tokens[u.keyword + 1..u.semicolon]
                .iter()
                .find(|t| t.kind == Kind::Ident)
                .map_or("", |t| t.text.as_str());
            let ours = matches!(root, "crate" | "self" | "super") || modules.contains(root);
            let mut leaves = Vec::new();
            u.root.leaves(&mut leaves);
            for lo in leaves {
                let Some((lo, hi)) = find_leaf(&u.root, lo).map(Node::range) else {
                    continue;
                };
                let last = &tokens[hi];
                if last.kind != Kind::Ident || tokens[lo..=hi].iter().any(|t| t.text == "as") {
                    continue;
                }
                if !ours {
                    // From std, or along a path not read here.
                    std_imports.insert((self.module_of(lo), last.text.as_str()));
                    continue;
                }
                if let Some(&[(keyword, at)]) = defined.get(last.text.as_str()).map(Vec::as_slice) {
                    if keyword > 0 && tokens[keyword].text != "type" {
                        imports.entry(at).or_default().push(lo);
                    }
                }
            }
        }
        // Whether the path that starts at token `start`, whose last name is
        // at token `name`, used at token `at`, names the item whose name is
        // at token `item`: a bare name in the item's module or in a module
        // that imports it from the bundle (and nothing of that name from
        // elsewhere), or a path from `crate`, `self` or `super` when the
        // bundle defines the name once.
        let names = |item: usize, at: usize, start: usize, name: usize| -> bool {
            let text = tokens[item].text.as_str();
            let once = defined.get(text).is_some_and(|items| items.len() == 1);
            if is_punct(&tokens[start], "::") {
                return false;
            }
            let here = self.module_of(at);
            if start != name {
                return once && matches!(tokens[start].text.as_str(), "crate" | "self" | "super");
            }
            if std_imports.contains(&(here, text)) {
                return false;
            }
            here == self.module_of(item)
                || imports
                    .get(&item)
                    .is_some_and(|leaves| leaves.iter().any(|&lo| self.module_of(lo) == here))
        };
        let mut attached = BTreeMap::new();
        for (name, items) in &defined {
            for &(keyword, at) in items {
                if keyword == 0 || tokens[keyword].text == "type" || self.item_tokens(at).is_none()
                {
                    continue;
                }
                let is_trait = tokens[keyword].text == "trait";
                let found: Vec<usize> = (0..self.impls.len())
                    .filter(|&b| {
                        let block = &self.impls[b];
                        let path = if is_trait {
                            &block.trait_path
                        } else {
                            &block.self_path
                        };
                        path.as_ref().is_some_and(|path| {
                            path.name == *name && names(at, block.first, path.start, path.at)
                        })
                    })
                    .collect();
                if !found.is_empty() {
                    attached.insert(at, found);
                }
            }
        }

        // Whether a `use` names an enum's variants (`E::V`, `E::*`) or
        // renames it.
        let imported = |name: &str| {
            self.uses.iter().any(|u| {
                (u.keyword..u.semicolon).any(|i| {
                    let next = &tokens[i + 1];
                    tokens[i].kind == Kind::Ident
                        && tokens[i].text == name
                        && (is_punct(next, "::") || next.text == "as")
                })
            })
        };
        let hashes_discriminants = tokens
            .iter()
            .any(|t| t.kind == Kind::Ident && t.text == "discriminant");
        // Removing a variant can make an enum smaller.
        let sizes = tokens
            .iter()
            .any(|t| t.kind == Kind::Ident && matches!(t.text.as_str(), "size_of" | "size_of_val"));
        let mut enums = BTreeMap::new();
        let mut variant_of = BTreeMap::new();
        for (name, items) in &defined {
            // Another item may share the enum's name if it is not an enum:
            // then `Name::Variant` can only be the variant when no item can
            // be named `Variant` in a pattern (a `const`, a `static`, a unit
            // or tuple `struct`).
            let mut found = items.iter().filter(|&&(k, _)| tokens[k].text == "enum");
            let (Some(&(keyword, at)), None) = (found.next(), found.next()) else {
                continue;
            };
            let shared = items.len() > 1;
            if tokens[keyword].text != "enum"
                || !item_position(tokens, start_of_item(tokens, keyword))
                || imported(name)
            {
                continue;
            }
            let Some((parsed, refused)) = shapes::parse_enum(tokens, keyword) else {
                continue;
            };
            let hash_impl = self
                .impls
                .iter()
                .any(|b| b.trait_name() == Some("Hash") && b.self_name() == Some(*name));
            let pattern_item = |v: &shapes::Variant| {
                tokens.windows(2).any(|w| {
                    matches!(w[0].text.as_str(), "const" | "static" | "struct")
                        && w[0].kind == Kind::Ident
                        && w[1].text == tokens[v.name].text
                })
            };
            if refused
                || sizes
                || parsed.variants.len() < 2
                || shared && parsed.variants.iter().any(pattern_item)
                || !parsed.fieldless && (hash_impl || hashes_discriminants)
            {
                continue;
            }
            for v in &parsed.variants {
                variant_of.insert(v.name, at);
            }
            enums.insert(at, parsed);
        }
        let mut patterns = shapes::patterns(tokens);
        let mut arms = BTreeMap::new();
        for p in &patterns {
            if let PatternKind::Arm { first, last } = p.kind {
                let (lo, hi) = (p.alts[0].0, p.alts[p.alts.len() - 1].1);
                arms.insert((tokens[lo].start, tokens[hi].end), (first, last));
            }
        }
        shapes::resolve(tokens, &mut patterns, &enums, &self.impls, names);
        patterns.retain(|p| p.refs.iter().any(|r| !r.is_empty()));

        self.attached = attached;
        self.imports = imports;
        self.enums = enums;
        self.variant_of = variant_of;
        self.patterns = patterns;
        self.arms = arms;
    }

    /// The removal of the arm whose whole pattern rustc reports unreachable
    /// at the bytes `span`; not when it reports one alternative only.
    pub fn arm_at(&self, span: (usize, usize)) -> Option<Id> {
        self.arms.get(&span).map(|&(first, _)| Id::Arm(first))
    }

    /// The index of the token that spans exactly the bytes `range`.
    fn token_at(&self, range: (usize, usize)) -> Option<usize> {
        let at = self.tokens.partition_point(|t| t.start < range.0);
        let token = self.tokens.get(at)?;
        (token.start == range.0 && token.end == range.1).then_some(at)
    }

    /// The first and last token of the item named by token `name`.
    fn item_tokens(&self, name: usize) -> Option<(usize, usize)> {
        let tokens = &self.tokens;
        if tokens[name].kind != Kind::Ident || tokens[name].text == "main" {
            return None;
        }
        let mut keyword = name.checked_sub(1)?;
        if tokens[keyword].kind == Kind::Ident && tokens[keyword].text == "mut" {
            keyword = keyword.checked_sub(1)?;
            if tokens[keyword].text != "static" {
                return None;
            }
        }
        if tokens[keyword].kind != Kind::Ident {
            return None;
        }
        let end = match tokens[keyword].text.as_str() {
            "fn" | "struct" => {
                let stop = scan(tokens, name + 1, &["{", ";"])?;
                if tokens[stop].text == "{" {
                    match_forward(tokens, stop)?
                } else {
                    stop
                }
            }
            "enum" | "union" | "trait" => match_forward(tokens, scan(tokens, name + 1, &["{"])?)?,
            "const" | "static" | "type" => scan(tokens, name + 1, &[";"])?,
            _ => return None,
        };
        Some((start_of_item(tokens, keyword), end))
    }

    /// The bytes that go when the tokens `first..=last` go: from the end of
    /// the token before them, so that doc comments go too.
    fn cut(&self, first: usize, last: usize) -> (usize, usize) {
        let start = if first == 0 {
            0
        } else {
            self.tokens[first - 1].end
        };
        (start, self.tokens[last].end)
    }

    /// The removal of the item that rustc reports at the bytes `span`, if
    /// they are the name of an item whose extent is clear.
    pub fn item_at(&self, span: (usize, usize)) -> Option<Id> {
        let name = self.token_at(span)?;
        if self.variant_of.contains_key(&name) {
            return Some(Id::Variant(name));
        }
        self.item_tokens(name).map(|_| Id::Item(name))
    }

    /// The variant whose name is at token `name`.
    fn variant(&self, name: usize) -> &shapes::Variant {
        self.enums[&self.variant_of[&name]]
            .variants
            .iter()
            .find(|v| v.name == name)
            .expect("an offered variant is in its enum")
    }

    /// The removals of the names that rustc reports unused at the bytes
    /// `span`: one name, or all those of a group.
    pub fn leaves_at(&self, span: (usize, usize)) -> Vec<Id> {
        let first = self.tokens.partition_point(|t| t.start < span.0);
        let last = self.tokens.partition_point(|t| t.end <= span.1);
        let mut out = Vec::new();
        if first >= last {
            return out;
        }
        let range = (first, last - 1);
        let at = self.uses.partition_point(|u| u.keyword < first);
        if let Some(item) = at.checked_sub(1).and_then(|at| self.uses.get(at)) {
            if item.semicolon > range.1 {
                let mut found = Vec::new();
                if let Some(node) = item.root.find(range) {
                    node.leaves(&mut found);
                }
                out.extend(found.into_iter().map(Id::Leaf));
            }
        }
        out
    }

    /// The bytes that removing `id` deletes (for a name of a `use`, the
    /// name's own).
    pub fn bytes_of(&self, id: Id) -> (usize, usize) {
        match id {
            Id::Item(name) => {
                let (first, last) = self.item_tokens(name).expect("an item has an extent");
                self.cut(first, last)
            }
            Id::Container(keyword) => {
                let container = self
                    .containers
                    .iter()
                    .find(|c| c.keyword == keyword)
                    .expect("a container is known");
                self.cut(start_of_item(&self.tokens, keyword), container.close)
            }
            Id::Leaf(lo) => {
                let item = self.use_of(lo).expect("a name of a use is in a use");
                let node = find_leaf(&item.root, lo).expect("a name is in its tree");
                let (lo, hi) = node.range();
                (self.tokens[lo].start, self.tokens[hi].end)
            }
            Id::Variant(name) => {
                let variant = self.variant(name);
                self.cut(variant.first, variant.last)
            }
            Id::Arm(first) => {
                let &(_, last) = self
                    .arms
                    .values()
                    .find(|(f, _)| *f == first)
                    .expect("an arm is known");
                self.cut(first, last)
            }
        }
    }

    fn use_of(&self, token: usize) -> Option<&UseItem> {
        let at = self.uses.partition_point(|u| u.keyword < token);
        self.uses
            .get(at.checked_sub(1)?)
            .filter(|u| u.semicolon > token)
    }

    /// The containers that have no token left in `rendered`, as removals.
    pub fn empty_containers(&self, rendered: &Rendered) -> Vec<Id> {
        let removals = &rendered.removals;
        let deleted = |start: usize, end: usize| -> Option<usize> {
            let at = removals.partition_point(|r| r.1 <= start);
            removals
                .get(at)
                .filter(|r| r.0 <= start && end <= r.1)
                .map(|r| r.1)
        };
        let mut out = Vec::new();
        for c in &self.containers {
            let token = &self.tokens[c.keyword];
            if deleted(token.start, token.end).is_some() {
                continue;
            }
            let mut i = c.open + 1;
            let mut empty = true;
            while i < c.close {
                let t = &self.tokens[i];
                match deleted(t.start, t.end) {
                    Some(until) => i = self.tokens.partition_point(|t| t.start < until).max(i + 1),
                    None => {
                        empty = false;
                        break;
                    }
                }
            }
            if empty {
                out.push(Id::Container(c.keyword));
            }
        }
        out
    }

    /// The text without the removals `selected`.
    pub fn render(&self, selected: &BTreeSet<Id>) -> Rendered {
        let mut edits = Vec::new();
        let mut leaves: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        let mut variants: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for &id in selected {
            match id {
                Id::Item(_) | Id::Container(_) | Id::Arm(_) => {
                    let (start, end) = self.bytes_of(id);
                    edits.push(Edit {
                        start,
                        end,
                        pieces: Vec::new(),
                        removal: true,
                    });
                    if let Id::Item(name) = id {
                        for &lo in self.imports.get(&name).into_iter().flatten() {
                            let at = self
                                .uses
                                .partition_point(|u| u.keyword < lo)
                                .saturating_sub(1);
                            leaves.entry(at).or_default().insert(lo);
                        }
                        for &b in self.attached.get(&name).into_iter().flatten() {
                            let block = &self.impls[b];
                            let (start, end) = self.cut(block.first, block.close);
                            edits.push(Edit {
                                start,
                                end,
                                pieces: Vec::new(),
                                removal: true,
                            });
                        }
                    }
                }
                Id::Variant(name) => {
                    variants
                        .entry(self.variant_of[&name])
                        .or_default()
                        .insert(name);
                }
                Id::Leaf(lo) => {
                    let at = self
                        .uses
                        .partition_point(|u| u.keyword < lo)
                        .saturating_sub(1);
                    leaves.entry(at).or_default().insert(lo);
                }
            }
        }
        for (at, chosen) in &leaves {
            let item = &self.uses[*at];
            match self.render_node(&item.root, chosen) {
                None => {
                    let first = start_of_item(&self.tokens, item.keyword);
                    let (start, end) = self.cut(first, item.semicolon);
                    edits.push(Edit {
                        start,
                        end,
                        pieces: Vec::new(),
                        removal: true,
                    });
                }
                Some(pieces) => {
                    let (lo, hi) = item.root.range();
                    edits.push(Edit {
                        start: self.tokens[lo].start,
                        end: self.tokens[hi].end,
                        pieces,
                        removal: false,
                    });
                }
            }
        }
        self.render_variants(variants, &mut edits);
        // In order; an edit inside another one is dropped.
        edits.sort_by_key(|e| (e.start, std::cmp::Reverse(e.end)));
        let mut rendered = Rendered {
            text: String::with_capacity(self.text.len()),
            copies: Vec::new(),
            removals: Vec::new(),
        };
        let mut cursor = 0;
        for edit in &edits {
            if edit.start < cursor {
                continue;
            }
            rendered.copy(&self.text, cursor, edit.start);
            for piece in &edit.pieces {
                match piece {
                    Piece::Copy(start, end) => rendered.copy(&self.text, *start, *end),
                    Piece::Lit(text) => rendered.text.push_str(text),
                }
            }
            if edit.removal {
                rendered.removals.push((edit.start, edit.end));
                // Keep the neighbours apart, whatever the layout.
                if !self.text[edit.end..].starts_with(char::is_whitespace) {
                    rendered.text.push('\n');
                }
            }
            cursor = edit.end;
        }
        rendered.copy(&self.text, cursor, self.text.len());
        rendered
    }

    /// The pieces of `node` without the names `chosen`, or `None` when
    /// nothing is left of it.
    fn render_node(&self, node: &Node, chosen: &BTreeSet<usize>) -> Option<Vec<Piece>> {
        match node {
            Node::Leaf { lo, hi } => {
                if chosen.contains(lo) {
                    None
                } else {
                    Some(vec![Piece::Copy(
                        self.tokens[*lo].start,
                        self.tokens[*hi].end,
                    )])
                }
            }
            Node::Group {
                lo, open, children, ..
            } => {
                let kept: Vec<Vec<Piece>> = children
                    .iter()
                    .filter_map(|c| self.render_node(c, chosen))
                    .collect();
                if kept.is_empty() {
                    return None;
                }
                let mut pieces = vec![Piece::Copy(self.tokens[*lo].start, self.tokens[*open].end)];
                for (i, child) in kept.into_iter().enumerate() {
                    if i > 0 {
                        pieces.push(Piece::Lit(",".into()));
                    }
                    pieces.extend(child);
                }
                pieces.push(Piece::Lit("}".into()));
                Some(pieces)
            }
        }
    }

    /// The edits that remove the variants `chosen` (by enum) from their
    /// enums and from the patterns that name them. An enum keeps at least
    /// one variant: when all are chosen, none of its variants goes.
    fn render_variants(&self, mut chosen: BTreeMap<usize, BTreeSet<usize>>, edits: &mut Vec<Edit>) {
        chosen.retain(|e, names| names.len() < self.enums[e].variants.len());
        let tokens = &self.tokens;
        for (e, names) in &chosen {
            let parsed = &self.enums[e];
            let mut pieces = Vec::new();
            let mut previous = None;
            for (at, v) in parsed.variants.iter().enumerate() {
                if names.contains(&v.name) {
                    continue;
                }
                if !pieces.is_empty() {
                    pieces.push(Piece::Lit(",".into()));
                }
                pieces.push(Piece::Copy(tokens[v.first].start, tokens[v.last].end));
                // A variant without fields after a removed one keeps its
                // number, for `as` casts.
                let renumbered = previous.map_or(at != 0, |p: usize| p + 1 != at);
                if parsed.fieldless && renumbered {
                    pieces.push(Piece::Lit(format!("={at}")));
                }
                previous = Some(at);
            }
            let (first, last) = (
                &parsed.variants[0],
                &parsed.variants[parsed.variants.len() - 1],
            );
            edits.push(Edit {
                start: tokens[first.first].start,
                end: tokens[last.last].end,
                pieces,
                removal: false,
            });
        }
        let dead: BTreeSet<usize> = chosen.into_values().flatten().collect();
        for pattern in &self.patterns {
            let gone: Vec<bool> = pattern.refs.iter().map(|r| !r.is_disjoint(&dead)).collect();
            if !gone.contains(&true) {
                continue;
            }
            if !gone.contains(&false) {
                match pattern.kind {
                    PatternKind::Arm { first, last } => {
                        let (start, end) = self.cut(first, last);
                        edits.push(Edit {
                            start,
                            end,
                            pieces: Vec::new(),
                            removal: true,
                        });
                    }
                    PatternKind::Matches {
                        first,
                        last,
                        plain: true,
                    } => edits.push(Edit {
                        start: tokens[first].start,
                        end: tokens[last].end,
                        pieces: vec![Piece::Lit("false".into())],
                        removal: false,
                    }),
                    // The removal does not compile, and the variant stays.
                    PatternKind::Matches { .. } => {}
                }
                continue;
            }
            let mut pieces = Vec::new();
            for (&(lo, hi), _) in pattern.alts.iter().zip(&gone).filter(|(_, &g)| !g) {
                if !pieces.is_empty() {
                    pieces.push(Piece::Lit("|".into()));
                }
                pieces.push(Piece::Copy(tokens[lo].start, tokens[hi].end));
            }
            edits.push(Edit {
                start: tokens[pattern.alts[0].0].start,
                end: tokens[pattern.alts[pattern.alts.len() - 1].1].end,
                pieces,
                removal: false,
            });
        }
    }
}

impl Source {
    /// The name that other code uses for the item `id` removes, when it
    /// has one: an item's name, the last name of an import.
    pub fn name_of(&self, id: Id) -> Option<&str> {
        match id {
            Id::Item(name) | Id::Variant(name) => Some(&self.tokens[name].text),
            Id::Container(_) | Id::Arm(_) => None,
            Id::Leaf(lo) => {
                let item = self.use_of(lo)?;
                let (lo, hi) = find_leaf(&item.root, lo)?.range();
                self.tokens[lo..=hi]
                    .iter()
                    .rev()
                    .find(|t| t.kind == Kind::Ident)
                    .map(|t| t.text.as_str())
            }
        }
    }
}

/// The leaf of `node` whose first token is `lo`.
fn find_leaf(node: &Node, lo: usize) -> Option<&Node> {
    match node {
        Node::Leaf { lo: at, .. } => (*at == lo).then_some(node),
        Node::Group { children, .. } => children.iter().find_map(|c| find_leaf(c, lo)),
    }
}

impl Rendered {
    /// Appends the original's bytes `start..end`.
    fn copy(&mut self, original: &str, start: usize, end: usize) {
        if start < end {
            self.copies.push((self.text.len(), start, end - start));
            self.text.push_str(&original[start..end]);
        }
    }
}
