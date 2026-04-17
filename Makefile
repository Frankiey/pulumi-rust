.PHONY: all check fmt lint test build build-release clean codegen-random codegen-azure-native help install-check

# Default: run all checks
all: check

## help: Show this help message
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development:"
	@echo "  check              Run fmt, lint, and test (CI equivalent)"
	@echo "  fmt                Format all code with cargo fmt"
	@echo "  lint               Run clippy with deny warnings"
	@echo "  test               Run all workspace tests"
	@echo "  build              Build all workspace crates (debug)"
	@echo "  build-release      Build all workspace crates (release)"
	@echo "  clean              Remove build artifacts"
	@echo ""
	@echo "Code Generation:"
	@echo "  codegen-random     Generate random provider from schema"
	@echo "  codegen-azure      Generate azure-native provider from schema"
	@echo ""
	@echo "Utilities:"
	@echo "  install-check      Verify required tools are installed"
	@echo "  doc                Build and open rustdoc"
	@echo "  doc-build          Build rustdoc without opening"

# ── Development ──────────────────────────────────────────────

## check: Run all quality gates (mirrors CI)
check: fmt-check lint test

## fmt: Format all code
fmt:
	cargo fmt --all

## fmt-check: Check formatting without modifying files
fmt-check:
	cargo fmt --all -- --check

## lint: Run clippy with deny-warnings policy
lint:
	cargo clippy --all-targets --all-features -- -D warnings

## test: Run all workspace tests
test:
	cargo test --workspace --lib

## test-all: Run all tests including integration tests
test-all:
	cargo test --workspace

## build: Debug build of the full workspace
build:
	cargo build --workspace

## build-release: Release build of the full workspace
build-release:
	cargo build --workspace --release

## clean: Remove target directory
clean:
	cargo clean

# ── Code Generation ──────────────────────────────────────────

RANDOM_SCHEMA ?= /tmp/random-schema.json
AZURE_SCHEMA  ?= /tmp/azure-native-schema.json

## codegen-random: Generate the random provider crate from schema
codegen-random: $(RANDOM_SCHEMA)
	cargo run -p pulumi-codegen-cli -- generate \
		--schema $(RANDOM_SCHEMA) \
		--out crates/pulumi-random-generated

## codegen-azure: Generate the azure-native provider crate from schema
codegen-azure: $(AZURE_SCHEMA)
	cargo run -p pulumi-codegen-cli -- generate \
		--schema $(AZURE_SCHEMA) \
		--out crates/pulumi-azure-native-generated

## fetch-random-schema: Download the random provider schema
fetch-random-schema:
	curl -sL https://raw.githubusercontent.com/pulumi/pulumi-random/master/provider/cmd/pulumi-resource-random/schema.json \
		-o $(RANDOM_SCHEMA)

# ── Documentation ────────────────────────────────────────────

## doc: Build and open rustdoc
doc:
	cargo doc --workspace --no-deps --open

## doc-build: Build rustdoc without opening
doc-build:
	cargo doc --workspace --no-deps

# ── Utilities ────────────────────────────────────────────────

## install-check: Verify that required build tools are installed
install-check:
	@echo "Checking required tools..."
	@command -v cargo  >/dev/null 2>&1 || { echo "✗ cargo not found  — install via https://rustup.rs"; exit 1; }
	@command -v protoc >/dev/null 2>&1 || { echo "✗ protoc not found — brew install protobuf / apt install protobuf-compiler"; exit 1; }
	@command -v pulumi >/dev/null 2>&1 || { echo "✗ pulumi not found — curl -fsSL https://get.pulumi.com | sh"; exit 1; }
	@echo "✓ All required tools found"
	@echo "  cargo  $$(cargo --version)"
	@echo "  protoc $$(protoc --version)"
	@echo "  pulumi $$(pulumi version)"
