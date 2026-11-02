#![cfg(unix)]
use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};
fn fixture(p: &Path, scope: &str) {
    fs::create_dir_all(p.join("tasks/specs")).unwrap();
    fs::write(p.join(".gitignore"), ".factory/\nartifacts/\n").unwrap();
    fs::write(p.join("tasks/specs/T001.toml"),format!("id='T001'\ntitle='fixture'\nstatus='ready'\ndependencies=[]\nrequirements=['EXT-13']\nacceptance=['committed file']\nscope=['{scope}']\nprompt='fixture'\n")).unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["add", "."],
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
}
fn run(p: &Path, bin: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_spindle-factory"))
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .args([
            "--repo",
            p.to_str().unwrap(),
            "run",
            "T001",
            "fixture-worker",
            "--timeout",
            "10",
        ])
        .output()
        .unwrap()
}
fn fake_agent(bin: &Path) {
    fs::create_dir_all(bin).unwrap();
    let path = bin.join("codex");
    fs::write(&path,"#!/bin/sh\nset -eu\nprintf 'fixture change\\n' > changed.txt\ngit add changed.txt\ngit -c user.name=Fixture -c user.email=fixture@example.invalid commit -qm change\n").unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}
#[test]
fn runner_retains_isolated_committed_work_for_review() {
    let repo = tempfile::tempdir().unwrap();
    let runtime = tempfile::tempdir().unwrap();
    fixture(repo.path(), "changed.txt");
    fake_agent(runtime.path());
    let result = run(repo.path(), runtime.path());
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(!repo.path().join("changed.txt").exists());
    assert!(
        repo.path()
            .join(".factory/workspaces/T001-1/changed.txt")
            .exists()
    );
    assert!(repo.path().join(".factory/attempts/T001-1.json").exists());
    let l = spindle_factory::Ledger::open(&repo.path().join(".factory/ledger.db")).unwrap();
    assert_eq!(l.status().unwrap()[0].state, "review");
}
#[test]
fn out_of_scope_change_is_not_handed_off() {
    let repo = tempfile::tempdir().unwrap();
    let runtime = tempfile::tempdir().unwrap();
    fixture(repo.path(), "allowed.txt");
    fake_agent(runtime.path());
    let result = run(repo.path(), runtime.path());
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("out-of-scope"));
    let l = spindle_factory::Ledger::open(&repo.path().join(".factory/ledger.db")).unwrap();
    assert_eq!(l.status().unwrap()[0].state, "running");
}
#[test]
fn timed_out_worker_does_not_complete_or_lose_its_claim() {
    let repo = tempfile::tempdir().unwrap();
    let runtime = tempfile::tempdir().unwrap();
    fixture(repo.path(), "changed.txt");
    fake_agent(runtime.path());
    fs::write(runtime.path().join("codex"), "#!/bin/sh\nsleep 30\n").unwrap();
    let started = std::time::Instant::now();
    let result = Command::new(env!("CARGO_BIN_EXE_spindle-factory"))
        .env(
            "PATH",
            format!(
                "{}:{}",
                runtime.path().display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .args([
            "--repo",
            repo.path().to_str().unwrap(),
            "run",
            "T001",
            "fixture-worker",
            "--timeout",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(started.elapsed().as_secs() < 10);
    assert!(String::from_utf8_lossy(&result.stderr).contains("timed out"));
    let l = spindle_factory::Ledger::open(&repo.path().join(".factory/ledger.db")).unwrap();
    assert_eq!(l.status().unwrap()[0].state, "running");
}
