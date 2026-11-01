# Initial task backlog

All tasks are **proposed**, not in progress or complete. This backlog decomposes the
plan; it does not authorize unattended execution of every task. M0 will introduce
validated TOML records and command tooling. Each implemented task must add exact
requirement/acceptance IDs, verification commands and evidence before becoming done.

| ID | Milestone | Depends on | Task and acceptance evidence |
|---|---|---|---|
| T001 | M0 | — | Record accepted architecture and trace/retention defaults; host, search, Cypher and SPARQL spikes document tested capabilities and limitations |
| T002 | M0 | T001 | Pin toolchain; add workspace/Compose/migrations and bootstrap/doctor; clean checkout starts real dependencies and restart preserves expected data |
| T003 | M0 | T002 | Build task schema, Make interface, CI and artifact manifests; missing suites and intentionally broken qualification fail |
| T004 | M0 | T003 | Implement independent reviewer runner and branch/queue runbook; exact-SHA report, stale evidence rejection and failure propagation demonstrated |
| T005 | M1 | T002, T003 | Project/context identity, membership, activation and Software blueprint; deny unauthorized reads/writes and snapshot session bindings |
| T006 | M1 | T005 | Both host integrations, Claude SDK bridge, capture spool and privacy; restart/duplicate/partial-session tests and real-host smoke evidence |
| T007 | M1 | T005 | Versioned sources, transformations, DAG/reverse index and jobs; independent support, cycle rejection and resumability verified |
| T008 | M1 | T006, T007 | Extraction/synthesis, OpenSearch hybrid retrieval, Cypher/RDF projections; qualified evidence retrieval and projection rebuilds verified |
| T009 | M1 | T008 | Correction and full rollback; preview, 30-hop/shared-descendant cases, concurrent synthesis and reconciled completion verified |
| T010 | M2 | T004, T009 | Codex gym runner and isolated worlds; real multi-persona software scenario with observable production capture/tools |
| T011 | M2 | T010 | Treatment/control execution, artifact capture, grading and budget termination; first honest baseline comparison and follow-up task |
| T012 | M3 | T009 | Complete lifecycle, search modes, collaboration controls, steering, blueprint catalog, archive/history and privacy rescans |
| T013 | M3 | T011, T012 | Briefing/Impact/feedback and daily/on-demand Night Shift; source-grounded insight and correction-policy coverage |
| T014 | M3 | T009, T012 | Bounded/semantic rollback with residuals, escalation and deletion accounting; full AT-01–20 coverage |
| T015 | M3/M4 | T011, T013, T014 | Numeric, ETL, BI, research and aging scenarios; external oracles, repeated comparisons and holdout discipline |
| T016 | M4 | T015 | Recovery/load/concurrency, migrations and restore; measured budgets and failure evidence |
| T017 | M5 | T016 | AWS IaC and real service adapters; authorized deploy, contract/E2E suite, cost/retention and teardown evidence |
| T018 | M6 | T016 | Separate formal-reasoning design and implementation plan; no engine work until requirements and justification tests are adopted |

T002 can use provisional adapter configurations established by T001; the factory
should not wait for every future AWS decision. T005–T009 should be integrated in
small slices rather than developed as isolated subsystems that only meet at the end.
