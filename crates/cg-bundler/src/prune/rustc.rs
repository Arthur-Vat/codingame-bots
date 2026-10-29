//! Finds CodinGame's rustc and compiles a bundle with it, for the pruner.
//!
//! The output of the pruner must not depend on the session's compiler, so
//! only rustc 1.90.0 is accepted ([`VERSION`]); there is no fallback.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::Value;

/// CodinGame's version of Rust (ADR 0010).
pub const VERSION: &str = "1.90.0";

/// The environment variable that names the `rustc` to use instead of
/// looking for one.
pub const ENV: &str = "CG_RUSTC";

/// A rustc 1.90.0 that can be run.
#[derive(Clone, Debug)]
pub struct Rustc {
    program: OsString,
    /// Arguments before the compiler's own (`run 1.90.0 rustc` for rustup).
    prefix: Vec<OsString>,
}

/// What one run of rustc reported about a bundle.
#[derive(Debug)]
pub enum Outcome {
    /// It compiled; these are the diagnostics of the two lints.
    Compiled(Vec<Diagnostic>),
    /// It did not.
    Failed {
        /// The errors, as rustc writes them.
        errors: String,
        /// The byte ranges of the spans of the errors and their notes.
        spans: Vec<(usize, usize)>,
    },
}

/// A diagnostic of the lints `dead_code` or `unused_imports`: its lint and
/// the byte ranges of its primary spans in the file, without those that
/// come from a macro.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub lint: String,
    pub spans: Vec<(usize, usize)>,
}

impl Rustc {
    /// The compiler to use: the one `CG_RUSTC` names if it is set, else
    /// `rustup run 1.90.0 rustc` if rustup exists, else `rustc`. It fails
    /// when that is not rustc 1.90.0.
    pub fn find() -> Result<Rustc, String> {
        Rustc::find_with(std::env::var_os(ENV))
    }

    /// As [`Rustc::find`], with the value of `CG_RUSTC` given, so that
    /// tests need not set a variable of the process.
    pub fn find_with(cg_rustc: Option<OsString>) -> Result<Rustc, String> {
        if let Some(path) = cg_rustc.filter(|path| !path.is_empty()) {
            let rustc = Rustc {
                program: path.clone(),
                prefix: Vec::new(),
            };
            let version = rustc.version().map_err(|err| {
                format!(
                    "cannot run the compiler named by {ENV} ({}): {err}",
                    path.to_string_lossy()
                )
            })?;
            return rustc.accept(version, &format!("{ENV}={}", path.to_string_lossy()));
        }

        let rustup = Rustc {
            program: "rustup".into(),
            prefix: vec!["run".into(), VERSION.into(), "rustc".into()],
        };
        match rustup.version() {
            Ok(version) => return rustup.accept(version, &format!("rustup run {VERSION} rustc")),
            Err(Failure::Missing) => {}
            Err(Failure::Failed(why)) => {
                return Err(format!(
                    "rustup cannot run rustc {VERSION}, which the minifier needs (CodinGame's \
                     version, ADR 0045); install it with `rustup toolchain install {VERSION}`: {why}"
                ))
            }
        }

        let plain = Rustc {
            program: "rustc".into(),
            prefix: Vec::new(),
        };
        match plain.version() {
            Ok(version) => plain.accept(version, "rustc"),
            Err(_) => Err(format!(
                "the minifier needs rustc {VERSION} (CodinGame's version, ADR 0045), but there \
                 is neither rustup nor rustc: install rustup and run `rustup toolchain install \
                 {VERSION}`, or set {ENV} to a rustc {VERSION}"
            )),
        }
    }

    /// `Ok` when `version` (the first line of `rustc --version`) is
    /// [`VERSION`].
    fn accept(self, version: String, from: &str) -> Result<Rustc, String> {
        let wanted = format!("rustc {VERSION}");
        let right = version == wanted || version.starts_with(&format!("{wanted} "));
        if right {
            Ok(self)
        } else {
            Err(format!(
                "the minifier needs rustc {VERSION} (CodinGame's version, ADR 0045), but \
                 {from} is `{version}`: install it with `rustup toolchain install {VERSION}`, \
                 or set {ENV} to a rustc {VERSION}"
            ))
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.prefix);
        command
    }

    /// The first line of `--version`.
    fn version(&self) -> Result<String, Failure> {
        let output = self
            .command()
            .arg("--version")
            .stdin(Stdio::null())
            .output()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Failure::Missing
                } else {
                    Failure::Failed(err.to_string())
                }
            })?;
        if !output.status.success() {
            return Err(Failure::Failed(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        let text = String::from_utf8_lossy(&output.stdout);
        Ok(text.lines().next().unwrap_or("").trim().to_string())
    }
}

/// Why `--version` gave nothing.
enum Failure {
    /// There is no such program.
    Missing,
    Failed(String),
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::Missing => write!(f, "no such program"),
            Failure::Failed(why) => write!(f, "{why}"),
        }
    }
}

