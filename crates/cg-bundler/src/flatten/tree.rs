//! What the flattener reads in the tokens of a bundle: the brackets and what
//! each `{` opens, where items start, the modules with their items and `use`
//! declarations, and what each name a module binds stands for.
//!
//! It reads only the forms the bundles use. Anything else (a glob import, a
//! `mod name;`, a macro defined or called at module level other than
//! `thread_local!`, an `extern` block, a path that goes through an item) is
//! an error that says what was found, and the bundle is not flattened.

use std::collections::{BTreeMap, BTreeSet};

use crate::minify::{Kind, Token};

/// No token.
pub const NONE: usize = usize::MAX;

/// What a `{` opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brace {
    /// The body of a `mod`.
    Module,
    /// The body of an inherent `impl`.
    Impl,
    /// The body of an `impl Trait for Type`.
    TraitImpl,
    /// The body of a `trait`.
    Trait,
    /// The fields or variants of a `struct`, an `enum` or a `union`, or the
    /// fields of a struct variant.
    Fields,
    /// The body of a `fn`.
    Fn,
    /// Anything else: a block, a struct literal or pattern, a `match`, a
    /// `use` tree, the braces of a macro call.
    Other,
}

/// The brackets of a text and what its `{` open.
pub struct Shape {
    /// For each `(`, `[` or `{`, its closer, and for each closer its opener;
    /// [`NONE`] for other tokens.
    pub pair: Vec<usize>,
    /// For each token, the innermost bracket that holds it (its opener), or
    /// [`NONE`] at the top level.
    pub parent: Vec<usize>,
    /// What each `{` opens, by its index.
    pub brace: BTreeMap<usize, Brace>,
    /// Whether each token is part of an attribute, `#` and brackets
    /// included.
    pub attribute: Vec<bool>,
}

