use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified virtual network gateway connection by resource group.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayConnectionArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway connection.
    pub virtual_network_gateway_connection_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayConnectionResult {
    /// The authorizationKey.
    pub authorization_key: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The connection mode for this connection.
    pub connection_mode: Option<String>,
    /// Connection protocol used for this connection.
    pub connection_protocol: Option<String>,
    /// Virtual Network Gateway connection status.
    pub connection_status: String,
    /// Gateway connection type.
    pub connection_type: String,
    /// The dead peer detection timeout of this connection in seconds.
    pub dpd_timeout_seconds: Option<i64>,
    /// The egress bytes transferred in this connection.
    pub egress_bytes_transferred: f64,
    /// List of egress NatRules.
    pub egress_nat_rules: Option<Vec<network::v20240501::SubResourceResponse>>,
    /// EnableBgp flag.
    pub enable_bgp: Option<bool>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Bypass ExpressRoute Gateway for data forwarding.
    pub express_route_gateway_bypass: Option<bool>,
    /// GatewayCustomBgpIpAddresses to be used for virtual network gateway Connection.
    pub gateway_custom_bgp_ip_addresses: Option<Vec<network::v20240501::GatewayCustomBgpIpAddressIpConfigurationResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// The ingress bytes transferred in this connection.
    pub ingress_bytes_transferred: f64,
    /// List of ingress NatRules.
    pub ingress_nat_rules: Option<Vec<network::v20240501::SubResourceResponse>>,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Option<Vec<network::v20240501::IpsecPolicyResponse>>,
    /// The reference to local network gateway resource.
    pub local_network_gateway2: Option<network::v20240501::LocalNetworkGatewayResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The reference to peerings resource.
    pub peer: Option<network::v20240501::SubResourceResponse>,
    /// The provisioning state of the virtual network gateway connection resource.
    pub provisioning_state: String,
    /// The resource GUID property of the virtual network gateway connection resource.
    pub resource_guid: String,
    /// The routing weight.
    pub routing_weight: Option<i64>,
    /// The IPSec shared key.
    pub shared_key: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Option<Vec<network::v20240501::TrafficSelectorPolicyResponse>>,
    /// Collection of all tunnels' connection health status.
    pub tunnel_connection_status: Vec<network::v20240501::TunnelConnectionHealthResponse>,
    /// Resource type.
    pub type_: String,
    /// Use private local Azure IP for the connection.
    pub use_local_azure_ip_address: Option<bool>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Option<bool>,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway1: network::v20240501::VirtualNetworkGatewayResponse,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway2: Option<network::v20240501::VirtualNetworkGatewayResponse>,
}

/// Gets the specified virtual network gateway connection by resource group.
pub async fn get_virtual_network_gateway_connection(
    ctx: &Context,
    args: GetVirtualNetworkGatewayConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayConnectionName".to_string(), json!(args.virtual_network_gateway_connection_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240501:getVirtualNetworkGatewayConnection", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayConnectionResult {
        authorization_key: result.fields.get("authorizationKey").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connection_mode: result.fields.get("connectionMode").cloned().map(serde_json::from_value).transpose()?,
        connection_protocol: result.fields.get("connectionProtocol").cloned().map(serde_json::from_value).transpose()?,
        connection_status: serde_json::from_value(result.fields.get("connectionStatus").cloned().unwrap_or_default())?
            ,
        connection_type: serde_json::from_value(result.fields.get("connectionType").cloned().unwrap_or_default())?
            ,
        dpd_timeout_seconds: result.fields.get("dpdTimeoutSeconds").cloned().map(serde_json::from_value).transpose()?,
        egress_bytes_transferred: serde_json::from_value(result.fields.get("egressBytesTransferred").cloned().unwrap_or_default())?
            ,
        egress_nat_rules: result.fields.get("egressNatRules").cloned().map(serde_json::from_value).transpose()?,
        enable_bgp: result.fields.get("enableBgp").cloned().map(serde_json::from_value).transpose()?,
        enable_private_link_fast_path: result.fields.get("enablePrivateLinkFastPath").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_gateway_bypass: result.fields.get("expressRouteGatewayBypass").cloned().map(serde_json::from_value).transpose()?,
        gateway_custom_bgp_ip_addresses: result.fields.get("gatewayCustomBgpIpAddresses").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ingress_bytes_transferred: serde_json::from_value(result.fields.get("ingressBytesTransferred").cloned().unwrap_or_default())?
            ,
        ingress_nat_rules: result.fields.get("ingressNatRules").cloned().map(serde_json::from_value).transpose()?,
        ipsec_policies: result.fields.get("ipsecPolicies").cloned().map(serde_json::from_value).transpose()?,
        local_network_gateway2: result.fields.get("localNetworkGateway2").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        peer: result.fields.get("peer").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        routing_weight: result.fields.get("routingWeight").cloned().map(serde_json::from_value).transpose()?,
        shared_key: result.fields.get("sharedKey").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        traffic_selector_policies: result.fields.get("trafficSelectorPolicies").cloned().map(serde_json::from_value).transpose()?,
        tunnel_connection_status: serde_json::from_value(result.fields.get("tunnelConnectionStatus").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        use_local_azure_ip_address: result.fields.get("useLocalAzureIpAddress").cloned().map(serde_json::from_value).transpose()?,
        use_policy_based_traffic_selectors: result.fields.get("usePolicyBasedTrafficSelectors").cloned().map(serde_json::from_value).transpose()?,
        virtual_network_gateway1: serde_json::from_value(result.fields.get("virtualNetworkGateway1").cloned().unwrap_or_default())?
            ,
        virtual_network_gateway2: result.fields.get("virtualNetworkGateway2").cloned().map(serde_json::from_value).transpose()?,
    })
}
