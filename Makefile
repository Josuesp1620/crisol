# Short commands for the Docker workflow. Nothing is installed on the host.
export HOST_UID := $(shell id -u)
export HOST_GID := $(shell id -g)

COMPOSE := docker compose run --rm
WINDOWS_TARGET := x86_64-pc-windows-msvc
CORE_PACKAGES := -p crisol-model -p crisol-catalog -p crisol-commands -p crisol-engine -p crisol-codegen -p crisol-cli
BUILD_JOBS ?= 8

.PHONY: help images test test-core check-examples generate-examples test-all lint format windows-check windows-build run-cli version release-notes clean

help:
	@echo "make images                 Build the Docker images"
	@echo "make test                   Run the tests of the crates without a user interface"
	@echo "make check-examples        Validate the example forms with the command line"
	@echo "make test-all               Run every test, the GPUI crates included"
	@echo "make lint                   Check formatting and clippy on every crate"
	@echo "make format                 Format the Rust code"
	@echo "make windows-check          Lint the whole workspace for Windows"
	@echo "make windows-build          Build crisol.exe for Windows (debug, cross-compiled)"
	@echo "make version VERSION=x.y.z  Set the version of every crate"
	@echo "make clean                  Remove build output"

images:
	docker compose build

test: test-core check-examples

test-core:
	$(COMPOSE) core cargo test $(CORE_PACKAGES)

check-examples:
	$(COMPOSE) core cargo run -q -p crisol-cli -- check examples/customer/customer.crisol

# Regenerates the example form after editing customer.crisol.
generate-examples:
	$(COMPOSE) core cargo run -q -p crisol-cli -- generate examples/customer/customer.crisol --out examples/customer/src/forms

test-all:
	$(COMPOSE) -e CARGO_BUILD_JOBS=$(BUILD_JOBS) core cargo test --workspace

lint:
	$(COMPOSE) -e CARGO_BUILD_JOBS=$(BUILD_JOBS) core sh -c "cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings"

format:
	$(COMPOSE) core cargo fmt --all

windows-check:
	$(COMPOSE) -e CARGO_BUILD_JOBS=$(BUILD_JOBS) windows cargo xwin clippy --workspace --target $(WINDOWS_TARGET) -- -D warnings

windows-build:
	$(COMPOSE) -e CARGO_BUILD_JOBS=$(BUILD_JOBS) windows cargo xwin build -p crisol-designer --target $(WINDOWS_TARGET)
	@echo "Executable: target/$(WINDOWS_TARGET)/debug/crisol.exe"

# Sets the workspace version; then move the CHANGELOG "Unreleased" entries under it,
# commit, and push the tag vVERSION to build the Windows release on GitHub Actions.
version:
	@test -n "$(VERSION)" || (echo "Usage: make version VERSION=0.2.0" && exit 1)
	sed -i '0,/^version = ".*"/s//version = "$(VERSION)"/' Cargo.toml
	@echo "Version set to $(VERSION). Next: update CHANGELOG.md, commit, then:"
	@echo "  git tag v$(VERSION) && git push origin main v$(VERSION)"

clean:
	rm -rf target
