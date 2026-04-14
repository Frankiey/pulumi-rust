# pulumi-rust

A Rust SDK for [Pulumi](https://www.pulumi.com/) — idiomatic, async-first Rust bindings for the Pulumi resource model, with a fully functional gRPC protocol layer and provider bindings.

## Status

**Working end-to-end.** The stack below is implemented and the example program runs against a live Pulumi engine.

## What's Implemented

### Crates

| Crate | Purpose |
|---|---|
| `pulumi-proto` | tonic-generated gRPC stubs compiled from vendored Pulumi proto files |
| `pulumi-sdk` | Core SDK: `Output<T>`, serialization, gRPC client, `Context`, `run()` |
| `pulumi-random` | Provider bindings for `@pulumi/random` (`RandomString`) |
| `pulumi-language-rust` | Language host binary — bridges Pulumi CLI ↔ Rust programs |

### Key types

- **`Output<T>`** — async value with dependency tracking, secret flag, and `.apply()` chaining backed by `tokio::sync::watch`
- **`Input<T>`** — `Value(T) | Output(Output<T>)` for resource arguments
- **`Context`** — per-run state, resource registration, stack exports
- **`run()`** — boots tokio, connects to the Pulumi monitor/engine over gRPC, registers the root Stack resource, calls your program closure, then flushes stack outputs

### Wire protocol

All Pulumi special encodings are implemented in `pulumi-sdk::serialize`:

| Constant | Purpose |
|---|---|
| `UNKNOWN_SENTINEL` | Marks unknown output values during previews |
| `SECRET_SIG` | Wraps secret property values |
| `OUTPUT_SIG` | Represents output values with deps |

## Project Structure

```
crates/
  pulumi-proto/         # tonic gRPC stubs (compiled from proto/)
  pulumi-sdk/           # Core SDK (Output<T>, serialize, grpc_client, context)
  pulumi-random/        # @pulumi/random provider bindings
  pulumi-language-rust/ # Language host binary
examples/
  random-strings/       # Benchmark: two RandomStrings with Output dependency
proto/pulumi/           # Vendored Pulumi proto files
docs/
  architecture.md       # Design doc: crate layout, gRPC protocol, type system
```

## Example

The `random-strings` example replicates the core mechanic of the TypeScript reference program: two `RandomString` resources where the second resource's length is derived from the first's output via `.apply()`.

### Rust

```rust
use anyhow::Result;
use pulumi_random::{Input, RandomString, RandomStringArgs};
use pulumi_sdk::run;

fn main() -> Result<()> {
    run(|ctx| {
        let rt = tokio::runtime::Handle::current();

        // Resource A — fixed 8-character string, no special chars
        let string_a = rt.block_on(RandomString::new(
            ctx,
            "resource-a",
            RandomStringArgs {
                length: Input::Value(8),
                special: Some(false),
                ..Default::default()
            },
        ))?;

        // Resource B — length is an Output<i64> derived from A's result
        let length_b = string_a.result.apply(|val| {
            serde_json::json!(val.as_str().unwrap_or("").len() as i64 * 2)
        });

        let string_b = rt.block_on(RandomString::new(
            ctx,
            "resource-b",
            RandomStringArgs {
                length: Input::Output(length_b.apply(|v| v.as_i64().unwrap_or(16))),
                special: Some(true),
                ..Default::default()
            },
        ))?;

        rt.block_on(ctx.export("stringA", string_a.result));
        rt.block_on(ctx.export("stringB", string_b.result));
        Ok(())
    })
}
```

### Equivalent TypeScript (pulumi-learn reference)

```typescript
import * as random from "@pulumi/random";

const resourceA = new random.RandomString("resource-a", {
    length: 8,
    special: false,
});

const resourceB = new random.RandomString("resource-b", {
    length: resourceA.result.apply(s => s.length * 2),
    special: true,
});

export const stringA = resourceA.result;
export const stringB = resourceB.result;
```

### Running the example

```bash
# Build first
cargo build -p random-strings

# Run with Pulumi (local backend)
cd examples/random-strings
pulumi login --local
pulumi stack init dev
pulumi up --yes
# Outputs:
#   stringA: "xK9mPqRt"       (8 chars)
#   stringB: "xK9mPqRt..."    (16 chars)
```

## Getting Started

> Crates are not yet published to crates.io — reference them from source.

```toml
[dependencies]
pulumi-sdk    = { path = "crates/pulumi-sdk" }
pulumi-random = { path = "crates/pulumi-random" }
```

**Prerequisites:** Rust stable, `protoc` (for the proto build step).

```bash
# macOS
brew install protobuf

# Ubuntu / Debian
apt-get install -y protobuf-compiler
```

## Development

```bash
# Format
cargo fmt --all

# Lint (zero warnings policy)
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all --lib

# Build everything
cargo build --all-targets
```

**Conventions:**
- No `unwrap()` / `expect()` in library code — use `?` and `thiserror`
- `BTreeMap` required for `prost_types::Struct` fields (protobuf ordering)
- Language host binds a random port and prints it to stdout for CLI discovery

## License

MIT
