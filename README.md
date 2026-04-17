# pulumi-rust

A Rust SDK for [Pulumi](https://www.pulumi.com/) — idiomatic, async-first Rust bindings for the Pulumi resource model, with schema-driven code generation, a fully functional gRPC protocol layer, and a Pulumi language host.

## Status

**Working end-to-end.** The full stack is implemented: SDK → codegen → language host → generated providers. Programs run against a live Pulumi engine via `pulumi up`.

## What's Implemented

### Crates

| Crate | Purpose |
|---|---|
| `pulumi-sdk` | Core SDK: `Output<T>`, `Input<T>`, `Asset`/`Archive`, serialization, gRPC client, `Context`, `run()` |
| `pulumi-proto` | tonic-generated gRPC stubs compiled from vendored Pulumi proto files |
| `pulumi-codegen` | Schema-driven code generator: reads Pulumi JSON schema, emits Rust crate |
| `pulumi-codegen-cli` | CLI wrapper for code generation (`pulumi-codegen-rust generate`) |
| `pulumi-language-rust` | Language host binary — bridges Pulumi CLI ↔ Rust programs |
| `pulumi-sdk-test` | Test utilities: mock gRPC monitor + integration test harness |
| `pulumi-random` | Hand-written provider bindings for `@pulumi/random` |
| `pulumi-random-generated` | Auto-generated `@pulumi/random` from schema |
| `pulumi-azure-native-generated` | Auto-generated `azure-native` with feature-gated modules |

### Key types

- **`Output<T>`** — async value with dependency tracking, secret flag, and `.apply()` chaining backed by `tokio::sync::watch`
- **`Input<T>`** — `Value(T) | Output(Output<T>)` for resource arguments
- **`Asset` / `Archive` / `AssetOrArchive`** — file, string, and remote assets with Pulumi wire format serialization
- **`Context`** — per-run state, resource registration, stack exports
- **`run()` / `run_async()`** — boots tokio, connects to the Pulumi monitor/engine over gRPC, registers the root Stack resource, calls your program closure, then flushes stack outputs

### Code generation

The `pulumi-codegen` crate reads a Pulumi provider schema JSON and generates a complete Rust crate:

- **Resources** → Rust structs with `Args` types and `::new()` constructors
- **Functions** → `invoke_*()` async functions
- **Complex types** — object types, enums with string/integer values
- **Feature flags** — each top-level module is a Cargo feature (opt-in, `full` enables all)
- **Full azure-native** — 245K+ source files, compiles clean with `--features full`

### Language host

The `pulumi-language-rust` binary implements the Pulumi language host protocol:

- **`Run`** — smart binary discovery (checks release/debug builds, auto-builds if missing)
- **`InstallDependencies`** — streams `cargo build --release` output back to CLI
- **`GetRequiredPackages`** — parses `Cargo.toml` for `pulumi-*` dependencies
- **`Template`** — generates skeleton project for `pulumi new rust`

### Testing

- **Mock testing** — `test_with_mocks()` starts real gRPC mock servers, runs your program in-process
- **Integration testing** — `PulumiTestStack` with `up()` / `preview()` / `destroy()` for end-to-end validation

## Project Structure

```
crates/
  pulumi-sdk/                     # Core SDK
  pulumi-proto/                   # gRPC stubs (compiled from proto/)
  pulumi-codegen/                 # Schema → Rust code generator
  pulumi-codegen-cli/             # CLI for code generation
  pulumi-language-rust/           # Pulumi language host
  pulumi-sdk-test/                # Mock + integration test framework
  pulumi-random/                  # Hand-written random provider
  pulumi-random-generated/        # Auto-generated random provider
  pulumi-azure-native-generated/  # Auto-generated azure-native provider
examples/
  random-strings/                 # Two RandomStrings with Output dependency
  azure-resource-group/           # Azure resource group with tags
proto/pulumi/                     # Vendored Pulumi proto files
docs/                             # Design docs & research notes
```

## Quick Start

### Prerequisites

- Rust stable (1.75+)
- `protoc` (for the proto build step)
- Pulumi CLI (for running programs)

```bash
# macOS
brew install protobuf pulumi

# Ubuntu / Debian
apt-get install -y protobuf-compiler
curl -fsSL https://get.pulumi.com | sh
```

### Create a new project

```bash
mkdir my-infra && cd my-infra
pulumi new rust   # uses the language host template
```

Or manually:

```toml
# Cargo.toml
[dependencies]
pulumi-sdk = { path = "path/to/pulumi-rust/crates/pulumi-sdk" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

```rust
// src/main.rs
use pulumi_sdk::{run, Output};

fn main() -> anyhow::Result<()> {
    run(|ctx| {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(ctx.export("greeting", Output::known(serde_json::json!("Hello from Rust!"))));
        Ok(())
    })
}
```

### Generate a provider

```bash
# Download a schema
curl -sL https://raw.githubusercontent.com/pulumi/pulumi-random/master/provider/cmd/pulumi-resource-random/schema.json -o random-schema.json

# Generate Rust crate
cargo run -p pulumi-codegen-cli -- generate --schema random-schema.json --out my-random-provider
```

### Using azure-native with feature flags

```toml
[dependencies]
pulumi-azure-native = { path = "crates/pulumi-azure-native-generated", features = ["resources"] }
# Or enable everything:
# pulumi-azure-native = { path = "crates/pulumi-azure-native-generated", features = ["full"] }
```

### Running examples

```bash
# Random strings example
cargo build -p random-strings
cd examples/random-strings
pulumi login --local
pulumi stack init dev
pulumi up --yes

# Azure resource group example (requires Azure credentials)
cargo build -p azure-resource-group
cd examples/azure-resource-group
pulumi stack init dev
pulumi config set azure-native:location westeurope
pulumi up --yes
```

### Writing tests

```rust
use pulumi_sdk_test::{test_with_mocks, Mocks};

struct MyMocks;

impl Mocks for MyMocks {
    fn new_resource(
        &self, _type: &str, name: &str, inputs: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), String> {
        Ok((format!("{name}-id"), inputs.clone()))
    }
}

#[tokio::test]
async fn test_my_stack() {
    let result = test_with_mocks(MyMocks, "test", "dev", |ctx| async move {
        ctx.export("hello", pulumi_sdk::Output::known(serde_json::json!("world"))).await;
        Ok(())
    }).await.unwrap();
    assert_eq!(result.exports["hello"], "world");
}
```

## Development

Common tasks are available via `make`:

```bash
make help            # Show all available targets
make check           # Run fmt + lint + test (CI equivalent)
make fmt             # Format all code
make lint            # Clippy with deny-warnings
make test            # Run workspace tests
make build           # Debug build
make build-release   # Release build
make install-check   # Verify protoc, pulumi, cargo are installed
make doc             # Build and open rustdoc
```

Or run cargo directly:

```bash
# Format
cargo fmt --all

# Lint (zero warnings policy)
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --workspace --lib

# Build everything
cargo build --workspace

# Generate azure-native (requires schema at /tmp/azure-native-schema.json)
cargo run -p pulumi-codegen-cli -- generate --schema /tmp/azure-native-schema.json --out crates/pulumi-azure-native-generated
```

**Conventions:**
- No `unwrap()` / `expect()` in library code — use `?` and `thiserror`
- `BTreeMap` required for `prost_types::Struct` fields (protobuf ordering)
- Language host binds a random port and prints it to stdout for CLI discovery
- Generated providers use `default = []` features — consumers opt in to modules they need

## Documentation

- [Getting Started](docs/getting-started.md) — first project walkthrough
- [Architecture (v1)](docs/architecture.md) — original design: protocol, key types, RPC sequence
- [Architecture (v2)](docs/architecture-v2.md) — full provider support design document
- [Packaging Strategy](docs/packaging-strategy.md) — crates.io publishing plan

## License

Apache-2.0
