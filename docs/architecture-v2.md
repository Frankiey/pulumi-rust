# Architecture v2 — Pulumi Rust SDK: Full Provider Support

> This document describes the complete architecture needed to bring the Pulumi Rust SDK
> from its current MVP state to a production-grade SDK capable of deploying Azure, AWS,
> GCP, Kubernetes, and any other Pulumi provider. It is designed to be split into
> actionable tasks once reviewed.

---

## Table of Contents

1. [Current State Assessment](#1-current-state-assessment)
2. [High-Level Architecture](#2-high-level-architecture)
3. [Layer 1: Core SDK Completion](#3-layer-1-core-sdk-completion)
4. [Layer 2: Schema-Driven Code Generation](#4-layer-2-schema-driven-code-generation)
5. [Layer 3: Provider SDK Packaging](#5-layer-3-provider-sdk-packaging)
6. [Layer 4: Language Host Completion](#6-layer-4-language-host-completion)
7. [Layer 5: Testing Infrastructure](#7-layer-5-testing-infrastructure)
8. [Layer 6: Developer Experience](#8-layer-6-developer-experience)
9. [Crate Layout (Final)](#9-crate-layout-final)
10. [Dependency Graph & Ordering](#10-dependency-graph--ordering)
11. [Open Design Questions](#11-open-design-questions)

---

## 1. Current State Assessment

### What exists today

| Component | Status | Notes |
|---|---|---|
| `pulumi-proto` | Complete | All proto stubs compiled via tonic-build |
| `Output<T>` | Functional | Pending/Known/Unknown, apply(), deps, secrets |
| Serialization | Functional | Protobuf <-> serde_json, secrets, unknowns, output sigs |
| `Context` / `run()` | Functional | register_resource, export, project/stack/dry_run |
| gRPC client | Functional | RegisterResource, RegisterResourceOutputs, SetRootResource, Log |
| Language host | Functional | Handshake, Run, GetPluginInfo, About |
| `pulumi-random` | Hand-written | RandomString only, demonstrates the pattern |

### What's missing for full provider support

The gaps fall into five categories:

1. **Core SDK gaps** — resource options, component resources, config, invoke, async run
2. **Code generation** — a tool that reads Pulumi schema JSON and emits Rust crates
3. **Provider packaging** — how generated crates are built, versioned, published
4. **Language host gaps** — install_dependencies, GetRequiredPackages, schema awareness
5. **Testing & DX** — mock engine, project templates, error messages, documentation

---

## 2. High-Level Architecture

```
                                    ┌─────────────────────────┐
                                    │   pulumi-codegen-rust    │
                                    │   (standalone CLI tool)  │
                                    │                         │
                                    │  Reads schema.json ──►  │
                                    │  Emits Rust crate source │
                                    └──────────┬──────────────┘
                                               │ generates
                    ┌──────────────────────────┼──────────────────────────┐
                    ▼                          ▼                          ▼
          ┌─────────────────┐     ┌──────────────────┐     ┌──────────────────┐
          │ pulumi-azure-   │     │ pulumi-aws-       │     │ pulumi-random    │
          │ native          │     │ native            │     │ (generated)      │
          │ (generated)     │     │ (generated)       │     │                  │
          └────────┬────────┘     └────────┬──────────┘     └────────┬─────────┘
                   │                       │                         │
                   └───────────────────────┼─────────────────────────┘
                                           │ depends on
                                           ▼
                               ┌───────────────────────┐
                               │     pulumi-sdk         │
                               │                       │
                               │  Output<T>, Input<T>  │
                               │  Context, run()       │
                               │  ResourceOptions      │
                               │  Config, Invoke       │
                               │  Component resources  │
                               │  Serialization        │
                               │  gRPC client          │
                               │  Error types          │
                               └───────────┬───────────┘
                                           │ depends on
                                           ▼
                               ┌───────────────────────┐
                               │     pulumi-proto       │
                               │  (tonic gRPC stubs)   │
                               └───────────────────────┘

        Separate binary (spawned by Pulumi CLI):

                               ┌───────────────────────┐
                               │ pulumi-language-rust   │
                               │ (LanguageRuntime gRPC) │
                               └───────────────────────┘
```

---

## 3. Layer 1: Core SDK Completion

These are changes to `pulumi-sdk` that must happen before codegen can target it.

### 3.1 Input<T> — Move to SDK

Currently `Input<T>` lives in `pulumi-random`. It's a fundamental SDK type that every
generated provider will use.

```rust
/// An input that can be either a plain value or an Output.
/// This is the primary type used for resource arguments.
pub enum Input<T: Clone + Send + Sync + 'static> {
    Value(T),
    Output(Output<T>),
}

/// Convenience: allow `Input<Option<T>>` for optional properties.
/// The codegen emits `Option<Input<T>>` for optional fields, but
/// users should also be able to pass `Input<Option<T>>`.
```

**Key design decisions:**

- `Input<T>` implements `From<T>` and `From<Output<T>>` for ergonomic construction
- For optional properties, codegen emits `Option<Input<T>>` (not `Input<Option<T>>`)
  so users can simply omit the field entirely
- `Input<T>` needs a method to "resolve" itself into an `Output<T>` so the SDK can
  uniformly handle both cases when collecting dependencies and serializing

```rust
impl<T: Clone + Send + Sync + 'static> Input<T> {
    /// Convert this input into an Output, lifting plain values into known outputs.
    pub fn into_output(self) -> Output<T> {
        match self {
            Input::Value(v) => Output::known(v),
            Input::Output(o) => o,
        }
    }
}
```

### 3.2 ResourceOptions

Every resource registration in Pulumi can carry options. The current SDK hardcodes
`parent` as a string parameter. We need a proper options struct.

```rust
/// Options that control how a resource is managed by the Pulumi engine.
#[derive(Default, Clone)]
pub struct ResourceOptions {
    /// The parent resource. Defaults to the stack.
    pub parent: Option<String>,

    /// An explicit provider to use for this resource.
    pub provider: Option<ProviderResource>,

    /// Additional provider map for component resources.
    pub providers: HashMap<String, ProviderResource>,

    /// URNs this resource depends on (explicit dependencies).
    pub depends_on: Vec<Output<String>>,

    /// If true, the resource is protected from deletion.
    pub protect: Option<bool>,

    /// Properties to ignore during updates.
    pub ignore_changes: Vec<String>,

    /// Properties that trigger replacement when changed.
    pub replace_on_changes: Vec<String>,

    /// If true, delete before creating the replacement.
    pub delete_before_replace: Option<bool>,

    /// Additional output properties to treat as secret.
    pub additional_secret_outputs: Vec<String>,

    /// Custom timeouts for CRUD operations.
    pub custom_timeouts: Option<CustomTimeouts>,

    /// Import an existing resource by provider ID.
    pub import: Option<String>,

    /// If true, retain the resource when removed from the program.
    pub retain_on_delete: Option<bool>,

    /// Resource aliases for migration/renaming.
    pub aliases: Vec<Alias>,

    /// The version of the provider plugin to use.
    pub version: Option<String>,

    /// The download URL for the provider plugin.
    pub plugin_download_url: Option<String>,

    /// Stack transforms to apply.
    pub transforms: Vec<ResourceTransform>,

    /// If set, resource is deleted when this resource is deleted.
    pub deleted_with: Option<String>,
}

pub struct CustomTimeouts {
    pub create: Option<Duration>,
    pub update: Option<Duration>,
    pub delete: Option<Duration>,
}
```

**Builder pattern for ergonomics:**

```rust
// Users can write:
let opts = ResourceOptions::builder()
    .protect(true)
    .depends_on(vec![other_resource.urn()])
    .build();
```

### 3.3 Context: register_resource Overhaul

The current `register_resource` takes too many positional arguments. It needs to accept
`ResourceOptions` and support both custom and component resources.

```rust
impl Context {
    /// Register a custom resource (provider-managed).
    pub async fn register_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: Struct,
        opts: ResourceOptions,
    ) -> Result<RegisteredResource>;

    /// Register a component resource (logical grouping, no provider CRUD).
    pub async fn register_component(
        &self,
        resource_type: &str,
        name: &str,
        opts: ResourceOptions,
    ) -> Result<ComponentContext>;

    /// Register outputs for a component resource.
    pub async fn register_component_outputs(
        &self,
        urn: &str,
        outputs: Struct,
    ) -> Result<()>;
}
```

**ComponentContext** wraps a `Context` scoped to a component, automatically setting the
parent URN for any child resources registered through it:

```rust
pub struct ComponentContext {
    pub urn: String,
    inner: Context,  // cloned with parent set
}

impl ComponentContext {
    /// Register a child resource under this component.
    pub async fn register_resource(...) -> Result<RegisteredResource>;
}
```

### 3.4 Async Run

The current `run()` takes a synchronous closure. This makes it impossible to `.await`
inside the user program without manual runtime management. The primary entrypoint should
be async.

```rust
/// Primary entrypoint. Runs an async Pulumi program.
pub async fn run_async<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce(Context) -> Fut,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    // ... setup ...
    f(ctx).await?;
    // ... collect exports, register outputs ...
}

/// Convenience synchronous wrapper that creates a tokio runtime.
pub fn run<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce(Context) -> Fut,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_async(f))
}
```

The user writes:

```rust
fn main() -> Result<()> {
    pulumi_sdk::run(|ctx| async move {
        let rg = azure_native::resources::ResourceGroup::new(&ctx, "rg", args, None).await?;
        ctx.export("rgName", rg.name.clone()).await;
        Ok(())
    })
}
```

### 3.5 Config

Pulumi passes configuration as env vars (`PULUMI_CONFIG_{key}`) and via the config map
in `RunRequest`. The SDK needs a Config reader.

```rust
/// Reads configuration values for the current stack.
pub struct Config {
    namespace: String,
    values: HashMap<String, String>,
    secret_keys: HashSet<String>,
}

impl Config {
    /// Create a Config reader for the given namespace (typically the project name).
    pub fn new(namespace: &str) -> Self;

    /// Get a plain configuration value.
    pub fn get(&self, key: &str) -> Option<&str>;

    /// Get a required configuration value (returns error if missing).
    pub fn require(&self, key: &str) -> Result<&str>;

    /// Get a secret configuration value, returned as Output to preserve secrecy.
    pub fn get_secret(&self, key: &str) -> Option<Output<String>>;

    /// Get a required secret configuration value.
    pub fn require_secret(&self, key: &str) -> Result<Output<String>>;

    /// Get a config value and deserialize from JSON into T.
    pub fn get_object<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;

    /// Get a required config object.
    pub fn require_object<T: DeserializeOwned>(&self, key: &str) -> Result<T>;
}
```

Config values come from two sources:
1. `PULUMI_CONFIG_{key}` environment variables (set by the language host)
2. Direct passing through the `Context` (for component providers receiving config)

The `Context` should expose config:

```rust
impl Context {
    pub fn config(&self) -> &Config;
}
```

### 3.6 Invoke (Provider Functions)

Pulumi providers expose functions (e.g., `aws:ec2/getAmi:getAmi`) via the `Invoke` RPC
on the resource monitor. The SDK needs to support calling these.

```rust
impl Context {
    /// Invoke a provider function.
    pub async fn invoke(
        &self,
        token: &str,
        args: Struct,
        opts: InvokeOptions,
    ) -> Result<InvokeResult>;
}

pub struct InvokeOptions {
    pub provider: Option<ProviderResource>,
    pub version: Option<String>,
    pub plugin_download_url: Option<String>,
}

pub struct InvokeResult {
    pub fields: HashMap<String, serde_json::Value>,
}
```

The codegen tool will generate typed wrappers:

```rust
// Generated: azure_native::network::get_subnet
pub async fn get_subnet(
    ctx: &Context,
    args: GetSubnetArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSubnetResult> { ... }
```

### 3.7 Provider Resources

Users need to create explicit provider instances to configure multi-region deployments
or use different credentials.

```rust
/// A reference to an explicit provider instance.
#[derive(Clone)]
pub struct ProviderResource {
    urn: String,
    id: Output<String>,
}

impl ProviderResource {
    /// The provider reference string ("urn::id") used in gRPC calls.
    pub fn reference(&self) -> String;
}
```

Codegen will produce provider resource types:

```rust
// Generated: azure_native::Provider
pub struct Provider {
    pub urn: String,
    pub id: Output<String>,
}

impl Provider {
    pub async fn new(ctx: &Context, name: &str, args: ProviderArgs, opts: ...) -> Result<Self>;

    /// Convert to a ProviderResource for use in ResourceOptions.
    pub fn as_provider_resource(&self) -> ProviderResource;
}
```

### 3.8 Stack References

Reading outputs from other stacks:

```rust
pub struct StackReference {
    urn: String,
    outputs: HashMap<String, Output<serde_json::Value>>,
}

impl StackReference {
    pub async fn new(ctx: &Context, name: &str) -> Result<Self>;
    pub fn get_output(&self, name: &str) -> Output<serde_json::Value>;
    pub fn require_output(&self, name: &str) -> Result<Output<serde_json::Value>>;
    pub fn get_secret_output(&self, name: &str) -> Output<serde_json::Value>;
}
```

Stack references are registered as a resource of type `pulumi:pulumi:StackReference`.

### 3.9 Error Types

Replace `anyhow::Result` in the public API with structured errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum PulumiError {
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("missing required configuration: {namespace}:{key}")]
    MissingConfig { namespace: String, key: String },

    #[error("resource registration failed: {0}")]
    ResourceRegistration(String),

    #[error("invoke failed for {token}: {message}")]
    InvokeFailed { token: String, message: String },

    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, PulumiError>;
```

Library code uses `PulumiError`; the `run()` function catches everything and logs to the
engine before exiting.

### 3.10 Logging

Expose engine logging through the Context:

```rust
impl Context {
    pub async fn log_debug(&self, msg: &str);
    pub async fn log_info(&self, msg: &str);
    pub async fn log_warn(&self, msg: &str);
    pub async fn log_error(&self, msg: &str);
}
```

These map to the `Engine.Log` RPC with severity levels 1-4 (DEBUG, INFO, WARNING, ERROR).

### 3.11 RegisterPackage Support

Modern Pulumi SDKs use `RegisterPackage` to register the provider plugin version with
the resource monitor before registering resources. This replaces the older pattern of
passing `version` and `pluginDownloadURL` on every `RegisterResource` call.

```rust
impl GrpcEngine {
    /// Register a provider package and get a package reference.
    pub async fn register_package(
        &mut self,
        name: &str,
        version: &str,
        download_url: &str,
        parameterization: Option<Parameterization>,
    ) -> Result<String>;  // returns the packageRef UUID
}
```

Generated code will call this once per provider at program startup and use the returned
`packageRef` in all subsequent `RegisterResource` calls.

### 3.12 SupportsFeature

The SDK should query the resource monitor for supported features at startup and adjust
behavior accordingly. Key features to check:

- `secrets` — engine supports secret values
- `resourceReferences` — engine supports resource references
- `outputValues` — engine supports output values in inputs
- `deletedWith` — engine supports the deletedWith field

```rust
impl GrpcEngine {
    pub async fn supports_feature(&mut self, feature: &str) -> Result<bool>;
}
```

---

## 4. Layer 2: Schema-Driven Code Generation

This is the largest and most critical piece. It's what makes `pulumi-azure-native`,
`pulumi-aws`, etc. possible.

### 4.1 Pulumi Schema Format

Every Pulumi provider publishes a JSON schema. The schema describes:

- **Resources** — type token, input properties, output properties, required fields,
  deprecation, description
- **Functions** — token, inputs, outputs
- **Types** — reusable complex types (objects, enums)
- **Provider** — the provider resource's own configuration properties
- **Metadata** — name, version, description, language-specific info

Schema example (abbreviated):

```json
{
  "name": "azure-native",
  "version": "2.0.0",
  "resources": {
    "azure-native:resources:ResourceGroup": {
      "inputProperties": {
        "resourceGroupName": { "type": "string" },
        "location": { "type": "string" },
        "tags": {
          "type": "object",
          "additionalProperties": { "type": "string" }
        }
      },
      "requiredInputs": ["location"],
      "properties": {
        "name": { "type": "string" },
        "location": { "type": "string" },
        "id": { "type": "string" },
        "tags": { ... }
      }
    }
  },
  "functions": {
    "azure-native:resources:getResourceGroup": {
      "inputs": { "properties": { "resourceGroupName": { "type": "string" } } },
      "outputs": { "properties": { "name": { ... }, "location": { ... } } }
    }
  },
  "types": {
    "azure-native:network:SubnetType": {
      "properties": { "name": { "type": "string" }, "id": { "type": "string" } }
    }
  }
}
```

The full schema spec is at:
https://www.pulumi.com/docs/using-pulumi/pulumi-packages/schema/

### 4.2 Codegen Tool Design: `pulumi-codegen-rust`

**This is a standalone binary**, not a proc macro or build.rs script. Reasons:

1. **Schema size**: `azure-native` schema is ~200MB JSON. Proc macros would tank
   compile times. Build.rs would add minutes to every `cargo build`.
2. **Debuggability**: A standalone tool can be tested, profiled, and debugged
   independently. Its output is reviewable Rust source.
3. **Precedent**: Go, Python, Java, and .NET SDKs all use standalone codegen tools.
4. **CI integration**: Generated code can be committed (or generated in CI), so users
   don't need the codegen tool to build provider crates.

```
crates/
  pulumi-codegen/        # The code generation library
    src/
      lib.rs             # Public API
      schema.rs          # Schema JSON deserialization (serde)
      naming.rs          # Token → Rust module/type name conversion
      types.rs           # Schema type → Rust type mapping
      resources.rs       # Resource struct + new() generation
      functions.rs       # Invoke function generation
      enums.rs           # Enum type generation
      provider.rs        # Provider resource generation
      modules.rs         # Module tree + mod.rs generation
      format.rs          # Output formatting (prettyplease)
  pulumi-codegen-cli/    # CLI wrapper
    src/main.rs          # `pulumi-codegen-rust generate --schema ./schema.json --out ./src`
```

### 4.3 Schema Deserialization

Define Rust types mirroring the Pulumi schema spec:

```rust
#[derive(Deserialize)]
pub struct PulumiSchema {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub publisher: Option<String>,
    pub provider: ResourceSpec,
    pub resources: HashMap<String, ResourceSpec>,
    pub functions: HashMap<String, FunctionSpec>,
    pub types: HashMap<String, ComplexTypeSpec>,
    pub language: Option<HashMap<String, serde_json::Value>>,
    pub config: Option<ConfigSpec>,
}

#[derive(Deserialize)]
pub struct ResourceSpec {
    pub description: Option<String>,
    #[serde(rename = "inputProperties")]
    pub input_properties: Option<HashMap<String, PropertySpec>>,
    pub properties: Option<HashMap<String, PropertySpec>>,
    #[serde(rename = "requiredInputs")]
    pub required_inputs: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
    pub aliases: Option<Vec<AliasSpec>>,
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
    #[serde(rename = "isComponent")]
    pub is_component: Option<bool>,
    #[serde(rename = "stateInputs")]
    pub state_inputs: Option<ObjectTypeSpec>,
}

#[derive(Deserialize)]
pub struct PropertySpec {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub property_type: Option<String>,       // "string", "integer", "number", "boolean", "array"
    #[serde(rename = "$ref")]
    pub ref_type: Option<String>,            // "#/types/pkg:mod:TypeName"
    pub items: Option<Box<PropertySpec>>,     // for arrays
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Box<PropertySpec>>, // for maps
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<PropertySpec>>,    // union types
    pub default: Option<serde_json::Value>,
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
    pub secret: Option<bool>,
    #[serde(rename = "replaceOnChanges")]
    pub replace_on_changes: Option<bool>,
    pub const_value: Option<serde_json::Value>,
    #[serde(rename = "willReplaceOnChanges")]
    pub will_replace_on_changes: Option<bool>,
}

#[derive(Deserialize)]
pub struct FunctionSpec {
    pub description: Option<String>,
    pub inputs: Option<ObjectTypeSpec>,
    pub outputs: Option<ObjectTypeSpec>,   // sometimes inline, sometimes $ref
    #[serde(rename = "multiOutputs")]
    pub multi_outputs: Option<bool>,
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
}

#[derive(Deserialize)]
pub struct ObjectTypeSpec {
    pub properties: Option<HashMap<String, PropertySpec>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub description: Option<String>,
}

/// A complex type can be an object type or an enum type.
#[derive(Deserialize)]
pub struct ComplexTypeSpec {
    // Object fields
    pub properties: Option<HashMap<String, PropertySpec>>,
    pub required: Option<Vec<String>>,
    pub description: Option<String>,
    // Enum fields
    #[serde(rename = "type")]
    pub enum_type: Option<String>,   // "string", "integer", etc.
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<EnumValueSpec>>,
}

#[derive(Deserialize)]
pub struct EnumValueSpec {
    pub name: Option<String>,
    pub value: serde_json::Value,
    pub description: Option<String>,
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
}
```

### 4.4 Naming Conventions

Pulumi type tokens follow the format: `pkg:module:TypeName`

The codegen must convert these to idiomatic Rust:

| Pulumi Token | Rust Module | Rust Type |
|---|---|---|
| `azure-native:resources:ResourceGroup` | `azure_native::resources` | `ResourceGroup` |
| `azure-native:network:VirtualNetwork` | `azure_native::network` | `VirtualNetwork` |
| `azure-native:storage/blobService:BlobService` | `azure_native::storage::blob_service` | `BlobService` |
| `azure-native:network:SubnetType` (type) | `azure_native::network::types` | `SubnetType` |

Rules:
- Package name: kebab-case to snake_case (`azure-native` → `azure_native`)
- Module path: camelCase to snake_case, `/` becomes `::`
- Type names: PascalCase preserved
- Property names: camelCase to snake_case (`resourceGroupName` → `resource_group_name`)
- Enum variants: PascalCase preserved, with original value stored for serialization
- Conflicting names: append `_` suffix (rare, but handle it)

```rust
pub struct NamingContext {
    package_name: String,  // e.g., "azure_native"
}

impl NamingContext {
    pub fn token_to_module_path(&self, token: &str) -> Vec<String>;
    pub fn token_to_type_name(&self, token: &str) -> String;
    pub fn property_to_field_name(&self, property: &str) -> String;
    pub fn camel_to_snake(s: &str) -> String;
}
```

### 4.5 Type Mapping

Schema types map to Rust types:

| Schema Type | Rust Type (Input) | Rust Type (Output) |
|---|---|---|
| `"string"` | `Input<String>` | `Output<String>` |
| `"integer"` | `Input<i64>` | `Output<i64>` |
| `"number"` | `Input<f64>` | `Output<f64>` |
| `"boolean"` | `Input<bool>` | `Output<bool>` |
| `"array"` items=string | `Input<Vec<Input<String>>>` | `Output<Vec<String>>` |
| `"object"` additionalProperties=string | `Input<HashMap<String, Input<String>>>` | `Output<HashMap<String, String>>` |
| `$ref: "#/types/pkg:mod:Type"` (object) | `Input<TypeArgs>` | `Output<Type>` |
| `$ref: "#/types/pkg:mod:Enum"` (enum) | `Input<Enum>` | `Output<Enum>` |
| `oneOf: [string, number]` | `Input<StringOrNumber>` | `Output<StringOrNumber>` |
| `"string"` + secret | `Input<String>` | `Output<String>` (secret) |
| Asset/Archive types | `Input<AssetOrArchive>` | `Output<AssetOrArchive>` |

**Critical: Input vs Output types**

For each complex type in the schema, codegen generates **two** structs:

```rust
// Input type (used in resource args) — fields are Input<T> or Option<Input<T>>
pub struct VirtualNetworkArgs {
    pub resource_group_name: Input<String>,
    pub location: Option<Input<String>>,
    pub address_space: Option<Input<AddressSpaceArgs>>,
    pub tags: Option<Input<HashMap<String, Input<String>>>>,
}

// Output type (used in resource outputs) — fields are Output<T>
pub struct VirtualNetwork {
    pub urn: String,
    pub name: Output<String>,
    pub location: Output<String>,
    pub address_space: Output<AddressSpace>,
    pub tags: Output<HashMap<String, String>>,
}
```

For **nested object types** referenced by `$ref`:

```rust
// types/address_space.rs
pub struct AddressSpaceArgs {         // input variant
    pub address_prefixes: Option<Input<Vec<Input<String>>>>,
}

pub struct AddressSpace {             // output variant
    pub address_prefixes: Output<Vec<String>>,
}
```

### 4.6 Resource Code Generation

For each resource in the schema, generate:

```rust
// resources/resource_group.rs

use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// A resource group in Azure.
///
/// ## Example
///
/// ```rust,no_run
/// use azure_native::resources::ResourceGroup;
///
/// let rg = ResourceGroup::new(&ctx, "my-rg", ResourceGroupArgs {
///     location: "westeurope".into(),
///     ..Default::default()
/// }, None).await?;
/// ```
pub struct ResourceGroup {
    pub urn: String,
    pub id: Output<String>,
    pub name: Output<String>,
    pub location: Output<String>,
    pub tags: Output<HashMap<String, String>>,
}

/// Input arguments for ResourceGroup.
#[derive(Default)]
pub struct ResourceGroupArgs {
    /// The name of the resource group.
    pub resource_group_name: Option<Input<String>>,
    /// The location of the resource group.
    pub location: Input<String>,
    /// Tags for the resource group.
    pub tags: Option<Input<HashMap<String, Input<String>>>>,
}

impl ResourceGroup {
    const TYPE_TOKEN: &'static str = "azure-native:resources:ResourceGroup";

    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ResourceGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        // Each field: resolve Input → collect deps → serialize to JSON
        pulumi_sdk::resolve_input("location", args.location, &mut inputs, &mut deps, &mut prop_deps).await;

        if let Some(v) = args.resource_group_name {
            pulumi_sdk::resolve_input("resourceGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            pulumi_sdk::serialize_properties(&inputs, &[]),
            opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.output("id"),
            name: registered.output("name"),
            location: registered.output("location"),
            tags: registered.output_map("tags"),
        })
    }
}
```

**Note: `resolve_input` helper** — this is a utility function in the SDK that:
1. Converts `Input<T>` to `Output<T>`
2. Waits for the output value (or marks as unknown)
3. Inserts the serialized value into the inputs map
4. Collects dependency URNs into both the global deps list and per-property deps

### 4.7 Function Code Generation

For each function/invoke in the schema:

```rust
// functions/get_resource_group.rs

pub struct GetResourceGroupArgs {
    pub resource_group_name: String,  // plain types, not Input — invokes are synchronous
}

pub struct GetResourceGroupResult {
    pub id: String,
    pub name: String,
    pub location: String,
    pub tags: HashMap<String, String>,
}

pub async fn get_resource_group(
    ctx: &Context,
    args: GetResourceGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetResourceGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName", json!(args.resource_group_name));

    let result = ctx.invoke(
        "azure-native:resources:getResourceGroup",
        serialize_properties(&invoke_args, &[]),
        opts.unwrap_or_default(),
    ).await?;

    Ok(GetResourceGroupResult {
        id: result.get_string("id")?,
        name: result.get_string("name")?,
        location: result.get_string("location")?,
        tags: result.get_map("tags")?,
    })
}
```

### 4.8 Enum Code Generation

```rust
// types/enums.rs

/// The SKU name of a storage account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkuName {
    StandardLrs,
    StandardGrs,
    StandardRagrs,
    StandardZrs,
    PremiumLrs,
}

impl SkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StandardLrs => "Standard_LRS",
            Self::StandardGrs => "Standard_GRS",
            Self::StandardRagrs => "Standard_RAGRS",
            Self::StandardZrs => "Standard_ZRS",
            Self::PremiumLrs => "Premium_LRS",
        }
    }
}

impl From<SkuName> for serde_json::Value {
    fn from(v: SkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SkuName { ... }
impl<'de> serde::Deserialize<'de> for SkuName { ... }
```

### 4.9 Module Tree Generation

The codegen needs to produce a coherent module tree:

```
azure_native/
  src/
    lib.rs                  # pub mod resources; pub mod network; pub mod storage; ...
    provider.rs             # Provider resource + ProviderArgs
    resources/
      mod.rs                # pub mod resource_group;
      resource_group.rs
    network/
      mod.rs
      virtual_network.rs
      subnet.rs
      types/                # complex types used by this module
        mod.rs
        address_space.rs
        subnet_type.rs
      functions/
        mod.rs
        get_subnet.rs
        get_virtual_network.rs
    storage/
      mod.rs
      storage_account.rs
      types/
        mod.rs
        sku.rs              # contains SkuName enum
      functions/
        mod.rs
    types/                  # top-level / cross-module types
      mod.rs
      enums.rs
    Cargo.toml              # generated, depends on pulumi-sdk
```

The `lib.rs` re-exports key types at the crate root for convenience.

### 4.10 Handling Large Schemas

`azure-native` has ~2000+ resource types and ~5000+ complex types. Considerations:

1. **Cargo features per module**: Each top-level module (resources, network, storage, etc.)
   becomes a Cargo feature. Users opt-in to only the modules they need:
   ```toml
   [dependencies]
   pulumi-azure-native = { version = "2.0", features = ["resources", "network"] }
   ```
   This dramatically reduces compile times.

2. **Lazy module compilation**: Modules behind features are only compiled when enabled.

3. **Split crates (alternative)**: For very large providers, consider splitting into
   sub-crates (`pulumi-azure-native-network`, etc.). This is what azure-sdk-for-rust does.
   However, the feature-flag approach is simpler and should be tried first.

4. **Codegen performance**: Use `rayon` for parallel code generation across modules.
   The codegen tool itself should complete in under 30 seconds even for azure-native.

### 4.11 Serialization Strategy for Generated Types

The generated input types need to serialize into `HashMap<String, serde_json::Value>`
for the SDK's `register_resource` call. Two approaches:

**Option A: Direct JSON serialization (recommended)**

Each `*Args` struct implements a method that serializes into the property map:

```rust
impl ResourceGroupArgs {
    pub(crate) async fn serialize(
        self,
    ) -> (HashMap<String, serde_json::Value>, Vec<String>, HashMap<String, Vec<String>>) {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();
        // ... resolve each field ...
        (inputs, deps, prop_deps)
    }
}
```

This is generated per-type and avoids serde overhead / reflection.

**Option B: serde::Serialize derive**

Derive `Serialize` on args structs and use `serde_json::to_value()`. Simpler codegen
but harder to handle `Input<T>` resolution and dependency tracking. Not recommended
because `Input<T>` is inherently async (it may need to `.await` an `Output<T>`).

**Decision: Option A.** Each generated args struct gets a `serialize` method that
handles Input resolution, dependency collection, and camelCase field name mapping
in one pass.

---

## 5. Layer 3: Provider SDK Packaging

### 5.1 Generated Crate Structure

Each provider becomes a Rust crate:

```toml
# pulumi-azure-native/Cargo.toml (generated)
[package]
name = "pulumi-azure-native"
version = "2.67.0"  # matches provider version
edition = "2021"

[dependencies]
pulumi-sdk = { version = "0.1" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[features]
default = []
resources = []
network = []
storage = []
compute = []
# ... one feature per top-level module
full = ["resources", "network", "storage", "compute", ...]
```

### 5.2 Version Alignment

Provider crate versions should match the upstream provider version:
- `pulumi-azure-native` v2.67.0 → Azure Native provider v2.67.0
- `pulumi-random` v4.16.0 → Random provider v4.16.0

The codegen tool reads the version from the schema and writes it into `Cargo.toml`.

### 5.3 Schema Acquisition

The codegen CLI needs to fetch schemas. Options:

```bash
# From a local file (provider must be installed)
pulumi-codegen-rust generate --schema ./schema.json --out ./pulumi-azure-native/src

# From a running provider (uses GetSchema RPC)
pulumi-codegen-rust generate --provider azure-native --version 2.67.0 --out ./src

# From the Pulumi registry (future)
pulumi-codegen-rust generate --registry azure-native@2.67.0 --out ./src
```

For the initial implementation, support `--schema <path>` only. Users can get schemas via:
```bash
pulumi package get-schema azure-native > schema.json
```

### 5.4 Publishing Strategy

Two models:

1. **Pre-generated crates on crates.io** — We (or CI) run codegen and publish. Users just
   `cargo add pulumi-azure-native`. Best UX, but requires maintaining publish pipelines
   for every provider version.

2. **User-runs-codegen** — Users run `pulumi-codegen-rust generate` themselves. More
   control, works with any provider version, but worse DX.

**Recommendation:** Start with (2) for development, build toward (1) for popular
providers (azure-native, aws, gcp, kubernetes, random). Ship a GitHub Action / CI
template that automates codegen + publish.

---

## 6. Layer 4: Language Host Completion

### 6.1 InstallDependencies

Currently returns an empty stream. Should run `cargo build --release`:

```rust
async fn install_dependencies(
    &self,
    request: Request<InstallDependenciesRequest>,
) -> Result<Response<Self::InstallDependenciesStream>, Status> {
    let req = request.into_inner();
    let dir = req.info.map(|i| i.program_directory).unwrap_or_default();

    let (tx, rx) = tokio::sync::mpsc::channel(16);

    tokio::spawn(async move {
        let mut cmd = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn cargo build");

        // Stream stdout/stderr lines back as InstallDependenciesResponse
        // ...
    });

    Ok(Response::new(ReceiverStream::new(rx)))
}
```

### 6.2 GetRequiredPackages

Should parse `Cargo.toml` to find `pulumi-*` dependencies and report them:

```rust
async fn get_required_packages(
    &self,
    request: Request<GetRequiredPackagesRequest>,
) -> Result<Response<GetRequiredPackagesResponse>, Status> {
    // Parse Cargo.toml, find dependencies matching `pulumi-*`
    // Map crate name → provider name + version
    // Return as PackageDependency list
}
```

This tells the engine which provider plugins to install.

### 6.3 Run: Binary Discovery

The current `Run` implementation assumes the binary name is the entry point. It should:

1. Check if a pre-built binary exists
2. If not, run `cargo build --release` first
3. Find the binary in `target/release/`
4. Execute it with the correct env vars

```rust
async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
    let (work_dir, entry_point) = parse_program_info(&request);

    // Determine binary path
    let binary = if entry_point != "." && Path::new(&entry_point).exists() {
        entry_point
    } else {
        // Find binary from Cargo.toml [[bin]] or package name
        let package_name = read_cargo_package_name(&work_dir)?;
        let binary_path = format!("{work_dir}/target/release/{package_name}");

        if !Path::new(&binary_path).exists() {
            // Build first
            build_project(&work_dir).await?;
        }

        binary_path
    };

    // Execute with env vars (as currently implemented)
    // ...
}
```

### 6.4 Template Support

Implement the `Template` RPC to support `pulumi new rust`:

```yaml
# template/Pulumi.yaml
name: ${PROJECT}
runtime:
  name: rust
  options:
    binary: target/release/${PROJECT}
description: A Pulumi program written in Rust
```

```toml
# template/Cargo.toml
[package]
name = "${PROJECT}"
edition = "2021"

[dependencies]
pulumi-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

```rust
// template/src/main.rs
use anyhow::Result;

fn main() -> Result<()> {
    pulumi_sdk::run(|ctx| async move {
        // Create resources here
        Ok(())
    })
}
```

---

## 7. Layer 5: Testing Infrastructure

### 7.1 Mock Engine for Unit Tests

Users need to test their Pulumi programs without actually deploying. The Go and Node SDKs
provide mock monitors. We need the same.

```rust
// pulumi-sdk-test (new crate)

pub struct MockMonitor {
    resources: Mutex<Vec<MockResourceCall>>,
    mocks: Arc<dyn Mocks>,
}

pub trait Mocks: Send + Sync {
    /// Called when a resource is registered. Return mock outputs.
    fn new_resource(
        &self,
        args: MockResourceArgs,
    ) -> Result<(String, HashMap<String, serde_json::Value>)>;

    /// Called when a function is invoked. Return mock results.
    fn call(
        &self,
        args: MockCallArgs,
    ) -> Result<HashMap<String, serde_json::Value>>;
}

pub struct MockResourceArgs {
    pub resource_type: String,
    pub name: String,
    pub inputs: HashMap<String, serde_json::Value>,
}

pub struct MockCallArgs {
    pub token: String,
    pub args: HashMap<String, serde_json::Value>,
}

/// Run a Pulumi program with mocked infrastructure.
pub async fn test_with_mocks<F, Fut>(
    project: &str,
    stack: &str,
    mocks: impl Mocks + 'static,
    f: F,
) -> Result<TestResult>
where
    F: FnOnce(Context) -> Fut,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    // Start a mock gRPC server implementing ResourceMonitor + Engine
    // Run the user's program against it
    // Collect and return all registered resources + exports
}

pub struct TestResult {
    pub resources: Vec<RegisteredMockResource>,
    pub exports: HashMap<String, serde_json::Value>,
}
```

Usage:

```rust
#[tokio::test]
async fn test_creates_resource_group() {
    struct MyMocks;
    impl Mocks for MyMocks {
        fn new_resource(&self, args: MockResourceArgs) -> Result<(String, HashMap<String, serde_json::Value>)> {
            Ok(("mock-id".into(), args.inputs))
        }
        fn call(&self, _args: MockCallArgs) -> Result<HashMap<String, serde_json::Value>> {
            Ok(HashMap::new())
        }
    }

    let result = test_with_mocks("test", "dev", MyMocks, |ctx| async move {
        let rg = ResourceGroup::new(&ctx, "rg", ResourceGroupArgs {
            location: "westeurope".into(),
            ..Default::default()
        }, None).await?;
        ctx.export("rgName", rg.name).await;
        Ok(())
    }).await.unwrap();

    assert_eq!(result.resources.len(), 1);
    assert_eq!(result.resources[0].resource_type, "azure-native:resources:ResourceGroup");
}
```

### 7.2 Integration Test Framework

A test harness that runs real `pulumi up` / `pulumi destroy`:

```rust
// In tests/
#[tokio::test]
#[ignore]  // only run with --ignored
async fn integration_test_random_string() {
    let stack = PulumiTestStack::new("integration-test", "random-test").await;
    let result = stack.up().await.unwrap();
    assert!(result.outputs.contains_key("randomValue"));
    stack.destroy().await.unwrap();
}
```

### 7.3 Codegen Tests

The codegen tool needs golden-file tests:

```
crates/pulumi-codegen/tests/
  testdata/
    random/
      schema.json          # input
      expected/            # expected output
        src/
          lib.rs
          random_string.rs
          ...
    mini-azure/
      schema.json          # trimmed azure-native schema (few resources)
      expected/
        src/
          lib.rs
          resources/
          network/
          ...
```

Run codegen on the schema, diff against expected output.

---

## 8. Layer 6: Developer Experience

### 8.1 Error Messages

When a resource registration fails, the error should include:
- The resource type and name
- Which property caused the issue (if known)
- A link to the Pulumi docs for that resource type

```
Error: failed to register resource "my-rg" (azure-native:resources:ResourceGroup)
  Property "location" is required but was not provided.
  See: https://www.pulumi.com/registry/packages/azure-native/api-docs/resources/resourcegroup/
```

### 8.2 Derive Macros (Future)

For users defining custom component resources in Rust, we could provide derive macros:

```rust
#[derive(PulumiComponent)]
#[pulumi(type = "my:components:WebApp")]
pub struct WebApp {
    #[pulumi(input)]
    pub domain: Input<String>,

    #[pulumi(output)]
    pub url: Output<String>,
}
```

This is a nice-to-have after codegen works.

### 8.3 Documentation Generation

The codegen should emit doc comments on every generated type, pulled from the schema's
`description` fields. This means `cargo doc` on a generated provider crate produces
full API docs.

### 8.4 IDE Experience

Generated code should be workspace-friendly:
- Proper `mod.rs` / `lib.rs` structure so rust-analyzer can resolve all types
- No use of `include!()` or other patterns that confuse IDEs
- Type annotations on all public fields

---

## 9. Crate Layout (Final)

```
pulumi-rust/
  crates/
    pulumi-proto/              # tonic gRPC stubs (existing)
    pulumi-sdk/                # Core SDK (existing, to be extended)
      src/
        lib.rs
        context.rs             # Context, run(), run_async()
        output.rs              # Output<T>, OutputState
        input.rs               # Input<T> (NEW — moved from pulumi-random)
        serialize.rs           # Property serialization (existing)
        grpc_client.rs         # GrpcEngine (existing, to be extended)
        config.rs              # Config reader (NEW)
        resource_options.rs    # ResourceOptions (NEW)
        provider_resource.rs   # ProviderResource (NEW)
        stack_reference.rs     # StackReference (NEW)
        error.rs               # PulumiError (NEW)
        log.rs                 # Logging helpers (NEW)
        resolve.rs             # Input resolution helpers for codegen (NEW)
    pulumi-codegen/            # Code generation library (NEW)
      src/
        lib.rs
        schema.rs
        naming.rs
        types.rs
        resources.rs
        functions.rs
        enums.rs
        provider.rs
        modules.rs
        format.rs
    pulumi-codegen-cli/        # Codegen CLI binary (NEW)
      src/
        main.rs
    pulumi-language-rust/      # Language host (existing, to be extended)
    pulumi-sdk-test/           # Mock testing framework (NEW)
      src/
        lib.rs
        mock_monitor.rs
        mock_engine.rs

  # Generated provider crates (outside main workspace, or in a sub-workspace)
  providers/
    pulumi-random/             # Generated from random schema (replaces hand-written)
    pulumi-azure-native/       # Generated from azure-native schema
    pulumi-aws/                # Generated from aws schema
    ...

  examples/
    random-strings/            # (existing)
    azure-resource-group/      # (NEW)
    aws-s3-bucket/             # (NEW)

  templates/
    rust-default/              # Template for `pulumi new rust`
```

---

## 10. Dependency Graph & Ordering

```
Phase 1 — SDK Foundation (no external dependencies, pure Rust work)
  ├── 1a. Input<T> → move to SDK, add into_output()
  ├── 1b. ResourceOptions struct + builder
  ├── 1c. Error types (PulumiError, Result)
  ├── 1d. Config reader
  └── 1e. Logging helpers
  All parallelizable.

Phase 2 — SDK Protocol (extends gRPC layer)
  ├── 2a. Context.register_resource() overhaul (takes ResourceOptions)
  │        Depends on: 1b, 1c
  ├── 2b. Invoke support (Context.invoke())
  │        Depends on: 1c
  ├── 2c. SupportsFeature queries
  ├── 2d. RegisterPackage support
  ├── 2e. Component resources (register_component, ComponentContext)
  │        Depends on: 2a
  ├── 2f. Stack references
  │        Depends on: 2a
  └── 2g. Provider resources
           Depends on: 2a
  Partially parallelizable (2a must come first).

Phase 3 — Async Run (breaking change to user API)
  └── 3a. Async run() + run_async()
           Depends on: 2a (new register_resource signature)
  This changes the user-facing entrypoint, so do it once the
  new Context API is stable.

Phase 4 — Code Generation (can start in parallel with Phase 2)
  ├── 4a. Schema deserialization (schema.rs)
  │        No SDK dependency — pure parsing
  ├── 4b. Naming conventions (naming.rs)
  │        No SDK dependency
  ├── 4c. Type mapping (types.rs)
  │        Depends on: 4a, 4b
  ├── 4d. Resource codegen (resources.rs)
  │        Depends on: 4c, and must target the Phase 2 SDK API
  ├── 4e. Function codegen (functions.rs)
  │        Depends on: 4c
  ├── 4f. Enum codegen (enums.rs)
  │        Depends on: 4c
  ├── 4g. Provider codegen (provider.rs)
  │        Depends on: 4d
  ├── 4h. Module tree generation (modules.rs)
  │        Depends on: 4d, 4e, 4f
  ├── 4i. CLI wrapper (pulumi-codegen-cli)
  │        Depends on: 4h
  └── 4j. Golden file tests for codegen
           Depends on: 4i

Phase 5 — First Generated Provider (validation gate)
  └── 5a. Generate pulumi-random from schema
  └── 5b. Run existing random-strings example against generated code
  └── 5c. Generate trimmed azure-native (resources module only)
  └── 5d. Write azure-resource-group example, run pulumi up
  Depends on: Phase 3 + Phase 4

Phase 6 — Language Host Completion
  ├── 6a. InstallDependencies (cargo build)
  ├── 6b. GetRequiredPackages (parse Cargo.toml)
  ├── 6c. Binary discovery in Run
  └── 6d. Template support
  Can start after Phase 3.

Phase 7 — Testing & Polish
  ├── 7a. Mock monitor / engine (pulumi-sdk-test)
  ├── 7b. Integration test framework
  ├── 7c. Full azure-native generation + feature flags
  └── 7d. Documentation, examples, README
  Can start after Phase 5.
```

**Critical path:** 1b → 2a → 3a → 4d → 5a → 5d

**Calendar estimate disclaimer:** Deliberately omitted. Scope is clear; pace depends
on how we parallelize.

---

## 11. Open Design Questions

These should be resolved before or during implementation:

### Q1: Input nesting depth for collections

When a resource input is `Vec<SubnetArgs>`, should the generated type be:
- `Input<Vec<Input<SubnetArgs>>>` (fully wrapped — matches Node/Python behavior)
- `Input<Vec<SubnetArgs>>` (simpler — user wraps the outer vec only)
- `Vec<Input<SubnetArgs>>` (no outer Input — common in Go SDK)

**Recommendation:** `Input<Vec<Input<SubnetArgs>>>` for full fidelity. Individual
elements of a list can be outputs of other resources. Provide a `From` impl so
plain `Vec<SubnetArgs>` works too.

### Q2: Output field access — typed or untyped?

Should `ResourceGroup.name` be `Output<String>` or `Output<serde_json::Value>`?

Typed is better for UX but requires codegen to emit deserialization from
`serde_json::Value` into the concrete type. This adds complexity but is essential
for a good developer experience.

**Recommendation:** `Output<String>`. The codegen emits typed access via
`registered.output_typed::<String>("name")` which internally deserializes from
the JSON value.

### Q3: Separate input/output type names

Go uses `TypeArgs` for inputs and `Type` for outputs. Python uses `TypeArgs` for inputs
and `Type` for outputs. Should we follow this or use a different convention?

**Recommendation:** Follow Go/Python convention:
- `ResourceGroupArgs` — input struct (used when creating)
- `ResourceGroup` — output struct (the created resource with Output<T> fields)
- `AddressSpaceArgs` — input nested type
- `AddressSpace` — output nested type

### Q4: Feature flags vs sub-crates for large providers

For azure-native with 50+ modules:
- Feature flags: single crate, features per module. Simpler `Cargo.toml` for users.
- Sub-crates: `pulumi-azure-native-network`, etc. Faster individual compilation.

**Recommendation:** Start with feature flags. If compile times are unacceptable even
with features (unlikely given Rust's incremental compilation), switch to sub-crates.

### Q5: Should codegen produce a Cargo workspace or flat crate?

For very large providers, a workspace with one crate per module would parallelize
compilation across modules. But it complicates the user's `Cargo.toml`.

**Recommendation:** Single crate with feature flags. Only split if compile time
proves to be a real issue.

### Q6: How to handle `pulumi:pulumi:StackReference` and other built-in types?

These are "resources" that exist in every Pulumi program but don't belong to any
provider. They should live in `pulumi-sdk` directly.

**Recommendation:** `pulumi_sdk::StackReference` is hand-written in the SDK, not generated.

### Q7: Asset and Archive types

Some resources accept file assets or archives (e.g., Lambda function code). The Pulumi
wire protocol has special encodings for these.

```rust
pub enum AssetOrArchive {
    FileAsset(PathBuf),
    StringAsset(String),
    RemoteAsset(String),  // URL
    FileArchive(PathBuf),
    RemoteArchive(String),
    AssetArchive(HashMap<String, AssetOrArchive>),
}
```

**Recommendation:** Implement in the SDK, codegen references it. This is needed for
AWS Lambda, Azure Functions, etc. but is not on the critical path for basic
resource support.

### Q8: Parallel resource registration

Currently, resources are registered sequentially. Pulumi supports parallel registration
(the engine manages the dependency graph). Should `Context` allow spawning parallel
resource registrations?

**Recommendation:** Yes, but defer to Phase 7. The `Context` should be `Clone`-able
(it's already Arc-wrapped internally), and users can `tokio::spawn` multiple resource
registrations. The SDK just needs to make this safe.

---

## Appendix A: Comparison with Other SDK Architectures

| Feature | Go SDK | Python SDK | Node SDK | This Rust SDK |
|---|---|---|---|---|
| Codegen tool | `pulumi-gen-*` | `pulumi-gen-*` | `pulumi-gen-*` | `pulumi-codegen-rust` |
| Input type | `pulumi.Input[T]` | `pulumi.Input[T]` | `pulumi.Input<T>` | `Input<T>` |
| Output type | `pulumi.Output` | `pulumi.Output[T]` | `pulumi.Output<T>` | `Output<T>` |
| Async model | goroutines | asyncio | Promises | tokio async/await |
| Resource options | `pulumi.ResourceOption` functional opts | `ResourceOptions` class | `CustomResourceOptions` | `ResourceOptions` struct + builder |
| Component model | `pulumi.ComponentResource` | `ComponentResource` class | `ComponentResource` class | `Context.register_component()` |
| Config | `config.Config` | `pulumi.Config` | `pulumi.Config` | `Config` |
| Mock testing | `integration.ProgramTest` | `pulumi.runtime.mocks` | `pulumi.runtime.setMocks` | `pulumi_sdk_test::test_with_mocks` |
| Schema source | JSON file | JSON file | JSON file | JSON file |
| Package format | Go module | PyPI package | npm package | crates.io crate |

## Appendix B: Wire Format Examples

### RegisterResource request for an Azure Resource Group

```
type: "azure-native:resources:ResourceGroup"
name: "my-rg"
custom: true
parent: "urn:pulumi:dev::myproject::pulumi:pulumi:Stack::myproject-dev"
object: {
  "location": { stringValue: "westeurope" },
  "tags": { structValue: { fields: { "env": { stringValue: "dev" } } } }
}
dependencies: []
propertyDependencies: {}
acceptSecrets: true
acceptResources: true
packageRef: "abc-123-..."
```

### Invoke request for getResourceGroup

```
tok: "azure-native:resources:getResourceGroup"
args: {
  "resourceGroupName": { stringValue: "my-rg" }
}
packageRef: "abc-123-..."
```

---

*This document should be reviewed, iterated on, and then decomposed into implementation
tasks. Each Phase in Section 10 maps roughly to an epic, and each numbered item within
a phase maps to one or more tasks.*
