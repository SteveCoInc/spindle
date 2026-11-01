# Proposed delivery plan

This is a proposal based on the uploaded v0.2 spec and the user's subsequent
request. It is not a claim that any milestone has shipped. The [source spec](../specs/project-spindle-spec-v0.2.md)
is retained unchanged. The user confirmed a standalone product: the spec's assumed
existing Context backend does not exist here. Build a local Context instance and
general-context support as part of Spindle.

## Scope and boundaries

The first product release includes the spec's non-formal-reasoning features, staged
across milestones rather than compressed into the first demo. Formal claims,
OWL/SHACL/Datalog/SMT, inference engines, and formal proof maintenance come later.
Ordinary LLM extraction, synthesis, temporal contradiction detection, and labeled
Night Shift insights remain in scope, as confirmed by the user.

Build local first. A single-machine service architecture is sufficient. AWS is a
late deployment milestone, not a prerequisite for the local feedback loop. No new
customer web console or clickable HTML mock is required by this request.

## Milestones

| Milestone | Deliverables | Exit evidence |
|---|---|---|
| M0 — Factory foundation | Accepted decisions, toolchain pins/lockfile, workspace layout, Make entry points, local Compose dependencies, CI, task schema, PR template, reviewer runner, manual setup runbook | Fresh checkout boots; an intentionally failing check fails qualification; local and CI run the same checks; reviewer produces a structured report on an exact commit |
| M1 — First useful local product | Project/membership/activation; Software blueprint; capture + privacy; original evidence; synthesis; lineage; OpenSearch hybrid retrieval; limited Cypher and SPARQL projections; inspect/correct/full rollback; both Codex and Claude Code/SDK adapters | Two isolated identities use real services: A contributes, B retrieves a scoped answer with evidence, A retracts, and B no longer sees unsupported content after completion; restart/retry and unauthorized-access tests pass on both hosts |
| M2 — First live gym loop | Codex runner; separate users/workspaces; software seed project; enabled/disabled arms; trajectory and state capture; external graders; summary artifacts | Real Codex sessions complete a real project task through the same integration as a user; run → analyze → candidate change → rerun works, with bounded cost and honest inconclusive outcomes |
| M3 — Complete local feature set | Integration hardening; all four blueprints; all search modes; author/collaborative correction; feedback/Briefing/Impact; Night Shift; steering; privacy rescan; bounded and semantic rollback; ETL/BI and research scenarios | Every non-reasoning requirement has executable coverage; AT-01–20 pass applicable tests; real-host smoke tests and representative live gym reports exist |
| M4 — Quality and resilience | Longitudinal gym; memory aging/contamination experiments; recovery and fault injection; concurrency/load budgets; migrations/restore; held-out evaluation | No invariant regressions; measured quality and resource budgets; repeated treatment/control results with uncertainty; successful crash/restart and backup-restore exercises |
| M5 — AWS deployment | Infrastructure as code, container delivery, identity/secrets, OpenSearch Serverless + Neptune Analytics adapter qualification, telemetry and restore runbooks | Same acceptance suite passes against deployed adapters; IAM denial tests and teardown checked; costs documented; deploy and rollback exercised in an authorized sandbox |
| M6 — Formal reasoning (separate phase) | Typed claims; RDF mapping; external validator/reasoner; alternate justifications; reasoning evaluations | AT-21–24 plus privacy, rule-version, scope/time, independent-support, and cross-representation retraction tests |

M0 should stay small. Do not build an elaborate scheduling platform before M1.
Develop deterministic scenarios with M1; M2 adds real agents rather than replacing
the tests. M1 includes full rollback before allowing multi-source synthesis to be
called usable. M3 completes the requested prototype breadth. M4 determines whether
it is effective enough to move to AWS. M6 is not dependent on an AWS roadmap promise.

## First demonstration

Use the spec's retry-policy scenario. Avery records that legacy charge writes must
not be retried without idempotency keys. Sam retrieves that decision while coding.
Jordan supplies v2-specific evidence. The synthesis keeps the distinction. Retract
Jordan's session and verify that legacy guidance and other independent evidence
survive while the unsupported v2 conclusion is rebuilt or retired. Include a
non-member attempting the same search and graph traversal.

## Decisions before implementation

See [manual setup](manual-setup.md) for confirmed scope and proposed defaults.
Recommend TypeScript, PostgreSQL ledger/jobs, OpenSearch, Neo4j behind a small
portable Cypher interface, and a Jena/Fuseki RDF projection for local SPARQL.
Both hosts in M1, Codex gym first, hosted model calls and a $100/four-hour live-run
ceiling are confirmed. Specific storage/language choices remain recommendations.
Estimate calendar time only after M0's integration/storage spikes; host capture and
rollback correctness are the main early uncertainties.
