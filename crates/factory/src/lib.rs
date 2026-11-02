use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: String,
    pub dependencies: Vec<String>,
    pub requirements: Vec<String>,
    pub acceptance: Vec<String>,
    pub scope: Vec<String>,
    pub prompt: String,
}
pub fn tasks(path: &Path) -> Result<BTreeMap<String, Task>> {
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(path)? {
        let p = entry?.path();
        if p.extension().is_none_or(|v| v != "toml") {
            continue;
        }
        let task: Task = toml::from_str(&fs::read_to_string(&p)?)
            .with_context(|| format!("invalid task {}", p.display()))?;
        if !task.id.starts_with('T')
            || task.id.len() != 4
            || !task.id[1..].bytes().all(|b| b.is_ascii_digit())
        {
            bail!("invalid task id")
        }
        if !["proposed", "ready", "done", "blocked"].contains(&task.status.as_str())
            || task.title.trim().is_empty()
            || task.prompt.trim().is_empty()
            || task.acceptance.is_empty()
            || task.requirements.is_empty()
            || task.scope.is_empty()
            || task.acceptance.iter().any(|a| a.trim().is_empty())
        {
            bail!("incomplete task {}", task.id)
        }
        if result.insert(task.id.clone(), task).is_some() {
            bail!("duplicate task id")
        }
    }
    if result.is_empty() {
        bail!("no tasks found")
    }
    fn visit(
        id: &str,
        tasks: &BTreeMap<String, Task>,
        stack: &mut BTreeSet<String>,
        done: &mut BTreeSet<String>,
    ) -> Result<()> {
        if done.contains(id) {
            return Ok(());
        }
        if !stack.insert(id.into()) {
            bail!("dependency cycle at {id}")
        }
        let task = tasks
            .get(id)
            .with_context(|| format!("missing dependency {id}"))?;
        for dep in &task.dependencies {
            visit(dep, tasks, stack, done)?;
        }
        stack.remove(id);
        done.insert(id.into());
        Ok(())
    }
    let mut done = BTreeSet::new();
    for id in result.keys() {
        visit(id, &result, &mut BTreeSet::new(), &mut done)?;
    }
    Ok(result)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub task: String,
    pub worker: String,
    pub generation: i64,
    pub expires: i64,
    pub state: String,
}
pub struct Ledger {
    conn: Connection,
}
impl Ledger {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.busy_timeout(std::time::Duration::from_secs(10))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE IF NOT EXISTS claims(task TEXT PRIMARY KEY,worker TEXT NOT NULL,generation INTEGER NOT NULL,expires INTEGER NOT NULL,state TEXT NOT NULL); CREATE TABLE IF NOT EXISTS control(id INTEGER PRIMARY KEY CHECK(id=1), paused INTEGER NOT NULL); INSERT OR IGNORE INTO control VALUES(1,0);")?;
        Ok(Self { conn })
    }
    pub fn claim(&mut self, task: &str, worker: &str, now: i64, ttl: i64) -> Result<Claim> {
        if ttl <= 0 || worker.trim().is_empty() {
            bail!("positive lease and worker required")
        }
        let expires = now.checked_add(ttl).context("lease overflow")?;
        let tx = self
            .conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        if tx.query_row("SELECT paused FROM control WHERE id=1", [], |r| {
            r.get::<_, bool>(0)
        })? {
            bail!("dispatch paused")
        }
        let old = tx
            .query_row(
                "SELECT generation,state FROM claims WHERE task=?",
                [task],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()?;
        let generation = match old {
            Some((g, state)) => {
                if state != "released" {
                    bail!(
                        "task already claimed; explicit reconcile/release required, even if expired"
                    )
                }
                g.checked_add(1).context("generation overflow")?
            }
            None => 1,
        };
        tx.execute("INSERT INTO claims VALUES(?1,?2,?3,?4,'running') ON CONFLICT(task) DO UPDATE SET worker=excluded.worker,generation=excluded.generation,expires=excluded.expires,state='running'",params![task,worker,generation,expires])?;
        tx.commit()?;
        Ok(Claim {
            task: task.into(),
            worker: worker.into(),
            generation,
            expires,
            state: "running".into(),
        })
    }
    pub fn heartbeat(
        &self,
        task: &str,
        worker: &str,
        generation: i64,
        now: i64,
        ttl: i64,
    ) -> Result<()> {
        if ttl <= 0 {
            bail!("lease must be positive")
        }
        let expires = now.checked_add(ttl).context("lease overflow")?;
        if self.conn.execute("UPDATE claims SET expires=?1 WHERE task=?2 AND worker=?3 AND generation=?4 AND expires>?5 AND state='running'",params![expires,task,worker,generation,now])?!=1 { bail!("stale or expired claim") }
        Ok(())
    }
    pub fn finish(&self, task: &str, worker: &str, generation: i64, now: i64) -> Result<()> {
        if self.conn.execute("UPDATE claims SET state='review' WHERE task=?1 AND worker=?2 AND generation=?3 AND expires>?4 AND state='running'",params![task,worker,generation,now])?!=1 { bail!("stale or expired completion") }
        Ok(())
    }
    pub fn release(&self, task: &str, generation: i64) -> Result<()> {
        if self.conn.execute(
            "UPDATE claims SET state='released' WHERE task=?1 AND generation=?2",
            params![task, generation],
        )? != 1
        {
            bail!("stale release")
        }
        Ok(())
    }
    pub fn pause(&self, paused: bool) -> Result<()> {
        self.conn
            .execute("UPDATE control SET paused=? WHERE id=1", [paused])?;
        Ok(())
    }
    pub fn status(&self) -> Result<Vec<Claim>> {
        let mut q = self
            .conn
            .prepare("SELECT task,worker,generation,expires,state FROM claims ORDER BY task")?;
        Ok(q.query_map([], |r| {
            Ok(Claim {
                task: r.get(0)?,
                worker: r.get(1)?,
                generation: r.get(2)?,
                expires: r.get(3)?,
                state: r.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expired_worker_cannot_finish_and_reassignment_fences_it() -> Result<()> {
        let d = tempfile::tempdir()?;
        let p = d.path().join("ledger.db");
        let mut l = Ledger::open(&p)?;
        let a = l.claim("T001", "alice", 100, 10)?;
        assert!(l.claim("T001", "bob", 111, 10).is_err());
        assert!(l.finish("T001", "alice", a.generation, 111).is_err());
        l.release("T001", a.generation)?;
        let b = l.claim("T001", "bob", 112, 10)?;
        assert!(l.finish("T001", "alice", a.generation, 113).is_err());
        assert!(l.heartbeat("T001", "alice", a.generation, 113, 20).is_err());
        assert!(l.release("T001", a.generation).is_err());
        l.finish("T001", "bob", b.generation, 113)?;
        assert_eq!(Ledger::open(&p)?.status()?[0].state, "review");
        Ok(())
    }
    #[test]
    fn competing_claims_have_one_winner() -> Result<()> {
        let d = tempfile::tempdir()?;
        let p = d.path().join("ledger.db");
        Ledger::open(&p)?;
        let handles: Vec<_> = (0..4)
            .map(|n| {
                let p = p.clone();
                std::thread::spawn(move || {
                    Ledger::open(&p)
                        .unwrap()
                        .claim("T001", &format!("worker{n}"), 100, 20)
                        .is_ok()
                })
            })
            .collect();
        assert_eq!(
            handles
                .into_iter()
                .filter_map(|h| h.join().ok())
                .filter(|v| *v)
                .count(),
            1
        );
        Ok(())
    }
    #[test]
    fn pause_blocks_new_work_but_not_heartbeats() -> Result<()> {
        let d = tempfile::tempdir()?;
        let mut l = Ledger::open(&d.path().join("db"))?;
        let c = l.claim("T001", "a", 100, 10)?;
        l.pause(true)?;
        assert!(l.claim("T002", "b", 101, 10).is_err());
        l.heartbeat("T001", "a", c.generation, 101, 10)?;
        Ok(())
    }
    #[test]
    fn missing_dependencies_and_cycles_fail() -> Result<()> {
        let d = tempfile::tempdir()?;
        let write = |id: &str, deps: &str| {
            fs::write(
                d.path().join(format!("{id}.toml")),
                format!(
                    "id='{id}'\ntitle='test'\nstatus='ready'\ndependencies=[{deps}]\nrequirements=['EXT-05']\nacceptance=['real check']\nscope=['crates']\nprompt='Implement task'\n"
                ),
            )
        };
        assert!(tasks(d.path()).is_err());
        write("T001", "'T002'")?;
        assert!(tasks(d.path()).is_err());
        write("T002", "'T001'")?;
        assert!(tasks(d.path()).is_err());
        write("T002", "")?;
        assert_eq!(tasks(d.path())?.len(), 2);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewReport {
    pub source_sha: String,
    pub status: ReviewStatus,
    pub findings: Vec<Finding>,
    pub limitations: Vec<String>,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Pass,
    ChangesRequested,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub file: String,
    pub issue: String,
}
pub fn validate_review(bytes: &[u8], sha: &str) -> Result<ReviewReport> {
    let report: ReviewReport = serde_json::from_slice(bytes)?;
    if report.source_sha != sha
        || report.status != ReviewStatus::Pass
        || !report.findings.is_empty()
    {
        bail!("review has findings or mismatched commit")
    }
    Ok(report)
}
#[cfg(test)]
mod review_tests {
    use super::*;
    #[test]
    fn reviews_fail_closed() {
        let good = br#"{"source_sha":"abc","status":"pass","findings":[],"limitations":[]}"#;
        assert!(validate_review(good, "abc").is_ok());
        assert!(validate_review(good, "def").is_err());
        assert!(
            validate_review(
                br#"{"source_sha":"abc","status":"pass","findings":[]}"#,
                "abc"
            )
            .is_err()
        );
        assert!(validate_review(br#"{"source_sha":"abc","status":"pass","findings":[{"file":"a","issue":"broken"}],"limitations":[]}"#,"abc").is_err());
        assert!(validate_review(br#"{"source_sha":"abc","status":"changes_requested","findings":[],"limitations":[]}"#,"abc").is_err());
    }
}