impl Shape {
    pub fn of(tokens: &[Token]) -> Result<Shape, String> {
        let n = tokens.len();
        let mut pair = vec![NONE; n];
        let mut parent = vec![NONE; n];
        let mut stack: Vec<usize> = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            parent[i] = stack.last().copied().unwrap_or(NONE);
            if token.kind != Kind::Punct {
                continue;
            }
            match token.text.as_str() {
                "(" | "[" | "{" => stack.push(i),
                ")" | "]" | "}" => {
                    let open = stack.pop().ok_or_else(|| {
                        format!("line {}: unbalanced `{}`", token.line, token.text)
                    })?;
                    let expected = match token.text.as_str() {
                        ")" => "(",
                        "]" => "[",
                        _ => "{",
                    };
                    if tokens[open].text != expected {
                        return Err(format!("line {}: unbalanced `{}`", token.line, token.text));
                    }
                    pair[open] = i;
                    pair[i] = open;
                    parent[i] = stack.last().copied().unwrap_or(NONE);
                }
                _ => {}
            }
        }
        if let Some(&open) = stack.last() {
            return Err(format!("line {}: unclosed bracket", tokens[open].line));
        }
        let mut attribute = vec![false; n];
        for i in 0..n {
            if tokens[i].text != "#" || tokens[i].kind != Kind::Punct {
                continue;
            }
            let open = match tokens.get(i + 1).map(|t| t.text.as_str()) {
                Some("[") => i + 1,
                Some("!") if tokens.get(i + 2).is_some_and(|t| t.text == "[") => i + 2,
                _ => continue,
            };
            for flag in &mut attribute[i..=pair[open]] {
                *flag = true;
            }
        }
        let mut shape = Shape {
            pair,
            parent,
            brace: BTreeMap::new(),
            attribute,
        };
        shape.braces(tokens);
        Ok(shape)
    }

    /// Finds what each `{` opens, from the keyword of the item it belongs
    /// to.
    fn braces(&mut self, tokens: &[Token]) {
        // The kind that the next `{` with this parent opens.
        let mut pending: BTreeMap<usize, Brace> = BTreeMap::new();
        for (i, token) in tokens.iter().enumerate() {
            let parent = self.parent[i];
            match (token.kind, token.text.as_str()) {
                (Kind::Punct, "{") => {
                    let kind = match pending.remove(&parent) {
                        Some(kind) => kind,
                        None if self.brace.get(&parent) == Some(&Brace::Fields) => Brace::Fields,
                        None => Brace::Other,
                    };
                    self.brace.insert(i, kind);
                }
                (Kind::Punct, ";") => {
                    pending.remove(&parent);
                }
                (Kind::Ident, "for") if pending.get(&parent) == Some(&Brace::Impl) => {
                    pending.insert(parent, Brace::TraitImpl);
                }
                (Kind::Ident, word) => {
                    let kind = match word {
                        "mod" => Brace::Module,
                        "impl" => Brace::Impl,
                        "trait" => Brace::Trait,
                        "struct" | "enum" => Brace::Fields,
                        "union" if tokens.get(i + 1).is_some_and(|t| t.kind == Kind::Ident) => {
                            Brace::Fields
                        }
                        "fn" => Brace::Fn,
                        "const" | "static" | "type" | "use" => Brace::Other,
                        _ => continue,
                    };
                    if word == "const" && tokens.get(i + 1).is_some_and(|t| t.text == "fn") {
                        continue;
                    }
                    if self.item_start(tokens, i).is_some() {
                        pending.insert(parent, kind);
                    }
                }
                _ => {}
            }
        }
    }

    /// Where the item whose keyword is at `keyword` starts, after its
    /// attributes: the first of its qualifiers (`pub`, `pub(crate)`,
    /// `unsafe`, `const fn`...), or the keyword itself. `None` when the
    /// keyword does not start an item (`fn` in a type, `const` in generics).
    pub fn item_start(&self, tokens: &[Token], keyword: usize) -> Option<usize> {
        let mut at = keyword;
        while at > 0 {
            let prev = &tokens[at - 1];
            match (prev.kind, prev.text.as_str()) {
                (Kind::Ident, "pub" | "unsafe" | "async" | "default" | "auto" | "extern") => {
                    at -= 1
                }
                (Kind::Ident, "const") if tokens[keyword].text == "fn" => at -= 1,
                (Kind::Str, _) if at >= 2 && tokens[at - 2].text == "extern" => at -= 1,
                (Kind::Punct, ")") => {
                    let open = self.pair[at - 1];
                    if open > 0 && tokens[open - 1].text == "pub" {
                        at = open - 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        if at == 0 {
            return Some(0);
        }
        let prev = &tokens[at - 1];
        match prev.text.as_str() {
            ";" | "{" | "}" if prev.kind == Kind::Punct => Some(at),
            "]" if self.attribute[at - 1] => Some(at),
            _ => None,
        }
    }
}

/// The kind of an item that binds a name in a module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Fn,
    Struct,
    Enum,
    Union,
    Trait,
    Type,
    Const,
    Static,
}

impl ItemKind {
    /// Whether a bare name of this kind can be a pattern (`N =>`, `Unit`),
    /// where a binding of the same name would otherwise be.
    pub fn in_patterns(self) -> bool {
        matches!(self, ItemKind::Const | ItemKind::Static | ItemKind::Struct)
    }
}

/// An item that binds a name at the top level of a module.
#[derive(Clone, Debug)]
pub struct Item {
    pub kind: ItemKind,
    /// The token of its name.
    pub name: usize,
}

/// One name of a `use` tree.
#[derive(Clone, Debug)]
pub struct Leaf {
    /// The segments of its path and their tokens, without a final `self`.
    pub path: Vec<(String, usize)>,
    /// Whether it ends with `self` (`use a::b::{self}`).
    pub self_leaf: bool,
    /// The token of its last segment (`self` included).
    pub last: usize,
    /// `as name` or `as _`: the name and its token.
    pub alias: Option<(String, usize)>,
}

impl Leaf {
    /// The name it binds, `None` for `as _`.
    pub fn binding(&self) -> Option<&str> {
        match &self.alias {
            Some((name, _)) if name == "_" => None,
            Some((name, _)) => Some(name),
            None => self.path.last().map(|(name, _)| name.as_str()),
        }
    }
}

/// A `use` declaration at the top level of a module.
#[derive(Clone, Debug)]
pub struct Use {
    /// Its tokens, attributes and visibility included, to the `;`.
    pub tokens: (usize, usize),
    pub leaves: Vec<Leaf>,
}

/// A module of the bundle, the root included.
#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub parent: Option<usize>,
    /// The tokens of its body, between its braces (all of them for the
    /// root).
    pub body: (usize, usize),
    /// The tokens that open it (its attributes, visibility, `mod`, name and
    /// `{`), and its `}`; none for the root.
    pub opener: Option<(usize, usize)>,
    pub closer: Option<usize>,
    pub children: BTreeMap<String, usize>,
    /// Its items, by name.
    pub items: BTreeMap<String, Item>,
    pub uses: Vec<Use>,
}

/// What a name that a module binds stands for.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    /// An item of a module, by its module and name.
    Item(usize, String),
    /// A module.
    Module(usize),
    /// A path of std, `core` or `alloc`.
    Std(String),
}

