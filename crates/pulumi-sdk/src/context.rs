use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context as AnyhowContext, Result};
use prost_types::Struct;
use tokio::sync::Mutex;

use crate::grpc_client::GrpcEngine;
use crate::output::{Output, OutputState};
use crate::serialize;

/// The Pulumi context passed to user programs inside `run()`.
///
/// Provides methods to register resources and export stack outputs.
pub struct Context {
    engine: Arc<Mutex<GrpcEngine>>,
    project: String,
    stack: String,
    dry_run: bool,
    stack_urn: String,
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

    /// Register a resource with the Pulumi engine.
    ///
    /// Returns a map of output property names to Output values.
    pub async fn register_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: HashMap<String, serde_json::Value>,
        dependencies: Vec<String>,
        property_deps: HashMap<String, Vec<String>>,
        parent: &str,
    ) -> Result<RegisteredResource> {
        let input_struct = serialize::serialize_properties(&inputs, &[]);

        let parent_urn = if parent.is_empty() {
            &self.stack_urn
        } else {
            parent
        };

        let resp = self
            .engine
            .lock()
            .await
            .register_resource(
                resource_type,
                name,
                true, // custom resource
                input_struct,
                parent_urn,
                dependencies,
                property_deps,
            )
            .await?;

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

    /// Export a stack output.
    pub async fn export(&self, name: &str, value: Output<serde_json::Value>) {
        self.exports.lock().await.insert(name.to_string(), value);
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
/// This is the main entrypoint for user programs. It:
/// 1. Reads PULUMI_MONITOR and PULUMI_ENGINE from env vars
/// 2. Connects gRPC clients
/// 3. Registers the stack resource
/// 4. Calls the user's closure with a Context
/// 5. Registers stack outputs
/// 6. Exits
pub fn run<F>(f: F) -> Result<()>
where
    F: FnOnce(&Context) -> Result<()> + Send + 'static,
{
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_async(f))
}

async fn run_async<F>(f: F) -> Result<()>
where
    F: FnOnce(&Context) -> Result<()> + Send + 'static,
{
    let monitor_addr = std::env::var("PULUMI_MONITOR").context("PULUMI_MONITOR env var not set")?;
    let engine_addr = std::env::var("PULUMI_ENGINE").context("PULUMI_ENGINE env var not set")?;
    let project = std::env::var("PULUMI_PROJECT").unwrap_or_else(|_| "project".to_string());
    let stack = std::env::var("PULUMI_STACK").unwrap_or_else(|_| "stack".to_string());
    let dry_run = std::env::var("PULUMI_DRY_RUN")
        .map(|v| v == "true")
        .unwrap_or(false);

    let mut engine = GrpcEngine::connect(&monitor_addr, &engine_addr).await?;

    // Register the stack resource
    let stack_type = "pulumi:pulumi:Stack".to_string();
    let stack_name = format!("{project}-{stack}");
    let stack_resp = engine
        .register_resource(
            &stack_type,
            &stack_name,
            false, // not custom — stack is a component
            Struct::default(),
            "",
            vec![],
            HashMap::new(),
        )
        .await?;

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
        exports: Arc::clone(&exports),
    };

    // Run the user's program
    f(&ctx)?;

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
                // Should not happen after wait, but handle gracefully
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
