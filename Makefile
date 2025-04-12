default: help

.PHONY: help
help:
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Run "make fmt" and "make lint" before committing code.'
	@echo ''
	@echo 'Commands:'
	@echo 'build  Builds the code.'
	@echo 'test   Test the code.'
	@echo 'run    Runs the code.'
	@echo ''
	@echo 'build-docker  Build the docker image.'
	@echo 'dev           Runs the code in development mode with hot reloading.'
	@echo 'down          Down the docker containers.'
	@echo 'logs          Get the logs.'
	@echo ''
	@echo 'fmt           Format code.'
	@echo 'fmt/check     Check for formatting errors.'
	@echo 'install       Install the necessary rust components.'
	@echo 'lint          Fix linting issues.'
	@echo 'lint/check    Check for bad coding practices.'




.PHONY: build
build:
	 cargo build

.PHONY: run
run:
	 cargo run

.PHONY: test
test:
	cargo test




.PHONY: build-docker
build-docker:
	docker compose -f infra/docker-compose.yml build

.PHONY: dev
dev:
	docker compose -f infra/docker-compose.yml -f infra/docker-compose.dev.yml up --watch --build

.PHONY: down
down:
	docker compose -f infra/docker-compose.yml down

.PHONY: logs
logs:
	docker compose -f infra/docker-compose.yml logs --follow




.PHONY: install
install:
	rustup component add clippy
	rustup component add rustfmt

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: fmt/check
fmt/check:
	cargo fmt --check

.PHONY: lint
lint:
	cargo clippy --fix

.PHONY: lint/check
lint/check:
	cargo clippy

