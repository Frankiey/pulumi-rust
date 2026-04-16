use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Configuration information for analysis run.
pub struct ReachabilityAnalysisRunArgs {
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// Represents the Reachability Analysis Run properties.
    pub properties: Input<network::v20250501::ReachabilityAnalysisRunPropertiesArgs>,
    /// Reachability Analysis Run name.
    pub reachability_analysis_run_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Workspace name.
    pub workspace_name: Input<String>,
}

/// Configuration information for analysis run.
pub struct ReachabilityAnalysisRun {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Represents the Reachability Analysis Run properties.
    pub properties: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl ReachabilityAnalysisRun {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:ReachabilityAnalysisRun";

    /// Create a new ReachabilityAnalysisRun resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ReachabilityAnalysisRunArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("networkManagerName", args.network_manager_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("properties", args.properties, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.reachability_analysis_run_name {
            pulumi_sdk::resolve_input("reachabilityAnalysisRunName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("workspaceName", args.workspace_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
