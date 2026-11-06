//! Puts every item of the bundle at its root, the minifier's second stage
//! (ADR 0045, ADR 0046): once there is one module, `mod name { }`, the
//! `use` declarations of the bundle's own items, the module paths
//! (`crate::cg_core::rng::Rng` becomes `Rng`) and every `pub` and
//! `pub(crate)` say nothing, and they go. The imports from std, `core` and
//! `alloc` are written once at the top, grouped by their path
//! (`use std::io::{self,BufRead,Write};`). Comments go too, since a doc
//! comment of a module would be left without its item.
//!
//! At the root, names of different modules meet, so some get a new name
//! first, made of the old one and a number (`play` becomes `play1`), and
//! the renaming stage shortens them later like any other:
//!
//! - **Clashes.** A name that two modules bind to different things (two
//!   functions `play`, an import of `std::fmt::Write` and one of
//!   `std::io::Write`): the first in the bundle keeps it, the others get
//!   new names.
//! - **Patterns and the prelude.** A constant, a static, a struct or a std
//!   import whose name appears in a module that does not bind it, and an
//!   item named like a name of std's prelude or a std crate that appears in
//!   such a module: there the name was a local binding or the prelude's,
//!   and at the root it could become the item (`N =>` would match the
//!   constant instead of binding any value).
//!
//! A new name replaces the old one in the module that binds it: at the
//! definition (or the import, with `as` for a std one), and at every
//! occurrence of the bare name in the module's own code, except where it is
//! a member (after `.`, a field or variant of a type defined there, a method
//! of an `impl` or a `trait`, a field in a struct literal or pattern), a
//! macro or in an attribute; and in the paths of other modules that lead to
//! it. Renaming every bare occurrence in one module, local bindings
//! included, keeps the meaning there, since a module sees only its own
//! names and the prelude, and none of these is a prelude name. The renamed
//! bundle, modules still in place, must compile: a name that a format
//! string captures, or a field written like a local, would not.
//!
//! **The check.** Every item that can carry it is marked `#[deprecated]`,
//! and rustc reports each use of each one with its span and the item's path
//! ([`probe`]). The bundle before flattening and the flattened one, both
//! marked, must report the same item at the same token, every time: a name
//! that would reach another item, a method call that a newly visible trait
//! would take, a pattern that would become a constant, all show. What the
//! check cannot see is a call that would move to a method of a std trait,
//! the bundle's impls of it included (trait impl items carry no mark),
//! because that trait is now in scope in every module: for that, the trait
//! must have a method with the name of the one called that applies to the
//! receiver at an earlier step of the method lookup than it. The answers
//! check of each release (ADR 0045, decision 4) could show such a change, in
//! the games it plays.
//!
//! The flattener reads the forms the bundles use: modules with a body, `use`
//! trees without globs, paths that start with `crate`, `self`, `super` or a
//! module's name, `thread_local!` at module level ([`tree`]). When it meets
//! another form, or when a compile or the check fails, the bundle is left
//! as it is and the [`Report`] says why; the tests check that every bot of
//! the workspace is flattened.

mod probe;
mod tree;

pub use tree::{Brace, Shape, Target, Tree};

use std::collections::{BTreeMap, BTreeSet};

use crate::minify::{first_difference, tokens, Kind, Token};
use crate::prune::{Compiler, Outcome, Rustc};
use crate::rename::StdNames;
use tree::{is_keyword, Wanted, NONE, STD_CRATES};

/// What a flattening did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Modules removed.
    pub modules: usize,
    /// `use` declarations removed.
    pub uses: usize,
    /// Names given a new name, each old name with its new names.
    pub renamed: BTreeMap<String, BTreeSet<String>>,
    /// Why the bundle was left as it is, when it was.
    pub skipped: Option<String>,
    /// Runs of rustc.
    pub compiles: usize,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(why) = &self.skipped {
            return write!(f, "not flattened: {why}; {} compiles", self.compiles);
        }
        let renamed: usize = self.renamed.values().map(BTreeSet::len).sum();
        write!(
            f,
            "removed {} modules and {} `use` declarations, renamed {} names, {} compiles",
            self.modules, self.uses, renamed, self.compiles
        )
    }
}

