//! Shortens the names of the bundle and some of its words (ADR 0045, ADR
//! 0046). It works on tokens ([`crate::minify::tokens`]), and makes five
//! kinds of changes:
//!
//! - **Names.** A name the bundle defines gets a shorter name: every
//!   identifier with the text `score` becomes `a`, wherever it is, except at
//!   the places listed below. The new name is in no token of the bundle and
//!   in no name the standard library defines.
//! - **`self`.** In a method that takes `self` and uses it often enough, the
//!   body starts with `let s=self;` (`let mut s=self;` for `mut self`) and
//!   its other `self` tokens become `s`, one name for every method, used
//!   nowhere else. `self::` paths keep `self`, and a body is left as it is
//!   when it holds a `fn`, an `impl` or a `macro_rules!` (they can have a
//!   `self` of their own), a string that formats `{self}` (it would read a
//!   stale `self` in a `mut self` method of a `Copy` type), or a `self` in
//!   an attribute, a text macro or the arguments of a macro of the bundle
//!   (a rule may match the word `self`).
//! - **Type aliases.** Each of the types of [`ALIASED`] (`usize`, `u8`,
//!   `String`, `Instant`, `Vec<T>`, `Option<T>`...) gets one alias name, and
//!   a module where its uses save more than the declaration costs declares
//!   it (`type U=usize;`, `type V<T>=Vec<T>;`) and uses it. A use that is a
//!   path's later segment (`std::time::Instant`), a member, a field, the
//!   name of an item, or in an attribute, a `use` declaration, a
//!   `macro_rules!` definition, the arguments of a macro that turns its
//!   tokens into text or those of a macro of the bundle (a rule may match
//!   the word `usize`) keeps the type's name.
//! - **Numbers** lose their digit separators: `100_000` becomes `100000`.
//!
//! A name is kept as it is, everywhere, when it is:
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
//!   method lookup skips the others): `x.len()`, `u32::MAX`, `fn fmt` in an
//!   `impl Display`, `type Item`. Wherever it is, such a name may mean std's
//!   member, and renaming the bundle's own would make the call reach the
//!   bundle's instead (a trait of the bundle implemented for a std type)
//!   without an error; also in an associated type binding
//!   (`Iterator<Item = u8>`);
//! - the last segment of a path that starts with `std::`, `core::` or
//!   `alloc::`, a name a `use` tree of such a path imports (`io` in
//!   `use std::io::{self, Read}`), or a name after `::` in a path whose
//!   first name is one of those, a primitive type, a prelude name or a name
//!   given with `as` to what a std `use` imports, and so on along the path
//!   (`io::stdin` after `use std::io`, `Ordering::Less`,
//!   `Vec::with_capacity`, `u32::MAX`). Only a path's first name leads:
//!   `crate::board::Ordering::Less` is the bundle's own path;
//! - in an attribute (`#[derive(Clone)]`, `#[allow(dead_code)]`);
//! - in braces in a string literal, where formatting macros read names
//!   (`"{score}"`, `"{:width$}"`), or in the arguments of `stringify!`,
//!   `concat!` and `module_path!`, which turn names into text, and of the
//!   bundle's macros that call them;
//! - anywhere, when std defines it (an enum variant included) and the bundle
//!   imports from std with a glob (`use std::cmp::Ordering::*`): a variant
//!   renamed in a pattern would become a binding that matches anything.
//!
//! The middle segments of a std path (`cell` in `std::cell::RefCell`, `io`
//! in `use std::io::{Read, Write}`) keep their names only there: only
//! std's modules can be meant, so the bundle's own `cell` may be renamed
//! elsewhere.
//!
//! The std names are in `rename/std-names.txt`, written from the sources of
//! rustc 1.90.0's standard library by `cargo run -p cg-bundler --example
//! std-names`: the names that std, core and alloc define, with their kinds.
//!
//! The result is compiled with rustc 1.90.0 ([`crate::prune`]'s compiler).
//! Each change is a [`Unit`]: a name, the `self` of one method, or the
//! alias of one type. When the result does not compile, the units whose
//! text is in the errors' spans are undone and the rest is tried again; if
//! the errors name none, the units are halved until those that rustc
//! rejects are found. What compiles is the same program, since only names
//! changed, and `let s=self;` and an alias stand for what they replace. A
//! few things can still see a new name without an error, and none changes
//! what a bot plays: the text of `{:?}` for a type that derives `Debug`
//! (bots write it to stderr only), of `std::any::type_name` (in error
//! messages), and `column!` and `line!` (bots do not use them). The answers
//! check of ADR 0045 (decision 4) would catch a bot whose answers change.
//!
//! New names are as short as can be, counted in UTF-16 units as CodinGame
//! counts them (ADR 0037), and keep the case of the old one, so rustc warns
//! about nothing new: `snake_case` names get `a` to `z`, `CamelCase` names
//! `A` to `Z` and `UPPER_CASE` names the same letters (each name once);
//! then all of them get CJK ideographs from U+4E00 (`一`), one UTF-16 unit
//! each, which have no case; then names of two ASCII characters, and so on.
//! The names that occur most get the letters first. The name of `self` is
//! a lower-case one, an alias a capital one. A name is renamed only when
//! the new one is shorter, `self` and a type only when that saves room. The
//! result depends only on the bundle and on rustc 1.90.0.

