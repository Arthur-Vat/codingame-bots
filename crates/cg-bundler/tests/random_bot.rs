//! Bundles the real random bot, compiles the bundle on its own with rustc,
//! the way CodinGame does, and plays a turn with it.

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn the_bundled_random_bot_compiles_alone_and_plays() {
    let bundle = cg_bundler::bundle_package("uttt-bot-random", None, true).unwrap();
    assert!(bundle.starts_with("// Bundled by cg-bundler from the `uttt-bot-random` package."));
    assert!(bundle.contains("mod cg_core {"));
    assert!(
        bundle.contains("use crate::cg_core::rng::"),
        "paths were not rewritten"
    );
    assert!(
        !bundle.contains("mod tests"),
        "test modules must be dropped"
    );
    assert!(bundle.len() < cg_bundler::SIZE_LIMIT);

    let dir = std::env::temp_dir().join(format!("cg-bundler-random-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("random.rs");
    let binary = dir.join("random");
    std::fs::write(&source, &bundle).unwrap();
    let compiled = Command::new("rustc")
        .args(["--edition", "2021", "-O", "--crate-type", "bin", "-o"])
        .arg(&binary)
        .arg(&source)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "the bundle does not compile:\n{}",
        String::from_utf8_lossy(&compiled.stderr)
    );

    let mut bot = Command::new(&binary)
        .env("CG_SEED", "3")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    bot.stdin
        .take()
        .unwrap()
        .write_all(b"-1 -1\n3\n4 4\n0 0\n8 8\n")
        .unwrap();
    let output = bot.wait_with_output().unwrap();
    let answer = String::from_utf8(output.stdout).unwrap();
    assert!(
        ["4 4\n", "0 0\n", "8 8\n"].contains(&answer.as_str()),
        "{answer:?}"
    );
}