/// Changes to the tokens of a text.
#[derive(Clone, Debug, Default)]
pub struct Edits {
    /// New texts of tokens.
    pub replace: BTreeMap<usize, String>,
    /// Tokens removed.
    pub delete: BTreeSet<usize>,
    /// Text written before a token (or at the end, after the last one).
    pub before: BTreeMap<usize, Vec<String>>,
}

impl Edits {
    /// These edits and then `other`'s.
    fn and(&self, other: &Edits) -> Edits {
        let mut all = self.clone();
        all.replace.extend(other.replace.clone());
        all.delete.extend(other.delete.iter().copied());
        for (&at, texts) in &other.before {
            all.before
                .entry(at)
                .or_default()
                .extend(texts.iter().cloned());
        }
        all
    }
}

/// A text written from tokens and edits.
#[derive(Clone, Debug)]
pub struct Rendered {
    pub text: String,
    /// The offset in `text` of each source token that it holds.
    starts: BTreeMap<usize, usize>,
}

impl Rendered {
    /// The source token that starts at `offset` of the text.
    pub fn token_at(&self, offset: usize) -> Option<usize> {
        self.starts.get(&offset).copied()
    }
}

/// Writes `tokens` of `source` with `edits`. Tokens that were neighbours
/// keep the whitespace between them, without comments; others get a space
/// or a newline. It fails if the result does not read back as the tokens
/// written.
pub fn render(source: &str, tokens: &[Token], edits: &Edits) -> Result<Rendered, String> {
    let mut text = String::with_capacity(source.len());
    let mut expected: Vec<Token> = Vec::new();
    let mut starts = BTreeMap::new();
    // The last source token written, if it is the last thing written, and
    // the end of the last source token passed.
    let mut last: Option<usize> = None;
    let mut passed = 0;
    for i in 0..=tokens.len() {
        for piece in edits.before.get(&i).into_iter().flatten() {
            if !text.is_empty() {
                text.push('\n');
            }
            expected.extend(tokens_of(piece)?);
            text.push_str(piece);
            last = None;
        }
        let Some(token) = tokens.get(i) else {
            break;
        };
        if edits.delete.contains(&i) {
            continue;
        }
        let gap = match last {
            Some(prev) if prev + 1 == i => {
                let gap = &source[tokens[prev].end..token.start];
                if gap.trim().is_empty() {
                    gap
                } else {
                    "\n"
                }
            }
            _ if source[passed.min(token.start)..token.start].contains('\n') => "\n",
            _ => " ",
        };
        if !text.is_empty() {
            text.push_str(gap);
        }
        let piece = edits
            .replace
            .get(&i)
            .map_or(token.text.as_str(), String::as_str);
        starts.insert(text.len(), i);
        expected.extend(tokens_of(piece)?);
        text.push_str(piece);
        last = Some(i);
        passed = token.end;
    }
    text.push('\n');
    let got = tokens_of(&text)?;
    if let Some(at) = first_difference(&expected, &got) {
        return Err(format!(
            "the flattened text does not read back: token {at} became `{}`",
            got.get(at).map_or("the end", |t| t.text.as_str())
        ));
    }
    Ok(Rendered { text, starts })
}

fn tokens_of(text: &str) -> Result<Vec<Token>, String> {
    tokens(text).map_err(|err| format!("cannot read `{text}`: {err}"))
}

/// A name that can get a new one: an item, by its module and name, or a
/// std path imported under a name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Item(usize, String),
    Std(String, String),
}

/// What flattening a bundle changes, worked out from its tokens alone.
pub struct Plan {
    pub shape: Shape,
    pub tree: Tree,
    /// What each name of each module stands for, modules aside.
    pub bindings: Vec<BTreeMap<String, Target>>,
    /// The modules that each module names: its children and the modules it
    /// imports.
    pub module_names: Vec<BTreeMap<String, usize>>,
    /// The paths through modules.
    paths: Vec<ModulePath>,
    /// The name at the root of each name that changes.
    pub flat: BTreeMap<Key, String>,
    /// The new names, in the modules still in place.
    pub renames: Edits,
    /// The removals and the std imports at the top.
    pub flatten: Edits,
    /// The tokens of the `use` declarations at module level.
    pub use_tokens: BTreeSet<usize>,
}