mod forms;
mod kept;

pub use forms::{forms, Forms, Method, Uses, ALIASED};
pub use kept::{kept, Kept};

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::cg_size;
use crate::minify::{tokens, Kind, Token};
use crate::prune::{Compiler, Outcome, Rustc};

/// The names of the standard library, written by the example `std-names`.
const STD_NAMES: &str = include_str!("rename/std-names.txt");

/// Error spans longer than this say nothing about one change.
const NAME_SPAN: usize = 64;

/// The first CJK ideograph given as a name, and how many follow it: those
/// of Unicode 1.1, U+4E00 to U+9FA5.
pub const CJK_FIRST: u32 = 0x4E00;
pub const CJK_COUNT: u32 = 0x9FA5 - CJK_FIRST + 1;

/// What a renaming did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Names given a shorter name.
    pub renamed: usize,
    /// The name that stands for `self`, when a method uses one.
    pub self_name: Option<String>,
    /// The methods where `self` has that name.
    pub methods: usize,
    /// Each type given an alias, with its alias and the number of modules
    /// that declare it.
    pub aliases: Vec<(String, String, usize)>,
    /// Changes that rustc rejected, undone.
    pub refused: usize,
    /// Runs of rustc.
    pub compiles: usize,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let declarations: usize = self.aliases.iter().map(|(_, _, n)| n).sum();
        write!(
            f,
            "renamed {} names, `self` in {} methods, {} types aliased in {} declarations, \
             {} changes that do not compile undone, {} compiles",
            self.renamed,
            self.methods,
            self.aliases.len(),
            declarations,
            self.refused,
            self.compiles
        )
    }
}

/// One change, which rustc may reject and which is undone alone.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// A name given a new name.
    Name(String),
    /// `self` given its new name in a method, an index in
    /// [`Forms::methods`].
    Method(usize),
    /// A type given an alias.
    Alias(&'static str),
}

/// The changes of a renaming.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Edits {
    /// Each old name and its new name.
    pub names: BTreeMap<String, String>,
    /// The name of `self`, when it has one.
    pub self_name: Option<String>,
    /// The methods where `self` has that name ([`Forms::methods`]).
    pub methods: BTreeSet<usize>,
    /// Each aliased type and its alias.
    pub aliases: BTreeMap<&'static str, String>,
}

impl Edits {
    /// The units of these changes, in order.
    pub fn units(&self) -> Vec<Unit> {
        let names = self.names.keys().map(|name| Unit::Name(name.clone()));
        let methods = self.methods.iter().map(|&m| Unit::Method(m));
        let aliases = self.aliases.keys().map(|&ty| Unit::Alias(ty));
        names.chain(methods).chain(aliases).collect()
    }

