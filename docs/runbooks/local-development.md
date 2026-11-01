# Local foundation

Install Rust 1.93.0 (rustup), Docker with Compose v2, Git and Make. `rust-toolchain.toml`
pins the compiler; Cargo.lock pins dependencies. Run from the repository root:

```sh
make bootstrap
make up
make check test test-integration
make doctor
make serve
```

The API exposes GET `/healthz` and `/readyz` on 127.0.0.1:8080. These are foundation
endpoints, not the project/memory product API. Readiness checks the applied SQL
migration and OpenSearch. `make down` preserves volumes. Never run `down -v` on
another attempt's data. The smoke test uses a rolled-back SQL transaction and a
unique temporary search index, then deletes its index.

Compose binds PostgreSQL (5432) and OpenSearch (9200) only to loopback. The fixed
PostgreSQL password and disabled OpenSearch security are local-development settings,
not cloud configuration. Do not expose these services publicly. Set SPINDLE_STACK
for a separate Compose project. Separate concurrent stacks also need distinct
SPINDLE_PG_PORT/SPINDLE_SEARCH_PORT and matching DATABASE_URL/OPENSEARCH_URL.

For this managed workspace, Rust was installed outside the repository:

```sh
export CARGO_HOME=/workspace/tooling/cargo
export RUSTUP_HOME=/workspace/tooling/rustup
export PATH=/workspace/tooling/cargo/bin:$PATH
```

Those paths are instance setup, not portable repository requirements. The managed
sandbox may require command escalation for Docker, network or local TCP operations.
Do not disable TLS verification to work around proxy failures.