impl Plan {
    pub fn of(tokens: &[Token]) -> Result<Plan, String> {
        let shape = Shape::of(tokens)?;
        let tree = Tree::of(tokens, &shape)?;
        let std = StdNames::get();

        // What each module binds, and what each `use` leaf stands for.
        let mut bindings = Vec::new();
        let mut module_names = Vec::new();
        let mut leaf_targets: Vec<Vec<Target>> = Vec::new();
        for (m, module) in tree.modules.iter().enumerate() {
            let mut bound = BTreeMap::new();
            for name in module.items.keys() {
                bound.insert(name.clone(), Target::Item(m, name.clone()));
            }
            let mut modules = module.children.clone();
            let mut targets = Vec::new();
            for leaf in module.uses.iter().flat_map(|u| &u.leaves) {
                let target = tree.resolve_leaf(m, leaf)?;
                let line = tokens[leaf.last].line;
                if leaf.alias.is_some() && !matches!(target, Target::Std(_)) {
                    if leaf.binding().is_some() {
                        return Err(format!(
                            "line {line}: an item of the bundle imported with `as`"
                        ));
                    }
                } else if let Some(name) = leaf.binding() {
                    let twice = match &target {
                        Target::Module(child) => modules.insert(name.to_string(), *child).is_some(),
                        _ => bound.insert(name.to_string(), target.clone()).is_some(),
                    };
                    if twice {
                        return Err(format!("line {line}: `{name}` bound twice"));
                    }
                }
                targets.push(target);
            }
            bindings.push(bound);
            module_names.push(modules);
            leaf_targets.push(targets);
        }
        let key = |name: &str, target: &Target| match target {
            Target::Item(m, item) => Some(Key::Item(*m, item.clone())),
            Target::Std(path) => Some(Key::Std(path.clone(), name.to_string())),
            Target::Module(_) => None,
        };

        // The tokens that are not code of a module's own: attributes and
        // `use` declarations.
        let mut use_tokens = BTreeSet::new();
        for module in &tree.modules {
            for u in &module.uses {
                use_tokens.extend(u.tokens.0..=u.tokens.1);
            }
        }
        let own = |m: usize| -> Vec<usize> {
            let module = &tree.modules[m];
            let mut skip = BTreeSet::new();
            for &child in module.children.values() {
                let child = &tree.modules[child];
                if let (Some((start, _)), Some(close)) = (child.opener, child.closer) {
                    skip.extend(start..=close);
                }
            }
            (module.body.0..module.body.1)
                .filter(|i| !skip.contains(i) && !use_tokens.contains(i) && !shape.attribute[*i])
                .collect()
        };
        let owned: Vec<Vec<usize>> = (0..tree.modules.len()).map(own).collect();
        // The bare names of each module: not after `.` or `::`, not a macro.
        let bare = |i: usize| -> bool {
            let token = &tokens[i];
            token.kind == Kind::Ident
                && !is_keyword(&token.text)
                && !(i > 0 && matches!(tokens[i - 1].text.as_str(), "." | "::"))
                && tokens.get(i + 1).is_none_or(|t| t.text != "!")
        };

        // The first place of each key, which decides who keeps a name.
        let mut first: BTreeMap<Key, usize> = BTreeMap::new();
        for (m, bound) in bindings.iter().enumerate() {
            for (name, target) in bound {
                let Some(key) = key(name, target) else {
                    continue;
                };
                let at = match target {
                    Target::Item(owner, item) => tree.modules[*owner].items[item].name,
                    _ => tree.modules[m]
                        .uses
                        .iter()
                        .flat_map(|u| &u.leaves)
                        .find(|leaf| leaf.binding() == Some(name))
                        .map_or(NONE, |leaf| leaf.last),
                };
                let entry = first.entry(key).or_insert(at);
                *entry = (*entry).min(at);
            }
        }
        let mut by_name: BTreeMap<&str, Vec<(usize, &Key)>> = BTreeMap::new();
        for (key, &at) in &first {
            let name = match key {
                Key::Item(_, name) | Key::Std(_, name) => name.as_str(),
            };
            by_name.entry(name).or_default().push((at, key));
        }

        // New names: of clashes, then of names that could be captured.
        let mut used: BTreeSet<String> = tokens
            .iter()
            .filter(|t| t.kind == Kind::Ident)
            .map(|t| t.text.clone())
            .collect();
        let mut fresh = |name: &str| -> String {
            let mut n = 1;
            loop {
                let candidate = format!("{name}{n}");
                if !used.contains(&candidate) && !std.has(&candidate) {
                    used.insert(candidate.clone());
                    return candidate;
                }
                n += 1;
            }
        };
        let mut flat: BTreeMap<Key, String> = BTreeMap::new();
        for (name, keys) in &mut by_name {
            keys.sort();
            for (index, (_, key)) in keys.iter().enumerate() {
                let new = if index == 0 {
                    name.to_string()
                } else {
                    fresh(name)
                };
                flat.insert((*key).clone(), new);
            }
        }
        for (key, new) in flat.iter_mut() {
            // A std import can be a constant or a variant, which are
            // written with a capital.
            let (name, risky) = match key {
                Key::Item(m, name) => (name, tree.modules[*m].items[name].kind.in_patterns()),
                Key::Std(_, name) => (name, name.starts_with(|c: char| c.is_uppercase())),
            };
            if new != name {
                continue;
            }
            let special = std.prelude(name) || STD_CRATES.contains(&name.as_str());
            if !risky && !special {
                continue;
            }
            let captured = (0..tree.modules.len()).any(|m| {
                !bindings[m].contains_key(name)
                    && owned[m].iter().any(|&i| {
                        bare(i)
                            && &tokens[i].text == name
                            && !(tokens.get(i + 1).is_some_and(|t| t.text == "::")
                                && module_names[m].contains_key(name))
                    })
            });
            if captured {
                *new = fresh(name);
            }
        }

        let mut plan = Plan {
            shape,
            tree,
            bindings,
            module_names,
            paths: Vec::new(),
            flat,
            renames: Edits::default(),
            flatten: Edits::default(),
            use_tokens,
        };
        plan.paths = plan.module_paths(tokens, &owned)?;
        let members = plan.members(tokens);
        plan.rename_edits(tokens, &owned, &leaf_targets, &members, &bare)?;
        plan.flatten_edits(tokens, &leaf_targets)?;
        Ok(plan)
    }

