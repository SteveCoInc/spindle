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
    RescanPrivacy {},
    PrivacyRunStatus {
        run: String,
    },
    CorrectMemory {
        memory: String,
        request_key: String,
        text: String,
    },
    PublishMemory {
        request_key: String,
        project: String,
        text: String,
        inputs: Vec<String>,
    },
    RetractionStatus {
        session: String,
    },
    PersonalPrivacy {
        terms: Vec<String>,
    },
    ProjectPrivacy {
        project: String,
        terms: Vec<String>,
    },
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
pub(crate) fn member(
    db: &mut impl GenericClient,
    project: &str,
    who: &str,
    owner: bool,
) -> Result<()> {
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
        None => super::lineage::inspect(db, id, who),
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
        Operation::RescanPrivacy {} => super::lineage::rescan(&mut tx, &who)?,
        Operation::PrivacyRunStatus { run } => {
            let roots: Vec<String> = tx
                .query_opt(
                    "SELECT roots FROM privacy_runs WHERE id=$1 AND principal_id=$2",
                    &[&run, &who],
                )?
                .ok_or(ApiError(403, "forbidden"))?
                .get(0);
            super::lineage::reconcile_roots(&mut tx, roots)?
        }

        Operation::CorrectMemory {
            memory,
            request_key,
            text,
        } => super::lineage::correct(&mut tx, &memory, &who, &request_key, &text)?,
        Operation::PublishMemory {
            request_key,
            project,
            text,
            inputs,
        } => super::lineage::publish(&mut tx, &project, &who, &request_key, &text, &inputs)?,
        Operation::RetractionStatus { session } => {
            super::lineage::reconcile(&mut tx, &session, &who)?
        }
        Operation::PersonalPrivacy { terms } => {
            validate_terms(&terms)?;
            let version:i64=tx.query_one("INSERT INTO personal_exclusions(principal_id,terms) VALUES($1,$2) ON CONFLICT(principal_id) DO UPDATE SET terms=excluded.terms,version=personal_exclusions.version+1 RETURNING version", &[&who,&terms])?.get(0);
            json!({"version":version,"scope":"future_captures","historical_rescan":false})
        }
        Operation::ProjectPrivacy { project, terms } => {
            member(&mut tx, &project, &who, true)?;
            validate_terms(&terms)?;
            let version:i64=tx.query_one("INSERT INTO project_exclusions(project_id,terms) VALUES($1,$2) ON CONFLICT(project_id) DO UPDATE SET terms=excluded.terms,version=project_exclusions.version+1 RETURNING version", &[&project,&terms])?.get(0);
            json!({"version":version,"scope":"future_captures","historical_rescan":false})
        }

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
            let (personal_version, project_version) =
                check_privacy(&mut tx, &project, &who, &text)?;
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
                    "INSERT INTO evidence(id,session_id,request_key,text,personal_policy_version,project_policy_version) VALUES($1,$2,$3,$4,$5,$6)",
                    &[&id, &session, &request_key, &text, &personal_version, &project_version],
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
            let response:Value=super::search()?.post(format!("{}/{}/_search",super::endpoint(),super::index()))
                .json(&json!({"size":100,"_source":false,"query":{"bool":{"must":[{"match":{"text":query}}],"filter":[{"term":{"project":project}}]}}}))
                .send()?.error_for_status()?.json()?;
            let mut hits = Vec::new();
            for hit in response["hits"]["hits"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("invalid search response"))?
            {
                if let Some(id) = hit["_id"].as_str() {
                    let v = match source(&mut tx, id, &who) {
                        Ok(value) => value,
                        Err(e) if e.downcast_ref::<ApiError>().is_some_and(|e| e.0 == 403) => {
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    if v["project"] == project && v["retracted"] == false {
                        hits.push(v)
                    }
                }
            }
            json!({"hits":hits,"mode":"lexical","limit":100})
        }
        Operation::Retract { session } => {
            let _project: String = tx
                .query_opt(
                    "SELECT project_id FROM sessions WHERE id=$1 AND principal_id=$2",
                    &[&session, &who],
                )?
                .ok_or(ApiError(403, "forbidden"))?
                .get(0);

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
            let roots = rows.iter().map(|r| r.get::<_, String>(0)).collect();
            let retired = super::lineage::invalidate(&mut tx, roots)?;
            json!({"status":"projection_pending","sources_retracted":rows.len(),"memories_retired":retired})
        }
    };
    tx.commit()?;
    Ok(result)
}

