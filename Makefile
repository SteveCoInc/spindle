SHELL := /bin/bash
.DEFAULT_GOAL := help
CARGO ?= cargo
COMPOSE := docker compose -p $(or $(SPINDLE_STACK),spindle-dev) -f infra/compose.yaml
FACTORY := $(CARGO) run --quiet --locked -p spindle-factory --
.PHONY: help bootstrap check test up down migrate doctor test-integration serve qualify review factory-status factory-pause factory-resume
help:
	@echo 'bootstrap check test up down migrate doctor test-integration serve qualify review factory-status factory-pause factory-resume'
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
test-integration: migrate
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
