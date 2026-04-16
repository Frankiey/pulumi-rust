use std::collections::HashMap;
use std::sync::Arc;

use prost_types::Struct;
use tokio::sync::Mutex;

use pulumi_proto::pulumirpc::{
    self, register_resource_request, Alias as ProtoAlias, RegisterResourceRequest,
    ResourceInvokeRequest,
};

use crate::error::{PulumiError, Result};
use crate::grpc_client::GrpcEngine;
use crate::invoke::{InvokeOptions, InvokeResult};
use crate::output::{Output, OutputState};
use crate::package::Parameterization;
use crate::resource_options::{Alias, ResourceOptions};
use crate::serialize;

/// Cached results of SupportsFeature queries made at startup.
#[derive(Debug, Clone, Default)]
pub struct FeatureSupport {
    /// Whether the engine supports secret values.
    pub secrets: bool,
    /// Whether the engine supports resource references.
    pub resource_references: bool,
    /// Whether the engine supports output values in inputs.
    pub output_values: bool,
    /// Whether the engine supports the `deletedWith` field.
    pub deleted_with: bool,
}

/// The Pulumi context passed to user programs inside `run()`.
///
/// Provides methods to register resources and export stack outputs.
#[derive(Clone)]
pub struct Context {
    engine: Arc<Mutex<GrpcEngine>>,
    project: String,
    stack: String,
    dry_run: bool,
    stack_urn: String,
    features: FeatureSupport,
    exports: Arc<Mutex<HashMap<String, Output<serde_json::Value>>>>,
}

impl Context {
    /// Returns the project name.
    pub fn project(&self) -> &str {
        &self.project
    }

    /// Returns the stack name.
    pub fn stack(&self) -> &str {
        &self.stack
    }

    /// Returns whether this is a preview (dry run).
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Returns the cached feature support flags queried at startup.
    pub fn features(&self) -> &FeatureSupport {
        &self.features
    }

