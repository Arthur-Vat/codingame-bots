//! Minifies the bundle of every bot of the workspace and checks each stage
//! of the minifier (ADR 0045, ADR 0046): the pruned code compiles with rustc
//! 1.90.0 and has no more tokens than the joined code; the renamed code
//! compiles and has the pruned code's tokens with only names, `self`, the
//! aliased types and the digit separators of numbers changed, each old name
//! to one new name (except at the places kept), plus the declarations of
//! `self`'s name and of the aliases; the minified code reads back as the
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

    // Unused items removed.
    let (pruned, report) = prune::prune(&joined, rustc).unwrap();
    assert_eq!(Some(&report), bundled.pruned.as_ref(), "{package}");
    prune::check(&pruned, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    assert!(
        tokens(&pruned).unwrap().len() <= tokens(&joined).unwrap().len(),
        "{package}"
    );

    // Names, `self`, types and numbers shortened.
    let (renamed, report) = rename::rename(&pruned, rustc).unwrap();
    assert_eq!(Some(&report), bundled.renamed.as_ref(), "{package}");
    prune::check(&renamed, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    check_renaming(package, &pruned, &renamed, &report);

    // Only the whitespace Rust needs.
    let minified = minify::minify(&renamed).unwrap();
    assert_eq!(
        first_difference(&tokens(&renamed).unwrap(), &tokens(&minified).unwrap()),
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
/// separators.
fn check_renaming(package: &str, pruned: &str, renamed: &str, report: &rename::Report) {
    let (old, new) = (tokens(pruned).unwrap(), tokens(renamed).unwrap());
    let kept = rename::kept(&old, StdNames::get());
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
        let (o, n) = (&old[i], &new[j]);
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
