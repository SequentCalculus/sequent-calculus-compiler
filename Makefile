.PHONY: test
test:
	cargo test --all --no-fail-fast

check:
	cargo fmt --all -- --check
	cargo clippy

