//! Minifies the bundle of every bot of the workspace and checks that it
//! reads back as the same tokens as the readable bundle and is smaller.

use cg_bundler::minify::{first_difference, tokens};
use cg_bundler::{bundle_package, cg_size, Options};

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
fn every_bot_minifies_to_the_same_tokens_in_less_room() {
    let packages = bot_packages();
    assert!(
        packages.iter().any(|p| p == "uttt-bot-random")
            && packages.iter().any(|p| p == "ice-and-fire-bot-plan"),
        "bots found: {packages:?}"
    );
    for package in packages {
        let readable = bundle_package(
            &package,
            None,
            Options {
                format: true,
                minify: false,
            },
        )
        .unwrap();
        let minified = bundle_package(&package, None, Options::default()).unwrap();
        let (readable_tokens, minified_tokens) = (
            tokens(&readable).unwrap_or_else(|err| panic!("{package}: {err}")),
            tokens(&minified).unwrap_or_else(|err| panic!("{package}: {err}")),
        );
        assert_eq!(
            first_difference(&readable_tokens, &minified_tokens),
            None,
            "{package}"
        );
        assert!(
            cg_size(&minified) < cg_size(&readable),
            "{package}: {} is not smaller than {}",
            cg_size(&minified),
            cg_size(&readable)
        );
    }
}
