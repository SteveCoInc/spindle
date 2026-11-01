# Requirements and acceptance coverage

This maps the supplied spec to the proposed delivery plan. **All implementation
and test statuses are unimplemented/unrun.** A milestone assignment is not evidence
of delivery. M0 will make this mapping machine-checkable against task/test IDs.

The user confirmed all non-formal-reasoning features remain in scope. Local
Context/general memory, both agent hosts, Rust, the factory and gym are additional
requirements from the conversation; the existing-service premise is superseded.
The latest user instruction defers dedicated graph engines and Cypher/SPARQL to
formal reasoning. RET-05/06 and graph-related acceptance scenarios use PostgreSQL
entity/relationship tables for the prototype; literal Neptune materialization is
not an M1 obligation. This is an explicit storage amendment, not a dropped behavior.

## Functional requirements

| Requirement | Target | Planned verification |
|---|---|---|
| PRJ-01 | M1 | Project API and both-host lifecycle E2E |
| PRJ-02 | M1 | Project API and both-host lifecycle E2E |
| PRJ-03 | M1 | Project API and both-host lifecycle E2E |
| PRJ-04 | M1 | Project API and both-host lifecycle E2E |
| PRJ-05 | M1 | Project API and both-host lifecycle E2E |
| PRJ-06 | M1 | Project API and both-host lifecycle E2E |
| PRJ-07 | M1 | Project API and both-host lifecycle E2E |
| PRJ-08 | M3 | Project API and both-host lifecycle E2E |
| PRJ-09 | M3 | Project API and both-host lifecycle E2E |
| MEM-01 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-02 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-03 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-04 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-05 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-06 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-07 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-08 | M1 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-09 | M3 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| MEM-10 | M3 | Extraction fixtures, evidence/DAG invariants, temporal and history tests |
| RET-01 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| RET-02 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| RET-03 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| RET-04 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| RET-05 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| RET-06 | M1 | Real hybrid search, ACL and PostgreSQL relationship contracts |
| ACL-01 | M1 | Multi-identity authorization and private-trace isolation tests |
| ACL-02 | M3 | Multi-identity authorization and private-trace isolation tests |
| ACL-03 | M1 | Multi-identity authorization and private-trace isolation tests |
| ACL-04 | M1 | Multi-identity authorization and private-trace isolation tests |
| PRI-01 | M1 | Policy precedence, pattern and retrospective remediation tests |
| PRI-02 | M1 | Policy precedence, pattern and retrospective remediation tests |
| PRI-03 | M3 | Policy precedence, pattern and retrospective remediation tests |
| PRI-04 | M3 | Policy precedence, pattern and retrospective remediation tests |
| AUD-01 | M1 | Inspection/correction and dependent-rebuild E2E |
| AUD-02 | M1 | Inspection/correction and dependent-rebuild E2E |
| ROL-01 | M1 | Rollback closure, availability, retry and independent-support tests |
| ROL-02 | M1 | Rollback closure, availability, retry and independent-support tests |
| ROL-03 | M3 | Rollback closure, availability, retry and independent-support tests |
| ROL-04 | M3 | Rollback closure, availability, retry and independent-support tests |
| ROL-05 | M1 | Rollback closure, availability, retry and independent-support tests |
| ROL-06 | M1 | Rollback closure, availability, retry and independent-support tests |
| ROL-07 | M1 | Rollback closure, availability, retry and independent-support tests |
| VAL-01 | M3 | Event attribution, briefing/impact and quality-feedback tests |
| VAL-02 | M3 | Event attribution, briefing/impact and quality-feedback tests |
| VAL-03 | M3 | Event attribution, briefing/impact and quality-feedback tests |
| VAL-04 | M3 | Event attribution, briefing/impact and quality-feedback tests |
| NGT-01 | M3 | Clock-controlled scheduler, labeled insight and policy tests |
| NGT-02 | M3 | Clock-controlled scheduler, labeled insight and policy tests |
| NGT-03 | M3 | Clock-controlled scheduler, labeled insight and policy tests |
| NGT-04 | M3 | Clock-controlled scheduler, labeled insight and policy tests |
| BLU-01 | M3 (Software in M1) | Versioned defaults, project overrides and blueprint evaluation fixtures |
| BLU-02 | M3 (Software in M1) | Versioned defaults, project overrides and blueprint evaluation fixtures |
| BLU-03 | M3 (Software in M1) | Versioned defaults, project overrides and blueprint evaluation fixtures |
| RSN-01 | M6 | Formal claim/justification and cross-representation tests (deferred) |
| RSN-02 | M6 | Formal claim/justification and cross-representation tests (deferred) |
| RSN-03 | M6 | Formal claim/justification and cross-representation tests (deferred) |
| RSN-04 | M6 | Formal claim/justification and cross-representation tests (deferred) |
| RSN-05 | M6 | Formal claim/justification and cross-representation tests (deferred) |
| RSN-06 | M6 | Formal claim/justification and cross-representation tests (deferred) |