pub fn project_search() -> Result<usize> {
    let c = super::search()?;
    let url = format!("{}/{}", super::endpoint(), super::index());
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
    Ok(ids.len() + super::lineage::project()?)
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
            .env("SPINDLE_INDEX", super::index())
            .env("SPINDLE_SCHEMA", super::schema())
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
        // Exercise the same stdio subprocess transport used by both coding hosts.
        let mut mcp = std::process::Command::new(std::env::current_exe()?)
            .arg("mcp")
            .env("SPINDLE_TOKEN", &tokens[0])
            .env("SPINDLE_URL", format!("{base}/v1"))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        {
            use std::io::Write;
            let mut input = mcp.stdin.take().unwrap();
            writeln!(
                input,
                "{}",
                json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}})
            )?;
            writeln!(
                input,
                "{}",
                json!({"jsonrpc":"2.0","method":"notifications/initialized"})
            )?;
            writeln!(
                input,
                "{}",
                json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"spindle_status","arguments":{"workspace":"repo"}}})
            )?;
        }
        let output = mcp.wait_with_output()?;
        assert!(output.status.success());
        let wire = String::from_utf8(output.stdout)?;
        let responses: Vec<Value> = wire
            .lines()
            .map(serde_json::from_str)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[1]["result"]["isError"], false);
        let status: Value = serde_json::from_str(
            responses[1]["result"]["content"][0]["text"]
                .as_str()
                .unwrap(),
        )?;
        assert_eq!(status["project"], p);
        // Activation removal must not change an already-bound session.
        call(0, json!({"op":"leave","workspace":"repo"}))?;
        assert!(call(0, json!({"op":"start_session","workspace":"repo"})).is_err());
        call(
            0,
            json!({"op":"personal_privacy","terms":["private-value"]}),
        )?;
        call(
            0,
            json!({"op":"project_privacy","project":p,"terms":["customer-key"]}),
        )?;
        assert!(call(1, json!({"op":"project_privacy","project":p,"terms":[]})).is_err());
        for text in ["Never share PRIVATE-VALUE", "never share Customer-Key"] {
            assert!(
                call(
                    0,
                    json!({"op":"capture","session":session,"request_key":"secret","text":text})
                )
                .is_err()
            );
        }
        // A project owner cannot relax the contributor's personal policy.
        call(0, json!({"op":"project_privacy","project":p,"terms":[]}))?;
        assert!(call(0,json!({"op":"capture","session":session,"request_key":"secret","text":"private-value"})).is_err());
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
        let independent_session =
            call(0, json!({"op":"activate","workspace":"other","project":p}))?;
        assert_eq!(independent_session["project"], p);
        let independent_session =
            call(0, json!({"op":"start_session","workspace":"other"}))?["session"]
                .as_str()
                .unwrap()
                .to_owned();
        let independent=call(0,json!({"op":"capture","session":independent_session,"request_key":"independent","text":"Unrelated retention guidance"}))?["evidence"].as_str().unwrap().to_owned();
        let independent_memory=call(0,json!({"op":"publish_memory","request_key":uuid::Uuid::new_v4().to_string(),"project":p,"text":"Retention policy remains supported","inputs":[independent]}))?["memory"].as_str().unwrap().to_owned();
        let mut parent = id.clone();
        for depth in 0..30 {
            parent=call(0,json!({"op":"publish_memory","request_key":uuid::Uuid::new_v4().to_string(),"project":p,"text":format!("Derived charge policy level {depth}"),"inputs":[parent]}))?["memory"].as_str().unwrap().to_owned();
        }
        let shared=call(0,json!({"op":"publish_memory","request_key":uuid::Uuid::new_v4().to_string(),"project":p,"text":"Combined policy","inputs":[parent,independent_memory]}))?["memory"].as_str().unwrap().to_owned();
        assert!(call(2, json!({"op":"inspect","evidence":shared})).is_err());
        project_search()?;
        let query = json!({"op":"search","project":p,"query":"idempotency"});
        assert_eq!(call(1, query.clone())?["hits"][0]["id"], id);
        assert!(call(2, query.clone()).is_err());
        call(
            1,
            json!({"op":"activate","workspace":"member-repo","project":p}),
        )?;
        let member_session =
            call(1, json!({"op":"start_session","workspace":"member-repo"}))?["session"]
                .as_str()
                .unwrap()
                .to_owned();
        call(
            1,
            json!({"op":"capture","session":member_session,"request_key":"allowed","text":"Member-authored note"}),
        )?;
        call(
            0,
            json!({"op":"remove_member","project":p,"principal":users[1]}),
        )?;
        assert!(call(1,json!({"op":"capture","session":member_session,"request_key":"revoked","text":"No longer authorized"})).is_err());
        assert!(call(1, query.clone()).is_err());
        call(
            1,
            json!({"op":"personal_privacy","terms":["Member-authored"]}),
        )?;
        let revoked_remediation = call(1, json!({"op":"rescan_privacy"}))?;
        assert_eq!(revoked_remediation["affected_artifacts"], 1);
        project_search()?;
        assert_eq!(
            call(
                1,
                json!({"op":"privacy_run_status","run":revoked_remediation["run"]})
            )?["status"],
            "complete"
        );
        call(
            0,
            json!({"op":"add_member","project":p,"principal":users[1]}),
        )?;
        assert!(call(1, json!({"op":"retract","session":session})).is_err());
        let rollback = call(0, json!({"op":"retract","session":session}))?;
        assert_eq!(rollback["memories_retired"], 31);
        assert_eq!(
            call(0, json!({"op":"retraction_status","session":session}))?["status"],
            "projection_pending"
        );
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":shared}))?["retracted"],
            true
        );
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":independent_memory}))?["retracted"],
            false
        );
        assert!(call(0,json!({"op":"publish_memory","request_key":uuid::Uuid::new_v4().to_string(),"project":p,"text":"stale synthesis","inputs":[parent]})).is_err());
        // Canonical filtering denies stale index hits before projection catches up.
        assert_eq!(call(1, query.clone())?["hits"], json!([]));
        assert!(call(0,json!({"op":"capture","session":session,"request_key":"later","text":"must not resurrect"})).is_err());
        project_search()?;
        assert_eq!(call(1, query)?["hits"], json!([]));
        assert_eq!(
            call(0, json!({"op":"retraction_status","session":session}))?["status"],
            "complete"
        );
        let correction = json!({"op":"correct_memory","memory":independent_memory,"request_key":"fix-retention","text":"Corrected retention policy"});
        assert!(call(1, correction.clone()).is_err());
        let corrected = call(0, correction.clone())?["memory"]
            .as_str()
            .unwrap()
            .to_owned();
        assert_eq!(call(0, correction)?["memory"], corrected);
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":independent_memory}))?["retracted"],
            true
        );
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":corrected}))?["retracted"],
            false
        );
        project_search()?;
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":id}))?["text"],
            Value::Null
        );
        // The integration command owns a unique index, so loss/rebuild does not
        // delete developer data or race another run's search collection.
        super::search()?
            .delete(format!("{}/{}", super::endpoint(), super::index()))
            .send()?
            .error_for_status()?;
        rebuild()?;
        let restored = call(
            1,
            json!({"op":"search","project":p,"query":"Corrected retention"}),
        )?;
        assert!(
            restored["hits"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["id"] == corrected)
        );
        assert_eq!(
            call(1, json!({"op":"search","project":p,"query":"idempotency"}))?["hits"],
            json!([])
        );
        call(
            0,
            json!({"op":"activate","workspace":"privacy","project":p}),
        )?;
        let privacy_session =
            call(0, json!({"op":"start_session","workspace":"privacy"}))?["session"]
                .as_str()
                .unwrap()
                .to_owned();
        let secret=call(0,json!({"op":"capture","session":privacy_session,"request_key":"prior-policy","text":"late-secret-marker"}))?["evidence"].as_str().unwrap().to_owned();
        let paraphrase=call(0,json!({"op":"publish_memory","request_key":"paraphrase","project":p,"text":"A paraphrase without the literal marker","inputs":[secret]}))?["memory"].as_str().unwrap().to_owned();
        let replacement=call(0,json!({"op":"correct_memory","request_key":"paraphrase-fix","memory":paraphrase,"text":"Corrected paraphrase without the literal marker"}))?["memory"].as_str().unwrap().to_owned();
        project_search()?;
        call(
            0,
            json!({"op":"personal_privacy","terms":["private-value","late-secret-marker"]}),
        )?;
        let remediation = call(0, json!({"op":"rescan_privacy"}))?;
        assert_eq!(remediation["affected_artifacts"], 3);
        let run = remediation["run"].as_str().unwrap();
        assert!(call(1, json!({"op":"privacy_run_status","run":run})).is_err());
        assert_eq!(
            call(0, json!({"op":"privacy_run_status","run":run}))?["status"],
            "projection_pending"
        );
        assert_eq!(
            call(1, json!({"op":"inspect","evidence":replacement}))?["retracted"],
            true
        );
        let mut verify = super::database()?;
        assert_eq!(
            verify
                .query_one("SELECT text FROM evidence WHERE id=$1", &[&secret])?
                .get::<_, String>(0),
            ""
        );
        for id in [paraphrase, replacement] {
            assert_eq!(
                verify
                    .query_one("SELECT text FROM memory_versions WHERE id=$1", &[&id])?
                    .get::<_, String>(0),
                ""
            );
        }
        project_search()?;
        assert_eq!(
            call(0, json!({"op":"privacy_run_status","run":run}))?["status"],
            "complete"
        );
        // Exercise independent database connections; the HTTP server itself is
        // serial today, but the future worker/API boundary must also be race-safe.
        call(0, json!({"op":"activate","workspace":"race","project":p}))?;
        let race_session = call(0, json!({"op":"start_session","workspace":"race"}))?["session"]
            .as_str()
            .unwrap()
            .to_owned();
        let race_source=call(0,json!({"op":"capture","session":race_session,"request_key":"race","text":"Race source"}))?["evidence"].as_str().unwrap().to_owned();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let token = tokens[0].clone();
        let guard = barrier.clone();
        let project = p.clone();
        let publisher = std::thread::spawn(move || {
            guard.wait();
            execute(
                &token,
                Operation::PublishMemory {
                    request_key: "race-memory".into(),
                    project,
                    text: "Racing synthesis".into(),
                    inputs: vec![race_source],
                },
            )
        });
        let token = tokens[0].clone();
        let retracter = std::thread::spawn(move || {
            barrier.wait();
            execute(
                &token,
                Operation::Retract {
                    session: race_session,
                },
            )
        });
        let published = publisher.join().expect("publisher panicked");
        retracter.join().expect("retracter panicked")?;
        if let Err(ref error) = published {
            anyhow::ensure!(
                error.downcast_ref::<ApiError>().is_some_and(|e| e.0 == 409),
                "unexpected publication failure: {error:#}"
            );
        }
        if let Ok(value) = published {
            let memory = value["memory"].as_str().unwrap();
            assert_eq!(
                call(0, json!({"op":"inspect","evidence":memory}))?["retracted"],
                true
            );
        }
        project_search()?;
        Ok(())
    })();
    // Cleanup follows returned errors; failed assertions retain diagnostic state.
    let mut db = super::database()?;
    for p in projects {
        for r in db.query("SELECT id FROM memory_versions WHERE project_id=$1", &[&p])? {
            let id: String = r.get(0);
            super::search()?
                .delete(format!(
                    "{}/{}/_doc/{id}?refresh=true",
                    super::endpoint(),
                    super::index()
                ))
                .send()?;
        }
        db.execute("DELETE FROM memory_outbox WHERE memory_id IN (SELECT id FROM memory_versions WHERE project_id=$1)", &[&p])?;
        db.execute("DELETE FROM memory_inputs WHERE memory_id IN (SELECT id FROM memory_versions WHERE project_id=$1)", &[&p])?;
        db.execute("DELETE FROM memory_versions WHERE project_id=$1", &[&p])?;
        let rows=db.query("SELECT e.id FROM evidence e JOIN sessions s ON s.id=e.session_id WHERE s.project_id=$1", &[&p])?;
        for r in rows {
            let id: String = r.get(0);
            super::search()?
                .delete(format!(
                    "{}/{}/_doc/{id}?refresh=true",
                    super::endpoint(),
                    super::index()
                ))
                .send()?;
            db.execute("DELETE FROM search_outbox WHERE evidence_id=$1", &[&id])?;
            db.execute("DELETE FROM evidence WHERE id=$1", &[&id])?;
        }
        db.execute("DELETE FROM sessions WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM activations WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM memberships WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM project_exclusions WHERE project_id=$1", &[&p])?;
        db.execute("DELETE FROM projects WHERE id=$1", &[&p])?;
    }
    for u in users {
        db.execute(
            "DELETE FROM personal_exclusions WHERE principal_id=$1",
            &[&u],
        )?;
        db.execute("DELETE FROM privacy_runs WHERE principal_id=$1", &[&u])?;
        db.execute("DELETE FROM principals WHERE id=$1", &[&u])?;
    }
    result?;
    println!(
        "Product acceptance passed: membership, session binding, idempotent capture, scoped retrieval, stale-index filtering and retraction"
    );
    Ok(())
}

