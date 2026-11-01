# First local product slice

This slice shares explicitly supplied original evidence. It does not yet extract
or synthesize memories from transcripts. Search is lexical OpenSearch retrieval;
semantic embeddings, derived lineage, privacy rescans and full derived-memory
rollback remain unfinished. The Software blueprint is an identity/version marker;
its extraction policies are not implemented yet.

Start `make up`, `make serve` and, in another terminal, `make worker`. The worker
retries durable projection outbox entries every two seconds. `make project` drains
one batch of at most 1,000 entries. Acknowledged evidence survives a worker crash.

Provision a user with a high-entropy token from your password manager (at least
32 characters). In a local Bash terminal, these prompts avoid command history:

```sh
read -r -p 'Principal ID: ' SPINDLE_PRINCIPAL
read -r -s -p 'Token: ' SPINDLE_TOKEN
export SPINDLE_PRINCIPAL SPINDLE_TOKEN
cargo run --locked -p spindle-api -- provision
```

The database stores a SHA-256 token digest. Provision separate principals for each
user; do not share tokens. This is a development authentication mechanism, not
production identity/OIDC or a token-rotation system. Database access is trusted
administration. Service and dependencies must stay on loopback; no remote TLS
profile has been qualified.

Send JSON operations on stdin (or use the [MCP tools](../../integrations/README.md)):

```sh
printf '%s\n' '{"op":"create_project","name":"Payments"}' | cargo run --quiet --locked -p spindle-api -- request
```

Using the returned project ID, invoke `add_member` with `project`/`principal`,
`activate` with `project`/`workspace`, then `start_session` with `workspace`. Capture
with `session`, a stable `request_key`, and `text`. Repeating the same key and text
returns the same evidence ID; changed payloads conflict. Search with `project` and
`query`; inspect with `evidence`. All operations require the bearer token.

`leave` removes future activation; existing sessions retain their original binding.
Membership revocation immediately prevents reads and contributions, including
through existing sessions. Only the session author can capture or retract it.
`retract` closes the session and marks its evidence inactive in one transaction.
Search rechecks PostgreSQL, so stale index hits are excluded immediately. The
response reports `projection_pending` until the worker removes the index entries;
this is source-only retraction, not a completed full derived-memory rollback job.
Retraction retains canonical historical source text for now; it is not a purge.

The initial consistency guard serializes membership and publication changes,
including search and projection network calls. This favors correctness for a
single-machine prototype; throughput and bounded scoped guards need qualification
before increasing concurrency. The search endpoint returns up to 100 candidates.

`make test-integration` runs actual HTTP calls with separate temporary principals,
real PostgreSQL and real OpenSearch. It covers unauthorized access, invitation
roles, session binding, duplicate/conflicting capture, cross-user capture denial,
member revocation, stale-index filtering and source retraction.
