use std::{fs, process::Command};
#[test]
fn failing_check_cannot_qualify_even_when_other_checks_pass() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path();
    fs::write(
        p.join("Makefile"),
        "check:\n\t@exit 7\ntest:\n\t@true\ntest-integration:\n\t@true\n",
    )
    .unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["add", "Makefile"],
        vec![
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-qm",
            "fixture",
        ],
    ] {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(p)
                .status()
                .unwrap()
                .success()
        );
    }
    let sha = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(p)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_spindle-factory"))
        .args(["--repo", p.to_str().unwrap(), "qualify"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    let v: serde_json::Value = serde_json::from_slice(
        &fs::read(
            p.join("artifacts")
                .join(sha.trim())
                .join("qualification.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(v["passed"], false);
    assert_eq!(v["checks"][0]["passed"], false);
    assert_eq!(v["checks"][1]["passed"], true);
    assert_eq!(v["source_sha"], sha.trim());
}
#[test]
fn dirty_tree_is_not_qualified_as_committed_code() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path();
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(p)
            .status()
            .unwrap()
            .success()
    );
    fs::write(p.join("uncommitted"), "content").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_spindle-factory"))
        .args(["--repo", p.to_str().unwrap(), "qualify"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("clean tree"));
}
