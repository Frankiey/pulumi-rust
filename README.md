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
