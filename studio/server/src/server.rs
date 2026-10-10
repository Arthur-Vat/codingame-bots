//! The sockets: `tiny_http` requests adapted to [`handle`].

use std::io::Read;
use std::sync::Arc;
use std::thread;

use tiny_http::{Header, Request, Server};

use crate::api::{handle, Response};
use crate::state::State;

/// The only address the server listens on: this computer, never the network.
pub const BIND_ADDRESS: &str = "127.0.0.1";

/// Worker threads answering requests.
const WORKERS: usize = 4;

/// The largest request body read.
const MAX_BODY: u64 = 1 << 20;

/// The socket address for `port`.
pub fn bind_address(port: u16) -> String {
    format!("{BIND_ADDRESS}:{port}")
}

/// The address of the studio in a browser.
pub fn url(port: u16) -> String {
    format!("http://{BIND_ADDRESS}:{port}/")
}

/// Whether a request may be served: its `Host` is this server's, and a
/// browser's `Origin`, if there is one, is the page this server gave. That
/// keeps other web pages (or a rebound DNS name) from driving the studio.
pub fn request_allowed(port: u16, host: Option<&str>, origin: Option<&str>) -> bool {
    let hosts = [
        format!("{BIND_ADDRESS}:{port}"),
        format!("localhost:{port}"),
    ];
    let host_ok = host.is_some_and(|host| hosts.iter().any(|allowed| allowed == host));
    let origin_ok = origin.is_none_or(|origin| {
        hosts
            .iter()
            .any(|allowed| origin == format!("http://{allowed}"))
    });
    host_ok && origin_ok
}

/// Serves requests until the process ends.
pub fn serve(state: &State, port: u16) -> Result<(), String> {
    let address = bind_address(port);
    let server =
        Server::http(&address).map_err(|error| format!("cannot listen on {address}: {error}"))?;
    println!("studio: {}", url(port));
    let server = Arc::new(server);
    let workers: Vec<_> = (0..WORKERS)
        .map(|_| {
            let (server, state) = (server.clone(), state.clone());
            thread::spawn(move || {
                while let Ok(request) = server.recv() {
                    answer(&state, port, request);
                }
            })
        })
        .collect();
    for worker in workers {
        let _ = worker.join();
    }
    Ok(())
}

fn header(request: &Request, name: &str) -> Option<String> {
    request
        .headers()
        .iter()
        .find(|header| header.field.as_str().as_str().eq_ignore_ascii_case(name))
        .map(|header| header.value.as_str().to_string())
}

fn answer(state: &State, port: u16, mut request: Request) {
    let host = header(&request, "Host");
    let origin = header(&request, "Origin");
    let response = if request_allowed(port, host.as_deref(), origin.as_deref()) {
        let mut body = Vec::new();
        let _ = request
            .as_reader()
            .take(MAX_BODY + 1)
            .read_to_end(&mut body);
        if body.len() as u64 > MAX_BODY {
            Response::error(413, "the request body is larger than 1 MiB")
        } else {
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            // `handle` splits the query string off itself.
            let path = url.split('#').next().unwrap_or("");
            handle(state, &method, path, &body)
        }
    } else {
        Response::error(403, "forbidden")
    };
    let mut reply = tiny_http::Response::from_data(response.body).with_status_code(response.status);
    for (name, value) in [
        ("Content-Type", response.content_type.as_str()),
        ("Cache-Control", "no-store"),
    ] {
        if let Ok(header) = Header::from_bytes(name, value) {
            reply = reply.with_header(header);
        }
    }
    let _ = request.respond(reply);
}
