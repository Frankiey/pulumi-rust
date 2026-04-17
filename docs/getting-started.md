# Getting Started with pulumi-rust

This guide walks you through creating your first Pulumi program in Rust.

## Prerequisites

1. **Rust** (stable 1.75+): [rustup.rs](https://rustup.rs)
2. **Protocol Buffers compiler** (`protoc`):
   ```bash
   # macOS
   brew install protobuf
   # Ubuntu / Debian
   apt-get install -y protobuf-compiler
   ```
3. **Pulumi CLI**: [pulumi.com/docs/install](https://www.pulumi.com/docs/install/)
   ```bash
   curl -fsSL https://get.pulumi.com | sh
   ```
4. A Pulumi backend (local or Pulumi Cloud):
   ```bash
   pulumi login --local   # file-based backend
   # or
   pulumi login           # Pulumi Cloud
   ```

You can verify all prerequisites at once:

```bash
make install-check
```

## Option A: Using `pulumi new`

The language host provides a template:

```bash
mkdir my-infra && cd my-infra
pulumi new rust
```

This scaffolds `Cargo.toml`, `Pulumi.yaml`, and `src/main.rs`.

## Option B: Manual setup

### 1. Create the Cargo project

```bash
cargo init my-infra
cd my-infra
```

### 2. Add dependencies

```toml
# Cargo.toml
[dependencies]
pulumi-sdk = { path = "../pulumi-rust/crates/pulumi-sdk" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
anyhow = "1"
```

### 3. Create `Pulumi.yaml`

```yaml
name: my-infra
runtime:
  name: rust
description: My first Pulumi Rust program
```

### 4. Write your program

```rust
// src/main.rs
use anyhow::Result;
use pulumi_sdk::{run, Output};

fn main() -> Result<()> {
    run(|ctx| {
        let rt = tokio::runtime::Handle::current();

        // Export a simple value
        rt.block_on(ctx.export(
            "greeting",
            Output::known(serde_json::json!("Hello from Rust!")),
        ));

        Ok(())
    })
}
```

### 5. Run it

```bash
pulumi stack init dev
pulumi up --yes
```

You should see:

```
Outputs:
    greeting: "Hello from Rust!"
```

## Using a provider

Providers are Rust crates generated from Pulumi schemas. Here's an example with the random provider:

### Add the dependency

```toml
[dependencies]
pulumi-sdk = { path = "../pulumi-rust/crates/pulumi-sdk" }
pulumi-random = { path = "../pulumi-rust/crates/pulumi-random" }
```

### Create a random resource

```rust
use anyhow::Result;
use pulumi_random::{Input, RandomString, RandomStringArgs};
use pulumi_sdk::run;

fn main() -> Result<()> {
    run(|ctx| {
        let rt = tokio::runtime::Handle::current();

        let password = rt.block_on(RandomString::new(
            ctx,
            "my-password",
            RandomStringArgs {
                length: Input::Value(32),
                special: Some(true),
                ..Default::default()
            },
        ))?;

        rt.block_on(ctx.export("password", password.result));
        Ok(())
    })
}
```

## Generating a provider from schema

If a provider doesn't have pre-built Rust bindings, you can generate them:

```bash
# Download the provider schema
curl -sL https://raw.githubusercontent.com/pulumi/pulumi-random/master/provider/cmd/pulumi-resource-random/schema.json \
  -o random-schema.json

# Generate the Rust crate
cargo run -p pulumi-codegen-cli -- generate \
  --schema random-schema.json \
  --out my-random-provider
```

This creates a full Rust crate in `my-random-provider/` with:
- Resource types with `Args` structs and `::new()` constructors
- Function invocations
- Complex types (objects, enums)
- Feature flags for large providers

### Feature flags

Large providers like `azure-native` generate one feature per top-level module:

```toml
# Only include the modules you need
[dependencies]
pulumi-azure-native = { path = "crates/pulumi-azure-native-generated", features = ["resources", "network"] }

# Or include everything (slow compile)
pulumi-azure-native = { path = "crates/pulumi-azure-native-generated", features = ["full"] }
```

## Working with Outputs

`Output<T>` is the core async primitive — it represents a value that may not be known until deployment time.

### Creating outputs

```rust
use pulumi_sdk::Output;
use serde_json::json;

// Known value
let greeting = Output::known(json!("hello"));

// Unknown (preview-time placeholder)
let unknown = Output::unknown();

// Secret
let secret = Output::secret(json!("s3cr3t"));
```

### Chaining with `.apply()`

```rust
let name = Output::known(json!("world"));
let greeting = name.apply(|v| {
    let s = v.as_str().unwrap_or("???");
    serde_json::json!(format!("Hello, {s}!"))
});
```

### Using outputs as resource inputs

```rust
use pulumi_random::{Input, RandomStringArgs};

let length = some_output.apply(|v| v.as_i64().unwrap_or(16));

let args = RandomStringArgs {
    length: Input::Output(length),
    ..Default::default()
};
```

## Assets and Archives

For resources that accept file content (e.g., Lambda function code):

```rust
use pulumi_sdk::{Asset, Archive};

// String content
let inline = Asset::text("console.log('hello')");

// Local file
let file = Asset::file("./handler.js");

// Remote URL
let remote = Asset::remote("https://example.com/code.zip");

// Archive from local path
let archive = Archive::path("./my-lambda/");

// Archive from URL
let archive_url = Archive::remote("https://example.com/code.zip");
```

## Testing

### Unit tests with mocks

```rust
use pulumi_sdk_test::{test_with_mocks, Mocks};
use pulumi_sdk::Output;

struct MyMocks;

impl Mocks for MyMocks {
    fn new_resource(
        &self, _type: &str, name: &str, inputs: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), String> {
        // Return (resource_id, output_properties)
        Ok((format!("{name}-id"), inputs.clone()))
    }
}

#[tokio::test]
async fn test_my_stack() {
    let result = test_with_mocks(MyMocks, "test", "dev", |ctx| async move {
        ctx.export("key", Output::known(serde_json::json!("value"))).await;
        Ok(())
    }).await.unwrap();

    assert_eq!(result.exports["key"], "value");
}
```

### Integration tests

For end-to-end tests against real providers:

```rust
use pulumi_sdk_test::PulumiTestStack;

#[tokio::test]
#[ignore] // requires Pulumi CLI + credentials
async fn test_real_deployment() {
    let stack = PulumiTestStack::new("examples/random-strings", "test-stack")
        .with_env("PULUMI_CONFIG_PASSPHRASE", "test");

    let output = stack.up().await.unwrap();
    assert!(output.outputs.contains_key("stringA"));

    stack.destroy().await.unwrap();
    stack.cleanup().await.unwrap();
}
```

## Next steps

- Browse the [architecture docs](./architecture.md) for SDK internals
- Read the [packaging strategy](./packaging-strategy.md) for the crates.io publishing plan
- Check the [examples/](../examples/) directory for working programs
- Look at `pulumi-random` for a complete hand-written provider example
- Look at `pulumi-random-generated` for what code generation produces
- Run `make help` to see all available development commands
