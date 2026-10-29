//! `cg-tokens A.rs B.rs`: tells whether two Rust files have the same
//! tokens, comments and whitespace aside (ADR 0045).
//!
//! It applies to level 1: a level 1 bundle has the tokens of the readable
//! bundle. A level 2 release has fewer, since its unused items are gone, so
//! the release checks do not use this tool for releases; it stays for
//! comparing a file with its level 1 form.
//!
//! Exit status: 0 when the token sequences are the same (it prints
//! `same tokens (N tokens)`), 1 when they differ (it prints where they first
//! differ), 2 on wrong usage or when a file cannot be read or lexed.

use std::process::ExitCode;

use cg_bundler::minify::{first_difference, tokens, Token};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [a, b] = args.as_slice() else {
        eprintln!("usage: cg-tokens A.rs B.rs");
        return ExitCode::from(2);
    };
    let (first, second) = match (read(a), read(b)) {
        (Ok(first), Ok(second)) => (first, second),
        (first, second) => {
            for err in [first.err(), second.err()].into_iter().flatten() {
                eprintln!("error: {err}");
            }
            return ExitCode::from(2);
        }
    };
    match first_difference(&first, &second) {
        None => {
            println!("same tokens ({} tokens)", first.len());
            ExitCode::SUCCESS
        }
        Some(at) => {
            println!("different tokens: the first difference is token {at}");
            println!("  {a}: {}", describe(first.get(at)));
            println!("  {b}: {}", describe(second.get(at)));
            ExitCode::from(1)
        }
    }
}

/// The tokens of the file at `path`.
fn read(path: &str) -> Result<Vec<Token>, String> {
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let text = String::from_utf8(bytes).map_err(|_| format!("{path} is not valid UTF-8"))?;
    tokens(&text).map_err(|err| format!("cannot lex {path}: {err}"))
}

fn describe(token: Option<&Token>) -> String {
    match token {
        Some(token) => format!("line {}, `{}`", token.line, token.text),
        None => "the file ends".to_string(),
    }
}
