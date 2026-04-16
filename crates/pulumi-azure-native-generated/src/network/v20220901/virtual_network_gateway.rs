use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A common class for general resource information.
pub struct VirtualNetworkGatewayArgs {
    /// ActiveActive flag.
    pub active_active: Option<Input<bool>>,
    /// Configure this gateway to accept traffic from other Azure Virtual Networks. This configuration does not support connectivity to Azure Virtual WAN.
    pub allow_remote_vnet_traffic: Option<Input<bool>>,
    /// Configures this gateway to accept traffic from remote Virtual WAN networks.
    pub allow_virtual_wan_traffic: Option<Input<bool>>,
    /// Virtual network gateway's BGP speaker settings.
    pub bgp_settings: Option<Input<network::v20220901::BgpSettingsArgs>>,
    /// The reference to the address space resource which represents the custom routes address space specified by the customer for virtual network gateway and VpnClient.
    pub custom_routes: Option<Input<network::v20220901::AddressSpaceArgs>>,
    /// disableIPSecReplayProtection flag.
    pub disable_ip_sec_replay_protection: Option<Input<bool>>,
    /// Whether BGP is enabled for this virtual network gateway or not.
    pub enable_bgp: Option<Input<bool>>,
    /// EnableBgpRouteTranslationForNat flag.
    pub enable_bgp_route_translation_for_nat: Option<Input<bool>>,
    /// Whether dns forwarding is enabled or not.
    pub enable_dns_forwarding: Option<Input<bool>>,
    /// Whether private IP needs to be enabled on this gateway for connections or not.
    pub enable_private_ip_address: Option<Input<bool>>,
    /// The extended location of type local virtual network gateway.
    pub extended_location: Option<Input<network::v20220901::ExtendedLocationArgs>>,
    /// The reference to the LocalNetworkGateway resource which represents local network site having default routes. Assign Null value in case of removing existing default site setting.
    pub gateway_default_site: Option<Input<network::v20220901::SubResourceArgs>>,
    /// The type of this virtual network gateway.
    pub gateway_type: Option<Input<serde_json::Value>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// IP configurations for virtual network gateway.
    pub ip_configurations: Option<Vec<Input<network::v20220901::VirtualNetworkGatewayIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// NatRules for virtual network gateway.
    pub nat_rules: Option<Vec<Input<network::v20220901::VirtualNetworkGatewayNatRuleArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The reference to the VirtualNetworkGatewaySku resource which represents the SKU selected for Virtual network gateway.
    pub sku: Option<Input<network::v20220901::VirtualNetworkGatewaySkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Customer vnet resource id. VirtualNetworkGateway of type local gateway is associated with the customer vnet.
    pub v_net_extended_location_resource_id: Option<Input<String>>,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: Option<Input<String>>,
    /// The reference to the VirtualNetworkGatewayPolicyGroup resource which represents the available VirtualNetworkGatewayPolicyGroup for the gateway.
    pub virtual_network_gateway_policy_groups: Option<Vec<Input<network::v20220901::VirtualNetworkGatewayPolicyGroupArgs>>>,
    /// The reference to the VpnClientConfiguration resource which represents the P2S VpnClient configurations.
    pub vpn_client_configuration: Option<Input<network::v20220901::VpnClientConfigurationArgs>>,
    /// The generation for this VirtualNetworkGateway. Must be None if gatewayType is not VPN.
    pub vpn_gateway_generation: Option<Input<serde_json::Value>>,
    /// The type of this virtual network gateway.
    pub vpn_type: Option<Input<serde_json::Value>>,
}

