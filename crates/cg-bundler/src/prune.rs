//! Removes the items that rustc reports as never used (ADR 0045, level 2).
//!
//! The bundle is compiled with rustc 1.90.0, CodinGame's version, with the
//! lints `dead_code` and `unused_imports` forced on, so that the `allow`
//! the bundler puts on library modules hides nothing. Every item they report
//! is removed, whole: its attributes and doc comments with it, and an
//! inherent `impl` or a `mod` that has nothing left. The text is compiled
//! again, and the new reports are removed in turn, since removing items
//! makes others unused.
//!
//! Some removals do not compile: a struct that a trait impl still names, a
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
//! What may go: `fn`, `const`, `static`, `struct`, `enum`, `union`, `type`
//! and `trait` items, wherever they are (methods and associated items
//! included), empty inherent `impl`s and `mod`s, and the names of `use`
//! declarations. What stays whatever rustc reports: enum variants (removing
//! one renumbers the next, which changes an `as` cast without an error),
//! struct fields, trait impls, `main`, and macros. An item is found from
//! rustc's span of its name, read in the tokens of [`crate::minify`] to its
//! extent; if the extent is not clear, the item stays.
//!
//! The compiler is found by [`Rustc::find`]. There is no fallback to another
//! version, since the output must be reproducible.

mod items;
mod rustc;

pub use rustc::{Compiler, Diagnostic, Outcome, Rustc, ENV, VERSION};

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
            "removed {} items and {} imported names, {} removals that do not compile kept, \
             {} compiles",
            self.items, self.imports, self.refused, self.compiles
        )
    }
}

/// `source` without the items rustc reports as never used, and what was
/// done. It fails if `source` does not compile or rustc cannot be run.
pub fn prune(source: &str, rustc: &Rustc) -> Result<(String, Report), String> {
    let mut pruner = Pruner::new(source, rustc)?;
    // What the errors of earlier rounds kept: not tried again until nothing
    // else is left, since it stays kept unless the items that need it go.
    let mut blocked = BTreeSet::new();
    let mut refused = 0;
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
    let imports = pruner
        .state
        .iter()
        .filter(|id| matches!(id, Id::Leaf(_)))
        .count();
    let report = Report {
        items: pruner.state.len() - imports,
        imports,
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

struct Pruner<'a> {
    source: Source,
    compiler: Compiler<'a>,
    /// The removals made: the text compiles without these.
    state: BTreeSet<Id>,
    /// The text of `state`, which compiled, and rustc's diagnostics of it.
    rendered: Rendered,
    diagnostics: Vec<Diagnostic>,
    /// The compiles made by other threads than the pruner's.
    other_compiles: usize,
}

impl<'a> Pruner<'a> {
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
                    _ => found.extend(self.source.leaves_at(span)),
                }
            }
        }
        found.extend(self.source.empty_containers(&self.rendered));
        found.retain(|id| !self.state.contains(id) && !skip.contains(id));

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
                Outcome::Compiled(diagnostics) => {
                    self.state = all;
                    self.rendered = rendered;
                    self.diagnostics = diagnostics;
                    break;
                }
                Outcome::Failed { spans, .. } => {
                    let blamed = self.blame(&trial, &rendered.text, &spans);
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
    /// compile of `text`.
    fn blame(&self, trial: &[Id], text: &str, spans: &[(usize, usize)]) -> BTreeSet<Id> {
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
            .collect()
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
                            done.push((at, matches!(compiled, Outcome::Compiled(_))));
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
            if let Outcome::Compiled(diagnostics) = self.compiler.compile(&rendered.text)? {
                self.state = trial;
                self.rendered = rendered;
                self.diagnostics = diagnostics;
                return Ok(());
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
