//! The `cg-packer` command writes a module that unpacks to the input file.

use std::path::Path;
use std::process::Command;

#[test]
fn packs_a_file_into_a_module() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cg-packer-cli");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("table.bin");
    let output = dir.join("table.rs");
    let data: Vec<u8> = (0..1_000u32).map(|i| (i * 7 % 251) as u8).collect();
    std::fs::write(&input, &data).unwrap();

    let run = Command::new(env!("CARGO_BIN_EXE_cg-packer"))
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .args(["--name", "TABLE"])
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let module = std::fs::read_to_string(&output).unwrap();
    assert_eq!(
        module,
        cg_packer::rust_module(&data, "TABLE", "`table.bin`").unwrap()
    );

    let bad = Command::new(env!("CARGO_BIN_EXE_cg-packer"))
        .arg(&input)
        .arg("--output")
        .arg(dir.join("bad.rs"))
        .args(["--name", "table"])
        .output()
        .unwrap();
    assert!(!bad.status.success());
    assert!(
        !dir.join("bad.rs").exists(),
        "nothing is written on an error"
    );
    let missing = Command::new(env!("CARGO_BIN_EXE_cg-packer"))
        .arg(dir.join("missing.bin"))
        .arg("--output")
        .arg(dir.join("missing.rs"))
        .output()
        .unwrap();
    assert!(!missing.status.success());
    std::fs::remove_dir_all(&dir).unwrap();
}
