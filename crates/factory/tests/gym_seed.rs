//! Calibrate the development scenario oracle against its trusted defective seed.
//! This is not a runner for untrusted agent-produced code and not a live gym run.
use std::{fs, path::Path, process::Command};

#[test]
fn retry_policy_oracle_detects_the_seed_defect() {
    let scenario =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../gym/scenarios/retry-policy-v1");
    let work = tempfile::tempdir().unwrap();
    fs::create_dir(work.path().join("src")).unwrap();
    fs::create_dir(work.path().join("tests")).unwrap();
    for file in ["Cargo.toml", "Cargo.lock", "src/lib.rs"] {
        fs::copy(scenario.join("seed").join(file), work.path().join(file)).unwrap();
    }
    fs::copy(
        scenario.join("grader/behavior.rs"),
        work.path().join("tests/behavior.rs"),
    )
    .unwrap();
    let result = Command::new(env!("CARGO"))
        .args(["test", "--locked", "--offline"])
        .current_dir(work.path())
        .env("CARGO_TARGET_DIR", work.path().join("target"))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        !result.status.success(),
        "defective seed unexpectedly passed"
    );
    assert!(
        stdout.contains("legacy_requires_a_key ... FAILED"),
        "wrong failure: {stdout}"
    );
    assert!(
        stdout.contains("2 passed; 1 failed"),
        "oracle did not execute: {stdout}"
    );
}