    /// Whether there is no change of a unit (numbers aside).
    pub fn is_empty(&self) -> bool {
        self.names.is_empty() && self.methods.is_empty() && self.aliases.is_empty()
    }

    /// These changes, with only the units in `units`.
    pub fn restrict(&self, units: &BTreeSet<Unit>) -> Edits {
        let methods: BTreeSet<usize> = self
            .methods
            .iter()
            .copied()
            .filter(|&m| units.contains(&Unit::Method(m)))
            .collect();
        Edits {
            names: self
                .names
                .iter()
                .filter(|(old, _)| units.contains(&Unit::Name(old.to_string())))
                .map(|(old, new)| (old.clone(), new.clone()))
                .collect(),
            self_name: self.self_name.clone().filter(|_| !methods.is_empty()),
            methods,
            aliases: self
                .aliases
                .iter()
                .filter(|(ty, _)| units.contains(&Unit::Alias(ty)))
                .map(|(&ty, alias)| (ty, alias.clone()))
                .collect(),
        }
    }
}

/// What the renaming may change in a bundle's tokens.
pub struct Analysis<'t> {
    pub tokens: &'t [Token],
    pub kept: Kept,
    pub forms: Forms,
    /// The names that may be renamed, with the number of places where they
    /// would be, most frequent first (ties in the order of the names).
    pub candidates: Vec<(String, usize)>,
}

impl<'t> Analysis<'t> {
    pub fn of(tokens: &'t [Token], std: &StdNames) -> Analysis<'t> {
        let kept = kept(tokens, std);
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for (at, token) in tokens.iter().enumerate() {
            if token.kind == Kind::Ident
                && !kept.names.contains(&token.text)
                && !kept.positions.contains(&at)
            {
                *counts.entry(token.text.as_str()).or_insert(0) += 1;
            }
        }
        let mut candidates: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(name, count)| (name.to_string(), count))
            .collect();
        candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let mut forms = forms(tokens);
        // A type that the bundle defines itself (its own `Instant`) is
        // renamed, not aliased.
        forms
            .uses
            .retain(|uses| !candidates.iter().any(|(name, _)| name == uses.ty));
        Analysis {
            tokens,
            kept,
            forms,
            candidates,
        }
    }

    /// Every change that may be made.
    pub fn units(&self) -> BTreeSet<Unit> {
        let names = self
            .candidates
            .iter()
            .map(|(name, _)| Unit::Name(name.clone()));
        let methods = (0..self.forms.methods.len()).map(Unit::Method);
        let aliases = self.forms.uses.iter().map(|uses| Unit::Alias(uses.ty));
        names.chain(methods).chain(aliases).collect()
    }
}

/// `source` with the names it defines renamed and `self` and some types
/// shortened, and what was done. It fails if `source` cannot be lexed or
/// rustc cannot be run.
pub fn rename(source: &str, rustc: &Rustc) -> Result<(String, Report), String> {
    let tokens = tokens(source).map_err(|err| format!("cannot read the bundle: {err}"))?;
    let std = StdNames::get();
    let analysis = Analysis::of(&tokens, std);
    let mut active = analysis.units();
    let mut compiler = Compiler::new(rustc)?;
    let mut refused = 0;
    loop {
        let edits = assign(&analysis, &active, std);
        let applied = apply(source, &analysis, &edits);
        let done = |compiler: &Compiler<'_>| {
            let report = Report {
                renamed: edits.names.len(),
                self_name: edits.self_name.clone(),
                methods: edits.methods.len(),
                aliases: applied
                    .declared
                    .iter()
                    .map(|(&ty, &n)| (ty.to_string(), edits.aliases[ty].clone(), n))
                    .collect(),
                refused,
                compiles: compiler.compiles(),
            };
            Ok((applied.text.clone(), report))
        };
        if edits.is_empty() {
            return done(&compiler);
        }
        let spans = match compiler.compile(&applied.text)? {
            Outcome::Compiled(_) => return done(&compiler),
            Outcome::Failed { spans, .. } => spans,
        };
        let mut blamed = applied.blame(&spans);
        if blamed.is_empty() {
            // The errors name no change: halve the changes until those that
            // do not compile are found.
            let units = edits.units();
            let mut accepted = BTreeSet::new();
            let mut settler = Settler {
                compiler: &mut compiler,
                source,
                analysis: &analysis,
                edits: &edits,
            };
            settler.settle(&mut accepted, &units, true)?;
            blamed = units
                .into_iter()
                .filter(|unit| !accepted.contains(unit))
                .collect();
        }
        let before = active.len();
        active.retain(|unit| !blamed.contains(unit));
        refused += before - active.len();
        if active.len() == before {
            // Nothing changed is to blame, yet it fails: change nothing.
            active.clear();
        }
    }
}

