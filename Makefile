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
ifeq ($(heapsize),)
	cargo run --release -p benchmarks --bin bench -- run -n $(name)
else
	cargo run --release -p benchmarks --bin bench -- run -n $(name) --heap-size $(heapsize)
endif
endif

.PHONY: missing-bench
missing-bench:
	cargo run -p benchmarks -- run -s

.PHONY: comp-bench
comp-bench:
	cargo run codegen benchmarks/suite/$(name)/$(name).sc x86-64

.PHONY: update-expected
update-expected:
	UPDATE_GOLDENFILES=1 cargo test --workspace