    /// The edits that flatten: the new names, but those in `use`
    /// declarations, which go, then the removals.
    pub fn flat_edits(&self) -> Edits {
        let mut renames = self.renames.clone();
        renames.before.retain(|at, _| !self.use_tokens.contains(at));
        renames.and(&self.flatten)
    }

    /// The new name of `name` bound to `target`, if it changes.
    fn new_name(&self, name: &str, target: &Target) -> Option<&String> {
        let key = match target {
            Target::Item(m, item) => Key::Item(*m, item.clone()),
            Target::Std(path) => Key::Std(path.clone(), name.to_string()),
            Target::Module(_) => return None,
        };
        self.flat.get(&key).filter(|new| *new != name)
    }

    /// The bare names that are members, which a new name leaves alone: a
    /// method or constant of an `impl` or a `trait`, a field or variant
    /// where a type defines it, a field in a struct literal or pattern.
    fn members(&self, tokens: &[Token]) -> BTreeSet<usize> {
        let shape = &self.shape;
        let mut members = BTreeSet::new();
        for (&brace, &kind) in &shape.brace {
            let close = shape.pair[brace];
            let children = (brace + 1..close).filter(|&i| shape.parent[i] == brace);
            match kind {
                Brace::Impl | Brace::TraitImpl | Brace::Trait => {
                    for i in children {
                        if matches!(tokens[i].text.as_str(), "fn" | "const" | "type")
                            && tokens.get(i + 1).is_some_and(|t| t.kind == Kind::Ident)
                        {
                            members.insert(i + 1);
                        }
                    }
                }
                Brace::Fields => {
                    // The first name of each slot, after its attributes and
                    // visibility; `<` and `>` count, since a type's `,` can
                    // be between them.
                    let (mut angle, mut slot) = (0i32, true);
                    for i in children {
                        let text = tokens[i].text.as_str();
                        let visibility = text == "pub"
                            || (text == "(" && tokens[i - 1].text == "pub")
                            || (text == ")" && tokens[shape.pair[i] - 1].text == "pub");
                        if shape.attribute[i] || (slot && visibility) {
                            continue;
                        }
                        match text {
                            "<" => angle += 1,
                            ">" => angle -= 1,
                            ">>" => angle -= 2,
                            _ => {}
                        }
                        if text == "," && angle == 0 {
                            slot = true;
                        } else {
                            if slot && tokens[i].kind == Kind::Ident {
                                members.insert(i);
                            }
                            slot = false;
                        }
                    }
                }
                Brace::Other => {
                    // `{ name:` and `, name:` in a struct literal or
                    // pattern, unless the `|` so far are odd: a closure's
                    // parameters.
                    let mut bars = 0;
                    let mut prev = brace;
                    for i in children {
                        if tokens[i].text == "|" {
                            bars += 1;
                        }
                        let after_slot = prev == brace || tokens[prev].text == ",";
                        if after_slot
                            && bars % 2 == 0
                            && tokens[i].kind == Kind::Ident
                            && tokens.get(i + 1).is_some_and(|t| t.text == ":")
                        {
                            members.insert(i);
                        }
                        prev = i;
                    }
                }
                Brace::Module | Brace::Fn => {}
            }
        }
        members
    }

