use anyhow::{Context, Result, bail};
use postgres::{Client, NoTls};
use serde_json::json;
use std::{env, time::Duration};

fn database() -> Result<Client> {
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://spindle:local-development-only@127.0.0.1:5432/spindle?connect_timeout=5".into()
    });
    let mut config: postgres::Config = url.parse().context("invalid DATABASE_URL")?;
    if config.get_hosts().is_empty() || config.get_hosts().iter().any(|host| !matches!(host, postgres::config::Host::Tcp(h) if h == "localhost" || h.parse::<std::net::IpAddr>().is_ok_and(|ip|ip.is_loopback()))) {
        bail!("foundation profile requires loopback PostgreSQL; remote TLS adapter is not implemented")
    }
    config.connect_timeout(Duration::from_secs(5));
    config
        .connect(NoTls)
        .context("PostgreSQL unavailable (local development profile)")
}
fn search() -> Result<reqwest::blocking::Client> {
    let url = reqwest::Url::parse(&endpoint())?;
    if url.scheme() != "http"
        || !url.host_str().is_some_and(|h| {
            h == "localhost"
                || h.parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        })
    {
        bail!("foundation profile requires loopback OpenSearch")
    }
    Ok(reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()?)
}
fn endpoint() -> String {
    env::var("OPENSEARCH_URL").unwrap_or_else(|_| "http://127.0.0.1:9200".into())
}
fn ready() -> Result<()> {
    let row = database()?.query_one(
        "SELECT version FROM spindle_schema_migrations WHERE version = 1",
        &[],
    )?;
    let _: i64 = row.get(0);
    search()?
        .get(format!("{}/_cluster/health", endpoint()))
        .send()?
        .error_for_status()?;
    Ok(())
}
fn integration() -> Result<()> {
    let mut db = database()?;
    let mut tx = db.transaction()?;
    tx.execute("INSERT INTO context_instances(id,tenant_id,name) VALUES ('00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000002','integration')", &[])?;
    assert_eq!(
        tx.query_one(
            "SELECT name FROM context_instances WHERE id='00000000-0000-0000-0000-000000000001'",
            &[]
        )?
        .get::<_, String>(0),
        "integration"
    );
    tx.rollback()?;
    let c = search()?;
    let index = format!(
        "spindle-smoke-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    let url = format!("{}/{}", endpoint(), index);
    c.put(&url)
        .json(&json!({"mappings":{"properties":{"text":{"type":"text"}}}}))
        .send()?
        .error_for_status()?;
    let result = (|| -> Result<()> {
        c.put(format!("{url}/_doc/1?refresh=true"))
            .json(&json!({"text":"legacy charge retries require idempotency"}))
            .send()?
            .error_for_status()?;
        let v: serde_json::Value = c
            .post(format!("{url}/_search"))
            .json(&json!({"query":{"match":{"text":"idempotency"}}}))
            .send()?
            .error_for_status()?
            .json()?;
        if v["hits"]["total"]["value"] != 1 {
            bail!("search failed to retrieve indexed document")
        }
        Ok(())
    })();
    c.delete(&url).send()?.error_for_status()?;
    result?;
    println!("PostgreSQL transaction and OpenSearch index/write/search/delete passed");
    Ok(())
}
fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("migrate") => {
            database()?.batch_execute(include_str!("../../../migrations/0001_foundation.sql"))?;
            println!("migration 1 applied");
        }
        Some("check") => {
            ready()?;
            println!("storage ready");
        }
        Some("integration") => {
            ready()?;
            integration()?;
        }
        Some("serve") => {
            let address = env::var("SPINDLE_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
            let server = tiny_http::Server::http(&address).map_err(|e| anyhow::anyhow!("{e}"))?;
            println!("Spindle foundation listening on {address}");
            for request in server.incoming_requests() {
                let (status, body) = match (request.method().as_str(), request.url()) {
                    ("GET", "/healthz") => (200, json!({"status":"alive","phase":"foundation"})),
                    ("GET", "/readyz") => match ready() {
                        Ok(()) => (200, json!({"status":"ready"})),
                        Err(_) => (503, json!({"status":"dependencies_unavailable"})),
                    },
                    _ => (404, json!({"error":"not_found"})),
                };
                request.respond(
                    tiny_http::Response::from_string(body.to_string())
                        .with_status_code(status)
                        .with_header(
                            tiny_http::Header::from_bytes("Content-Type", "application/json")
                                .expect("static header"),
                        ),
                )?;
            }
        }
        _ => bail!("usage: spindle-api migrate|check|integration|serve"),
    }
    Ok(())
}
