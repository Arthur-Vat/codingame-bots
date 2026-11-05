//! Minifies the bundle of every bot of the workspace and checks each stage
//! of the minifier (ADR 0045, ADR 0046): the pruned code compiles with rustc
//! 1.90.0 and has no more tokens than the joined code; the renamed code
//! compiles and has the pruned code's tokens with only names, `self`, the
//! aliased types and the digit separators of numbers changed, each old name
//! to one new name (except at the places kept), plus the declarations of
//! `self`'s name and of the aliases, and the captures of format strings
//! that became arguments; the minified code reads back as the
//! renamed code's tokens, is what the bundler writes, and is smaller than
//! the readable bundle.

use std::collections::BTreeMap;

use cg_bundler::minify::{first_difference, tokens, Kind, Token};
use cg_bundler::rename::{self, StdNames, ALIASED};
use cg_bundler::{bundle, bundle_package, bundle_report, cg_size, joined, minify, plan, prune};
use cg_bundler::{Options, SIZE_LIMIT};

/// The package names of the bots in `games/*/bots/*`.
fn bot_packages() -> Vec<String> {
    let games = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../games");
    let mut packages = Vec::new();
    for game in std::fs::read_dir(games).unwrap() {
        let Ok(bots) = std::fs::read_dir(game.unwrap().path().join("bots")) else {
            continue;
        };
        for bot in bots {
            let Ok(manifest) = std::fs::read_to_string(bot.unwrap().path().join("Cargo.toml"))
            else {
                continue;
            };
            let name = manifest
                .lines()
                .find_map(|line| line.strip_prefix("name = \""))
                .and_then(|rest| rest.strip_suffix('"'))
                .expect("a bot manifest names its package");
            packages.push(name.to_string());
        }
    }
    packages.sort();
    packages
}

#[test]
fn every_bot_minifies_and_each_stage_checks() {
    let packages = bot_packages();
    assert!(
        packages.iter().any(|p| p == "uttt-bot-random")
            && packages.iter().any(|p| p == "ice-and-fire-bot-plan"),
        "bots found: {packages:?}"
    );
    let rustc = prune::Rustc::find().expect("these tests need rustc 1.90.0");
    // Four threads: the compiles dominate.
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| loop {
                let at = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(package) = packages.get(at) else {
                    break;
                };
                check_bot(package, &rustc);
            });
        }
    });
}

