//! Removes the items that rustc reports as never used (ADR 0045, ADR 0046),
//! the minifier's first stage.
//!
//! The bundle is compiled with rustc 1.90.0, CodinGame's version, with the
//! lints `dead_code`, `unused_imports` and `unreachable_patterns` forced on,
//! so that no `allow` in the code hides anything. Every item they report
//! is removed, whole: its attributes and doc comments with it, and an
//! inherent `impl` or a `mod` that has nothing left. The text is compiled
//! again, and the new reports are removed in turn, since removing items
//! makes others unused.
//!
//! Some removals do not compile: a trait method that an impl still defines, a
//! method that another unused method, itself kept, still calls. When a
//! round fails, the errors point at the names that are missing (in their
//! spans and their notes); the items with those names are left out of the
//! round and the rest is compiled again, until it compiles. What was left
//! out is not tried again while other items remain; once none is left, all
//! of it is tried together, since the removals made may have freed it. If
//! the errors name no candidate, the round is halved until the removals that
//! rustc rejects are found. When that try removes nothing, each of the items
//! is tried alone (on several threads), because a name that two items share
//! can have put the blame on one that could go; those that compile alone are
//! removed and the loop goes on. It ends when nothing more goes: what rustc
//! still reports does not compile without it, alone or with the rest. The
//! result depends on the bundle and on rustc 1.90.0 only: it does not depend
//! on the threads, and everything else runs in the order of the source.
//!
//! An unused named constant goes even when its value is a compile-time
//! assertion (`const CHECK: () = assert!(...)`): rustc has already checked
//! it on the full bundle, and removing it changes nothing at run time.
//!
//! Once nothing that rustc reports goes, the pruner tries what it does not
//! report ([`Pruner::trials`]): each name of a `derive` list (except
//! `Default` on an enum, which its `#[default]` needs), each impl of a trait
//! that the bundle does not define (except `Drop`, which acts unseen), and
//! each `debug_assert!`, `debug_assert_eq!` or `debug_assert_ne!` that is a
//! statement (CodinGame compiles without debug assertions,
//! `docs/CODINGAME.md`). They are tried all at once, rounds as above, with
//! the blame read from the errors' messages: an error blames the removals
//! whose type and trait it names, and never the removal it sits in. A
//! removal that compiles is kept only if rustc does not report more calls
//! that do nothing (lint `noop_method_call`): without `Clone`, `r.clone()`
//! on a reference `r` still compiles and copies the reference (`Clone` is
//! not offered at all when the bundle calls `to_owned`, which the lint does
//! not cover). Such a fallback to another impl is the one way known for a
//! removal that compiles to change what the code does; the answers check of
//! each release (ADR 0045, decision 4) would show any other. When code goes
//! after them, they are tried again, since that code may have needed a
//! trait.
//!
//! What may go: `fn`, `const`, `static`, `struct`, `enum`, `union`, `type`
//! and `trait` items, wherever they are (methods and associated items
//! included), empty inherent `impl`s and `mod`s, the names of `use`
//! declarations, enum variants that are never constructed, and `match` arms
//! that rustc reports unreachable (lint `unreachable_patterns`). A type or a
//! trait goes with its `impl`s, trait impls included, and with the names of
//! `use` declarations that import it; a variant goes with the alternatives
//! of the patterns that name it, and an enum without fields loses only
//! variants that no kept one follows, so that no number changes (see
//! `items/shapes.rs` for when each is offered). Every removal only deletes
//! text. What stays
//! whatever rustc reports: struct fields, an impl of the bundle's own trait
//! whose type and trait both stay, `Drop` impls, `main`, and macros. An
//! item is found from rustc's span of its name, read in the tokens of
//! [`crate::minify`] to its extent; if the extent is not clear, the item
//! stays.
//!
//! The compiler is found by [`Rustc::find`]. There is no fallback to another
//! version, since the output must be reproducible.

mod items;
mod rustc;

pub use rustc::{Compiler, Diagnostic, ErrorNames, Outcome, Rustc, ENV, VERSION};

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::minify::{tokens, Kind};
use items::{Id, Rendered, Source};

/// The most threads that compile at once.
const MAX_WORKERS: usize = 8;

