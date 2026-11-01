SHELL := /bin/bash
.DEFAULT_GOAL := help
CARGO ?= cargo
COMPOSE := docker compose -p $(or $(SPINDLE_STACK),spindle-dev) -f infra/compose.yaml
FACTORY := $(CARGO) run --quiet --locked -p spindle-factory --
.PHONY: help bootstrap check test up down migrate doctor test-integration serve qualify review factory-status factory-pause factory-resume
help:
	@echo 'bootstrap check test up down migrate doctor test-integration serve worker project rebuild-search mcp qualify review factory-status factory-pause factory-resume gym-bootstrap gym-contracts'
bootstrap:
	$(CARGO) fetch --locked
check:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --workspace --all-targets --locked -- -D warnings
	$(FACTORY) validate
test:
	$(CARGO) test --workspace --locked
up:
	$(COMPOSE) up -d --wait --wait-timeout 300
	$(MAKE) migrate
down:
	$(COMPOSE) down
migrate:
	$(CARGO) run --quiet --locked -p spindle-api -- migrate
doctor:
	$(FACTORY) doctor
	$(CARGO) run --quiet --locked -p spindle-api -- check
test-integration:
	$(CARGO) run --quiet --locked -p spindle-api -- integration
serve:
	$(CARGO) run --quiet --locked -p spindle-api -- serve
qualify:
	$(FACTORY) qualify
review:
	$(FACTORY) review
factory-status:
	$(FACTORY) status
factory-pause:
	$(FACTORY) pause
factory-resume:
	$(FACTORY) resume

.PHONY: worker project mcp
worker:
	$(CARGO) run --quiet --locked -p spindle-api -- worker
project:
	$(CARGO) run --quiet --locked -p spindle-api -- project
mcp:
	$(CARGO) run --quiet --locked -p spindle-api -- mcp

# Optional external framework spike. No model calls or Daytona resources.
NEMO_GYM_DIR ?= $(CURDIR)/.factory/vendor/nemo-gym
NEMO_GYM_REV := 0c7ac5119c99e0aa0dfeda7be59157c589680ffd
UV ?= uv
.PHONY: gym-bootstrap gym-contracts
gym-bootstrap:
	@test -d "$(NEMO_GYM_DIR)/.git" || git clone --no-checkout https://github.com/NVIDIA-NeMo/Gym.git "$(NEMO_GYM_DIR)"
	git -C "$(NEMO_GYM_DIR)" checkout --detach $(NEMO_GYM_REV)
	cd "$(NEMO_GYM_DIR)" && $(UV) sync --frozen --extra dev --no-default-groups --system-certs
gym-contracts:
	@test "$$(git -C "$(NEMO_GYM_DIR)" rev-parse HEAD)" = "$(NEMO_GYM_REV)"
	mkdir -p "$(CURDIR)/artifacts/gym"
	cd "$(NEMO_GYM_DIR)" && .venv/bin/python -m pytest responses_api_agents/codex_agent/tests/test_exec_transport.py responses_api_agents/codex_agent/tests/test_sandbox_sessions.py responses_api_agents/codex_agent/tests/test_sandbox_runner.py responses_api_agents/codex_agent/tests/test_bootstrap.py --junitxml="$(CURDIR)/artifacts/gym/nemo-contracts.xml"

.PHONY: rebuild-search
rebuild-search:
	$(CARGO) run --quiet --locked -p spindle-api -- rebuild-search
