use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a vpn connection.
#[derive(Default)]
pub struct GetVpnConnectionArgs {
    /// The name of the vpn connection.
    pub connection_name: String,
    /// The name of the gateway.
    pub gateway_name: String,
    /// The resource group name of the VpnGateway.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetVpnConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Expected bandwidth in MBPS.
    pub connection_bandwidth: Option<i64>,
    /// The connection status.
    pub connection_status: String,
    /// DPD timeout in seconds for vpn connection.
    pub dpd_timeout_seconds: Option<i64>,
    /// Egress bytes transferred.
    pub egress_bytes_transferred: f64,
    /// EnableBgp flag.
    pub enable_bgp: Option<bool>,
    /// Enable internet security.
    pub enable_internet_security: Option<bool>,
    /// EnableBgp flag.
    pub enable_rate_limiting: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Ingress bytes transferred.
    pub ingress_bytes_transferred: f64,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Option<Vec<network::v20241001::IpsecPolicyResponse>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the VPN connection resource.
    pub provisioning_state: String,
    /// Id of the connected vpn site.
    pub remote_vpn_site: Option<network::v20241001::SubResourceResponse>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<network::v20241001::RoutingConfigurationResponse>,
    /// Routing weight for vpn connection.
    pub routing_weight: Option<i64>,
    /// Deprecated: SharedKey for the vpn connection. This is no more used.
    pub shared_key: Option<String>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Option<Vec<network::v20241001::TrafficSelectorPolicyResponse>>,
    /// Use local azure ip to initiate connection.
    pub use_local_azure_ip_address: Option<bool>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Option<bool>,
    /// Connection protocol used for this connection.
    pub vpn_connection_protocol_type: Option<String>,
    /// List of all vpn site link connections to the gateway.
    pub vpn_link_connections: Option<Vec<network::v20241001::VpnSiteLinkConnectionResponse>>,
}

/// Retrieves the details of a vpn connection.
pub async fn get_vpn_connection(
    ctx: &Context,
    args: GetVpnConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVpnConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20241001:getVpnConnection", invoke_args, &opts).await?;

    Ok(GetVpnConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connection_bandwidth: result.fields.get("connectionBandwidth").cloned().map(serde_json::from_value).transpose()?,
        connection_status: serde_json::from_value(result.fields.get("connectionStatus").cloned().unwrap_or_default())?
            ,
        dpd_timeout_seconds: result.fields.get("dpdTimeoutSeconds").cloned().map(serde_json::from_value).transpose()?,
        egress_bytes_transferred: serde_json::from_value(result.fields.get("egressBytesTransferred").cloned().unwrap_or_default())?
            ,
        enable_bgp: result.fields.get("enableBgp").cloned().map(serde_json::from_value).transpose()?,
        enable_internet_security: result.fields.get("enableInternetSecurity").cloned().map(serde_json::from_value).transpose()?,
        enable_rate_limiting: result.fields.get("enableRateLimiting").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ingress_bytes_transferred: serde_json::from_value(result.fields.get("ingressBytesTransferred").cloned().unwrap_or_default())?
            ,
        ipsec_policies: result.fields.get("ipsecPolicies").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        remote_vpn_site: result.fields.get("remoteVpnSite").cloned().map(serde_json::from_value).transpose()?,
        routing_configuration: result.fields.get("routingConfiguration").cloned().map(serde_json::from_value).transpose()?,
        routing_weight: result.fields.get("routingWeight").cloned().map(serde_json::from_value).transpose()?,
        shared_key: result.fields.get("sharedKey").cloned().map(serde_json::from_value).transpose()?,
        traffic_selector_policies: result.fields.get("trafficSelectorPolicies").cloned().map(serde_json::from_value).transpose()?,
        use_local_azure_ip_address: result.fields.get("useLocalAzureIpAddress").cloned().map(serde_json::from_value).transpose()?,
        use_policy_based_traffic_selectors: result.fields.get("usePolicyBasedTrafficSelectors").cloned().map(serde_json::from_value).transpose()?,
        vpn_connection_protocol_type: result.fields.get("vpnConnectionProtocolType").cloned().map(serde_json::from_value).transpose()?,
        vpn_link_connections: result.fields.get("vpnLinkConnections").cloned().map(serde_json::from_value).transpose()?,
    })
}
