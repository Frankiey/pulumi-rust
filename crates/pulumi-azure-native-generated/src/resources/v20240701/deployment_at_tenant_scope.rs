use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Deployment information.
pub struct DeploymentAtTenantScopeArgs {
    /// The name of the deployment.
    pub deployment_name: Option<Input<String>>,
    /// The location to store the deployment data.
    pub location: Option<Input<String>>,
    /// The deployment properties.
    pub properties: Input<resources::v20240701::DeploymentPropertiesArgs>,
    /// Deployment tags
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Deployment information.
pub struct DeploymentAtTenantScope {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// the location of the deployment.
    pub location: Output<serde_json::Value>,
    /// The name of the deployment.
    pub name: Output<serde_json::Value>,
    /// Deployment properties.
    pub properties: Output<serde_json::Value>,
    /// Deployment tags
    pub tags: Output<serde_json::Value>,
    /// The type of the deployment.
    pub type_: Output<serde_json::Value>,
}

impl DeploymentAtTenantScope {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20240701:DeploymentAtTenantScope";

    /// Create a new DeploymentAtTenantScope resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: DeploymentAtTenantScopeArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.deployment_name {
            pulumi_sdk::resolve_input("deploymentName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("properties", args.properties, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
