.PHONY: test
test:
	cargo test --all --no-fail-fast

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy

.PHONY: install
install:
	cargo install --path app --force
