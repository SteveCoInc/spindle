# Proposed Codex software factory

## Repository as the durable workspace

Commit specs, decisions, tasks, code, migrations, prompts, tool contracts, sanitized
fixtures, scenario definitions, and evaluation summaries. Keep large/raw/private
trajectories outside Git; commit their content-addressed manifests and retention
classification. Each new session can discover the current task and evidence from
the repository rather than reconstructing a chat.

Proposed layout (directories below are not all implemented):

```text
crates/api/               Rust HTTP and MCP entry points
crates/worker/            Rust durable jobs and scheduled maintenance
crates/domain/            invariants and application services
crates/storage/           PostgreSQL adjacency, search and blob adapters
crates/integrations/      Codex and Claude Code lifecycle interfaces
crates/contracts/         API/event/schema definitions
crates/gym/               Rust scenario control, graders and reports
gym/                      pinned NeMo config and external harness integration
gym/scenarios/            seed refs, user roles, schedules, tool capabilities
tests/                    unit, contract, integration and end-to-end suites
evals/                    development datasets and grader contracts
factory/                  task schema, qualification and reviewer orchestration
tasks/                    committed task status and evidence
docs/                     specs, ADRs, plans, runbooks, experiment reports
infra/                    Compose first; AWS IaC later
artifacts/                ignored local reports; uploaded by CI when appropriate
```

Use ADRs for decisions that affect storage semantics, host behavior, privacy,
deployment, or evaluation. Mark proposals as proposed. Record replacements instead
of silently rewriting history. Changes to the source spec use a new version or an
explicit amendment; retain the supplied artifact unchanged.

## Task contract

Implement a validated TOML task format in M0. Required fields: ID, title, status,
dependencies, scope/non-goals, requirement IDs, acceptance scenarios, permitted
paths, verification commands, risk, owner/lease, evidence references and blockers.
States: proposed → ready → in_progress → review → done, plus blocked. `done`
requires machine evidence, not an agent's assertion. Generate the human backlog
from the same records once the task tooling exists. A dependency cycle or a task
with no acceptance criterion must fail validation.

The user's speech about "last-commit-based code", TOML, branches, PRs and queues is
interpreted here as small task branches, structured task configuration, and checks
bound to the exact commit. If stacked PRs were intended, add them only after the
simple queue works; stack rebases invalidate descendants' evidence.

## One command interface

These are **proposed commands**, not executable targets yet. Each must do actual
work, preserve failure exit status, print its scope, and record tests run/skipped.

| Command | Contract |
|---|---|
| `make bootstrap` | Verify pinned tools; frozen dependency install; no credentials in output |
| `make doctor` | Check tools, services, versions and named credential presence; explain repairs |
| `make up` / `make down` | Start/stop local services without silently deleting persistent data |
| `make test` | Deterministic unit/domain/property tests; nonzero test count required |
| `make test-integration` | Real PostgreSQL adjacency/lineage and OpenSearch contract tests |
| `make test-e2e` | Scripted multi-user API/MCP lifecycle including rollback and authorization |
| `make check` | Rust fmt, Clippy, build, contracts, migrations and dependency-boundary checks |
| `make qualify TASK=…` | Required checks plus independent review in a clean checkout of candidate SHA |
| `make review TASK=…` | Reviewer runner against exact SHA, base spec and collected test evidence |
| `make pr TASK=…` | Verify evidence still matches HEAD, then submit branch/PR when authorized |
| `make queue-status` | Show failed/stale checks, merge-group state and precise recovery action |
| `make gym-smoke` | Fast deterministic scenario through production paths |
| `make gym-live SCENARIO=…` | Qualified NeMo harness; Docker default, optional Daytona; aggregate spend/time ceilings |
| `make gym-report RUN=…` | Artifacts, outcomes, failure attribution and comparison with baseline |

Use `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace --locked` (or a pinned qualified nextest runner), and locked
builds behind the Make interface once the workspace exists. Commit Cargo.lock for
the applications and pin rust-toolchain.toml. Test build features and release artifacts
explicitly; do not claim all-feature compatibility without checking feature conflicts.

