# Manual setup and decisions

## Confirmed by the user

- Build a standalone backend; there is no existing AWS Context service to integrate.
- Support both Claude Code and Codex in the first usable release; Codex drives the
  first live gym. Include Claude Agent SDK integration in the prototype.
- "Spark" meant **SPARQL**, not Apache Spark. The later instruction defers
  Cypher/SPARQL and dedicated graph engines until formal reasoning.
- Write Spindle code in **Rust**; retain **PostgreSQL + OpenSearch**. PostgreSQL
  adjacency represents graph data now. Aurora DSQL is a possible future AWS store.
- Defer formal reasoning only. LLM extraction, synthesis, contradiction handling,
  and evidence-backed Night Shift insights remain in scope.
- Run services locally; hosted model calls are acceptable. Offline agent inference
  is not a requirement.
- Full live gym run ceiling: **$100 and four hours**.

## Proposed defaults for review

Rust Cargo workspace with PostgreSQL ledger/jobs/entity adjacency and OpenSearch.
Author-controlled corrections, user/workspace activation, private filtered traces,
and no self-approval of quality gates. Qualify NeMo Gym with local Docker first;
Daytona is an optional remote execution profile, not a local prerequisite. See the
[platform evaluation](gym-platform-evaluation.md) for source evidence and remaining
work. DSQL requires a separate SQL/transaction compatibility spike before adoption.

## Actions that can be taken now

1. Review the proposed milestone order and NeMo qualification criteria. Implementation tasks
   remain proposed until the plan is adopted; the original spec is retained.
2. Choose the development machine used for the first local run. Linux or macOS
   with a Docker-compatible Compose runtime is the proposed target. Record OS,
   architecture and available RAM/disk in the M0 doctor report. Measure memory/disk needs for PostgreSQL, OpenSearch and concurrent agent
   containers; size the local concurrency accordingly.
3. Identify which existing OpenAI/Codex and Anthropic accounts may fund the live
   runs. Do not paste keys into chat or commit them. The repository's M0 runbook
   must name the exact supported authentication and credential locations after
   the integration spike. No credentials are required to review these docs.

## At M0: GitHub factory controls (repository administrator)

1. In repository **Settings → Actions → General**, enable permitted Actions and
   keep the default workflow token read-only. Grant write access only to the
   narrowly scoped PR automation job/app that needs it.
2. Once CI workflows exist, run them on a test PR and record the exact check names.
   Do not configure guessed check names: they can leave every PR permanently pending.
3. In **Settings → Rules → Rulesets**, create an active rule targeting the default
   branch. Require PRs, the implemented status checks, resolved conversations and
   protection against force-push/deletion. Restrict bypass permissions.
4. Add CODEOWNERS for `.github/`, `factory/`, acceptance definitions, grader policy
   and dependency/security controls. Choose the actual trusted reviewer/team; no
   fictitious GitHub handle is supplied here. Require their review for gate changes.
5. If the repository/account supports merge queues, enable the queue and verify
   CI handles `merge_group`. Otherwise document a serialized merge procedure that
   checks the rebased candidate. Never label the fallback a native merge queue.
6. Enable permitted auto-merge only after negative tests prove that failed/stale
   qualification cannot merge. Test a new commit after approval, a failed suite,
   and a base-branch update. Record results in the M0 task evidence.
7. Add reviewer/model credentials through GitHub **Settings → Secrets and variables
   → Actions**, using the exact variable names established by the implemented
   adapter. Restrict live jobs to trusted code; never expose these to fork PR code.
   Configure an explicit separate reviewer budget before requiring paid reviews.
8. If held-out evaluations must be inaccessible to builders, create a separate
   private grader repository or trusted CI artifact source and grant the evaluator
   read access only. Record its version and access model in the factory configuration.

These steps are future prerequisites, not claims that workflows or rules exist.
UI labels/features can vary by GitHub plan; the official queue docs are linked in
references but could not be fetched from this environment.

## At M1/M2: local credentials and first live run

1. Run the implemented `make doctor` and resolve its tool/service checks.
2. Follow each pinned host's supported authentication flow on the local machine.
   Configure provider credentials through the secret manager or ignored local
   config specified by the adapter; inspect names/status only in diagnostics.
