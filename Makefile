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

.PHONY: coverage
coverage:
	@echo "Make sure to install via cargo install cargo-llvm-cov first"
	cargo llvm-cov --workspace --html
	cargo llvm-cov --workspace --open

.PHONY: bench
bench:
	@echo "Make sure to install hyperfine first"
ifeq ($(name),)
	cargo run --release -p benchmarks -- run
else
	cargo run --release -p benchmarks --bin benchmarks -- run -n $(name)
endif

.PHONY: update-expected
update-expected:
	UPDATE_GOLDENFILES=1 cargo test --workspace