/// Finds the units that compile by halving.
struct Settler<'a, 'r, 't> {
    compiler: &'a mut Compiler<'r>,
    source: &'a str,
    analysis: &'a Analysis<'t>,
    edits: &'a Edits,
}

impl Settler<'_, '_, '_> {
    /// Adds to `accepted` as many of `units` as compile together with
    /// `accepted`, halving them until those that do not compile are found.
    /// `failing` says that all of them together are known not to compile.
    fn settle(
        &mut self,
        accepted: &mut BTreeSet<Unit>,
        units: &[Unit],
        failing: bool,
    ) -> Result<(), String> {
        if units.is_empty() {
            return Ok(());
        }
        if !failing {
            let mut trial = accepted.clone();
            trial.extend(units.iter().cloned());
            let edits = self.edits.restrict(&trial);
            let text = apply(self.source, self.analysis, &edits).text;
            if let Outcome::Compiled(_) = self.compiler.compile(&text)? {
                accepted.extend(units.iter().cloned());
                return Ok(());
            }
        }
        if units.len() == 1 {
            return Ok(());
        }
        let (left, right) = units.split_at(units.len() / 2);
        let before = accepted.len();
        self.settle(accepted, left, false)?;
        // If all of the left half went, the right half is what failed.
        let left_went = accepted.len() == before + left.len();
        self.settle(accepted, right, left_went)
    }
}

/// What a name stands for in [`assign`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Request {
    Name(String),
    SelfName,
    Alias(&'static str),
}

/// The next new name of each case: the ASCII letters first, then the CJK
/// ideographs, shared by all cases, then longer ASCII names.
#[derive(Default)]
struct Namer {
    ascii: [usize; 3],
    cjk: u32,
}

impl Namer {
    fn on_cjk(&self, case: Case) -> bool {
        self.ascii[case as usize] >= 26 && self.cjk < CJK_COUNT
    }

    fn peek(&self, case: Case) -> String {
        if self.on_cjk(case) {
            char::from_u32(CJK_FIRST + self.cjk)
                .expect("an ideograph")
                .to_string()
        } else {
            case.name(self.ascii[case as usize])
        }
    }

    fn advance(&mut self, case: Case) {
        if self.on_cjk(case) {
            self.cjk += 1;
        } else {
            self.ascii[case as usize] += 1;
        }
    }
}

/// `let s=self;` for the name `s`.
pub fn self_declaration(name: &str, mutable: bool) -> String {
    let mutable = if mutable { "mut " } else { "" };
    format!("let {mutable}{name}=self;")
}

/// `type U=usize;` for the alias `U` of `usize`, `type V<T>=Vec<T>;` for
/// one that takes a parameter.
pub fn alias_declaration(alias: &str, ty: &str, generic: bool) -> String {
    if generic {
        let param = if alias == "T" { "U" } else { "T" };
        format!("type {alias}<{param}>={ty}<{param}>;")
    } else {
        format!("type {alias}={ty};")
    }
}

/// Whether naming `self` `name` in `method` saves room.
fn self_pays(method: &Method, name: &str) -> bool {
    let saved = method.selves.len() * cg_size("self").saturating_sub(cg_size(name));
    saved > cg_size(&self_declaration(name, method.mutable))
}

/// Whether `uses` uses of `ty` in a module save room with the alias `alias`.
fn alias_pays(uses: usize, ty: &str, generic: bool, alias: &str) -> bool {
    let saved = uses * cg_size(ty).saturating_sub(cg_size(alias));
    saved > cg_size(&alias_declaration(alias, ty, generic))
}