/// The checks of `every_bot_minifies_and_each_stage_checks` for one bot.
fn check_bot(package: &str, rustc: &prune::Rustc) {
    let plan = plan::plan(package, None).unwrap();
    let readable = bundle(
        &plan,
        Options {
            format: true,
            minify: false,
        },
    )
    .unwrap();
    let bundled = bundle_report(&plan, Options::default()).unwrap();
    let joined = joined(&plan, Options::default()).unwrap();
    assert!(
        !joined.contains("allow(dead_code"),
        "{package}: the minifier starts without the readable bundle's `allow`"
    );

    // The bundler's own stages, checked here rather than run a second time:
    // the stages are where the compiles go.
    let stages = bundled
        .stages
        .as_ref()
        .expect("a minified bundle has stages");
    let (pruned, renamed) = (stages.pruned.as_str(), stages.renamed.as_str());

    // Unused items removed.
    assert!(bundled.pruned.is_some(), "{package}");
    prune::check(pruned, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    // The pruned code is the joined code with items removed: its tokens are
    // the joined code's, in order, with some left out.
    let (kept, all) = (tokens(pruned).unwrap(), tokens(&joined).unwrap());
    let mut rest = all.iter();
    for token in &kept {
        assert!(
            rest.any(|t| t.text == token.text),
            "{package}: the pruned code's `{}` is not in the joined code, in order",
            token.text
        );
    }

    // Names, `self`, types and numbers shortened.
    let report = bundled
        .renamed
        .as_ref()
        .expect("a minified bundle has a renaming report");
    prune::check(renamed, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    check_renaming(package, pruned, renamed, report);

    // Only the whitespace Rust needs.
    let minified = minify::minify(renamed).unwrap();
    assert_eq!(
        first_difference(&tokens(renamed).unwrap(), &tokens(&minified).unwrap()),
        None,
        "{package}"
    );
    let (header, code) = bundled.code.split_once('\n').unwrap();
    assert_eq!(
        header,
        format!("// Bundled by cg-bundler from the `{package}` package. Minified (ADR 0045).")
    );
    assert_eq!(code, minified, "{package}");
    assert!(
        cg_size(&bundled.code) < cg_size(&readable),
        "{package}: {} is not smaller than {}",
        cg_size(&bundled.code),
        cg_size(&readable)
    );
    assert!(cg_size(&bundled.code) < SIZE_LIMIT, "{package}");
}

/// Checks that `renamed` is `pruned` with only the changes that `report`
/// names: each old name to one new name and no two old names to one, except
/// at the places [`rename::kept`] keeps; `self` to `report.self_name`, after
/// a declaration of that name in each method that uses it; each aliased type
/// to its alias, after the alias's declarations; numbers without their digit
/// separators; in a format string that [`rename::calls`] finds, the captured
/// names that were renamed taken out, and each written as `,new` at its
/// slot, with its one new name.
fn check_renaming(package: &str, pruned: &str, renamed: &str, report: &rename::Report) {
    let (old, new) = (tokens(pruned).unwrap(), tokens(renamed).unwrap());
    let kept = rename::kept(&old, StdNames::get());
    let calls: BTreeMap<usize, rename::Call> = rename::calls(&old)
        .into_iter()
        .map(|call| (call.string, call))
        .collect();
    // The `,new` expected before an old token, with the captured name, and
    // the captured names left in the strings.
    let mut arguments: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let (mut made_arguments, mut left_captured) = (Vec::new(), Vec::new());
    let self_name = report.self_name.as_deref();
    let aliases: BTreeMap<&str, &str> = report
        .aliases
        .iter()
        .map(|(ty, alias, _)| (alias.as_str(), ty.as_str()))
        .collect();
    let text = |tokens: &[Token], at: usize| tokens.get(at).map(|t| t.text.clone());
    let starts_with = |at: usize, expected: &str| {
        let expected = tokens(expected).unwrap();
        (0..expected.len())
            .all(|k| new.get(at + k).is_some_and(|t| t.same(&expected[k])))
            .then_some(expected.len())
    };
    let (mut i, mut j) = (0, 0);
    let (mut declared_selves, mut declared_aliases) = (0, 0);
    let (mut new_of, mut old_of) = (BTreeMap::new(), BTreeMap::new());
    while j < new.len() {
        if let Some(name) = self_name {
            if let Some(length) = [false, true]
                .into_iter()
                .find_map(|mutable| starts_with(j, &rename::self_declaration(name, mutable)))
            {
                declared_selves += 1;
                j += length;
                continue;
            }
        }
        if text(&new, j).as_deref() == Some("type") {
            let alias = text(&new, j + 1).unwrap_or_default();
            if let Some(ty) = aliases.get(alias.as_str()) {
                let generic = ALIASED.iter().any(|&(t, generic)| t == *ty && generic);
                let length = starts_with(j, &rename::alias_declaration(&alias, ty, generic))
                    .unwrap_or_else(|| panic!("{package}: a wrong declaration of {alias}"));
                declared_aliases += 1;
                j += length;
                continue;
            }
        }
        if let Some(names) = arguments.remove(&i) {
            for name in names {
                assert_eq!(
                    text(&new, j).as_deref(),
                    Some(","),
                    "{package}: no `,{name}`"
                );
                let argument = &new[j + 1];
                assert_eq!(argument.kind, Kind::Ident, "{package}: `,{name}`");
                made_arguments.push((name, argument.text.clone()));
                j += 2;
            }
        }
        let (o, n) = (&old[i], &new[j]);
        if let (Some(call), true) = (calls.get(&i), o.text != n.text) {
            // The captures taken out of the string: the subset that gives
            // its new text.
            let count = call.captures.len();
            let out = (0..1u32 << count)
                .find(|set| call.string_text(&o.text, |c| out_of(call, c, *set)) == n.text)
                .unwrap_or_else(|| panic!("{package}: `{}` became `{}`", o.text, n.text));
            for (k, capture) in call.captures.iter().enumerate() {
                if out & (1 << k) != 0 {
                    let slot = call.slots[capture.slot];
                    arguments
                        .entry(slot)
                        .or_default()
                        .push(capture.name.clone());
                } else {
                    left_captured.push(capture.name.clone());
                }
            }
            i += 1;
            j += 1;
            continue;
        }
        if let Some(call) = calls.get(&i) {
            left_captured.extend(call.captures.iter().map(|c| c.name.clone()));
        }
        assert_eq!(
            o.kind, n.kind,
            "{package}: `{}` became `{}`",
            o.text, n.text
        );
        match o.kind {
            Kind::Number => assert_eq!(o.text.replace('_', ""), n.text, "{package}"),
            Kind::Ident if Some(n.text.as_str()) == self_name => {
                assert_eq!(
                    o.text, "self",
                    "{package}: `{}` became `{}`",
                    o.text, n.text
                )
            }
            Kind::Ident if aliases.contains_key(n.text.as_str()) => {
                assert_eq!(o.text, aliases[n.text.as_str()], "{package}")
            }
            Kind::Ident if kept.positions.contains(&i) => {
                assert_eq!(o.text, n.text, "{package}: a kept place changed")
            }
            Kind::Ident => {
                let new_name = new_of.entry(o.text.clone()).or_insert(n.text.clone());
                assert_eq!(
                    *new_name, n.text,
                    "{package}: `{}` has two new names",
                    o.text
                );
                let old_name = old_of.entry(n.text.clone()).or_insert(o.text.clone());
                assert_eq!(
                    *old_name, o.text,
                    "{package}: `{}` is the new name of two",
                    n.text
                );
            }
            _ => assert_eq!(o.text, n.text, "{package}"),
        }
        i += 1;
        j += 1;
    }
    assert_eq!(i, old.len(), "{package}: tokens are missing");
    assert!(arguments.is_empty(), "{package}: arguments are missing");
    for (name, argument) in made_arguments {
        assert_ne!(
            name, argument,
            "{package}: `{name}` made an argument, not renamed"
        );
        assert_eq!(
            new_of.get(&name),
            Some(&argument),
            "{package}: `{name}` became the argument `{argument}`"
        );
    }
    for name in left_captured {
        assert!(
            new_of.get(&name).is_none_or(|new| *new == name),
            "{package}: `{name}` was renamed but a string still captures it"
        );
    }
    assert_eq!(declared_selves, report.methods, "{package}");
    let declarations: usize = report.aliases.iter().map(|(_, _, n)| n).sum();
    assert_eq!(declared_aliases, declarations, "{package}");
    let changed = new_of.iter().filter(|(old, new)| old != new).count();
    assert_eq!(changed, report.renamed, "{package}");
}

#[test]
fn the_same_bundle_comes_out_every_time() {
    let options = Options::default();
    let first = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    let second = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    assert_eq!(first, second);
}

/// Whether `capture`, the `k`-th of `call`, is in the subset `set`.
fn out_of(call: &rename::Call, capture: &rename::Capture, set: u32) -> bool {
    let k = call
        .captures
        .iter()
        .position(|c| std::ptr::eq(c, capture))
        .expect("a capture of the call");
    set & (1 << k) != 0
}
