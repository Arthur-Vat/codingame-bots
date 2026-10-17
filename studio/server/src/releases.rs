//! A game's releases and baseline bots, compiled on first use.
//!
//! `games/<id>/releases/<id>-vNNN.rs` is compiled with `rustc` into
//! `<data>/bin/<release>-<hash>`, where the hash is of the file's contents,
//! and the binary is reused afterwards. The release files are never touched.
//!
//! A baseline bot (ADR 0040) is bundled from the current code of its folder
//! in `games/<id>/bots/`, then compiled the same way: its hash is of the
//! bundle, so a change of the code compiles it again. A bot is bundled once
//! per run of the server: a change is played after a restart.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use cg_bundler::Options;
use studio_game::StudioGame;

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
    /// The bundles made so far, by bot name. The lock is held while one is
    /// made, so that two games starting the same bot bundle it once.
    bundles: Mutex<HashMap<String, String>>,
}

impl Releases {
    pub fn new(repo: PathBuf, bin: PathBuf) -> Releases {
        Releases {
            repo,
            bin,
            compiling: Mutex::new(()),
            bundles: Mutex::new(HashMap::new()),
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

    /// Compiles the bot `name` of `game`, a release or a baseline bot, unless
    /// it is already compiled. Errors carry `rustc`'s messages, or the
    /// bundler's for a baseline bot.
    ///
    /// `name` is only ever compared with the game's lists: no path is made
    /// from it before it is found there.
    pub fn compile(&self, game: &dyn StudioGame, name: &str) -> Result<Compiled, String> {
        let id = game.info().id;
        if self.list(id).iter().any(|release| release == name) {
            let source = self.dir(id).join(format!("{name}.rs"));
            let contents =
                fs::read(&source).map_err(|error| format!("cannot read {name}: {error}"))?;
            return self.build(name, &contents, &Source::File(&source));
        }
        let names = studio_game::baseline_names(game);
        let Some(position) = names.iter().position(|baseline| baseline == name) else {
            return Err(format!("unknown release {name:?}"));
        };
        let bundle = self.bundle(id, name, game.baseline_bots()[position])?;
        self.build(name, bundle.as_bytes(), &Source::Text(&bundle))
    }

    /// The bundle of the baseline bot `name`, which lives in `folder` of
    /// `game`. It is made once per run of the server: later calls return
    /// the same text, without running `cargo metadata` again.
    fn bundle(&self, game: &str, name: &str, folder: &str) -> Result<String, String> {
        let mut bundles = lock(&self.bundles);
        if let Some(bundle) = bundles.get(name) {
            return Ok(bundle.clone());
        }
        let bot = self.repo.join("games").join(game).join("bots").join(folder);
        let manifest = bot.join("Cargo.toml");
        let text = fs::read_to_string(&manifest)
            .map_err(|error| format!("cannot read {}: {error}", manifest.display()))?;
        let package = package_name(&text)
            .ok_or_else(|| format!("{} has no package name", manifest.display()))?;
        // No rustfmt: `rustc` checks the bundle anyway (ADR 0040).
        let options = Options {
            format: false,
            strip: true,
        };
        let bundle =
            cg_bundler::bundle_package(&package, Some(&self.repo.join("Cargo.toml")), options)
                .map_err(|error| format!("cannot bundle {name}: {error}"))?;
        bundles.insert(name.to_string(), bundle.clone());
        Ok(bundle)
    }

    /// Compiles `source` into the binary named after `name` and the hash of
    /// `contents`, unless it exists.
    fn build(&self, name: &str, contents: &[u8], source: &Source) -> Result<Compiled, String> {
        // `DefaultHasher` is only a cache key here, and its algorithm may
        // change with the Rust version: the bot is then compiled again,
        // which is harmless.
        let mut hasher = DefaultHasher::new();
        hasher.write(contents);
        let hash = format!("{:016x}", hasher.finish());
        let path = self
            .bin
            .join(format!("{name}-{hash}{}", std::env::consts::EXE_SUFFIX));
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
            ".{name}.{}.partial{}",
            std::process::id(),
            std::env::consts::EXE_SUFFIX
        ));
        match source {
            Source::File(file) => run_rustc(file, &partial, None),
            Source::Text(text) => {
                let file = self
                    .bin
                    .join(format!(".{name}.{}.partial.rs", std::process::id()));
                let written = fs::write(&file, text)
                    .map_err(|error| format!("cannot write {}: {error}", file.display()));
                let result =
                    written.and_then(|()| run_rustc(&file, &partial, Some("baseline_bot")));
                let _ = fs::remove_file(&file);
                result
            }
        }
        .map_err(|error| format!("cannot compile {name}: {error}"))?;
        fs::rename(&partial, &compiled.path)
            .map_err(|error| format!("cannot store the binary of {name}: {error}"))?;
        Ok(compiled)
    }
}

/// Where the source of a bot is.
enum Source<'a> {
    /// A release file.
    File(&'a Path),
    /// A bundle made in memory.
    Text(&'a str),
}

/// The package name in the text of a bot's `Cargo.toml`: the first line
/// `name = "..."`, as `scripts/bundle-bots.sh` reads it.
fn package_name(manifest: &str) -> Option<String> {
    manifest.lines().find_map(|line| {
        let name = line.strip_prefix("name = \"")?.strip_suffix('"')?;
        Some(name.to_string())
    })
}

/// Compiles `source` into `output`. A crate name is given when the file's
/// name would not make a valid one.
fn run_rustc(source: &Path, output: &Path, crate_name: Option<&str>) -> Result<(), String> {
    let mut command = Command::new("rustc");
    if let Some(crate_name) = crate_name {
        command.args(["--crate-name", crate_name]);
    }
    let result = command
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
