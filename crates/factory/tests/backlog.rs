use spindle_factory::{Ledger, backlog, tasks};
use std::{fs, process::Command};

#[test]
fn pickup_respects_dependencies_pause_and_unreconciled_claims() {
    let dir = tempfile::tempdir().unwrap();
    let specs = dir.path().join("tasks/specs");
    fs::create_dir_all(&specs).unwrap();
    for (id, status, deps) in [
        ("T001", "done", ""),
        ("T002", "ready", "'T001'"),
        ("T003", "ready", "'T002'"),
        ("T004", "proposed", ""),
    ] {
        fs::write(specs.join(format!("{id}.toml")), format!("id='{id}'\ntitle='test'\nstatus='{status}'\ndependencies=[{deps}]\nrequirements=['EXT-05']\nacceptance=['checked']\nscope=['crates/']\nprompt='implement'\n")).unwrap();
    }
    let ts = tasks(&specs).unwrap();
    let mut ledger = Ledger::open(&dir.path().join(".factory/ledger.db")).unwrap();
    let eligible = |rows: Vec<spindle_factory::TaskReadiness>| {
        rows.into_iter()
            .filter(|t| t.eligible)
            .map(|t| t.id)
            .collect::<Vec<_>>()
    };
    assert_eq!(eligible(backlog(&ts, &[], false)), ["T002"]);
    assert!(eligible(backlog(&ts, &[], true)).is_empty());
    // An expired claim must not silently become dispatchable.
    let claim = ledger.claim("T002", "worker", 1, 1).unwrap();
    assert!(eligible(backlog(&ts, &ledger.status().unwrap(), false)).is_empty());
    ledger.release("T002", claim.generation).unwrap();
    let invoke = |action| {
        Command::new(env!("CARGO_BIN_EXE_spindle-factory"))
            .arg("--repo")
            .arg(dir.path())
            .arg(action)
            .output()
            .unwrap()
    };
    let next = invoke("next");
    assert!(next.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&next.stdout).unwrap()["id"],
        "T002"
    );
    // Selection is advisory and never acquires a claim.
    assert_eq!(ledger.status().unwrap()[0].state, "released");
    ledger.pause(true).unwrap();
    let next = invoke("next");
    assert!(next.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&next.stdout).unwrap(),
        serde_json::Value::Null
    );
    let all = invoke("backlog");
    assert!(all.status.success());
    let rows: serde_json::Value = serde_json::from_slice(&all.stdout).unwrap();
    assert_eq!(rows.as_array().unwrap().len(), 4);
    assert!(
        rows[2]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.as_str().unwrap().contains("dependency T002"))
    );
}