/// Which of a module and an item of one name a path means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wanted {
    /// The module: the name is followed by `::`.
    Module,
    /// The item.
    Item,
    /// The only one: a `use` imports both.
    Only,
}

/// The modules of a bundle.
pub struct Tree {
    pub modules: Vec<Module>,
}

/// The std crates a path can start with.
pub const STD_CRATES: [&str; 3] = ["std", "core", "alloc"];

impl Tree {
    pub fn of(tokens: &[Token], shape: &Shape) -> Result<Tree, String> {
        let mut tree = Tree {
            modules: vec![Module {
                name: String::new(),
                parent: None,
                body: (0, tokens.len()),
                opener: None,
                closer: None,
                children: BTreeMap::new(),
                items: BTreeMap::new(),
                uses: Vec::new(),
            }],
        };
        tree.read(0, tokens, shape)?;
        Ok(tree)
    }

    /// The path of a module from the root, `crate` for the root.
    pub fn path(&self, module: usize) -> String {
        let mut names = Vec::new();
        let mut at = Some(module);
        while let Some(m) = at {
            names.push(self.modules[m].name.as_str());
            at = self.modules[m].parent;
        }
        names.pop();
        names.reverse();
        match names.is_empty() {
            true => "crate".to_string(),
            false => names.join("::"),
        }
    }