    /// The new names, written in the modules still in place.
    fn rename_edits(
        &mut self,
        tokens: &[Token],
        owned: &[Vec<usize>],
        leaf_targets: &[Vec<Target>],
        members: &BTreeSet<usize>,
        bare: &dyn Fn(usize) -> bool,
    ) -> Result<(), String> {
        let mut edits = Edits::default();
        let in_paths: BTreeSet<usize> = self.paths.iter().flat_map(|p| p.start..p.item).collect();
        for (m, module) in self.tree.modules.iter().enumerate() {
            // Bare names in the module's code.
            for &i in &owned[m] {
                if !bare(i) || members.contains(&i) || in_paths.contains(&i) {
                    continue;
                }
                let name = tokens[i].text.as_str();
                if let Some(new) = self.bindings[m]
                    .get(name)
                    .and_then(|target| self.new_name(name, target))
                {
                    edits.replace.insert(i, new.clone());
                }
            }
            // The imports.
            let leaves = module.uses.iter().flat_map(|u| &u.leaves);
            for (leaf, target) in leaves.zip(&leaf_targets[m]) {
                let Some(name) = leaf.binding() else {
                    continue;
                };
                let Some(new) = self.new_name(name, target) else {
                    continue;
                };
                match (target, &leaf.alias) {
                    (Target::Std(_), Some((_, at))) => {
                        edits.replace.insert(*at, new.clone());
                    }
                    (Target::Std(_), None) => {
                        edits
                            .before
                            .entry(leaf.last + 1)
                            .or_default()
                            .push(format!("as {new}"));
                    }
                    _ => {
                        edits.replace.insert(leaf.last, new.clone());
                    }
                }
            }
        }
        // Paths through modules.
        for path in &self.paths {
            if let Some(new) = &path.new {
                edits.replace.insert(path.item, new.clone());
            }
        }
        self.renames = edits;
        Ok(())
    }

