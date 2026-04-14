# pulumi-rust

Rust SDK for Pulumi. Wraps Pulumi's automation API and resource model in idiomatic Rust, with optional TypeScript bindings.

## Project goals

- Idiomatic Rust SDK surface for Pulumi automation API
- Type-safe resource definitions generated from Pulumi schemas
- Optional TypeScript interop layer

## Stack

- **Rust** — primary SDK implementation
- **TypeScript** — bindings / tooling (secondary)
- **Pulumi** — underlying infrastructure engine

## Conventions

- `cargo fmt` + `cargo clippy` before committing
- No `unwrap()` in library code — use `?` and proper error types
- Keep TypeScript glue minimal; prefer Rust-native solutions

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

## Structure (planned)

```
crates/          # Rust crates
  sdk/           # Core SDK
  macros/        # Proc macros for resource codegen
bindings/        # TypeScript bindings
docs/            # Design docs & research notes
```