## Spec acceptance scenarios

| Scenario | Target |
|---|---|
| AT-01 — Zero-config creation | M1 |
| AT-02 — Persistent, visible activation | M1 |
| AT-03 — Session-bound source attribution | M1 |
| AT-04 — Teammate value from overlay retrieval | M1 |
| AT-05 — Independent multi-author evidence | M1 |
| AT-06 — Temporal contradiction handling | M1 |
| AT-07 — Global privacy override | M1 |
| AT-08 — After-the-fact session correction | M1 |
| AT-09 — Full multi-author rollback | M1 |
| AT-10 — Arbitrary-depth rollback | M1 |
| AT-11 — Bounded cleanup caveat | M3 |
| AT-12 — Deep semantic leakage match | M3 |
| AT-13 — Search availability during rollback | M1 |
| AT-14 — Correction permissions | M3 |
| AT-15 — Briefing and Impact | M3 |
| AT-16 — Night Shift produces an inference safely | M3 |
| AT-17 — Blueprint customization | M3 |
| AT-18 — Retrospective privacy policy scan | M3 |
| AT-19 — Project-only security boundary | M1 |
| AT-20 — Trace-to-output deletion accounting | M3 |
| AT-21 — Future structured claim remains optional | M6 (deferred) |
| AT-22 — Future independent reasoning justifications | M6 (deferred) |
| AT-23 — Future scoped contradiction and explicit negation | M6 (deferred) |
| AT-24 — Future cross-representation rollback | M6 (deferred) |

AT-08 first exposes full rollback in M1; all choices arrive in M3. AT-14 has
author-isolation tests from M1 and full collaborative-mode coverage in M3. AT-20
requires an initial retention contract in M1 and complete deletion accounting in M3.
AT-21–24 are deferred formal tests; preserving optional extension boundaries in
M1 is not the same as implementing them.

## Additional acceptance obligations

| ID | Obligation | Target |
|---|---|---|
| EXT-01 | Standalone Context instance and general-context ingest/search, not a nonexistent upstream service | M1 |
| EXT-02 | Codex and Claude Code/SDK capture, status, tools and restart tested independently | M1 |
| EXT-03 | PostgreSQL entity/lineage adjacency, stable IDs, project authorization and consistent retraction; dedicated graph engines/Cypher/SPARQL deferred | M1; language endpoints reconsidered M6 |
| EXT-04 | Clean local bootstrap and same deterministic checks locally/CI | M0/M1 |
| EXT-05 | Independent reviewer before PR and in CI; exact-commit evidence; failing checks prevent integration | M0 |
| EXT-06 | Multiple real Codex personas, shared project, isolated sessions, observable capture and injection | M2 |
| EXT-07 | Budget/time termination, $100/four-hour ceilings, private artifact retention and honest incomplete status | M2 |
| EXT-08 | Matched enabled/disabled arms; externally checked outcomes; no grader leakage | M2/M4 |
| EXT-09 | Numeric/ETL/BI/research and memory-aging scenarios; failure-driven improvement loop | M3/M4 |
| EXT-10 | Actual AWS adapter conformance, including DSQL if selected, identity and restore/teardown verification | M5 |
| EXT-11 | Rust application/factory/gym-owned code; external framework runtime boundary explicit | M0 onward |
| EXT-12 | NeMo/Docker and optional Daytona qualification: shared project lifecycle, capture, isolated grading, cleanup and total cost | M0 spike / M2 acceptance |
| EXT-13 | Multi-worker factory: atomic claims, isolation, fencing/recovery, bounded account capacity, independent review and one merge path | M0 foundation; qualification before 3–4 workers |

## Test layers

Unit and property tests exercise invariants. Adapter contracts use real storage.
Scripted end-to-end tests cover the entire API/MCP/job flow with deterministic model
fixtures. Host smoke tests establish actual lifecycle integration. Replay tests
diagnose captured trajectories. Live gyms measure model-dependent quality and
downstream usefulness. Fault/load tests establish operational limits. None of
these layers substitutes for the others.

Authorization and completed-full-rollback invariants are zero-tolerance gates in
the tested scope. Semantic quality targets need pilot calibration, labeled data
and uncertainty estimates; absence of a detected leak is not proof of universal
semantic non-leakage. Every report separates passed, failed, skipped, unrun and
inconclusive outcomes.
