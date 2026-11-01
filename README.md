# Spindle

Shared, traceable project memory for coding agents: capture useful knowledge,
retrieve it across teammates' sessions, and correct or retract it with evidence.

**Status: M0 foundation implemented and locally tested; live-agent validation blocked on network access.** The Rust
workspace includes a foundation HTTP service, PostgreSQL migration, OpenSearch
smoke check, TOML task validator, durable claim ledger, isolated task runner,
clean-commit qualification and structured Codex reviewer. Product memory features
and the NeMo gym are still planned.

Start development with [the local runbook](docs/runbooks/local-development.md)
and operate the [initial factory](docs/runbooks/factory.md). With Rust 1.93.0,
Docker Compose, Git and Make installed:

```sh
make bootstrap
make up
make check test test-integration
make serve
```

See [verification evidence](docs/runbooks/m0-evidence.md) and the remaining
[manual actions](docs/runbooks/manual-actions.md). No schedule is needed yet.

CI definitions cover Rust and storage checks. Independent review requires a
working Codex login and network access. The runner does not yet dispatch on a
schedule or merge changes automatically.

Start with [the delivery plan](docs/planning/plan.md), then read:

- [Architecture and storage choices](docs/planning/architecture.md)
- [Codex software factory](docs/planning/software-factory.md)
- [Parallel Codex workers and orchestration options](docs/planning/factory-orchestration.md)
- [Evaluation gym](docs/planning/gym.md)
- [Requirements and acceptance coverage](docs/planning/coverage.md)
- [Manual setup and outstanding decisions](docs/planning/manual-setup.md)
- [Research and references](docs/planning/references.md)
- [Initial task backlog](tasks/README.md)
- [Original product specification](docs/specs/project-spindle-spec-v0.2.md)

The selected product stack is **Rust + PostgreSQL + OpenSearch**. Lineage and
entity links use PostgreSQL tables; dedicated graph engines wait until formal
reasoning. Aurora DSQL is a candidate AWS store to qualify later. The recommended
gym foundation is [NeMo Gym with local Docker and optional Daytona](docs/planning/gym-platform-evaluation.md),
subject to the integration spike.

The proposed order is factory foundation → usable local product → first real-agent
gym → complete non-formal-reasoning features → AWS → optional formal reasoning.
The gym and factory are development infrastructure; the product remains agent-native.

New Codex sessions should read [AGENTS.md](AGENTS.md). The runbooks describe implemented commands; commands appearing only in planning
documents may still be future interfaces.