    /// Reads the items at the top level of `module`.
    fn read(&mut self, module: usize, tokens: &[Token], shape: &Shape) -> Result<(), String> {
        let (start, end) = self.modules[module].body;
        let mut i = start;
        // Where the current item starts, its attributes included.
        let mut item_start = start;
        let mut attributes: Vec<(usize, usize)> = Vec::new();
        while i < end {
            let token = &tokens[i];
            let line = token.line;
            if token.text == "#" && token.kind == Kind::Punct {
                let open = if tokens[i + 1].text == "!" {
                    i + 2
                } else {
                    i + 1
                };
                if open != i + 1 {
                    return Err(format!("line {line}: an inner attribute"));
                }
                attributes.push((i, shape.pair[open]));
                i = shape.pair[open] + 1;
                continue;
            }
            if token.text == ";" {
                i += 1;
                item_start = i;
                attributes.clear();
                continue;
            }
            // Skip the qualifiers to the keyword.
            let mut k = i;
            loop {
                match tokens[k].text.as_str() {
                    "pub" if tokens[k + 1].text == "(" => k = shape.pair[k + 1] + 1,
                    "pub" | "unsafe" | "async" | "default" | "auto" => k += 1,
                    "const" if tokens[k + 1].text == "fn" => k += 1,
                    "extern" => return Err(format!("line {line}: an `extern` item")),
                    _ => break,
                }
            }
            let keyword = tokens[k].text.as_str();
            let name_at = k + 1;
            let next = |at: usize| tokens.get(at).map_or("", |t| t.text.as_str());
            let ident = |at: usize| tokens.get(at).is_some_and(|t| t.kind == Kind::Ident);
            let item = |kind| -> Result<Option<(ItemKind, usize)>, String> {
                if !ident(name_at) {
                    return Err(format!("line {line}: `{keyword}` without a name"));
                }
                Ok(Some((kind, name_at)))
            };
            let found = match keyword {
                "mod" => {
                    if !ident(name_at) || next(name_at + 1) != "{" {
                        return Err(format!("line {line}: a `mod` without a body"));
                    }
                    for &(a, b) in &attributes {
                        let words: Vec<&str> =
                            tokens[a..=b].iter().map(|t| t.text.as_str()).collect();
                        if words.get(2) != Some(&"allow") {
                            return Err(format!(
                                "line {}: an attribute on a module other than `allow`",
                                tokens[a].line
                            ));
                        }
                    }
                    let open = name_at + 1;
                    let close = shape.pair[open];
                    let child = self.modules.len();
                    let name = tokens[name_at].text.clone();
                    self.modules.push(Module {
                        name: name.clone(),
                        parent: Some(module),
                        body: (open + 1, close),
                        opener: Some((item_start, open)),
                        closer: Some(close),
                        children: BTreeMap::new(),
                        items: BTreeMap::new(),
                        uses: Vec::new(),
                    });
                    if self.modules[module].children.insert(name, child).is_some() {
                        return Err(format!("line {line}: two modules with one name"));
                    }
                    self.read(child, tokens, shape)?;
                    i = close + 1;
                    item_start = i;
                    attributes.clear();
                    continue;
                }
                "use" => {
                    let semi = (k..end)
                        .find(|&at| tokens[at].text == ";" && shape.parent[at] == shape.parent[k])
                        .ok_or_else(|| format!("line {line}: a `use` without `;`"))?;
                    let mut leaves = Vec::new();
                    UseReader {
                        tokens,
                        at: k + 1,
                        end: semi,
                    }
                    .tree(&mut Vec::new(), &mut leaves)?;
                    self.modules[module].uses.push(Use {
                        tokens: (item_start, semi),
                        leaves,
                    });
                    i = semi + 1;
                    item_start = i;
                    attributes.clear();
                    continue;
                }
                "fn" => item(ItemKind::Fn)?,
                "struct" => item(ItemKind::Struct)?,
                "enum" => item(ItemKind::Enum)?,
                "union" if ident(name_at) => item(ItemKind::Union)?,
                "trait" => item(ItemKind::Trait)?,
                "type" => item(ItemKind::Type)?,
                "const" => item(ItemKind::Const)?,
                "static" if next(name_at) == "mut" => {
                    return Err(format!("line {line}: a `static mut`"))
                }
                "static" => item(ItemKind::Static)?,
                "impl" => None,
                "thread_local" if next(k + 1) == "!" && next(k + 2) == "{" => {
                    let open = k + 2;
                    for at in open + 1..shape.pair[open] {
                        if tokens[at].text == "static" && shape.parent[at] == open {
                            if !ident(at + 1) || next(at + 1) == "mut" {
                                return Err(format!("line {line}: a `static` without a name"));
                            }
                            self.add_item(module, tokens, ItemKind::Static, at + 1)?;
                        }
                    }
                    i = shape.pair[open] + 1;
                    item_start = i;
                    attributes.clear();
                    continue;
                }
                _ => {
                    return Err(format!(
                        "line {line}: `{keyword}` at the top level of a module"
                    ))
                }
            };
            if let Some((kind, name)) = found {
                if tokens[name].text != "_" {
                    self.add_item(module, tokens, kind, name)?;
                }
            }
            // The item ends at its first `;` or at the end of its first
            // `{...}` body at this level.
            let mut at = k + 1;
            loop {
                if at >= end {
                    return Err(format!("line {line}: an item without an end"));
                }
                match tokens[at].text.as_str() {
                    ";" => break,
                    "{" => {
                        at = shape.pair[at];
                        break;
                    }
                    "(" | "[" => at = shape.pair[at] + 1,
                    _ => at += 1,
                }
            }
            i = at + 1;
            item_start = i;
            attributes.clear();
        }
        Ok(())
    }

