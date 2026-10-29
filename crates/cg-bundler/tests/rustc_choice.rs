//! The bundler at level 2 needs rustc 1.90.0 and says so when it has none
//! (ADR 0045, decision 2): `CG_RUSTC` names the compiler, and a wrong
//! one fails the bundling, never falls back. The compilers here are
//! scripts, so that nothing needs to be uninstalled.

#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

/// A script named `rustc` that runs `body`, in a folder of its own.
fn fake_rustc(name: &str, body: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cg-bundler-choice-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("rustc");
    std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

/// Runs the bundler on a small bot with `args`; `CG_RUSTC` is `rustc` if
/// given.
fn bundler(args: &[&str], rustc: Option<&PathBuf>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cg-bundler"));
    command.arg("uttt-bot-random").args(args);
    command.env_remove("CG_RUSTC");
    if let Some(rustc) = rustc {
        command.env("CG_RUSTC", rustc);
    }
    command.output().unwrap()
}

#[test]
fn a_wrong_version_fails_the_bundling_at_level_2() {
    let old = fake_rustc("old", "echo 'rustc 1.80.0 (fake 2024-07-01)'");
    for args in [&[][..], &["--minify-level", "2"]] {
        let output = bundler(args, Some(&old));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{args:?}");
        assert!(
            stderr.contains("needs rustc 1.90.0") && stderr.contains("1.80.0"),
            "{stderr}"
        );
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn a_missing_compiler_fails_the_bundling_at_level_2() {
    let output = bundler(&[], Some(&PathBuf::from("/nonexistent/rustc")));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        stderr.contains("CG_RUSTC") && stderr.contains("/nonexistent/rustc"),
        "{stderr}"
    );
}

#[test]
fn levels_that_need_no_compiler_do_not_ask_for_one() {
    let old = fake_rustc("unused", "echo 'rustc 1.80.0 (fake 2024-07-01)'; exit 9");
    for args in [&["--minify-level", "1"][..], &["--keep-comments"]] {
        let output = bundler(args, Some(&old));
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bundle = String::from_utf8(output.stdout).unwrap();
        assert!(bundle.starts_with("// Bundled by cg-bundler from the `uttt-bot-random` package."));
    }
}

#[test]
fn the_level_is_one_or_two() {
    for level in ["0", "3", "x"] {
        let output = bundler(&["--minify-level", level], None);
        assert!(!output.status.success(), "{level}");
    }
}

#[test]
fn a_compiler_that_is_right_is_used() {
    // A compiler that has the right version but rejects everything: the
    // bundler reports that the bundle does not compile.
    let strict = fake_rustc(
        "strict",
        "case \"$1\" in --version) echo 'rustc 1.90.0 (fake 2025-09-01)';; *) echo 'refused' >&2; exit 1;; esac",
    );
    let output = bundler(&[], Some(&strict));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        stderr.contains("does not compile") && stderr.contains("refused"),
        "{stderr}"
    );
}
