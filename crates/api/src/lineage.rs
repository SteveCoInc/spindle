//! Versioned transformations with one conjunctive input group. Future reasoners
//! publish through the same canonical validation boundary, never directly to search.
use super::product::{ApiError, member};
use anyhow::{Result, bail};
use postgres::GenericClient;
use serde_json::{Value, json};
use std::collections::{BTreeSet, VecDeque};

pub fn inspect(db: &mut impl GenericClient, id: &str, who: &str) -> Result<Value> {
    let r=db.query_opt("SELECT v.project_id,v.author,v.text,v.active,v.transformation,v.recorded_at::text FROM memory_versions v JOIN memberships m ON m.project_id=v.project_id AND m.principal_id=$2 WHERE v.id=$1", &[&id,&who])?.ok_or(ApiError(403,"forbidden"))?;
    let inputs: Vec<String> = db
        .query(
            "SELECT input_id FROM memory_inputs WHERE memory_id=$1 ORDER BY input_id",
            &[&id],
        )?
        .into_iter()
        .map(|r| r.get(0))
        .collect();
    Ok(
        json!({"id":id,"project":r.get::<_,String>(0),"author":r.get::<_,String>(1),"text":if r.get::<_,bool>(3){json!(r.get::<_,String>(2))}else{Value::Null},"retracted":!r.get::<_,bool>(3),"kind":"derived_memory","transformation":r.get::<_,String>(4),"recorded_at":r.get::<_,String>(5),"inputs":inputs,"support":"all_inputs_required"}),
    )
}
pub fn publish(
    db: &mut impl GenericClient,
    project: &str,
    who: &str,
    request_key: &str,
    text: &str,
    inputs: &[String],
) -> Result<Value> {
    member(db, project, who, false)?;
    if request_key.trim().is_empty()
        || request_key.len() > 200
        || text.trim().is_empty()
        || text.len() > 65536
        || inputs.is_empty()
        || inputs.len() > 100
    {
        return Err(ApiError(400, "invalid_request").into());
    }
    super::product::check_privacy(db, project, who, text)?;
    if let Some(row)=db.query_opt("SELECT id,text,active FROM memory_versions WHERE project_id=$1 AND author=$2 AND request_key=$3", &[&project,&who,&request_key])? {
        let old:String=row.get(0);
        let old_inputs:BTreeSet<String>=db.query("SELECT input_id FROM memory_inputs WHERE memory_id=$1", &[&old])?.into_iter().map(|r|r.get(0)).collect();
        if row.get::<_,String>(1)!=text || !row.get::<_,bool>(2) || old_inputs!=inputs.iter().cloned().collect(){return Err(ApiError(409,"idempotency_conflict").into())}
        return Ok(json!({"memory":old,"duplicate":true}));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let mut parents = Vec::new();
    // New immutable IDs can only refer to already-existing versions: cycles
    // cannot be introduced by a concurrent update or a client-supplied ID.
    for input in inputs.iter().collect::<BTreeSet<_>>() {
        if db.query_opt("SELECT e.id FROM evidence e JOIN sessions s ON s.id=e.session_id WHERE e.id=$1 AND s.project_id=$2 AND NOT e.retracted", &[&input,&project])?.is_some(){
            parents.push((input,true));
        }else if db.query_opt("SELECT id FROM memory_versions WHERE id=$1 AND project_id=$2 AND active", &[&input,&project])?.is_some(){
            parents.push((input,false));
        }else{return Err(ApiError(409,"invalid_or_inactive_input").into())}
    }
    db.execute("INSERT INTO memory_versions(id,project_id,author,text,transformation,request_key) VALUES($1,$2,$3,$4,'manual_synthesis',$5)", &[&id,&project,&who,&text,&request_key])?;
    for (input, evidence) in parents {
        let source = if evidence { Some(input.as_str()) } else { None };
        let parent = if evidence { None } else { Some(input.as_str()) };
        db.execute("INSERT INTO memory_inputs(memory_id,input_id,evidence_id,parent_memory_id) VALUES($1,$2,$3,$4)", &[&id,&input,&source,&parent])?;
    }
    db.execute("INSERT INTO memory_outbox VALUES($1)", &[&id])?;
    Ok(json!({"memory":id,"transformation":"manual_synthesis","projection":"pending"}))
}
/// Retire the transitive closure under the caller's authority transaction guard.
/// Rust traversal avoids recursive SQL; the whole operation is atomic in M1.
pub fn invalidate(db: &mut impl GenericClient, roots: Vec<String>) -> Result<usize> {
    let mut queue: VecDeque<String> = roots.into();
    let mut seen = BTreeSet::new();
    let mut retired = 0;
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let children = db.query(
            "SELECT memory_id FROM memory_inputs WHERE input_id=$1",
            &[&id],
        )?;
        for child in children {
            let child: String = child.get(0);
            retired += db.execute(
                "UPDATE memory_versions SET active=FALSE WHERE id=$1 AND active",
                &[&child],
            )? as usize;
            db.execute(
                "INSERT INTO memory_outbox VALUES($1) ON CONFLICT DO NOTHING",
                &[&child],
            )?;
            queue.push_back(child);
        }
    }
    Ok(retired)
}
pub fn project() -> Result<usize> {
    let mut db = super::database()?;
    let rows = db.query(
        "SELECT memory_id FROM memory_outbox ORDER BY memory_id LIMIT 1000",
        &[],
    )?;
    let c = super::search()?;
    let url = format!("{}/{}", super::endpoint(), super::index());
    for row in &rows {
        let id: String = row.get(0);
        let mut tx = db.transaction()?;
        tx.batch_execute("LOCK TABLE memberships IN SHARE ROW EXCLUSIVE MODE")?;
        let r = tx.query_one(
            "SELECT project_id,text,active FROM memory_versions WHERE id=$1",
            &[&id],
        )?;
        if r.get::<_, bool>(2) {
            c.put(format!("{url}/_doc/{id}?refresh=true"))
                .json(&json!({"project":r.get::<_,String>(0),"text":r.get::<_,String>(1)}))
                .send()?
                .error_for_status()?;
        } else {
            let response = c.delete(format!("{url}/_doc/{id}?refresh=true")).send()?;
            if response.status().as_u16() != 404 {
                response.error_for_status()?;
            }
        }
        tx.execute("DELETE FROM memory_outbox WHERE memory_id=$1", &[&id])?;
        tx.commit()?;
    }
    Ok(rows.len())
}
pub fn reconcile(db: &mut impl GenericClient, session: &str, who: &str) -> Result<Value> {
    let row = db
        .query_opt(
            "SELECT project_id,retracted FROM sessions WHERE id=$1 AND principal_id=$2",
            &[&session, &who],
        )?
        .ok_or(ApiError(403, "forbidden"))?;
    let _project: String = row.get(0);
    if !row.get::<_, bool>(1) {
        bail!(ApiError(409, "session_not_retracted"))
    }
    let roots: Vec<String> = db
        .query("SELECT id FROM evidence WHERE session_id=$1", &[&session])?
        .into_iter()
        .map(|r| r.get(0))
        .collect();
    reconcile_roots(db, roots)
}
pub fn reconcile_roots(db: &mut impl GenericClient, roots: Vec<String>) -> Result<Value> {
    let mut queue: VecDeque<String> = roots.into();
    let mut seen = BTreeSet::new();
    let mut pending = 0;
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let count:i64=db.query_one("SELECT (SELECT count(*) FROM search_outbox WHERE evidence_id=$1)+(SELECT count(*) FROM memory_outbox WHERE memory_id=$1)", &[&id])?.get(0);
        pending += count;
        for r in db.query(
            "SELECT memory_id FROM memory_inputs WHERE input_id=$1",
            &[&id],
        )? {
            queue.push_back(r.get(0));
        }
    }
    Ok(
        json!({"status":if pending==0{"complete"}else{"projection_pending"},"pending_projections":pending,"affected_artifacts":seen.len(),"historical_content_purged":false}),
    )
}