3. Install the development integration using its tested installation command,
   provided by M0/M1. Verify active-project visibility, capture, leave/switch and
   restart behavior on both hosts. Do not assume MCP registration installs hooks.
4. Set gym ceilings to at most $100 and 14,400 seconds, and run the smaller pilot
   scenario first. Confirm that no credentials or raw traces enter Git/reports.
5. Inspect the baseline/treatment report and evidence before enabling scheduled
   unattended live evaluations. Configure a daily aggregate ceiling too: a per-run
   cap alone does not limit unlimited repeated runs.

## Before enabling three or four factory workers

1. First qualify concurrent workers using the current supported entitlement. Model
   accounts are not worker-count licenses. Check current Codex plan/seat terms and
   limits before buying multiple subscriptions; compare supported extra capacity,
   appropriate team seats and API-funded automation. This environment could not
   fetch the current pricing/auth pages and does not establish pooling eligibility.
2. In the implemented worker configuration, register non-secret aliases, machines,
   runtime versions, capabilities and concurrency limits. Each worker uses its own
   supported authenticated OS/container credential store; keep credentials out of
   Git, prompts and other workers. Verify account/workspace and auth health without
   printing tokens. Document reauthentication and per-worker pause/resume steps.
3. Set the coordinator's initial lanes to two builders and one reviewer. Use one
   writable checkout per attempt and unique ports, Compose project names, test
   databases/indices and writable artifacts. A fourth lane is an additional builder,
   subject to review/merge capacity. Existing isolated cloud tasks need no extra
   worktree; local factory worktree mode must be explicitly enabled and qualified.
4. Configure separate factory per-task and daily budgets; the gym's $100/four-hour
   cap is not a factory budget. Track observed quota/reset signals, unknown states,
   nested sub-agents, timeouts and backoff. Never swap a running worker's login.
5. Run the factory qualification suite against a disposable repository: duplicate
   claims, shared-file contention, worker/coordinator crash, lost heartbeat, stale
   completion, auth expiry, rate limit, merge conflict and failing required check.
   Require preserved work, bounded retries and no bypassed gates before automation.
6. Keep the GitHub authority separate: a model login is not a GitHub approver. Use
   trusted status checks for agent reviews and real independent review where branch
   rules require it. Measure accepted-task throughput, rework, queue time and spend
   before purchasing or enabling more worker capacity.

These are preparation/qualification steps, not commands that exist today. See
[factory orchestration](factory-orchestration.md) for the research and failure model.

## Optional Daytona profile: after local gym qualification

1. Select the Daytona organization and region permitted for synthetic test projects.
   Check current pricing and concurrency/resource quotas. Do not deploy the old
   unmaintained public core as an assumed supported local service.
2. In the Daytona dashboard, create a controller-only API key with the documented
   sandbox create/cleanup permissions and expiry. Store `DAYTONA_API_KEY` in the
   configured secret manager; never mount it into an agent sandbox. Snapshot
   permissions are only needed if the chosen provider configuration uses them.
3. Pin the compatible SDK/NeMo version and image/snapshot using the M0 runbook.
   Record network endpoints and verify that remote agents reach authenticated
   Spindle/NeMo/model services; localhost on a developer machine is not reachable
   from a remote sandbox by default.
4. Run one small scenario and verify artifacts, cancellation and confirmed deletion.
   Add TTL/janitor cleanup and aggregate cost accounting before scheduled runs.
   Keep local Docker functional without any Daytona credentials.

## At M5: AWS (deferred)

Choose sandbox account, region, cost owner and monthly ceiling. Authorize a narrowly
scoped deployment identity (prefer GitHub OIDC/workload roles), then review IaC
plans, service availability and minimum costs. Implemented runbooks must provide
exact deploy, smoke-test, restore, rollback and teardown commands and explain data
retention. No AWS setup, deploy, publication or new cloud-environment draft update
has been performed as part of this design proposal.

## Documentation network access

This environment allowed official SDK README retrieval from GitHub but denied
`docs.aws.amazon.com` and `docs.github.com`. A later live documentation check needs
those destinations allowed in environment settings, preserving existing rules, or
access from the local machine. No allowlist was changed. Runtime services/model
destinations will be enumerated by the implementation, not guessed from SDK names.