    /// The removals: modules, `use` declarations, visibilities and the
    /// module parts of paths; and the std imports, written at the top.
    fn flatten_edits(
        &mut self,
        tokens: &[Token],
        leaf_targets: &[Vec<Target>],
    ) -> Result<(), String> {
        let mut edits = Edits::default();
        let mut imports: BTreeSet<(String, String)> = BTreeSet::new();
        for (m, module) in self.tree.modules.iter().enumerate() {
            if let (Some((start, open)), Some(close)) = (module.opener, module.closer) {
                edits.delete.extend(start..=open);
                edits.delete.insert(close);
            }
            for u in &module.uses {
                edits.delete.extend(u.tokens.0..=u.tokens.1);
            }
            let leaves = module.uses.iter().flat_map(|u| &u.leaves);
            for (leaf, target) in leaves.zip(&leaf_targets[m]) {
                if let Target::Std(path) = target {
                    let name = match leaf.binding() {
                        Some(name) => self.new_name(name, target).map_or(name, String::as_str),
                        None => "_",
                    };
                    imports.insert((path.clone(), name.to_string()));
                }
            }
        }
        // Visibilities, everywhere.
        for (i, token) in tokens.iter().enumerate() {
            if token.text != "pub" || token.kind != Kind::Ident || self.shape.attribute[i] {
                continue;
            }
            edits.delete.insert(i);
            let restricted = tokens.get(i + 1).is_some_and(|t| t.text == "(")
                && tokens
                    .get(i + 2)
                    .is_some_and(|t| matches!(t.text.as_str(), "crate" | "self" | "super" | "in"));
            if restricted {
                edits.delete.extend(i + 1..=self.shape.pair[i + 1]);
            }
        }
        for path in &self.paths {
            if !edits.delete.contains(&path.start) {
                edits.delete.extend(path.start..path.item);
            }
        }
        // `use` inside function bodies must not name the bundle's modules.
        for (i, token) in tokens.iter().enumerate() {
            let in_body = token.text == "use"
                && !self.use_tokens.contains(&i)
                && self.shape.item_start(tokens, i).is_some();
            if in_body {
                let first = tokens.get(i + 1).map_or("", |t| t.text.as_str());
                if !STD_CRATES.contains(&first) {
                    return Err(format!("line {}: a `use` in a function body", token.line));
                }
            }
        }
        let header = std_imports(&imports)?;
        if !header.is_empty() {
            edits.before.entry(0).or_default().push(header);
        }
        self.flatten = edits;
        Ok(())
    }

    /// The paths that start with `crate`, `self`, `super` or a module's
    /// name, outside `use` declarations and attributes.
    fn module_paths(
        &self,
        tokens: &[Token],
        owned: &[Vec<usize>],
    ) -> Result<Vec<ModulePath>, String> {
        let mut paths = Vec::new();
        for (m, own) in owned.iter().enumerate() {
            for &i in own {
                let token = &tokens[i];
                // `pub(in path)` goes with its `pub`.
                let restriction = i > 0
                    && tokens[i - 1].text == "in"
                    && self.shape.parent[i] != NONE
                    && self.shape.parent[i] > 0
                    && tokens[self.shape.parent[i] - 1].text == "pub";
                let starts = token.kind == Kind::Ident
                    && tokens.get(i + 1).is_some_and(|t| t.text == "::")
                    && !(i > 0 && matches!(tokens[i - 1].text.as_str(), "::" | "." | "$"))
                    && !restriction;
                if !starts {
                    continue;
                }
                let mut module = match token.text.as_str() {
                    "crate" => 0,
                    "self" => m,
                    "super" => self.tree.modules[m]
                        .parent
                        .ok_or_else(|| format!("line {}: `super` at the root", token.line))?,
                    name => match self.module_names[m].get(name) {
                        Some(&child) => child,
                        None => continue,
                    },
                };
                let mut j = i;
                let item = loop {
                    let segment = tokens
                        .get(j + 2)
                        .filter(|t| tokens[j + 1].text == "::" && t.kind == Kind::Ident);
                    let Some(segment) = segment else {
                        return Err(format!("line {}: a path that ends at a module", token.line));
                    };
                    j += 2;
                    let target = match segment.text.as_str() {
                        "self" => Target::Module(module),
                        "super" => {
                            Target::Module(self.tree.modules[module].parent.ok_or_else(|| {
                                format!("line {}: `super` at the root", token.line)
                            })?)
                        }
                        name => {
                            let more = tokens.get(j + 1).is_some_and(|t| t.text == "::")
                                && tokens.get(j + 2).is_some_and(|t| t.kind == Kind::Ident);
                            let wanted = if more { Wanted::Module } else { Wanted::Item };
                            self.tree.resolve_name(module, name, wanted)?
                        }
                    };
                    match target {
                        Target::Module(next) => module = next,
                        target => break (j, target),
                    }
                };
                let (item, target) = item;
                let new = self.new_name(&tokens[item].text, &target).cloned();
                paths.push(ModulePath {
                    start: i,
                    item,
                    new,
                });
            }
        }
        Ok(paths)
    }