/// An author correction is a new version using the old version's validated inputs.
/// Supersession is history, not a derivation edge: retired prose is never input.
pub fn correct(
    db: &mut impl GenericClient,
    id: &str,
    who: &str,
    key: &str,
    text: &str,
) -> Result<Value> {
    let old = db
        .query_opt(
            "SELECT project_id,author,active FROM memory_versions WHERE id=$1",
            &[&id],
        )?
        .ok_or(ApiError(403, "forbidden"))?;
    let project: String = old.get(0);
    member(db, &project, who, false)?;
    if old.get::<_, String>(1) != who {
        return Err(ApiError(403, "forbidden").into());
    }
    if let Some(row)=db.query_opt("SELECT id,text,active,supersedes FROM memory_versions WHERE project_id=$1 AND author=$2 AND request_key=$3", &[&project,&who,&key])? {
        if row.get::<_,String>(1)==text && row.get::<_,bool>(2) && row.get::<_,Option<String>>(3).as_deref()==Some(id){return Ok(json!({"memory":row.get::<_,String>(0),"duplicate":true}))}
        return Err(ApiError(409,"idempotency_conflict").into())
    }
    if !old.get::<_, bool>(2) {
        return Err(ApiError(409, "inactive_memory").into());
    }
    let inputs: Vec<String> = db
        .query(
            "SELECT input_id FROM memory_inputs WHERE memory_id=$1 ORDER BY input_id",
            &[&id],
        )?
        .into_iter()
        .map(|r| r.get(0))
        .collect();
    let mut result = publish(db, &project, who, key, text, &inputs)?;
    let new = result["memory"].as_str().unwrap();
    db.execute(
        "UPDATE memory_versions SET supersedes=$1 WHERE id=$2",
        &[&id, &new],
    )?;
    db.execute(
        "UPDATE memory_versions SET active=FALSE WHERE id=$1",
        &[&id],
    )?;
    db.execute(
        "INSERT INTO memory_outbox VALUES($1) ON CONFLICT DO NOTHING",
        &[&id],
    )?;
    result["descendants_retired"] = json!(invalidate(db, vec![id.to_owned()])?);
    result["supersedes"] = json!(id);
    Ok(result)
}

