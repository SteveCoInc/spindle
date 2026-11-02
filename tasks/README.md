# Initial task backlog

The repository owns task definitions and acceptance. GitHub [phase tracker #7](https://github.com/SteveCoInc/spindle/issues/7)
mirrors progress. `make factory-backlog` shows executable readiness and exclusions;
`make factory-next` selects the first eligible task by ID without claiming it.
The table below includes broader planning items not yet split into executable files.
T001/T005/T019 remain partial or blocked; unrelated product work can proceed.
PR #4 proved native merge-group checks and automatic queue merge.

Current bounded dispatch backlog:

| Task | State | Tracking | Acceptance boundary |
|---|---|---|---|
| T027 | Ready / active claim during implementation | [#5](https://github.com/SteveCoInc/spindle/issues/5) | Task readiness and pickup; no unattended scheduler claim |
| T028 | Ready | [#6](https://github.com/SteveCoInc/spindle/issues/6) | Optional board enable/disable only; part of T023 |


| ID | Milestone | Depends on | Task and acceptance evidence |
|---|---|---|---|
| T001 | M0 | — | Record accepted architecture and trace/retention defaults; Rust/host/search and PostgreSQL adjacency contracts; record DSQL portability questions and the deferred graph scope |
| T002 | M0 | T001 | Pin Rust/Cargo.lock; add Cargo workspace/Compose/migrations and bootstrap/doctor; clean checkout starts real dependencies and restart preserves expected data |
| T003 | M0 | T002 | Build task schema, Make interface, CI and artifact manifests; missing suites and intentionally broken qualification fail |
| T004 | M0 | T003 | Implement independent reviewer runner and branch/queue runbook; exact-SHA report, stale evidence rejection and failure propagation demonstrated |
| T005 | M1 | T002, T003 | Project/context identity, membership, activation and Software blueprint; deny unauthorized reads/writes and snapshot session bindings |
| T006 | M1 | T005 | Both host integrations, Claude SDK bridge, capture spool and privacy; restart/duplicate/partial-session tests and real-host smoke evidence |
| T007 | M1 | T005 | Versioned sources, transformations, DAG/reverse index and jobs; independent support, cycle rejection and resumability verified |
| T008 | M1 | T006, T007 | Rust extraction/synthesis, OpenSearch hybrid retrieval, PostgreSQL entity/lineage adjacency; qualified evidence retrieval and projection rebuilds verified |
| T009 | M1 | T008 | Correction and full rollback; preview, 30-hop/shared-descendant cases, concurrent synthesis and reconciled completion verified |
| T010 | M2 | T004, T009, T019 | Integrate qualified NeMo Codex harness, Rust scenario controller and isolated Docker worlds; real multi-persona scenario with production capture/tools |
| T011 | M2 | T010 | Treatment/control execution, artifact capture, grading and budget termination; first honest baseline comparison and follow-up task |
| T012 | M3 | T009 | Complete lifecycle, search modes, collaboration controls, steering, blueprint catalog, archive/history and privacy rescans |
| T013 | M3 | T011, T012 | Briefing/Impact/feedback and daily/on-demand Night Shift; source-grounded insight and correction-policy coverage |
| T014 | M3 | T009, T012 | Bounded/semantic rollback with residuals, escalation and deletion accounting; full AT-01–20 coverage |
| T015 | M3/M4 | T011, T013, T014 | Numeric, ETL, BI, research and aging scenarios; external oracles, repeated comparisons and holdout discipline |
| T016 | M4 | T015 | Recovery/load/concurrency, migrations and restore; measured budgets and failure evidence |
| T017 | M5 | T016 | AWS IaC and storage adapters; qualify PostgreSQL versus DSQL and OpenSearch Serverless; authorized deploy, contract/E2E suite, cost/retention and teardown evidence |
| T018 | M6 | T016 | Formal-reasoning plan, reevaluate Cypher/SPARQL and dedicated graph engines; no engine work until requirements and justification tests are adopted |
| T019 | M0 | T002 | NeMo/Docker framework spike on upstream fixtures; qualify Codex/Claude protocol boundaries and Rust integration feasibility. Product-backed multi-session/capture/grading/cleanup acceptance belongs to T010/T011 after T009 |
| T020 | M2 optional | T010, T011 | Qualify Daytona on a small scenario: authenticated network access, persona isolation, artifact export, aggregate cost and confirmed remote cleanup |
| T021 | M0 | T003 | Compare/adapt Codex CLI/app-server and Symphony/Agent Orchestrator patterns; implement thin Rust coordinator boundary with Git tasks, durable claims/fencing and independent worker identities; no dashboard/platform rewrite |
| T022 | Before 3–4 workers | T004, T021 | Qualify isolated workers and test resources, capacity/backoff, crash/restart/stale-lease recovery, protected merge admission and throughput metrics on a disposable repo; operator verifies subscription entitlement separately |

T002 can use provisional adapter configurations established by T001; the factory
should not wait for every future AWS decision. T005–T009 should be integrated in
small slices rather than developed as isolated subsystems that only meet at the end.

## v0.3 board and note follow-up

These are newly specified work, not completed capabilities. Preserve the existing
service while introducing a versioned naming migration and the optional board.

| ID | Work | Acceptance |
|---|---|---|
| T023 | Adopt note terminology with API/storage compatibility; unified optional board identity and lifecycle | AT-25; legacy IDs/retries/lineage preserved |
| T024 | Hierarchy/tag listing, blueprint/custom attributes, valid time, typed dependencies and concurrent edits/claims | AT-26–30, AT-35–36 |
| T025 | Durable session subscriptions and actual host steering/fallback | AT-31–39, including snapshot/replay and membership revocation |
| T026 | Data-project collaboration gym and Night Shift board maintenance | AT-37, AT-40; external dataset grading and matched comparison arms |

## Future backlog — not scheduled in this phase

External audit export and project off-switch integration are captured in v0.3 §31
(FUT-AUD-01–03, FUT-CTL-01–04). Organizational hazard-rule evaluation belongs to the
external service. Do not add this extension to the active board/subscription tasks;
implementation needs a future scheduling decision and qualified control interfaces.
