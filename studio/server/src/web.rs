//! The front end's files.
//!
//! Any path that is not a file under the web directory gives its
//! `index.html`, so the single-page app can handle its own routes.

use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::api::Response;

const MISSING_PAGE: &str = "<!doctype html><html><head><meta charset=\"utf-8\"><title>Studio</title></head><body><h1>The studio's front end is not built</h1><p>Run <code>npm install</code> and <code>npm run build</code> in <code>studio/web</code>, then reload this page. The API under <code>/api</code> works without it.</p></body></html>";

/// The file for a request path, or the front end's `index.html`.
pub fn serve(web: &Path, path: &str) -> Response {
    if let Some(file) = file_for(web, path) {
        if let Ok(body) = fs::read(&file) {
            return Response::new(200, content_type(&file), body);
        }
    }
    match fs::read(web.join("index.html")) {
        Ok(body) => Response::new(200, "text/html; charset=utf-8", body),
        Err(_) => Response::new(
            200,
            "text/html; charset=utf-8",
            MISSING_PAGE.as_bytes().to_vec(),
        ),
    }
}

/// The file under `web` that `path` names, if it exists. Paths that climb
/// out of `web` name nothing.
fn file_for(web: &Path, path: &str) -> Option<PathBuf> {
    let relative = path.trim_start_matches('/');
    if relative.is_empty() || relative.contains('\\') || relative.contains('%') {
        return None;
    }
    let mut file = web.to_path_buf();
    for component in Path::new(relative).components() {
        match component {
            Component::Normal(part) => file.push(part),
            _ => return None,
        }
    }
    file.is_file().then_some(file)
}

fn content_type(file: &Path) -> &'static str {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json" | "map") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
