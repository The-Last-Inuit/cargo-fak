.PHONY: help build test check fmt fmt-check clippy ci clean

help:
	@printf "Targets:\n"
	@printf "  build      Build the project\n"
	@printf "  test       Run tests\n"
	@printf "  check      Type-check without producing artifacts\n"
	@printf "  fmt        Format code\n"
	@printf "  fmt-check  Verify formatting\n"
	@printf "  clippy     Run clippy lints\n"
	@printf "  ci         Run the CI command set\n"
	@printf "  clean      Remove build artifacts\n"

build:
	cargo build

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

ci:
	cargo build --verbose
	cargo test --verbose
	cargo fmt -- --check

clean:
	cargo clean
