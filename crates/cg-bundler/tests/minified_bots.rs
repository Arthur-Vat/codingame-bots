//! Minifies the bundle of every bot of the workspace at each level and
//! checks them (ADR 0045): level 1 reads back as the same tokens as the
//! readable bundle and is smaller; level 2 compiles with rustc 1.90.0 and
//! is no larger than level 1; level 3 compiles, is no larger than level 2
//! and has its tokens with only names changed, each old name to one new
//! name.

use std::collections::BTreeMap;

use cg_bundler::minify::{first_difference, tokens, Kind};
use cg_bundler::{bundle, bundle_package, bundle_report, cg_size, plan, prune, Minify, Options};

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
fn every_bot_minifies_at_each_level() {
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

/// The checks of `every_bot_minifies_at_each_level` for one bot.
fn check_bot(package: &str, rustc: &prune::Rustc) {
    let options = |minify| Options {
        format: true,
        minify,
    };
    let plan = plan::plan(package, None).unwrap();
    let readable = bundle(&plan, options(Minify::Off)).unwrap();
    let level1 = bundle(&plan, options(Minify::Level1)).unwrap();
    let (readable_tokens, level1_tokens) = (
        tokens(&readable).unwrap_or_else(|err| panic!("{package}: {err}")),
        tokens(&level1).unwrap_or_else(|err| panic!("{package}: {err}")),
    );
    assert_eq!(
        first_difference(&readable_tokens, &level1_tokens),
        None,
        "{package}"
    );
    assert!(
        cg_size(&level1) < cg_size(&readable),
        "{package}: {} is not smaller than {}",
        cg_size(&level1),
        cg_size(&readable)
    );

    let level2 = bundle_report(&plan, options(Minify::Level2)).unwrap();
    let report = level2.pruned.expect("level 2 reports what it removed");
    prune::check(&level2.code, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    assert!(
        cg_size(&level2.code) <= cg_size(&level1),
        "{package}: level 2 has {} characters, level 1 {}",
        cg_size(&level2.code),
        cg_size(&level1)
    );
    // Level 2 only removes tokens, and the result is minified.
    let level2_tokens = tokens(&level2.code).unwrap();
    assert!(level2_tokens.len() <= level1_tokens.len(), "{package}");
    if report.items + report.imports == 0 {
        assert_eq!(level2.code, level1, "{package}");
    }
    assert_eq!(
        level2.code.lines().next().unwrap(),
        level1.lines().next().unwrap(),
        "{package}"
    );

    let level3 = bundle_report(&plan, options(Minify::Level3)).unwrap();
    assert_eq!(
        level3.pruned,
        Some(report),
        "{package}: level 3 prunes as level 2 does"
    );
    let renamed = level3.renamed.expect("level 3 reports what it renamed");
    prune::check(&level3.code, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    assert!(
        cg_size(&level3.code) <= cg_size(&level2.code),
        "{package}: level 3 has {} characters, level 2 {}",
        cg_size(&level3.code),
        cg_size(&level2.code)
    );
    // Only names change, each old name always to the same new one, and no
    // two old names to the same.
    let level3_tokens = tokens(&level3.code).unwrap();
    assert_eq!(level3_tokens.len(), level2_tokens.len(), "{package}");
    let (mut new_of, mut old_of) = (BTreeMap::new(), BTreeMap::new());
    for (old, new) in level2_tokens.iter().zip(&level3_tokens) {
        assert_eq!(
            old.kind, new.kind,
            "{package}: `{}` became `{}`",
            old.text, new.text
        );
        if old.kind != Kind::Ident {
            assert_eq!(old.text, new.text, "{package}");
            continue;
        }
        let new_name = new_of.entry(old.text.as_str()).or_insert(new.text.as_str());
        assert_eq!(
            *new_name, new.text,
            "{package}: `{}` has two new names",
            old.text
        );
        let old_name = old_of.entry(new.text.as_str()).or_insert(old.text.as_str());
        assert_eq!(
            *old_name, old.text,
            "{package}: `{}` is the new name of two",
            new.text
        );
    }
    let changed = new_of.iter().filter(|(old, new)| old != new).count();
    assert_eq!(changed, renamed.renamed, "{package}");
    assert_eq!(
        level3.code.lines().next().unwrap(),
        level1.lines().next().unwrap(),
        "{package}"
    );
}

#[test]
fn the_same_bundle_comes_out_every_time() {
    let options = Options::default();
    let first = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    let second = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    assert_eq!(first, second);
}
