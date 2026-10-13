//! `uttt-analysis`: reads one JSON request on stdin (to its end), writes the
//! answer as JSON lines on stdout, flushing after each, and exits with
//! status 0. See the library for the protocol.

use std::io::{Read, Write};

fn main() {
    let mut bytes = Vec::new();
    if let Err(error) = std::io::stdin().read_to_end(&mut bytes) {
        eprintln!("uttt-analysis: cannot read the request: {error}");
    }
    // Bytes that are not UTF-8 become U+FFFD, and fail as JSON or as a move.
    let request = String::from_utf8_lossy(&bytes);
    let mut out = std::io::stdout().lock();
    uttt_analysis::analyse(&request, &mut |line| {
        writeln!(out, "{line}").and_then(|()| out.flush()).is_ok()
    });
}
