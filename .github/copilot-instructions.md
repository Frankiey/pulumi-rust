# Copilot instructions

This repo is a Rust SDK for Pulumi.

## Context

- Primary language: **Rust** (idiomatic, safe, no unwrap in lib code)
- Target: wrapping Pulumi's automation API and resource model
- Build system: Cargo workspace + Makefile (`make help` for targets)

## Code style

- Rust: follow `rustfmt` defaults, clippy clean, use `thiserror` for error types
- Prefer explicit error handling over panics
- `BTreeMap` for protobuf `Struct` fields (ordering matters)

## What to suggest

- Idiomatic Rust patterns (builder pattern for resource configs, `From`/`Into` trait impls)
- Pulumi-aware types (URN, resource refs, output/input types)
- Async-first design using `tokio`

## Issue Tracking

This project uses **bd** (beads) for issue tracking.

- Use `bd` for ALL task tracking — do NOT use markdown TODO lists
- Run `bd prime` for full workflow context
- Use `bd remember` for persistent knowledge

### Session Completion

Work is NOT complete until all changes are **staged**. Do NOT commit or push.

```bash
git add <changed files>
git status  # all relevant changes must be staged
```

## What to avoid

- `unwrap()` / `expect()` in library code
- Mutable global state
- Overly generic abstractions without concrete use cases