/// A common class for general resource information.
pub struct VirtualNetworkGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// ActiveActive flag.
    pub active_active: Output<serde_json::Value>,
    /// Configure this gateway to accept traffic from other Azure Virtual Networks. This configuration does not support connectivity to Azure Virtual WAN.
    pub allow_remote_vnet_traffic: Output<serde_json::Value>,
    /// Configures this gateway to accept traffic from remote Virtual WAN networks.
    pub allow_virtual_wan_traffic: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Virtual network gateway's BGP speaker settings.
    pub bgp_settings: Output<serde_json::Value>,
    /// The reference to the address space resource which represents the custom routes address space specified by the customer for virtual network gateway and VpnClient.
    pub custom_routes: Output<serde_json::Value>,
    /// disableIPSecReplayProtection flag.
    pub disable_ip_sec_replay_protection: Output<serde_json::Value>,
    /// Whether BGP is enabled for this virtual network gateway or not.
    pub enable_bgp: Output<serde_json::Value>,
    /// EnableBgpRouteTranslationForNat flag.
    pub enable_bgp_route_translation_for_nat: Output<serde_json::Value>,
    /// Whether dns forwarding is enabled or not.
    pub enable_dns_forwarding: Output<serde_json::Value>,
    /// Whether private IP needs to be enabled on this gateway for connections or not.
    pub enable_private_ip_address: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of type local virtual network gateway.
    pub extended_location: Output<serde_json::Value>,
    /// The reference to the LocalNetworkGateway resource which represents local network site having default routes. Assign Null value in case of removing existing default site setting.
    pub gateway_default_site: Output<serde_json::Value>,
    /// The type of this virtual network gateway.
    pub gateway_type: Output<serde_json::Value>,
    /// The IP address allocated by the gateway to which dns requests can be sent.
    pub inbound_dns_forwarding_endpoint: Output<serde_json::Value>,
    /// IP configurations for virtual network gateway.
    pub ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// NatRules for virtual network gateway.
    pub nat_rules: Output<serde_json::Value>,
    /// The provisioning state of the virtual network gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the virtual network gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The reference to the VirtualNetworkGatewaySku resource which represents the SKU selected for Virtual network gateway.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// Customer vnet resource id. VirtualNetworkGateway of type local gateway is associated with the customer vnet.
    pub v_net_extended_location_resource_id: Output<serde_json::Value>,
    /// The reference to the VirtualNetworkGatewayPolicyGroup resource which represents the available VirtualNetworkGatewayPolicyGroup for the gateway.
    pub virtual_network_gateway_policy_groups: Output<serde_json::Value>,
    /// The reference to the VpnClientConfiguration resource which represents the P2S VpnClient configurations.
    pub vpn_client_configuration: Output<serde_json::Value>,
    /// The generation for this VirtualNetworkGateway. Must be None if gatewayType is not VPN.
    pub vpn_gateway_generation: Output<serde_json::Value>,
    /// The type of this virtual network gateway.
    pub vpn_type: Output<serde_json::Value>,
}

impl VirtualNetworkGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220901:VirtualNetworkGateway";

    /// Create a new VirtualNetworkGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.active_active {
            pulumi_sdk::resolve_input("activeActive", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_remote_vnet_traffic {
            pulumi_sdk::resolve_input("allowRemoteVnetTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_virtual_wan_traffic {
            pulumi_sdk::resolve_input("allowVirtualWanTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.bgp_settings {
            pulumi_sdk::resolve_input("bgpSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_routes {
            pulumi_sdk::resolve_input("customRoutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_ip_sec_replay_protection {
            pulumi_sdk::resolve_input("disableIPSecReplayProtection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_bgp {
            pulumi_sdk::resolve_input("enableBgp", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_bgp_route_translation_for_nat {
            pulumi_sdk::resolve_input("enableBgpRouteTranslationForNat", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_dns_forwarding {
            pulumi_sdk::resolve_input("enableDnsForwarding", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_private_ip_address {
            pulumi_sdk::resolve_input("enablePrivateIpAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_default_site {
            pulumi_sdk::resolve_input("gatewayDefaultSite", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_type {
            pulumi_sdk::resolve_input("gatewayType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_rules {
            pulumi_sdk::resolve_input_vec("natRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.v_net_extended_location_resource_id {
            pulumi_sdk::resolve_input("vNetExtendedLocationResourceId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_gateway_name {
            pulumi_sdk::resolve_input("virtualNetworkGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_gateway_policy_groups {
            pulumi_sdk::resolve_input_vec("virtualNetworkGatewayPolicyGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_client_configuration {
            pulumi_sdk::resolve_input("vpnClientConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway_generation {
            pulumi_sdk::resolve_input("vpnGatewayGeneration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_type {
            pulumi_sdk::resolve_input("vpnType", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            active_active: registered.outputs.get("activeActive")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_remote_vnet_traffic: registered.outputs.get("allowRemoteVnetTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_virtual_wan_traffic: registered.outputs.get("allowVirtualWanTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            bgp_settings: registered.outputs.get("bgpSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_routes: registered.outputs.get("customRoutes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            disable_ip_sec_replay_protection: registered.outputs.get("disableIPSecReplayProtection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_bgp: registered.outputs.get("enableBgp")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_bgp_route_translation_for_nat: registered.outputs.get("enableBgpRouteTranslationForNat")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_dns_forwarding: registered.outputs.get("enableDnsForwarding")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_private_ip_address: registered.outputs.get("enablePrivateIpAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_default_site: registered.outputs.get("gatewayDefaultSite")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_type: registered.outputs.get("gatewayType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            inbound_dns_forwarding_endpoint: registered.outputs.get("inboundDnsForwardingEndpoint")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nat_rules: registered.outputs.get("natRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            v_net_extended_location_resource_id: registered.outputs.get("vNetExtendedLocationResourceId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_gateway_policy_groups: registered.outputs.get("virtualNetworkGatewayPolicyGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_client_configuration: registered.outputs.get("vpnClientConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway_generation: registered.outputs.get("vpnGatewayGeneration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_type: registered.outputs.get("vpnType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