    fn add_item(
        &mut self,
        module: usize,
        tokens: &[Token],
        kind: ItemKind,
        name: usize,
    ) -> Result<(), String> {
        let text = tokens[name].text.clone();
        let item = Item { kind, name };
        if self.modules[module].items.insert(text, item).is_some() {
            // Two items of one name in two namespaces (a struct and a fn).
            return Err(format!(
                "line {}: two items named `{}` in one module",
                tokens[name].line, tokens[name].text
            ));
        }
        Ok(())
    }

    /// What `name` stands for in `module`: an item, a name its `use`
    /// declarations bind, or a child module. A module and an item can share
    /// a name (`mod plan` and `fn plan`): `module` says which one is wanted,
    /// as when the name is followed by `::`.
    pub fn resolve_name(
        &self,
        module: usize,
        name: &str,
        wanted: Wanted,
    ) -> Result<Target, String> {
        self.resolve_name_in(module, name, wanted, &mut BTreeSet::new())
    }

    fn resolve_name_in(
        &self,
        module: usize,
        name: &str,
        wanted: Wanted,
        seen: &mut BTreeSet<(usize, String, bool)>,
    ) -> Result<Target, String> {
        let m = &self.modules[module];
        if !seen.insert((module, name.to_string(), wanted == Wanted::Module)) {
            return Err(format!("`{name}` is imported in a cycle"));
        }
        let mut found = Vec::new();
        if m.items.contains_key(name) {
            found.push(Target::Item(module, name.to_string()));
        }
        if let Some(&child) = m.children.get(name) {
            found.push(Target::Module(child));
        }
        for leaf in m.uses.iter().flat_map(|u| &u.leaves) {
            if leaf.binding() == Some(name) {
                found.push(self.resolve_path_in(module, leaf, seen)?);
            }
        }
        seen.remove(&(module, name.to_string(), wanted == Wanted::Module));
        let is_module = |t: &&Target| matches!(t, Target::Module(_));
        let pick = match wanted {
            Wanted::Module => found.iter().find(is_module),
            Wanted::Item => found.iter().find(|t| !is_module(t)),
            Wanted::Only => match found.as_slice() {
                [one] => Some(one),
                [] => None,
                _ => {
                    return Err(format!(
                        "`{name}` names a module and an item in module `{}`",
                        self.path(module)
                    ))
                }
            },
        };
        let pick = pick.or_else(|| found.first());
        pick.cloned()
            .ok_or_else(|| format!("`{name}` is not defined in module `{}`", self.path(module)))
    }

    /// What the path of a `use` leaf in `module` stands for. Its last name
    /// must not be both a module and an item, since the `use` would import
    /// both.
    pub fn resolve_leaf(&self, module: usize, leaf: &Leaf) -> Result<Target, String> {
        self.resolve_path_in(module, leaf, &mut BTreeSet::new())
    }

    fn resolve_path_in(
        &self,
        module: usize,
        leaf: &Leaf,
        seen: &mut BTreeSet<(usize, String, bool)>,
    ) -> Result<Target, String> {
        let names: Vec<&str> = leaf.path.iter().map(|(name, _)| name.as_str()).collect();
        let target = self.walk_in(module, &names, Wanted::Only, seen)?;
        if leaf.self_leaf && !matches!(target, Target::Module(_) | Target::Std(_)) {
            return Err(format!("`{}::self` is not a module", names.join("::")));
        }
        Ok(target)
    }

