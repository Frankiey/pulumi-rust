# Packaging & Publishing Strategy

> How this project's crates will be versioned, packaged, and published to crates.io once stable.

---

## Crate Tiers

The workspace contains crates with very different lifecycles. We group them into three tiers:

### Tier 1 — Core SDK (publish to crates.io)

These crates form the public API that users depend on directly.

| Crate | crates.io name | Description |
|---|---|---|
| `pulumi-sdk` | `pulumi-sdk` | Core SDK: `Output<T>`, `Context`, `run()`, serialization |
| `pulumi-proto` | `pulumi-proto` | gRPC stubs for the Pulumi protocol |
| `pulumi-codegen` | `pulumi-codegen` | Schema → Rust code generator library |
| `pulumi-codegen-cli` | `pulumi-codegen-cli` | CLI wrapper for code generation |
| `pulumi-sdk-test` | `pulumi-sdk-test` | Mock + integration test framework |

### Tier 2 — Tooling (publish to crates.io as binary)

| Crate | Distribution | Description |
|---|---|---|
| `pulumi-language-rust` | Binary install / crates.io | Language host binary — bridges Pulumi CLI ↔ Rust |

The language host should be installable via `cargo install pulumi-language-rust` and discoverable by the Pulumi CLI on `$PATH`.

### Tier 3 — Generated Providers (publish separately)

| Crate | crates.io name | Description |
|---|---|---|
| `pulumi-random-generated` | `pulumi-random` | Auto-generated random provider |
| `pulumi-azure-native-generated` | `pulumi-azure-native` | Auto-generated azure-native provider |

Generated providers are published independently with their **provider version** as the crate version (e.g., `pulumi-random@4.19.2`, `pulumi-azure-native@3.16.0`).

### Not Published

| Crate | Reason |
|---|---|
| `pulumi-random` (hand-written) | Superseded by `pulumi-random-generated`; kept for reference |
| `examples/*` | Example programs, not library crates |

---

## Versioning Strategy

### Core crates (Tier 1): Synchronized workspace version

All Tier 1 crates share a single version number and are released together. This avoids dependency hell where `pulumi-sdk@0.3.0` requires `pulumi-proto@0.2.7` — users just match versions.

```toml
# Cargo.toml (workspace root)
[workspace.package]
version = "0.1.0"   # All Tier 1 crates inherit this
```

Each Tier 1 crate's `Cargo.toml`:
```toml
[package]
version.workspace = true
```

Inter-crate dependencies use exact version pins:
```toml
[dependencies]
pulumi-proto = { version = "=0.1.0" }  # exact match
```

**Versioning scheme:**
- `0.x.y` during active development (breaking changes allowed in minor bumps per semver)
- `1.0.0` when the public API stabilizes
- Post-1.0: semver strictly (patch for fixes, minor for additions, major for breaks)

### Generated providers (Tier 3): Provider version

Generated crates use the upstream provider's version directly:
- `pulumi-random` → `4.19.2` (matches `@pulumi/random@4.19.2`)
- `pulumi-azure-native` → `3.16.0`

They declare a minimum compatible SDK version:
```toml
[dependencies]
pulumi-sdk = { version = ">=0.1.0, <1.0.0" }
```

### Language host (Tier 2): Tracks core version

`pulumi-language-rust` tracks the core SDK version since it needs to stay compatible.

---

## Naming Conventions

| Purpose | Convention | Example |
|---|---|---|
| Core SDK crate | `pulumi-{name}` | `pulumi-sdk`, `pulumi-proto` |
| Codegen tool | `pulumi-codegen[-cli]` | `pulumi-codegen`, `pulumi-codegen-cli` |
| Generated provider | `pulumi-{provider}` | `pulumi-random`, `pulumi-azure-native` |
| Test utilities | `pulumi-sdk-test` | `pulumi-sdk-test` |
| Language host | `pulumi-language-rust` | `pulumi-language-rust` |

**Note:** On crates.io, the generated `pulumi-random` replaces the hand-written one. The hand-written crate in this repo is `pulumi-random` (directory `crates/pulumi-random/`) but it will **not** be published — only the generated version will.

---

## Feature Flags (Large Providers)

Large providers like `azure-native` generate hundreds of modules. The strategy:

1. **One feature per top-level module** — e.g., `compute`, `network`, `storage`
2. **`default = []`** — nothing enabled by default; users opt in
3. **`full` feature** — enables everything (useful for CI / exploration)

```toml
# Generated Cargo.toml for azure-native
[features]
default = []
full = ["compute", "network", "storage", "web", ...]
compute = []
network = []
storage = []
```

This keeps compile times manageable — users only build the modules they use.

---

## Dependency Graph

```
                    ┌──────────────┐
                    │ pulumi-proto │  (gRPC stubs)
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  pulumi-sdk  │  (core SDK)
                    └──┬───┬───┬───┘
                       │   │   │
            ┌──────────┘   │   └──────────┐
            ▼              ▼              ▼
   ┌────────────┐  ┌──────────────┐  ┌──────────────────┐
   │ pulumi-sdk │  │   pulumi-    │  │ pulumi-language-  │
   │   -test    │  │   codegen    │  │      rust         │
   └────────────┘  └──────┬───────┘  └──────────────────┘
                          │
                   ┌──────▼───────┐
                   │   pulumi-    │
                   │ codegen-cli  │
                   └──────────────┘

   Generated providers depend only on pulumi-sdk:

   ┌──────────────┐     ┌────────────────────┐
   │pulumi-random │     │pulumi-azure-native │
   └──────┬───────┘     └─────────┬──────────┘
          │                       │
          └───────────┬───────────┘
                      ▼
               ┌──────────┐
               │pulumi-sdk│
               └──────────┘
```

---

## Phased Rollout Plan

### Phase 1: Internal / path dependencies (current)

All crates use `path = "..."` dependencies within the workspace. Users clone the repo and reference crates by path. No crates.io publishing.

**Exit criteria:** API surface is stable enough that breaking changes are infrequent.

### Phase 2: Pre-release on crates.io

Publish `0.1.0-alpha.N` versions to crates.io. This lets early adopters use the SDK without cloning the repo while signaling instability.

**Publish order** (respects dependency graph):
1. `pulumi-proto`
2. `pulumi-sdk`
3. `pulumi-sdk-test`
4. `pulumi-codegen`
5. `pulumi-codegen-cli`
6. `pulumi-language-rust`
7. `pulumi-random` (generated)

**Actions:**
- Set up `cargo-release` or `release-plz` for automated publishing
- Add `publish = true` and fill in `description`, `readme`, `keywords`, `categories` in each crate
- Add CI workflow for crates.io publishing on git tags
- Reserve crate names on crates.io early

### Phase 3: Stable release (1.0)

Publish `1.0.0` when:
- The `Output<T>` / `Context` / `run()` API has been stable for 3+ months
- At least 2 generated providers work end-to-end (random + one cloud provider)
- The language host works with `pulumi new rust` and `pulumi up`
- Documentation covers all public APIs

### Phase 4: Provider ecosystem

Set up automated provider generation and publishing:
- CI job fetches latest schema for each provider
- Runs codegen, builds, tests
- Publishes new crate version if schema changed
- Tracks provider version → crate version mapping

---

## Repository Layout Considerations

The current monorepo layout works well during development. For the long term:

**Keep in monorepo:**
- Core SDK crates (Tier 1)
- Language host (Tier 2)
- Codegen tool
- Examples

**Consider separate repos for:**
- Individual generated providers (once publishing is automated) — each provider repo would contain only the generated code and its CI. This keeps the core repo lean and allows independent release cadence per provider.

**Alternative: monorepo with path-based publishing:**
- Keep everything in one repo
- Use `cargo-release` workspace support to publish subsets
- Simpler to maintain, harder to scale to 100+ providers

**Recommendation:** Stay monorepo through Phase 3. Evaluate provider repos at Phase 4 when the number of providers exceeds 5-10.

---

## Checklist Before First Publish

- [ ] Fill `description`, `readme`, `repository`, `keywords`, `categories` in all publishable crates
- [ ] Add `LICENSE` file at repo root (Apache-2.0, matching `Cargo.toml`)
- [ ] Ensure `license.workspace = true` on all crates
- [ ] Run `cargo publish --dry-run` for each crate in dependency order
- [ ] Reserve crate names on crates.io (`cargo publish` with minimal crates)
- [ ] Set up CI publish workflow (tag-based or manual trigger)
- [ ] Add CHANGELOG.md (per crate or workspace-level)