    /// Register a resource with the Pulumi engine.
    ///
    /// The `opts` parameter controls all resource options (parent, provider,
    /// dependencies, protection, etc.). Pass `ResourceOptions::default()` for defaults.
    pub async fn register_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: HashMap<String, serde_json::Value>,
        property_deps: HashMap<String, Vec<String>>,
        opts: &ResourceOptions,
    ) -> Result<RegisteredResource> {
        let input_struct = serialize::serialize_properties(&inputs, &[]);

        let parent_urn = match &opts.parent {
            Some(p) if !p.is_empty() => p.clone(),
            _ => self.stack_urn.clone(),
        };

        // Resolve explicit dependencies from Output<String> values
        let mut dependencies = Vec::new();
        for dep in &opts.depends_on {
            if let OutputState::Known(urn) = dep.wait().await {
                dependencies.push(urn);
            }
        }

        let prop_dep_map = property_deps
            .into_iter()
            .map(|(k, urns)| (k, register_resource_request::PropertyDependencies { urns }))
            .collect();

        // Convert SDK CustomTimeouts to proto CustomTimeouts
        let custom_timeouts =
            opts.custom_timeouts
                .as_ref()
                .map(|ct| register_resource_request::CustomTimeouts {
                    create: ct
                        .create
                        .map(|d| format!("{}s", d.as_secs()))
                        .unwrap_or_default(),
                    update: ct
                        .update
                        .map(|d| format!("{}s", d.as_secs()))
                        .unwrap_or_default(),
                    delete: ct
                        .delete
                        .map(|d| format!("{}s", d.as_secs()))
                        .unwrap_or_default(),
                });

        // Convert SDK Alias to proto Alias
        let aliases: Vec<ProtoAlias> = opts.aliases.iter().map(|a| sdk_alias_to_proto(a)).collect();

        let delete_before_replace = opts.delete_before_replace.unwrap_or(false);
        let delete_before_replace_defined = opts.delete_before_replace.is_some();

        let req = RegisterResourceRequest {
            r#type: resource_type.to_string(),
            name: name.to_string(),
            parent: parent_urn,
            custom: true,
            object: Some(input_struct),
            protect: opts.protect,
            dependencies,
            provider: opts.provider.clone().unwrap_or_default(),
            property_dependencies: prop_dep_map,
            delete_before_replace,
            version: opts.version.clone().unwrap_or_default(),
            ignore_changes: opts.ignore_changes.clone(),
            accept_secrets: true,
            additional_secret_outputs: opts.additional_secret_outputs.clone(),
            import_id: opts.import.clone().unwrap_or_default(),
            custom_timeouts,
            delete_before_replace_defined,
            accept_resources: true,
            providers: opts.providers.clone(),
            replace_on_changes: opts.replace_on_changes.clone(),
            plugin_download_url: opts.plugin_download_url.clone().unwrap_or_default(),
            retain_on_delete: opts.retain_on_delete,
            aliases,
            deleted_with: opts.deleted_with.clone().unwrap_or_default(),
            alias_specs: true,
            supports_partial_values: true,
            supports_result_reporting: true,
            ..Default::default()
        };

        let resp = self.engine.lock().await.register_resource(req).await?;

        let mut outputs = HashMap::new();
        for (key, prop_val) in resp.properties {
            let output = if !prop_val.known {
                Output::unknown(prop_val.deps).with_secret(prop_val.secret)
            } else {
                Output::known(prop_val.value)
                    .with_secret(prop_val.secret)
                    .with_deps(prop_val.deps)
            };
            outputs.insert(key, output);
        }

        Ok(RegisteredResource {
            urn: resp.urn,
            id: resp.id,
            outputs,
        })
    }

    /// Register a provider package and get a package reference UUID.
    ///
    /// Generated code calls this once per provider at startup. The returned
    /// `packageRef` is then passed in `RegisterResource` calls via
    /// `ResourceOptions` or set on the request directly.
    pub async fn register_package(
        &self,
        name: &str,
        version: &str,
        download_url: &str,
        parameterization: Option<Parameterization>,
    ) -> Result<String> {
        self.engine
            .lock()
            .await
            .register_package(name, version, download_url, parameterization)
            .await
    }

    /// Invoke a provider function.
    ///
    /// `token` is the function token (e.g., `aws:ec2/getAmi:getAmi`).
    /// `args` is a map of argument name → JSON value.
    /// `opts` controls provider, version, and plugin download URL.
    pub async fn invoke(
        &self,
        token: &str,
        args: HashMap<String, serde_json::Value>,
        opts: &InvokeOptions,
    ) -> Result<InvokeResult> {
        let proto_args = serialize::serialize_properties(&args, &[]);
        let req = ResourceInvokeRequest {
            tok: token.to_string(),
            args: Some(proto_args),
            provider: opts.provider.clone().unwrap_or_default(),
            version: opts.version.clone().unwrap_or_default(),
            accept_resources: true,
            plugin_download_url: opts.plugin_download_url.clone().unwrap_or_default(),
            ..Default::default()
        };

        let properties = self
            .engine
            .lock()
            .await
            .invoke(req)
            .await
            .map_err(|e| match e {
                PulumiError::InvokeFailed { message, .. } => PulumiError::InvokeFailed {
                    token: token.to_string(),
                    message,
                },
                other => other,
            })?;

        let fields = properties
            .into_iter()
            .filter_map(|(k, pv)| if pv.known { Some((k, pv.value)) } else { None })
            .collect();

        Ok(InvokeResult { fields })
    }

    /// Export a stack output.
    pub async fn export(&self, name: &str, value: Output<serde_json::Value>) {
        self.exports.lock().await.insert(name.to_string(), value);
    }

    // -- Logging helpers (map to Engine.Log RPC) --

    /// Log a debug message (severity 1).
    pub async fn log_debug(&self, message: &str) -> Result<()> {
        self.log(1, message).await
    }

    /// Log an informational message (severity 2).
    pub async fn log_info(&self, message: &str) -> Result<()> {
        self.log(2, message).await
    }

    /// Log a warning message (severity 3).
    pub async fn log_warn(&self, message: &str) -> Result<()> {
        self.log(3, message).await
    }

    /// Log an error message (severity 4).
    pub async fn log_error(&self, message: &str) -> Result<()> {
        self.log(4, message).await
    }

    async fn log(&self, severity: i32, message: &str) -> Result<()> {
        self.engine.lock().await.log(severity, message, "").await
    }

    /// Register a component resource (logical grouping, no provider CRUD).
    ///
    /// Returns a `ComponentContext` that automatically sets this component's URN
    /// as the parent for any child resources registered through it.
    pub async fn register_component(
        &self,
        resource_type: &str,
        name: &str,
        opts: &ResourceOptions,
    ) -> Result<ComponentContext> {
        let parent_urn = match &opts.parent {
            Some(p) if !p.is_empty() => p.clone(),
            _ => self.stack_urn.clone(),
        };

        let req = RegisterResourceRequest {
            r#type: resource_type.to_string(),
            name: name.to_string(),
            parent: parent_urn,
            custom: false,
            protect: opts.protect,
            accept_secrets: true,
            accept_resources: true,
            aliases: opts.aliases.iter().map(|a| sdk_alias_to_proto(a)).collect(),
            providers: opts.providers.clone(),
            alias_specs: true,
            ..Default::default()
        };

        let resp = self.engine.lock().await.register_resource(req).await?;

        let mut child_ctx = self.clone();
        child_ctx.stack_urn = resp.urn.clone();

        Ok(ComponentContext {
            urn: resp.urn,
            inner: child_ctx,
        })
    }

    /// Register outputs for a completed component resource.
    pub async fn register_component_outputs(&self, urn: &str, outputs: Struct) -> Result<()> {
        self.engine
            .lock()
            .await
            .register_resource_outputs(urn, outputs)
            .await
    }
}