/// Gives the units of `active` their new names, the most frequent first,
/// each the next unused name of its case when that is shorter and saves
/// room.
pub fn assign(analysis: &Analysis<'_>, active: &BTreeSet<Unit>, std: &StdNames) -> Edits {
    let mut taken: BTreeSet<String> = analysis
        .tokens
        .iter()
        .filter(|t| t.kind == Kind::Ident)
        .map(|t| t.text.clone())
        .collect();
    let methods: Vec<usize> = (0..analysis.forms.methods.len())
        .filter(|&m| active.contains(&Unit::Method(m)))
        .collect();
    let mut requests: Vec<(usize, Request)> = Vec::new();
    for (name, count) in &analysis.candidates {
        if active.contains(&Unit::Name(name.clone())) {
            requests.push((*count, Request::Name(name.clone())));
        }
    }
    let selves: usize = methods
        .iter()
        .map(|&m| analysis.forms.methods[m].selves.len())
        .sum();
    if selves > 0 {
        requests.push((selves, Request::SelfName));
    }
    for uses in &analysis.forms.uses {
        if active.contains(&Unit::Alias(uses.ty)) {
            let count = uses.by_module.values().map(Vec::len).sum();
            requests.push((count, Request::Alias(uses.ty)));
        }
    }
    requests.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

    let mut namer = Namer::default();
    let mut edits = Edits::default();
    for (_, request) in requests {
        let (case, old) = match &request {
            Request::Name(name) => (Case::of(name), name.as_str()),
            Request::SelfName => (Case::Lower, "self"),
            Request::Alias(ty) => (Case::Camel, *ty),
        };
        let fresh = loop {
            let fresh = namer.peek(case);
            if taken.contains(&fresh) || std.has(&fresh) || is_fixed(&fresh, std) {
                namer.advance(case);
            } else {
                break fresh;
            }
        };
        if cg_size(&fresh) >= cg_size(old) {
            continue;
        }
        match request {
            Request::Name(name) => {
                edits.names.insert(name, fresh.clone());
            }
            Request::SelfName => {
                let paying: BTreeSet<usize> = methods
                    .iter()
                    .copied()
                    .filter(|&m| self_pays(&analysis.forms.methods[m], &fresh))
                    .collect();
                if paying.is_empty() {
                    continue;
                }
                edits.methods = paying;
                edits.self_name = Some(fresh.clone());
            }
            Request::Alias(ty) => {
                let uses = analysis.forms.uses.iter().find(|u| u.ty == ty);
                let pays = uses.is_some_and(|uses| {
                    uses.by_module
                        .values()
                        .any(|list| alias_pays(list.len(), ty, uses.generic, &fresh))
                });
                if !pays {
                    continue;
                }
                edits.aliases.insert(ty, fresh.clone());
            }
        }
        namer.advance(case);
        taken.insert(fresh);
    }
    edits
}

/// A text with its changes made, and where each change is in it.
pub struct Applied {
    pub text: String,
    /// The byte range of each change in `text`, with its unit.
    regions: Vec<(usize, usize, Unit)>,
    /// The modules that declare each alias.
    pub declared: BTreeMap<&'static str, usize>,
}

impl Applied {
    /// The units whose changes are in the short error `spans`.
    pub fn blame(&self, spans: &[(usize, usize)]) -> BTreeSet<Unit> {
        let mut blamed = BTreeSet::new();
        for &(start, end) in spans {
            if end.saturating_sub(start) > NAME_SPAN {
                continue;
            }
            for (from, to, unit) in &self.regions {
                let inside = if start == end {
                    *from <= start && start < *to
                } else {
                    start < *to && *from < end
                };
                if inside {
                    blamed.insert(unit.clone());
                }
            }
        }
        blamed
    }
}

