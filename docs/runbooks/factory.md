# Running the initial factory

The initial implementation is an explicit single-task runner. It does not yet poll
or dispatch continuously, create PRs automatically or merge code. No schedule is
required. Tasks are TOML files in tasks/specs; inspect `make help` and
`cargo run --locked -p spindle-factory -- --help`.

```sh
make check test
make factory-status
cargo run --locked -p spindle-factory -- claim T001 worker-a --ttl 900
```

A claim prints its generation. Heartbeat/finish require the matching task, worker
and generation. Expired claims are never stolen automatically. Stop and inspect
that worker, preserve its work, then use `release TASK GENERATION --worker-stopped`.
The flag is an operator attestation, not process-liveness detection. New claims
increment the generation; old workers cannot finish. Finish means review pending,
not accepted/merged/done. Pause blocks new claims but allows active heartbeats.

For a ready task with completed dependencies, `run TASK WORKER --timeout 900`
creates an isolated clone and task branch under .factory/workspaces, invokes Codex,
retains logs/work, and hands successful execution to review. The worker is instructed
not to push/merge/deploy or launch nested agents. This first runner uses the current
operator's supported Codex login; it is not a multi-account scheduler or a hardened
sandbox manager. The parent lease is bounded by timeout plus cleanup allowance.
Failed/crashed attempts remain claimed until explicit reconciliation.

Commit changes before `make qualify`. Qualification clones that commit into a
fresh temporary directory and runs check, test and test-integration. It records
source SHA, check results and logs under artifacts/SHA. It never interprets a
successful worker exit as passing tests. `make review` starts a read-only Codex
session in another clean clone, requests structured findings, and fails on errors,
timeouts, mismatched SHA or findings. Existing Codex authentication is required.
Review evidence is not a trusted CI attestation until protected execution is wired.
The implementer can still modify local artifacts; do not use local reports alone as
unforgeable merge authority.

CI currently defines `Rust checks and tests` and `Storage integration` for pushes,
PRs, merge groups and manual dispatch. Configure repository rules only after the
checks have executed remotely. AI review remains a local command until the secure
CI authentication/budget path is configured. Do not put an interactive ChatGPT
credential cache into GitHub secrets; choose a supported CI authentication method.

Factory state (.factory) and artifacts are ignored. Commit task definitions and
sanitized evidence summaries. Never commit model credentials or raw private traces.
