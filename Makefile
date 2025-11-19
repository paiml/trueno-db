# Trueno-DB Makefile
# Toyota Way: Extreme TDD with quality gates

# Quality directives (bashrs enforcement)
.SUFFIXES:
.DELETE_ON_ERROR:
.ONESHELL:

.PHONY: help build test bench lint check coverage mutants tdg clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

## Development Commands

build: ## Build the project
	cargo build

build-release: ## Build release version
	cargo build --release

test: ## Run all tests
	cargo test --all-features

test-verbose: ## Run tests with verbose output
	cargo test --all-features -- --nocapture

bench: ## Run benchmarks
	cargo bench

## Quality Gates (EXTREME TDD)

lint: ## Run clippy with zero tolerance
	cargo clippy --all-features -- -D warnings

check: lint test ## Run basic quality checks

coverage: ## Generate coverage report (>90% required, <10 min target)
	@echo "📊 Generating coverage report (target: >90%, <10 min)..."
	@# Temporarily disable mold linker (breaks LLVM coverage)
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	@cargo llvm-cov report --html --output-dir target/coverage/html
	@# Restore mold linker
	@test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true
	@echo "✅ Coverage report: target/coverage/html/index.html"
	@cargo llvm-cov report | grep TOTAL

mutants: ## Run mutation testing (target: ≥80% kill rate)
	cargo mutants --all-features

tdg: ## Run TDG analysis (target: ≥B+ / 85)
	pmat analyze tdg

quality-gate: lint test coverage ## Run full quality gate
	@echo "✅ All quality gates passed"

## Backend Equivalence Tests (Critical)

test-equivalence: ## Test GPU == SIMD == Scalar
	cargo test --test backend_equivalence --all-features

## WASM Build

wasm: ## Build for WASM target
	cargo build --target wasm32-unknown-unknown --release --features wasm

wasm-test: ## Test WASM build
	wasm-pack test --headless --firefox

## Benchmarking

bench-gpu: ## Benchmark GPU backend
	cargo bench --bench aggregations -- --gpu

bench-simd: ## Benchmark SIMD backend
	cargo bench --bench aggregations -- --simd

bench-comparison: ## Compare all backends
	cargo bench --bench backend_comparison

## Maintenance

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/ || exit 1
	rm -rf coverage/ || exit 1

update-trueno: ## Update trueno to latest version
	@echo "Checking latest trueno version..."
	cargo search trueno | head -1
	@echo "Current version:"
	cargo tree | grep trueno
	@echo "To update: cargo update trueno"

check-deps: ## Check dependency versions
	cargo tree | grep -E "(trueno|wgpu|arrow|parquet)"

## Docker (Future)

docker-build: ## Build Docker image
	docker build -t trueno-db:latest .

docker-test: ## Run tests in Docker
	docker run --rm trueno-db:latest cargo test

## CI/CD

ci: lint test coverage ## Run CI pipeline
	@echo "✅ CI pipeline passed"

pre-commit: lint test ## Pre-commit hook
	@echo "✅ Pre-commit checks passed"
