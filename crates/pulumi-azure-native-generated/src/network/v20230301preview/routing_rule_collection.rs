use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Defines the routing rule collection.
pub struct RoutingRuleCollectionArgs {
    /// Groups for configuration
    pub applies_to: Vec<Input<network::v20230301preview::NetworkManagerRoutingGroupItemArgs>>,
    /// The name of the network manager Routing Configuration.
    pub configuration_name: Input<String>,
    /// A description of the routing rule collection.
    pub description: Option<Input<String>>,
    /// Determines whether BGP route propagation is enabled. Defaults to true.
    pub disable_bgp_route_propagation: Option<Input<bool>>,
    /// Indicates local route setting for this particular rule collection.
    pub local_route_setting: Input<serde_json::Value>,
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The name of the network manager routing Configuration rule collection.
    pub rule_collection_name: Option<Input<String>>,
}

/// Defines the routing rule collection.
pub struct RoutingRuleCollection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Groups for configuration
    pub applies_to: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A description of the routing rule collection.
    pub description: Output<serde_json::Value>,
    /// Determines whether BGP route propagation is enabled. Defaults to true.
    pub disable_bgp_route_propagation: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Indicates local route setting for this particular rule collection.
    pub local_route_setting: Output<serde_json::Value>,
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

impl RoutingRuleCollection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230301preview:RoutingRuleCollection";

    /// Create a new RoutingRuleCollection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RoutingRuleCollectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input_vec("appliesTo", args.applies_to, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("configurationName", args.configuration_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_bgp_route_propagation {
            pulumi_sdk::resolve_input("disableBgpRoutePropagation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("localRouteSetting", args.local_route_setting, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("networkManagerName", args.network_manager_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rule_collection_name {
            pulumi_sdk::resolve_input("ruleCollectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            applies_to: registered.outputs.get("appliesTo")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            disable_bgp_route_propagation: registered.outputs.get("disableBgpRoutePropagation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            local_route_setting: registered.outputs.get("localRouteSetting")
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