/// Explicit personal-policy remediation, including contributions in projects the
/// author has left. Metadata remains; active content and derived prose are purged.
pub fn rescan(db: &mut impl GenericClient, who: &str) -> Result<Value> {
    let mut roots = Vec::<String>::new();
    let rows=db.query("SELECT e.id,e.text,s.project_id FROM evidence e JOIN sessions s ON s.id=e.session_id WHERE s.principal_id=$1 UNION ALL SELECT id,text,project_id FROM memory_versions WHERE author=$1", &[&who])?;
    for row in rows {
        let id: String = row.get(0);
        let text: String = row.get(1);
        let project: String = row.get(2);
        match super::product::check_privacy(db, &project, who, &text) {
            Ok(_) => {}
            Err(e) if e.downcast_ref::<ApiError>().is_some_and(|e| e.0 == 422) => roots.push(id),
            Err(e) => return Err(e),
        }
    }
    let mut queue: VecDeque<String> = roots.clone().into();
    let mut seen = BTreeSet::new();
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if db.execute(
            "UPDATE evidence SET retracted=TRUE,text='' WHERE id=$1",
            &[&id],
        )? > 0
        {
            db.execute(
                "INSERT INTO search_outbox VALUES($1) ON CONFLICT DO NOTHING",
                &[&id],
            )?;
        } else {
            db.execute(
                "UPDATE memory_versions SET active=FALSE,text='' WHERE id=$1",
                &[&id],
            )?;
            db.execute(
                "INSERT INTO memory_outbox VALUES($1) ON CONFLICT DO NOTHING",
                &[&id],
            )?;
        }
        for row in db.query(
            "SELECT memory_id FROM memory_inputs WHERE input_id=$1",
            &[&id],
        )? {
            queue.push_back(row.get(0));
        }
        // A corrected replacement uses the same old inputs but may paraphrase
        // private text. Include supersession history in privacy remediation.
        for row in db.query("SELECT id FROM memory_versions WHERE supersedes=$1", &[&id])? {
            queue.push_back(row.get(0));
        }
    }
    let id = uuid::Uuid::new_v4().to_string();
    // Save the entire affected set so status includes supersession-only links.
    let affected: Vec<String> = seen.into_iter().collect();
    db.execute(
        "INSERT INTO privacy_runs(id,principal_id,roots) VALUES($1,$2,$3)",
        &[&id, &who, &affected],
    )?;
    Ok(
        json!({"run":id,"status":if affected.is_empty(){"complete"}else{"projection_pending"},"affected_artifacts":affected.len(),"canonical_content_purged":true,"backup_erasure_verified":false}),
    )
}
