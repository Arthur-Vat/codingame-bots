//! Minifies the bundle of every bot of the workspace at both levels and
//! checks them (ADR 0045): level 1 reads back as the same tokens as the
//! readable bundle and is smaller; level 2 compiles with rustc 1.90.0 and
//! is no larger than level 1.

use cg_bundler::minify::{first_difference, tokens};
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
fn every_bot_minifies_at_both_levels() {
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

/// The checks of `every_bot_minifies_at_both_levels` for one bot.
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
}

#[test]
fn the_same_bundle_comes_out_every_time() {
    let options = Options::default();
    let first = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    let second = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    assert_eq!(first, second);
}