/// `source` with the changes of `edits`, and numbers without their digit
/// separators; everything else, comments and whitespace included, stays.
pub fn apply(source: &str, analysis: &Analysis<'_>, edits: &Edits) -> Applied {
    let tokens = analysis.tokens;
    let mut replace: BTreeMap<usize, (String, Option<Unit>)> = BTreeMap::new();
    let mut insert: BTreeMap<usize, Vec<(String, Unit)>> = BTreeMap::new();
    for (at, token) in tokens.iter().enumerate() {
        match token.kind {
            Kind::Ident if !analysis.kept.positions.contains(&at) => {
                if let Some(new) = edits.names.get(&token.text) {
                    replace.insert(at, (new.clone(), Some(Unit::Name(token.text.clone()))));
                }
            }
            Kind::Number if token.text.contains('_') => {
                replace.insert(at, (token.text.replace('_', ""), None));
            }
            _ => {}
        }
    }
    if let Some(name) = &edits.self_name {
        for &m in &edits.methods {
            let method = &analysis.forms.methods[m];
            insert
                .entry(method.at)
                .or_default()
                .push((self_declaration(name, method.mutable), Unit::Method(m)));
            for &at in &method.selves {
                replace.insert(at, (name.clone(), Some(Unit::Method(m))));
            }
        }
    }
    let mut declared = BTreeMap::new();
    for uses in &analysis.forms.uses {
        let Some(alias) = edits.aliases.get(uses.ty) else {
            continue;
        };
        for (&module, list) in &uses.by_module {
            if !alias_pays(list.len(), uses.ty, uses.generic, alias) {
                continue;
            }
            *declared.entry(uses.ty).or_insert(0) += 1;
            insert
                .entry(analysis.forms.modules[module])
                .or_default()
                .push((
                    alias_declaration(alias, uses.ty, uses.generic),
                    Unit::Alias(uses.ty),
                ));
            for &at in list {
                replace.insert(at, (alias.clone(), Some(Unit::Alias(uses.ty))));
            }
        }
    }

    let mut places: Vec<usize> = replace.keys().chain(insert.keys()).copied().collect();
    places.sort_unstable();
    places.dedup();
    let mut out = String::with_capacity(source.len());
    let mut regions = Vec::new();
    let mut copied = 0;
    for at in places {
        let start = tokens.get(at).map_or(source.len(), |t| t.start);
        out.push_str(&source[copied..start]);
        copied = start;
        for (text, unit) in insert.get(&at).into_iter().flatten() {
            let from = out.len();
            out.push_str(text);
            regions.push((from, out.len(), unit.clone()));
        }
        if let Some((text, unit)) = replace.get(&at) {
            let from = out.len();
            out.push_str(text);
            if let Some(unit) = unit {
                regions.push((from, out.len(), unit.clone()));
            }
            copied = tokens[at].end;
        }
    }
    out.push_str(&source[copied..]);
    Applied {
        text: out,
        regions,
        declared,
    }
}

/// The names that the standard library defines, with their kinds.
pub struct StdNames {
    /// Each name and its kinds: some of `i` (an item), `m` (a member), `p`
    /// (in a prelude) and `v` (an enum variant).
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
                        && kind.chars().all(|c| matches!(c, 'i' | 'm' | 'p' | 'v'))
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
pub(crate) const OTHERS: &[&str] = &[
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
pub(crate) const TEXT_MACROS: &[&str] = &["stringify", "concat", "module_path"];

/// Names never renamed, whatever the code around them.
pub(crate) fn is_fixed(name: &str, std: &StdNames) -> bool {
    KEYWORDS.contains(&name)
        || OTHERS.contains(&name)
        || FRAGMENTS.contains(&name)
        || name.starts_with('_')
        || name.starts_with("r#")
        || std.prelude(name)
}

/// The index after the group that opens at `open` (`(`, `[` or `{`), or
/// `None` when no group opens there or it does not close.
pub(crate) fn group_end(tokens: &[Token], open: usize) -> Option<usize> {
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

/// The words in braces in a string literal's text: those that formatting
/// macros may read as names (`{score}`, `{0:width$}`).
pub(crate) fn braced_words(literal: &str) -> Vec<&str> {
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
