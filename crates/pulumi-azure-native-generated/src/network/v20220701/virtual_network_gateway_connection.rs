use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A common class for general resource information.
pub struct VirtualNetworkGatewayConnectionArgs {
    /// The authorizationKey.
    pub authorization_key: Option<Input<String>>,
    /// The connection mode for this connection.
    pub connection_mode: Option<Input<serde_json::Value>>,
    /// Connection protocol used for this connection.
    pub connection_protocol: Option<Input<serde_json::Value>>,
    /// Gateway connection type.
    pub connection_type: Input<serde_json::Value>,
    /// The dead peer detection timeout of this connection in seconds.
    pub dpd_timeout_seconds: Option<Input<i64>>,
    /// List of egress NatRules.
    pub egress_nat_rules: Option<Vec<Input<network::v20220701::SubResourceArgs>>>,
    /// EnableBgp flag.
    pub enable_bgp: Option<Input<bool>>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Option<Input<bool>>,
    /// Bypass ExpressRoute Gateway for data forwarding.
    pub express_route_gateway_bypass: Option<Input<bool>>,
    /// GatewayCustomBgpIpAddresses to be used for virtual network gateway Connection.
    pub gateway_custom_bgp_ip_addresses: Option<Vec<Input<network::v20220701::GatewayCustomBgpIpAddressIpConfigurationArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// List of ingress NatRules.
    pub ingress_nat_rules: Option<Vec<Input<network::v20220701::SubResourceArgs>>>,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Option<Vec<Input<network::v20220701::IpsecPolicyArgs>>>,
    /// The reference to local network gateway resource.
    pub local_network_gateway2: Option<Input<network::v20220701::LocalNetworkGatewayArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The reference to peerings resource.
    pub peer: Option<Input<network::v20220701::SubResourceArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The routing weight.
    pub routing_weight: Option<Input<i64>>,
    /// The IPSec shared key.
    pub shared_key: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Option<Vec<Input<network::v20220701::TrafficSelectorPolicyArgs>>>,
    /// Use private local Azure IP for the connection.
    pub use_local_azure_ip_address: Option<Input<bool>>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Option<Input<bool>>,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway1: Input<network::v20220701::VirtualNetworkGatewayArgs>,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway2: Option<Input<network::v20220701::VirtualNetworkGatewayArgs>>,
    /// The name of the virtual network gateway connection.
    pub virtual_network_gateway_connection_name: Option<Input<String>>,
}

/// A common class for general resource information.
pub struct VirtualNetworkGatewayConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The authorizationKey.
    pub authorization_key: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The connection mode for this connection.
    pub connection_mode: Output<serde_json::Value>,
    /// Connection protocol used for this connection.
    pub connection_protocol: Output<serde_json::Value>,
    /// Virtual Network Gateway connection status.
    pub connection_status: Output<serde_json::Value>,
    /// Gateway connection type.
    pub connection_type: Output<serde_json::Value>,
    /// The dead peer detection timeout of this connection in seconds.
    pub dpd_timeout_seconds: Output<serde_json::Value>,
    /// The egress bytes transferred in this connection.
    pub egress_bytes_transferred: Output<serde_json::Value>,
    /// List of egress NatRules.
    pub egress_nat_rules: Output<serde_json::Value>,
    /// EnableBgp flag.
    pub enable_bgp: Output<serde_json::Value>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Bypass ExpressRoute Gateway for data forwarding.
    pub express_route_gateway_bypass: Output<serde_json::Value>,
    /// GatewayCustomBgpIpAddresses to be used for virtual network gateway Connection.
    pub gateway_custom_bgp_ip_addresses: Output<serde_json::Value>,
    /// The ingress bytes transferred in this connection.
    pub ingress_bytes_transferred: Output<serde_json::Value>,
    /// List of ingress NatRules.
    pub ingress_nat_rules: Output<serde_json::Value>,
    /// The IPSec Policies to be considered by this connection.
    pub ipsec_policies: Output<serde_json::Value>,
    /// The reference to local network gateway resource.
    pub local_network_gateway2: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The reference to peerings resource.
    pub peer: Output<serde_json::Value>,
    /// The provisioning state of the virtual network gateway connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the virtual network gateway connection resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The routing weight.
    pub routing_weight: Output<serde_json::Value>,
    /// The IPSec shared key.
    pub shared_key: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The Traffic Selector Policies to be considered by this connection.
    pub traffic_selector_policies: Output<serde_json::Value>,
    /// Collection of all tunnels' connection health status.
    pub tunnel_connection_status: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// Use private local Azure IP for the connection.
    pub use_local_azure_ip_address: Output<serde_json::Value>,
    /// Enable policy-based traffic selectors.
    pub use_policy_based_traffic_selectors: Output<serde_json::Value>,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway1: Output<serde_json::Value>,
    /// The reference to virtual network gateway resource.
    pub virtual_network_gateway2: Output<serde_json::Value>,
}

impl VirtualNetworkGatewayConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220701:VirtualNetworkGatewayConnection";

    /// Create a new VirtualNetworkGatewayConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkGatewayConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.authorization_key {
            pulumi_sdk::resolve_input("authorizationKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_mode {
            pulumi_sdk::resolve_input("connectionMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_protocol {
            pulumi_sdk::resolve_input("connectionProtocol", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("connectionType", args.connection_type, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.dpd_timeout_seconds {
            pulumi_sdk::resolve_input("dpdTimeoutSeconds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.egress_nat_rules {
            pulumi_sdk::resolve_input_vec("egressNatRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_bgp {
            pulumi_sdk::resolve_input("enableBgp", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_private_link_fast_path {
            pulumi_sdk::resolve_input("enablePrivateLinkFastPath", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_gateway_bypass {
            pulumi_sdk::resolve_input("expressRouteGatewayBypass", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_custom_bgp_ip_addresses {
            pulumi_sdk::resolve_input_vec("gatewayCustomBgpIpAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ingress_nat_rules {
            pulumi_sdk::resolve_input_vec("ingressNatRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ipsec_policies {
            pulumi_sdk::resolve_input_vec("ipsecPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.local_network_gateway2 {
            pulumi_sdk::resolve_input("localNetworkGateway2", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer {
            pulumi_sdk::resolve_input("peer", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.routing_weight {
            pulumi_sdk::resolve_input("routingWeight", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.shared_key {
            pulumi_sdk::resolve_input("sharedKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        pulumi_sdk::resolve_input("virtualNetworkGateway1", args.virtual_network_gateway1, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.virtual_network_gateway2 {
            pulumi_sdk::resolve_input("virtualNetworkGateway2", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_gateway_connection_name {
            pulumi_sdk::resolve_input("virtualNetworkGatewayConnectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            authorization_key: registered.outputs.get("authorizationKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_mode: registered.outputs.get("connectionMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_protocol: registered.outputs.get("connectionProtocol")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_status: registered.outputs.get("connectionStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_type: registered.outputs.get("connectionType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dpd_timeout_seconds: registered.outputs.get("dpdTimeoutSeconds")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            egress_bytes_transferred: registered.outputs.get("egressBytesTransferred")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            egress_nat_rules: registered.outputs.get("egressNatRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_bgp: registered.outputs.get("enableBgp")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_private_link_fast_path: registered.outputs.get("enablePrivateLinkFastPath")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_gateway_bypass: registered.outputs.get("expressRouteGatewayBypass")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_custom_bgp_ip_addresses: registered.outputs.get("gatewayCustomBgpIpAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ingress_bytes_transferred: registered.outputs.get("ingressBytesTransferred")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ingress_nat_rules: registered.outputs.get("ingressNatRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ipsec_policies: registered.outputs.get("ipsecPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            local_network_gateway2: registered.outputs.get("localNetworkGateway2")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer: registered.outputs.get("peer")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_weight: registered.outputs.get("routingWeight")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            shared_key: registered.outputs.get("sharedKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            traffic_selector_policies: registered.outputs.get("trafficSelectorPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tunnel_connection_status: registered.outputs.get("tunnelConnectionStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            use_local_azure_ip_address: registered.outputs.get("useLocalAzureIpAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            use_policy_based_traffic_selectors: registered.outputs.get("usePolicyBasedTrafficSelectors")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_gateway1: registered.outputs.get("virtualNetworkGateway1")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_gateway2: registered.outputs.get("virtualNetworkGateway2")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
