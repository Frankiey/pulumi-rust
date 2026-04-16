use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// P2SVpnGateway Resource.
pub struct P2sVpnGatewayArgs {
    /// List of all customer specified DNS servers IP addresses.
    pub custom_dns_servers: Option<Vec<Input<String>>>,
    /// The name of the gateway.
    pub gateway_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Enable Routing Preference property for the Public IP Interface of the P2SVpnGateway.
    pub is_routing_preference_internet: Option<Input<bool>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// List of all p2s connection configurations of the gateway.
    pub p2s_connection_configurations: Option<Vec<Input<network::v20240301::P2SConnectionConfigurationArgs>>>,
    /// The resource group name of the P2SVpnGateway.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Option<Input<network::v20240301::SubResourceArgs>>,
    /// The scale unit for this p2s vpn gateway.
    pub vpn_gateway_scale_unit: Option<Input<i64>>,
    /// The VpnServerConfiguration to which the p2sVpnGateway is attached to.
    pub vpn_server_configuration: Option<Input<network::v20240301::SubResourceArgs>>,
}

/// P2SVpnGateway Resource.
pub struct P2sVpnGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// List of all customer specified DNS servers IP addresses.
    pub custom_dns_servers: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Enable Routing Preference property for the Public IP Interface of the P2SVpnGateway.
    pub is_routing_preference_internet: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// List of all p2s connection configurations of the gateway.
    pub p2s_connection_configurations: Output<serde_json::Value>,
    /// The provisioning state of the P2S VPN gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Output<serde_json::Value>,
    /// All P2S VPN clients' connection health status.
    pub vpn_client_connection_health: Output<serde_json::Value>,
    /// The scale unit for this p2s vpn gateway.
    pub vpn_gateway_scale_unit: Output<serde_json::Value>,
    /// The VpnServerConfiguration to which the p2sVpnGateway is attached to.
    pub vpn_server_configuration: Output<serde_json::Value>,
}

impl P2sVpnGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240301:P2sVpnGateway";

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

        if let Some(v) = args.custom_dns_servers {
            pulumi_sdk::resolve_input_vec("customDnsServers", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_name {
            pulumi_sdk::resolve_input("gatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_routing_preference_internet {
            pulumi_sdk::resolve_input("isRoutingPreferenceInternet", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.p2s_connection_configurations {
            pulumi_sdk::resolve_input_vec("p2SConnectionConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub {
            pulumi_sdk::resolve_input("virtualHub", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway_scale_unit {
            pulumi_sdk::resolve_input("vpnGatewayScaleUnit", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_server_configuration {
            pulumi_sdk::resolve_input("vpnServerConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            custom_dns_servers: registered.outputs.get("customDnsServers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_routing_preference_internet: registered.outputs.get("isRoutingPreferenceInternet")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            p2s_connection_configurations: registered.outputs.get("p2SConnectionConfigurations")
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
            vpn_client_connection_health: registered.outputs.get("vpnClientConnectionHealth")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway_scale_unit: registered.outputs.get("vpnGatewayScaleUnit")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_server_configuration: registered.outputs.get("vpnServerConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