    /// `item`, as rustc names an item in a message about the bundle with
    /// modules (`a::b::Board::play`, `<impl a::State>::read`), as it reads
    /// at the root: each path in it with [`Plan::flat_path`].
    pub fn flat_item(&self, item: &str) -> String {
        let mut out = String::new();
        let mut run = String::new();
        let is_path = |c: char| c.is_alphanumeric() || c == '_' || c == ':';
        for c in item.chars().chain(std::iter::once(' ')) {
            if is_path(c) {
                run.push(c);
                continue;
            }
            if !run.is_empty() {
                match run.starts_with(':') {
                    true => out.push_str(&run),
                    false => out.push_str(&self.flat_path(&run)),
                }
                run.clear();
            }
            out.push(c);
        }
        out.pop();
        out
    }

    /// `path`, a path that rustc writes in a message about the bundle with
    /// modules, as it reads at the root.
    pub fn flat_path(&self, path: &str) -> String {
        let mut segments: Vec<&str> = path.split("::").collect();
        let mut module = 0;
        while segments.len() > 1 {
            match self.tree.modules[module].children.get(segments[0]) {
                Some(&child) => {
                    module = child;
                    segments.remove(0);
                }
                None => break,
            }
        }
        let first = segments[0].to_string();
        let new = self
            .flat
            .get(&Key::Item(module, first.clone()))
            .cloned()
            .unwrap_or(first);
        std::iter::once(new.as_str())
            .chain(segments[1..].iter().copied())
            .collect::<Vec<_>>()
            .join("::")
    }
}

/// A path through modules to an item.
struct ModulePath {
    /// Its first token.
    start: usize,
    /// The token of the item's name.
    item: usize,
    /// The item's new name, if it has one.
    new: Option<String>,
}

/// The `use` declarations of the std `imports` (path, name or `_`), one per
/// parent path: `use std::io::{self,BufRead,Write};`.
fn std_imports(imports: &BTreeSet<(String, String)>) -> Result<String, String> {
    let named: BTreeSet<&str> = imports
        .iter()
        .filter(|(_, name)| name != "_")
        .map(|(path, _)| path.as_str())
        .collect();
    let mut groups: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for (path, name) in imports {
        if name == "_" && named.contains(path.as_str()) {
            continue;
        }
        let (parent, last) = path
            .rsplit_once("::")
            .ok_or_else(|| format!("an import of a whole crate: `{path}`"))?;
        let leaf = if name == last {
            last.to_string()
        } else {
            format!("{last} as {name}")
        };
        groups.entry(parent).or_default().insert(leaf);
    }
    // A module imported with names under it is `self` in their group.
    let parents: Vec<&str> = groups.keys().copied().collect();
    for path in parents {
        let Some((parent, last)) = path.rsplit_once("::") else {
            continue;
        };
        let Some(siblings) = groups.get_mut(parent) else {
            continue;
        };
        if siblings.remove(last) {
            groups
                .get_mut(path)
                .expect("listed")
                .insert("self".to_string());
        }
    }
    let mut text = String::new();
    for (parent, leaves) in groups {
        if leaves.is_empty() {
            continue;
        }
        let mut leaves: Vec<String> = leaves.into_iter().collect();
        // `self` first, as rustfmt writes it.
        leaves.sort_by_key(|leaf| (leaf != "self", leaf.clone()));
        match leaves.as_slice() {
            [one] => text.push_str(&format!("use {parent}::{one};\n")),
            _ => text.push_str(&format!("use {parent}::{{{}}};\n", leaves.join(","))),
        }
    }
    Ok(text.trim_end().to_string())
}

