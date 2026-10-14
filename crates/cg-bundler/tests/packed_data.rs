//! A bot that carries packed data (cg-pack) bundles into a file of more
//! than 100,000 bytes but fewer than 100,000 characters, which compiles on
//! its own, passes `scripts/cg-check.sh` and unpacks its data (ADR 0037).

use std::path::Path;
use std::process::Command;

use cg_bundler::plan::{CrateSource, Plan};

/// Bytes from a fixed xorshift64 sequence.
fn noise(len: usize) -> Vec<u8> {
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 56) as u8
        })
        .collect()
}

/// FNV-1a, which the bot below computes too.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xCBF2_9CE4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01B3)
    })
}

const MAIN: &str = r#"mod data;

fn main() {
    let bytes = cg_pack::unpack(data::DATA).expect("packed data");
    let hash = bytes.iter().fold(0xCBF2_9CE4_8422_2325u64, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01B3)
    });
    println!("{} {}", bytes.len(), hash);
}
"#;

#[test]
fn a_bot_with_packed_data_over_100000_bytes_bundles_compiles_and_unpacks_it() {
    let dir = std::env::temp_dir().join(format!("cg-bundler-packed-{}", std::process::id()));
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    // 160,000 bytes take 80,640 characters, about 242,000 bytes of UTF-8.
    let data = noise(160_000);
    std::fs::write(src.join("main.rs"), MAIN).unwrap();
    std::fs::write(
        src.join("data.rs"),
        cg_packer::rust_module(&data, "DATA", "a test").unwrap(),
    )
    .unwrap();

    let cg_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("../cg-pack/src/lib.rs");
    let plan = Plan {
        package: "packed-probe".to_string(),
        bot: CrateSource {
            name: "packed_probe".to_string(),
            root: src.join("main.rs"),
        },
        libraries: vec![CrateSource {
            name: "cg_pack".to_string(),
            root: cg_pack,
        }],
    };
    let bundle = cg_bundler::bundle(&plan, cg_bundler::Options::default()).unwrap();
    assert!(bundle.len() > 100_000, "{} bytes", bundle.len());
    assert!(
        cg_bundler::cg_size(&bundle) < cg_bundler::SIZE_LIMIT,
        "{} characters",
        cg_bundler::cg_size(&bundle)
    );
    assert!(
        !bundle.contains("mod tests"),
        "test modules must be dropped"
    );

    let file = dir.join("packed_probe.rs");
    std::fs::write(&file, &bundle).unwrap();
    let binary = compile(&file, &dir);
    let run = Command::new(&binary).output().unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        String::from_utf8(run.stdout).unwrap(),
        format!("{} {}\n", data.len(), fnv1a(&data))
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

/// Compiles the bundle with `scripts/cg-check.sh`, the CodinGame check CI
/// runs on bots, and returns the binary.
#[cfg(unix)]
fn compile(file: &Path, dir: &Path) -> std::path::PathBuf {
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/cg-check.sh");
    let check = Command::new(script)
        .arg(file)
        .env("CG_BIN_DIR", dir)
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "cg-check.sh rejects the bundle:\n{}{}",
        String::from_utf8_lossy(&check.stdout),
        String::from_utf8_lossy(&check.stderr)
    );
    dir.join(file.file_stem().unwrap())
}

/// Compiles the bundle with rustc alone, as CodinGame does: the check
/// script needs bash.
#[cfg(not(unix))]
fn compile(file: &Path, dir: &Path) -> std::path::PathBuf {
    let binary = dir.join("packed_probe");
    let compiled = Command::new("rustc")
        .args(["--edition", "2021", "-O", "--crate-type", "bin", "-o"])
        .arg(&binary)
        .arg(file)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "the bundle does not compile:\n{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    binary
}
