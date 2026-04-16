use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// P2SVpnGateway Resource.
pub struct P2sVpnGatewayArgs {
    /// The reference of the address space resource which represents the custom routes specified by the customer for P2SVpnGateway and P2S VpnClient.
    pub custom_routes: Option<Input<network::v20190401::AddressSpaceArgs>>,
    /// The name of the gateway.
    pub gateway_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The P2SVpnServerConfiguration to which the p2sVpnGateway is attached to.
    pub p2s_vpn_server_configuration: Option<Input<network::v20190401::SubResourceArgs>>,
    /// The resource group name of the P2SVpnGateway.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Option<Input<network::v20190401::SubResourceArgs>>,
    /// The reference of the address space resource which represents Address space for P2S VpnClient.
    pub vpn_client_address_pool: Option<Input<network::v20190401::AddressSpaceArgs>>,
    /// The scale unit for this p2s vpn gateway.
    pub vpn_gateway_scale_unit: Option<Input<i64>>,
}

/// P2SVpnGateway Resource.
pub struct P2sVpnGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The reference of the address space resource which represents the custom routes specified by the customer for P2SVpnGateway and P2S VpnClient.
    pub custom_routes: Output<serde_json::Value>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The P2SVpnServerConfiguration to which the p2sVpnGateway is attached to.
    pub p2s_vpn_server_configuration: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Output<serde_json::Value>,
    /// The reference of the address space resource which represents Address space for P2S VpnClient.
    pub vpn_client_address_pool: Output<serde_json::Value>,
    /// All P2S VPN clients' connection health status.
    pub vpn_client_connection_health: Output<serde_json::Value>,
    /// The scale unit for this p2s vpn gateway.
    pub vpn_gateway_scale_unit: Output<serde_json::Value>,
}

impl P2sVpnGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190401:P2sVpnGateway";

    /// Create a new P2sVpnGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: P2sVpnGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.custom_routes {
            pulumi_sdk::resolve_input("customRoutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_name {
            pulumi_sdk::resolve_input("gatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.p2s_vpn_server_configuration {
            pulumi_sdk::resolve_input("p2SVpnServerConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub {
            pulumi_sdk::resolve_input("virtualHub", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_client_address_pool {
            pulumi_sdk::resolve_input("vpnClientAddressPool", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway_scale_unit {
            pulumi_sdk::resolve_input("vpnGatewayScaleUnit", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            custom_routes: registered.outputs.get("customRoutes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            p2s_vpn_server_configuration: registered.outputs.get("p2SVpnServerConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_hub: registered.outputs.get("virtualHub")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_client_address_pool: registered.outputs.get("vpnClientAddressPool")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_client_connection_health: registered.outputs.get("vpnClientConnectionHealth")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway_scale_unit: registered.outputs.get("vpnGatewayScaleUnit")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
