# Factory orchestration and parallel Codex workers

Status: researched design proposal, not an installed orchestrator or measured
throughput result. The initial factory plan had useful quality controls but had
not compared orchestration products, account handling or crash recovery in detail.
This assessment refines that plan; it does not authorize spending or account setup.

## What the existing plan gets right and what changes

Retain repository-owned tasks/specs, small branches, executable acceptance checks,
exact-commit evidence, independent review and one protected integration path.
These provide the boundaries that make autonomous work assessable.

Strengthen the operational design: atomic claims rather than competing Markdown
edits; one coordinator rather than independent planning swarms; isolated workspace
and test resources; explicit retry/recovery; ownership of shared interfaces; capacity
and budget limits; and reporting of accepted output rather than number of agents.
Reduce M0 scope: validate a thin control loop before building a dashboard or a
general orchestration platform. An LLM is useful for planning/review, not necessary
for every heartbeat or queue transition.

## Research and alternatives

| Approach | Evidence and useful patterns | Fit for Spindle |
|---|---|---|
| Codex CLI/app-server plus native sub-agents | Published source exposes thread/turn/review lifecycle and account/rate-limit status; sub-agent implementation exists | Default execution substrate. Native sub-agents help with bounded work within a task; they do not replace the durable task/PR scheduler or supply independent accounts |
| OpenAI Symphony | Repo-owned WORKFLOW.md, one dispatch authority, isolated issue workspaces, bounded concurrency, retry and reconciliation; language-neutral specification and Elixir reference | Best architectural reference for unattended task execution. The reference is explicitly a prototype; do not import all its behavior or assume production guarantees |
| Agent Orchestrator / Orchestrator.inc | Per-task branches/worktrees, Codex and other agents, PR/CI feedback routing, persistent daemon and session lifecycle | Strong candidate for an existing operator interface. Qualify repo-owned task integration and trust boundaries before adopting; external tooling need not be Rust |
| Gas Town | Persistent handoffs, task ledger, roles, merge processing and capacity governor | Useful recovery/queue patterns; larger operational and conceptual footprint than needed for the first 3–4 workers |
| Fully custom swarm framework | Complete control | Avoid. Write only the Rust task adapter/control logic actually missing from the chosen tools |

This is evidence of implemented/documented approaches, not a survey establishing
which is most popular or a benchmark proving one is faster. Do not repeat projects'
scale/quality marketing as measured Spindle results. NeMo Gym and Daytona evaluate
and isolate product experiments; they do not replace the factory's PR/merge policy.

## Recommended path

Start with Codex execution and a small Rust coordinator inspired by Symphony's
single-authority model. Read versioned task definitions directly from Git. Keep
dispatch state durable and separate from the working repository. Evaluate adopting
Symphony or Agent Orchestrator as an external control plane before implementing
equivalent runtime features; any choice must preserve task authority, independent
gates and supported worker auth. Do not port the entire Symphony reference merely
to use Rust, or change the product language to match an orchestration tool.

Use `codex exec` for a bounded one-shot worker where sufficient. Qualify app-server
stdio/JSON-RPC for continued turns, cancellation, status and richer review sessions.
Pin the CLI and generate/validate its protocol schema. Do not link Spindle directly
to unpublished Codex internal Rust crates or assume source on main is a stable SDK.

Two levels of parallelism should remain explicit:

1. Independent task workers: separate workspace, branch, runtime and task claim;
   results go through the integration queue.
2. Native sub-agents inside one task: bounded research, review or disjoint work
   under the parent task's ownership and limits. They do not get a new task lease
   or assumed independent model quota. Count them in capacity/spend accounting.

Early scaffold/schema work has a narrow dependency chain. Start with one or two
builders until interfaces exist. Then run two builders and one reviewer; add a
third builder only if review latency, conflict rate and CI capacity allow it.
The deterministic coordinator/merge controller does not need a dedicated model
subscription. One account is not intrinsically one process or one role.

## Task authority, claims and recovery

Git stores definitions, acceptance criteria, dependency DAG, decisions, configuration
and terminal evidence summaries. It is not an atomic runtime lock service. Reading
the same ready task and committing two owner edits does not prevent double dispatch.