/// Error spans longer than this say nothing about one missing name.
const NAME_SPAN: usize = 64;

/// What a pruning did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Items removed, empty `impl`s and `mod`s included.
    pub items: usize,
    /// Names removed from `use` declarations.
    pub imports: usize,
    /// Names removed from `derive` lists.
    pub derives: usize,
    /// Impls of traits that the bundle does not define.
    pub trait_impls: usize,
    /// Debug assertions.
    pub asserts: usize,
    /// Removals that rustc reports but that do not compile, in the last
    /// round.
    pub refused: usize,
    /// Runs of rustc, the first on the bundle itself.
    pub compiles: usize,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "removed {} items, {} imported names, {} derived traits, {} trait impls and {} \
             debug assertions, {} removals that do not compile kept, {} compiles",
            self.items,
            self.imports,
            self.derives,
            self.trait_impls,
            self.asserts,
            self.refused,
            self.compiles
        )
    }
}

/// `source` without the items rustc reports as never used, and what was
/// done. It fails if `source` does not compile or rustc cannot be run.
pub fn prune(source: &str, rustc: &Rustc) -> Result<(String, Report), String> {
    let mut pruner = Pruner::new(source, rustc)?;
    let mut refused = pruner.reported()?;
    // Removing code (not imports or empty containers) may free a trait.
    let code = |pruner: &Pruner| {
        let code = pruner.state.iter();
        code.filter(|id| matches!(id, Id::Item(_) | Id::Variant(_) | Id::Arm(_)))
            .count()
    };
    while pruner.trials()? {
        let before = code(&pruner);
        refused = pruner.reported()?;
        if code(&pruner) == before {
            break;
        }
    }
    let count = |kind: fn(&Id) -> bool| pruner.state.iter().filter(|id| kind(id)).count();
    let imports = count(|id| matches!(id, Id::Leaf(_)));
    let derives = count(|id| matches!(id, Id::Derive(_)));
    let trait_impls = count(|id| matches!(id, Id::TraitImpl(_)));
    let asserts = count(|id| matches!(id, Id::Assert(_)));
    let report = Report {
        items: pruner.state.len() - imports - derives - trait_impls - asserts,
        imports,
        derives,
        trait_impls,
        asserts,
        refused,
        compiles: pruner.compiler.compiles() + pruner.other_compiles,
    };
    Ok((pruner.rendered.text, report))
}

/// Compiles `text` with `rustc`, to check what the pruner produced after it
/// has been minified. It fails with rustc's errors.
pub fn check(text: &str, rustc: &Rustc) -> Result<(), String> {
    match Compiler::new(rustc)?.compile(text)? {
        Outcome::Compiled(_) => Ok(()),
        Outcome::Failed { errors, .. } => Err(format!(
            "the text does not compile with rustc {VERSION}:\n{errors}"
        )),
    }
}

/// How many of `diagnostics` say that a call does nothing (lint
/// `noop_method_call`): a `.clone()` on a reference to a type that no
/// longer derives `Clone` compiles, and copies the reference instead.
fn noops(diagnostics: &[Diagnostic]) -> usize {
    diagnostics.iter().filter(|d| d.lint == NOOP).count()
}

/// The traits that code uses without naming them or their methods, so that
/// an error that their impl is missing may not name them.
const UNNAMED: [&str; 4] = ["Deref", "DerefMut", "Index", "IndexMut"];

/// The lint of calls that do nothing.
const NOOP: &str = "noop_method_call";

struct Pruner<'a> {
    source: Source,
    compiler: Compiler<'a>,
    /// The removals made: the text compiles without these.
    state: BTreeSet<Id>,
    /// The text of `state`, which compiled, and rustc's diagnostics of it.
    rendered: Rendered,
    diagnostics: Vec<Diagnostic>,
    /// The calls that do nothing in the bundle itself: a removal that
    /// makes more is refused.
    noops: usize,
    /// The compiles made by other threads than the pruner's.
    other_compiles: usize,
}

