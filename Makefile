.PHONY: fmt lint check

fmt:
	cargo fmt

lint:
	rustup component add clippy-preview
	cargo clippy

check:
	cargo check
