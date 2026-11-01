//! Canonical authorization and immutable source evidence. Search is only a projection.
use anyhow::{Result, bail};
use postgres::GenericClient;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug)]
pub struct ApiError(pub u16, pub &'static str);
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}
impl std::error::Error for ApiError {}
fn denied<T>() -> Result<T> {
    Err(ApiError(403, "forbidden").into())
}
fn invalid<T>() -> Result<T> {
    Err(ApiError(400, "invalid_request").into())
}
pub fn hash(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}
pub fn provision(id: &str, token: &str) -> Result<()> {
    if id.is_empty() || id.len() > 128 || token.len() < 32 {
        bail!("identity required; token must contain at least 32 characters")
    }
    super::database()?.execute(
        "INSERT INTO principals(id,token_hash) VALUES($1,$2)",
        &[&id, &hash(token)],
    )?;
    Ok(())
}
#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    CreateProject {
        name: String,
    },
    AddMember {
        project: String,
        principal: String,
    },
    RemoveMember {
        project: String,
        principal: String,
    },
    Activate {
        workspace: String,
        project: String,
    },
    Leave {
        workspace: String,
    },
    Status {
        workspace: String,
    },
    StartSession {
        workspace: String,
    },
    Capture {
        session: String,
        request_key: String,
        text: String,
    },
    Search {
        project: String,
        query: String,
    },
    Inspect {
        evidence: String,
    },
    Retract {
        session: String,
    },
}
fn member(db: &mut impl GenericClient, project: &str, who: &str, owner: bool) -> Result<()> {
    let row = db.query_opt(
        "SELECT role FROM memberships WHERE project_id=$1 AND principal_id=$2",
        &[&project, &who],
    )?;
    if row.is_none_or(|r| owner && r.get::<_, String>(0) != "owner") {
        return denied();
    }
    Ok(())
}
fn text_ok(text: &str, max: usize) -> bool {
    !text.trim().is_empty() && text.len() <= max
}
fn source(db: &mut impl GenericClient, id: &str, who: &str) -> Result<Value> {
    let r=db.query_opt("SELECT e.id,e.text,e.retracted,e.session_id,s.project_id,s.principal_id,e.recorded_at::text FROM evidence e JOIN sessions s ON s.id=e.session_id JOIN memberships m ON m.project_id=s.project_id AND m.principal_id=$2 WHERE e.id=$1", &[&id,&who])?;
    match r {
        None => denied(),
        Some(r) => Ok(
            json!({"id":r.get::<_,String>(0),"text":if r.get::<_,bool>(2){Value::Null}else{json!(r.get::<_,String>(1))},"retracted":r.get::<_,bool>(2),"session":r.get::<_,String>(3),"project":r.get::<_,String>(4),"author":r.get::<_,String>(5),"recorded_at":r.get::<_,String>(6),"kind":"original_evidence"}),
        ),
    }
}
pub fn execute(token: &str, op: Operation) -> Result<Value> {
    let mut db = super::database()?;
    let who: String = db
        .query_opt(
            "SELECT id FROM principals WHERE token_hash=$1",
            &[&hash(token)],
        )?
        .ok_or(ApiError(401, "unauthorized"))?
        .get(0);
    let mut tx = db.transaction()?;
    // A transaction-scoped global guard serializes the first prototype's authority
    // changes. Replace with scoped guards after measured concurrency requirements.
    tx.batch_execute("LOCK TABLE memberships IN SHARE ROW EXCLUSIVE MODE")?;
    let result = match op {
        Operation::CreateProject { name } => {
            if !text_ok(&name, 200) {
                return invalid();
            }
            let id = Uuid::new_v4().to_string();
            tx.execute("INSERT INTO projects(id,name) VALUES($1,$2)", &[&id, &name])?;
            tx.execute(
                "INSERT INTO memberships VALUES($1,$2,'owner')",
                &[&id, &who],
            )?;
            json!({"project":id,"blueprint":"software-v1","role":"owner","active":false})
        }
        Operation::AddMember { project, principal } => {
            member(&mut tx, &project, &who, true)?;
            if tx
                .query_opt("SELECT id FROM principals WHERE id=$1", &[&principal])?
                .is_none()
            {
                return invalid();
            }
            tx.execute(
                "INSERT INTO memberships VALUES($1,$2,'member') ON CONFLICT DO NOTHING",
                &[&project, &principal],
            )?;
            json!({"added":true})
        }
        Operation::RemoveMember { project, principal } => {
            member(&mut tx, &project, &who, true)?;
            if principal == who {
                return invalid();
            }
            tx.execute(
                "DELETE FROM memberships WHERE project_id=$1 AND principal_id=$2 AND role='member'",
                &[&project, &principal],
            )?;
            tx.execute(
                "DELETE FROM activations WHERE project_id=$1 AND principal_id=$2",
                &[&project, &principal],
            )?;
            json!({"removed":true})
        }
        Operation::Activate { workspace, project } => {
            if !text_ok(&workspace, 1000) {
                return invalid();
            }
            member(&mut tx, &project, &who, false)?;
            tx.execute("INSERT INTO activations VALUES($1,$2,$3) ON CONFLICT(principal_id,workspace) DO UPDATE SET project_id=excluded.project_id", &[&who,&workspace,&project])?;
            json!({"project":project,"workspace":workspace,"capture":"explicit_evidence_only"})
        }
        Operation::Leave { workspace } => {
            tx.execute(
                "DELETE FROM activations WHERE principal_id=$1 AND workspace=$2",
                &[&who, &workspace],
            )?;
            json!({"active":false})
        }
        Operation::Status { workspace } => {
            let r=tx.query_opt("SELECT a.project_id FROM activations a JOIN memberships m ON m.project_id=a.project_id AND m.principal_id=a.principal_id WHERE a.principal_id=$1 AND workspace=$2", &[&who,&workspace])?;
            json!({"project":r.map(|r|r.get::<_,String>(0)),"capture":"explicit_evidence_only"})
        }
        Operation::StartSession { workspace } => {
            let project: String = tx
                .query_opt(
                    "SELECT project_id FROM activations WHERE principal_id=$1 AND workspace=$2",
                    &[&who, &workspace],
                )?
                .ok_or(ApiError(409, "no_active_project"))?
                .get(0);
            member(&mut tx, &project, &who, false)?;
            let id = Uuid::new_v4().to_string();
            tx.execute(
                "INSERT INTO sessions(id,principal_id,project_id,workspace) VALUES($1,$2,$3,$4)",
                &[&id, &who, &project, &workspace],
            )?;
            json!({"session":id,"project":project,"capture":"explicit_evidence_only"})
        }
        Operation::Capture {
            session,
            request_key,
            text,
        } => {
            if !text_ok(&text, 65536) || !text_ok(&request_key, 200) {
                return invalid();
            }
            let project:String=tx.query_opt("SELECT project_id FROM sessions WHERE id=$1 AND principal_id=$2 AND NOT retracted", &[&session,&who])?.ok_or(ApiError(403,"forbidden"))?.get(0);
            member(&mut tx, &project, &who, false)?;
            if let Some(r) = tx.query_opt(
                "SELECT id,text,retracted FROM evidence WHERE session_id=$1 AND request_key=$2",
                &[&session, &request_key],
            )? {
                if r.get::<_, String>(1) != text || r.get::<_, bool>(2) {
                    return Err(ApiError(409, "idempotency_conflict").into());
                }
                json!({"evidence":r.get::<_,String>(0),"duplicate":true})
            } else {
                let id = Uuid::new_v4().to_string();
                tx.execute(
                    "INSERT INTO evidence(id,session_id,request_key,text) VALUES($1,$2,$3,$4)",
                    &[&id, &session, &request_key, &text],
                )?;
                tx.execute("INSERT INTO search_outbox VALUES($1)", &[&id])?;
                json!({"evidence":id,"duplicate":false})
            }
        }
        Operation::Inspect { evidence } => source(&mut tx, &evidence, &who)?,
        Operation::Search { project, query } => {
            member(&mut tx, &project, &who, false)?;
            if !text_ok(&query, 2000) {
                return invalid();
            }
            let response:Value=super::search()?.post(format!("{}/spindle-evidence-v1/_search",super::endpoint()))
                .json(&json!({"size":100,"_source":false,"query":{"bool":{"must":[{"match":{"text":query}}],"filter":[{"term":{"project":project}}]}}}))
                .send()?.error_for_status()?.json()?;
            let mut hits = Vec::new();
            for hit in response["hits"]["hits"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("invalid search response"))?
            {
                if let Some(id) = hit["_id"].as_str() {
                    let v = source(&mut tx, id, &who)?;
                    if v["project"] == project && v["retracted"] == false {
                        hits.push(v)
                    }
                }
            }
            json!({"hits":hits,"mode":"lexical","limit":100})
        }
        Operation::Retract { session } => {
            let project: String = tx
                .query_opt(
                    "SELECT project_id FROM sessions WHERE id=$1 AND principal_id=$2",
                    &[&session, &who],
                )?
                .ok_or(ApiError(403, "forbidden"))?
                .get(0);
            member(&mut tx, &project, &who, false)?;
            tx.execute(
                "UPDATE sessions SET retracted=TRUE WHERE id=$1",
                &[&session],
            )?;
            let rows = tx.query(
                "UPDATE evidence SET retracted=TRUE WHERE session_id=$1 RETURNING id",
                &[&session],
            )?;
            for row in &rows {
                let id: String = row.get(0);
                tx.execute(
                    "INSERT INTO search_outbox VALUES($1) ON CONFLICT DO NOTHING",
                    &[&id],
                )?;
            }
            json!({"status":"projection_pending","sources_retracted":rows.len()})
        }
    };
    tx.commit()?;
    Ok(result)
}