impl<'a> Pruner<'a> {
    /// Removes what rustc reports, until nothing more goes, and gives the
    /// number of reported removals that do not compile.
    fn reported(&mut self) -> Result<usize, String> {
        // What the errors of earlier rounds kept: not tried again until nothing
        // else is left, since it stays kept unless the items that need it go.
        let mut blocked = BTreeSet::new();
        let mut refused = 0;
        let pruner = self;
        loop {
            let mut candidates = pruner.candidates(&blocked);
            let retry = candidates.is_empty();
            if retry {
                candidates = pruner.candidates(&BTreeSet::new());
                if candidates.is_empty() {
                    break;
                }
            }
            let before = pruner.state.len();
            let left_out = pruner.round(&candidates)?;
            if retry {
                blocked.clear();
            }
            blocked.extend(left_out);
            if pruner.state.len() == before && retry {
                // Everything was tried together and nothing went. A name that
                // two items share may have put the blame on one that can go
                // alone: try each.
                let alone = pruner.alone(&candidates)?;
                pruner.settle(&alone, false)?;
                if pruner.state.len() == before {
                    refused = candidates.len();
                    break;
                }
                blocked.clear();
            }
        }
        Ok(refused)
    }

    /// Removes what rustc does not report but compiles without
    /// ([`Source::trials`]), and says whether anything went. They are
    /// tried all at once, as in [`Pruner::reported`]: the removals that the
    /// errors blame ([`Pruner::blame`]) are left out of the round, and what
    /// was left out is tried again once nothing else goes. They are not
    /// tried alone.
    fn trials(&mut self) -> Result<bool, String> {
        let start = self.state.len();
        let open: Vec<Id> = self
            .source
            .trials()
            .into_iter()
            .filter(|&id| {
                let (start, end) = self.source.bytes_of(id);
                self.rendered.deleted(start, end).is_none()
            })
            .collect();
        let mut blocked = BTreeSet::new();
        loop {
            let left = |skip: &BTreeSet<Id>| -> Vec<Id> {
                let gone = |id: &Id| self.state.contains(id) || skip.contains(id);
                open.iter().copied().filter(|id| !gone(id)).collect()
            };
            let mut candidates = left(&blocked);
            let retry = candidates.is_empty();
            if retry {
                candidates = left(&BTreeSet::new());
                if candidates.is_empty() {
                    break;
                }
            }
            let before = self.state.len();
            let left_out = self.round(&candidates)?;
            if retry {
                blocked.clear();
                if self.state.len() == before {
                    break;
                }
            }
            blocked.extend(left_out);
        }
        Ok(self.state.len() > start)
    }

    fn new(text: &str, rustc: &'a Rustc) -> Result<Self, String> {
        let source = Source::new(text).map_err(|err| format!("cannot read the bundle: {err}"))?;
        let mut compiler = Compiler::new(rustc)?;
        let state = BTreeSet::new();
        let rendered = source.render(&state);
        let diagnostics = match compiler.compile(&rendered.text)? {
            Outcome::Compiled(diagnostics) => diagnostics,
            Outcome::Failed { errors, .. } => {
                return Err(format!(
                    "the bundle does not compile with rustc {VERSION}:\n{errors}"
                ))
            }
        };
        Ok(Pruner {
            source,
            compiler,
            state,
            rendered,
            noops: noops(&diagnostics),
            diagnostics,
            other_compiles: 0,
        })
    }

    /// The removals to try now, in the order of the source: what rustc
    /// reports about the current text and the containers it leaves empty,
    /// except `skip` and what lies inside another candidate (it comes back
    /// if the outer one does not go).
    fn candidates(&self, skip: &BTreeSet<Id>) -> Vec<Id> {
        let mut found = BTreeSet::new();
        for diagnostic in &self.diagnostics {
            for &span in &diagnostic.spans {
                let Some(span) = self.rendered.to_original(span) else {
                    continue;
                };
                match diagnostic.lint.as_str() {
                    "dead_code" => found.extend(self.source.item_at(span)),
                    "unreachable_patterns" => found.extend(self.source.arm_at(span)),
                    "unused_imports" => found.extend(self.source.leaves_at(span)),
                    _ => {}
                }
            }
        }
        found.extend(self.source.empty_containers(&self.rendered));
        found.retain(|id| !self.state.contains(id) && !skip.contains(id));
        // A variant of an enum without fields goes only after the ones that
        // follow it.
        let going = found.clone();
        found.retain(|&id| {
            self.source.trailing(id, |name| {
                going.contains(&Id::Variant(name)) || self.state.contains(&Id::Variant(name))
            })
        });

        let mut by_place: Vec<((usize, usize), Id)> = found
            .into_iter()
            .map(|id| (self.source.bytes_of(id), id))
            .collect();
        by_place.sort_by_key(|&((start, end), id)| (start, std::cmp::Reverse(end), id));
        let mut kept = Vec::new();
        let mut until = 0;
        for ((start, end), id) in by_place {
            if start >= until {
                kept.push(id);
                until = end;
            }
        }
        kept
    }

