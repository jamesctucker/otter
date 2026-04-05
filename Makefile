.PHONY: fmt check clippy test all

fmt:
	cargo fmt --all

check:
	cargo check --workspace

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace

all: fmt check clippy test
