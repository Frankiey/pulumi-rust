use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Defines the security admin configuration
pub struct SecurityAdminConfigurationArgs {
    /// Enum list of network intent policy based services.
    pub apply_on_network_intent_policy_based_services: Option<Vec<Input<serde_json::Value>>>,
    /// The name of the network manager Security Configuration.
    pub configuration_name: Option<Input<String>>,
    /// A description of the security configuration.
    pub description: Option<Input<String>>,
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Defines the security admin configuration
pub struct SecurityAdminConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Enum list of network intent policy based services.
    pub apply_on_network_intent_policy_based_services: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A description of the security configuration.
    pub description: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Unique identifier for this resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The system metadata related to this resource.
    pub system_data: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl SecurityAdminConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230401:SecurityAdminConfiguration";

    /// Create a new SecurityAdminConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: SecurityAdminConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.apply_on_network_intent_policy_based_services {
            pulumi_sdk::resolve_input_vec("applyOnNetworkIntentPolicyBasedServices", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.configuration_name {
            pulumi_sdk::resolve_input("configurationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkManagerName", args.network_manager_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            apply_on_network_intent_policy_based_services: registered.outputs.get("applyOnNetworkIntentPolicyBasedServices")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
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