    fn walk_in(
        &self,
        module: usize,
        names: &[&str],
        last: Wanted,
        seen: &mut BTreeSet<(usize, String, bool)>,
    ) -> Result<Target, String> {
        let Some((first, rest)) = names.split_first() else {
            return Err("an empty path".to_string());
        };
        let wanted = |at: usize| {
            if at + 1 == names.len() {
                last
            } else {
                Wanted::Module
            }
        };
        let mut target = match *first {
            name if STD_CRATES.contains(&name) => Target::Std(name.to_string()),
            "crate" => Target::Module(0),
            "self" => Target::Module(module),
            "super" => Target::Module(self.parent(module)?),
            name => self.resolve_name_in(module, name, wanted(0), seen)?,
        };
        for (at, &name) in rest.iter().enumerate() {
            target = match target {
                Target::Std(path) => Target::Std(format!("{path}::{name}")),
                Target::Module(m) => match name {
                    "self" => Target::Module(m),
                    "super" => Target::Module(self.parent(m)?),
                    _ => self.resolve_name_in(m, name, wanted(at + 1), seen)?,
                },
                Target::Item(..) => {
                    return Err(format!(
                        "the path `{}` goes through an item",
                        names.join("::")
                    ))
                }
            };
        }
        Ok(target)
    }

    fn parent(&self, module: usize) -> Result<usize, String> {
        self.modules[module]
            .parent
            .ok_or_else(|| "`super` at the root".to_string())
    }
}

/// Reads the tree of a `use` declaration into its leaves.
struct UseReader<'t> {
    tokens: &'t [Token],
    at: usize,
    /// The `;`.
    end: usize,
}

impl UseReader<'_> {
    fn text(&self) -> &str {
        match self.at < self.end {
            true => &self.tokens[self.at].text,
            false => "",
        }
    }

    fn fail(&self, what: &str) -> String {
        let line = self.tokens[self.at.min(self.end)].line;
        format!("line {line}: {what} in a `use`")
    }

    fn tree(
        &mut self,
        prefix: &mut Vec<(String, usize)>,
        out: &mut Vec<Leaf>,
    ) -> Result<(), String> {
        match self.text() {
            "{" => {
                self.at += 1;
                while self.text() != "}" {
                    let mut inner = prefix.clone();
                    self.tree(&mut inner, out)?;
                    match self.text() {
                        "," => self.at += 1,
                        "}" => {}
                        _ => return Err(self.fail("an unexpected token")),
                    }
                }
                self.at += 1;
                Ok(())
            }
            "*" => Err(self.fail("a glob import")),
            "::" if prefix.is_empty() => Err(self.fail("a path that starts with `::`")),
            _ => {
                if self.at >= self.end || self.tokens[self.at].kind != Kind::Ident {
                    return Err(self.fail("an unexpected token"));
                }
                let (name, token) = (self.text().to_string(), self.at);
                self.at += 1;
                if self.text() == "::" {
                    self.at += 1;
                    prefix.push((name, token));
                    return self.tree(prefix, out);
                }
                let alias = if self.text() == "as" {
                    self.at += 1;
                    let alias = (self.text().to_string(), self.at);
                    self.at += 1;
                    Some(alias)
                } else {
                    None
                };
                let self_leaf = name == "self";
                let mut path = prefix.clone();
                if self_leaf {
                    if path.is_empty() {
                        return Err(self.fail("a `self` with no path"));
                    }
                } else {
                    path.push((name, token));
                }
                out.push(Leaf {
                    path,
                    self_leaf,
                    last: token,
                    alias,
                });
                Ok(())
            }
        }
    }
}

/// The keywords of Rust 2021, which are never a name of the bundle.
pub fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}