/// `source` flattened, and what was done. A bundle that the flattener cannot
/// read, or whose flattening rustc or the check rejects, comes back as it
/// is, with the reason in the report. It fails only if rustc cannot be run
/// or `source` cannot be lexed.
pub fn flatten(source: &str, rustc: &Rustc) -> Result<(String, Report), String> {
    let tokens = tokens(source).map_err(|err| format!("cannot read the bundle: {err}"))?;
    let mut compiler = Compiler::new(rustc)?;
    match attempt(source, &tokens, &mut compiler)? {
        Ok((text, mut report)) => {
            report.compiles = compiler.compiles();
            Ok((text, report))
        }
        Err(why) => Ok((
            source.to_string(),
            Report {
                skipped: Some(why),
                compiles: compiler.compiles(),
                ..Report::default()
            },
        )),
    }
}

/// Flattens `source`: the outer error is rustc's that cannot run, the
/// inner one why the bundle stays as it is.
fn attempt(
    source: &str,
    tokens: &[Token],
    compiler: &mut Compiler<'_>,
) -> Result<Result<(String, Report), String>, String> {
    let plan = match Plan::of(tokens) {
        Ok(plan) => plan,
        Err(why) => return Ok(Err(why)),
    };
    let mut marked = Edits::default();
    for at in probe::marks(tokens, &plan.shape) {
        marked
            .before
            .entry(at)
            .or_default()
            .push(probe::MARK.to_string());
    }
    let flat = plan.flat_edits();

    // The renamed bundle, with its modules.
    if !plan.renames.replace.is_empty() || !plan.renames.before.is_empty() {
        let renamed = match render(source, tokens, &plan.renames) {
            Ok(renamed) => renamed,
            Err(why) => return Ok(Err(why)),
        };
        if let Outcome::Failed { errors, .. } = compiler.compile(&renamed.text)? {
            let first = errors.lines().next().unwrap_or("").to_string();
            return Ok(Err(format!("the new names do not compile: {first}")));
        }
    }

    // The check: the same uses before and after.
    let mut found = Vec::new();
    for (edits, before) in [(marked.clone(), true), (flat.and(&marked), false)] {
        let rendered = match render(source, tokens, &edits) {
            Ok(rendered) => rendered,
            Err(why) => return Ok(Err(why)),
        };
        let diagnostics = match compiler.compile_with(&rendered.text, &[probe::LINT])? {
            Outcome::Compiled(diagnostics) => diagnostics,
            Outcome::Failed { errors, .. } => {
                let which = if before {
                    "the marked bundle"
                } else {
                    "the flattened bundle"
                };
                let first = errors.lines().next().unwrap_or("").to_string();
                return Ok(Err(format!("{which} does not compile: {first}")));
            }
        };
        let skip = |token: usize| plan.use_tokens.contains(&token);
        found.push(match before {
            true => probe::uses(&diagnostics, &rendered, skip, |item| plan.flat_item(item)),
            false => probe::uses(&diagnostics, &rendered, skip, str::to_string),
        });
    }
    if let Some(difference) = probe::difference(&found[0], &found[1], tokens) {
        return Ok(Err(format!(
            "the meaning of a name would change: {difference}"
        )));
    }

    let text = match render(source, tokens, &flat) {
        Ok(rendered) => rendered.text,
        Err(why) => return Ok(Err(why)),
    };
    let mut renamed: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (key, new) in &plan.flat {
        let (Key::Item(_, name) | Key::Std(_, name)) = key;
        if new != name {
            renamed.entry(name.clone()).or_default().insert(new.clone());
        }
    }
    let report = Report {
        modules: plan.tree.modules.len() - 1,
        uses: plan.tree.modules.iter().map(|m| m.uses.len()).sum(),
        renamed,
        skipped: None,
        compiles: 0,
    };
    Ok(Ok((text, report)))
}

#[cfg(test)]
mod tests;
