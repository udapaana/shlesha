.PHONY: help build test clean fmt lint docs bench install-dev publish-test

# Default target
help:
	@echo "Shlesha Development Commands:"
	@echo "  make build        - Build all targets"
	@echo "  make test         - Run all tests"
	@echo "  make fmt          - Format code"
	@echo "  make lint         - Run clippy lints"
	@echo "  make docs         - Build documentation"
	@echo "  make bench        - Run benchmarks"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install-dev  - Install development dependencies"
	@echo "  make python       - Build Python wheel"
	@echo "  make wasm         - Build WASM package"
	@echo "  make publish-test - Publish to test registries"

# Build all targets
build:
	cargo build --all-features

# Run all tests
test:
	cargo test --all-features
	cargo test --doc

# Format code
fmt:
	cargo fmt

# Run lints
lint:
	cargo clippy --all-features -- -D warnings

# Build documentation
docs:
	cargo doc --all-features --no-deps --open

# Run benchmarks
bench:
	cargo bench

# Clean build artifacts
clean:
	cargo clean
	rm -rf pkg pkg-node
	rm -rf target/wheels

# Install development dependencies
install-dev:
	rustup component add rustfmt clippy
	cargo install cargo-tarpaulin cargo-audit wasm-pack
	pip install maturin pytest

# Build Python wheel
python:
	maturin build --features python

# Build WASM package
wasm:
	wasm-pack build --target web --out-dir pkg --features wasm

# Test publishing (dry run)
publish-test: fmt lint test
	@echo "Testing Python publish..."
	maturin build --features python
	@echo "Testing WASM publish..."
	wasm-pack build --target web --out-dir pkg --features wasm
	cd pkg && npm pack
	@echo "Testing crates.io publish..."
	cargo publish --dry-run