/// A folder for the files of compiles, removed when dropped.
pub struct Compiler<'a> {
    rustc: &'a Rustc,
    dir: PathBuf,
    /// The compiles made so far.
    compiles: usize,
}

static FOLDERS: AtomicUsize = AtomicUsize::new(0);

impl<'a> Compiler<'a> {
    pub fn new(rustc: &'a Rustc) -> Result<Self, String> {
        let dir = std::env::temp_dir().join(format!(
            "cg-bundler-{}-{}",
            std::process::id(),
            FOLDERS.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir)
            .map_err(|err| format!("cannot make the folder {}: {err}", dir.display()))?;
        Ok(Self {
            rustc,
            dir,
            compiles: 0,
        })
    }

    /// The compiler this one runs.
    pub fn rustc(&self) -> &'a Rustc {
        self.rustc
    }

    /// How many times [`Compiler::compile`] ran rustc.
    pub fn compiles(&self) -> usize {
        self.compiles
    }

    /// Type-checks `source` as a program with `main`, with the options
    /// CodinGame compiles with (`scripts/cg-check.sh`), and the two lints
    /// forced on, so that the `allow` the bundler puts on library modules
    /// does not hide anything. It fails only when rustc cannot be run.
    pub fn compile(&mut self, source: &str) -> Result<Outcome, String> {
        self.compiles += 1;
        let file = self.dir.join("bundle.rs");
        std::fs::write(&file, source)
            .map_err(|err| format!("cannot write {}: {err}", file.display()))?;
        let output = self
            .rustc
            .command()
            .args(["--edition", "2021", "--crate-type", "bin"])
            .args(["--crate-name", "bundle", "-C", "opt-level=3"])
            .args(["--emit=metadata", "--error-format=json", "-A", "warnings"])
            .args([
                "--force-warn",
                "dead_code",
                "--force-warn",
                "unused_imports",
            ])
            .arg("--out-dir")
            .arg(&self.dir)
            .arg(&file)
            .stdin(Stdio::null())
            .output()
            .map_err(|err| format!("cannot run rustc: {err}"))?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let name = file.to_string_lossy();
        let mut diagnostics = Vec::new();
        let mut errors = String::new();
        let mut error_spans = Vec::new();
        for line in stderr.lines().filter(|line| !line.trim().is_empty()) {
            let Ok(value) = serde_json::from_str::<Value>(line) else {
                // Not a diagnostic: keep it, it may say why rustc failed.
                errors.push_str(line);
                errors.push('\n');
                continue;
            };
            match value["level"].as_str() {
                Some("error" | "error: internal compiler error") => {
                    errors.push_str(value["rendered"].as_str().unwrap_or(line));
                    error_spans.extend(spans_of(&value, &name, false));
                }
                _ => {
                    if let Some(diagnostic) = lint_diagnostic(&value, &name) {
                        diagnostics.push(diagnostic);
                    }
                }
            }
        }
        if output.status.success() {
            Ok(Outcome::Compiled(diagnostics))
        } else {
            if errors.is_empty() {
                errors = stderr.into_owned();
            }
            Ok(Outcome::Failed {
                errors,
                spans: error_spans,
            })
        }
    }
}

impl Drop for Compiler<'_> {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// The diagnostic `value` if it is one of the two lints, with its primary
/// spans in the file `name` that come from the text itself.
fn lint_diagnostic(value: &Value, name: &str) -> Option<Diagnostic> {
    let lint = value["code"]["code"].as_str()?;
    if lint != "dead_code" && lint != "unused_imports" {
        return None;
    }
    Some(Diagnostic {
        lint: lint.to_string(),
        spans: spans_of(value, name, true),
    })
}

/// The byte ranges of the spans of `value` that are in the file `name` and
/// do not come from a macro: the primary ones, or with `primary` false all
/// of them, those of its notes included (they show the calls and the
/// definitions that an error is about).
fn spans_of(value: &Value, name: &str, primary: bool) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    for span in value["spans"].as_array().into_iter().flatten() {
        if (primary && span["is_primary"].as_bool() != Some(true))
            || !span["expansion"].is_null()
            || span["file_name"].as_str() != Some(name)
        {
            continue;
        }
        let (Some(start), Some(end)) = (span["byte_start"].as_u64(), span["byte_end"].as_u64())
        else {
            continue;
        };
        if let (Ok(start), Ok(end)) = (usize::try_from(start), usize::try_from(end)) {
            spans.push((start, end));
        }
    }
    if !primary {
        for child in value["children"].as_array().into_iter().flatten() {
            spans.extend(spans_of(child, name, false));
        }
    }
    spans
}