/// A context scoped to a component resource.
///
/// All child resources registered through this context automatically have
/// the component's URN as their parent.
pub struct ComponentContext {
    /// The URN of this component resource.
    pub urn: String,
    inner: Context,
}

impl ComponentContext {
    /// Register a child resource under this component.
    pub async fn register_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: HashMap<String, serde_json::Value>,
        property_deps: HashMap<String, Vec<String>>,
        opts: &ResourceOptions,
    ) -> Result<RegisteredResource> {
        // If the caller didn't set a parent, use this component's URN
        let mut child_opts;
        if opts.parent.is_none() {
            child_opts = opts.clone();
            child_opts.parent = Some(self.urn.clone());
            self.inner
                .register_resource(resource_type, name, inputs, property_deps, &child_opts)
                .await
        } else {
            self.inner
                .register_resource(resource_type, name, inputs, property_deps, opts)
                .await
        }
    }

    /// Register outputs for this component resource.
    pub async fn register_outputs(&self, outputs: Struct) -> Result<()> {
        self.inner
            .register_component_outputs(&self.urn, outputs)
            .await
    }

    /// Returns the underlying `Context`.
    pub fn context(&self) -> &Context {
        &self.inner
    }
}

/// Convert an SDK `Alias` to the protobuf `Alias` type.
fn sdk_alias_to_proto(alias: &Alias) -> ProtoAlias {
    // If parent_urn is set (as a plain URN string), use it as an Alias::Urn
    if let Some(ref urn) = alias.parent_urn {
        if alias.name.is_none()
            && alias.r#type.is_none()
            && alias.project.is_none()
            && alias.stack.is_none()
        {
            return ProtoAlias {
                alias: Some(pulumirpc::alias::Alias::Urn(urn.clone())),
            };
        }
    }

    let parent = if let Some(true) = alias.no_parent {
        Some(pulumirpc::alias::spec::Parent::NoParent(true))
    } else {
        alias
            .parent_urn
            .as_ref()
            .map(|u| pulumirpc::alias::spec::Parent::ParentUrn(u.clone()))
    };

    let spec = pulumirpc::alias::Spec {
        name: alias.name.clone().unwrap_or_default(),
        r#type: alias.r#type.clone().unwrap_or_default(),
        stack: alias.stack.clone().unwrap_or_default(),
        project: alias.project.clone().unwrap_or_default(),
        parent,
    };

    ProtoAlias {
        alias: Some(pulumirpc::alias::Alias::Spec(spec)),
    }
}

