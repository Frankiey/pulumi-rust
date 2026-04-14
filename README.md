# pulumi-rust

A Rust SDK for [Pulumi](https://www.pulumi.com/) — idiomatic Rust bindings for the Pulumi automation API and resource model, with optional TypeScript interop.

## Goals

- Idiomatic Rust surface for the Pulumi automation API
- Type-safe resource definitions generated from Pulumi schemas
- Async-first design with `tokio`
- Optional TypeScript bindings layer

## Project Structure

```
crates/
  sdk/        # Core SDK implementation
  macros/     # Proc macros for resource code generation
bindings/     # TypeScript bindings
docs/         # Design docs & research notes
```

## Examples

### TypeScript — Two resources with an `Output<T>` dependency

The simplest way to explore the Pulumi model before touching Rust. This example creates two random strings where the second resource's length is derived from the first resource's output — the core dependency mechanic you'll replicate in Rust.

```typescript
import * as random from "@pulumi/random";

// Resource A — fixed length
const stringA = new random.RandomString("string-a", {
    length: 8,
    special: false,
});

// Resource B — length is an Output<number> derived from A's result
const stringB = new random.RandomString("string-b", {
    length: stringA.result.apply(s => s.length * 2),
    special: false,
});

export const valueA = stringA.result;  // Output<string>, 8 chars
export const valueB = stringB.result;  // Output<string>, 16 chars
```

Run it:

```bash
pulumi login --local
pulumi up --yes
# valueA: "xK9mPqRt"
# valueB: "xK9mPqRtxK9mPqRt"
```

### TypeScript — Stack references and config

Demonstrates reading typed config values and referencing outputs across stacks:

```typescript
import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";

const config = new pulumi.Config();
const length = config.getNumber("length") ?? 12;
const env = pulumi.getStack(); // "dev" | "prod" | ...

const secret = new random.RandomPassword(`db-password-${env}`, {
    length,
    special: true,
    overrideSpecial: "!#$%^&*",
});

// Marked as secret — Pulumi will encrypt this in state
export const dbPassword = pulumi.secret(secret.result);
```

```bash
pulumi config set length 16
pulumi up --yes
```

### TypeScript — Automation API (inline program)

The Automation API is what the Rust SDK wraps. Understanding it in TypeScript first makes the Rust surface much clearer:

```typescript
import { LocalWorkspace } from "@pulumi/pulumi/automation";
import * as random from "@pulumi/random";

async function main() {
    const stack = await LocalWorkspace.createOrSelectStack({
        stackName: "dev",
        projectName: "inline-example",
        program: async () => {
            const r = new random.RandomString("inline-string", {
                length: 10,
                special: false,
            });
            return { result: r.result };
        },
    });

    await stack.setConfig("pulumi:disable-default-providers", { value: "[]" });
    const up = await stack.up({ onOutput: console.log });
    console.log("result:", up.outputs.result.value);

    await stack.destroy();
    await stack.workspace.removeStack("dev");
}

main().catch(console.error);
```

## Getting Started

> This project is in early development. Crates are not yet published to crates.io.

Add the SDK as a dependency from source:

```toml
[dependencies]
pulumi-sdk = { path = "../crates/sdk" }
```

## Development

**Prerequisites:** Rust stable, `cargo`, optionally Node.js for TypeScript bindings.

```bash
# Format and lint
cargo fmt
cargo clippy

# Run tests
cargo test
```

**Conventions:**
- No `unwrap()` in library code — use `?` and proper error types
- Prefer `thiserror` for error definitions
- Keep TypeScript glue minimal; prefer Rust-native solutions

## License

MIT
