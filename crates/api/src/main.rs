mod lineage;
mod mcp;
mod product;
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
    config.options(&format!("-c search_path={}", schema()));
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
fn schema() -> &'static str {
    static SCHEMA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SCHEMA.get_or_init(|| {
        if env::args().nth(1).as_deref() == Some("integration") {
            format!("spindle_test_{}", uuid::Uuid::new_v4().simple())
        } else {
            env::var("SPINDLE_SCHEMA").unwrap_or_else(|_| "public".into())
        }
    })
}
fn migrate() -> Result<()> {
    let mut db = database()?;
    for sql in [
        include_str!("../../../migrations/0001_foundation.sql"),
        include_str!("../../../migrations/0002_product.sql"),
        include_str!("../../../migrations/0003_privacy.sql"),
        include_str!("../../../migrations/0004_lineage.sql"),
        include_str!("../../../migrations/0005_memory_idempotency.sql"),
        include_str!("../../../migrations/0006_corrections.sql"),
    ] {
        db.batch_execute(sql)?;
    }
    Ok(())
}
fn index() -> &'static str {
    static INDEX: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    INDEX.get_or_init(|| {
        if env::args().nth(1).as_deref() == Some("integration") {
            format!("spindle-evidence-test-{}", uuid::Uuid::new_v4())
        } else {
            env::var("SPINDLE_INDEX").unwrap_or_else(|_| "spindle-evidence-v1".into())
        }
    })
}
fn ready() -> Result<()> {
    let row = database()?.query_one(
        "SELECT version FROM spindle_schema_migrations WHERE version = 6",
        &[],
    )?;
    let _: i64 = row.get(0);
    let health: serde_json::Value = search()?
        .get(format!("{}/_cluster/health", endpoint()))
        .send()?
        .error_for_status()?
        .json()?;
    if !matches!(health["status"].as_str(), Some("green" | "yellow")) {
        bail!("OpenSearch is not ready")
    }

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
    if schema().is_empty()
        || schema().len() > 63
        || !schema()
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        bail!("invalid SPINDLE_SCHEMA")
    }

    if !index().starts_with("spindle-evidence-")
        || index().len() > 200
        || !index()
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        bail!("SPINDLE_INDEX must be a lowercase spindle-evidence- index name")
    }
    match env::args().nth(1).as_deref() {
        Some("migrate") => {
            migrate()?;
            println!("migrations 1–6 applied");
        }
        Some("check") => {
            ready()?;
            println!("storage ready");
        }
        Some("integration") => {
            // Unique schema and index isolate all fixtures, outboxes and rebuilds
            // from developer data and concurrent qualification attempts.
            database()?.batch_execute(&format!("CREATE SCHEMA {}", schema()))?;
            let result = (|| -> Result<()> {
                migrate()?;
                ready()?;
                integration()?;
                product::integration()?;
                Ok(())
            })();
            database()?.batch_execute(&format!("DROP SCHEMA {} CASCADE", schema()))?;
            result?;
        }
        Some("mcp") => mcp::serve()?,
        Some("request") => {
            let args: serde_json::Value = serde_json::from_reader(std::io::stdin())?;
            println!("{}", mcp::call(&args)?);
        }
        Some("worker") => loop {
            match product::project_search() {
                Ok(_) => {}
                Err(e) => eprintln!("projection failed; will retry: {e:#}"),
            }
            std::thread::sleep(Duration::from_secs(2));
        },
        Some("provision") => {
            product::provision(&env::var("SPINDLE_PRINCIPAL")?, &env::var("SPINDLE_TOKEN")?)?;
            println!("principal provisioned");
        }
        Some("rebuild-search") => {
            product::rebuild()?;
            println!("canonical search projection rebuilt");
        }
        Some("project") => println!("projected {} sources", product::project_search()?),
        Some("serve") => {
            let address = env::var("SPINDLE_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
            let socket: std::net::SocketAddr = address
                .parse()
                .context("SPINDLE_BIND must be a loopback IP:port")?;
            if !socket.ip().is_loopback() {
                bail!("local profile only supports loopback API bindings")
            }
            let server = tiny_http::Server::http(&address).map_err(|e| anyhow::anyhow!("{e}"))?;
            println!("Spindle foundation listening on {address}");
            for mut request in server.incoming_requests() {
                let (status, body) = match (request.method().as_str(), request.url()) {
                    ("GET", "/healthz") => (200, json!({"status":"alive","phase":"local-product"})),
                    ("GET", "/readyz") => match ready() {
                        Ok(()) => (200, json!({"status":"ready"})),
                        Err(_) => (503, json!({"status":"dependencies_unavailable"})),
                    },
                    ("POST", "/v1") => {
                        let token = request
                            .headers()
                            .iter()
                            .find(|h| h.field.equiv("Authorization"))
                            .and_then(|h| h.value.as_str().strip_prefix("Bearer "))
                            .unwrap_or("")
                            .to_owned();
                        let mut bytes = Vec::new();
                        use std::io::Read;
                        let result = request.as_reader().take(70_001).read_to_end(&mut bytes);
                        if result.is_err() || bytes.len() > 70_000 {
                            (413, json!({"error":"request_too_large"}))
                        } else {
                            match serde_json::from_slice::<product::Operation>(&bytes) {
                                Err(_) => (400, json!({"error":"invalid_request"})),
                                Ok(op) => match product::execute(&token, op) {
                                    Ok(v) => (200, v),
                                    Err(e) => match e.downcast_ref::<product::ApiError>() {
                                        Some(e) => (e.0, json!({"error":e.1})),
                                        None => {
                                            eprintln!("request failed: {e:#}");
                                            (503, json!({"error":"service_unavailable"}))
                                        }
                                    },
                                },
                            }
                        }
                    }
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
        _ => bail!(
            "usage: spindle-api migrate|check|integration|provision|project|worker|request|mcp|serve"
        ),
    }
    Ok(())
}