    /// Removes the `candidates` that compile together. When they do not,
    /// the candidates that the errors name are left out and the others are
    /// tried again.
    fn round(&mut self, candidates: &[Id]) -> Result<Vec<Id>, String> {
        let mut trial = candidates.to_vec();
        let mut left_out = Vec::new();
        while !trial.is_empty() {
            let mut all = self.state.clone();
            all.extend(trial.iter().copied());
            let rendered = self.source.render(&all);
            match self.compiler.compile(&rendered.text)? {
                Outcome::Compiled(diagnostics) if noops(&diagnostics) <= self.noops => {
                    self.source.drop_kept_variants(&mut all);
                    self.state = all;
                    self.rendered = rendered;
                    self.diagnostics = diagnostics;
                    break;
                }
                outcome => {
                    let (spans, names) = match outcome {
                        Outcome::Failed { spans, names, .. } => (spans, names),
                        Outcome::Compiled(diagnostics) => {
                            let noops = diagnostics.into_iter().filter(|d| d.lint == NOOP);
                            let names = noops.map(|d| ErrorNames {
                                quoted: d.names,
                                spanned: BTreeSet::new(),
                                at: d.spans,
                            });
                            (Vec::new(), names.collect())
                        }
                    };
                    let blamed = self.blame(&trial, &rendered, &spans, &names);
                    if blamed.is_empty() {
                        self.settle(&trial, true)?;
                        left_out.extend(trial.iter().filter(|id| !self.state.contains(id)));
                        break;
                    }
                    trial.retain(|id| !blamed.contains(id));
                    left_out.extend(blamed);
                }
            }
        }
        Ok(left_out)
    }

    /// The members of `trial` whose names are in the error spans of the
    /// compile of `rendered`, and the removals of [`Source::trials`] that
    /// one of the errors (their `names`) blames, among those it is not in
    /// (an error in an impl is about what the impl needs): those whose type
    /// and trait it quotes; else those whose type and trait are in its
    /// quotes or its spans (`proof: Proof` in a field that is not `Copy`);
    /// else the impls of [`UNNAMED`] traits of the types it quotes (an
    /// impl of `Deref` that a coercion needs); else the derived `Clone` of
    /// the types it quotes (a `.clone()` that copies a reference gives a
    /// mismatched type).
    fn blame(
        &self,
        trial: &[Id],
        rendered: &Rendered,
        spans: &[(usize, usize)],
        names: &[ErrorNames],
    ) -> BTreeSet<Id> {
        let text = &rendered.text;
        let mut named = BTreeSet::new();
        for &(start, end) in spans {
            if end - start > NAME_SPAN {
                continue;
            }
            let Some(snippet) = text.get(start..end) else {
                continue;
            };
            for token in tokens(snippet).unwrap_or_default() {
                if token.kind == Kind::Ident {
                    named.insert(token.text);
                }
            }
        }
        trial
            .iter()
            .copied()
            .filter(|&id| {
                self.source
                    .name_of(id)
                    .is_some_and(|name| named.contains(name))
            })
            .chain(
                names
                    .iter()
                    .flat_map(|n| self.blame_trials(trial, rendered, n)),
            )
            .collect()
    }

