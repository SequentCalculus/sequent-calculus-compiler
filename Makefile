.PHONY: vendor
	cargo vendor

.PHONY: test
test: vendor
	cargo test --all --no-fail-fast
	$(MAKE) -C compiling-sc-bench

.PHONY: check
check: vendor
	cargo fmt --all -- --check
	cargo clippy --all

.PHONY: install
install: vendor
	cargo install --path app --force

.PHONY: uninstall
uninstall: vendor
	cargo uninstall scc

.PHONY: doc
doc:
	cargo doc --workspace --document-private-items

.PHONY: coverage
coverage: vendor
	@echo "Make sure to install via cargo install cargo-llvm-cov first"
	cargo llvm-cov --workspace --html
	cargo llvm-cov --workspace --open

.PHONY: bench
bench: vendor
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
missing-bench: vendor
	cargo run -p benchmarks -- run -s

.PHONY: comp-bench
comp-bench: vendor
	cargo run codegen benchmarks/suite/$(name)/$(name).sc x86-64

.PHONY: update-expected
update-expected: vendor
	UPDATE_GOLDENFILES=1 cargo test --workspace
