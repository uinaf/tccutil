SHELL := /bin/bash
.DEFAULT_GOAL := verify
.DELETE_ON_ERROR:

VERIFY_JOBS ?= 3
VERIFY_FULL_JOBS ?= 4
VERIFY_CACHE_DIR := target/verify-cache
RUST_SOURCES := $(shell find src tests -type f -name '*.rs' -print)
VERIFY_INPUTS := Cargo.toml Cargo.lock rust-toolchain.toml Makefile src tests $(RUST_SOURCES)
FMT_STAMP := $(VERIFY_CACHE_DIR)/fmt
CLIPPY_STAMP := $(VERIFY_CACHE_DIR)/clippy
TEST_STAMP := $(VERIFY_CACHE_DIR)/test

.PHONY: verify verify-full verify-coverage verify-release

$(VERIFY_CACHE_DIR):
	mkdir -p $@

$(FMT_STAMP): $(VERIFY_INPUTS) | $(VERIFY_CACHE_DIR)
	@printf '%s\n' '→ cargo fmt --check'
	cargo fmt --check
	@touch $@

$(CLIPPY_STAMP): $(VERIFY_INPUTS) | $(VERIFY_CACHE_DIR)
	@printf '%s\n' '→ cargo clippy -- -D warnings'
	cargo clippy --quiet -- -D warnings
	@touch $@

$(TEST_STAMP): $(VERIFY_INPUTS) | $(VERIFY_CACHE_DIR)
	@printf '%s\n' '→ cargo test'
	cargo test --quiet
	@touch $@

verify:
	+$(MAKE) --no-print-directory --jobs=$(VERIFY_JOBS) $(FMT_STAMP) $(CLIPPY_STAMP) $(TEST_STAMP)
	@printf '%s\n' '✓ fast verification passed'

verify-coverage: | $(VERIFY_CACHE_DIR)
	@printf '%s\n' '→ cargo llvm-cov --fail-under-lines 75'
	cargo llvm-cov --fail-under-lines 75
	@touch $(TEST_STAMP)

verify-release:
	@printf '%s\n' '→ cargo build --release'
	cargo build --release --quiet

verify-full:
	@printf '%s\n' '→ provision coverage tooling'
	rustup component add llvm-tools-preview
	@command -v cargo-llvm-cov >/dev/null 2>&1 || cargo install cargo-llvm-cov --locked
	+$(MAKE) --no-print-directory --always-make --jobs=$(VERIFY_FULL_JOBS) \
		$(FMT_STAMP) $(CLIPPY_STAMP) verify-coverage verify-release
	@printf '%s\n' '✓ full verification passed'
