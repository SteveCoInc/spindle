# Manual setup and decisions

## Confirmed by the user

- Build a standalone backend; there is no existing AWS Context service to integrate.
- Support both Claude Code and Codex in the first usable release; Codex drives the
  first live gym. Include Claude Agent SDK integration in the prototype.
- "Spark" meant **SPARQL**, not Apache Spark.
- Defer formal reasoning only. LLM extraction, synthesis, contradiction handling,
  and evidence-backed Night Shift insights remain in scope.
- Run services locally; hosted model calls are acceptable. Offline agent inference
  is not a requirement.
- Full live gym run ceiling: **$100 and four hours**.

## Proposed defaults for review

TypeScript monorepo; PostgreSQL ledger/jobs; local OpenSearch; Cypher adapter using
Neo4j; RDF/SPARQL projection using Jena/Fuseki; managed AWS equivalents later.
Author-controlled corrections by default, activation scoped to user/workspace,
private filtered traces, and no self-approval of quality gates. The remaining key
architecture decision is whether the two local graph projections are acceptable.
We recommend that tradeoff over claiming a single local engine exactly emulates
Neptune Analytics plus SPARQL. M0 tests the actual image/resource/query compatibility.

## Actions that can be taken now

1. Review the proposed milestone order and graph tradeoff. Implementation tasks
   remain proposed until the plan is adopted; the original spec is retained.
2. Choose the development machine used for the first local run. Linux or macOS
   with a Docker-compatible Compose runtime is the proposed target. Record OS,
   architecture and available RAM/disk in the M0 doctor report. Plan roughly 16 GB
   RAM as an initial development budget for the full storage stack; measure it.
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