Use one coordinator with a local durable SQLite ledger initially, or a separately
managed factory PostgreSQL database if multiple coordinator hosts become necessary.
Workers contact the coordinator; they do not share a network SQLite file. This
operational state must not depend on the Spindle product being implemented or on
the gym's resettable data. One coordinator can manage workers on different machines.

Each claim records task revision/hash, base SHA, worker ID, attempt ID, lease expiry
and monotonically increasing fencing token. Claim acquisition and capacity
reservation are atomic. Heartbeats renew leases; stale attempts cannot publish or
finalize after reassignment. Branch names include task and attempt IDs. Retain the
last commit, changed files, tests, blockers and next steps in structured checkpoints.

On restart, reconcile the ledger with worker liveness, saved attempts, branches,
PRs and check status. Expired heartbeat means inspect/reconcile, not blindly launch
a duplicate. Terminate or fence the old attempt before reassignment; preserve
committed and recoverable uncommitted work. Resume native conversation only when
the pinned runtime supports it; otherwise start a fresh session from the checkpoint.
Never copy a live authentication/session cache between accounts to resume work.

Only the coordinator writes the consolidated status view and terminal task receipts.
Commit durable summaries referencing exact source and merge SHAs after transitions;
heartbeats and raw logs remain runtime artifacts. GitHub issues/boards can mirror
tasks, but must not become an independently edited second backlog. A worker finishing
a turn is not task completion: verify acceptance, review, integration and required
post-merge checks before reporting done.

## Workspace and integration rules

One writable checkout per task attempt. Existing cloud-task isolation is adequate;
do not add worktrees inside each managed task. On a local multi-worker host, factory-
managed separate clones or explicitly enabled Git worktrees are appropriate. This
is a future factory mode, not permission for an individual agent to create worktrees
contrary to the session's instructions. Worktrees isolate Git files, not credentials,
network access, build caches or databases.

Give every attempt unique Compose project names, ports, database/schema and search
index names, temporary paths and writable build outputs. Reuse only compatible,
appropriately trusted caches. Test resets may touch only that attempt's resources.
Held-out graders, runner control sockets and deployment credentials stay outside
builder sandboxes. Native sub-agents must not silently multiply the declared worker
budget or share write access to conflicting tasks.

Declare read/write footprints and interface dependencies. Serialize changes to
Cargo.lock/workspace manifests, common contracts, migrations and gate definitions
where concurrent edits would conflict. Establish/merge an interface contract before
parallel implementation; use a separate integration task for cross-crate changes.
Do not forbid a necessary shared edit: return it to the coordinator for scheduling.

All workers start from an identified accepted base and produce small PRs. One merge
queue/controller integrates them. New commits, material base changes and contract
changes invalidate relevant evidence. Re-run checks on the merge candidate. Bound
repair attempts and return failures to the owning worker; pause admission when
review or CI queues exceed capacity instead of accumulating unfinished branches.

## Three or four accounts

The design can represent 3–4 authenticated worker slots on one host or several.
Qualify them as workers; do not build a credential-rotation pool. Each slot has an
operator/account alias, isolated credential storage, supported login, runtime version,
machine, role/capabilities, concurrency cap, auth health and observed quota state.
No keys/tokens go into Git, task prompts or other workers' sandboxes. Use independent
OS/container credential stores and avoid changing the login under a running worker.

The coordinator assigns eligible new work within each slot's approved capacity.
Authentication expiry pauses that worker for reauthentication. Rate-limit signals
cause backoff/checkpointing; unknown quota is not infinite quota. A handoff to another
authorized worker is an explicit fenced task reassignment with preserved provenance,
not hot-swapping credentials to continue a blocked request. Stop on provider/account
restrictions and seek the supported capacity path.

Separate four identities in reports: task/attempt, worker/runtime, model account or
workspace, and GitHub actor. Multiple model subscriptions do not create distinct
GitHub reviewers, additional merge permissions or statistically independent judgment.
Review independence comes from separate context/permissions and external checks;
reviewing with a different model can reduce correlated mistakes but is not a proof.
Agent review should produce a trusted required check, not impersonate a human
approval. A PR author cannot satisfy an independent human-approval requirement by
changing model logins while retaining the same GitHub identity.

