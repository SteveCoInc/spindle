# Running the initial factory

The initial implementation is an explicit single-task runner. It does not yet poll
or dispatch continuously, create PRs automatically or merge code. A persistent scheduler is not installed; CI and the merge queue do not dispatch builders. Tasks are TOML files in tasks/specs; inspect `make help` and
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

## Active-session task pickup

`make factory-backlog` explains every executable task's status, incomplete
 dependencies, pause state and existing claim. `make factory-next` returns the first
eligible task by task ID, or JSON null if none is eligible. These are advisory
snapshots; they do not claim work, launch models or establish acceptance. Concurrent
workers must still acquire the atomic claim. Expired and review-pending claims are
excluded until explicitly reconciled. `make factory-status` retains its runtime
claim output for compatibility. Task IDs currently determine priority; split large
planning items into bounded ready task files before dispatch.

The current chat can implement work directly using its supported runtime. A new
session should execute this loop without asking the user to assign each task:

1. Read AGENTS.md, inspect status, fetch origin and inspect active PRs/claims. Preserve
   existing work. Start each new task branch from accepted origin/main.
2. Run `make factory-backlog` and `make factory-next`. Inspect the selected task's
   acceptance and scope, then acquire `claim TASK WORKER --ttl SECONDS`. Renew with
   `heartbeat TASK WORKER GENERATION --ttl SECONDS` before expiry. If none is ready,
   reconcile completed work or split an authorized roadmap item; do not declare the
   whole project blocked because one model adapter is unavailable.
3. Implement in this isolated cloud checkout. Keep code, acceptance tests and docs
   together. Run `make check test`; run real storage integration for storage work.
4. Commit, run `make qualify`, push the task branch and open a PR linking the task
   issue and exact-head verification. `finish TASK WORKER GENERATION` hands the
   claim to review; it does not mark the task done. Report unavailable independent
   model review honestly. Current required GitHub gates are the two deterministic
   checks; do not claim they provide model review.
5. Wait for both PR checks. Enqueue through native GitHub merge queue with the
   expected head SHA (see github-governance.md); never bypass or push main.
   Watch merge-group checks and verify the merged commit appears on origin/main.
   On failure, repair and requalify the new head instead of accepting stale checks.
6. Record the source/merge SHAs and tests in a repository evidence receipt. Mark
   the task done only after its acceptance and merge are verified, in a follow-up
   PR (normally the next task's PR). Reconcile the stopped prior attempt explicitly.
   Pick up the next ready task in the same session. Issue closure alone never
   updates task authority or releases a claim.

Repository TOML is the executable authority; tasks/README.md is the broader roadmap.
GitHub issues mirror that authority for visibility. [Issue #7](https://github.com/SteveCoInc/spindle/issues/7)
tracks phases; [#5](https://github.com/SteveCoInc/spindle/issues/5) tracks T027 and
[#6](https://github.com/SteveCoInc/spindle/issues/6) tracks T028. T028 is the next
product slice, not completion of T023 or the entire board.

## What keeps running after chat ends

Nothing currently launches another builder after this chat ends. The hosted chat
is not a durable daemon, and adding cron inside its temporary environment would
not establish one. GitHub CI runs on PR/merge events; it cannot implement tasks by
itself. A timer alone would repeatedly hit the unresolved nested-Codex auth failure.

Unattended dispatch still needs a persistent runner and a supported authenticated
model execution path, with separate factory runtime/spend limits. The existing
`run` command handles one task only; reconciliation, PR submission/repair, queue
observation and repeated dispatch still need implementation and qualification.
Do not configure a paid scheduled job or copy managed ChatGPT login files into CI.
See manual-actions.md for the precise boundary. These gaps do not block the
active-session loop above or deterministic product development.
