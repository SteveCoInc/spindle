# First local product slice

This slice shares explicitly supplied original evidence and manually authored
synthesis with immutable lineage. Automatic LLM extraction/synthesis, semantic
embeddings, resumable large rollback jobs remain
unfinished. Search uses lexical OpenSearch retrieval. The Software blueprint is an identity/version marker;
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
use `retraction_status` with the session ID to check completion across source and
transitive derived-memory projections. The current synchronous closure has no
resumable batching or bounded/semantic rollback mode.
Retraction retains canonical historical source text for now; it is not a purge.

The initial consistency guard serializes membership and publication changes,
including search and projection network calls. This favors correctness for a
single-machine prototype; throughput and bounded scoped guards need qualification
before increasing concurrency. The search endpoint returns up to 100 candidates.

`make test-integration` runs actual HTTP calls with separate temporary principals,
real PostgreSQL and real OpenSearch. It covers unauthorized access, invitation
roles, session binding, duplicate/conflicting capture, cross-user capture denial,
member revocation, stale-index filtering and source retraction.

## Privacy and synthesis

`personal_privacy` takes `terms`, a list of case-insensitive literal exclusions.
`project_privacy` takes `project` and `terms` and requires the project owner.
Both apply before capture and synthesis storage; project policy cannot relax a
personal exclusion. Matching contributions are rejected whole with HTTP 422.
Policies are versioned; source evidence records applied versions. Changes apply
only to future submissions. These are literal filters, not a complete secret/PII
classifier or Unicode-confusable defense.

`publish_memory` takes `project`, `request_key`, `text`, and `inputs` (evidence or
memory IDs). Every input must be active and belong to that project. All inputs
are required; alternate OR justifications are deferred. The transformation is
explicitly labeled `manual_synthesis`. Provenance does not prove the author's
text follows logically from its inputs. Publication keys make retries idempotent.

`correct_memory` takes `memory`, `request_key`, and `text`. Only the original
author can correct it. A new immutable version uses the original version's inputs;
the previous version and its dependents retire atomically. Old prose is never
fed back as an input. Retraction walks reverse lineage, including shared
descendants, while leaving unrelated independently supported material active.
Formal reasoning can later publish validated outputs through this boundary.

The acceptance suite also exercises the actual MCP subprocess → HTTP transport,
privacy precedence, a 30-hop dependency chain, shared descendants, correction
ownership and retry, and projection completion. It does not establish real-host
hook integration or automatic model quality.

## Managed environment restoration

If a restored OpenSearch volume fails with Lucene `Underlying file changed by an
external force` lock errors, preserve it. The current development session uses
`SPINDLE_STACK=spindle-current` with fresh volumes; old `spindle-dev` volumes remain.
Set that variable before Make lifecycle commands in this session. This workaround
is not a validated backup/restore procedure and does not silently recover old data.

## Projection recovery and test isolation

`make rebuild-search` requeues canonical sources and memory versions and drains the
outboxes. It reconstructs active documents and deletes retired ones. Model calls
are not needed. Use it after restoring PostgreSQL into a fresh OpenSearch instance.
It rebuilds the configured index; backup consistency and production restore remain
separate qualification work.

The integration command creates its own PostgreSQL schema and OpenSearch index,
passes those namespaces to its HTTP subprocess, and removes them after success.
Its outbox drains and index-loss tests therefore cannot consume development events
or delete development documents. Concurrent runs use different namespaces. A hard
process kill or assertion failure may retain diagnostic state; do not indiscriminately
delete schemas/indices from another active run. Development defaults remain
`public` and `spindle-evidence-v1`.

## Historical privacy remediation

After updating exclusions, explicitly invoke `rescan_privacy`. It applies current
personal and project policies to your historical sources and authored memories,
even in projects where membership was revoked. Matching originals and transitive
derived/superseded prose are cleared from canonical content columns and retired.
The operation returns a run ID; `privacy_run_status` with `run` reports completion
after queued index deletions finish. Other users cannot read that job status.

Metadata and lineage remain for auditing. This is application-level content purge;
physical PostgreSQL WAL, prior backups, external exports and provider retention are
not claimed erased. No raw transcript store exists yet. Ordinary retraction still
retains historical text; explicit privacy remediation has the stronger behavior.
Authors may retract their own sessions after membership removal, but cannot use old
sessions to write or regain project retrieval access.

## Note terminology compatibility (first v0.3 migration slice)

New clients can use `publish_note` with `project`, `request_key`, `text`, and
`inputs`, and `correct_note` with `note`, `request_key`, and `text`. Both return
`note` as the resulting object ID. MCP exposes `spindle_publish_note` and
`spindle_correct_note` with those same fields.

The existing `publish_memory` and `correct_memory` operations remain available
and keep their existing `memory` response field (`correct_memory` also accepts the
existing `memory` input field). They share authorization, privacy, idempotency keys,
transactions, storage and lineage with the note-named operations. A request retried
under either spelling returns the same object ID; changing content under the same
key still conflicts. Mixed `note`/`memory` correction fields are rejected.

This is an additive wire-name migration only. Existing IDs, database tables,
inspection/search kinds and retraction counters are unchanged. Board lifecycle,
hierarchy, attributes and subscriptions remain subsequent T023–T026 work.

## Optional board lifecycle

After `make migrate` (schema 8), every project has `board_enabled=false` by default,
including preexisting projects. The HTTP `/v1` operations `board_status`,
`enable_board`, and `disable_board` take a `project` ID. Any current member can read
status; only the owner can change it. Responses contain `project` and `enabled`.
The shared MCP exposes `spindle_board_status`, `spindle_enable_board` and
`spindle_disable_board`. Repeating a desired-state operation is idempotent; changes
are serialized with membership updates through the existing transaction guard.

The flag is PostgreSQL state and survives service restarts. Disabling preserves
all evidence, note identities, versions, lineage and ordinary search/inspection.
This slice does not yet provide placements, folder/tag listing, attributes,
subscriptions or lifecycle event delivery. Those later APIs must enforce the flag;
enabling it today does not claim those organization features exist.
