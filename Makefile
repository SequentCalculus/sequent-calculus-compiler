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
ifeq ($(name),)
	cargo run --release -p bench -- run
else
	cargo run --release -p bench --bin bench -- run -n $(name)
endif

.PHONY: report
report: 
ifeq ($(name),)
	cargo run -p bench --bin bench -- report 
else 
	cargo run -p bench --bin bench --report -n $(name)
endif


.PHONY: update-expected
update-expected:
	UPDATE_GOLDENFILES=1 cargo test --workspace