/// A resource that has been registered with the engine.
pub struct RegisteredResource {
    pub urn: String,
    pub id: String,
    pub outputs: HashMap<String, Output<serde_json::Value>>,
}

/// Run a Pulumi program.
///
/// Convenience synchronous wrapper that creates a tokio runtime and
/// delegates to [`run_async`]. Accepts an async closure.
///
/// # Example
///
/// ```ignore
/// pulumi_sdk::run(|ctx| async move {
///     // register resources using ctx...
///     Ok(())
/// })
/// ```
pub fn run<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce(Context) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| PulumiError::Custom(format!("failed to create tokio runtime: {e}")))?;
    rt.block_on(run_async(f))
}

/// Primary async entrypoint for Pulumi programs.
///
/// 1. Reads PULUMI_MONITOR and PULUMI_ENGINE from env vars
/// 2. Connects gRPC clients
/// 3. Queries engine feature support
/// 4. Registers the stack resource
/// 5. Calls the user's async closure with a `Context`
/// 6. Registers stack outputs
pub async fn run_async<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce(Context) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    let monitor_addr = std::env::var("PULUMI_MONITOR")
        .map_err(|_| PulumiError::MissingEnvVar("PULUMI_MONITOR".to_string()))?;
    let engine_addr = std::env::var("PULUMI_ENGINE")
        .map_err(|_| PulumiError::MissingEnvVar("PULUMI_ENGINE".to_string()))?;
    let project = std::env::var("PULUMI_PROJECT").unwrap_or_else(|_| "project".to_string());
    let stack = std::env::var("PULUMI_STACK").unwrap_or_else(|_| "stack".to_string());
    let dry_run = std::env::var("PULUMI_DRY_RUN")
        .map(|v| v == "true")
        .unwrap_or(false);

    let mut engine = GrpcEngine::connect(&monitor_addr, &engine_addr).await?;

    // Query supported features at startup
    let features = FeatureSupport {
        secrets: engine.supports_feature("secrets").await.unwrap_or(false),
        resource_references: engine
            .supports_feature("resourceReferences")
            .await
            .unwrap_or(false),
        output_values: engine
            .supports_feature("outputValues")
            .await
            .unwrap_or(false),
        deleted_with: engine
            .supports_feature("deletedWith")
            .await
            .unwrap_or(false),
    };

    // Register the stack resource (component, not custom)
    let stack_type = "pulumi:pulumi:Stack".to_string();
    let stack_name = format!("{project}-{stack}");
    let stack_req = RegisterResourceRequest {
        r#type: stack_type,
        name: stack_name,
        custom: false,
        object: Some(Struct::default()),
        accept_secrets: true,
        accept_resources: true,
        ..Default::default()
    };
    let stack_resp = engine.register_resource(stack_req).await?;

    let stack_urn = stack_resp.urn.clone();

    // Set the root resource
    engine.set_root_resource(&stack_urn).await?;

    let engine = Arc::new(Mutex::new(engine));
    let exports: Arc<Mutex<HashMap<String, Output<serde_json::Value>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let ctx = Context {
        engine: Arc::clone(&engine),
        project,
        stack,
        dry_run,
        stack_urn: stack_urn.clone(),
        features,
        exports: Arc::clone(&exports),
    };

    // Run the user's async program
    f(ctx).await?;

    // Collect and register stack outputs
    let export_map = exports.lock().await;
    let mut output_fields = std::collections::BTreeMap::new();

    for (name, output) in export_map.iter() {
        match output.wait().await {
            OutputState::Known(val) => {
                output_fields.insert(
                    name.clone(),
                    serialize::serialize_property(&val, output.is_secret()),
                );
            }
            OutputState::Unknown => {
                output_fields.insert(name.clone(), serialize::serialize_unknown());
            }
            OutputState::Pending => {
                output_fields.insert(name.clone(), serialize::serialize_unknown());
            }
        }
    }

    let outputs_struct = Struct {
        fields: output_fields,
    };

    engine
        .lock()
        .await
        .register_resource_outputs(&stack_urn, outputs_struct)
        .await?;

    Ok(())
}