fn validate_terms(terms: &[String]) -> Result<()> {
    if terms.len() > 100 || terms.iter().any(|t| !text_ok(t, 200)) {
        return invalid();
    }
    Ok(())
}
fn excluded(text: &str, terms: &[String]) -> bool {
    let text = text.to_lowercase();
    terms.iter().any(|term| text.contains(&term.to_lowercase()))
}
pub(crate) fn check_privacy(
    db: &mut impl GenericClient,
    project: &str,
    who: &str,
    text: &str,
) -> Result<(i64, i64)> {
    let personal = db.query_opt(
        "SELECT terms,version FROM personal_exclusions WHERE principal_id=$1",
        &[&who],
    )?;
    let project_policy = db.query_opt(
        "SELECT terms,version FROM project_exclusions WHERE project_id=$1",
        &[&project],
    )?;
    let personal_version = personal.as_ref().map_or(0, |r| r.get::<_, i64>(1));
    let project_version = project_policy.as_ref().map_or(0, |r| r.get::<_, i64>(1));
    let mut terms = Vec::<String>::new();
    for row in [personal, project_policy].into_iter().flatten() {
        terms.extend(row.get::<_, Vec<String>>(0));
    }
    if excluded(text, &terms) {
        return Err(ApiError(422, "privacy_exclusion").into());
    }
    Ok((personal_version, project_version))
}

pub fn rebuild() -> Result<()> {
    let mut db = super::database()?;
    let mut tx = db.transaction()?;
    tx.batch_execute("LOCK TABLE memberships IN SHARE ROW EXCLUSIVE MODE")?;
    tx.execute(
        "INSERT INTO search_outbox SELECT id FROM evidence ON CONFLICT DO NOTHING",
        &[],
    )?;
    tx.execute(
        "INSERT INTO memory_outbox SELECT id FROM memory_versions ON CONFLICT DO NOTHING",
        &[],
    )?;
    tx.commit()?;
    while project_search()? > 0 {}
    Ok(())
}

#[cfg(test)]
mod privacy_tests {
    use super::*;
    #[test]
    fn exclusions_are_literal_case_insensitive_and_validated() {
        assert!(excluded("token SECRET-key", &["secret-KEY".into()]));
        assert!(!excluded("public note", &["secret".into()]));
        assert!(validate_terms(&["".into()]).is_err());
        assert!(validate_terms(&vec!["term".into(); 101]).is_err());
    }
}
