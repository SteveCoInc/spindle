# Spindle

Shared, traceable project memory for coding agents: capture useful knowledge,
retrieve it across teammates' sessions, and correct or retract it with evidence.

**Status: design proposal, not a runnable application.** The repository currently
contains the source specification and a proposed implementation plan. No service,
plugin, gym, CI gate, or deployment has been implemented or validated yet.

Start with [the delivery plan](docs/planning/plan.md), then read:

- [Architecture and storage choices](docs/planning/architecture.md)
- [Codex software factory](docs/planning/software-factory.md)
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

New Codex sessions should read [AGENTS.md](AGENTS.md). Commands described in the
plans are future interfaces, not currently executable instructions.
