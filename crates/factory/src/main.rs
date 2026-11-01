use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::json;
use spindle_factory::{Ledger, tasks};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[arg(long, default_value = ".factory/ledger.db")]
    ledger: PathBuf,
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    Validate,
    Status,
    Claim {
        task: String,
        worker: String,
        #[arg(long, default_value_t = 900)]
        ttl: i64,
    },
    Heartbeat {
        task: String,
        worker: String,
        generation: i64,
        #[arg(long, default_value_t = 900)]
        ttl: i64,
    },
    Finish {
        task: String,
        worker: String,
        generation: i64,
    },
    /// Release only after confirming the old worker is stopped; never auto-steal an expired lease.
    Release {
        task: String,
        generation: i64,
        #[arg(long)]
        worker_stopped: bool,
    },
    Pause,
    Resume,
    Doctor,
    Qualify,
    Review {
        #[arg(long, default_value_t = 180)]
        timeout: u64,
    },
    Run {
        task: String,
        worker: String,
        #[arg(long, default_value_t = 900)]
        timeout: u64,
    },
}
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs() as i64
}
fn output(repo: &Path, args: &[&str]) -> Result<String> {
    let result = Command::new("git").current_dir(repo).args(args).output()?;
    if !result.status.success() {
        bail!(
            "git {} failed: {}",
            args[0],
            String::from_utf8_lossy(&result.stderr)
        )
    }
    Ok(String::from_utf8(result.stdout)?.trim().into())
}
fn clean(repo: &Path) -> Result<String> {
    if !output(repo, &["status", "--porcelain"])?.is_empty() {
        bail!("commit changes first: exact-commit operations require a clean tree")
    }
    output(repo, &["rev-parse", "HEAD"])
}
fn clone_at(repo: &Path, dest: &Path, sha: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["clone", "--quiet", "--no-hardlinks"])
        .arg(repo)
        .arg(dest)
        .status()?;
    if !status.success() {
        bail!("isolated clone failed")
    }
    output(dest, &["checkout", "--quiet", "--detach", sha])?;
    Ok(())
}
fn bounded(command: &mut Command, timeout: u64) -> Result<bool> {
    if timeout == 0 {
        bail!("timeout must be positive")
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.stdin(Stdio::null()).spawn()?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status.success());
        }
        if start.elapsed() >= Duration::from_secs(timeout) {
            #[cfg(unix)]
            {
                Command::new("kill")
                    .args(["-TERM", "--", &format!("-{}", child.id())])
                    .status()
                    .ok();
                std::thread::sleep(Duration::from_millis(200));
                Command::new("kill")
                    .args(["-KILL", "--", &format!("-{}", child.id())])
                    .status()
                    .ok();
            }
            child.kill().ok();
            child.wait().ok();
            bail!("worker timed out; artifacts retained")
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo = cli.repo.canonicalize()?;
    let db = if cli.ledger.is_absolute() {
        cli.ledger
    } else {
        repo.join(cli.ledger)
    };
    match cli.command {
        Action::Validate => {
            let ts = tasks(&repo.join("tasks/specs"))?;
            println!("validated {} tasks", ts.len());
        }
        Action::Status => println!(
            "{}",
            serde_json::to_string_pretty(&Ledger::open(&db)?.status()?)?
        ),
        Action::Claim { task, worker, ttl } => {
            let ts = tasks(&repo.join("tasks/specs"))?;
            let t = ts.get(&task).context("unknown task")?;
            if t.status != "ready" || t.dependencies.iter().any(|d| ts[d].status != "done") {
                bail!("task is not ready or dependencies incomplete")
            }
            println!(
                "{}",
                serde_json::to_string(&Ledger::open(&db)?.claim(&task, &worker, now(), ttl)?)?
            );
        }
        Action::Heartbeat {
            task,
            worker,
            generation,
            ttl,
        } => Ledger::open(&db)?.heartbeat(&task, &worker, generation, now(), ttl)?,
        Action::Finish {
            task,
            worker,
            generation,
        } => Ledger::open(&db)?.finish(&task, &worker, generation, now())?,
        Action::Release {
            task,
            generation,
            worker_stopped,
        } => {
            if !worker_stopped {
                bail!("confirm --worker-stopped before releasing")
            };
            Ledger::open(&db)?.release(&task, generation)?;
        }
        Action::Pause => Ledger::open(&db)?.pause(true)?,
        Action::Resume => Ledger::open(&db)?.pause(false)?,
        Action::Doctor => {
            let mut bad = false;
            for (tool, args) in [
                ("cargo", vec!["--version"]),
                ("docker", vec!["compose", "version"]),
                ("docker", vec!["info", "--format", "{{.ServerVersion}}"]),
                ("codex", vec!["--version"]),
            ] {
                let ok = Command::new(tool)
                    .args(args)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .is_ok_and(|s| s.success());
                println!("{tool}: {}", if ok { "available" } else { "unavailable" });
                bad |= !ok;
            }
            if bad {
                bail!("required tool unavailable")
            }
        }
        Action::Qualify => {
            let sha = clean(&repo)?;
            let temp = tempfile::tempdir()?;
            let work = temp.path().join("checkout");
            clone_at(&repo, &work, &sha)?;
            let dir = repo.join("artifacts").join(&sha);
            fs::create_dir_all(&dir)?;
            let mut results = Vec::new();
            for target in ["check", "test", "test-integration"] {
                let log = fs::File::create(dir.join(format!("{target}.log")))?;
                let passed = bounded(
                    Command::new("make")
                        .arg(target)
                        .current_dir(&work)
                        .stdout(log.try_clone()?)
                        .stderr(log),
                    900,
                )
                .unwrap_or(false);
                results.push(json!({"target":target,"passed":passed}));
            }
            let passed = results.iter().all(|r| r["passed"] == true);
            fs::write(
                dir.join("qualification.json"),
                serde_json::to_vec_pretty(
                    &json!({"source_sha":sha,"checks":results,"passed":passed,"review_required":true}),
                )?,
            )?;
            if !passed {
                bail!("qualification failed; see {}", dir.display())
            }
            println!(
                "deterministic qualification passed for {sha}; independent review still required"
            );
        }
        Action::Review { timeout } => {
            let sha = clean(&repo)?;
            let dir = repo.join("artifacts").join(&sha);
            fs::create_dir_all(&dir)?;
            let temp = tempfile::tempdir()?;
            let work = temp.path().join("checkout");
            clone_at(&repo, &work, &sha)?;
            let schema = repo.join("factory/review.schema.json");
            let report = dir.join("review.json");
            if report.exists() {
                fs::remove_file(&report)?;
            }
            let log = fs::File::create(dir.join("review.log"))?;
            let prompt = format!(
                "Review commit {sha} in this repository against AGENTS.md, docs/planning and tasks. Inspect code and tests for correctness, security, missing behavior and weakened gates. Do not edit files. Return the schema with this exact source_sha. Status pass only if no blocking findings. Review limitations must be explicit. This is independent review, not implementation. Do not run paid agents or deploy anything."
            );
            let ok = bounded(
                Command::new("codex")
                    .args(["exec", "--sandbox", "read-only", "--output-schema"])
                    .arg(schema)
                    .arg("--output-last-message")
                    .arg(&report)
                    .arg(prompt)
                    .current_dir(work)
                    .stdout(log.try_clone()?)
                    .stderr(log),
                timeout,
            )?;
            if !ok {
                bail!("reviewer failed; no approval")
            }
            spindle_factory::validate_review(&fs::read(&report)?, &sha)?;
            println!("review passed for {sha}; report {}", report.display());
        }
        Action::Run {
            task,
            worker,
            timeout,
        } => {
            let sha = clean(&repo)?;
            let ts = tasks(&repo.join("tasks/specs"))?;
            let t = ts.get(&task).context("unknown task")?;
            if t.status != "ready" || t.dependencies.iter().any(|d| ts[d].status != "done") {
                bail!("task is not eligible")
            }
            let ttl = i64::try_from(timeout)?
                .checked_add(60)
                .context("timeout overflow")?;
            let claim = Ledger::open(&db)?.claim(&task, &worker, now(), ttl)?;
            let work = repo
                .join(".factory/workspaces")
                .join(format!("{task}-{}", claim.generation));
            fs::create_dir_all(work.parent().unwrap())?;
            let receipts = repo.join(".factory/attempts");
            fs::create_dir_all(&receipts)?;
            fs::write(
                receipts.join(format!("{task}-{}.json", claim.generation)),
                serde_json::to_vec_pretty(
                    &json!({"claim":claim,"base_sha":sha,"task_snapshot":t,"workspace":work}),
                )?,
            )?;
            clone_at(&repo, &work, &sha)?;
            output(
                &work,
                &[
                    "switch",
                    "-c",
                    &format!("factory/{task}-{}", claim.generation),
                ],
            )?;
            let log = fs::File::create(
                work.parent()
                    .unwrap()
                    .join(format!("{task}-{}.log", claim.generation)),
            )?;
            let prompt = format!(
                "Implement task {}: {}\n{}\nScope: {:?}\nAcceptance: {:?}\nFollow AGENTS.md. Do not merge, push, deploy, change factory controls or launch nested agents. Commit your changes and report checks and blockers. Completion is a handoff to independent review, not approval.",
                t.id, t.title, t.prompt, t.scope, t.acceptance
            );
            let ok = bounded(
                Command::new("codex")
                    .args([
                        "exec",
                        "--sandbox",
                        "workspace-write",
                        "-c",
                        "approval_policy=\"never\"",
                    ])
                    .arg(prompt)
                    .current_dir(&work)
                    .stdout(log.try_clone()?)
                    .stderr(log),
                timeout,
            )?;
            if !ok {
                bail!("worker failed; claim/workspace retained for recovery")
            }
            let result_sha = clean(&work)?;
            if result_sha == sha {
                bail!("worker produced no committed change; claim retained")
            }
            let changed = output(&work, &["diff", "--name-only", &sha, &result_sha])?;
            for file in changed.lines() {
                if !t.scope.iter().any(|scope| {
                    if scope.ends_with('/') {
                        file.starts_with(scope)
                    } else {
                        file == scope
                    }
                }) {
                    bail!("worker modified out-of-scope path {file}; no review handoff")
                }
            }
            Ledger::open(&db)?.finish(&task, &worker, claim.generation, now())?;
            println!(
                "worker handed off for review at {}; no merge performed",
                work.display()
            );
        }
    }
    Ok(())
}
