use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The network manager connectivity configuration resource
pub struct ConnectivityConfigurationArgs {
    /// Groups for configuration
    pub applies_to_groups: Vec<Input<network::v20230401::ConnectivityGroupItemArgs>>,
    /// The name of the network manager connectivity configuration.
    pub configuration_name: Option<Input<String>>,
    /// Connectivity topology type.
    pub connectivity_topology: Input<serde_json::Value>,
    /// Flag if need to remove current existing peerings.
    pub delete_existing_peering: Option<Input<serde_json::Value>>,
    /// A description of the connectivity configuration.
    pub description: Option<Input<String>>,
    /// List of hubItems
    pub hubs: Option<Vec<Input<network::v20230401::HubArgs>>>,
    /// Flag if global mesh is supported.
    pub is_global: Option<Input<serde_json::Value>>,
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// The network manager connectivity configuration resource
pub struct ConnectivityConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Groups for configuration
    pub applies_to_groups: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Connectivity topology type.
    pub connectivity_topology: Output<serde_json::Value>,
    /// Flag if need to remove current existing peerings.
    pub delete_existing_peering: Output<serde_json::Value>,
    /// A description of the connectivity configuration.
    pub description: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// List of hubItems
    pub hubs: Output<serde_json::Value>,
    /// Flag if global mesh is supported.
    pub is_global: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the connectivity configuration resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Unique identifier for this resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The system metadata related to this resource.
    pub system_data: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl ConnectivityConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230401:ConnectivityConfiguration";

    /// Create a new ConnectivityConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ConnectivityConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input_vec("appliesToGroups", args.applies_to_groups, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.configuration_name {
            pulumi_sdk::resolve_input("configurationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("connectivityTopology", args.connectivity_topology, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.delete_existing_peering {
            pulumi_sdk::resolve_input("deleteExistingPeering", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.hubs {
            pulumi_sdk::resolve_input_vec("hubs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_global {
            pulumi_sdk::resolve_input("isGlobal", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            applies_to_groups: registered.outputs.get("appliesToGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connectivity_topology: registered.outputs.get("connectivityTopology")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            delete_existing_peering: registered.outputs.get("deleteExistingPeering")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hubs: registered.outputs.get("hubs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_global: registered.outputs.get("isGlobal")
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