pub fn project_search() -> Result<usize> {
    let c = super::search()?;
    let url = format!("{}/spindle-evidence-v1", super::endpoint());
    let response=c.put(&url).json(&json!({"mappings":{"properties":{"project":{"type":"keyword"},"text":{"type":"text"}}}})).send()?;
    if !response.status().is_success() {
        let status = response.status();
        let body: Value = response.json()?;
        if status.as_u16() != 400 || body["error"]["type"] != "resource_already_exists_exception" {
            bail!("cannot initialize evidence search index")
        }
    }
    let mut db = super::database()?;
    let ids = db.query(
        "SELECT evidence_id FROM search_outbox ORDER BY evidence_id LIMIT 1000",
        &[],
    )?;
    for row in &ids {
        let id: String = row.get(0);
        let mut tx = db.transaction()?;
        tx.batch_execute("LOCK TABLE memberships IN SHARE ROW EXCLUSIVE MODE")?;
        // Read canonical state after acquiring the guard: a delayed retry cannot
        // resurrect a source after its retraction projection has completed.
        let r=tx.query_one("SELECT e.text,e.retracted,s.project_id FROM evidence e JOIN sessions s ON s.id=e.session_id WHERE e.id=$1", &[&id])?;
        if r.get::<_, bool>(1) {
            let response = c.delete(format!("{url}/_doc/{id}?refresh=true")).send()?;
            if response.status().as_u16() != 404 {
                response.error_for_status()?;
            }
        } else {
            c.put(format!("{url}/_doc/{id}?refresh=true"))
                .json(&json!({"text":r.get::<_,String>(0),"project":r.get::<_,String>(2)}))
                .send()?
                .error_for_status()?;
        }
        tx.execute("DELETE FROM search_outbox WHERE evidence_id=$1", &[&id])?;
        tx.commit()?;
    }
    Ok(ids.len())
}

