//! Integration tests for the `init` subcommand.
//!
//! `init` compares two builds and writes a starter `.safeguard.toml` with
//! commented-out `[[suppress]]` blocks for each finding.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn wasm(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wasm")
        .join(name)
}

#[test]
fn init_writes_a_safeguard_toml_for_a_breaking_pair() {
    let dir = std::env::temp_dir().join(format!(
        "soroban-upgrade-safeguard-init-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("failed to create temp working directory");

    let output = Command::new(env!("CARGO_BIN_EXE_soroban-upgrade-safeguard"))
        .arg("init")
        .arg(wasm("v1.wasm"))
        .arg(wasm("v2.wasm"))
        .current_dir(&dir)
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let config = fs::read_to_string(dir.join(".safeguard.toml"));
    let _ = fs::remove_dir_all(&dir);

    assert_eq!(output.status.code(), Some(0), "init must succeed: {stdout}");
    assert!(
        stdout.contains("Generated .safeguard.toml"),
        "init should announce the file it wrote, got: {stdout}"
    );

    let config = config.expect(".safeguard.toml should be written to the working directory");
    assert!(
        config.contains("[[suppress]]"),
        "the generated config should include suppress blocks, got: {config}"
    );
}