Before CI exists, no command should falsely pass by printing "not implemented".
Paid runs cannot silently fall back to mock mode. Missing model credentials produce
an explicit unavailable/blocked result. M0 installs a qualification self-test that
introduces a known failure in a disposable fixture and proves the gate rejects it.

## Build, review, and merge loop

1. Select a ready task; establish a small branch and expected base SHA. Record
   ownership through a task update/central scheduler lease if several workers run.
2. Implement code and meaningful tests. The implementer may run fast incremental
   checks, but authoritative qualification uses committed inputs in a clean checkout.
3. Produce an evidence manifest: source/base SHA, lockfile/tool versions, required
   suite IDs, executed test counts, exit codes, artifact hashes and environment.
4. Run a second agent in an isolated reviewer workspace with read-only source and
   output-only report permissions. Give it the exact diff, baseline requirements,
   relevant code and test evidence. It reports requirement mismatches, test gaps,
   stubbed behavior, architecture drift and actionable findings with locations.
5. Qualification validates the report schema and resolves blocking findings. The
   implementer cannot self-approve the independent report. AI judgment supplements
   external checks; it cannot certify correctness by itself.
6. Submit the PR with problem/behavior, task/spec links, checks and material limits.
   New commits invalidate earlier qualification and review. Re-run on the actual
   merge candidate, not just a branch that used to pass.
7. CI on `pull_request` and `merge_group` runs the applicable checks; the queue
   integrates verified candidates. A base change triggers new combined validation.

The reviewer should work locally before PR submission and in CI. It needs the
same supported runner interface as gym agents, but separate credentials, permissions
and prompts. Reviewer timeout/unavailability is not approval. Budget consumption
must be reported separately from gym experiment costs.

Keep the default workflow fast: changed-component checks locally, complete relevant
integration/acceptance checks in CI, expensive repeated gym experiments on schedule
or deliberate qualification. Do not make every documentation PR spend $100.

## Gates agents cannot quietly weaken

An agent with unrestricted write access to tests, workflows, graders and branch
protection can alter its own exam. AGENTS.md alone cannot prevent this. Establish
trust boundaries: protected main, no author bypass, CODEOWNERS for factory/grader/
workflow changes, trusted baseline workflows or centrally controlled required
checks, held-out tests unavailable to the implementing workspace, and CI-generated
evidence. A trusted runner checks out the candidate as data in a restricted job;
never execute untrusted PR code with deployment credentials or a broad write token.

Detect missing/disabled/skipped suites and assertion deletion; require explicit
review for changes to expectations or acceptance thresholds. Use property and
metamorphic tests (duplicate ingestion, input-order changes, replay, independent
support, time/scope changes), adversarial authorization tests, and targeted mutation
testing of rollback/privacy modules. Coverage alone is not a quality target.

Held-out fixtures/grader implementations may live in a separate access-controlled
repository while remaining versioned. Public regression fixtures stay in this
repository. With only one unrestricted agent-visible repository, claim tamper
evidence and review, not cheat-proof evaluation. A human initially owns changes
to the gates; routine compliant feature changes can merge automatically once the
user enables the queue. Verify queue availability for the account/repository plan.

For stuck queues, inspect pending check names, merge-group triggers, stale review,
conflicts and flaky jobs. Retry only diagnosed transient failures; never bypass
gates or delete another task's branch to make progress. Provide a documented manual
recovery path if native merge queues are unavailable.

## Observability and artifacts

Every run needs task/run/session/trace IDs, source SHA, configuration and prompt
hashes, model identifiers, tool versions, test counts, failure categories, latency,
token/cost accounting and artifact locations. Use structured logs and OpenTelemetry
where useful. Keep service traces distinct from private agent trajectories. A local
report must be inspectable without a hosted dashboard. CI uploads JUnit/coverage,
review reports and sanitized logs with bounded retention.

Pinned dependencies, digest-pinned containers, reproducible seed inputs, disposable
integration data, migrations, artifact manifests, secret scanning, dependency/license
checks, and a backup/restore exercise round out the factory. Do not add machinery
that has no gate, owner or demonstrated failure mode.
