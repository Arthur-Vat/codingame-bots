//! A game's releases, compiled on first use.
//!
//! `games/<id>/releases/<id>-vNNN.rs` is compiled with `rustc` into
//! `<data>/bin/<release>-<hash>`, where the hash is of the file's contents,
//! and the binary is reused afterwards. The release files are never touched.

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use crate::state::lock;

/// A compiled release.
#[derive(Clone, Debug)]
pub struct Compiled {
    pub path: PathBuf,
    /// 16 hex digits of a hash of the source: a cache key, not a security
    /// measure.
    pub hash: String,
}

pub struct Releases {
    repo: PathBuf,
    bin: PathBuf,
    compiling: Mutex<()>,
}

impl Releases {
    pub fn new(repo: PathBuf, bin: PathBuf) -> Releases {
        Releases {
            repo,
            bin,
            compiling: Mutex::new(()),
        }
    }

    fn dir(&self, game: &str) -> PathBuf {
        self.repo.join("games").join(game).join("releases")
    }

    /// The names of the releases of `game`, newest first.
    pub fn list(&self, game: &str) -> Vec<String> {
        let Ok(entries) = fs::read_dir(self.dir(game)) else {
            return Vec::new();
        };
        let mut found: Vec<(u32, String)> = entries
            .filter_map(|entry| {
                let name = entry.ok()?.file_name().into_string().ok()?;
                let stem = name.strip_suffix(".rs")?;
                let number = stem.strip_prefix(game)?.strip_prefix("-v")?;
                Some((number.parse().ok()?, stem.to_string()))
            })
            .collect();
        found.sort_by(|a, b| b.cmp(a));
        found.into_iter().map(|(_, name)| name).collect()
    }

    /// Compiles `release` of `game` unless it is already compiled.
    /// Errors carry `rustc`'s messages.
    pub fn compile(&self, game: &str, release: &str) -> Result<Compiled, String> {
        if !self.list(game).iter().any(|name| name == release) {
            return Err(format!("unknown release {release:?}"));
        }
        let source = self.dir(game).join(format!("{release}.rs"));
        let contents =
            fs::read(&source).map_err(|error| format!("cannot read {release}: {error}"))?;
        // `DefaultHasher` is only a cache key here, and its algorithm may
        // change with the Rust version: the release is then compiled again,
        // which is harmless.
        let mut hasher = DefaultHasher::new();
        hasher.write(&contents);
        let hash = format!("{:016x}", hasher.finish());
        let path = self
            .bin
            .join(format!("{release}-{hash}{}", std::env::consts::EXE_SUFFIX));
        let compiled = Compiled { path, hash };
        if compiled.path.is_file() {
            return Ok(compiled);
        }
        let _one_at_a_time = lock(&self.compiling);
        if compiled.path.is_file() {
            return Ok(compiled);
        }
        fs::create_dir_all(&self.bin)
            .map_err(|error| format!("cannot create {}: {error}", self.bin.display()))?;
        let partial = self.bin.join(format!(
            ".{release}.{}.partial{}",
            std::process::id(),
            std::env::consts::EXE_SUFFIX
        ));
        run_rustc(&source, &partial)
            .map_err(|error| format!("cannot compile {release}: {error}"))?;
        fs::rename(&partial, &compiled.path)
            .map_err(|error| format!("cannot store the binary of {release}: {error}"))?;
        Ok(compiled)
    }
}

fn run_rustc(source: &Path, output: &Path) -> Result<(), String> {
    let result = Command::new("rustc")
        .args(["--edition", "2021", "-C", "opt-level=3", "-o"])
        .arg(output)
        .arg(source)
        .output()
        .map_err(|error| format!("cannot run rustc (is Rust installed and on PATH?): {error}"))?;
    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).into_owned())
    }
}
