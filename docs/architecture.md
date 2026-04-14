# Architecture — Pulumi Rust SDK (Iteration 1)

## Two-Process Model

Pulumi uses a two-process architecture. The CLI/engine runs providers and state management; a **language host** bridges the CLI to user programs written in a specific language.

```
CLI / Engine                    Language Host                 User Program
┌──────────────┐    gRPC     ┌───────────────────────┐
│ ResourceMonitor│◄──────────│ pulumi-language-rust   │
│ Engine service │           │ (LanguageRuntime svc)  │
└──────────────┘            │                       │
                             │  1. Receives Run()    │
                             │  2. Spawns user binary │
                             │  3. Passes env vars   │
                             └───────────┬───────────┘
                                         │ PULUMI_MONITOR
                                         │ PULUMI_ENGINE
                                         ▼
                            ┌───────────────────────┐
                            │ User Rust program      │
                            │                       │
                            │  pulumi_sdk::run(|ctx|{│──── RegisterResource() ────► Monitor
                            │    RandomString::new() │◄─── {urn, id, outputs} ─────
                            │    ctx.export(...)     │──── RegisterResourceOutputs()► Monitor
                            │  })                   │
                            └───────────────────────┘
```

## Crate Layout

```
crates/
  pulumi-proto/           # tonic-generated gRPC stubs from Pulumi proto files
  pulumi-sdk/             # Core SDK: Output<T>, serialization, gRPC client, run()
  pulumi-random/          # Hand-written type-safe bindings for pulumi-random provider
  pulumi-language-rust/   # LanguageRuntime gRPC server binary
examples/
  random-strings/         # Benchmark program (two RandomStrings with Output dep)
```

| Crate | Type | Purpose |
|---|---|---|
| `pulumi-proto` | lib | Vendored proto files + tonic-build codegen |
| `pulumi-sdk` | lib | `Output<T>`, property serialization, gRPC client, `Context`, `run()` |
| `pulumi-random` | lib | `RandomString`, `RandomStringArgs` — type-safe resource wrappers |
| `pulumi-language-rust` | bin | gRPC server the CLI spawns; implements `LanguageRuntime` |
| `examples/random-strings` | bin | End-to-end integration test / benchmark |

## Key Types

### Output\<T\>

The central abstraction. Wraps an async value that may be:

- **Pending** — not yet resolved (the engine hasn't returned the response)
- **Known** — resolved with a concrete value
- **Unknown** — value won't be known until apply (preview mode)
- **Secret** — value is sensitive, must be encrypted in state

Each `Output<T>` carries a `Vec<String>` of resource URNs it depends on. `.apply(f)` creates a new output that inherits and extends those dependencies.

```rust
// Internal structure (simplified)
struct OutputInner<T> {
    rx: watch::Receiver<OutputState<T>>,
    deps: Vec<String>,  // resource URNs
    secret: bool,
}

enum OutputState<T> {
    Pending,
    Known(T),
    Unknown,
}
```

### Property Serialization

Resource inputs/outputs are serialized as `google.protobuf.Struct`. Pulumi uses a special-signature wire format for non-trivial values:

| Type | Signature |
|---|---|
| Secret | `4dabf18...` → `1b47061...` + `value` field |
| Unknown | Sentinel UUID `04da6b54-80e4-46f7-96ec-b56ff0331ba9` |
| Output value | `4dabf18...` → `d0e6a83...` + `value`, `secret`, `dependencies` |

### Context

Passed to the user's closure inside `run()`. Provides:

- `register_resource(type, name, inputs, opts)` — sends `RegisterResource` RPC
- `export(name, output)` — collects stack outputs
- Project/stack metadata

## gRPC Protocol

### Services the SDK Calls (as client)

- **ResourceMonitor** — `RegisterResource`, `RegisterResourceOutputs`, `SupportsFeature`
- **Engine** — `SetRootResource`, `Log`

### Service the Language Host Implements (as server)

- **LanguageRuntime** — `Handshake`, `Run`, `GetPluginInfo`, `GetRequiredPackages`

### RPC Sequence (pulumi up)

```
CLI                         Language Host              User Program
 │                               │                          │
 │──spawn─────────────────────►  │                          │
 │  ◄──────── port on stdout ─── │                          │
 │──Handshake(engine_addr)─────► │                          │
 │──GetRequiredPackages()───────►│                          │
 │──Run(monitor_addr, ...)─────► │                          │
 │                               │──spawn(PULUMI_MONITOR)──►│
 │                               │                          │──SetRootResource()──►Engine
 │                               │                          │──RegisterResource(Stack)──►Monitor
 │                               │                          │──RegisterResource(string-a)──►Monitor
 │                               │                          │  ◄── {urn, id, outputs} ──
 │                               │                          │──RegisterResource(string-b, deps=[a])──►Monitor
 │                               │                          │  ◄── {urn, id, outputs} ──
 │                               │                          │──RegisterResourceOutputs(Stack)──►Monitor
 │                               │                          │──exit(0)──►
 │  ◄──RunResponse(ok)────────── │                          │
```

### RegisterResource — Key Fields

**Request:**
- `type` — e.g. `random:index/randomString:RandomString`
- `name` — logical name in the program
- `custom` — true for provider-managed resources
- `object` — input properties as `Struct`
- `dependencies` — all URNs this resource depends on
- `propertyDependencies` — per-property URN lists (for fine-grained ordering)
- `parent` — parent URN (empty = stack)
- `acceptSecrets`, `acceptResources` — capability flags

**Response:**
- `urn` — assigned resource URN
- `id` — provider-assigned ID
- `object` — output properties as `Struct`

## Dependency Graph

```
Phase 0 — Scaffolding
  Workspace setup (Cargo workspace, CI)

Phase 1 — Foundations (parallelizable)
  Proto stubs          Output<T> type
       │                    │
       ▼                    │
Phase 2 — Core plumbing     │
  Serialization    Language host
       │                    │
       ▼                    │
Phase 3 — Integration       │
  gRPC client ◄─────────────┘

Phase 4 — User-facing API
  Context + run()

Phase 5 — Provider layer
  pulumi-random bindings

Phase 6 — End-to-end
  Benchmark program (depends on bindings + language host)
```

## Design Decisions

1. **Vendored protos** — proto files copied from `pulumi/pulumi` into `proto/`, not submoduled
2. **Separate language host binary** — follows Go SDK pattern; CLI spawns it, it spawns user code
3. **tokio async runtime** — `Output<T>` resolution is async; `run()` drives the runtime
4. **tonic for gRPC** — standard Rust gRPC stack
5. **thiserror for SDK errors, anyhow in binaries**
6. **MVP scope** — only `RegisterResource`, `RegisterResourceOutputs`, `SetRootResource`, `Log`; no transforms, hooks, or component resources
