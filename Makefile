.PHONY: vendor
	cargo vendor

.PHONY: test
test: vendor
	cargo test --all --no-fail-fast

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

.PHONY: update-expected
update-expected: vendor
	UPDATE_GOLDENFILES=1 cargo test --workspace

.PHONY: package-quick
package-quick:
	@cargo package --workspace --no-verify --exclude testsuite --exclude axcut_examples --exclude scc-macro-utils --exclude scc-core-macros --exclude axcut_macros

.PHONY: package
package:
	@cargo package --workspace --exclude testsuite --exclude axcut_examples --exclude scc-macro-utils --exclude scc-core-macros --exclude axcut_macros
