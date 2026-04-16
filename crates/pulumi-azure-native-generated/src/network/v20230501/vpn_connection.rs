use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VpnConnection Resource.
pub struct VpnConnectionArgs {
    /// Expected bandwidth in MBPS.
    pub connection_bandwidth: Option<Input<i64>>,
    /// The name of the connection.
    pub connection_name: Option<Input<String>>,
    /// DPD timeout in seconds for vpn connection.
    pub dpd_timeout_seconds: Option<Input<i64>>,
    /// EnableBgp flag.
    pub enable_bgp: Option<Input<bool>>,
    /// Enable internet security.
    pub enable_internet_security: Option<Input<bool>>,
    /// EnableBgp flag.
    pub enable_rate_limiting: Option<Input<bool>>,
    /// The name of the gateway.
    pub gateway_name: Input<String>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Option<Vec<Input<network::v20230501::IpsecPolicyArgs>>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Id of the connected vpn site.
    pub remote_vpn_site: Option<Input<network::v20230501::SubResourceArgs>>,
    /// The resource group name of the VpnGateway.
    pub resource_group_name: Input<String>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<Input<network::v20230501::RoutingConfigurationArgs>>,
    /// Routing weight for vpn connection.
    pub routing_weight: Option<Input<i64>>,
    /// SharedKey for the vpn connection.
    pub shared_key: Option<Input<String>>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Option<Vec<Input<network::v20230501::TrafficSelectorPolicyArgs>>>,
    /// Use local azure ip to initiate connection.
    pub use_local_azure_ip_address: Option<Input<bool>>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Option<Input<bool>>,
    /// Connection protocol used for this connection.
    pub vpn_connection_protocol_type: Option<Input<serde_json::Value>>,
    /// List of all vpn site link connections to the gateway.
    pub vpn_link_connections: Option<Vec<Input<network::v20230501::VpnSiteLinkConnectionArgs>>>,
}

/// VpnConnection Resource.
pub struct VpnConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Expected bandwidth in MBPS.
    pub connection_bandwidth: Output<serde_json::Value>,
    /// The connection status.
    pub connection_status: Output<serde_json::Value>,
    /// DPD timeout in seconds for vpn connection.
    pub dpd_timeout_seconds: Output<serde_json::Value>,
    /// Egress bytes transferred.
    pub egress_bytes_transferred: Output<serde_json::Value>,
    /// EnableBgp flag.
    pub enable_bgp: Output<serde_json::Value>,
    /// Enable internet security.
    pub enable_internet_security: Output<serde_json::Value>,
    /// EnableBgp flag.
    pub enable_rate_limiting: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Ingress bytes transferred.
    pub ingress_bytes_transferred: Output<serde_json::Value>,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the VPN connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Id of the connected vpn site.
    pub remote_vpn_site: Output<serde_json::Value>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Output<serde_json::Value>,
    /// Routing weight for vpn connection.
    pub routing_weight: Output<serde_json::Value>,
    /// SharedKey for the vpn connection.
    pub shared_key: Output<serde_json::Value>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Output<serde_json::Value>,
    /// Use local azure ip to initiate connection.
    pub use_local_azure_ip_address: Output<serde_json::Value>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Output<serde_json::Value>,
    /// Connection protocol used for this connection.
    pub vpn_connection_protocol_type: Output<serde_json::Value>,
    /// List of all vpn site link connections to the gateway.
    pub vpn_link_connections: Output<serde_json::Value>,
}

impl VpnConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230501:VpnConnection";

    /// Create a new VpnConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VpnConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.connection_bandwidth {
            pulumi_sdk::resolve_input("connectionBandwidth", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dpd_timeout_seconds {
            pulumi_sdk::resolve_input("dpdTimeoutSeconds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_bgp {
            pulumi_sdk::resolve_input("enableBgp", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_internet_security {
            pulumi_sdk::resolve_input("enableInternetSecurity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_rate_limiting {
            pulumi_sdk::resolve_input("enableRateLimiting", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("gatewayName", args.gateway_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ipsec_policies {
            pulumi_sdk::resolve_input_vec("ipsecPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_vpn_site {
            pulumi_sdk::resolve_input("remoteVpnSite", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.routing_configuration {
            pulumi_sdk::resolve_input("routingConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.routing_weight {
            pulumi_sdk::resolve_input("routingWeight", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.shared_key {
            pulumi_sdk::resolve_input("sharedKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.traffic_selector_policies {
            pulumi_sdk::resolve_input_vec("trafficSelectorPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_local_azure_ip_address {
            pulumi_sdk::resolve_input("useLocalAzureIpAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_policy_based_traffic_selectors {
            pulumi_sdk::resolve_input("usePolicyBasedTrafficSelectors", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_connection_protocol_type {
            pulumi_sdk::resolve_input("vpnConnectionProtocolType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_link_connections {
            pulumi_sdk::resolve_input_vec("vpnLinkConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            connection_bandwidth: registered.outputs.get("connectionBandwidth")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_status: registered.outputs.get("connectionStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dpd_timeout_seconds: registered.outputs.get("dpdTimeoutSeconds")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            egress_bytes_transferred: registered.outputs.get("egressBytesTransferred")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_bgp: registered.outputs.get("enableBgp")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_internet_security: registered.outputs.get("enableInternetSecurity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_rate_limiting: registered.outputs.get("enableRateLimiting")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ingress_bytes_transferred: registered.outputs.get("ingressBytesTransferred")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ipsec_policies: registered.outputs.get("ipsecPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_vpn_site: registered.outputs.get("remoteVpnSite")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_configuration: registered.outputs.get("routingConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_weight: registered.outputs.get("routingWeight")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            shared_key: registered.outputs.get("sharedKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            traffic_selector_policies: registered.outputs.get("trafficSelectorPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            use_local_azure_ip_address: registered.outputs.get("useLocalAzureIpAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            use_policy_based_traffic_selectors: registered.outputs.get("usePolicyBasedTrafficSelectors")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_connection_protocol_type: registered.outputs.get("vpnConnectionProtocolType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_link_connections: registered.outputs.get("vpnLinkConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