    /// The removals of [`Source::trials`] in `trial` that one error that
    /// gives `names` blames (see [`Pruner::blame`]).
    fn blame_trials(&self, trial: &[Id], rendered: &Rendered, names: &ErrorNames) -> Vec<Id> {
        let places: Vec<(usize, usize)> = names
            .at
            .iter()
            .filter_map(|&span| rendered.to_original(span))
            .collect();
        let outside = |id: Id| {
            let (start, end) = self.source.bytes_of(id);
            !places.iter().any(|&(s, e)| start <= s && e <= end)
        };
        let subjects: Vec<(Id, (&str, &str))> = trial
            .iter()
            .filter(|&&id| outside(id))
            .filter_map(|&id| Some((id, self.source.subject(id)?)))
            .collect();
        let quoted = |name: &str| names.quoted.contains(name);
        let given = |name: &str| quoted(name) || names.spanned.contains(name);
        let applies = |rule: usize, id: Id, ty: &str, tr: &str| match rule {
            0 => quoted(ty) && quoted(tr),
            1 => given(ty) && given(tr),
            2 => matches!(id, Id::TraitImpl(_)) && UNNAMED.contains(&tr) && quoted(ty),
            _ => matches!(id, Id::Derive(_)) && tr == "Clone" && quoted(ty),
        };
        (0..4)
            .map(|rule| {
                let blamed = subjects
                    .iter()
                    .filter(|&&(id, (ty, tr))| applies(rule, id, ty, tr));
                blamed.map(|&(id, _)| id).collect::<Vec<Id>>()
            })
            .find(|blamed| !blamed.is_empty())
            .unwrap_or_default()
    }

    /// The `candidates` whose removal alone compiles, found by compiling
    /// them at once on several threads. The result does not depend on how
    /// many.
    fn alone(&mut self, candidates: &[Id]) -> Result<Vec<Id>, String> {
        let workers = std::thread::available_parallelism()
            .map_or(1, usize::from)
            .min(MAX_WORKERS)
            .min(candidates.len());
        let (source, state, rustc) = (&self.source, &self.state, self.compiler.rustc());
        let baseline = self.noops;
        let next = AtomicUsize::new(0);
        let results = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..workers)
                .map(|_| {
                    scope.spawn(|| -> Result<(Vec<(usize, bool)>, usize), String> {
                        let mut compiler = Compiler::new(rustc)?;
                        let mut done = Vec::new();
                        loop {
                            let at = next.fetch_add(1, Ordering::Relaxed);
                            let Some(&id) = candidates.get(at) else {
                                break;
                            };
                            let mut trial = state.clone();
                            trial.insert(id);
                            let compiled = compiler.compile(&source.render(&trial).text)?;
                            let clean = matches!(
                                compiled,
                                Outcome::Compiled(diagnostics) if noops(&diagnostics) <= baseline
                            );
                            done.push((at, clean));
                        }
                        Ok((done, compiler.compiles()))
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| {
                    handle
                        .join()
                        .map_err(|_| "a compile thread panicked".to_string())?
                })
                .collect::<Result<Vec<_>, String>>()
        })?;
        let mut compiles = 0;
        let mut found = Vec::new();
        for (done, count) in results {
            compiles += count;
            found.extend(done.into_iter().filter(|&(_, ok)| ok).map(|(at, _)| at));
        }
        self.other_compiles += compiles;
        found.sort_unstable();
        Ok(found.into_iter().map(|at| candidates[at]).collect())
    }

    /// Adds as many of `candidates` to the removals as compile, halving
    /// them until the ones that do not are found. `failing` says that
    /// adding all of them is known not to compile.
    fn settle(&mut self, candidates: &[Id], failing: bool) -> Result<(), String> {
        if candidates.is_empty() {
            return Ok(());
        }
        if !failing {
            let mut trial = self.state.clone();
            trial.extend(candidates.iter().copied());
            let rendered = self.source.render(&trial);
            match self.compiler.compile(&rendered.text)? {
                Outcome::Compiled(diagnostics) if noops(&diagnostics) <= self.noops => {
                    self.source.drop_kept_variants(&mut trial);
                    self.state = trial;
                    self.rendered = rendered;
                    self.diagnostics = diagnostics;
                    return Ok(());
                }
                _ => {}
            }
        }
        if candidates.len() == 1 {
            return Ok(());
        }
        let (left, right) = candidates.split_at(candidates.len() / 2);
        let before = self.state.len();
        self.settle(left, false)?;
        // If all of the left half went, the right half is what failed.
        let left_went = self.state.len() == before + left.len();
        self.settle(right, left_went)
    }
}

#[cfg(test)]
mod tests;
