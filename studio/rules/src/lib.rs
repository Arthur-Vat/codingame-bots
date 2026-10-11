//! The studio's rules as a program for the browser
//! ([ADR 0030](../../../docs/adr/0030-studio-on-github-pages.md)).
//!
//! [`handle`] answers one JSON request with one JSON response, using the
//! same Rust code as the local server (`studio-core`) and the games'
//! adapters. It keeps nothing between requests: a game in progress travels
//! in each request as a `State`. The binary `studio-rules` reads a request
//! on stdin and writes the response on stdout, so that it runs as a WASI
//! command (`wasm32-wasip1`).
//!
//! # Protocol
//!
//! A request is an object with a string field `op`; the response is
//! `{"ok": value}` or `{"error": "message"}`. The operations are in
//! [`ops`] and the shapes they exchange in [`wire`]; `studio/README.md`
//! lists them. A record travels as text (`record_text`) and a bot seed as a
//! decimal string, because JavaScript loses the precision of a 64-bit
//! number.
//!
//! - [`wire`]: the shapes of the protocol.
//! - [`ops`]: one function per operation.

pub mod ops;
pub mod wire;

use serde_json::{json, Value};

/// Answers `request`. Never panics on a bad request: the answer says why it
/// is bad.
pub fn handle(request: &str) -> String {
    let response = match run(request) {
        Ok(value) => json!({ "ok": value }),
        Err(message) => json!({ "error": message }),
    };
    serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"error":"the answer cannot be written"}"#.to_string())
}

fn run(request: &str) -> Result<Value, String> {
    let request: Value =
        serde_json::from_str(request).map_err(|error| format!("invalid request: {error}"))?;
    let Some(op) = request.get("op").and_then(Value::as_str) else {
        return Err("the request must be an object with a string field \"op\"".to_string());
    };
    ops::run(op, &request)
}

#[cfg(test)]
mod tests;
