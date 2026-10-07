//! Which files go into a bundle, found with `cargo metadata`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// A crate to include: its name in Rust code and the file it starts from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateSource {
    pub name: String,
    pub root: PathBuf,
}

/// Everything a bundle is made of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    /// The bot's package name.
    pub package: String,
    /// The bot's binary crate.
    pub bot: CrateSource,
    /// The workspace libraries the bot uses, directly or not, by name.
    pub libraries: Vec<CrateSource>,
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    manifest_path: PathBuf,
    targets: Vec<Target>,
    dependencies: Vec<Dependency>,
}

#[derive(Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
    src_path: PathBuf,
}

#[derive(Deserialize)]
struct Dependency {
    name: String,
    kind: Option<String>,
    path: Option<PathBuf>,
    rename: Option<String>,
}

/// Plans the bundle of the bot package `package`, using the workspace of
/// `manifest` (or of the current directory).
pub fn plan(package: &str, manifest: Option<&Path>) -> Result<Plan, String> {
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
    command.args(["metadata", "--format-version", "1", "--no-deps"]);
    if let Some(manifest) = manifest {
        command.arg("--manifest-path").arg(manifest);
    }
    let output = command
        .output()
        .map_err(|err| format!("cannot run cargo metadata: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .map_err(|err| format!("cannot read cargo metadata: {err}"))?;
    plan_from(&metadata, package)
}

fn plan_from(metadata: &Metadata, package: &str) -> Result<Plan, String> {
    let bot = metadata
        .packages
        .iter()
        .find(|p| p.name == package)
        .ok_or_else(|| format!("no package named {package:?} in the workspace"))?;
    let binaries: Vec<&Target> = bot
        .targets
        .iter()
        .filter(|t| t.kind.iter().any(|k| k == "bin"))
        .collect();
    let [binary] = binaries.as_slice() else {
        return Err(format!(
            "package {package:?} must have exactly one binary, it has {}",
            binaries.len()
        ));
    };

    let mut libraries = BTreeMap::new();
    collect_libraries(metadata, bot, &mut libraries)?;
    Ok(Plan {
        package: package.to_string(),
        bot: CrateSource {
            name: binary.name.replace('-', "_"),
            root: binary.src_path.clone(),
        },
        libraries: libraries.into_values().collect(),
    })
}

/// Adds the workspace libraries `package` depends on, recursively.
fn collect_libraries(
    metadata: &Metadata,
    package: &Package,
    found: &mut BTreeMap<String, CrateSource>,
) -> Result<(), String> {
    for dependency in &package.dependencies {
        if dependency.kind.is_some() {
            continue; // dev- and build-dependencies never reach the bot.
        }
        let Some(path) = &dependency.path else {
            return Err(format!(
                "{} depends on {:?} from a registry; code in a bot may only use workspace crates",
                package.name, dependency.name
            ));
        };
        if dependency.rename.is_some() {
            return Err(format!(
                "{} renames its dependency {:?}; the bundler does not support renames",
                package.name, dependency.name
            ));
        }
        let library_package = metadata
            .packages
            .iter()
            .find(|p| p.manifest_path.parent() == Some(path.as_path()))
            .ok_or_else(|| {
                format!(
                    "{:?} at {} is not a workspace member",
                    dependency.name,
                    path.display()
                )
            })?;
        let library = library_package
            .targets
            .iter()
            .find(|t| t.kind.iter().any(|k| k == "lib"))
            .ok_or_else(|| format!("{:?} has no library target", library_package.name))?;
        let name = library.name.replace('-', "_");
        if found.contains_key(&name) {
            continue;
        }
        found.insert(
            name.clone(),
            CrateSource {
                name,
                root: library.src_path.clone(),
            },
        );
        collect_libraries(metadata, library_package, found)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
