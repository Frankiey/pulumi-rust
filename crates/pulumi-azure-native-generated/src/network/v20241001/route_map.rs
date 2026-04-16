use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The RouteMap child resource of a Virtual hub.
pub struct RouteMapArgs {
    /// List of connections which have this RoutMap associated for inbound traffic.
    pub associated_inbound_connections: Option<Vec<Input<String>>>,
    /// List of connections which have this RoutMap associated for outbound traffic.
    pub associated_outbound_connections: Option<Vec<Input<String>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The resource group name of the RouteMap's resource group.
    pub resource_group_name: Input<String>,
    /// The name of the RouteMap.
    pub route_map_name: Option<Input<String>>,
    /// List of RouteMap rules to be applied.
    pub rules: Option<Vec<Input<network::v20241001::RouteMapRuleArgs>>>,
    /// The name of the VirtualHub containing the RouteMap.
    pub virtual_hub_name: Input<String>,
}

/// The RouteMap child resource of a Virtual hub.
pub struct RouteMap {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// List of connections which have this RoutMap associated for inbound traffic.
    pub associated_inbound_connections: Output<serde_json::Value>,
    /// List of connections which have this RoutMap associated for outbound traffic.
    pub associated_outbound_connections: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the RouteMap resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// List of RouteMap rules to be applied.
    pub rules: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl RouteMap {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:RouteMap";

    /// Create a new RouteMap resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RouteMapArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.associated_inbound_connections {
            pulumi_sdk::resolve_input_vec("associatedInboundConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.associated_outbound_connections {
            pulumi_sdk::resolve_input_vec("associatedOutboundConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_map_name {
            pulumi_sdk::resolve_input("routeMapName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.rules {
            pulumi_sdk::resolve_input_vec("rules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualHubName", args.virtual_hub_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            associated_inbound_connections: registered.outputs.get("associatedInboundConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            associated_outbound_connections: registered.outputs.get("associatedOutboundConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
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
            rules: registered.outputs.get("rules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