Subscription-backed Codex and API-key-backed automation are different capacity and
billing paths. The public Codex README documents both. Current subscription pooling,
seat eligibility, concurrent-use limits and unattended CI suitability were **not
verified**: pricing/auth/help pages were blocked by this environment's policy.
Do not infer that one individual should buy several personal subscriptions, that
seats for multiple people are interchangeable, or that quotas combine linearly.
Before purchasing, check current official plan terms/limits or obtain provider
confirmation. Compare existing entitled parallel sessions, higher supported capacity,
appropriate team seats and API-funded runners by measured cost per accepted task.

Use a separate factory budget (per task, worker and day); the $100/four-hour cap is
for a gym run and does not authorize unlimited builder/reviewer usage. A good initial
topology is two builders plus one reviewer, or three builders plus one reviewer,
with one deterministic coordinator and merge queue. These are lanes, not a demand
to purchase three or four accounts.

## Qualification and rollout

1. One worker: complete a small change through checks, review and merge; prove a
   deliberately failing check blocks it. Use a disposable qualification repository.
2. Two workers: independent tasks complete; overlapping claims/files are rejected
   or serialized. Verify unique test state and reviewer artifacts.
3. Three/four workers: inject process death, lost heartbeat, coordinator restart,
   stale completion, auth expiry, rate limits, rebase and merge conflict. Require
   no duplicate accepted task, lost work, leaked credential or bypassed gate.
4. Count native sub-agents toward total execution limits. Confirm review/CI pressure
   slows dispatch and that operator intervention preserves the task claim.
5. Compare one/two/four-worker runs on similar bounded tasks: accepted tasks per
   hour, time to reviewed merge, rework/conflicts, defect escapes, human interventions,
   queue wait and cost per accepted task. Do not promise 4x throughput from 4x agents.

No accounts, orchestrators or swarms were started in this research. The repository
plan can support this scale after these control mechanisms are implemented and tested.

## Sources

- [Codex source](https://github.com/openai/codex/tree/67f0994b532b8740f14b5b738af7b7b2a915b1d6), especially [app-server protocol](https://github.com/openai/codex/blob/67f0994b532b8740f14b5b738af7b7b2a915b1d6/codex-rs/app-server-protocol/src/protocol/common.rs): thread/turn/review methods, `account/read`, `account/rateLimits/read` and updates. Runtime support must be qualified at the installed release.
- [Symphony specification](https://github.com/openai/symphony/blob/be10a1b79df723d6d7612b5651c8522704dafb2e/SPEC.md) and [reference implementation](https://github.com/openai/symphony/blob/be10a1b79df723d6d7612b5651c8522704dafb2e/elixir/README.md): single-authority dispatch, tracker adapters, retries, workspace policy and limitations of restart state. Do not assume durable multi-coordinator leases are provided by the reference.
- [Agent Orchestrator architecture](https://github.com/OrchestratorInc/agent-orchestrator/blob/9041e10a6c1dd0acb3e254063265d03ee95268d1/docs/architecture.md): durable facts, derived status, workspace/session lifecycle and native Codex controller. The original ComposioHQ link redirects to this project.
- [Gas Town](https://github.com/steveyegge/gastown/blob/466aa41c83ee73b1fa0eb79eb9d172fb1eca9476/README.md) and [scheduler design](https://github.com/steveyegge/gastown/blob/466aa41c83ee73b1fa0eb79eb9d172fb1eca9476/docs/design/scheduler.md): work tracking, merge processing, capacity and backpressure. Documented behavior, not an executed reliability test.
- Official [Codex multi-agent](https://developers.openai.com/codex/multi-agent/), [worktrees](https://developers.openai.com/codex/app/worktrees/), [authentication](https://developers.openai.com/codex/auth/) and [pricing](https://developers.openai.com/codex/pricing/) pages: requested but unavailable here (proxy denied). No current price/quota or subscription entitlement claim is based on those pages.
