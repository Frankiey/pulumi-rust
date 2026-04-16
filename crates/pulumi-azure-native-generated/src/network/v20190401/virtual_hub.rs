use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VirtualHub Resource.
pub struct VirtualHubArgs {
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Option<Input<String>>,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Option<Input<network::v20190401::SubResourceArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Option<Input<network::v20190401::SubResourceArgs>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The routeTable associated with this virtual hub.
    pub route_table: Option<Input<network::v20190401::VirtualHubRouteTableArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Option<Input<String>>,
    /// List of all vnet connections with this VirtualHub.
    pub virtual_network_connections: Option<Vec<Input<network::v20190401::HubVirtualNetworkConnectionArgs>>>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Option<Input<network::v20190401::SubResourceArgs>>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Option<Input<network::v20190401::SubResourceArgs>>,
}

/// VirtualHub Resource.
pub struct VirtualHub {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The routeTable associated with this virtual hub.
    pub route_table: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// List of all vnet connections with this VirtualHub.
    pub virtual_network_connections: Output<serde_json::Value>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Output<serde_json::Value>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Output<serde_json::Value>,
}

impl VirtualHub {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190401:VirtualHub";

    /// Create a new VirtualHub resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualHubArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_prefix {
            pulumi_sdk::resolve_input("addressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_gateway {
            pulumi_sdk::resolve_input("expressRouteGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.p2s_vpn_gateway {
            pulumi_sdk::resolve_input("p2SVpnGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_table {
            pulumi_sdk::resolve_input("routeTable", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub_name {
            pulumi_sdk::resolve_input("virtualHubName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_connections {
            pulumi_sdk::resolve_input_vec("virtualNetworkConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_wan {
            pulumi_sdk::resolve_input("virtualWan", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway {
            pulumi_sdk::resolve_input("vpnGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_gateway: registered.outputs.get("expressRouteGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            p2s_vpn_gateway: registered.outputs.get("p2SVpnGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_table: registered.outputs.get("routeTable")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_connections: registered.outputs.get("virtualNetworkConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_wan: registered.outputs.get("virtualWan")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway: registered.outputs.get("vpnGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
