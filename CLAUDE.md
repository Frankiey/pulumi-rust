# pulumi-rust

Rust SDK for Pulumi. Wraps Pulumi's automation API and resource model in idiomatic Rust with schema-driven code generation and a fully functional gRPC protocol layer.

## Project goals

- Idiomatic Rust SDK surface for Pulumi automation API
- Type-safe resource definitions generated from Pulumi schemas
- Language host binary for `pulumi new rust` / `pulumi up`

## Stack

- **Rust** — primary SDK implementation (all crates)
- **Pulumi** — underlying infrastructure engine
- **gRPC / tonic** — protocol layer
- **protobuf** — wire format (vendored proto files)

## Conventions

- `cargo fmt` + `cargo clippy` before committing (or `make check`)
- No `unwrap()` in library code — use `?` and proper error types (`thiserror`)
- `BTreeMap` for `prost_types::Struct` fields (protobuf ordering)

## Issue Tracking

This project uses **bd** (beads) for issue tracking.

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for full workflow context and command reference
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Session Completion

Work is NOT complete until all changes are **staged**. Committing and pushing are NOT required.

```bash
git add <changed files>
git status  # all relevant changes must be staged
```

## Structure

```
crates/
  pulumi-sdk/                     # Core SDK: Output<T>, Context, run()
  pulumi-proto/                   # gRPC stubs (compiled from proto/)
  pulumi-codegen/                 # Schema → Rust code generator
  pulumi-codegen-cli/             # CLI for code generation
  pulumi-language-rust/           # Pulumi language host binary
  pulumi-sdk-test/                # Mock + integration test framework
  pulumi-random/                  # Hand-written random provider (reference)
  pulumi-random-generated/        # Auto-generated random provider
  pulumi-azure-native-generated/  # Auto-generated azure-native provider
examples/
  random-strings/                 # Two RandomStrings with Output dependency
  azure-resource-group/           # Azure resource group with tags
proto/pulumi/                     # Vendored Pulumi proto files
docs/                             # Architecture, getting started, packaging strategy
```
