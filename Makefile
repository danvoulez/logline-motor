# === File: logline-motor/Makefile ===
#
#    Description: Comandos de conveniência para build, teste e lint de todo o workspace.
#    LogLine Motor
#    Author: @danvoulez
#    License: Apache-2.0
#    Version: v1.0.0
#    Date: 2025-08-07
#    Repository: https://git.danvoulez/loglinemotor
#    Contact: dan@danvoulez.com
#

.PHONY: preflight build-local test-all lint

preflight:
	rustup toolchain install --file rust-toolchain.toml

build-local: preflight
	cargo build --workspace --release

lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings

test-all: lint
	cargo test --workspace