/// Real-storage acceptance slice. Random identities isolate repeated runs;
/// cleanup removes only rows belonging to those test identities.
pub fn integration() -> Result<()> {
    struct Server(std::process::Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    drop(listener);
    let _server = Server(
        std::process::Command::new(std::env::current_exe()?)
            .arg("serve")
            .env("SPINDLE_BIND", address.to_string())
            .stdout(std::process::Stdio::null())
            .spawn()?,
    );
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let base = format!("http://{address}");
    let mut started = false;
    for _ in 0..100 {
        if client
            .get(format!("{base}/healthz"))
            .send()
            .is_ok_and(|r| r.status().is_success())
        {
            started = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    anyhow::ensure!(started, "test HTTP server did not start");
    anyhow::ensure!(
        client
            .post(format!("{base}/v1"))
            .json(&json!({"op":"status","workspace":"repo"}))
            .send()?
            .status()
            .as_u16()
            == 401,
        "missing token accepted"
    );
    anyhow::ensure!(
        client
            .post(format!("{base}/v1"))
            .body("x".repeat(70_001))
            .send()?
            .status()
            .as_u16()
            == 413,
        "oversized request accepted"
    );
    let run = Uuid::new_v4().to_string();
    let users = [
        format!("a-{run}"),
        format!("b-{run}"),
        format!("outsider-{run}"),
    ];
    let tokens = [
        Uuid::new_v4().to_string(),
        Uuid::new_v4().to_string(),
        Uuid::new_v4().to_string(),
    ];
    for (user, token) in users.iter().zip(&tokens) {
        provision(user, token)?;
    }
    let mut projects = Vec::<String>::new();
    let result = (|| -> Result<()> {
        let call = |i: usize, v: Value| -> Result<Value> {
            let response = client
                .post(format!("{base}/v1"))
                .bearer_auth(&tokens[i])
                .json(&v)
                .send()?;
            let status = response.status();
            let result: Value = response.json()?;
            if !status.is_success() {
                bail!("HTTP {status}: {result}")
            }
            Ok(result)
        };
        let p = call(0, json!({"op":"create_project","name":"Retry policy"}))?["project"]
            .as_str()
            .unwrap()
            .to_owned();
        projects.push(p.clone());
        assert!(
            execute(
                "invalid",
                Operation::Status {
                    workspace: "repo".into()
                }
            )
            .is_err()
        );
        assert!(call(2, json!({"op":"activate","workspace":"repo","project":p})).is_err());
        call(
            0,
            json!({"op":"add_member","project":p,"principal":users[1]}),
        )?;
        assert!(
            call(
                1,
                json!({"op":"add_member","project":p,"principal":users[2]})
            )
            .is_err()
        );
        call(0, json!({"op":"activate","workspace":"repo","project":p}))?;
        let session = call(0, json!({"op":"start_session","workspace":"repo"}))?["session"]
            .as_str()
            .unwrap()
            .to_owned();
        // Activation removal must not change an already-bound session.
        call(0, json!({"op":"leave","workspace":"repo"}))?;
        assert!(call(0, json!({"op":"start_session","workspace":"repo"})).is_err());
        let capture = json!({"op":"capture","session":session,"request_key":"one","text":"Legacy charge retries require idempotency keys"});
        assert!(call(1, capture.clone()).is_err());
        let id = call(0, capture.clone())?["evidence"]
            .as_str()
            .unwrap()
            .to_owned();
        assert_eq!(call(0, capture.clone())?["duplicate"], true);
        let mut conflict = capture;
        conflict["text"] = json!("changed");
        assert!(call(0, conflict).is_err());
        assert!(call(2, json!({"op":"inspect","evidence":id})).is_err());
        project_search()?;
        let query = json!({"op":"search","project":p,"query":"idempotency"});
        assert_eq!(call(1, query.clone())?["hits"][0]["id"], id);
        assert!(call(2, query.clone()).is_err());
        call(
            0,
            json!({"op":"remove_member","project":p,"principal":users[1]}),
        )?;
        assert!(call(1, query.clone()).is_err());
        call(
            0,
            json!({"op":"add_member","project":p,"principal":users[1]}),
        )?;
        assert!(call(1, json!({"op":"retract","session":session})).is_err());
        call(0, json!({"op":"retract","session":session}))?;
        // Canonical filtering denies stale index hits before projection catches up.
        assert_eq!(call(1, query.clone())?["hits"], json!([]));
        assert!(call(0,json!({"op":"capture","session":session,"request_key":"later","text":"must not resurrect"})).is_err());
        project_search()?;
        assert_eq!(call(1, query)?["hits"], json!([]));
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":id}))?["text"],
            Value::Null
        );
        Ok(())
    })();
    // Attempt cleanup even after a failed assertion returned as an error.
    let mut db = super::database()?;
    for p in projects {
        let rows=db.query("SELECT e.id FROM evidence e JOIN sessions s ON s.id=e.session_id WHERE s.project_id=$1", &[&p])?;
        for r in rows {
            let id: String = r.get(0);
            super::search()?
                .delete(format!(
                    "{}/spindle-evidence-v1/_doc/{id}?refresh=true",
                    super::endpoint()
                ))
                .send()?;
            db.execute("DELETE FROM search_outbox WHERE evidence_id=$1", &[&id])?;
            db.execute("DELETE FROM evidence WHERE id=$1", &[&id])?;
        }
        db.execute("DELETE FROM sessions WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM activations WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM memberships WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM projects WHERE id=$1", &[&p])?;
    }
    for u in users {
        db.execute("DELETE FROM principals WHERE id=$1", &[&u])?;
    }
    result?;
    println!(
        "Product acceptance passed: membership, session binding, idempotent capture, scoped retrieval, stale-index filtering and retraction"
    );
    Ok(())
}
