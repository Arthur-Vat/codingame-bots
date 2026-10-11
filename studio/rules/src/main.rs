//! `studio-rules`: reads one JSON request on stdin, writes the response and
//! a newline on stdout. See the library for the protocol.

use std::io::{Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut request = String::new();
    if let Err(error) = std::io::stdin().read_to_string(&mut request) {
        eprintln!("studio-rules: cannot read the request: {error}");
        return ExitCode::FAILURE;
    }
    let response = studio_rules::handle(&request);
    let mut out = std::io::stdout().lock();
    if let Err(error) = writeln!(out, "{response}").and_then(|()| out.flush()) {
        eprintln!("studio-rules: cannot write the response: {error}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
